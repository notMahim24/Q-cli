use crate::data::quran;
use crate::ui::browser::{self, BrowserState, Panel};
use crate::ui::theme;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::time::Duration;

pub struct App {
    state: BrowserState,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: BrowserState::new(),
            should_quit: false,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code).await;
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let theme = theme::get_theme(self.state.theme);
        browser::render_browser(frame, frame.area(), &mut self.state, &theme);
    }

    async fn handle_key(&mut self, key: KeyCode) {
        if let browser::SearchMode::Active { ref mut query, ref mut list_state, ref mut results } = self.state.search {
            match key {
                KeyCode::Esc => {
                    self.state.search = browser::SearchMode::Off;
                }
                KeyCode::Backspace => {
                    query.pop();
                    self.live_search().await;
                }
                KeyCode::Char(c) => {
                    query.push(c);
                    self.live_search().await;
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
                    if let Some(res) = self.state.selected_search_result() {
                        let surah_id = res.surah_id;
                        let ayah_num = res.ayah_num;
                        self.state.jump_to_result(surah_id, ayah_num);
                        self.load_surah();
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
                self.state.search = browser::SearchMode::Active {
                    query: String::new(),
                    results: vec![],
                    list_state: ratatui::widgets::ListState::default(),
                };
            }
            KeyCode::Char('t') => {
                self.state.theme = self.state.theme.next();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.state.prev_panel();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let changed = self.state.select_current();
                if changed {
                    self.load_surah();
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.state.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.state.move_down();
            }
            KeyCode::Enter => {
                let changed = self.state.select_current();
                if changed {
                    self.load_surah();
                }
            }
            _ => {}
        }
    }

    async fn live_search(&mut self) {
        if let browser::SearchMode::Active { ref query, ref mut results, ref mut list_state } = self.state.search {
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

    fn load_surah(&mut self) {
        let id = self.state.selected_surah_id;
        self.state.current_surah = quran::get_surah(id).cloned();
    }
}
