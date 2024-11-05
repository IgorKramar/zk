use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::notes::metadata::NoteMetadata;

pub struct NoteStore {
    notes_dir: PathBuf,
    notes: HashMap<String, PathBuf>,
}

impl NoteStore {
    pub fn new(notes_dir: PathBuf) -> std::io::Result<Self> {
        let mut store = NoteStore {
            notes_dir,
            notes: HashMap::new(),
        };
        store.refresh()?;
        Ok(store)
    }

    pub fn refresh(&mut self) -> std::io::Result<()> {
        self.notes.clear();
        
        for entry in WalkDir::new(&self.notes_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let content = fs::read_to_string(entry.path())?;
            if let Ok((metadata, _)) = parse_frontmatter(&content) {
                self.notes.insert(metadata.id.clone(), entry.path().to_path_buf());
            }
        }
        
        Ok(())
    }

    pub fn get_path(&self, id: &str) -> Option<&PathBuf> {
        // Поддержка как полного ID, так и короткого
        self.notes.iter()
            .find(|(note_id, _)| note_id.starts_with(id))
            .map(|(_, path)| path)
    }

    pub fn get_metadata(&self, id: &str) -> std::io::Result<Option<NoteMetadata>> {
        if let Some(path) = self.get_path(id) {
            let (metadata, _) = parse_frontmatter(&fs::read_to_string(path)?)?;
            Ok(Some(metadata))
        } else {
            Ok(None)
        }
    }

    pub fn search_by_tags(&self, tags: &[String]) -> std::io::Result<Vec<NoteMetadata>> {
        let mut results = Vec::new();
        let tags_set: std::collections::HashSet<_> = tags.iter().collect();

        for path in self.notes.values() {
            if let Ok((metadata, _)) = parse_frontmatter(&fs::read_to_string(path)?) {
                let note_tags: std::collections::HashSet<_> = metadata.tags.iter().collect();
                if tags_set.is_subset(&note_tags) {
                    results.push(metadata);
                }
            }
        }

        // Сортируем результаты по дате создания (новые сначала)
        results.sort_by(|a, b| b.created.cmp(&a.created));
        Ok(results)
    }
}

fn parse_frontmatter(content: &str) -> std::io::Result<(NoteMetadata, String)> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() != 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Некорректный формат frontmatter",
        ));
    }

    let metadata: NoteMetadata = serde_yaml::from_str(parts[1].trim())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok((metadata, parts[2].to_string()))
} 