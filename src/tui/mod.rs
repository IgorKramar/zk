use std::io;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use crate::notes::store::NoteStore;
use crate::config::Config;
use crate::notes::metadata::NoteMetadata;

pub struct App {
    notes: Vec<(String, NoteMetadata)>, // (id, metadata)
    selected: Option<usize>,
    store: NoteStore,
}

impl App {
    pub fn new(config: &Config) -> io::Result<App> {
        let store = NoteStore::new(config.notes_dir.clone())?;
        let notes = store.list()?;

        Ok(App {
            notes,
            selected: None,
            store,
        })
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

                // Заголовок
                let title = Paragraph::new("Zk - База знаний (q - выход, ↑↓ - навигация, Enter - открыть)")
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(title, chunks[0]);

                // Список заметок
                let items: Vec<ListItem> = self.notes
                    .iter()
                    .map(|(id, note)| {
                        ListItem::new(format!("{}: {}", id, note.title))
                    })
                    .collect();

                let notes = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Заметки"))
                    .highlight_style(Style::default().bg(Color::DarkGray));
                f.render_widget(notes, chunks[1]);

                // Статус
                let status = match self.selected {
                    Some(i) => format!("Вырана заметка: {}", self.notes[i].0),
                    None => "Нет выбранной заметки".to_string(),
                };
                let status = Paragraph::new(status)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(status, chunks[2]);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => {
                            if let Some(selected) = self.selected {
                                self.selected = Some(selected.saturating_sub(1));
                            } else {
                                self.selected = Some(0);
                            }
                        }
                        KeyCode::Down => {
                            if let Some(selected) = self.selected {
                                if selected < self.notes.len().saturating_sub(1) {
                                    self.selected = Some(selected + 1);
                                }
                            } else {
                                self.selected = Some(0);
                            }
                        }
                        KeyCode::Enter => {
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