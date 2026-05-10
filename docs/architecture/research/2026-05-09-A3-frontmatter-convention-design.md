# Design: A3 — Frontmatter convention — alternatives

- **Date**: 2026-05-09
- **Cycle**: A3, deep
- **Inputs**: discovery (round 1, 7 leans приняты), research digest 2026-05-09
- **Status**: design phase, ожидает подтверждения альтернативы или pivot

## Зафиксированные предпосылки

Из ADR-0002 + ADR-0003 + 7 leans Discovery:
- `id:` обязательно (ULID, ADR-0002).
- `title:` обязательно (ADR-0003 render-fallback).
- YAML format (ADR-0002 §Crate dependencies: `gray_matter` + `serde_yaml`).
- Tags = facets (STRATEGY anti-pattern).
- `aliases:` включён в v1 для D4 anticipation.
- `created`/`updated` — auto-managed.
- Lenient strictness + `x-*` namespace для extensions.

Из research digest 2026-05-09:
- **Hand-rolled write предпочтительнее** парсить-и-перепарсить (`gray_matter` round-trip lossy).
- **Obsidian aliases regression в 1.12.7** — собственный alias-resolver в zetto обязателен; D4=read-write через Obsidian не работает «out of the box» — нужен active resolver.
- **`updated` content-hash pattern** от obsidian-frontmatter-modified-date — обновляется iff hash тела (без frontmatter) изменился.
- **`x-*` convention НЕ имеет PKM precedent** — в ADR-0004 явно зафиксировать как «borrow from OpenAPI/JSON Schema».

## Альтернативы

### A. Lean-set canonical schema (рекомендация)

**Required (zetto refuses to operate)**:
- `id: <ULID>` — string, regex `^[0-9A-HJKMNP-TV-Z]{26}$` (per ADR-0002).
- `title: <string>` — non-empty string, free-form UTF-8.

**Standard optional (zetto знает и обрабатывает специально)**:
- `tags: [<string>, ...]` — flat list строк (kebab-case, lowercase recommended; не enforce).
- `aliases: [<string>, ...]` — flat list альтернативных titles. Используется собственным zetto alias-resolver-ом (для name-based wikilink резолва — `[[Some Alias]]` находит заметку с `aliases: [some-alias, "Some Alias"]`).
- `created: <RFC 3339 timestamp>` — auto-set zetto при `zetto new`. UTC. Format: `2026-05-09T12:34:56Z`.
- `updated: <RFC 3339 timestamp>` — auto-set zetto при save iff content-hash тела изменился (pattern от obsidian-frontmatter-modified-date). Config-knob default-auto, opt-out per-vault или per-note (`x-skip-updated: true`).

**Custom user/extension fields**: `x-<name>` prefix. Lint flag `zetto/unknown-frontmatter-field` (severity warn в `recommended-luhmann`) flag-ит non-prefixed unknown fields. Prefixed `x-*` — preserved verbatim, никогда не lint-flag-ятся.

**Schema strictness**: lenient. Unknown без `x-*` prefix = lint warn (preserved verbatim, не блокирует операции). Required missing = lint **error** + zetto refuses mutate-операции (write/link/retitle); read-only операции работают с warning.

**Write strategy**: **hand-rolled** в фиксированном order (per research §7). zetto не парсит-и-перепарсит frontmatter; вместо этого собирает YAML строкой через шаблон. Order: `id`, `title`, `tags`, `aliases`, `created`, `updated`, then `x-*` extensions (alphabetical), then unknown без prefix (alphabetical, preserved). Comments в frontmatter — НЕ сохраняются (zetto не пишет комменты; user-edited комменты теряются на следующем zetto-write).

**Validation pipeline**:
1. Parse через `gray_matter` → YAML AST.
2. Extract known fields через `serde::Deserialize` struct + `serde(flatten)` extra-bag для unknown.
3. Required-check: `id` valid ULID, `title` non-empty.
4. Lint-pass: known optional fields — type-check; extra-bag — split into `x-*` (silent preserve) vs unknown (lint warn).

