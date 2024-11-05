use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zk")]
#[command(about = "CLI для управления заметками в стиле Zettelkasten")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Создать новую заметку
    New {
        /// Заголовок заметки
        #[arg(short, long)]
        title: String,
    },
    /// Инициализировать новую базу знаний в текущей директории
    Init,
    /// Управление конфигурацией
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Показать текущую конфигурацию
    Show,
    /// Установить значение параметра конфигурации
    Set {
        /// Имя параметра (notes_dir, filename_template)
        key: String,
        /// Новое значение параметра
        value: String,
    },
} 