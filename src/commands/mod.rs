pub mod open;
use crate::tui::App;

use std::env;
use crate::cli::{Commands, ConfigCommands, TemplateCommands, TagCommands, LinkCommands};
use crate::config::{self, Config};
use crate::notes;
use crate::templates::TemplateEngine;
use crate::editor;
use crate::tags;
use crate::notes::store::{NoteStore, SearchQuery, SearchPattern};
use chrono::Local;
use colored::*;

pub fn handle_command(command: Commands) {
    match command {
        Commands::Init => handle_init(),
        Commands::New { title, template } => {
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Ошибка загрузки конфигурации: {}", e);
                    return;
                }
            };

            match notes::create_note(&config, &title, template.as_deref()) {
                Ok(()) => println!("Заметка успешно создана: {}", title),
                Err(e) => eprintln!("Ошибка при создании заметки: {}", e),
            }
        }
        Commands::Config(config_cmd) => handle_config(config_cmd),
        Commands::Template(template_cmd) => {
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Ошибка загрузки конфигурации: {}", e);
                    return;
                }
            };

            let template_engine = match TemplateEngine::new(&config) {
                Ok(engine) => engine,
                Err(e) => {
                    eprintln!("Ошибка инициализации движка шаблонов: {}", e);
                    return;
                }
            };

            match template_cmd {
                TemplateCommands::New { name } => {
                    match template_engine.create_template(&name) {
                        Ok(()) => println!("Шаблон '{}' создан", name),
                        Err(e) => eprintln!("Ошибка при создании шаблона: {}", e),
                    }
                }
                TemplateCommands::List => {
                    let templates = template_engine.list_templates();
                    if templates.is_empty() {
                        println!("Шаблоны не найдены");
                    } else {
                        println!("Доступные шаблоны:");
                        for template in templates {
                            println!("  - {}", template);
                        }
                    }
                }
                TemplateCommands::Edit { name } => {
                    let path = template_engine.get_template_path(&name);
                    if !path.exists() {
                        match template_engine.create_template(&name) {
                            Ok(()) => println!("Создан новый аблон '{}'", name),
                            Err(e) => {
                                eprintln!("Ошибка при создании шаблона: {}", e);
                                return;
                            }
                        }
                    }
                    
                    if let Err(e) = editor::edit_file(&path) {
                        eprintln!("Ошибка при открытии редактора: {}", e);
                    }
                }
                TemplateCommands::Show { name } => {
                    match template_engine.get_template_content(&name) {
                        Ok(content) => println!("{}", content),
                        Err(e) => eprintln!("Ошибка при чтении шаблона: {}", e),
                    }
                }
            }
        }
        Commands::Tag(tag_cmd) => {
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Ошибка загрузки конфигурации: {}", e);
                    return;
                }
            };

            let store = match NoteStore::new(config.notes_dir.clone()) {
                Ok(store) => store,
                Err(e) => {
                    eprintln!("Ошибка инициализации хранилища заметок: {}", e);
                    return;
                }
            };

            match tag_cmd {
                TagCommands::Add { id, tags } => {
                    match store.get_path(&id) {
                        Some(path) => match tags::add_tags(path, &tags) {
                            Ok(()) => println!("Теги успешно добавлены"),
                            Err(e) => eprintln!("Ошибка при добавлении тегов: {}", e),
                        },
                        None => eprintln!("Заметка с ID '{}' не найдена", id),
                    }
                }
                TagCommands::Remove { id, tags } => {
                    match store.get_path(&id) {
                        Some(path) => match tags::remove_tags(path, &tags) {
                            Ok(()) => println!("Теги успешно удалены"),
                            Err(e) => eprintln!("Ошибка при удалении тегов: {}", e),
                        },
                        None => eprintln!("Заметка с ID '{}' не найдена", id),
                    }
                }
                TagCommands::List => {
                    match tags::list_all_tags(&config.notes_dir) {
                        Ok(tags) => {
                            if tags.is_empty() {
                                println!("Теги не найдены");
                            } else {
                                println!("Найденные теги:");
                                for tag in tags {
                                    println!("  - {}", tag);
                                }
                            }
                        }
                        Err(e) => eprintln!("Ошибка при получении списка тегов: {}", e),
                    }
                }
            }
        }
        Commands::Show { id } => {
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Ошибка загрузки конфигурации: {}", e);
                    return;
                }
            };

            let store = match NoteStore::new(config.notes_dir) {
                Ok(store) => store,
                Err(e) => {
                    eprintln!("Ошибка инициализации хранилища заметок: {}", e);
                    return;
                }
            };

            match store.get_metadata(&id) {
                Ok(Some(metadata)) => {
                    println!("ID: {}", metadata.id);
                    println!("Заголовок: {}", metadata.title);
                    println!("Создана: {}", metadata.created.with_timezone(&Local));
                    if !metadata.tags.is_empty() {
                        println!("Теги: {}", metadata.tags.join(", "));
                    }
                    if !metadata.links.is_empty() {
                        println!("Связи: {}", metadata.links.iter().map(|link| link.to_string()).collect::<Vec<_>>().join(", "));
                    }
                    if let Some(desc) = metadata.description {
                        println!("Описание: {}", desc);
                    }
                }
                Ok(None) => {
                    eprintln!("Заметка с ID '{}' не найдена", id);
                }
                Err(e) => {
                    eprintln!("Ошибка при чтении заметки: {}", e);
                }
            }
        }
        Commands::Search { tags, title, content, use_regex } => {
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Ошибка загрузки конфигурации: {}", e);
                    return;
                }
            };

            let store = match NoteStore::new(config.notes_dir) {
                Ok(store) => store,
                Err(e) => {
                    eprintln!("Ошибка инициализации хранилища заметок: {}", e);
                    return;
                }
            };

            let title_pattern = if let Some(title) = title {
                match SearchPattern::new(title, use_regex) {
                    Ok(pattern) => Some(pattern),
                    Err(e) => {
                        eprintln!("Ошибка в регулярном выражении для заголовка: {}", e);
                        return;
                    }
                }
            } else {
                None
            };

            let content_pattern = if let Some(content) = content {
                match SearchPattern::new(content, use_regex) {
                    Ok(pattern) => Some(pattern),
                    Err(e) => {
                        eprintln!("Ошибка в регулярном выражении для содержимого: {}", e);
                        return;
                    }
                }
            } else {
                None
            };

            let query = SearchQuery {
                tags,
                title: title_pattern,
                content: content_pattern,
            };

            match store.search(&query) {
                Ok(notes) => {
                    if notes.is_empty() {
                        println!("Заметки не найдены");
                    } else {
                        println!("Найдены заметки:");
                        for note in notes {
                            println!("\nID: {} ({})", 
                                note.metadata.id, 
                                note.metadata.created.format("%Y-%m-%d %H:%M")
                            );
                            
                            // Подсвечиваем совпадения в заголовке
                            let title = highlight_matches(&note.metadata.title, &note.title_matches);
                            println!("Заголовок: {}", title);

                            if !note.metadata.tags.is_empty() {
                                println!("Теги: {}", note.metadata.tags.join(", "));
                            }

                            // Показываем фрагмент с совпаднием
                            if !note.content_matches.is_empty() {
                                for &(start, end) in &note.content_matches {
                                    let preview_start = start.saturating_sub(50);
                                    let preview_end = (end + 50).min(note.content.len());
                                    let preview = &note.content[preview_start..preview_end];
                                    
                                    let relative_match = (start - preview_start, end - preview_start);
                                    let highlighted = highlight_matches(preview, &[relative_match]);
                                    
                                    println!("Совпадение: ...{}...", highlighted);
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Ошибка при поиске заметок: {}", e),
            }
        }
        Commands::Link { command } => {
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Ошибка загрузки конфигурации: {}", e);
                    return;
                }
            };

            let mut store = match NoteStore::new(config.notes_dir) {
                Ok(store) => store,
                Err(e) => {
                    eprintln!("Ошибка инициализации хранилища заметок: {}", e);
                    return;
                }
            };

            match command {
                LinkCommands::Add { from, to, description } => {
                    match store.add_link(&from, &to, description) {
                        Ok(()) => println!("Связь добавлена"),
                        Err(e) => eprintln!("Ошибка при добавлении с��язи: {}", e),
                    }
                }
                LinkCommands::Remove { from, to } => {
                    match store.remove_link(&from, &to) {
                        Ok(()) => println!("Связь удалена"),
                        Err(e) => eprintln!("Ошибка при удалении связи: {}", e),
                    }
                }
                LinkCommands::Show { id, backlinks } => {
                    match store.get_links(&id, backlinks) {
                        Ok(links) => {
                            if links.is_empty() {
                                println!("Связи не найдены");
                            } else {
                                println!("Связи для заметки {}:", id);
                                for link in links {
                                    let direction = if link.from == id {
                                        "→"
                                    } else {
                                        "←"
                                    };
                                    
                                    let other_note = if link.from == id {
                                        &link.to
                                    } else {
                                        &link.from
                                    };

                                    if let Some(desc) = link.description {
                                        println!("  {} {} ({})", direction, other_note, desc);
                                    } else {
                                        println!("  {} {}", direction, other_note);
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("Ошибка при получении связей: {}", e),
                    }
                }
            }
        }
        Commands::Open { id, app } => {
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Ошибка загрузки конфигурации: {}", e);
                    return;
                }
            };

            match open::open_note(id, app, &config) {
                Ok(()) => (),
                Err(e) => eprintln!("Ошибка при открытии заметки: {}", e),
            }
        }
        Commands::Tui => {
            let config = match Config::load() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Ошибка загрузки конфигурации: {}", e);
                    return;
                }
            };

            match App::new(&config) {
                Ok(mut app) => {
                    if let Err(e) = app.run() {
                        eprintln!("Ошибка в TUI: {}", e);
                    }
                }
                Err(e) => eprintln!("Ошибка при инициализации TUI: {}", e),
            }
        }
    }
}

