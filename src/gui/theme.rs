use graphics::types::Color;

mod color {
    use graphics::types::Color;
    pub const DARK: Color = [0.12, 0.12, 0.12, 1.0];
    pub const LIGHT: Color = [0.8, 0.8, 0.8, 1.0];
}

#[derive(Copy, Clone, Debug)]
/// The theme for the GUI.
pub enum Theme {
    /// The light theme.
    Light,
    /// The dark theme.
    Dark,
}

impl Theme {
    /// Gets the background color associated with the theme.
    pub fn background(self) -> Color {
        match self {
            Theme::Light => color::LIGHT,
            Theme::Dark => color::DARK,
        }
    }

    /// Gets the cell color associated with the theme.
    pub fn cell(self) -> Color {
        match self {
            Theme::Light => color::DARK,
            Theme::Dark => color::LIGHT,
        }
    }

    /// Swaps between available themes.
    pub fn swap(&mut self) {
        *self = match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}
