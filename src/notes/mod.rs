use std::fs;
use chrono::Local;
use crate::config::Config;
use crate::templates::TemplateEngine;

pub mod metadata;
use metadata::NoteMetadata;

pub fn create_note(config: &Config, title: &str) -> std::io::Result<()> {
    fs::create_dir_all(&config.notes_dir)?;

    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let filename = config.filename_template
        .replace("{timestamp}", &timestamp.to_string())
        .replace("{title}", &title.replace(' ', "-"))
        + ".md";
    
    let filepath = config.notes_dir.join(filename);

    let metadata = NoteMetadata::new(title.to_string());
    let template_engine = TemplateEngine::new()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    let content = template_engine.render_note(&metadata)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    fs::write(filepath, content)?;
    Ok(())
} 