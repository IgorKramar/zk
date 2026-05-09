# Design: A1 — Note ID scheme — alternatives

- **Date**: 2026-05-09
- **Cycle**: A1, deep
- **Inputs**: `2026-05-09-A1-note-id-scheme-discovery.md` (round 1, 7 leans все приняты), `2026-05-09-A1-ulid-slug-rust-research.md`
- **Status**: design phase, ожидает выбора альтернативы

## Зафиксированные предпосылки

Из принятых leans:
- ID format = **ULID** (26 chars, Crockford base32, time-prefixed)
- ID storage canonical = **YAML frontmatter** (`id: 01J9XQK7ZB...`)
- Filename = **эргономическая проекция** ID
- Slug = **pure ergonomic, не часть ID**
- ID generation = **auto на `zetto new`**
- Cross-machine = **глобально уникален** (ULID даёт бесплатно)
- Retitle = **slug пересчитывается, file переименовывается, ID константен, backlinks `[[ID]]` живут**

Из этих leans все три альтернативы ниже совместимы. Оставшийся вопрос — **что именно кладём в filename**.

## Три альтернативы

### A. ULID-prefix + slug

**Filename**: `<ULID>-<slug>.md`
**Пример**: `01J9XQK7ZBV5G2D8X3K7C4M9P0-on-fixed-ids.md`
**Пустой slug fallback**: `<ULID>.md`
**Frontmatter**: `id: 01J9XQK7ZBV5G2D8X3K7C4M9P0`, `title: "On fixed IDs"`
**Линки**: `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]` (резолвится по ULID-префиксу filename — O(1) lookup; fallback на frontmatter-сканирование)

**Pros:**
- Filename хронологически сортируется (`ls -1` показывает заметки в порядке создания)
- ULID всегда виден в filename → grep/fzf/git-log могут найти заметку по ID без TUI
- Slug-collision невозможна (ULID-префикс гарантирует уникальность filename)
- Lookup `[[ULID]]` → O(1) через filename-glob `<ULID>-*.md`
- Rename переписывает только slug-часть; ULID-префикс константен

**Cons:**
- Filename длинный (≥30 chars + slug). На 80-колоночном терминале `ls -1` без TUI визуально тяжёл
- ID-префикс «вытесняет» slug в правый край → muscle-memory автодополнения слабее
- Obsidian-compat (D4): Obsidian резолвит `[[Title]]` exact-match по filename → пользователь не сможет печатать `[[on-fixed-ids]]` — нужны frontmatter `aliases:` для совместимости

### B. Slug-only filename

**Filename**: `<slug>.md`
**Пример**: `on-fixed-ids.md`
**Пустой slug fallback**: `<ULID>.md` (когда title не даёт slug)
**Slug-collision**: добавлять `-2`, `-3`, etc. при коллизии (lock-on-create, никогда не снимать суффикс)
**Frontmatter**: `id: 01J9XQK7ZBV5G2D8X3K7C4M9P0`, `title: "On fixed IDs"`
**Линки**: `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]` (резолвится через индекс — frontmatter-scan или persistent index)

**Pros:**
- Максимально читаемые filenames (`ls` приятен)
- Лучшая Obsidian-compat (D4): пользователь Obsidian может писать `[[on-fixed-ids]]`, и оно работает
- Короткие filenames → меньше визуального шума в каждом операционном инструменте

**Cons:**
- Slug-collision требует runtime-логики (`on-fixed-ids.md` уже занят → пишем `on-fixed-ids-2.md`); collision-suffix навсегда залочен (если первый `on-fixed-ids.md` потом удалили, новый всё равно `-2`)
- Нет хронологической сортируемости в `ls`
- Lookup `[[ULID]]` требует индекса (нет filename-hint для O(1)); cold-start индексирования = full scan frontmatter всех файлов
- При retitle slug меняется → file переименовывается; если новый slug совпадает с существующим → нужна disambig-логика
- Filename ничего не говорит о возрасте/идентичности — оператор делает `git log` по filename и видит «заметка появилась когда?» только через git history

### C. ULID-only filename

**Filename**: `<ULID>.md`
**Пример**: `01J9XQK7ZBV5G2D8X3K7C4M9P0.md`
**Frontmatter**: `id: 01J9XQK7ZBV5G2D8X3K7C4M9P0`, `title: "On fixed IDs"`, `slug: "on-fixed-ids"` (только в frontmatter, для preview-инструментов)
**Линки**: `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]` (резолвится напрямую — filename = ID)

**Pros:**
- Простейшая реализация: filename = ID, никакого парсинга, никаких rename-ов при retitle
- Никогда не возникает slug-collision (slug живёт только в frontmatter, без uniqueness-требования)
- Хронологическая сортируемость
- Ни один retitle не меняет filename → git diff чистый

**Cons:**
- `ls -1` показывает «стену ID» — оператор без TUI/fzf-preview не различает заметки на глаз
- Cognitive cost для terminal-native пользователя — STRATEGY-метрика time-to-first-link рискует пострадать
- Худшая Obsidian-compat (D4): Obsidian-пользователь видит wall of opaque IDs
- Slug всё равно нужен (для preview, для grep по содержимому), но «ушёл» в frontmatter — двойная индексация

## Trade-off matrix

