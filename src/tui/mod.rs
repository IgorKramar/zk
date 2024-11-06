use crate::config::Config;
use crate::notes::metadata::NoteMetadata;
use crate::notes::store::NoteStore;
use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Span,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::io;

#[derive(PartialEq)]
enum Mode {
    List,
    Details,
}

pub struct App {
    notes: Vec<(String, NoteMetadata)>, // (id, metadata)
    selected: Option<usize>,
    store: NoteStore,
    mode: Mode,
}

impl App {
    pub fn new(config: &Config) -> io::Result<App> {
        let store = NoteStore::new(config.notes_dir.clone())?;
        let notes = store.list()?;

        Ok(App {
            notes,
            selected: None,
            store,
            mode: Mode::List,
        })
    }

    fn render_list(&self, f: &mut ratatui::Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .notes
            .iter()
            .map(|(id, note)| {
                let title_line = Line::from(vec![Span::styled(
                    &note.title,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]);

                let tags_style = if note.tags.is_empty() {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::Yellow)
                };

                let info_spans = vec![
                    Span::styled(id.to_string(), Style::default().fg(Color::DarkGray)),
                    Span::raw(" [".to_string()),
                    if note.tags.is_empty() {
                        Span::styled("нет тегов".to_string(), tags_style)
                    } else {
                        Span::styled(note.tags.join(", "), tags_style)
                    },
                    Span::raw("]".to_string()),
                ];

                let info_line = Line::from(info_spans);
                ListItem::new(vec![title_line, info_line])
            })
            .collect();

        let notes = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Заметки")
                    .border_style(Style::default().fg(Color::White)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        let mut state = ListState::default();
        state.select(self.selected);
        f.render_stateful_widget(notes, area, &mut state);
    }

    fn render_details(&self, f: &mut ratatui::Frame, area: Rect) {
        if let Some(selected) = self.selected {
            let (id, metadata) = &self.notes[selected];

            let tags_text = if metadata.tags.is_empty() {
                "нет тегов".to_string()
            } else {
                metadata.tags.join(", ")
            };

            let details = vec![
                Line::from(vec![
                    Span::raw("ID: "),
                    Span::styled(id, Style::default().fg(Color::DarkGray)),
                ]),
                Line::from(vec![
                    Span::raw("Заголовок: "),
                    Span::styled(
                        &metadata.title,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("Создана: "),
                    Span::styled(
                        metadata.created.with_timezone(&Local).to_string(),
                        Style::default().fg(Color::Yellow),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("Теги: "),
                    if metadata.tags.is_empty() {
                        Span::styled("нет тегов", Style::default().fg(Color::DarkGray))
                    } else {
                        Span::styled(&tags_text, Style::default().fg(Color::Yellow))
                    },
                ]),
                Line::raw(""),
                Line::from(vec![Span::styled(
                    "Связи:",
                    Style::default().add_modifier(Modifier::BOLD),
                )]),
            ];

            let mut content = details;

            if metadata.links.is_empty() {
                content.push(Line::from(vec![Span::styled(
                    "  нет связей",
                    Style::default().fg(Color::DarkGray),
                )]));
            } else {
                for link in &metadata.links {
                    content.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(&link.from, Style::default().fg(Color::Cyan)),
                        Span::raw(" -> "),
                        Span::styled(&link.to, Style::default().fg(Color::Cyan)),
                    ]));

                    if let Some(desc) = &link.description {
                        content.push(Line::from(vec![
                            Span::raw("    "),
                            Span::styled(desc, Style::default().fg(Color::Yellow)),
                        ]));
                    }
                }
            }

            let paragraph = Paragraph::new(content).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Детали заметки")
                    .border_style(Style::default().fg(Color::White)),
            );

            f.render_widget(paragraph, area);
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ])
                    .split(f.size());

                // Заголовок с инструкциями
                let title = match self.mode {
                    Mode::List => {
                        "Zk - База знаний (q - выход, ↑↓ - навигация, Enter - открыть, d - детали)"
                    }
                    Mode::Details => "Детали заметки (q - выход, Esc - вернуться к списку)",
                };
                let title = Paragraph::new(title).block(Block::default().borders(Borders::ALL));
                f.render_widget(title, chunks[0]);

                match self.mode {
                    Mode::List => {
                        self.render_list(f, chunks[1]);
                    }
                    Mode::Details => {
                        self.render_details(f, chunks[1]);
                    }
                }

                // Статус
                let status = match self.selected {
                    Some(i) => format!("Выбрана заметка: {}", self.notes[i].0),
                    None => "Нет выбранной заметки".to_string(),
                };
                let status = Paragraph::new(status).block(Block::default().borders(Borders::ALL));
                f.render_widget(status, chunks[2]);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('d') => {
                            if self.selected.is_some() {
                                self.mode = if self.mode == Mode::List {
                                    Mode::Details
                                } else {
                                    Mode::List
                                };
                            }
                        }
                        KeyCode::Esc => self.mode = Mode::List,
                        KeyCode::Up => {
                            if self.mode == Mode::List {
                                if let Some(selected) = self.selected {
                                    self.selected = Some(selected.saturating_sub(1));
                                } else {
                                    self.selected = Some(0);
                                }
                            }
                        }
                        KeyCode::Down => {
                            if self.mode == Mode::List {
                                if let Some(selected) = self.selected {
                                    if selected < self.notes.len().saturating_sub(1) {
                                        self.selected = Some(selected + 1);
                                    }
                                } else {
                                    self.selected = Some(0);
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if self.mode == Mode::List {
                                if let Some(selected) = self.selected {
                                    let note_id = &self.notes[selected].0;
                                    if let Some(path) = self.store.get_path(note_id) {
                                        terminal.clear()?;
                                        disable_raw_mode()?;
                                        io::stdout().execute(LeaveAlternateScreen)?;
                                        crate::editor::edit_file(&path)?;
                                        enable_raw_mode()?;
                                        io::stdout().execute(EnterAlternateScreen)?;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        io::stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
