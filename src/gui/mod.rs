//! This module forms the GUI for the simulation.

use crate::Args;
use crate::Game;

use drag_controller::DragController;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod app;
mod theme;

pub use self::app::App;
pub use self::theme::Theme;

/// The main function to run the GUI.
pub fn main(args: &Args) -> crate::Result<()> {
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Conway's Game of Life", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game.
    let game = if args.cmd_rle {
        Game::rle(&args.arg_file)?
    } else if args.cmd_random {
        Game::random(args.arg_width, args.arg_height)
    } else {
        unreachable!("Unexpected arguments: {:#?}", args);
    };

    // Create and run the App
    let mut app = App::new(GlGraphics::new(OpenGL::V3_2), game, 8.1);

    let mut ups = args.flag_speed;
    let mut events = Events::new(EventSettings::new().ups(ups));

    let mut drag = DragController::new();

    while let Some(e) = events.next(&mut window) {
        drag.event(&e, |action| app.drag(action));
        e.render(|&r| app.render(r));
        e.update(|&u| app.update(u));
        e.mouse_scroll(|[x, y]| app.mouse_scroll(x, y));
        e.press(|button| {
            use piston::input::Button::Keyboard;

            match button {
                Keyboard(Key::Left) => {
                    ups /= 2;
                    if ups == 0 {
                        ups = 1;
                    }
                    events = Events::new(EventSettings::new().ups(ups));
                }
                Keyboard(Key::Right) => {
                    ups *= 2;
                    events = Events::new(EventSettings::new().ups(ups));
                }
                _ => app.press(button),
            };
        });
    }

    Ok(())
}
