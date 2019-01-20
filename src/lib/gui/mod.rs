//! Draw composition GUI

use core::tree;
use core::ReloadingComposition;
use error::*;

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use error_chain::ChainedError;
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{Event, Key, Style};

mod drawable;
pub use self::drawable::Drawable;

const MAX_FPS: u32 = 30;
const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 400;

/// Start showing the GUI for a composition
pub fn start(composition: Arc<ReloadingComposition>) -> Result<()> {
    thread::spawn(|| {
        if let Err(err) = start_window(composition) {
            error!("Error in GUI thread: {}", err.display_chain());
        }
    });

    Ok(())
}

fn start_window(composition: Arc<ReloadingComposition>) -> Result<()> {
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Composer",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut last_draw_time = Instant::now();
    let min_draw_gap = Duration::from_millis((1000.0 / MAX_FPS as f32) as u64);
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return Ok(()),
                _ => {}
            }
        }

        let draw_gap = Instant::now() - last_draw_time;
        if draw_gap < min_draw_gap {
            thread::sleep(min_draw_gap - draw_gap);
        }

        window.clear(&Color::BLACK);
        draw_composition(&mut window, &composition)?;
        window.display();

        last_draw_time = Instant::now();
    }
}

fn draw_composition(
    window: &mut RenderWindow,
    composition: &ReloadingComposition,
) -> Result<()>
{
    // Get what to draw
    let composition = composition.get_composition();
    let player = composition.root_player.lock().unwrap();
    let drawables: Vec<&Drawable> = tree::flatten_tree(player.to_tree())
        .into_iter()
        .flat_map(tree::Tree::get_drawables)
        .collect();
    if drawables.is_empty() {
        return Ok(());
    }

    let num_drawables = drawables.len();
    let drawable_height = WINDOW_HEIGHT / num_drawables as u32;

    for (i, drawable) in drawables.into_iter().enumerate() {
        drawable.draw(
            window,
            WINDOW_WIDTH,
            drawable_height,
            0,
            drawable_height * i as u32,
        )?;
    }

    Ok(())
}
