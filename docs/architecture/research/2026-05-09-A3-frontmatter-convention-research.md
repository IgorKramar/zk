# Research digest: frontmatter conventions 2025–2026 (zetto A3)

- **Date**: 2026-05-09
- **Cycle**: A3 (Frontmatter convention), deep
- **Source**: archforge:researcher agent, 11 web queries
- **Status**: input для Phase 3 Design

## Headline finding

Из 7 leans только два опираются на устойчивые внешние конвенции (`tags`, `aliases` plural; lowercase `created`/`updated`). Остальные пять — особенно `x-*` префикс и lenient-схема — собственные дизайн-решения zetto без PKM-прецедента. Главные риски: (a) Obsidian 1.12.7 регрессия не резолвит `aliases` через wikilinks (D4-ожидание не оправдается — нужен собственный alias-resolver), (b) `gray_matter` всё ещё не делает round-trip fidelity → hand-rolled write предпочтительнее парсить-и-перепарсить, (c) inline `#tag` придётся реализовывать поверх pulldown-cmark вручную.

## Summary by topic

### 1. Obsidian `aliases:` resolution (актуально, D4-impacting)

**Obsidian 1.12.7** (май 2026): `aliases` парсятся и подсвечиваются автокомплитом, но **wikilink resolver их не учитывает**. Клик по `[[Alias]]` создаёт новую заметку вместо разрешения. Корень — `metadataCache.uniqueFileLookup` не пополняется aliases. **Активный регресс**, не legacy. **Импликация для zetto**: ship `aliases:` в v1 нормально (low cost), но «D4-readiness через Obsidian» — обещание неработающее в 2026. Нужен собственный alias-resolver в zetto, который читает `aliases:` из таргета.

### 2. Rust YAML schema validation — established patterns

- **`jsonschema` 0.42+** (MSRV 1.83): валидирует `serde_json::Value`. Pattern: `serde_yaml` → `serde_json::Value` → `jsonschema::validator_for(&schema)`. Iterate через `iter_errors()` — каждый error классифицируется как warn (unknown field) vs hard error (missing `id`/`title`).
- **`serde_valid`**: derive-based validation tied к структуре, compile-time. Custom validators через `#[validate(custom = ...)]`. Хуже подходит под lenient-режим (compile-time strict).
- **`yaml-schema`**: native YAML-валидатор без JSON-промежутка, structured JSON errors. Dependency-cost выше.

**Lean для zetto**: lenient-validation pattern через `jsonschema` поверх `gray_matter` parsed YAML (через intermediate JSON). Required-field-check внутри struct deserialize; unknown-field-detection через extra-bag `serde(flatten)` + iterate.

### 3. PKM frontmatter naming conventions — community converged on Obsidian

- **Lowercase singular `created`** + **`updated`** — устоялся как community default. Obsidian 1.9 (2025) deprecated singular `tag`/`alias`/`cssclass` в пользу plural lists `tags`/`aliases`/`cssclasses` — укрепляет plural-конвенцию для коллекций.
- **PKM ≠ static-site-generators**: PKM держится `created`/`updated`; Hugo использует `date`/`lastmod`/`publishDate`; Jekyll — `date`. Не выровнялись.
- **`created` в frontmatter, не FS mtime** — community recommendation: sync/backup tools сбивают mtime.
- **RFC 3339 UTC** для timestamps — корректно (Obsidian Bases августа 2025 делает date-арифметику).

**zetto-выбор `created`/`updated` совпадает с Obsidian-большинством.** Plural `tags`/`aliases` тоже совпадает.

### 4. Inline `#tag` parsing — hand-rolled поверх pulldown-cmark

