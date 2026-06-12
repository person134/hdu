use std::io;
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Clear, Gauge, Paragraph, Row, Table, TableState, Wrap},
    Frame, Terminal,
};

use crate::backend::Platform;
use crate::config::Config;
use crate::scanner::{self, Entry, SortField, SortOrder};
use crate::system::{self, DiskUsage};

const THEMES: &[(&str, fn() -> Theme)] = &[
    ("dark", Theme::dark),
    ("light", Theme::light),
    ("gruvbox", Theme::gruvbox),
    ("monokai", Theme::monokai),
    ("solarized", Theme::solarized),
    ("nord", Theme::nord),
    ("catppuccin", Theme::catppuccin),
    ("dracula", Theme::dracula),
    ("tokyo-night", Theme::tokyo_night),
    ("ayu-mirage", Theme::ayu_mirage),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppMode {
    List,
    Search,
    Detail,
}

#[derive(Debug, Clone)]
struct Theme {
    header: Color,
    dir: Color,
    file: Color,
    selected_bg: Color,
    selected_fg: Color,
    large: Color,
    medium: Color,
    small: Color,
    search: Color,
    help: Color,
    alert: Color,
    border: Color,
}

impl Theme {
    fn dark() -> Self {
        Theme {
            header: Color::Cyan,
            dir: Color::Blue,
            file: Color::White,
            selected_bg: Color::LightGreen,
            selected_fg: Color::Black,
            large: Color::Red,
            medium: Color::Yellow,
            small: Color::Green,
            search: Color::Yellow,
            help: Color::DarkGray,
            alert: Color::Red,
            border: Color::Cyan,
        }
    }

    fn light() -> Self {
        Theme {
            header: Color::Blue,
            dir: Color::Blue,
            file: Color::Black,
            selected_bg: Color::Blue,
            selected_fg: Color::White,
            large: Color::Red,
            medium: Color::Yellow,
            small: Color::Green,
            search: Color::Blue,
            help: Color::Gray,
            alert: Color::Red,
            border: Color::Blue,
        }
    }

    fn gruvbox() -> Self {
        Theme {
            header: Color::Rgb(0xfe, 0x80, 0x19),
            dir: Color::Rgb(0x83, 0xa5, 0x98),
            file: Color::Rgb(0xeb, 0xdb, 0xb2),
            selected_bg: Color::Rgb(0x83, 0xa5, 0x98),
            selected_fg: Color::Rgb(0x1d, 0x20, 0x21),
            large: Color::Rgb(0xfb, 0x49, 0x34),
            medium: Color::Rgb(0xd7, 0x99, 0x21),
            small: Color::Rgb(0x8e, 0xc0, 0x7c),
            search: Color::Rgb(0xd7, 0x99, 0x21),
            help: Color::Rgb(0x92, 0x84, 0x74),
            alert: Color::Rgb(0xfb, 0x49, 0x34),
            border: Color::Rgb(0xfe, 0x80, 0x19),
        }
    }

    fn monokai() -> Self {
        Theme {
            header: Color::Rgb(0xa6, 0xe2, 0x2e),
            dir: Color::Rgb(0x66, 0xd9, 0xef),
            file: Color::Rgb(0xf8, 0xf8, 0xf2),
            selected_bg: Color::Rgb(0xa6, 0xe2, 0x2e),
            selected_fg: Color::Rgb(0x27, 0x28, 0x22),
            large: Color::Rgb(0xf9, 0x26, 0x72),
            medium: Color::Rgb(0xe6, 0xdb, 0x74),
            small: Color::Rgb(0x66, 0xd9, 0xef),
            search: Color::Rgb(0xe6, 0xdb, 0x74),
            help: Color::Rgb(0x75, 0x71, 0x5e),
            alert: Color::Rgb(0xf9, 0x26, 0x72),
            border: Color::Rgb(0xa6, 0xe2, 0x2e),
        }
    }

    fn solarized() -> Self {
        Theme {
            header: Color::Rgb(0x26, 0x8b, 0xd2),
            dir: Color::Rgb(0x2a, 0xa1, 0x98),
            file: Color::Rgb(0x93, 0xa1, 0xa1),
            selected_bg: Color::Rgb(0x2a, 0xa1, 0x98),
            selected_fg: Color::Rgb(0x00, 0x2b, 0x36),
            large: Color::Rgb(0xdc, 0x32, 0x2f),
            medium: Color::Rgb(0xb5, 0x89, 0x00),
            small: Color::Rgb(0x85, 0x99, 0x00),
            search: Color::Rgb(0xb5, 0x89, 0x00),
            help: Color::Rgb(0x65, 0x7b, 0x83),
            alert: Color::Rgb(0xdc, 0x32, 0x2f),
            border: Color::Rgb(0x26, 0x8b, 0xd2),
        }
    }

    fn nord() -> Self {
        Theme {
            header: Color::Rgb(0x88, 0xc0, 0xd0),
            dir: Color::Rgb(0x5e, 0x81, 0xac),
            file: Color::Rgb(0xd8, 0xde, 0xe9),
            selected_bg: Color::Rgb(0x5e, 0x81, 0xac),
            selected_fg: Color::Rgb(0xec, 0xef, 0xf4),
            large: Color::Rgb(0xbf, 0x61, 0x6a),
            medium: Color::Rgb(0xeb, 0xcb, 0x8b),
            small: Color::Rgb(0xa3, 0xbe, 0x8c),
            search: Color::Rgb(0xd0, 0x87, 0x70),
            help: Color::Rgb(0x4c, 0x56, 0x6a),
            alert: Color::Rgb(0xbf, 0x61, 0x6a),
            border: Color::Rgb(0x81, 0xa1, 0xc1),
        }
    }

    fn catppuccin() -> Self {
        Theme {
            header: Color::Rgb(0x89, 0xb4, 0xfa),
            dir: Color::Rgb(0x89, 0xb4, 0xfa),
            file: Color::Rgb(0xcd, 0xd6, 0xf4),
            selected_bg: Color::Rgb(0xcb, 0xa6, 0xf7),
            selected_fg: Color::Rgb(0x1e, 0x1e, 0x2e),
            large: Color::Rgb(0xf3, 0x8b, 0xa8),
            medium: Color::Rgb(0xfa, 0xb3, 0x87),
            small: Color::Rgb(0xa6, 0xe3, 0xa1),
            search: Color::Rgb(0xfa, 0xb3, 0x87),
            help: Color::Rgb(0x6c, 0x70, 0x86),
            alert: Color::Rgb(0xf3, 0x8b, 0xa8),
            border: Color::Rgb(0xb4, 0xbe, 0xfe),
        }
    }

    fn dracula() -> Self {
        Theme {
            header: Color::Rgb(0xbd, 0x93, 0xf9),
            dir: Color::Rgb(0x8b, 0xe9, 0xfd),
            file: Color::Rgb(0xf8, 0xf8, 0xf2),
            selected_bg: Color::Rgb(0xff, 0x79, 0xc6),
            selected_fg: Color::Rgb(0x28, 0x2a, 0x36),
            large: Color::Rgb(0xff, 0x55, 0x55),
            medium: Color::Rgb(0xf1, 0xfa, 0x8c),
            small: Color::Rgb(0x50, 0xfa, 0x7b),
            search: Color::Rgb(0xff, 0xb8, 0x6c),
            help: Color::Rgb(0x62, 0x72, 0xa4),
            alert: Color::Rgb(0xff, 0x55, 0x55),
            border: Color::Rgb(0x8b, 0xe9, 0xfd),
        }
    }

    fn tokyo_night() -> Self {
        Theme {
            header: Color::Rgb(0x7a, 0xa2, 0xf7),
            dir: Color::Rgb(0x7d, 0xcf, 0xff),
            file: Color::Rgb(0xa9, 0xb1, 0xd6),
            selected_bg: Color::Rgb(0xbb, 0x9a, 0xf7),
            selected_fg: Color::Rgb(0x1a, 0x1b, 0x26),
            large: Color::Rgb(0xf7, 0x76, 0x8e),
            medium: Color::Rgb(0xe0, 0xaf, 0x68),
            small: Color::Rgb(0x9e, 0xce, 0x6a),
            search: Color::Rgb(0xff, 0x9e, 0x64),
            help: Color::Rgb(0x56, 0x5f, 0x89),
            alert: Color::Rgb(0xf7, 0x76, 0x8e),
            border: Color::Rgb(0x7d, 0xcf, 0xff),
        }
    }

    fn ayu_mirage() -> Self {
        Theme {
            header: Color::Rgb(0x73, 0xd0, 0xff),
            dir: Color::Rgb(0x95, 0xe6, 0xcb),
            file: Color::Rgb(0xcb, 0xcb, 0xc2),
            selected_bg: Color::Rgb(0xd4, 0xbf, 0xff),
            selected_fg: Color::Rgb(0x1f, 0x24, 0x30),
            large: Color::Rgb(0xf2, 0x79, 0x83),
            medium: Color::Rgb(0xff, 0xa7, 0x59),
            small: Color::Rgb(0xa6, 0xcc, 0x70),
            search: Color::Rgb(0xff, 0xa7, 0x59),
            help: Color::Rgb(0x5c, 0x67, 0x73),
            alert: Color::Rgb(0xf2, 0x79, 0x83),
            border: Color::Rgb(0x95, 0xe6, 0xcb),
        }
    }
}

pub struct AppUi {
    platform: Platform,
    config: Config,
    root: Entry,
    scan_path: PathBuf,
    current_stack: Vec<String>,
    current_entries: Vec<(usize, Entry)>,
    should_quit: bool,
    table_state: TableState,
    sort_field: SortField,
    sort_order: SortOrder,
    search_query: String,
    mode: AppMode,
    theme: Theme,
    theme_name: String,
    refresh_rate: u64,
    detail_scroll: u16,
    pressed_button: Option<usize>,
    last_tick: Instant,
    notification: Option<(String, Instant)>,
    col_starts: Vec<u16>,
    scanning: bool,
    disks: Vec<DiskUsage>,
    last_click: Option<(Instant, u16, u16)>,
}

impl AppUi {
    pub fn new(platform: Platform, config: &Config, refresh_rate: u64) -> Self {
        let theme_name = config.settings.theme.clone();
        let theme = Self::theme_from_name(&theme_name);
        let sf = match config.settings.sort_by.as_str() {
            "name" => SortField::Name,
            "count" => SortField::Count,
            _ => SortField::Size,
        };
        let so = match config.settings.sort_order.as_str() {
            "asc" => SortOrder::Asc,
            _ => SortOrder::Desc,
        };

        let mut app = AppUi {
            platform,
            config: config.clone(),
            root: Entry {
                path: PathBuf::new(),
                name: String::new(),
                size: 0,
                is_dir: false,
                children: Vec::new(),
                item_count: 0,
                modified: None,
            },
            scan_path: PathBuf::from("."),
            current_stack: Vec::new(),
            current_entries: Vec::new(),
            should_quit: false,
            table_state: TableState::default(),
            sort_field: sf,
            sort_order: so,
            search_query: String::new(),
            mode: AppMode::List,
            theme,
            theme_name,
            refresh_rate,
            detail_scroll: 0,
            pressed_button: None,
            last_tick: Instant::now(),
            notification: None,
            col_starts: Vec::new(),
            scanning: true,
            disks: system::get_disk_usage(),
            last_click: None,
        };
        let _ = app.rescan();
        app
    }

    fn rescan(&mut self) -> Result<(), String> {
        let path = if self.current_stack.is_empty() {
            self.scan_path.clone()
        } else {
            let mut p = self.scan_path.clone();
            for comp in &self.current_stack {
                p.push(comp);
            }
            p
        };

        match scanner::scan(&path) {
            Ok(root) => {
                self.root = root;
                self.refresh_entries();
                self.scanning = false;
                Ok(())
            }
            Err(e) => {
                self.scanning = false;
                Err(format!("Scan error: {}", e))
            }
        }
    }

    fn refresh_entries(&mut self) {
        let mut entries: Vec<Entry> = self.root.children.clone();
        scanner::sort_entries(&mut entries, self.sort_field, self.sort_order);

        let filtered = scanner::filter_entries(&entries, &self.search_query);
        self.current_entries = filtered.into_iter().map(|e| (0, e.clone())).collect();

        let len = self.current_entries.len();
        if let Some(sel) = self.table_state.selected() {
            if len == 0 {
                self.table_state.select(None);
            } else if sel >= len {
                self.table_state.select(Some(len.saturating_sub(1)));
            }
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;
            self.pressed_button = None;
            if let Some((_, until)) = self.notification {
                if Instant::now() >= until {
                    self.notification = None;
                }
            }

            let timeout = Duration::from_millis(self.refresh_rate)
                .checked_sub(self.last_tick.elapsed())
                .unwrap_or(Duration::ZERO);

            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            self.handle_key(key);
                        }
                    }
                    Event::Mouse(mouse) => self.handle_mouse(mouse),
                    _ => {}
                }
            }

            if self.last_tick.elapsed() >= Duration::from_millis(self.refresh_rate) {
                self.disks = system::get_disk_usage();
                self.last_tick = Instant::now();
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        if self.scanning {
            return;
        }

        match self.mode {
            AppMode::Search => {
                match key.code {
                    KeyCode::Esc => { self.mode = AppMode::List; self.search_query.clear(); }
                    KeyCode::Enter => { self.mode = AppMode::List; }
                    KeyCode::Backspace => { self.search_query.pop(); }
                    KeyCode::Char(c) => { self.search_query.push(c); }
                    _ => {}
                }
                self.refresh_entries();
                return;
            }
            AppMode::Detail => {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => { self.mode = AppMode::List; }
                    KeyCode::Down | KeyCode::Char('j') => self.detail_scroll = self.detail_scroll.saturating_add(1),
                    KeyCode::Up | KeyCode::Char('k') => self.detail_scroll = self.detail_scroll.saturating_sub(1),
                    _ => {}
                }
                return;
            }
            AppMode::List => {}
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('k') | KeyCode::Up => self.select_prev(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::PageUp => { for _ in 0..20 { self.select_prev(); } }
            KeyCode::PageDown => { for _ in 0..20 { self.select_next(); } }
            KeyCode::Home => self.table_state.select(Some(0)),
            KeyCode::End => {
                let len = self.current_entries.len().saturating_sub(1);
                self.table_state.select(Some(len));
            }
            KeyCode::Char('s') => self.cycle_sort(),
            KeyCode::Char('S') => {
                self.sort_order = match self.sort_order {
                    SortOrder::Asc => SortOrder::Desc,
                    SortOrder::Desc => SortOrder::Asc,
                };
                self.apply_sort();
            }
            KeyCode::Char('/') => { self.mode = AppMode::Search; self.search_query.clear(); }
            KeyCode::Enter | KeyCode::Right => self.enter_dir(),
            KeyCode::Backspace | KeyCode::Left => self.go_up(),
            KeyCode::Char('d') => {
                if self.selected_entry().is_some() {
                    self.mode = AppMode::Detail;
                    self.detail_scroll = 0;
                }
            }
            KeyCode::Char('g') => {
                self.current_stack.clear();
                self.scan_path = PathBuf::from(".");
                let _ = self.rescan();
            }
            KeyCode::Char('r') => {
                let _ = self.rescan();
            }
            KeyCode::Char('T') => self.cycle_theme(),
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.refresh_rate = (self.refresh_rate + 100).min(10000);
            }
            KeyCode::Char('-') | KeyCode::Char('_') => {
                self.refresh_rate = self.refresh_rate.saturating_sub(100).max(100);
            }
            _ => {}
        }
    }

    fn enter_dir(&mut self) {
        if let Some(entry) = self.selected_entry() {
            if entry.is_dir && !entry.children.is_empty() {
                self.current_stack.push(entry.name.clone());
                self.table_state.select(Some(0));
                self.detail_scroll = 0;
                self.scanning = true;
                let _ = self.rescan();
            }
        }
    }

    fn go_up(&mut self) {
        if !self.current_stack.is_empty() {
            self.current_stack.pop();
        } else {
            let abs = if self.scan_path.is_relative() {
                std::env::current_dir()
                    .map(|cwd| cwd.join(&self.scan_path))
                    .unwrap_or_else(|_| self.scan_path.clone())
            } else {
                self.scan_path.clone()
            };
            match abs.parent() {
                Some(parent) if parent != abs => self.scan_path = parent.to_path_buf(),
                _ => return,
            }
        }
        self.table_state.select(Some(0));
        self.detail_scroll = 0;
        self.scanning = true;
        let _ = self.rescan();
    }

    fn selected_entry(&self) -> Option<&Entry> {
        self.table_state
            .selected()
            .and_then(|i| self.current_entries.get(i))
            .map(|(_, e)| e)
    }

    fn select_prev(&mut self) {
        let i = self.table_state.selected().unwrap_or(0);
        let len = self.current_entries.len();
        self.table_state
            .select(Some(if i == 0 { len.saturating_sub(1) } else { i - 1 }));
    }

    fn select_next(&mut self) {
        let i = self.table_state.selected().unwrap_or(0);
        let len = self.current_entries.len();
        self.table_state
            .select(Some(if i >= len.saturating_sub(1) { 0 } else { i + 1 }));
    }

    fn cycle_sort(&mut self) {
        self.sort_field = match self.sort_field {
            SortField::Size => SortField::Name,
            SortField::Name => SortField::Count,
            SortField::Count => SortField::Size,
        };
        self.apply_sort();
    }

    fn apply_sort(&mut self) {
        let mut entries: Vec<Entry> = self.root.children.clone();
        scanner::sort_entries(&mut entries, self.sort_field, self.sort_order);
        let filtered = scanner::filter_entries(&entries, &self.search_query);
        self.current_entries = filtered.into_iter().map(|e| (0, e.clone())).collect();
    }

    fn theme_from_name(name: &str) -> Theme {
        for &(n, f) in THEMES {
            if n == name {
                return f();
            }
        }
        Theme::dark()
    }

    fn cycle_theme(&mut self) {
        let idx = THEMES.iter().position(|(n, _)| *n == self.theme_name);
        let next = idx.map(|i| (i + 1) % THEMES.len()).unwrap_or(0);
        let (name, _) = THEMES[next];
        self.theme_name = name.to_string();
        self.theme = Self::theme_from_name(name);
        let mut config = Config::load();
        config.settings.theme = self.theme_name.clone();
        let _ = config.save();
    }

    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        if self.scanning {
            return;
        }
        match mouse.kind {
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                let (term_w, term_h) = crossterm::terminal::size().unwrap_or((80, 24));
                let help_y = term_h.saturating_sub(4);
                let inner_w = term_w.saturating_sub(2);
                let n = 11u16;
                let per = (inner_w / n).max(6).min(10);
                let total_w = n * per;
                let x_offset = if total_w < inner_w {
                    (inner_w - total_w) / 2
                } else {
                    0
                };
                if mouse.row == help_y + 1 {
                    for i in 0..n {
                        let btn_inner_x0 = 1u16 + x_offset + i * per + 1;
                        let btn_inner_x1 = btn_inner_x0 + per - 2;
                        if mouse.column >= btn_inner_x0 && mouse.column < btn_inner_x1 {
                            self.handle_button_click(i as usize);
                            return;
                        }
                    }
                }

                let header_y = self.table_top().saturating_sub(1);
                let table_data_start = self.table_top();
                if mouse.row == header_y && self.col_starts.len() >= 5 {
                    for ci in 0..4 {
                        if mouse.column >= self.col_starts[ci] && mouse.column < self.col_starts[ci + 1] {
                            let field = match ci {
                                0 => SortField::Size,
                                1 => SortField::Name,
                                2 => SortField::Count,
                                _ => return,
                            };
                            if self.sort_field == field {
                                self.sort_order = match self.sort_order {
                                    SortOrder::Asc => SortOrder::Desc,
                                    SortOrder::Desc => SortOrder::Asc,
                                };
                            } else {
                                self.sort_field = field;
                            }
                            self.apply_sort();
                            return;
                        }
                    }
                }

                let row_clicked = mouse.row >= table_data_start
                    && mouse.row < table_data_start + self.current_entries.len() as u16;

                if row_clicked {
                    let idx = (mouse.row - table_data_start) as usize;
                    if idx < self.current_entries.len() {
                        self.table_state.select(Some(idx));

                        let is_double = self.last_click.map_or(false, |(t, r, _)| {
                            t.elapsed() < Duration::from_millis(400) && r == mouse.row
                        });
                        self.last_click = Some((Instant::now(), mouse.row, mouse.column));

                        if is_double {
                            self.enter_dir();
                        }
                    }
                }
            }
            MouseEventKind::ScrollUp => { for _ in 0..3 { self.select_prev(); } }
            MouseEventKind::ScrollDown => { for _ in 0..3 { self.select_next(); } }
            _ => {}
        }
    }

    fn handle_button_click(&mut self, idx: usize) {
        if self.scanning {
            return;
        }
        self.pressed_button = Some(idx);
        match idx {
            0 => self.cycle_sort(),
            1 => { self.mode = AppMode::Search; self.search_query.clear(); }
            2 => self.enter_dir(),
            3 => self.go_up(),
            4 => {
                if self.selected_entry().is_some() {
                    self.mode = AppMode::Detail;
                    self.detail_scroll = 0;
                }
            }
            5 => {
                self.current_stack.clear();
                self.scan_path = PathBuf::from(".");
                self.scanning = true;
                let _ = self.rescan();
            }
            6 => { let _ = self.rescan(); }
            7 => self.cycle_theme(),
            8 => self.refresh_rate = (self.refresh_rate + 100).min(10000),
            9 => self.refresh_rate = self.refresh_rate.saturating_sub(100).max(100),
            10 => self.should_quit = true,
            _ => {}
        }
    }

    fn table_top(&self) -> u16 {
        // 1 (outer border) + 3 (info gauge) + 1 (table block border) + 1 (table header) = 6
        6
    }

    fn header_text(&self) -> String {
        let sort_char = if self.sort_order == SortOrder::Desc { "↓" } else { "↑" };
        let path_display = if self.current_stack.is_empty() {
            self.scan_path.display().to_string()
        } else {
            self.current_stack.join("/")
        };
        format!(
            " hdu v{} | {:?} | {} | {}{} | {}ms ",
            env!("CARGO_PKG_VERSION"),
            self.platform,
            path_display,
            self.sort_field_label(),
            sort_char,
            self.refresh_rate,
        )
    }

    fn sort_field_label(&self) -> &'static str {
        match self.sort_field {
            SortField::Size => "Size",
            SortField::Name => "Name",
            SortField::Count => "Items",
        }
    }

    fn current_dir_size(&self) -> u64 {
        self.root.size
    }

    fn render(&mut self, f: &mut Frame) {
        let area = f.area();
        if area.width < 50 || area.height < 10 {
            let text = Paragraph::new("Terminal window is too small!")
                .style(Style::default().fg(self.theme.header).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            f.render_widget(text, area);
            return;
        }

        let outer = Block::default()
            .title(self.header_text())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.theme.border));
        let inner = outer.inner(area);
        f.render_widget(outer, area);

        let mut rows = vec![
            Constraint::Length(3),
        ];
        rows.push(Constraint::Min(0));

        if self.mode == AppMode::Search {
            rows.push(Constraint::Length(1));
        }
        rows.push(Constraint::Length(3));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(rows)
            .split(inner);

        let mut r = 0;
        self.render_info_gauge(f, chunks[r]);
        r += 1;

        self.render_entry_table(f, chunks[r]);
        r += 1;

        if self.mode == AppMode::Search {
            self.render_search(f, chunks[r]);
            r += 1;
        }

        self.render_help(f, chunks[r]);

        if let Some((ref msg, _)) = self.notification {
            let area = f.area();
            let lines: Vec<Line> = vec![Line::from(Span::styled(
                msg,
                Style::default().fg(self.theme.alert).add_modifier(Modifier::BOLD),
            ))];
            let h = 3;
            let notif_area = Rect {
                x: area.width.saturating_sub(msg.len() as u16 + 4).min(4) / 2,
                y: area.height.saturating_sub(h),
                width: (msg.len() as u16 + 4).min(area.width),
                height: h,
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.theme.alert));
            f.render_widget(Clear, notif_area);
            f.render_widget(Paragraph::new(lines).block(block).alignment(Alignment::Center), notif_area);
        }

        if self.mode == AppMode::Detail {
            self.render_detail_panel(f);
        }

        if self.scanning {
            let scan_text = Paragraph::new(" Scanning... ")
                .style(Style::default().fg(self.theme.search).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            f.render_widget(scan_text, inner);
        }
    }

    fn render_info_gauge(&self, f: &mut Frame, area: Rect) {
        let total = self.current_dir_size();
        let item_count = self.root.item_count;

        let disk = self.disks.iter().find(|d| {
            let p = self.scan_path.display().to_string();
            let mp = d.mount_point.trim_end_matches('/');
            p == mp || p.starts_with(mp)
        });

        if let Some(disk) = disk {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
                .split(area);

            let dir_info = format!(
                " {}  {} items  {} ",
                self.root.name,
                item_count,
                format_bytes(total),
            );
            let dir_block = Block::default()
                .title(dir_info)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.theme.border));
            f.render_widget(dir_block, chunks[0]);

            let used = disk.used_bytes;
            let total_bytes = disk.total_bytes;
            let pct = if total_bytes > 0 {
                (used as f64 / total_bytes as f64 * 100.0) as u16
            } else {
                0
            };
            let gauge_label = format!(
                " {}  {}%  {}/{} ",
                disk.mount_point,
                pct,
                format_bytes(used),
                format_bytes(total_bytes),
            );
            let gauge_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.theme.border));
            let inner = gauge_block.inner(chunks[1]);
            let gauge = Gauge::default()
                .gauge_style(Style::default().fg(self.theme.dir))
                .percent(pct)
                .label(gauge_label);
            f.render_widget(gauge_block, chunks[1]);
            f.render_widget(gauge, inner);
        } else {
            let text = format!(
                " {}  {} items  {} ",
                self.root.name,
                item_count,
                format_bytes(total),
            );
            let block = Block::default()
                .title(text)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.theme.border));
            f.render_widget(block, area);
        }
    }

    fn render_entry_table(&mut self, f: &mut Frame, area: Rect) {
        let table_x = area.x + 1;
        let table_w = area.width.saturating_sub(2);
        let col_w: [u16; 4] = [
            12,
            table_w.saturating_sub(36),
            10,
            12,
        ];
        let mut cs = Vec::with_capacity(5);
        let mut xp = table_x;
        for &w in &col_w { cs.push(xp); xp = xp.saturating_add(w); }
        cs.push(xp);
        self.col_starts = cs;

        let count = self.current_entries.len();
        let title = if self.search_query.is_empty() {
            format!(" Contents ({}) ", count)
        } else {
            format!(" Contents ({}/{}) ", count, self.root.children.len())
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.theme.border));
        let table_area = block.inner(area);
        f.render_widget(block, area);

        let sort_char = if self.sort_order == SortOrder::Desc { "↓" } else { "↑" };

        let sort_hdr = |name: &str, field: SortField| -> String {
            if self.sort_field == field {
                format!(" {}{} ", name, sort_char)
            } else {
                format!(" {} ", name)
            }
        };

        let widths = &[
            Constraint::Length(12),
            Constraint::Min(15),
            Constraint::Length(10),
            Constraint::Length(12),
        ];

        let headers = vec![
            sort_hdr("SIZE", SortField::Size),
            sort_hdr("NAME", SortField::Name),
            sort_hdr("ITEMS", SortField::Count),
            "  BAR".to_string(),
        ];
        let header_cells = headers
            .iter()
            .map(|h| Cell::from(h.as_str()).style(Style::default().add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1).bottom_margin(0);

        let total_size = self.current_dir_size();

        let rows: Vec<Row> = self
            .current_entries
            .iter()
            .map(|(_, entry)| {
                let name_style = if entry.is_dir {
                    Style::default().fg(self.theme.dir).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(self.theme.file)
                };

                let bar = if total_size > 0 {
                    let pct = (entry.size as f64 / total_size as f64) * 100.0;
                    let bar_len = 10usize;
                    let filled = ((pct / 100.0) * bar_len as f64).round() as usize;
                    let filled = filled.min(bar_len);
                    let color = if pct > 50.0 {
                        self.theme.large
                    } else if pct > 10.0 {
                        self.theme.medium
                    } else {
                        self.theme.small
                    };
                    let bar_str: String = (0..bar_len)
                        .map(|i| if i < filled { "█" } else { "░" })
                        .collect();
                    Span::styled(format!(" {:>5.1}% {}", pct, bar_str), Style::default().fg(color))
                } else {
                    Span::raw("")
                };

                let cells = vec![
                    Cell::from(format_size(entry.size))
                        .style(size_color(entry.size, total_size, &self.theme)),
                    Cell::from(entry.name.clone()).style(name_style),
                    Cell::from(if entry.is_dir {
                        entry.item_count.to_string()
                    } else {
                        "-".to_string()
                    }),
                    Cell::from(bar),
                ];
                Row::new(cells).height(1)
            })
            .collect();

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(
                Style::default()
                    .fg(self.theme.selected_fg)
                    .bg(self.theme.selected_bg)
                    .add_modifier(Modifier::BOLD),
            );

        let mut state = self.table_state.clone();
        f.render_stateful_widget(table, table_area, &mut state);
    }

    fn render_help(&self, f: &mut Frame, area: Rect) {
        let btns: &[(&str, &str)] = &[
            ("s", "Sort"),
            ("/", "Srch"),
            ("→", "Open"),
            ("←", "Back"),
            ("d", "Detl"),
            ("g", "Root"),
            ("r", "Rescn"),
            ("T", "Them"),
            ("+", "Rate"),
            ("-", "Rate"),
            ("q", "Quit"),
        ];

        let n = btns.len() as u16;
        let per = (area.width / n).max(6).min(10);
        let total_w = n * per;
        let x_offset = if total_w < area.width {
            (area.width - total_w) / 2
        } else {
            0
        };
        for (i, (key, label)) in btns.iter().enumerate() {
            let x = area.x + x_offset + i as u16 * per;
            let w = per.min(area.width.saturating_sub(x.saturating_sub(area.x)));
            let r = Rect { x, y: area.y, width: w, height: area.height };

            let pressed = self.pressed_button == Some(i);

            let border_style = if pressed {
                Style::default().fg(self.theme.selected_bg)
            } else {
                Style::default().fg(self.theme.border)
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style);
            let inner = block.inner(r);

            let text = if pressed {
                Line::from(vec![
                    Span::styled(*key, Style::default().fg(self.theme.selected_fg).add_modifier(Modifier::BOLD)),
                    Span::styled(*label, Style::default().fg(self.theme.selected_fg)),
                ])
            } else {
                Line::from(vec![
                    Span::styled(*key, Style::default().fg(self.theme.header).add_modifier(Modifier::BOLD)),
                    Span::styled(*label, Style::default().fg(self.theme.help)),
                ])
            };
            if pressed {
                let fill = Block::default().style(Style::default().bg(self.theme.border));
                f.render_widget(fill, inner);
            }
            let para = Paragraph::new(text).alignment(Alignment::Center);
            f.render_widget(block, r);
            f.render_widget(para, inner);
        }
    }

    fn render_search(&self, f: &mut Frame, area: Rect) {
        let text = Line::from(vec![
            Span::styled(" Search: ", Style::default().fg(self.theme.search).add_modifier(Modifier::BOLD)),
            Span::raw(&self.search_query),
            Span::styled("█", Style::default().fg(self.theme.search)),
        ]);
        f.render_widget(Paragraph::new(text), area);
    }

    fn render_detail_panel(&self, f: &mut Frame) {
        let entry = match self.selected_entry() {
            Some(e) => e,
            None => return,
        };

        let area = f.area();
        let w = area.width.min(50);
        let right = Rect {
            x: area.width.saturating_sub(w),
            y: 0,
            width: w,
            height: area.height,
        };

        let lines = vec![
            Line::from(format!(" Name:       {}", entry.name)),
            Line::from(format!(" Type:       {}", if entry.is_dir { "Directory" } else { "File" })),
            Line::from(format!(" Size:       {}", format_bytes(entry.size))),
            Line::from(format!(" Items:      {}", entry.item_count)),
            Line::from(format!(" Path:       {}", entry.path.display())),
        ];

        let block = Block::default()
            .title(format!(" {} ", entry.name))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.theme.border));

        let para = Paragraph::new(lines)
            .block(block)
            .scroll((self.detail_scroll, 0))
            .wrap(Wrap { trim: false });

        f.render_widget(Clear, right);
        f.render_widget(para, right);
    }
}

