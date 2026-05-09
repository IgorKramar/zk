---
name: zetto
last_updated: 2026-05-09
---

# zetto Strategy

## Target problem

Заметки — plain text, который должен жить и редактироваться в той же цепочке инструментов, что и код (vim, git, grep). Существующие PKM-инструменты (Obsidian) принудительно вытаскивают граф знаний в отдельный GUI-мир, делая его второсортным гражданином рабочего процесса инженера.

## Our approach

Поведение продукта диктуется доказанными паттернами Zettelkasten (Luhmann/Ahrens): atomic notes, link-before-save, фиксированная ID-схема, отказ от папок-как-таксономии и тегов-как-замены-ссылок. Research-grounded constraints держат plain-text-first модель от деградации в очередной markdown-редактор — продукт отказывается от анти-паттернов, даже если их просит пользователь.

## Who it's for

**Primary:** Terminal-native инженеры/исследователи — они уже живут в vim+tmux+git и нанимают zetto, чтобы вести Zettelkasten не выходя из своей цепочки инструментов.

## Key metrics

- **Orphan-note ratio** — доля заметок без исходящих ссылок; цель <5%. Проверяет, что constraint «link-before-save» работает. *Leading.*
- **Avg outgoing links per note** — средняя плотность связей; <1.5 = деградация Zettelkasten. *Leading.*
- **Capture latency** — медиана времени от запуска `zetto new` до сохранения связанной заметки; цель <5 сек. *Leading.*
- **Cross-session usage frequency** — заметки, созданные/отредактированные внутри coding-сессии (тот же tmux/git-repo как код). Регрессирует, если zetto вытесняется обратно в Obsidian. *Leading.*
- **Median time-to-first-link** — как быстро новый пользователь проходит Zettelkasten-инициацию (создание первой связанной заметки). *Lagging.*

## Tracks

### Note graph engine

Парсинг markdown, ID-схема, индекс ссылок, FS-watcher, инкрементальная пересборка графа. Чистая Rust-библиотека без UI.

_Why it serves the approach:_ без надёжного графа constraints (orphan-detection, link-validation) не на чем держать — это substrate-слой для всех остальных треков.

### TUI & capture flow

Single-keystroke capture, fuzzy-linking, навигация по графу, инкрементальный поиск, бюджет латентности по операциям.

_Why it serves the approach:_ constraints должны срабатывать в момент работы, иначе на них забьют. Воплощает «link-before-save» в реальном UX.

### Toolchain interop

Совместимость с git, $EDITOR, ripgrep, fzf; формат файлов как stable contract; CLI-команды, композирующиеся в pipe; hooks для скриптов.

_Why it serves the approach:_ прямо отвечает Target Problem — без этого трека заметки снова закрываются в собственном мирке вместо того, чтобы жить рядом с кодом.

### Methodology enforcement

Линтер заметок (atomic-check, orphan-check, ID-format), миграции, диагностики, опциональные strict-mode/guided пресеты.

_Why it serves the approach:_ прямо воплощает research-grounded constraints — продукт отказывается от анти-паттернов. Без этого трека zetto становится «ещё одним markdown-редактором с TUI».
