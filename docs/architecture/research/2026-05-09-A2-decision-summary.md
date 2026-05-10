# Decision summary: A2 — Link representation

- **Date**: 2026-05-09
- **Cycle**: A2, deep
- **Will become**: ADR-0003
- **Inputs**: discovery (7 leans приняты), research digest, design (Альтернатива A выбрана)
- **Status**: pending roast + meta-review

## Решение (одной фразой)

Internal-link-syntax — wikilink-primary: zetto **генерирует** `[[ULID]]` или `[[ULID|display]]`; **читает** оба wikilink-формата плюс canonical markdown `[text](filename.md)` (для back-compat с imported content). External URLs — только canonical markdown `[text](https://...)`. Embeds, anchor refs и block refs deferred в v2.

## Декомпозиция

### Wikilink-syntax v1

- **Forms zetto генерирует на write**:
  - `[[ULID]]` — link без display-text. Render берёт title из frontmatter target-а.
  - `[[ULID|display]]` — link с inline display-text. Render использует display.
- **Where zetto читает на read** (parser + resolver принимают любой из):
  - `[[ULID]]` — wikilink без display.
  - `[[ULID|display]]` — wikilink с display.
  - `[text](filename.md)` — canonical markdown. Резолвер extract-ит ULID-префикс из filename target-а regex-ом `^[0-9A-HJKMNP-TV-Z]{26}`; match → резолв через ULID; нет match → стандартный filename match.
  - `[text](https://...)`, `[text](mailto:...)` — external markdown. Никогда не резолвится в vault.

### Parser (применение research §1)

- Крейт: `pulldown-cmark` 0.13.3 (см. ADR-0002).
- Активация wikilinks: `Parser::new_ext(input, Options::ENABLE_WIKILINKS)`.
- Detection: `Event::Start(Tag::Link { link_type: LinkType::WikiLink { has_pothole }, dest_url, .. })`.
  - `has_pothole = false` → `[[X]]` form.
  - `has_pothole = true` → `[[X|display]]` form; display-text приходит как `Event::Text` между `Tag::Link` start/end.
- Embed detection: `Event::Start(Tag::Image { link_type: LinkType::WikiLink, .. })` → `![[X]]`.

### Resolver

Алгоритм для wikilink-target-а (`dest_url`):

1. **Split on `#`**: `(id_part, suffix_opt) = dest_url.split_once('#')`. В v1 `suffix_opt` игнорируется (см. anchor/block-ref поведение ниже).
2. **External URL detection**: regex `^(https?|ftp|mailto):` на `id_part`. Match → lint flag `zetto/external-url-as-wikilink`, render literal.
3. **ULID validation**: regex `^[0-9A-HJKMNP-TV-Z]{26}$`. Не match → lint flag `zetto/non-ulid-wikilink-target` (предположение, что user подразумевал internal link, но ID невалиден); render literal.
4. **File lookup**: glob `<id_part>-*.md` или `<id_part>.md` в notes-каталоге. Найдено — resolved. Не найдено — lint flag `zetto/no-broken-link`; render literal в broken-style.

Алгоритм для markdown-link-target-а (`dest_url`):

1. **External detection**: schema-prefix → external link, render как стандартный CommonMark, не lint-flag.
2. **Internal**: extract ULID-prefix из basename regex-ом; если есть — резолв через ULID-glob (как в шаге 4 wikilink); нет — стандартный filename match (для importable content).

### Render

Display-text приоритет (для wikilink):

1. Inline display из `[[ULID|display]]` — если есть, используем.
2. Frontmatter `title` из target-заметки — synchronous scan в v1 (TODO заменить на B1 graph index когда B1 закрыт).
3. ULID literal — если target не резолвится; визуально — broken style (CSS class в HTML render; в TUI — отдельный colour).

### Lint rules (применение research §2 + §5)

В v1 zetto поставляет следующие правила (предсет `recommended-luhmann`, см. ADR-0002 § Methodology engine architecture):

| Rule ID | Description | Default severity |
|---|---|---|
| `zetto/no-broken-link` | wikilink или markdown-link с unresolvable target | warn |
| `zetto/external-url-as-wikilink` | `[[https://...]]` или другая URL-схема внутри wikilink | error |
| `zetto/non-ulid-wikilink-target` | `[[X]]` где X не валидный ULID-format | warn |
| `zetto/embed-not-supported-in-v1` | `![[X]]` обнаружен — не render-ится в v1 | warn |
| `zetto/anchor-not-supported-in-v1` | `[[ULID#Heading]]` обнаружен — anchor не резолвится в v1 | warn |
| `zetto/block-ref-not-supported-in-v1` | `[[ULID#^block-id]]` обнаружен — block-ref не резолвится в v1 | warn |

