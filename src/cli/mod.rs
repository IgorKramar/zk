use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        #[arg(short = 't', long)]
        title: String,
        /// Имя шаблона для заметки
        #[arg(short = 'T', long)]
        template: Option<String>,
    },
    /// Управление тегами
    #[command(subcommand)]
    Tag(TagCommands),
    /// Инициализировать новую базу знаний в текущей директории
    Init,
    /// Управление конфигурацией
    #[command(subcommand)]
    Config(ConfigCommands),
    /// Управление шаблонами
    #[command(subcommand)]
    Template(TemplateCommands),
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

#[derive(Subcommand)]
pub enum TemplateCommands {
    /// Показать список доступных шаблонов
    List,
    /// Создать новый шаблон
    New {
        /// Имя шаблона
        name: String,
    },
    /// Редактировать существующий шаблон
    Edit {
        /// Имя шаблона
        name: String,
    },
    /// Показать содержимое шаблона
    Show {
        /// Имя шаблона
        name: String,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
    /// Добавить теги к заметке
    Add {
        /// Путь к заметке
        path: PathBuf,
        /// Теги для добавления
        #[arg(required = true)]
        tags: Vec<String>,
    },
    /// Удалить теги из заметки
    Remove {
        /// Путь к заметке
        path: PathBuf,
        /// Теги для удаления
        #[arg(required = true)]
        tags: Vec<String>,
    },
    /// Показать все теги в базе знаний
    List,
} 