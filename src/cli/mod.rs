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
        #[arg(short = 't', long)]
        title: String,
        /// Имя шаблона для заметки
        #[arg(short = 'T', long)]
        template: Option<String>,
    },
    /// Показать информацию о заметке
    Show {
        /// ID заметки
        id: String,
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
    /// Поиск заметок
    Search {
        /// Теги для поиска (все указанные теги должны быть в заметке)
        #[arg(short, long)]
        tags: Vec<String>,
    },
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
    /// Покаать содержимое шаблона
    Show {
        /// Имя шаблона
        name: String,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
    /// Добавить теги к заметке
    Add {
        /// ID заметки
        id: String,
        /// Теги для добавления
        #[arg(required = true)]
        tags: Vec<String>,
    },
    /// Удалить теги из заметки
    Remove {
        /// ID заметки
        id: String,
        /// Теги для удаления
        #[arg(required = true)]
        tags: Vec<String>,
    },
    /// Показать все теги в базе знаний
    List,
} 