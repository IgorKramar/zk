use std::fs;
use std::path::PathBuf;
use tera::{Tera, Context};
use chrono::Local;
use crate::notes::metadata::NoteMetadata;
use crate::config::Config;

pub const DEFAULT_NOTE_TEMPLATE: &str = r#"---
title: {{ metadata.title }}
created: {{ metadata.created | date(format="%Y-%m-%d %H:%M:%S %z") }}
tags: []
links: []
---

# {{ metadata.title }}

"#;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(config: &Config) -> Result<Self, tera::Error> {
        let mut tera = Tera::default();
        
        // Добавляем фильтр для форматирования даты
        tera.register_filter("date", date_filter);

        let templates_dir = config.templates_dir();
        if templates_dir.exists() {
            let files = fs::read_dir(&templates_dir)
                .map_err(|e| tera::Error::msg(format!("Ошибка чтения директории шаблонов: {}", e)))?;

            for entry in files {
                let entry = entry.map_err(|e| tera::Error::msg(format!("Ошибка чтения файла: {}", e)))?;
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    let name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .ok_or_else(|| tera::Error::msg("Некорректное имя файла"))?;
                    let content = fs::read_to_string(&path)
                        .map_err(|e| tera::Error::msg(format!("Ошибка чтения шаблона: {}", e)))?;
                    tera.add_raw_template(&format!("{}.md", name), &content)?;
                }
            }
        }

        tera.add_raw_template("default.md", DEFAULT_NOTE_TEMPLATE)?;

        Ok(TemplateEngine { tera })
    }

    pub fn create_template(config: &Config, name: &str) -> std::io::Result<PathBuf> {
        let templates_dir = config.templates_dir();
        fs::create_dir_all(&templates_dir)?;
        
        let template_path = templates_dir.join(format!("{}.md", name));
        if template_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Шаблон с таким именем уже существует"
            ));
        }

        fs::write(&template_path, DEFAULT_NOTE_TEMPLATE)?;
        Ok(template_path)
    }

    pub fn get_template_path(config: &Config, name: &str) -> std::io::Result<PathBuf> {
        let template_path = config.templates_dir().join(format!("{}.md", name));
        if !template_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Шаблон не найден"
            ));
        }

        Ok(template_path)
    }

    pub fn get_template_content(config: &Config, name: &str) -> std::io::Result<String> {
        let template_path = Self::get_template_path(config, name)?;
        fs::read_to_string(template_path)
    }

    pub fn list_templates(&self) -> Vec<String> {
        self.tera.get_template_names()
            .map(|name| name.trim_end_matches(".md").to_string())
            .collect()
    }

    pub fn render_note(&self, metadata: &NoteMetadata, template_name: &str) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("metadata", metadata);
        
        // Добавляем .md к имени шаблона, если его нет
        let template_name = if template_name.ends_with(".md") {
            template_name.to_string()
        } else {
            format!("{}.md", template_name)
        };

        self.tera.render(&template_name, &context)
    }
}

fn date_filter(value: &tera::Value, args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let datetime = value.as_i64().ok_or_else(|| {
        tera::Error::msg("Значение даты должно быть целым числом (timestamp)")
    })?;

    let format = args.get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d %H:%M:%S %z");

    let dt = chrono::DateTime::from_timestamp(datetime, 0)
        .ok_or_else(|| tera::Error::msg("Некорректное значение timestamp"))?
        .with_timezone(&Local);

    Ok(tera::Value::String(dt.format(format).to_string()))
} 