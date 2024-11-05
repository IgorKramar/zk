use std::env;
use crate::cli::{Commands, ConfigCommands, TemplateCommands, TagCommands};
use crate::config::{self, Config};
use crate::notes;
use crate::templates::TemplateEngine;
use crate::editor;
use crate::tags;
use crate::notes::store::{NoteStore, SearchQuery};
use chrono::Local;

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
                            Ok(()) => println!("Создан новый шаблон '{}'", name),
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
                        println!("Связи: {}", metadata.links.join(", "));
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
        Commands::Search { tags, title, content } => {
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

            let query = SearchQuery { tags, title, content };
            match store.search(&query) {
                Ok(notes) => {
                    if notes.is_empty() {
                        println!("Заметки не найдены");
                    } else {
                        println!("Найдены заметки:");
                        for (note, content) in notes {
                            println!("\nID: {} ({})", note.id, note.created.format("%Y-%m-%d %H:%M"));
                            println!("Заголовок: {}", note.title);
                            if !note.tags.is_empty() {
                                println!("Теги: {}", note.tags.join(", "));
                            }
                            if let Some(desc) = note.description {
                                println!("Описание: {}", desc);
                            }
                            
                            // Показываем фрагмент содержимого, если искали по нему
                            if query.content.is_some() {
                                let preview = content
                                    .lines()
                                    .filter(|line| !line.trim().is_empty())
                                    .take(3)
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                println!("Фрагмент:\n{}", preview);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Ошибка при поиске заметок: {}", e),
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
                Ok(()) => println!("Параметр {} успешно обновлен", key),
                Err(e) => eprintln!("Ошибка обновления параметра: {}", e),
            }
        }
    }
} 