### Deferred в v2 (применение research §2 + §8)

- **Embeds `![[ULID]]`**: parser уже принимает (`Tag::Image{LinkType::WikiLink}`). v1 — lint warn + literal render (без HTML `<img>`). v2 — добавить render-pass который inline-ит target-content. Migration cost — низкий, форвард-compat free.
- **Anchor refs `[[ULID#Heading]]`**: parser отдаёт `dest_url == "ULID#Heading"`. v1 — lint warn + игнорирование suffix в resolver. v2 — split-on-hash + heading-slug-stable contract. Heading-slug stability — отдельный sub-decision (как генерировать stable anchor-id из heading text при possible-edits — `slug::slugify(heading)` — но heading-rename ломает links).
- **Block refs `[[ULID#^block-id]]`**: parser отдаёт `dest_url == "ULID#^block-id"`. v1 — lint warn. v2 — Obsidian convention block-IDs (`^[a-z0-9]{6}` суффикс блока, auto-generated при write). Migration cost — низкий, форвард-compat free.

Defer-стратегия защищена тем, что **синтаксис уже валиден на уровне парсера** — пользователь, который случайно или намеренно напишет `![[X]]` или `[[X#H]]`, получит lint warn (не parser-error), и линки не сломаются при переходе v1 → v2 (resolver просто начинает понимать `#`-суффикс).

### Edge cases

- **Empty wikilink `[[]]`**: parser принимает, `dest_url == ""`. zetto lint flag `zetto/no-broken-link` (target empty); render literal.
- **Pipe-only `[[|display]]`**: `dest_url == ""`, `has_pothole = true`. Аналогично — lint flag, literal.
- **Multi-pipe `[[ID|a|b]]`**: parser обрабатывает по своим правилам (видит первый `|` как разделитель, остаток — display). zetto не делает additional checks.
- **Markdown link `[](path.md)` (empty text)**: parser принимает; resolver работает по target-у; render показывает path как fallback text.

### Forward-compat statement (применение research §2)

Этот ADR (A2) фиксирует **wikilink syntax + display-text + external-URL-разделение** как часть `format-v1` (см. A5 в decision-map). **`#`-suffix-семантика и embed-семантика — implementation-detail-будущего** (по образу ADR-0002 § Format versioning anchor): v2 может расширить resolver на anchor / block-ref / embed без поднятия major-версии format-v1, потому что синтаксис уже принимается parser-ом и существующие линки не ломаются.

## Лицензии и зависимости (delta к ADR-0002)

`pulldown-cmark` 0.13.3 уже зафиксирован в ADR-0002 § Crate dependencies. Этот ADR требует только включения `Options::ENABLE_WIKILINKS` — нет новых крейтов.

## Open questions, отложенные в смежные ADR / v2

- **A3** (frontmatter convention): обязательное поле `title:` подразумевается этим решением (render-fallback для display-text). A3 формализует whether `title:` mandatory или optional с какой-то seed-логикой.
- **B1** (graph index): synchronous frontmatter scan для render-fallback — temporary v1 решение; B1 заменит на index lookup.
- **C2a** (rule engine): 6 lint rules выше становятся первой партией правил `recommended-luhmann` preset.
- **D4** (Obsidian compat): уровень compat (read-only / read-write / variant a/b/c) определит, нужны ли zetto-side aliases-resolution и render-time-aliasing.
- **v2-cycle для embeds/anchors/block-refs** — отдельные ADR при появлении конкретного use case (вероятно после B1).

## Что дальше (после roast/meta-review)

1. Phase 5 — `/archforge:roast` 5 ролей.
2. Phase 5b — `/archforge:meta-review` на roast-output.
3. Phase 6 — пользователь выбирает: (a) применить findings → ADR-0003, (b) применить + re-roast, (c) шаг назад.
4. Phase 7 — `/archforge:document`: ADR-0003 + amendments в `decision-map.md` (A2 → decided), `ARCHITECTURE.md` §5 + §6 (Q2 закрыт), `docs/architecture/README.md`.
5. A3 (frontmatter convention) разблокирован.
