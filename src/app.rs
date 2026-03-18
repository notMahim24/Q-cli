use crate::data::quran;
use crate::ui::banner::{self, BannerState};
use crate::ui::browser::{self, BrowserState};
use crate::ui::theme;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::time::{Duration, Instant};

enum AppMode {
    Banner(BannerState),
    Browser(BrowserState),
}

pub struct App {
    mode: AppMode,
    should_quit: bool,
}

impl App {
    pub fn new(show_banner: bool) -> Self {
        let mode = if show_banner {
            AppMode::Banner(BannerState::new())
        } else {
            let mut state = BrowserState::new();
            state.current_surah = quran::get_surah(1).cloned();
            AppMode::Browser(state)
        };

        Self {
            mode,
            should_quit: false,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16); // ~60 FPS

        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code).await;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let AppMode::Banner(ref mut state) = self.mode {
                    state.tick();
                    if state.done {
                        let mut browser_state = BrowserState::new();
                        browser_state.current_surah = quran::get_surah(1).cloned();
                        self.mode = AppMode::Browser(browser_state);
                    }
                }
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match &mut self.mode {
            AppMode::Banner(state) => {
                let theme = theme::get_theme(crate::ui::theme::ThemeName::Slate);
                banner::render_banner(frame, frame.area(), state, &theme);
            }
            AppMode::Browser(state) => {
                let theme = theme::get_theme(state.theme);
                browser::render_browser(frame, frame.area(), state, &theme);
            }
        }
    }

    async fn handle_key(&mut self, key: KeyCode) {
        let state = match &mut self.mode {
            AppMode::Banner(_) => {
                // Skip banner on any key
                if key != KeyCode::Null {
                    let mut browser_state = BrowserState::new();
                    browser_state.current_surah = quran::get_surah(1).cloned();
                    self.mode = AppMode::Browser(browser_state);
                }
                return;
            }
            AppMode::Browser(state) => state,
        };

        if let browser::SearchMode::Active { ref mut query, ref mut list_state, ref mut results } = state.search {
            match key {
                KeyCode::Esc => {
                    state.search = browser::SearchMode::Off;
                }
                KeyCode::Backspace => {
                    query.pop();
                    Self::live_search(state).await;
                }
                KeyCode::Char(c) => {
                    query.push(c);
                    Self::live_search(state).await;
                }
                KeyCode::Up => {
                    let i = list_state.selected().unwrap_or(0);
                    if i > 0 {
                        list_state.select(Some(i - 1));
                    }
                }
                KeyCode::Down => {
                    let i = list_state.selected().unwrap_or(0);
                    if i < results.len().saturating_sub(1) {
                        list_state.select(Some(i + 1));
                    }
                }
                KeyCode::Enter => {
                    if let Some(res) = state.selected_search_result() {
                        let surah_id = res.surah_id;
                        let ayah_num = res.ayah_num;
                        state.jump_to_result(surah_id, ayah_num);
                        Self::load_surah(state);
                    }
                }
                _ => {}
            }
            return;
        }

        match key {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('/') => {
                state.search = browser::SearchMode::Active {
                    query: String::new(),
                    results: vec![],
                    list_state: ratatui::widgets::ListState::default(),
                };
            }
            KeyCode::Char('t') => {
                state.theme = state.theme.next();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                state.prev_panel();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let should_load = state.select_current();
                if should_load {
                    Self::load_surah(state);
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                state.move_up();
                Self::load_surah(state);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                state.move_down();
                Self::load_surah(state);
            }
            KeyCode::Enter => {
                let should_load = state.select_current();
                if should_load {
                    Self::load_surah(state);
                }
            }
            _ => {}
        }
    }

    async fn live_search(state: &mut BrowserState) {
        if let browser::SearchMode::Active { ref query, ref mut results, ref mut list_state } = state.search {
            if query.len() >= 3 {
                *results = quran::search(query);
                if !results.is_empty() {
                    list_state.select(Some(0));
                }
            } else {
                results.clear();
                list_state.select(None);
            }
        }
    }

    fn load_surah(state: &mut BrowserState) {
        let id = state.selected_surah_id;
        state.current_surah = quran::get_surah(id).cloned();
    }
}