- **Не first-class extension** в pulldown-cmark. CommonMark + GFM (tables, task lists, strikethrough, footnotes, math) — это всё.
- **Готового крейта** на crates.io нет.
- **Pattern**: `Parser::new(md).map(|ev| match ev { Event::Text(s) => extract_tags(s), _ => ev })`. CowStr держит zero-copy refs.
- **Edge cases (Obsidian Forum threads #18479, #6635)**:
  - `#tag` внутри `()` — Obsidian не парсит (open bug).
  - `#` в code blocks — false positives (regression-prone). pulldown-cmark выдаёт `Tag::CodeBlock`/`Code` события — fence-detection бесплатный.
  - URL-fragments `https://x.com/#section` — false positives.
  - `# Heading` — markdown заголовок, НЕ тэг (различение по start-of-line + space).
- **Regex**: `(?:^|\s)#([a-zA-Z][\w/-]*)` поверх `Text` payload. Whitelist Unicode classes если важно.
- **Cost**: ~50 строк.

### 5. `x-*` prefix convention — НЕТ PKM precedent

**Никто в PKM не использует `x-*`**. Альтернативные паттерны в индустрии:
- **Hugo**: namespacing под `params:` ключом (зарезервированные имена нельзя override).
- **Jekyll/Assemble**: free-form, всё в template engine.
- **GitHub Docs**: strict schema, custom rejected.
- **Typora**: tool-specific префиксы (`typora-root-url`).
- **Obsidian/Logseq/Foam**: free-form, никакой convention.

**Источник `x-*`** — OpenAPI/Swagger + JSON Schema. **RFC 6648 deprecated `X-` для HTTP headers** как antipattern. Ситуация двойственная.

**Импликация для zetto**: `x-*` — собственный design без community precedent. Не вредно, но не universally-recognised. **В ADR-0004 явно зафиксировать**: «we borrow from OpenAPI/JSON Schema, not from PKM ecosystem». Для round-trip-safe interop с Obsidian/Logseq irrelevant — они не строжат unknown поля.

### 6. `updated` auto-management — established pattern

**Канонический Obsidian-плагин** `obsidian-frontmatter-modified-date` (2024–2026) решает «mtime noise»:
- Hooks на `editor-change` event, **не** filesystem mtime — исключает sync/backup/Linter false positives.
- Опция «use typing events instead of Obsidian events» — strict-фильтрация (только реальный набор).
- Folder excludes (templates/ scripts/).
- Per-note opt-out через `exclude_modified_update: true`.
- 10-секундный debounce после последнего typing — иначе «external modification» warning.

**Pattern для zetto** (CLI, не editor):
1. На save: compute content-hash тела (без frontmatter).
2. Compare с last known hash (cached в frontmatter `x-content-hash:` или в external state).
3. Update `updated` iff hash changed.

Это исключает noise от frontmatter-only edits (добавление tag через UI). **Config-knob** (default-auto, можно отключить per-vault) — корректно, совпадает с community practice.

### 7. `gray_matter` round-trip fidelity — НЕ улучшилось

`gray_matter` дизайн ориентирован на **extraction**, не write-back. Никаких 2026 features про round-trip. Comments, key order, anchor preservation теряются.

**Альтернативы для preservation**:
- **`saphyr`** (успешник `yaml-rust2`, 2025): YAML 1.2 compliant, проходит yaml-test-suite. Comment-preserving round-trip **НЕ заявлен** как feature.
- **`serde-saphyr`**: serde-фреймворк поверх saphyr. 2000+ tests. Round-trip не цель.
- **`yamp`**: lightweight YAML parser, явно promo «comments как first-class». Минус: малый ecosystem.
- **`serde_yaml` ecosystem**: deprecated в марте 2024. Активные форки — `serde_yml` (controversial init), `serde-yaml-ng` (acatton), `serde_norway`, `yaml_serde` (от official YAML org). **Ни один не делает full round-trip с comments.**

**Вывод для zetto**: `gray_matter` для read остаётся (зафиксирован в ADR-0002). Для write — **hand-rolled** через шаблон в фиксированном порядке (`id`, `title`, `tags`, `aliases`, `created`, `updated`, `x-*`, then unknown). Не парсить-и-перепарсить — ручной control порядка, comments не сохраняются (в zetto-сгенерированных заметках их нет; user-edited frontmatter-edits zetto не пере-эмитит, только когда явно triggered retitle/save).

## Caveats and unknowns

- Obsidian aliases-resolver bug — на момент исследования открытый, патч не вышел; проверять перед finalize ADR.
- `yaml_serde` (от official YAML org) — поиск показал, но maturity vs `serde-yaml-ng`/`serde_norway` детально не оценил. Если ADR-0002 будет ревизировать выбор — отдельный round.
- Inline `#tag` parsing — не нашёл готового крейта; не исключено что есть в `marksman`, `tealdeer`, `silver-bullet` (адъяцентные Rust PKM).
- `x-*` convention — поискал mainstream PKM; не проверял niche (Athens, Roam-export, Foam community plugins). Не ожидаю precedent.
- «Update updated при изменении тела» — pattern существует, но точный invariant (hash тела vs hash полного документа vs typing events) — community split.

## Sources (24)

1. [Obsidian Forum #113902 — Wikilink resolution does not honor frontmatter aliases (1.12.7)](https://forum.obsidian.md/t/wikilink-resolution-does-not-honor-frontmatter-aliases-1-12-7/113902)
2. [docs.rs/jsonschema](https://docs.rs/jsonschema/latest/jsonschema/index.html)
3. [crates.io/yaml-schema](https://crates.io/crates/yaml-schema)
4. [practicalpkm.com — A Complete Guide to Obsidian Properties (2025)](https://practicalpkm.com/complete-guide-to-obsidian-properties/)
5. [github.com/alangrainger/obsidian-frontmatter-modified-date](https://github.com/alangrainger/obsidian-frontmatter-modified-date)
6. [Obsidian Forum #18479 — Parenthesis blocks hashtag parsing](https://forum.obsidian.md/t/parenthesis-and-more-will-block-hashtag-parsing/18479)
7. [Obsidian Forum #6635 — Link/tag autocompletion in code blocks](https://forum.obsidian.md/t/link-and-tag-autocompletion-activate-in-code-blocks-and-inline-code/6635)
8. [crates.io/gray_matter](https://crates.io/crates/gray_matter)
9. [github.com/saphyr-rs/saphyr](https://github.com/saphyr-rs/saphyr)
10. [github.com/acatton/serde-yaml-ng](https://github.com/acatton/serde-yaml-ng)
11. [Hugo Front matter docs](https://gohugo.io/content-management/front-matter/)
12. [GitHub Docs — Using YAML frontmatter](https://docs.github.com/en/contributing/writing-for-github-docs/using-yaml-frontmatter)
13. [Logseq Discuss — Frontmatter as standard YAML?](https://discuss.logseq.com/t/frontmatter-as-standard-yaml/27910)

(Полный список 24 источников — в transcript researcher-агента.)
