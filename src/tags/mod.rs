use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::notes::metadata::NoteMetadata;

pub fn add_tags(path: &Path, tags: &[String]) -> std::io::Result<()> {
    let content = fs::read_to_string(path)?;
    let (mut metadata, rest) = parse_frontmatter(&content)?;
    
    // Добавляем новые теги
    let mut new_tags: HashSet<_> = metadata.tags.into_iter().collect();
    new_tags.extend(tags.iter().cloned());
    metadata.tags = new_tags.into_iter().collect();
    metadata.tags.sort();

    // Записываем обновленный файл
    let yaml = serde_yaml::to_string(&metadata)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let new_content = format!("---\n{}---\n{}", yaml, rest);
    fs::write(path, new_content)?;

    Ok(())
}

pub fn remove_tags(path: &Path, tags: &[String]) -> std::io::Result<()> {
    let content = fs::read_to_string(path)?;
    let (mut metadata, rest) = parse_frontmatter(&content)?;
    
    // Удаляем указанные теги
    let tags_to_remove: HashSet<_> = tags.iter().collect();
    metadata.tags.retain(|tag| !tags_to_remove.contains(tag));

    // Записываем обновленный файл
    let yaml = serde_yaml::to_string(&metadata)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let new_content = format!("---\n{}---\n{}", yaml, rest);
    fs::write(path, new_content)?;

    Ok(())
}

pub fn list_all_tags(notes_dir: &Path) -> std::io::Result<HashSet<String>> {
    let mut all_tags = HashSet::new();

    for entry in WalkDir::new(notes_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        if let Ok((metadata, _)) = parse_frontmatter(&fs::read_to_string(entry.path())?) {
            all_tags.extend(metadata.tags);
        }
    }

    Ok(all_tags)
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