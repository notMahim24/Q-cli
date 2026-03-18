use crate::api::types::{SearchResult, Surah};
use crate::data::quran;
use crate::ui::theme::{Theme, ThemeName};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph,
        Wrap,
    },
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Surahs,
    Scripture,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchMode {
    Off,
    Active {
        query: String,
        results: Vec<SearchResult>,
        list_state: ListState,
    },
}

pub struct BrowserState {
    pub active_panel: Panel,
    pub surah_list: ListState,
    pub scripture_scroll: u16,
    pub selected_surah_id: u32,
    pub current_surah: Option<Surah>,
    pub loading: bool,
    pub search: SearchMode,
    pub theme: ThemeName,
    pub scripture_max_scroll: u16,
}

impl BrowserState {
    pub fn new() -> Self {
        let mut surah_list = ListState::default();
        surah_list.select(Some(0));

        Self {
            active_panel: Panel::Surahs,
            surah_list,
            scripture_scroll: 0,
            selected_surah_id: 1,
            current_surah: None,
            loading: false,
            search: SearchMode::Off,
            theme: ThemeName::default(),
            scripture_max_scroll: 0,
        }
    }

    pub fn next_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Surahs => Panel::Scripture,
            Panel::Scripture => Panel::Scripture,
        };
    }

    pub fn prev_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Surahs => Panel::Surahs,
            Panel::Scripture => Panel::Surahs,
        };
    }

    pub fn move_up(&mut self) {
        match self.active_panel {
            Panel::Surahs => {
                let i = self.surah_list.selected().unwrap_or(0);
                if i > 0 {
                    self.surah_list.select(Some(i - 1));
                    self.selected_surah_id = i as u32;
                    self.scripture_scroll = 0; // Reset scroll for preview
                }
            }
            Panel::Scripture => {
                self.scripture_scroll = self.scripture_scroll.saturating_sub(3);
            }
        }
    }

    pub fn move_down(&mut self) {
        match self.active_panel {
            Panel::Surahs => {
                let i = self.surah_list.selected().unwrap_or(0);
                if i < 113 {
                    self.surah_list.select(Some(i + 1));
                    self.selected_surah_id = (i + 2) as u32;
                    self.scripture_scroll = 0; // Reset scroll for preview
                }
            }
            Panel::Scripture => {
                self.scripture_scroll = (self.scripture_scroll + 3).min(self.scripture_max_scroll);
            }
        }
    }

    pub fn select_current(&mut self) -> bool {
        match self.active_panel {
            Panel::Surahs => {
                let i = self.surah_list.selected().unwrap_or(0);
                let new_id = (i + 1) as u32;
                
                // Only reset scroll if we are switching to a DIFFERENT surah
                let id_changed = self.current_surah.as_ref().map(|s| s.id) != Some(new_id);
                if id_changed {
                    self.scripture_scroll = 0;
                }
                
                self.selected_surah_id = new_id;
                self.active_panel = Panel::Scripture;
                id_changed
            }
            Panel::Scripture => false,
        }
    }

    pub fn selected_search_result(&self) -> Option<&SearchResult> {
        if let SearchMode::Active { results, list_state, .. } = &self.search {
            let idx = list_state.selected()?;
            results.get(idx)
        } else {
            None
        }
    }

    pub fn jump_to_result(&mut self, surah_id: u32, ayah_num: u32) {
        self.selected_surah_id = surah_id;
        self.surah_list.select(Some((surah_id - 1) as usize));
        
        // Simple estimate: each verse is approx 2 lines
        self.scripture_scroll = (ayah_num.saturating_sub(1) * 2) as u16;
        
        self.active_panel = Panel::Scripture;
        self.search = SearchMode::Off;
    }
}

pub fn render_browser(
    frame: &mut Frame,
    area: Rect,
    state: &mut BrowserState,
    theme: &Theme,
) {
    let outer_block = Block::default()
        .title(Line::from(vec![
            Span::styled(" quran", Style::default().fg(theme.primary).bold()),
            Span::styled("-cli ", Style::default().fg(theme.secondary)),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    let has_search = matches!(state.search, SearchMode::Active { .. });

    let main_layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(if has_search { 3 } else { 0 }),
        Constraint::Length(1),
    ])
    .split(inner);

    let panels = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(70),
    ])
    .split(main_layout[0]);

    render_surahs_panel(frame, panels[0], state, theme);

    if has_search {
        render_search_results_panel(frame, panels[1], state, theme);
        render_search_input(frame, main_layout[1], state, theme);
    } else {
        render_scripture_panel(frame, panels[1], state, theme);
    }
    
    render_status_bar(frame, main_layout[2], theme);
}

fn render_status_bar(frame: &mut Frame, area: Rect, theme: &Theme) {
    let keybinds = vec![
        ("←→/hl", "panels"),
        ("↑↓/jk", "navigate"),
        ("Enter", "select"),
        ("/", "search"),
        ("t", "theme"),
        ("q", "quit"),
    ];

    let spans: Vec<Span> = keybinds
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(format!(" {} ", key), Style::default().fg(theme.primary).bold()),
                Span::styled(format!("{} ", desc), Style::default().fg(theme.secondary)),
                Span::styled("  ", Style::default()),
            ]
        })
        .collect();

    let bar = Paragraph::new(Line::from(spans)).style(Style::default().bg(theme.bg));
    frame.render_widget(bar, area);
}

