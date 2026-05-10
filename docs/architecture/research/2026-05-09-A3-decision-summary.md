# Decision summary: A3 — Frontmatter convention

- **Date**: 2026-05-09
- **Cycle**: A3, deep
- **Will become**: ADR-0004
- **Inputs**: discovery (7 leans приняты), research digest, design (Альтернатива A выбрана)
- **Status**: pending roast + meta-review

## Решение (одной фразой)

YAML frontmatter с **двумя обязательными полями** (`id`, `title`) и **четырьмя стандартными опциональными** (`tags`, `aliases`, `created`, `updated`); custom-поля под namespace `x-*`; схема **lenient** (preserve unknown verbatim, lint warn для unprefixed unknown); **hand-rolled write** в фиксированном порядке полей.

## Декомпозиция

### Required fields (zetto refuses mutate-операции при отсутствии)

- **`id: <ULID>`** — string, regex `^[0-9A-HJKMNP-TV-Z]{26}$`. Per ADR-0002. Lint rule `zetto/missing-required-field` (severity error) + `zetto/invalid-id-format` (severity error).
- **`title: <string>`** — non-empty UTF-8 string. Per ADR-0003 render-fallback. Lint rule `zetto/empty-title` (severity warn).

### Standard optional fields (zetto знает и обрабатывает специально)

- **`tags: [<string>, ...]`** — flat YAML list. Tags = facets (STRATEGY anti-pattern «теги ≠ ссылки»). Recommendation (не enforce): kebab-case lowercase. Inline `#tag` в body — recognized at read для derived view (lint surface для C2a `zetto/tag-not-in-frontmatter`), но zetto не сохраняет автоматически в `tags:` array.
- **`aliases: [<string>, ...]`** — flat YAML list альтернативных titles. Используется **zetto-собственным alias-resolver-ом** для name-based wikilink резолва (`[[Some Alias]]` находит заметку с `aliases: ["Some Alias", "some-alias"]`). **Caveat (per research §1)**: Obsidian 1.12.7 регресс — wikilink resolver там **не учитывает** aliases; D4=read-write через Obsidian native не работает out-of-the-box, нужен zetto-side resolver.
- **`created: <RFC 3339 timestamp>`** — auto-set zetto при `zetto new`. UTC. Format example: `2026-05-09T12:34:56Z`. User-edit-able (zetto preserves on subsequent saves).
- **`updated: <RFC 3339 timestamp>`** — auto-managed zetto. **Pattern (per research §6)**: на save zetto compute content-hash тела (без frontmatter), compare с last-known hash, update `updated` iff hash changed. Это исключает noise от frontmatter-only edits. Config-knob `update_timestamp: auto | manual | off` (default auto). Per-note opt-out через `x-skip-updated: true`.

### Custom user/extension fields

**Convention**: `x-<name>` prefix (borrow from OpenAPI/JSON Schema). **Без PKM precedent** — собственный design zetto, документируется явно в ADR-0004.

- **Prefixed `x-*`** (например `x-mood: happy`): preserved verbatim, никогда не lint-flag-ятся. Используются для user scripts, extension plugins, опытных config-knobs (`x-skip-updated`).
- **Unprefixed unknown** (например `mood: happy` без `x-`): preserved verbatim, но **lint flag `zetto/unknown-frontmatter-field`** (severity warn в `recommended-luhmann` preset, off в `lenient` preset).

### Schema strictness — lenient

- **Required missing** → lint error + zetto refuses mutate-операции (write/link/retitle); read-only операции работают with stderr warning.
- **Required invalid format** (например, `id` не валидный ULID) → lint error + same refuse-mutate.
- **Standard optional invalid format** (например, `created` не RFC 3339) → lint warn (`zetto/non-rfc3339-timestamp`); zetto не блокирует операции.
- **Unknown without `x-*` prefix** → lint warn; preserved verbatim.
- **Unknown с `x-*` prefix** → silent preserve.

### Validation pipeline

