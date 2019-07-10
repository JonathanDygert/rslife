use crate::gui::Theme;
use crate::Game;
use crate::Point;

use drag_controller::Drag;
use opengl_graphics::GlGraphics;
use piston::input::*;

/// The main struct for running the application.
pub struct App {
    /// OpenGL drawing backend.
    gl: GlGraphics,

    /// Underlying cell automata.
    game: Game,

    /// Render size of each cell.
    cell_size: f64,

    /// Offset based on dragging view.
    offset: Point<f64>,

    /// Used to calculate distance dragged.
    dragged: Point<f64>,

    /// Determines colors.
    theme: Theme,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("gl", &"...")
            .field("game", &self.game)
            .field("cell_size", &self.cell_size)
            .field("offset", &self.offset)
            .field("dragged", &self.dragged)
            .field("theme", &self.theme)
            .finish()
    }
}

impl App {
    /// Creates a new App with the given parameters.
    pub fn new(gl: GlGraphics, game: Game, cell_size: f64) -> Self {
        App {
            gl,
            game,
            cell_size,
            offset: Point::default(),
            dragged: Point::default(),
            theme: Theme::Dark,
        }
    }

    /// Renders the current state of the App.
    pub fn render(&mut self, args: RenderArgs) {
        use graphics::*;

        let App {
            ref mut gl,
            ref mut game,
            cell_size,
            offset,
            theme,
            ..
        } = *self;

        let [width, height] = args.window_size;

        let square = rectangle::centered_square(0.0, 0.0, cell_size * 0.9 / 2.0);

        gl.draw(args.viewport(), |c, gl| {
            // Center on screen and adjust for dragging.
            let transform = c
                .transform
                .trans(width / 2.0, height / 2.0)
                .trans(offset.x, offset.y);

            // Clear the screen.
            clear(theme.background(), gl);

            // Draws active cells.
            for p in game.active() {
                let transform =
                    transform.trans(f64::from(p.x) * cell_size, f64::from(p.y) * cell_size);

                rectangle(theme.cell(), square, transform, gl);
            }
        });
    }

    /// Updates the application, moving the game forward one generation.
    pub fn update(&mut self, _args: UpdateArgs) {
        self.game.advance();
    }

    /// Handles dragging to pan the view.
    pub fn drag(&mut self, action: Drag) -> bool {
        match action {
            Drag::Start(x, y) => {
                self.dragged = Point { x, y };
                true
            }
            Drag::Move(x, y) => {
                let p = Point { x, y };
                self.offset += p - self.dragged;
                self.dragged = p;
                true
            }
            Drag::End(_, _) => false,
            Drag::Interrupt => true,
        }
    }

    /// Handles scrolling to zoom in and out.
    pub fn mouse_scroll(&mut self, _x: f64, y: f64) {
        if y > 0.0 && self.cell_size < 100.0 {
            // Zoom in
            let scale = y * 1.25;
            self.cell_size *= scale;

            // Keep center of screen at center
            self.offset.x *= scale;
            self.offset.y *= scale;
        } else if y < 0.0 && self.cell_size > 0.5 {
            // Zoom out
            let scale = -y * 1.25;
            self.cell_size /= scale;

            // Keep center of screen at center
            self.offset.x /= scale;
            self.offset.y /= scale;
        }
    }

    /// Handles keypresses.
    pub fn press(&mut self, button: Button) {
        use piston::input::Button::Keyboard;

        match button {
            Keyboard(Key::Up) => self.cell_size *= 2.0,
            Keyboard(Key::Down) => self.cell_size /= 2.0,
            Keyboard(Key::G) => println!("Generation: {}", self.game.generation()),
            Keyboard(Key::T) => self.theme.swap(),
            _ => {}
        }
    }
}