fn render_search_results_panel(frame: &mut Frame, area: Rect, state: &mut BrowserState, theme: &Theme) {
    let (query, results, list_state) = match &mut state.search {
        SearchMode::Active { query, results, list_state } => (query, results, list_state),
        _ => return,
    };

    let block = Block::default()
        .title(format!(" Search Results for \"{}\" ", query))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.primary))
        .padding(Padding::horizontal(1));

    if results.is_empty() {
        let msg = if query.len() < 3 {
            "Type at least 3 characters..."
        } else {
            "No results found."
        };
        frame.render_widget(Paragraph::new(msg).block(block).alignment(Alignment::Center), area);
        return;
    }

    let items: Vec<ListItem> = results.iter().enumerate().map(|(i, res)| {
        let is_selected = Some(i) == list_state.selected();
        let style = if is_selected {
            Style::default().fg(theme.primary).bg(theme.selection_bg).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        ListItem::new(vec![
            Line::from(vec![
                Span::styled(format!("{} {}:{}", res.surah_name, res.surah_id, res.ayah_num), 
                Style::default().fg(theme.primary).bold()),
            ]),
            Line::from(vec![
                Span::styled(&res.translation, Style::default().fg(theme.secondary)),
            ]),
        ]).style(style)
    }).collect();

    let list = List::new(items).block(block).highlight_symbol("> ");
    frame.render_stateful_widget(list, area, list_state);
}

fn render_search_input(frame: &mut Frame, area: Rect, state: &BrowserState, theme: &Theme) {
    let query = match &state.search {
        SearchMode::Active { query, .. } => query,
        _ => return,
    };

    let block = Block::default()
        .title(" / Search ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.primary))
        .padding(Padding::horizontal(1));

    let input = Paragraph::new(format!("{}_", query)).block(block);
    frame.render_widget(input, area);
}

fn render_surahs_panel(frame: &mut Frame, area: Rect, state: &mut BrowserState, theme: &Theme) {
    let is_active = state.active_panel == Panel::Surahs;
    let block = Block::default()
        .title(" Surahs ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if is_active { theme.primary } else { theme.border }))
        .padding(Padding::horizontal(1));

    let mut items = Vec::new();
    for i in 1..=114 {
        if let Some(surah) = quran::get_surah(i) {
            let style = if Some((i - 1) as usize) == state.surah_list.selected() {
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD).bg(theme.selection_bg)
            } else {
                Style::default().fg(theme.fg)
            };
            items.push(ListItem::new(format!("{}. {}", i, surah.transliteration)).style(style));
        }
    }

    let list = List::new(items).block(block).highlight_symbol("> ");
    frame.render_stateful_widget(list, area, &mut state.surah_list);
}

fn render_scripture_panel(frame: &mut Frame, area: Rect, state: &mut BrowserState, theme: &Theme) {
    let is_active = state.active_panel == Panel::Scripture;
    
    let title = if let Some(ref surah) = state.current_surah {
        format!(" {} ({}) ", surah.transliteration, surah.translation)
    } else {
        " Scripture ".to_string()
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if is_active { theme.primary } else { theme.border }))
        .padding(Padding::new(2, 2, 1, 1));

    if let Some(ref surah) = state.current_surah {
        let mut lines = Vec::new();
        
        // Bismillah (except for Surah 9)
        if surah.id != 9 && surah.id != 1 {
             lines.push(Line::from(vec![
                Span::styled("In the name of Allah, the Entirely Merciful, the Especially Merciful.", 
                Style::default().fg(theme.secondary).italic())
            ]));
            lines.push(Line::default());
        }

        for ayah in &surah.verses {
            lines.push(Line::from(vec![
                Span::styled(format!(" {} ", ayah.id), Style::default().fg(theme.primary).bold()),
                Span::styled(&ayah.translation, Style::default().fg(theme.fg)),
            ]));
            lines.push(Line::default());
        }
        
        // Add decorative footer
        lines.push(Line::default());
        lines.push(Line::from(vec![
            Span::styled("───────── ✧ ─────────", Style::default().fg(theme.secondary))
        ]).alignment(Alignment::Center));
        lines.push(Line::from(vec![
            Span::styled("Sadaqallahul Azeem", Style::default().fg(theme.primary).italic())
        ]).alignment(Alignment::Center));
        lines.push(Line::default());

        let inner = block.inner(area);
        let visible_height = inner.height;
        let content_height = lines.len() as u16;
        
        // Update max scroll
        state.scripture_max_scroll = content_height.saturating_sub(visible_height);

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((state.scripture_scroll, 0));

        frame.render_widget(paragraph, area);
    } else {
        let welcome = Paragraph::new("Select a Surah to begin reading.")
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(welcome, area);
    }
}