1. **Parse** через `gray_matter` (per ADR-0002 §Crate dependencies) → YAML AST.
2. **Deserialize** в struct с `serde::Deserialize`:
   ```rust
   #[derive(Deserialize)]
   struct Frontmatter {
       id: Ulid,
       title: String,
       #[serde(default)] tags: Vec<String>,
       #[serde(default)] aliases: Vec<String>,
       #[serde(default)] created: Option<DateTime<Utc>>,
       #[serde(default)] updated: Option<DateTime<Utc>>,
       #[serde(flatten)] extra: BTreeMap<String, serde_yaml::Value>,
   }
   ```
3. **Required-check**: `id` parsed (else lint error), `title` non-empty.
4. **Optional-check**: `created`/`updated` parse как RFC 3339 (else lint warn).
5. **Extra-bag split**: split keys по `x-` prefix → silent preserve vs lint-warn unknown.

### Write strategy — hand-rolled fixed order (per research §7)

zetto **не парсит-и-перепарсит** frontmatter (gray_matter round-trip lossy). Вместо этого собирает YAML строкой через шаблон:

```
---
id: <ULID>
title: <quoted-title>
tags: [<tag1>, <tag2>]              # only if non-empty
aliases: ["<alias1>", "<alias2>"]    # only if non-empty
created: <timestamp>                 # only if set
updated: <timestamp>                 # only if set
<x-extensions in alphabetical order>
<unknown without prefix in alphabetical order, preserved verbatim>
---
```

**Trade-off accepted**: comments в frontmatter теряются после zetto-write. Митигация: zetto **не пере-эмитит** frontmatter если zetto-known fields не изменились — user-edited комменты сохраняются до первого retitle/save с изменением известного поля. Документируется как known limitation.

**YAML quoting policy**: zetto quotes title и aliases (избегает edge cases с YAML-special-chars `:`, `#`, `[`, `]`, `&`, `*`, `?`); tags — bare если match `^[a-zA-Z][\w-]*$`, иначе quoted.

### Lint rules (имена резервируются здесь, semantics в C2a)

| Rule ID | Description | Default severity |
|---|---|---|
| `zetto/missing-required-field` | `id` или `title` отсутствует | error |
| `zetto/invalid-id-format` | `id` не валидный ULID (regex или out-of-range timestamp) | error |
| `zetto/empty-title` | `title: ""` | warn |
| `zetto/non-rfc3339-timestamp` | `created` или `updated` не валидный RFC 3339 | warn |
| `zetto/unknown-frontmatter-field` | поле без `x-*` prefix не в standard set | warn в `recommended-luhmann`, off в `lenient` |
| `zetto/tag-not-in-frontmatter` | inline `#tag` в body найден, но не в `tags:` array | info (suggestion) |

Семантика error/warn/info — определяется в C2a (rule engine architecture, open).

### `aliases` resolver behavior (для D4 anticipation)

zetto при резолве `[[X]]` после стандартных проходов (per ADR-0003 §Resolver):
1. ULID literal match — primary.
2. **Filename-prefix match** (если ULID-prefix-extract нашёл).
3. **Alias match** — scan frontmatter `aliases:` всех заметок vault-а через index (B1) или synchronous scan в v1; case-insensitive по умолчанию.
4. **Filename match** (legacy / imported content).

При multiple matches (alias collision между заметками) — lint flag `zetto/ambiguous-alias-resolution` (severity warn); zetto предпочитает первый по ULID-creation-time.

### Forward-compat statement

A3 расширяет format-v1 spec contract additive way: **новые standard fields могут быть добавлены в minor format-v1.x bump** (additive change, не breaking). Например, future `description:`, `status:`, `cssclasses:` (отложены в B-альтернативу) — добавятся при появлении use case без format-v2 migration. **Изменение существующего contract** (например, `id` теперь UUIDv7 вместо ULID, `tags` теперь nested namespace вместо flat list) — требует format-v2 + migration tool.

**Schema-version field**: `format: 1` — НЕ требуется (per [ADR-0002 § Format versioning anchor]). format-v1 — implicit; vault без `format:` field считается v1. format-v2 (если когда-то наступит) фиксирует `format: 2` обязательным.

