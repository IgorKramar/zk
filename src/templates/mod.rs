use std::fs;
use std::path::PathBuf;
use tera::{Tera, Context};
use crate::notes::metadata::NoteMetadata;
use crate::config::Config;

pub const DEFAULT_NOTE_TEMPLATE: &str = r#"---
title: {{ title }}
created: {{ created }}
id: {{ id }}
tags: []
links: []
---

# {{ title }}

"#;

pub struct TemplateEngine {
    tera: Tera,
    templates_dir: PathBuf,
}

impl TemplateEngine {
    pub fn new(config: &Config) -> Result<Self, tera::Error> {
        let mut tera = Tera::default();
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

        Ok(TemplateEngine { 
            tera,
            templates_dir,
        })
    }

    pub fn create_template(&self, name: &str) -> std::io::Result<()> {
        if !self.templates_dir.exists() {
            fs::create_dir_all(&self.templates_dir)?;
        }

        let template_path = self.templates_dir.join(format!("{}.md", name));
        fs::write(template_path, DEFAULT_NOTE_TEMPLATE)?;
        Ok(())
    }

    pub fn get_template_path(&self, name: &str) -> PathBuf {
        self.templates_dir.join(format!("{}.md", name))
    }

    pub fn get_template_content(&self, name: &str) -> std::io::Result<String> {
        let path = self.get_template_path(name);
        if path.exists() {
            fs::read_to_string(path)
        } else {
            Ok(DEFAULT_NOTE_TEMPLATE.to_string())
        }
    }

    pub fn render_note(&self, metadata: &NoteMetadata, template_name: &str) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("title", &metadata.title);
        context.insert("created", &metadata.created.format("%Y-%m-%d %H:%M:%S %z").to_string());
        context.insert("id", &metadata.id);
        context.insert("tags", &metadata.tags);
        context.insert("links", &metadata.links);
        if let Some(desc) = &metadata.description {
            context.insert("description", desc);
        }
        
        let template_name = if template_name.ends_with(".md") {
            template_name.to_string()
        } else {
            format!("{}.md", template_name)
        };

        self.tera.render(&template_name, &context)
    }

    pub fn list_templates(&self) -> Vec<String> {
        self.tera.get_template_names()
            .map(|name| name.trim_end_matches(".md").to_string())
            .collect()
    }
} 