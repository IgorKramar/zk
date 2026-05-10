# Research digest: link representation в Rust + PKM-экосистема (zetto A2)

- **Date**: 2026-05-09
- **Cycle**: A2 (Link representation), deep
- **Source**: archforge:researcher agent, 10 web queries
- **Status**: input для Phase 3 Design

## Headline finding

`pulldown-cmark` 0.13.3 (март 2026) **нативно** поддерживает wikilinks через `Options::ENABLE_WIKILINKS`. `LinkType::WikiLink { has_pothole: bool }` различает `[[ID]]` (false) и `[[ID|display]]` (true). Якоря `#Heading` и block-refs `#^id` *внутри* wikilink парсер **не разбирает** — они попадают в `dest_url` как часть строки, что **сохраняет forward-compat для отложенной v2**: split на `#` в resolver-е будет тривиален, существующие линки не сломаются. Embeds `![[ID]]` парсятся как `Tag::Image{LinkType::WikiLink}` — в v1 надо явно отлавливать в lint-rule, иначе HTML-рендер сделает `<img>` с битым src. Внешние URL внутри `[[...]]` парсер принимает (autolink alternative), поэтому ограничение «wikilinks только internal» — это lint-rule на нашей стороне, не parser-rule.

## Summary by topic

### 1. `pulldown-cmark` wikilinks API (latest 0.13.3, март 2026)

