# Discovery: A3 — Frontmatter convention

- **Date**: 2026-05-09
- **Cycle scale**: deep
- **Decision-map ref**: A3 — frontmatter schema; часть format-v1 контракта
- **Predecessors**: ADR-0002 (`id:` field), ADR-0003 (`title:` подразумевается через render-fallback)
- **Status**: round 1

## Проблема

ADR-0002 зафиксировал `id:` (ULID, canonical) во frontmatter; ADR-0003 — что render-fallback читает `title:` из frontmatter target-заметки. Это два обязательных поля. **A3 фиксирует полную схему frontmatter zetto-заметки**: какие поля есть, какие обязательны, какие optional, как они валидируются, что делает zetto при unknown полях, как пользователь добавляет custom-поля.

Решение становится частью **format-v1 spec contract** (см. A5). Изменения требуют либо additive minor-bump (если добавляем optional field), либо major-bump format-v2 + migration tool (если ломаем существующий контракт).

## Силы и драйверы

1. **Минимализм vs полнота схемы**. Минимальная схема (`id`, `title`) — низкий barrier-to-entry, простой парсинг. Богатая схема (10+ полей с предписанной семантикой) — сильнее enforcement, но выше cognitive cost при создании заметки.
2. **D4 (Obsidian-compat) anticipation**. Obsidian использует frontmatter поля: `aliases:` (для wikilink-резолва), `tags:` (для tag-pane), `cssclass:`, `created/updated`. Если zetto хочет D4=read-write — нужны как минимум `aliases:` + `tags:` совместимые.
3. **STRATEGY anti-pattern «теги ≠ ссылки»**. `tags:` в zetto должны быть **facets**, не связи; inline-hashtags `#tag` в body — синтаксис разный (если поддерживаем — нормализуем в `tags:` array).
4. **Lint-rule scope (C2a)**. Чем больше известных полей — тем больше правил можно написать (`zetto/missing-required-field`, `zetto/unknown-field`, `zetto/invalid-id-format`). Trade-off: rule maintenance vs strict guarantee.
5. **Render-fallback** (ADR-0003): `title` — mandatory для display-text без `[[ID|t]]`. Если `title` отсутствует — fallback на ULID literal (broken-style).
6. **Auto-managed fields** (`created`, `updated`). zetto может ставить timestamps автоматически при `zetto new`/`zetto save`. Trade-off: durability (timestamps в frontmatter переживают git-history-rewrite) vs simplicity (git mtime достаточен).
7. **Forward-compat для extensions**. Третьесторонние плагины/scripts могут хотеть свои поля (`my-script-state: ...`). Schema strictness определяет, разрешено ли это и как.
8. **YAML edge cases**. `gray_matter` write-back lossy для комментариев и порядка ключей (research-digest A1 §4 caveat). Schema должна предписывать **canonical key order** для диффа стабильности или принимать как есть.
9. **Migration cost**. Изменение required field — break format-v1; добавление optional — additive (no break). Stable schema позволяет accumulate user-content без migration.
10. **Aliases для D4**. Obsidian `aliases:` — список альтернативных titles, по которым wikilink резолвится. zetto использует ULID-резолв; aliases нужны только если D4=read-write (Obsidian читает zetto-vault и ожидает name-based wikilink). Решение «include aliases в v1» = подготовка к D4=read-write; «defer» = lean D4=own format / read-only.

## Связывающие ограничения (из ADR-0002 + ADR-0003 + STRATEGY/ARCHITECTURE)

- **`id:` обязательно**, ULID-format (ADR-0002 §Канонический ID).
- **`title:` подразумевается** для render-fallback (ADR-0003 §Render).
- **YAML format** (не TOML/JSON) — `gray_matter` + `serde_yaml` фиксированы в ADR-0002 §Crate dependencies; D4-compat (Obsidian/Logseq/Foam все ожидают `---`-delimited YAML).
- **Plain markdown на диске** — frontmatter в начале файла, `---`-разделители, body после.
- **Anti-pattern «теги ≠ замены ссылок»** — `tags:` как facet, не как linking mechanism.
- **Anti-pattern «папки-как-таксономия»** — frontmatter не должен дублировать папочную структуру.
- **format-v1 anchor (ADR-0002 §Format versioning anchor)**: A3 расширяет format-v1; additive changes в v1 ОК, breaking — format-v2 + migration tool.

## Прецеденты и сообщество (snapshot 2025–2026)

| PKM-tool | Required | Common optional | Notable |
|---|---|---|---|
| **Obsidian** | none mandated | `aliases`, `tags`, `cssclass`, `cover`, `publish`, `permalink` | YAML; Obsidian properties UI генерирует typed fields (text, number, date, list, checkbox); custom fields — preserve verbatim |
| **Logseq** | `title` (de-facto) | `tags`, `alias`, `id`, `public` | Page properties — frontmatter; block properties — inline `key:: value` |
| **Foam** | none mandated | `title`, `aliases`, `tags`, `date` | YAML; foam-specific fields prefixed with `foam-` |
| **Hugo** | `title`, `date` | `draft`, `slug`, `description`, `tags`, `categories`, `weight` | Hugo specifies extensive set; static-site generation |
| **Jekyll** | `layout` (de-facto) | `title`, `permalink`, `categories`, `tags`, `date` | YAML; hardcoded fields drive theming |
| **mdBook** | none | `title` (от первого H1) | Frontmatter not heavily used; book metadata в `book.toml` |
| **Org-roam** | `:ID:` (drawer-syntax) | `:ROAM_ALIASES:`, `:ROAM_REFS:`, `:CREATED:` | Org-mode drawers, не YAML |
| **neuron / Emanote** | `id` | `title`, `tags`, `description` | YAML (Emanote); neuron used `:title:` directive |