| Сила | A: ULID-prefix + slug | B: slug-only | C: ULID-only |
|---|---|---|---|
| Стабильность через rename | ✓✓ ULID = anchor в filename | ✓ ULID в frontmatter, file переименовывается | ✓✓✓ filename вообще не меняется |
| Читаемость в `ls` | ⚠ длинный filename, slug в правом крае | ✓✓✓ clean | ✗ wall of ID |
| Читаемость в `fzf` (с preview) | ✓✓ | ✓✓ | ⚠ зависит от preview |
| Хронологическая сортировка | ✓ ULID-префикс | ✗ alphabetic by slug | ✓ ULID |
| Collision-safety filename | ✓✓ ULID гарантирует | ⚠ нужна disambig-логика на slug | ✓✓✓ ULID |
| Brevity линков | medium (`[[ULID]]`) | medium | medium |
| Lookup `[[ULID]]` | O(1) glob по префиксу | O(N) frontmatter scan или persistent index | O(1) точное имя файла |
| Toolchain interop (rg/fzf/grep) | ✓ rg по ID + slug | ✓✓ rg по slug | ⚠ rg только по содержимому |
| D4 (Obsidian compat) | ⚠ требует frontmatter `aliases:` для `[[Title]]`-резолва | ✓✓✓ нативно работает | ✗ |
| Implementation complexity | medium (parse ULID-prefix + frontmatter) | medium-high (collision-handling + index-required для resolve) | low (filename = ID) |
| Migration cost при retitle | medium (rename file) | medium (rename + collision check) | zero (никогда не rename) |

**Подсчёт «✓✓✓»**: A — 2, B — 3, C — 4. **Подсчёт «✗»**: A — 0, B — 1, C — 2.
**Качественно**: C простейший, но его худший cons (cognitive cost wall-of-ID и плохая D4-compat) ударяет в две STRATEGY-метрики (cross-session usage frequency и median time-to-first-link); B чище для Obsidian, но cons (collision-handling + persistent index для resolve) усложняет engine-track; A — самый сбалансированный, но требует acceptance длинного filename.

## Lean (моя рекомендация)

**Альтернатива A (ULID-prefix + slug).** Reasoning:

1. **Сохраняет все хорошие свойства ULID** (sortable, collision-safe) ровно там, где они нужны — в файловой системе.
2. **Длинный filename — приемлемая цена** в TUI-first-инструменте, где `ls` без preview не основной use case. fzf/TUI всегда покажут slug-часть выделенно.
3. **Lookup O(1)** через filename-glob — критично для capture latency budget из ARCHITECTURE §2.1; альтернатива B без persistent index промахивается.
4. **Hostile к D4 = read-write Obsidian-compat не блокирует**, потому что:
   - Obsidian пользователь, которому нужна совместимость, может включать в свой workflow frontmatter `aliases: [on-fixed-ids]` — Obsidian резолвит aliases через свой кэш.
   - В D4 решение всё равно будет о *степени* совместимости (read-only / read-write / strict-checker); A не закрывает ни один из этих уровней — просто требует нюанса в реализации.
5. **Community ideation** (Idea #5 от 2026-05-09, выживший с уверенностью 85%) уже указал в эту сторону — это не само по себе аргумент, но снимает «разработать с нуля» нагрузку.

Альтернатива B остаётся внутренне привлекательной для D4-heavy будущего; если в Phase 4 D4-cycle придёт к «zetto = strict checker over Obsidian vault» (idea variant F3.4 из ideation), пересмотр A1 станет оправданным. Но это не сейчас.

Альтернатива C формально проще, но «wall of ID в `ls`» — это UX-регрессия, которая режет primary-user identity (terminal-native, daily в `ls`).

## Explicitly not considered

| Вариант | Почему отброшен |
|---|---|
| **UUID v4** (`550e8400-e29b-41d4-a716-446655440000`) | 36 chars, не сортируется по времени, нечитаем. ULID даёт всё то же + хронология + 26 chars. Прежний zetto использовал UUID — это спайк, не контракт. |
| **Timestamp-only prefix** (`202605091230-on-fixed-ids.md`) | Community-feedback явно негативный («visually noisy»); в одной миллисекунде collision; нет entropy для cross-device. ULID решает все три. |
| **Luhmann hierarchical** (`1a2b3c-on-fixed-ids.md`) | Требует ручного выбора при создании (нарушает Q3 lean = auto-only); paper-era artifact, не несёт смысла в системе с FTS. |
| **Three-letter human** (`zk1.md`, `zk2.md` — F6.4 ideation) | Требует ручного выбора; vault-local uniqueness only (нарушает Q5 lean); не масштабируется выше ~17k заметок. |
| **Content-hash** (`a3f5b9.md`, например первые 6 hex SHA-256) | Меняется при любом edit → не ID, а версия. Не удовлетворяет силу «Стабильность через rename». |
| **Filename-as-title** (`On fixed IDs.md`, Obsidian-style) | Retitle ломает все backlinks (нарушает Q6 lean = ID константен). Также: spaces в filename — против toolchain-interop (нужно quote-ить в shell). |
| **Slug + UUID-suffix** (`on-fixed-ids-550e8400.md`) | UUID длиннее ULID, не сортируется. Если уж suffix — то ULID-suffix (но это лишь A с обратным порядком; рефлексирующее filename-расположение хуже для sort). |

## После выбора альтернативы

После того как пользователь укажет A/B/C (или модификацию):

- Phase 4 (Decide) → `decision-summary.md`, формализующий выбранную схему + edge cases (empty slug, slug-collision при B, max filename length, normalization rules для slug).
- Phase 5 (Roast → Meta-review) — обязательно для deep cycle.
- Phase 6 (Document) → ADR-0002 формального формата + amendments в `decision-map.md` (A1 → decided), `ARCHITECTURE.md` §5 (Decision index), `docs/architecture/README.md` (ADR index).
- Hand-off: ADR-0003 (link representation, A2 — следующий вопрос) разблокирован.
