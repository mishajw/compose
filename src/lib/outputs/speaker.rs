//! Play music to the device speaker

use core::spec::create;
use core::spec::{Spec, Value};
use core::Output;
use core::Playable;
use error::*;

use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};

use core::Consts;
use portaudio;

const NUM_CHANNELS: i32 = 1;
const FRAMES: usize = 2048;
const MAX_UNPLAYED_BUFFERS: usize = 3;

type Stream = portaudio::stream::Stream<
    portaudio::stream::NonBlocking,
    portaudio::Output<i32>,
>;

/// Play music to a device speaker
pub struct Speaker {
    /// Stream to write audio data to
    audio_stream: Stream,
    /// Message sent to this receiver whenever a buffer has been writen to the
    /// stream
    write_reciever: mpsc::Receiver<()>,
    /// Buffers to write to the stream
    audio_buffers: Arc<Mutex<VecDeque<[Playable; FRAMES]>>>,
    /// Current buffer being created
    current_buffer: [Playable; FRAMES],
    /// Index of where we've written to  in the current buffer
    current_buffer_index: usize,
}

impl Speaker {
    #[allow(missing_docs)]
    fn new(output_frequency: f32) -> Result<Self> {
        // Initialize portaudio interface
        let audio = portaudio::PortAudio::new()
            .chain_err(|| "Failed to initialize PortAudio")?;

        // Create the stream settings
        let mut audio_settings = audio
            .default_output_stream_settings(
                NUM_CHANNELS,
                output_frequency as f64,
                FRAMES as u32,
            )
            .chain_err(|| "Failed to get default audio stream settings")?;
        audio_settings.flags = portaudio::stream_flags::CLIP_OFF;

        // Create and start the stream
        let audio_buffers = Arc::new(Mutex::new(VecDeque::new()));
        let (write_sender, write_reciever) = mpsc::channel();
        let callback =
            Self::create_callback(audio_buffers.clone(), write_sender);
        let mut audio_stream = audio
            .open_non_blocking_stream(audio_settings, callback)
            .chain_err(|| "Failed to open audio stream")?;
        audio_stream
            .start()
            .chain_err(|| "Failed to start audio stream")?;

        Ok(Speaker {
            audio_stream,
            write_reciever,
            audio_buffers,
            current_buffer: [Playable::new(0); FRAMES],
            current_buffer_index: 0,
        })
    }

    fn create_callback(
        audio_buffers: Arc<Mutex<VecDeque<[Playable; FRAMES]>>>,
        write_sender: mpsc::Sender<()>,
    ) -> impl Fn(
        portaudio::OutputStreamCallbackArgs<'static, i32>,
    ) -> portaudio::stream::CallbackResult
    {
        move |args: portaudio::OutputStreamCallbackArgs<'static, i32>| {
            let buffer = audio_buffers.lock().unwrap().pop_front();
            if buffer.is_none() {
                warn!("Playables aren't produced fast enough to write");
                return portaudio::Continue;
            }

            let playables = buffer.unwrap();
            for (mut buffer_value, playable) in
                args.buffer.iter_mut().zip(playables.iter())
            {
                *buffer_value = playable.get_value();
            }

            write_sender
                .send(())
                .expect("Failed to send on write_sender");
            portaudio::Continue
        }
    }
}

impl Output for Speaker {
    fn write(&mut self, playable: Playable) {
        // Write playable to buffer
        debug_assert!(self.current_buffer_index < FRAMES);
        self.current_buffer[self.current_buffer_index] = playable;
        self.current_buffer_index += 1;
        if self.current_buffer_index < FRAMES {
            return;
        }

        // Wait until there's enough space in the buffer queue
        while self.audio_buffers.lock().unwrap().len() > MAX_UNPLAYED_BUFFERS {
            self.write_reciever
                .recv()
                .expect("Failed to receive from write receiver");
        }

        // Add the current buffer to the buffer queue
        let mut audio_buffers = self.audio_buffers.lock().unwrap();
        audio_buffers.push_back(self.current_buffer);
        self.current_buffer_index = 0;
    }
}

impl create::FromSpec<Box<Output>> for Speaker {
    fn name() -> &'static str { "speaker" }
    fn from_spec(value: Value, consts: &Consts) -> Result<Box<Output>> {
        let spec: Spec = value.as_type()?;
        spec.ensure_all_used()?;
        Ok(Box::new(Speaker::new(
            consts.sample_hz,
        )?))
    }
}

impl Drop for Speaker {
    fn drop(&mut self) {
        self.audio_stream.stop().expect("Failed to stop audio");
        self.audio_stream.close().expect("Failed to close audio");
    }
}
