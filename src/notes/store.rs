use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::notes::metadata::NoteMetadata;

#[derive(Debug)]
pub struct SearchQuery {
    pub tags: Option<Vec<String>>,
    pub title: Option<String>,
    pub content: Option<String>,
}

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

    pub fn search(&self, query: &SearchQuery) -> std::io::Result<Vec<(NoteMetadata, String)>> {
        let mut results = Vec::new();

        // Подготовим теги для поиска, если они есть
        let tags_set = query.tags.as_ref().map(|tags| {
            tags.iter().collect::<std::collections::HashSet<_>>()
        });

        for path in self.notes.values() {
            let content = fs::read_to_string(path)?;
            if let Ok((metadata, note_content)) = parse_frontmatter(&content) {
                // Проверяем соответствие всем критериям поиска
                let matches = {
                    // Проверка тегов
                    let tags_match = tags_set.as_ref().map_or(true, |search_tags| {
                        let note_tags: std::collections::HashSet<_> = metadata.tags.iter().collect();
                        search_tags.is_subset(&note_tags)
                    });

                    // Проверка заголовка
                    let title_match = query.title.as_ref().map_or(true, |search_title| {
                        metadata.title.to_lowercase().contains(&search_title.to_lowercase())
                    });

                    // Проверка содержимого
                    let content_match = query.content.as_ref().map_or(true, |search_content| {
                        let search_lower = search_content.to_lowercase();
                        note_content.to_lowercase().contains(&search_lower) ||
                        metadata.description.as_ref().map_or(false, |desc| 
                            desc.to_lowercase().contains(&search_lower)
                        )
                    });

                    tags_match && title_match && content_match
                };

                if matches {
                    results.push((metadata, note_content));
                }
            }
        }

        // Сортируем результаты по дате создания (новые сначала)
        results.sort_by(|a, b| b.0.created.cmp(&a.0.created));
        Ok(results)
    }

    pub fn get_path(&self, id: &str) -> Option<&PathBuf> {
        self.notes.get(id)
    }

    pub fn get_metadata(&self, id: &str) -> std::io::Result<Option<NoteMetadata>> {
        if let Some(path) = self.get_path(id) {
            let content = fs::read_to_string(path)?;
            if let Ok((metadata, _)) = parse_frontmatter(&content) {
                Ok(Some(metadata))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
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