- Активация: `Parser::new_ext(input, Options::ENABLE_WIKILINKS)`.
- Событие: `Event::Start(Tag::Link { link_type: LinkType::WikiLink { has_pothole }, dest_url, title, id })`.
- `[[ID]]` → `has_pothole = false`, `dest_url == "ID"`.
- `[[ID|display]]` → `has_pothole = true`, `dest_url == "ID"`, display-text приходит между `Tag::Link` start/end как `Event::Text` (display-text может содержать markdown — `[[ID|*emphasis*]]` валидно).
- 0.13.3 фиксил «Wikilink offset issue» (issue [#516](https://github.com/pulldown-cmark/pulldown-cmark/issues/516)) — source-spans для wikilinks теперь корректны (важно для будущей TUI/LSP подсветки).
- Pull-parser — без AST. Быстрее `comrak` (последний это явно признаёт в README).

### 2. Anchor / block-ref / embed поведение парсера

- `[[ID#Heading]]` → `dest_url == "ID#Heading"`. Резолвер делает `dest_url.split_once('#')` когда v2 включит anchors. **Forward-compat OK.**
- `[[ID#^block-id]]` → `dest_url == "ID#^block-id"`. Аналогично.
- `![[ID]]` → `Tag::Image { link_type: LinkType::WikiLink, dest_url, ... }`. Парсер принимает; в v1 zetto должен:
  - lint-rule `zetto/embed-not-supported-in-v1` отлавливать `Tag::Image` с `LinkType::WikiLink`,
  - в render заменять на текстовый литерал `![[ID]]` (без HTML-рендера в `<img>`),
  - либо writing-time блокировать в capture flow, чтобы пользователь не писал.

### 3. Альтернативные парсеры в 2026 — ландшафт не сдвинулся

- `comrak` (kivikakk, поддерживается paid с 2025): имеет `wikilinks_title_before_pipe`/`wikilinks_title_after_pipe` опции; AST-based, медленнее `pulldown-cmark`. Используется crates.io / docs.rs / GitLab / Deno. Переключение на `comrak` — отдельный архитектурный шаг (ADR-0002 уже зафиксировал `pulldown-cmark`); не повод пересматривать.
- `markdown-rs` (wooorm) и `markdown-it.rs`: упоминаются в comrak-bench Makefile, но нативной wikilinks-поддержки в 2026 не подтверждено.
- `pulldown-cmark-wikilink` (rambip): был форк до 0.10, теперь избыточен.
- **Вывод**: `pulldown-cmark` 0.13+ — стабильный канонический выбор; альтернативы не дают преимуществ для wikilinks-сценария.

### 4. Render-fallback паттерны в установленных PKM (что показывать вместо `[[ID]]` без display-text)

| Tool | Поведение |
|---|---|
| **Obsidian** | filename basename (без `.md`); неразрешимый — сам ID курсивом; `aliases:` frontmatter — для автокомплита, **до 1.12.7+ не использовался для резолва** (известный gap) |
| **Foam** | case-insensitive filename match; `alias:` frontmatter поддерживается (PR #1014); неразрешённые → `[[placeholder]]`-style. Команда «convert wikilink → markdown» **умеет** доставать display из target's title |
| **Logseq** | title-must-be-globally-unique; display = filename. Frontmatter parsing — известные проблемы (issue #6958) |
| **Dendron** | «Copy Note Link» создаёт `[[title|note.path]]` — display из frontmatter `title`. Feature request #238: при rename апдейтить лейбл, **только если он совпадал со старым title** (кастомные не трогать) — полезный паттерн для нас |

**Implementation pattern**: все четыре используют **persistent index** (file-resolver + cache). Синхронный frontmatter scan на каждый рендер — O(N²) на backlinks. Для zetto в pre-alpha допустимо синхронный scan (B1 закроется отдельно), но pattern «index неизбежен» подтверждается.

### 5. Внешние URL внутри `[[...]]` — парсер сотрудничает с нами лишь частично

- `[[https://example.org/]]` → парсер **принимает** как «autolink alternative», `dest_url == "https://example.org/"`.
- Решение «wikilinks = internal-only» — это **post-parse lint-rule на нашей стороне**: проверка `dest_url` regex-ом на URL-схему (`http(s)://`, `mailto:`, `ftp:`, etc.), match → ошибка `zetto/external-url-as-wikilink`.
- Reasoning lint-rule: внешний URL `[text](https://...)` короче и читабельнее в исходнике; ULID validation rule на wikilink-target-ы тогда тривиальна (regex `^[0-9A-HJKMNP-TV-Z]{26}` опционально с `#suffix`).

### 6. Back-compat для канонических markdown-ссылок при rename

`pulldown-cmark` парсит `[text](filename.md)` стандартно — `dest_url == "filename.md"`. Если файл переименован (`01J9XQK7ZB-old.md` → `01J9XQK7ZB-new.md`), линк ломается на уровне filename, но **ULID prefix остаётся идентифицирующим**.

**Паттерн (наш)**: resolver парсит filename target, regex-ом извлекает префиксную ULID (`^[0-9A-HJKMNP-TV-Z]{26}`); если есть match — резолв через ULID (slug часть игнорируется); если нет ULID-префикса — стандартный filename match. Аналогичный «minimum identifier required» pattern используется в Foam для same-name files in different directories.

### 7. Эмпирика: Rust PKM с wikilinks+pipe в 2026

- **`mdzk`** — единственный заметный Rust-проект с wikilinks-as-first-class. Backlinks, frontmatter, mdBook-based рендер. Issue #26 показывает, что frontmatter title resolution и backlink context — open вопросы у мейнтейнера. Не bleeding-edge, но как референс синтаксиса годится.
- **`terror/zk`** (CLI 2025): фокус на fuzzy search и tags; pipe-syntax `[[ID|alias]]` явно не подтверждён в README.
- **`zettelkasten-cli`** (crates.io, июль 2025): слишком молодой.
- **Вывод**: zetto будет в довольно пустой нише. Лучшая заимствуемая практика — Dendron-style: pipe-display = frontmatter `title` на write-time; на rename апдейтить только если лейбл совпадал со старым title (кастомные не трогать).

### 8. Block-ID auto-generation (для отложенной v2)

- **Obsidian convention** (de-facto стандарт): `^` + 6-символьный alphanumeric lowercase `[a-z0-9]{6}`, размещается в конце блока через пробел. Manual `^my-custom-id` тоже валиден (буквы, цифры, дефис). Уникальность — within-file.
- **Logseq**: full block-id system с UUID per-block; тяжелее.
- **Migration cost для zetto** при будущем введении: low — (a) генератор 6-символьных ID, (b) lint на коллизии within-file, (c) extend resolver на `#^`-split. Существующие линки не ломаются (parser уже принимает `[[ID#^xyz]]` валидно).

## Caveats and unknowns

- Точный benchmark `pulldown-cmark` с `ENABLE_WIKILINKS` на типичных note-размерах (1–10 KB) в выдаче не нашёл; общее «pull-parser быстрее AST» подтверждено, но числа конкретно для wikilinks-режима не публиковались. Для pre-alpha CLI не критично.
- Поведение `pulldown-cmark` при невалидной UTF-8/edge-case символах в `dest_url` wikilink (например, ULID с пробелом или unicode confusable) отдельно не изучено. ADR-0002 валидирует ULID при создании, поэтому до парсера невалидный ULID не должен дойти — но user-edited контент может содержать опечатки.
- Source-spans для wikilink-токенов починены в 0.13.3 (issue #516); полностью не верифицировал, что offset tracking работает корректно для всех edge cases. Важно для будущей TUI/LSP integration.
- `markdown-rs` нативная wikilinks-поддержка в 2026 не подтверждена. Если станет важным при пересмотре B2 — отдельное исследование.

## Sources

(20 источников; полный список — в transcript researcher-агента. Ключевые:)

1. [pulldown-cmark wikilinks spec](https://pulldown-cmark.github.io/pulldown-cmark/specs/wikilinks.html)
2. [pulldown-cmark `LinkType` enum docs.rs](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.LinkType.html)
3. [pulldown-cmark releases (v0.13.3, March 2026)](https://github.com/pulldown-cmark/pulldown-cmark/releases)
4. [pulldown-cmark issue #516 — LinkType in broken_link_callback](https://github.com/pulldown-cmark/pulldown-cmark/issues/516)
5. [comrak GitHub](https://github.com/kivikakk/comrak)
6. [Foam wikilinks docs](https://foamnotes.com/user/features/wikilinks)
7. [Obsidian forum — frontmatter aliases not used by resolver (1.12.7)](https://forum.obsidian.md/t/wikilink-resolution-does-not-honor-frontmatter-aliases-1-12-7/113902)
8. [Dendron issue #238 — propagate title to wikilink labels](https://github.com/dendronhq/dendron/issues/238)
9. [mdzk-rs issue #26 — backlink context, frontmatter priorities](https://github.com/mdzk-rs/mdzk/issues/26)
10. [Block ID processing in Obsidian (DeepWiki)](https://deepwiki.com/l1xnan/obsidian-better-export-pdf/7.4-block-id-processing)