fn handle_init() {
    let current_dir = env::current_dir().expect("Не удалось получить текущую директорию");
    match config::Config::init(&current_dir) {
        Ok(_) => println!("База знаний успешно инициализирована"),
        Err(e) => eprintln!("Ошибка при инициализации: {}", e),
    }
}

fn handle_config(config_cmd: ConfigCommands) {
    let mut config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Ошибка загрузки конфигурации: {}", e);
            return;
        }
    };

    match config_cmd {
        ConfigCommands::Show => {
            match config.to_string_pretty() {
                Ok(config_str) => println!("{}", config_str),
                Err(e) => eprintln!("Ошибка отображения конфигурации: {}", e),
            }
        }
        ConfigCommands::Set { key, value } => {
            match config.set(&key, &value) {
                Ok(()) => println!("Параметр {} усешно обновлен", key),
                Err(e) => eprintln!("Ошибка обновления параметра: {}", e),
            }
        }
    }
}

fn highlight_matches(text: &str, matches: &[(usize, usize)]) -> String {
    if matches.is_empty() {
        return text.to_string();
    }

    let mut result = String::new();
    let mut last_end = 0;

    for &(start, end) in matches {
        // Добавляем текст до совпадения
        result.push_str(&text[last_end..start]);
        // Добавляем подсвеченное совпадение
        result.push_str(&text[start..end].on_yellow().black().to_string());
        last_end = end;
    }
    // Добавляем оставшийся текст
    result.push_str(&text[last_end..]);

    result
} 