**Lint rules** (имена резервируются, semantics в C2a):
- `zetto/missing-required-field` (severity error) — `id` или `title` отсутствует.
- `zetto/invalid-id-format` (severity error) — `id` не валидный ULID.
- `zetto/empty-title` (severity warn) — `title: ""`.
- `zetto/unknown-frontmatter-field` (severity warn в `recommended-luhmann`, off в `lenient` preset) — поле без `x-*` prefix не в standard set.
- `zetto/non-rfc3339-timestamp` (severity warn) — `created`/`updated` не валидный RFC 3339.

### B. Maximalist (richer standard schema)

A + дополнительные standard fields:
- `description: <string>` — короткое описание для preview/search.
- `status: <draft|published|archived>` — lifecycle-стейт заметки.
- `type: <string>` — типизация заметки (note, daily, MOC, literature, evergreen).
- `prev: <ULID>` / `next: <ULID>` — sequence linking (Luhmann Folgezettel-style).
- `cssclasses: [<string>, ...]` — Obsidian-compat для styling.

**Pros vs A**: больше PKM-функциональности из коробки; Folgezettel-sequence (Luhmann methodology) поддерживается явно; lifecycle-states — basis для C2a-rules «archived заметки игнорируются в orphan-ratio».

**Cons vs A**: больше lint-rules для maintain; больше cognitive cost при создании заметки (что заполнять?); each field — потенциальная migration-точка через format-v2; Folgezettel — controversial (community split, см. forum.zettelkasten.de — есть proponents и detractors). Lifecycle-states пересекаются с C2a (rule engine может cover это как extension namespace).

*Lost because*: extra fields добавляют schema-surface без явного use case в pre-alpha. **Каждый из них может быть введён additive в format-v1 minor-bump** позже, когда use case появится. Vorab фиксировать — over-design.

### C. Minimalist (only `id`+`title` required; everything else custom)

**Required**: `id`, `title`.
**Standard optional**: нет. Любое дополнительное поле трактуется как custom.
**Schema strictness**: super-lenient — никаких unknown-field warnings.
**Custom field namespace**: free-form, no `x-*` convention.

**Pros vs A**: минимально строгий контракт; max user-customization friendliness; meta-cost zero (никаких lint-rules для standard fields); format-v1 имеет минимальный surface — meньше шансов сломать.

**Cons vs A**: **никакого PKM-функционала из коробки** — `tags`/`aliases`/`created`/`updated` каждый пользователь придумывает сам. **D4 incompatible** — Obsidian ожидает `tags`/`aliases` (Obsidian-property-UI). Lint-engine (C2a) не имеет surface для writing rules — zetto становится «editor with constraints», не «PKM with methodology». Anti-pattern относительно STRATEGY: research-grounded constraints предполагают enforcement, который не работает без known fields.

*Lost because*: ни одна цель STRATEGY не достигается без минимального standard set. C2a превращается в no-op. D4 закрыт.

## Trade-off matrix

| Сила | A: lean-set canonical | B: maximalist | C: minimalist |
|---|---|---|---|
| Required-set minimality | ✓✓ (только `id`+`title`) | ✓✓ | ✓✓✓ |
| Standard-set coverage | ✓✓ (4 поля + ext namespace) | ✓✓✓ (9 полей + ext) | ✗ (нет) |
| D4 (Obsidian-compat) | ✓✓ (`aliases`/`tags`/`cssclasses`-compat возможен через ext) | ✓✓✓ | ✗ |
| Implementation cost (lint rules) | medium (5 rules в v1) | high (8+ rules) | low (2 rules) |
| User customization friendliness | ✓✓ (`x-*` namespace) | ⚠ (богатая standard конкурирует с custom) | ✓✓✓ |
| format-v1 stability (additive only) | ✓✓ | ⚠ (больше surface = больше шансов сломать) | ✓✓✓ |
| Migration cost при добавлении поля | low (additive minor bump) | low (additive) | low (но всё custom — нет migration) |
| Round-trip fidelity (hand-rolled write) | ✓✓ (фиксированный order, simple) | ✓ (длиннее template) | ✓✓✓ (минимум полей) |
| C2a rule engine surface | medium (5 rules как basis) | large (8+ rules + status/type-aware rules) | none (нет polей для rule against) |
| STRATEGY-fit | ✓✓ (research-grounded constraints поддержаны) | ✓✓ | ✗ (constraints без полей не работают) |

**Подсчёт ✓✓✓**: A — 0, B — 3, C — 4. **✗**: A — 0, B — 0, C — 3.

