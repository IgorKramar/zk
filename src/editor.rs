use std::env;
use std::process::Command;
use std::io;

pub fn edit_file(path: &std::path::Path) -> io::Result<()> {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    
    let status = Command::new(editor)
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Редактор завершился с ошибкой"
        ));
    }

    Ok(())
} 