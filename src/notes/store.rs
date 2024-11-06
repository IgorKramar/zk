use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use regex::Regex;
use crate::notes::metadata::{NoteMetadata, Link};
use serde_yaml;
use std::io;

#[derive(Debug)]
pub struct SearchQuery {
    pub tags: Option<Vec<String>>,
    pub title: Option<SearchPattern>,
    pub content: Option<SearchPattern>,
}

#[derive(Debug)]
pub enum SearchPattern {
    Plain(String),
    Regex(Regex),
}

#[derive(Debug)]
pub struct SearchMatch {
    pub metadata: NoteMetadata,
    pub content: String,
    pub title_matches: Vec<(usize, usize)>,   // (start, end) индексы совпадений
    pub content_matches: Vec<(usize, usize)>, // (start, end) индексы совпадений
}

impl SearchPattern {
    pub fn new(pattern: String, use_regex: bool) -> Result<Self, regex::Error> {
        if use_regex {
            // Конвертируем glob-подобные паттерны в регулярные выражения
            let regex_pattern = if !pattern.contains("\\") {
                pattern
                    .replace("*", ".*")
                    .replace("?", ".")
                    .replace("+", "\\+")
                    .replace("(", "\\(")
                    .replace(")", "\\)")
                    .replace("[", "\\[")
                    .replace("]", "\\]")
                    .replace("{", "\\{")
                    .replace("}", "\\}")
                    .replace("^", "\\^")
                    .replace("$", "\\$")
                    .replace("|", "\\|")
            } else {
                pattern
            };
            
            Ok(SearchPattern::Regex(Regex::new(&regex_pattern)?))
        } else {
            Ok(SearchPattern::Plain(pattern))
        }
    }

    pub fn find_matches(&self, text: &str) -> Vec<(usize, usize)> {
        match self {
            SearchPattern::Plain(pattern) => {
                let pattern = pattern.to_lowercase();
                let text_lower = text.to_lowercase();
                text_lower
                    .match_indices(&pattern)
                    .map(|(idx, matched)| (idx, idx + matched.len()))
                    .collect()
            }
            SearchPattern::Regex(regex) => {
                regex.find_iter(text)
                    .map(|m| (m.start(), m.end()))
                    .collect()
            }
        }
    }
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

    pub fn search(&self, query: &SearchQuery) -> std::io::Result<Vec<SearchMatch>> {
        let mut results = Vec::new();
        let tags_set = query.tags.as_ref().map(|tags| {
            tags.iter().collect::<std::collections::HashSet<_>>()
        });

        for path in self.notes.values() {
            let content = fs::read_to_string(path)?;
            if let Ok((metadata, note_content)) = parse_frontmatter(&content) {
                let mut title_matches = Vec::new();
                let mut content_matches = Vec::new();
                
                let matches = {
                    // Проверка тегов
                    let tags_match = tags_set.as_ref().map_or(true, |search_tags| {
                        let note_tags: std::collections::HashSet<_> = metadata.tags.iter().collect();
                        search_tags.is_subset(&note_tags)
                    });

                    // Проверка заголовка
                    let title_match = if let Some(pattern) = &query.title {
                        let found_matches = pattern.find_matches(&metadata.title);
                        if !found_matches.is_empty() {
                            title_matches = found_matches;
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    };

                    // Проверка содержимого
                    let content_match = if let Some(pattern) = &query.content {
                        let mut found = false;
                        if let Some(matches) = pattern.find_matches(&note_content).into_iter().next() {
                            content_matches.push(matches);
                            found = true;
                        }
                        if let Some(desc) = &metadata.description {
                            if let Some(matches) = pattern.find_matches(desc).into_iter().next() {
                                content_matches.push(matches);
                                found = true;
                            }
                        }
                        found
                    } else {
                        true
                    };

                    tags_match && (title_match || content_match)
                };

                if matches {
                    results.push(SearchMatch {
                        metadata,
                        content: note_content,
                        title_matches,
                        content_matches,
                    });
                }
            }
        }

        results.sort_by(|a, b| b.metadata.created.cmp(&a.metadata.created));
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

    pub fn add_link(&mut self, from: &str, to: &str, description: Option<String>) -> std::io::Result<()> {
        // Проверяем существование обеих заметок
        if !self.notes.contains_key(from) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Заметка с ID '{}' не найдена", from),
            ));
        }
        if !self.notes.contains_key(to) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Заметка с ID '{}' не найдена", to),
            ));
        }

        let path = self.notes.get(from).unwrap();
        let content = fs::read_to_string(path)?;
        let (mut metadata, note_content) = parse_frontmatter(&content)?;

        // Добавляем связь, если её ещё нет
        if !metadata.links.iter().any(|link| &link.to == to) {
            metadata.links.push(Link {
                from: from.to_string(),
                to: to.to_string(),
                description,
            });

            // Сохраняем обновленную заметку
            let updated_content = format!(
                "---\n{}\n---\n{}",
                serde_yaml::to_string(&metadata).map_err(|e| std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))?,
                note_content
            );
            fs::write(path, updated_content)?;
        }

        Ok(())
    }

    pub fn remove_link(&mut self, from: &str, to: &str) -> std::io::Result<()> {
        let path = self.notes.get(from).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Заметка с ID '{}' не найдена", from),
            )
        })?;

        let content = fs::read_to_string(path)?;
        let (mut metadata, note_content) = parse_frontmatter(&content)?;

        // Удаляем связь, если она есть
        if let Some(pos) = metadata.links.iter().position(|link| &link.to == to) {
            metadata.links.remove(pos);

            // Сохраняем обновленную заметку
            let updated_content = format!(
                "---\n{}\n---\n{}",
                serde_yaml::to_string(&metadata).map_err(|e| std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))?,
                note_content
            );
            fs::write(path, updated_content)?;
        }

        Ok(())
    }

    pub fn get_links(&self, id: &str, include_backlinks: bool) -> std::io::Result<Vec<Link>> {
        let mut links = Vec::new();

        // Получаем прямые связи
        if let Some(path) = self.notes.get(id) {
            let content = fs::read_to_string(path)?;
            if let Ok((metadata, _)) = parse_frontmatter(&content) {
                links.extend(metadata.links);
            }
        }

        // Получаем обратные связи, если требуется
        if include_backlinks {
            for (note_id, path) in &self.notes {
                if note_id == id {
                    continue;
                }

                let content = fs::read_to_string(path)?;
                if let Ok((metadata, _)) = parse_frontmatter(&content) {
                    for link in metadata.links {
                        if &link.to == id {
                            links.push(Link {
                                from: link.from,
                                to: id.to_string(),
                                description: link.description,
                            });
                        }
                    }
                }
            }
        }

        Ok(links)
    }

    pub fn list(&self) -> io::Result<Vec<(String, NoteMetadata)>> {
        let mut notes = Vec::new();
        
        for (id, path) in &self.notes {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok((metadata, _)) = parse_frontmatter(&content) {
                    notes.push((id.clone(), metadata));
                }
            }
        }
        
        Ok(notes)
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