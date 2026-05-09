# Decision summary: A1 — Note ID scheme + filename layout

- **Date**: 2026-05-09
- **Cycle**: A1, deep
- **Will become**: ADR-0002
- **Inputs**: discovery, research digest, design (Alternative A chosen)
- **Status**: pending roast + meta-review (deep cycle)

## Решение (одной фразой)

ID заметки — **ULID** (Crockford base32, 26 chars, time-prefixed), хранится канонически в YAML frontmatter; имя файла — эргономическая проекция формата `<ULID>-<slug>.md`, где slug — auto-генерируемая ASCII-транслитерация title.

## Декомпозиция

### Канонический ID

- **Формат**: ULID (Universally Unique Lexicographically Sortable Identifier), 26 chars Crockford base32.
- **Хранение**: YAML frontmatter поле `id`. Это source of truth.
- **Генерация**: автоматическая на `zetto new`. Реализация — крейт `ulid` 1.2.1 (dylanhart), функция `Ulid::new()`. Crockford-кодирование встроено в крейт (использовать внешний `crockford`/`fast32` нельзя — generic encoder выдаёт несовместимые строки, см. `ulid/spec` issue #81).
- **Жизненный цикл**: ID присваивается при создании заметки и **никогда не меняется**. Ручное изменение `id:` пользователем — нарушение контракта; backlinks `[[ULID]]` ломаются.
- **Cross-machine guarantee**: ULID глобально уникален бесплатно (80 бит entropy + ms timestamp); sync через git между машинами не требует никакой координации.
- **Sortability**: первые 48 бит = ms Unix epoch; Crockford lexicographic order = chronological order. Cross-machine ordering — approximately chronological modulo wall-clock skew (документированный caveat).

### Filename layout

- **Pattern**: `<ULID>-<slug>.md`
- **Пример**: `01J9XQK7ZBV5G2D8X3K7C4M9P0-on-fixed-ids.md`
- **Empty-slug fallback**: `<ULID>.md` (без хвостового `-`). Срабатывает когда title даёт пустой slug — все CJK кандзи, которые `deunicode` не транслитерирует читаемо, чистый emoji, нерасшифровываемые символы.
- **Max length**: slug-часть капится на **60 символов** (труним по word boundary). Полный filename ≤ ~92 ASCII chars + `.md`. Кросс-платформенно безопасно (NTFS, ext4, APFS все разрешают ≥255 char filenames).
- **Charset**: `[a-z0-9-]` для slug, плюс `[0-9A-Z]` для ULID-префикса. Никаких пробелов, спецсимволов, мульти-байт. Дружит с rg/fzf/git/vim/shell.
- **Slug normalization**: крейт `slug` (Stebalien, deunicode-backed). Поведение по языкам:
  - кириллица: `Заметка о ULID` → `zametka-o-ulid` ✓
  - японская хирагана/катакана: `ノート` → `noto` ✓
  - японские кандзи: мапятся на путунхуа-pronunciation (документированное ограничение `deunicode`); для японско-тяжёлых vault-ов — open question, fallback на ULID-only filename
  - emoji: `🔥` → `fire`, `🦄☣` → `unicorn-face-biohazard` (через deunicode)
  - mixed: best-effort; в degenerate-случаях fallback на empty-slug

### Линкование

- **Формат линка**: `[[<ULID>]]` — например, `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]`
- **Резолюция**: O(1) через filename-glob `<ULID>-*.md` или `<ULID>.md`. Нет необходимости в persistent index для одного lookup; индекс понадобится для inverse (backlinks), но это решается отдельно (B1).
- **Полная семантика линков** (включая display-text, anchor refs, embed) — отложена в ADR-0003 (A2). A1 фиксирует только то, что **identifier внутри линка — ULID**.

### Поведение при retitle

- Frontmatter `title` поменялся → zetto **регенерирует slug** из нового title через `slug` крейт.
- Если новый slug ≠ старого: zetto **переименовывает файл** через `std::fs::rename`. ULID-префикс константен; меняется только slug-часть.
- **ULID frontmatter не трогается**.
- Backlinks `[[ULID]]` остаются валидными — резолвят на новый filename через filename-glob.
- Атомарность: `std::fs::rename` достаточна для slug-rename (тот же inode на POSIX, atomic на Windows ≥10 1607 после PR #138133 в 2025). `atomic-write-file` используется только для body-edits.
- **Edge case**: если новый slug совпадает с slug другой существующей заметки — нет коллизии в filename (ULID-префиксы разные), значит rename проходит чисто.

### ID generation на `zetto new`

- Алгоритм:
  1. `Ulid::new()` → 26-char string.
  2. Title (опционально передан в command-line или собран в TUI capture-flow); если пустой — slug пустой, fallback на ULID-only filename.
  3. `slug::slugify(title)` → ASCII-slug, обрезается до 60 chars по word-boundary.
  4. Filename = `format!("{ulid}-{slug}.md")` или `format!("{ulid}.md")` если slug пустой.
  5. Frontmatter сериализуется через `serde_yaml`: `id: <ULID>\ntitle: <title>\n...\n---\n<body>`.
  6. Запись body — `atomic-write-file`.
  7. Готово; capture latency budget на этом шаге < 50 ms (ULID generation ~ns; slug normalization ~µs; FS write ~ms).

### ID monotonicity и concurrency

- Внутри одного `zetto`-процесса: используем `Ulid::new()`, который на overflow random-части в одной мс возвращает ID с инкрементированной random-частью (monotonic в spec). Для personal CLI с `<` 100 заметок/сек overflow невозможен.
- Между процессами: ULID timestamps независимы; collision вероятность 2^-80 при одновременном вызове в одной мс — пренебрежимо.
- Multi-device через git: каждое устройство генерирует ULID с собственного wall-clock; ordering приблизительно хронологический modulo skew. Документированный caveat для пользователя.

## Зависимости (внешние крейты)

| Крейт | Версия | Назначение | Лицензия |
|---|---|---|---|
| `ulid` | 1.2.1 (Mar 2025) | ULID generation + Crockford encoding | MIT/Apache-2.0 |
| `slug` | latest (deunicode-backed) | Title → ASCII slug | MIT/Apache-2.0 |
| `gray_matter` | latest | YAML frontmatter parsing (read) | MIT |
| `serde_yaml` | latest | YAML frontmatter serialization (write) | MIT/Apache-2.0 |
| `atomic-write-file` | latest | Atomic body writes | MIT/Apache-2.0 |

`pulldown-cmark-frontmatter` явно отвергнут: требует fenced code-block delimiter, что несовместимо с конвенцией `---` (используется Obsidian/Logseq/Foam). Зависимость от `gray_matter` обеспечивает совместимость с D4-будущим.

## Контракт публичного формата (закладка для A5)

ULID + slug filename layout фиксируется как часть `format-v1` (см. A5 в `decision-map.md`). Любое изменение этого layout требует:
- Mажорной версии `format-v2`
- Migration tool (`zetto migrate format-v1 format-v2`)
- Соответствующего ADR

CI test-suite zetto должен валидировать: парсер читает `<ULID>-<slug>.md` и `<ULID>.md` filenames, frontmatter `id:` поле обязательно, slug часть filename re-derivable из title (с tolerance на manual edit user-ом).

## Open questions, отложенные в смежные ADR

- **A2** (link representation): полная семантика wikilink (display-text, embeds, anchor refs). Этот ADR (A1) фиксирует только что identifier внутри линка — ULID.
- **A3** (frontmatter convention): полная схема frontmatter полей (created, updated, tags, aliases, etc.). A1 трогает только обязательное `id:` поле.
- **A4** (notes directory layout): single root vs id-prefix-buckets vs freeform. A1 не зависит — ULID-prefix filename работает в любом каталоге.
- **A5** (format versioning): A1 закладывает свои части в `format-v1`.
- **B1** (graph index): для `[[ULID]]` резолва без full FS-scan нужен индекс backlinks. Сам ULID lookup O(1) без индекса.
- **D4** (Obsidian compat): для read-write сценария потребуются frontmatter `aliases:` поле, перечисляющее slug в каждой заметке. Не блокер для A1.

## Rejection summary (краткая)

См. полную таблицу в `2026-05-09-A1-note-id-scheme-design.md` раздел «Explicitly not considered». Кратко отвергнуты: UUID v4 (длиннее, не сортируется), timestamp-only (community-feedback negative, collision), Luhmann hierarchical (paper-era, ручной выбор), three-letter human (manual choice + only-vault-local), content-hash (меняется с edit), filename-as-title (retitle ломает backlinks), slug+UUID-suffix (UUID длиннее ULID, hostile sortability).

## Альтернативы B и C из design — почему не выбраны

**B (slug-only filename)**: оптимален для D4-strict-checker позиции, но требует persistent index с первого дня (B1 принудительно становится persistent), плюс slug-collision logic, плюс O(N) cold-start. Cost не оправдан текущим состоянием D4 (ещё не решён).

**C (ULID-only filename)**: формально проще, но «стена ID в `ls`» режет terminal-native UX-метрику, которая прямо в STRATEGY (cross-session usage frequency, time-to-first-link). Цена выше, чем длинный filename в A.

## Что дальше (после roast/meta-review)

1. Phase 5 — `/archforge:roast` пять ролей: `devil-advocate`, `pragmatist`, `junior-engineer`, `compliance-officer`, `futurist`. Цель: найти failure modes, операционные расходы, дыры в документе, regulatory/security gaps, drift на 2-летнем горизонте.
2. Phase 5b — `/archforge:meta-review` на roast-output (template conformance, identifier preservation, language-pass evidence).
3. Phase 6 — пользователь выбирает: (a) применить findings → ADR-0002, (b) применить + re-roast, (c) шаг назад в design/discovery.
4. Phase 7 — `/archforge:document`: ADR-0002 + amendments в decision-map.md (A1 → decided), ARCHITECTURE.md §5, docs/architecture/README.md.
5. ADR-0003 (link representation, A2) разблокирован.
