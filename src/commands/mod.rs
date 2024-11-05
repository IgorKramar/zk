use std::env;
use crate::cli::{Commands, ConfigCommands};
use crate::config::{self, Config};
use crate::notes;

pub fn handle_command(command: Commands) {
    match command {
        Commands::Init => handle_init(),
        Commands::New { title } => handle_new(&title),
        Commands::Config(config_cmd) => handle_config(config_cmd),
    }
}

fn handle_init() {
    let current_dir = env::current_dir().expect("Не удалось получить текущую директорию");
    match config::Config::init(&current_dir) {
        Ok(_) => println!("База знаний успешно инициализирована"),
        Err(e) => eprintln!("Ошибка при инициализации: {}", e),
    }
}

fn handle_new(title: &str) {
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Ошибка загрузки конфигурации: {}", e);
            return;
        }
    };

    match notes::create_note(&config, title) {
        Ok(()) => println!("Заметка успешно создана: {}", title),
        Err(e) => eprintln!("Ошибка при создании заметки: {}", e),
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