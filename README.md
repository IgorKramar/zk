# ZK - Менеджер базы знаний

ZK (Zettelkasten) - это инструмент командной строки для ведения базы знаний в формате Zettelkasten.

## Возможности

- ✨ Создание заметок с автоматической генерацией ID
- 🏷️ Поддержка тегов
- 🔍 Полнотекстовый поиск с регулярными выражениями
- 🔗 Управление связями между заметками
- 📝 Поддержка шаблонов заметок
- ⚙️ Гибкая конфигурация

## Установка

### Установка из репозитория
```bash
cargo install --git https://github.com/IgorKramar/zk
```
### Установка из бинарных файлов

1. Скачайте последнюю версию для вашей платформы:
   - [Linux (x86_64)](https://github.com/IgorKramar/zk/releases/latest/download/zk-v0.1.0-linux-x86_64.tar.gz)
   - [Windows (x86_64)](https://github.com/IgorKramar/zk/releases/latest/download/zk-v0.1.0-windows-x86_64.zip)

2. Распакуйте архив:

#### Linux/macOS
```bash
tar xzf zk-*-x86_64.tar.gz
sudo mv zk /usr/local/bin/
```
#### Windows
Распакуйте zip-файл и добавьте путь к zk.exe в PATH

## Использование

### Инициализация базы знаний

```bash
# Создать новую базу знаний в текущей директории
zk init
```

### Работа с заметками

```bash
# Создать новую заметку
zk new "Название заметки"

# Создать заметку по шаблону
zk new "Название заметки" -t template_name

# Показать заметку
zk show abc123

# Поиск заметок
zk search -t "заголовок" -c "содержимое" --tags tag1,tag2
zk search -t "рег.*эксп" -r  # поиск по регулярному выражению
```

### Управление тегами

```bash
# Добавить теги к заметке
zk tag add abc123 tag1 tag2

# Удалить теги из заметки
zk tag remove abc123 tag1

# Показать все теги
zk tag list
```

### Управление связями

```bash
# Добавить связь между заметками
zk link add abc123 def456 -d "Описание связи"

# Показать связи заметки
zk link show abc123
zk link show abc123 --backlinks  # включая обратные связи

# Удалить связь
zk link remove abc123 def456
```

### Работа с шаблонами

```bash
# Показать список шаблонов
zk template list

# Создать новый шаблон
zk template new meeting

# Редактировать шаблон
zk template edit meeting

# Показать содержимое шаблона
zk template show meeting
```

### Конфигурация

```bash
# Показать текущую конфигурацию
zk config show

# Изменить параметр конфигурации
zk config set notes_dir ~/notes
```

## Формат заметок

Заметки хранятся в формате Markdown с YAML-фронтматтером:

```markdown
---
id: abc123
title: Название заметки
created: 2024-01-01T12:00:00Z
tags: [tag1, tag2]
links:
  - from: abc123
    to: def456
    description: Связанная тема
description: Краткое описание заметки
---

Содержимое заметки в формате Markdown...
```

## Обновление

### Обновление из репозитория

```bash
# Обновить до последней версии
cargo install --force --git https://github.com/IgorKramar/zk

# Или, если репозиторий уже склонирован локально:
cd path/to/zk
git pull
cargo install --force --path .
```

### Важно при обновлении

1. Перед обновлением рекомендуется сделать резервную копию ваших заметок:
```bash
# Создать резервную копию
cp -r ~/.zk/notes ~/.zk/notes_backup_$(date +%Y%m%d)
```

2. После обновления может потребоваться миграция данных. Следите за разделом [CHANGELOG.md](CHANGELOG.md) в репозитории.

3. Если вы вносили изменения в конфигурацию или шаблоны, сохраните их копии перед обновлением:
```bash
cp ~/.zkrc ~/.zkrc_backup
cp -r ~/.zk/templates ~/.zk/templates_backup
```

### Откат к предыдущей версии

Если после обновления возникли проблемы, вы можете вернуться к предыдущей версии:

```bash
# В локальном репозитории
git checkout <previous_version_tag>
cargo install --force --path .
```

## Лицензия

MIT