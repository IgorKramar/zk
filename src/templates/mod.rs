use tera::{Tera, Context};
use crate::notes::metadata::NoteMetadata;

const DEFAULT_NOTE_TEMPLATE: &str = r#"---
{{ metadata | yaml_encode }}
---

# {{ metadata.title }}

"#;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, tera::Error> {
        let mut tera = Tera::default();
        tera.register_filter("yaml_encode", yaml_encode);
        tera.add_raw_template("note", DEFAULT_NOTE_TEMPLATE)?;
        Ok(TemplateEngine { tera })
    }

    pub fn render_note(&self, metadata: &NoteMetadata) -> Result<String, tera::Error> {
        let mut context = Context::new();
        context.insert("metadata", metadata);
        self.tera.render("note", &context)
    }
}

fn yaml_encode(value: &tera::Value, _: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let yaml = serde_yaml::to_string(&value)
        .map_err(|e| tera::Error::msg(format!("Failed to serialize to YAML: {}", e)))?;
    // Убираем начальный маркер YAML документа "---"
    let yaml = yaml.trim_start_matches("---").trim();
    Ok(tera::Value::String(yaml.to_string()))
} 