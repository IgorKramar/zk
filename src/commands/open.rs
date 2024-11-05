use std::process::Command;
use crate::config::Config;
use crate::notes::store::NoteStore;
use std::io;
use crate::editor;

pub fn open_note(id: Option<String>, app: Option<String>, config: &Config) -> io::Result<()> {
    let store = NoteStore::new(config.notes_dir.clone())?;
    
    let note_id = match id.or_else(|| config.get_active_note().map(String::from)) {
        Some(id) => id,
        None => return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "ID заметки не указан и нет активной заметки\nПример: zk open abc123"
        )),
    };

    let path = store.get_path(&note_id)
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::NotFound,
            format!("Заметка с ID '{}' не найдена", note_id)
        ))?;

    if let Some(app) = app {
        let (cmd, args) = match app.as_str() {
            // Vim-подобные
            "vim" => ("vim", vec![path.to_str().unwrap()]),
            "nvim" => ("nvim", vec![path.to_str().unwrap()]),
            "gvim" => ("gvim", vec![path.to_str().unwrap()]),
            
            // VS Code и его варианты
            "code" | "vscode" => ("code", vec![path.to_str().unwrap()]),
            "codium" => ("codium", vec![path.to_str().unwrap()]),
            
            // Emacs
            "emacs" => ("emacs", vec![path.to_str().unwrap()]),
            "emacsclient" => ("emacsclient", vec!["-c", path.to_str().unwrap()]),
            
            // Sublime Text
            "subl" => ("subl", vec![path.to_str().unwrap()]),
            
            // Nano и другие консольные
            "nano" => ("nano", vec![path.to_str().unwrap()]),
            "micro" => ("micro", vec![path.to_str().unwrap()]),
            
            // Графические редакторы
            "gedit" => ("gedit", vec![path.to_str().unwrap()]),
            "kate" => ("kate", vec![path.to_str().unwrap()]),
            "mousepad" => ("mousepad", vec![path.to_str().unwrap()]),
            
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Неподдерживаемое приложение: {}\nПоддерживаются: vim, nvim, gvim, code, codium, emacs, emacsclient, subl, nano, micro, gedit, kate, mousepad", app)
            )),
        };

        let status = Command::new(cmd)
            .args(args)
            .status()?;

        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Ошибка при открытии файла в {}", app)
            ));
        }
    } else {
        editor::edit_file(&path)?;
    }

    if let Ok(mut config) = Config::load() {
        if let Err(e) = config.set_active_note(&note_id) {
            eprintln!("Предупреждение: не удалось обновить активную заметку: {}", e);
        }
    }

    Ok(())
}