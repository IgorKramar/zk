use std::env;
use crate::cli::{Commands, ConfigCommands, TemplateCommands, TagCommands};
use crate::config::{self, Config};
use crate::notes;
use crate::templates::TemplateEngine;
use crate::editor;
use crate::tags;

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

            match template_cmd {
                TemplateCommands::List => {
                    match TemplateEngine::new(&config) {
                        Ok(engine) => {
                            println!("Доступные шаблоны:");
                            for template in engine.list_templates() {
                                println!("  - {}", template);
                            }
                        }
                        Err(e) => eprintln!("Ошибка при загрузке шаблонов: {}", e),
                    }
                }
                TemplateCommands::New { name } => {
                    match TemplateEngine::create_template(&config, &name) {
                        Ok(path) => {
                            println!("Создан новый шаблон: {}", path.display());
                            if let Err(e) = editor::edit_file(&path) {
                                eprintln!("Ошибка при открытии редактора: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Ошибка при создании шаблона: {}", e),
                    }
                }
                TemplateCommands::Edit { name } => {
                    match TemplateEngine::get_template_path(&config, &name) {
                        Ok(path) => {
                            if let Err(e) = editor::edit_file(&path) {
                                eprintln!("Ошибка при открытии редактора: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Ошибка: {}", e),
                    }
                }
                TemplateCommands::Show { name } => {
                    match TemplateEngine::get_template_content(&config, &name) {
                        Ok(content) => println!("{}", content),
                        Err(e) => eprintln!("Ошибка: {}", e),
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

            match tag_cmd {
                TagCommands::Add { path, tags } => {
                    match tags::add_tags(&path, &tags) {
                        Ok(()) => println!("Теги успешно добавлены"),
                        Err(e) => eprintln!("Ошибка при добавлении тегов: {}", e),
                    }
                }
                TagCommands::Remove { path, tags } => {
                    match tags::remove_tags(&path, &tags) {
                        Ok(()) => println!("Теги успешно удалены"),
                        Err(e) => eprintln!("Ошибка при удалении тегов: {}", e),
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