fn size_color(size: u64, total: u64, theme: &Theme) -> Style {
    if total == 0 {
        return Style::default();
    }
    let pct = size as f64 / total as f64;
    if pct > 0.5 {
        Style::default().fg(theme.large).add_modifier(Modifier::BOLD)
    } else if pct > 0.1 {
        Style::default().fg(theme.medium)
    } else {
        Style::default().fg(theme.small)
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GiB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.2} MiB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.2} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

fn format_size(bytes: u64) -> String {
    format_bytes(bytes)
}

pub fn print_scan(root: &Entry) {
    let mut entries: Vec<Entry> = root.children.clone();
    scanner::sort_entries(&mut entries, SortField::Size, SortOrder::Desc);

    println!(
        "{:<12} {:<6} {:<50} {}",
        "SIZE", "ITEMS", "NAME", "%"
    );
    println!("{}", "-".repeat(90));

    let total = root.size;
    for entry in entries.iter().take(60) {
        let pct = if total > 0 {
            (entry.size as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let prefix = if entry.is_dir { "/" } else { "" };
        println!(
            "{:<12} {:<6} {:<50} {:>5.1}%",
            format_bytes(entry.size),
            if entry.is_dir { entry.item_count.to_string() } else { "-".to_string() },
            format!("{}{}", entry.name, prefix),
            pct
        );
    }
}

pub fn print_tree(entry: &Entry, depth: usize) {
    let indent = "  ".repeat(depth);
    let prefix = if entry.is_dir { "/" } else { "" };
    println!(
        "{}{:<12} {}{}",
        indent,
        format_bytes(entry.size),
        entry.name,
        prefix
    );
    let mut children = entry.children.clone();
    scanner::sort_entries(&mut children, SortField::Size, SortOrder::Desc);
    for child in children {
        print_tree(&child, depth + 1);
    }
}

pub fn watch_disk_usage(path: &Path, rate: u64) -> io::Result<()> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = AppUi::new(
        crate::backend::detect_platform(),
        &Config::load(),
        rate,
    );
    app.scan_path = path.to_path_buf();
    let res = app.run(&mut terminal);

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}
