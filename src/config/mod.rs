use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use crate::templates::DEFAULT_NOTE_TEMPLATE;

const CONFIG_FILENAME: &str = ".zkrc";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Путь к директории с заметками
    pub notes_dir: PathBuf,
    /// Шаблон имени файла для новых заметок
    pub filename_template: String,
    #[serde(default = "default_template_name")]
    pub default_template: String,
    #[serde(default)]
    pub active_note: Option<String>,
}

fn default_template_name() -> String {
    "default".to_string()
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Toml(toml::ser::Error),
    Config(config::ConfigError),
    Message(String),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(err: toml::ser::Error) -> Self {
        ConfigError::Toml(err)
    }
}

impl From<config::ConfigError> for ConfigError {
    fn from(err: config::ConfigError) -> Self {
        ConfigError::Config(err)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "Ошибка ввода/вывода: {}", e),
            ConfigError::Toml(e) => write!(f, "Ошибка сериализации TOML: {}", e),
            ConfigError::Config(e) => write!(f, "Ошибка конфигурации: {}", e),
            ConfigError::Message(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            notes_dir: PathBuf::from("notes"),
            filename_template: "{timestamp}-{title}".to_string(),
            default_template: default_template_name(),
            active_note: None,
        }
    }
}

impl Config {
    /// Загружает конфигурацию из текущей директории или её родителей
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::find_config()?;
        
        let settings = config::Config::builder()
            .add_source(config::File::with_name(
                config_path.to_str().ok_or_else(|| 
                    ConfigError::Message("Некорректный путь к файлу конфигурации".to_string())
                )?
            ).format(config::FileFormat::Toml))
            .build()?;

        Ok(settings.try_deserialize()?)
    }

    /// Инициализирует новую конфигурацию в текущей директории
    pub fn init(base_dir: &Path) -> Result<Self, ConfigError> {
        let config = Config::default();
        let config_path = base_dir.join(CONFIG_FILENAME);
        
        if config_path.exists() {
            return Err(ConfigError::Message("Конфигурация уже существует".to_string()));
        }

        // Создаём директорию для заметок
        fs::create_dir_all(base_dir.join(&config.notes_dir))?;
        
        // Создаём директорию для шаблонов
        fs::create_dir_all(base_dir.join(config.templates_dir()))?;
        
        // Создаём шаблон по умолчанию
        let default_template = base_dir
            .join(config.templates_dir())
            .join("default.md");
        fs::write(&default_template, DEFAULT_NOTE_TEMPLATE)?;

        let toml = toml::to_string_pretty(&config)?;
        fs::write(config_path, toml)?;

        Ok(config)
    }

    /// Ищет файл конфигурации в текущей директории и выше
    fn find_config() -> Result<PathBuf, ConfigError> {
        let current_dir = std::env::current_dir()?;
        let mut current = Some(current_dir.as_path());

        while let Some(dir) = current {
            let config_path = dir.join(CONFIG_FILENAME);
            if config_path.exists() {
                return Ok(config_path);
            }
            current = dir.parent();
        }

        Err(ConfigError::Message(
            "Файл конфигурации .zkrc не найден. Используйте 'zk init' для создания новой базы знаний".to_string()
        ))
    }

    /// Сохраняет текущую конфигурацию в файл
    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = Self::find_config()?;
        let toml = toml::to_string_pretty(&self)?;
        fs::write(&config_path, toml)?;
        Ok(())
    }

    /// Устанавливает значение параметра конфигурации
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), ConfigError> {
        match key {
            "notes_dir" => {
                self.notes_dir = PathBuf::from(value);
            },
            "filename_template" => {
                self.filename_template = value.to_string();
            },
            _ => return Err(ConfigError::Message(format!(
                "Неизвестный параметр конфигурации: {}", key
            ))),
        }
        self.save()?;
        Ok(())
    }

    /// Возвращает строковое представление конфигурации
    pub fn to_string_pretty(&self) -> Result<String, ConfigError> {
        Ok(toml::to_string_pretty(&self)?)
    }

    pub fn templates_dir(&self) -> PathBuf {
        PathBuf::from("_templates")
    }

    pub fn get_active_note(&self) -> Option<&str> {
        self.active_note.as_deref()
    }

    pub fn set_active_note(&mut self, id: &str) -> Result<(), ConfigError> {
        self.active_note = Some(id.to_string());
        self.save()
    }
} 