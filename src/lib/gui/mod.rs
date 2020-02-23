//! Draw composition GUI

use core::tree;
use core::Player;
use error::*;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use error_chain::ChainedError;
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::Vector2u;
use sfml::window::{Event, Key, Style};

mod drawable;
pub use self::drawable::Drawable;

const MAX_FPS: u32 = 30;
const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 400;

lazy_static! {
    static ref COLORS: Vec<Color> = vec![
        Color::rgb(255, 128, 128),
        Color::rgb(128, 255, 128),
        Color::rgb(128, 128, 255),
        Color::rgb(255, 255, 128),
        Color::rgb(255, 200, 128),
    ];
}

/// Start showing the GUI for a composition
pub fn start(player: Arc<Mutex<Box<dyn Player>>>) -> Result<()> {
    thread::spawn(|| {
        if let Err(err) = start_window(player) {
            error!("Error in GUI thread: {}", err.display_chain());
        }
    });

    Ok(())
}

fn start_window(player: Arc<Mutex<Box<dyn Player>>>) -> Result<()> {
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Composer",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let window_size = window.size();

    let mut last_draw_time = Instant::now();
    let min_draw_gap = Duration::from_millis((1000.0 / f64::from(MAX_FPS)) as u64);
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
        draw_composition(&mut window, window_size, &*player)?;
        window.display();

        last_draw_time = Instant::now();
    }
}

fn draw_composition(
    window: &mut RenderWindow,
    window_size: Vector2u,
    reloading_player: &Mutex<Box<dyn Player>>,
) -> Result<()> {
    // Get what to draw
    let locked_player = reloading_player.lock().unwrap();
    let drawables: Vec<&dyn Drawable> = tree::flatten_tree(locked_player.to_tree())
        .into_iter()
        .flat_map(tree::Tree::get_drawables)
        .collect();
    if drawables.is_empty() {
        return Ok(());
    }

    let num_drawables = drawables.len();
    let drawable_height = window_size.y as u32 / num_drawables as u32;

    for (i, drawable) in drawables.into_iter().enumerate() {
        drawable.draw(
            window,
            COLORS[i % COLORS.len()],
            window_size.x as u32,
            drawable_height,
            0,
            drawable_height * i as u32,
        )?;
    }

    Ok(())
}
