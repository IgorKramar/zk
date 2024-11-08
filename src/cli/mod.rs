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
        #[arg(short = 'g', long = "tags")]
        tags: Option<Vec<String>>,
        
        /// Поиск по заголовку
        #[arg(short = 't', long = "title")]
        title: Option<String>,
        
        /// Поиск по содержимому
        #[arg(short = 'c', long = "content")]
        content: Option<String>,

        /// Использовать регулярные выражения для поиска
        #[arg(short = 'r', long = "regex")]
        use_regex: bool,
    },
    /// Управление связями между заметками
    Link {
        #[command(subcommand)]
        command: LinkCommands,
    },
    /// Открыть заметку во внешнем приложении
    Open {
        /// ID заметки
        #[arg(value_name = "ID")]
        id: Option<String>,
        
        /// Приложение для открытия
        #[arg(long)]
        app: Option<String>,
    },
    /// Запустить интерактивный режим
    Tui,
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

#[derive(Subcommand)]
pub enum LinkCommands {
    /// Добавить связь между заметками
    Add {
        /// ID исходной заметки
        from: String,
        /// ID целевой заметки
        to: String,
        /// Описание связи (опционально)
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Удалить связь между заметками
    Remove {
        /// ID исходной заметки
        from: String,
        /// ID целевой заметки
        to: String,
    },
    /// Показать связи заметки
    Show {
        /// ID заметки
        id: String,
        /// Показать также обратные связи
        #[arg(short, long)]
        backlinks: bool,
    },
} 