## Edge cases

- **Empty frontmatter `---\n---\n<body>`** или **отсутствует frontmatter** (только body) — lint error `zetto/missing-required-field`; zetto refuses mutate.
- **`id:` присутствует, но `title:` отсутствует** — то же самое; refuses mutate.
- **`tags: []` (empty list)** — валидно; preserved verbatim; ничего не lint-flag-ит.
- **`tags: tag1`** (scalar вместо list) — `gray_matter` обычно coerce-ит scalar to single-element list. zetto принимает; lint flag `zetto/non-canonical-tag-format` (info) с suggestion канонизировать в list при следующем zetto-write.
- **`aliases: ["", "  "]`** (пустые/whitespace alias-ы) — preserved verbatim (lenient); lint warn `zetto/empty-alias`.
- **`x-` без имени** (e.g. `x-: value` или `x: value`) — `x-` rejected как malformed extension; `x` без `-` — обычное unknown field (lint warn).
- **Дублирующиеся ключи** в YAML (например, два `tags:`) — `gray_matter`/`serde_yaml` обычно takes last; zetto не fix-ит, lint flag `zetto/duplicate-frontmatter-key` (warn).
- **Очень длинный `title`** (>200 chars) — preserved verbatim; lint info `zetto/long-title` (suggestion, не warn).

## Privacy and security considerations

- **`title:` в frontmatter** — sensitive title leak channel при export/share одной заметки (см. ADR-0003 §Privacy CC-2). Этот ADR не меняет позицию; future export-mode (отдельный ADR) определит redaction policy.
- **`created:` timestamp** — раскрывает время создания при share. Дублирует ULID-decoded time (per ADR-0002 §Privacy); явное `created:` поле — additional privacy surface, но low because ULID уже это раскрывает.
- **`aliases:` field** — может leak alternative human-readable forms title (например, code-name или nickname). При share — те же caveats как `title:`.
- **Custom `x-*` fields** — пользователь несёт ответственность; zetto не sanitize.
- **`updated:` content-hash** — если zetto cache-ит hash в frontmatter (`x-content-hash`), — это дополнительная metadata surface (длиной ~64 hex chars). Recommendation: cache hash в external state (sqlite в B1), не в frontmatter, чтобы не загрязнять user-visible data.

## Open questions, отложенные в смежные ADR / v2

- **A4** (notes directory layout): не зависит от A3, но layout определит пути в filename-globs (см. ADR-0003 §Resolver).
- **A5** (format versioning policy): A3 фиксирует additive minor-bump policy в этом ADR; full format-v1 spec — в A5.
- **B1** (graph index): `aliases:` resolver требует index для O(1) lookup (synchronous scan слишком дорог per ADR-0003 §B1 trigger). Когда vault > 500 заметок — alias-резолюция станет блокером без B1.
- **C2a** (rule engine): 6 lint rules имена резервированы здесь; semantics + engine architecture в C2a (open).
- **D4** (Obsidian compat posture): `aliases:` design предполагает D4 ∈ {own format, read-only, read-write через aliases via zetto-side resolver}. Strict-checker variant исключён в ADR-0002. Если D4 пойдёт в read-write, **zetto-side aliases-resolver работает; Obsidian-side — current bug 1.12.7**.
- **format-v1.x additive bumps** (description, status, type, prev/next, cssclasses) — отложены до конкретного use case.

## Что дальше (после roast/meta-review)

1. Phase 5 — `/archforge:roast` 5 ролей.
2. Phase 5b — `/archforge:meta-review` на roast-output.
3. Phase 6 — пользователь выбирает: (a) применить findings → ADR-0004, (b) применить + re-roast, (c) шаг назад.
4. Phase 7 — `/archforge:document`: ADR-0004 + amendments в `decision-map.md` (A3 → decided), `ARCHITECTURE.md` §5 + §6, `docs/architecture/README.md` ADR index, `docs/architecture/decisions/README.md` ADR index.
5. A4 (Notes directory layout) и A5 (Format versioning policy) разблокированы полностью; Group A на финишной прямой.
