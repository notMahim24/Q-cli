use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

const TITLE_ART: &[&str] = &[
    " ██████╗ ██╗   ██╗██████╗  █████╗ ███╗   ██╗     ██████╗██╗     ██╗",
    "██╔═══██╗██║   ██║██╔══██╗██╔══██╗████╗  ██║    ██╔════╝██║     ██║",
    "██║   ██║██║   ██║██████╔╝███████║██╔██╗ ██║    ██║     ██║     ██║",
    "██║▄▄ ██║██║   ██║██╔══██╗██╔══██║██║╚██╗██║    ██║     ██║     ██║",
    "╚██████╔╝╚██████╔╝██║  ██║██║  ██║██║ ╚████║    ╚██████╗███████╗██║",
    " ╚══▀▀═╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝     ╚═════╝╚══════╝╚═╝",
];

pub struct BannerState {
    pub phase: u8,
    pub tick: u32,
    pub done: bool,
}

impl BannerState {
    pub fn new() -> Self {
        Self {
            phase: 0,
            tick: 0,
            done: false,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        match self.tick {
            0..=50 => self.phase = 0,    // Title fades in
            51..=95 => self.phase = 1,   // Tagline types in
            96..=120 => self.phase = 2,  // Settle
            _ => self.done = true,
        }
    }
}

pub fn render_banner(frame: &mut Frame, area: Rect, state: &BannerState, theme: &Theme) {
    let block = Block::default().style(Style::default().bg(theme.bg));
    frame.render_widget(block, area);

    let content_height = 6 + 1 + 1; // title + gap + tagline
    let vertical = Layout::vertical([Constraint::Length(content_height as u16)])
        .flex(Flex::Center)
        .split(area);
    let center = vertical[0];

    let chunks = Layout::vertical([
        Constraint::Length(6), // Title
        Constraint::Length(1), // Gap
        Constraint::Length(1), // Tagline
    ])
    .split(center);

    // Phase 0+: Title (Fade in)
    {
        let opacity = if state.phase == 0 {
            (state.tick as f32 / 50.0).min(1.0)
        } else {
            1.0
        };
        let title_color = interpolate_color(theme.bg, theme.primary, opacity);

        let title_lines: Vec<Line> = TITLE_ART
            .iter()
            .map(|line| Line::from(Span::styled(*line, Style::default().fg(title_color).bold())))
            .collect();
        let title = Paragraph::new(title_lines).alignment(Alignment::Center);
        frame.render_widget(title, chunks[0]);
    }

    // Phase 1+: Tagline (typewriter)
    if state.phase >= 1 {
        let tagline = "The Light of Knowledge";
        let chars_visible = if state.phase == 1 {
            let progress = (state.tick - 51) as usize;
            (progress * tagline.len() / 44).min(tagline.len())
        } else {
            tagline.len()
        };
        let visible: String = tagline.chars().take(chars_visible).collect();

        let tag = Paragraph::new(Line::from(Span::styled(
            visible,
            Style::default().fg(theme.fg).dim(),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(tag, chunks[2]);
    }
}

pub fn interpolate_color(from: Color, to: Color, t: f32) -> Color {
    match (from, to) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
            let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
            let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
            Color::Rgb(r, g, b)
        }
        _ => to,
    }
}
