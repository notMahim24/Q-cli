use ratatui::style::Color;

#[derive(Debug, Clone, Copy, Default)]
pub enum ThemeName {
    #[default]
    Slate,
    Emerald,
    Sand,
    Night,
}

pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub primary: Color,
    pub secondary: Color,
    pub border: Color,
    pub selection_bg: Color,
}

impl ThemeName {
    pub fn next(&self) -> Self {
        match self {
            ThemeName::Slate => ThemeName::Emerald,
            ThemeName::Emerald => ThemeName::Sand,
            ThemeName::Sand => ThemeName::Night,
            ThemeName::Night => ThemeName::Slate,
        }
    }
}

pub fn get_theme(name: ThemeName) -> Theme {
    match name {
        ThemeName::Slate => Theme {
            bg: Color::Rgb(15, 23, 42),
            fg: Color::Rgb(241, 245, 249),
            primary: Color::Rgb(56, 189, 248),
            secondary: Color::Rgb(148, 163, 184),
            border: Color::Rgb(30, 41, 59),
            selection_bg: Color::Rgb(30, 41, 59),
        },
        ThemeName::Emerald => Theme {
            bg: Color::Rgb(2, 44, 34),
            fg: Color::Rgb(236, 253, 245),
            primary: Color::Rgb(52, 211, 153),
            secondary: Color::Rgb(110, 231, 183),
            border: Color::Rgb(6, 78, 59),
            selection_bg: Color::Rgb(6, 78, 59),
        },
        ThemeName::Sand => Theme {
            bg: Color::Rgb(254, 252, 232),
            fg: Color::Rgb(66, 32, 6),
            primary: Color::Rgb(161, 98, 7),
            secondary: Color::Rgb(113, 63, 18),
            border: Color::Rgb(250, 204, 21),
            selection_bg: Color::Rgb(253, 224, 71),
        },
        ThemeName::Night => Theme {
            bg: Color::Rgb(10, 10, 10),
            fg: Color::Rgb(255, 255, 255),
            primary: Color::Rgb(255, 255, 255),
            secondary: Color::Rgb(161, 161, 170),
            border: Color::Rgb(39, 39, 42),
            selection_bg: Color::Rgb(39, 39, 42),
        },
    }
}