Закономерности:
- **`aliases:` стало de-facto Obsidian-стандартом** — для name-based wikilink резолва. Логично сохранять для compat с D4=read-write.
- **`tags:` как flat list** — universal; во всех инструментах списком в frontmatter + inline `#tag` в body (с разной полнотой нормализации).
- **`created:` / `updated:`** — встречается у Hugo/Jekyll-style tools; PKM (Obsidian/Foam/Logseq) — менее обязательны (полагаются на git/FS mtime).
- **Custom fields** — везде поддерживаются preserve-verbatim; Obsidian properties UI помечает как «Generic text»; Hugo передаёт в template-engine.
- **Schema strictness** — никто не делает hard-strict (reject unknown). Все принимают unknown fields.

`gray_matter` 0.2 (Rust) поддерживает любой YAML и `parse_with_struct::<T>` для typed-deserialization — значит, zetto может сериализовать known fields через struct, остальные — через `serde_yaml::Value` extra-bag.

## Открытые вопросы

Чтобы продвинуться к Design (3 альтернативы), нужно зафиксировать ответы:

1. **Минимальный обязательный набор полей.** Какие поля zetto **требует** во frontmatter каждой заметки?
   - *Lean*: `id` (ULID, ADR-0002), `title` (string, ADR-0003 render-fallback). Остальное — optional. Lint-rule `zetto/missing-required-field` flag-ит отсутствие.

2. **Standard optional fields.** Какие поля zetto **знает и обрабатывает специально** (не custom)?
   - *Lean*: `tags` (list), `aliases` (list — для D4 anticipation), `created` (RFC 3339 timestamp), `updated` (RFC 3339 timestamp). Минимум для PKM-функциональности. Все optional.

3. **`tags:` syntax**. YAML flat list / inline-hashtags в body / оба / только один?
   - *Lean*: YAML flat list `tags: [a, b, c]` — primary, canonical. Inline `#tag` в body — recognized at read (extracted into derived view), но НЕ сохраняется обратно в frontmatter. Тэги остаются facets.

4. **`aliases:` field в v1**. Включить (preparing для D4=read-write) или defer (D4 ещё open)?
   - *Lean*: **включить** в v1 как optional. Cost — низкий (одно YAML-поле + lint-rule); benefit — D4-cycle (когда наступит) не требует amendments к A3.

5. **`created` / `updated` semantics**. zetto auto-manages / user manages / git-derived / no auto-fields в v1?
   - *Lean*: `created` zetto auto-set при `zetto new`, format `RFC 3339` UTC. `updated` — user manages OR zetto auto-updates на save (config-knob; default — auto). Git-mtime fallback — нет (несовместимо с offline-edit).

6. **Schema strictness**. Strict (lint-flag на unknown fields) / lenient (preserve unknown verbatim) / hybrid (warn по умолчанию, off в strict-preset)?
   - *Lean*: **lenient by default** + lint-rule `zetto/unknown-frontmatter-field` (severity warn в `recommended-luhmann`, off в lenient preset). Preserve unknown — критично для extension/script-friendliness.

7. **Custom user fields namespace**. Free-form (`mood: happy`) / prefixed (`x-mood: happy`) / strict-deny / hybrid?
   - *Lean*: **free-form** для known-namespace + prefixed `x-*` для extensions/scripts. Lint flagging unknown without prefix — discourages typo-introduced silent customizations. Сообщество JSON Schema приняло `x-` — переносим convention.

## Заметки

- Q1 lean минимизирует обязательное (только `id` + `title`); это сохраняет low barrier-to-entry для свеже-созданных заметок (`zetto new` создаёт skeleton с двумя полями).
- Q2 lean определяет канонический набор; **порядок** в written frontmatter — отдельный sub-question (canonical order для diff-stability vs preserve-user-order). Lean — **canonical order** (`id`, `title`, `created`, `updated`, `tags`, `aliases`, then `x-*` extensions, then unknown), но это implementation-detail, не format-v1 contract.
- Q4 (`aliases`) — самый interesting trade-off. Включение готовит D4=read-write почти бесплатно. Defer экономит ~20 строк кода + один lint-rule сейчас, но требует amendment A3 при D4. Lean «include» — defensive против будущей миграции.
- Q5 — `updated` field semantics часто становится pain-point в PKM (Obsidian-плагины автоматически обновляют его, что создаёт mtime-noise; user-managed — забывают). Lean «zetto auto + config off» даёт обе опции.
- Q6/Q7 связаны: lenient + namespace для custom — это de-facto всех PKM. Strict (reject unknown) — too rigid для practical use.
- **NOT в скопе A3**: detail mapping каждого поля → lint rules (это C2a); render behavior каждого field (это implementation); template-engine (это outside scope — zetto не генерирует HTML beyond raw markdown).

## Что выйдет из ответов

После закрытия Q1–Q7:
- 3 альтернативы для Design — обычно (A) lean-set с canonical schema, (B) maximalist (богатый набор полей), (C) minimalist (только id+title, всё custom).
- Research нужен только для (a) state of frontmatter schema validation крейтов в Rust (есть ли что-то лучше `gray_matter`); (b) D4=read-write requirements от Obsidian — какие поля Obsidian реально использует.
- Decide → Roast → Meta-review → ADR-0004.
