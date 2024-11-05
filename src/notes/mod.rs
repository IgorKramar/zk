use std::fs;
use chrono::Local;
use crate::config::Config;
use crate::templates::TemplateEngine;

pub mod metadata;
use metadata::NoteMetadata;

pub fn create_note(config: &Config, title: &str, template: Option<&str>) -> std::io::Result<()> {
    fs::create_dir_all(&config.notes_dir)?;

    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let filename = config.filename_template
        .replace("{timestamp}", &timestamp.to_string())
        .replace("{title}", &title.replace(' ', "-"))
        + ".md";
    
    let filepath = config.notes_dir.join(filename);

    let metadata = NoteMetadata::new(title.to_string());
    let template_engine = TemplateEngine::new(config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    let template_name = template.unwrap_or(&config.default_template);
    let content = template_engine.render_note(&metadata, template_name)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    fs::write(filepath, content)?;
    Ok(())
} 