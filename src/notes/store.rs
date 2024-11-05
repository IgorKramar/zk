use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use regex::Regex;
use crate::notes::metadata::NoteMetadata;

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