Качественно: A — balance; B — over-design на уровне «фиксируем то, что не нужно»; C — под-design на уровне «без standard set нет продукта».

## Lean (моя рекомендация)

**Альтернатива A.** Reasoning:

1. **7 leans Discovery + research insights один-в-один соответствуют A.** Pivot на B потребует пересмотра lean Q2 (расширение standard set); pivot на C — пересмотра leans Q1/Q2/Q4/Q7.
2. **STRATEGY fit**: A даёт C2a (Methodology rule engine) минимальный surface для rules — `recommended-luhmann` preset может писать rules против `tags`, `aliases`, `created`, `updated`. C — нет; B — over-engineered.
3. **D4 anticipation**: A включает `aliases:` (Obsidian-compat key field). Research показал, что **Obsidian-resolver сейчас не использует aliases (1.12.7 regression)**, но zetto-resolver их использовать **сможет** для D4=read-write — это compensates за Obsidian bug.
4. **Hand-rolled write** (per research §7) — фиксированный order, comments не сохраняются. Это явная цена; принимается per «zetto не пере-эмитит user-edited frontmatter, кроме явных triggers (retitle, save с изменением известных полей)».
5. **`x-*` namespace** — собственный design без PKM precedent. ADR-0004 это явно зафиксирует; это **borrow from OpenAPI/JSON Schema**, не PKM-convention.
6. **`updated` через content-hash body** — pattern от obsidian-frontmatter-modified-date; community-tested.

**Cons A, принимаемые явно**:
- Comments в frontmatter теряются после zetto-write — known limitation, документируется. Митигация: zetto не пере-эмитит frontmatter если ничего не изменилось.
- `aliases:` в Obsidian-current-version не работает для wikilink-резолва — zetto-собственный resolver компенсирует. Если D4 пойдёт в read-write через Obsidian — рассчитывать на zetto-side resolution, не Obsidian-side.
- 5 lint rules в v1 — дополнительная сложность для C2a (но C2a их implementation так или иначе должен поддерживать — см. ADR-0003).

## Explicitly not considered

| Variant | Reason rejected |
|---|---|
| **TOML frontmatter** | ADR-0002 зафиксировал YAML; D4-compat (Obsidian/Logseq/Foam все ожидают `---`-delimited YAML); `gray_matter` setup. |
| **No frontmatter at all** | Required `id` и `title` (ADR-0002, ADR-0003) — нужен какой-то transport. |
| **Inline `key:: value` Logseq-style** | Non-YAML-compliant, сломает Obsidian/любой markdown-renderer; противоречит «plain markdown» STRATEGY anti-pattern. |
| **Strict schema (reject unknown)** | Kills extension-friendliness; никто из community PKM не делает; противоречит lean Q6 (lenient default). |
| **Properties в body/inline (`Properties: ...` heading-section)** | Sample mdBook pattern; ломает markdown-rendering для Obsidian/Logseq; cross-tool incompat. |
| **Hugo-style `params:` namespacing** | Adds nesting cost без явного benefit; PKM convention — flat. |
| **No `x-*` namespace, free-form custom (как Obsidian)** | Lean Q7 — `x-*` для discourage typo-introduced silent customizations; «no convention» дешевле, но теряет signal. |
| **Folgezettel `prev:`/`next:` в v1** | Часть Альтернативы B; controversial (community split в zettelkasten.de); добавимое позже как additive minor bump format-v1. |
| **Sigle-bracket value `key: value` без list-syntax для tags** | YAML allows `tags: tag1, tag2` (как scalar) — Obsidian принимает, но зависит от parser. List-syntax `tags: [a, b, c]` или multi-line — universally compatible. Принят list. |

## После выбора альтернативы

После подтверждения Альтернативы A (или модификации):

- Phase 4 (Decide) → `2026-05-09-A3-decision-summary.md` с full schema, validation pipeline, lint rules, hand-rolled write spec.
- Phase 5 (Roast → Meta-review) — обязательно для deep cycle.
- Phase 6 (Document) → ADR-0004 + amendments в `decision-map.md` (A3 → decided), `ARCHITECTURE.md` §5 + §6, `docs/architecture/README.md` ADR index, `docs/architecture/decisions/README.md` ADR index.
- A4 (Notes directory layout) и A5 (Format versioning) разблокированы полностью.
