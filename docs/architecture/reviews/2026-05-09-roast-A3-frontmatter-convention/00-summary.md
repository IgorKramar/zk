# Roast: A3 — Frontmatter convention

**Target**: `docs/architecture/research/2026-05-09-A3-decision-summary.md`
**Date**: 2026-05-10
**Roles run**: Devil-advocate, Pragmatist, Junior-engineer, Compliance-officer, Futurist

## Headline findings

- **Devil-advocate**: B-1 — `updated` content-hash паттерн **недетерминирован при первом save после внешнего edit** (git pull, vim напрямую, restore from backup). Last-known hash негде хранить (frontmatter — circular trigger; external state — B1 ещё нет). Каждый pull генерирует либо frontmatter-only-churn в git, либо систематически lying `updated:` timestamps.
- **Pragmatist**: H-5 — first-day experience: заметка без `title:` блокирует **все** mutate-операции; пользователь делает `zetto new`, удаляет `title:` в редакторе случайно (typo, paste-over) → `zetto link` refuses. Противоречит STRATEGY-метрике time-to-first-link. ADR не описывает, как `zetto new` гарантирует валидный `title:` после первого save.
- **Junior-engineer**: J-1 — «Альтернатива A», «B-альтернатива», «7 leans приняты» в шапке без раскрытия; читателю нужно открывать discovery/research/design files которые в посылке не упомянуты.
- **Compliance-officer**: C-2 — `x-*` extension namespace принимает любые user-defined fields verbatim, включая случайно вставленные секреты/credentials; zetto не sanitize. Усиление title-leak / aliases-leak / created-leak surfaces из ADR-0002/0003 § Privacy.
- **Futurist**: F-1 — `aliases:`-резолвер становится **канонической**, не временной мерой. Заявленный как обходной путь под Obsidian 1.12.7 регресс, через 1–2 года имеет собственный contract (правила нормализации, поведение при коллизии, кэш) — удалять дороже, чем содержать.

## Severity counts

| Role | High | Medium | Low |
|---|---|---|---|
| Devil-advocate | 3 (B-1, B-2, B-3) | 3 (B-4, B-5, B-6) | 2 (B-7, B-8) |
| Pragmatist | 2 (H-3, H-5) | 4 (H-1, H-2, H-4, H-6) | 1 (H-7) |
| Compliance-officer | 0 | 2 (C-2, C-5) | 4 (C-1, C-3, C-4, C-6) |
| Junior-engineer | — | — | — |
| Futurist | — | — | — |

(Junior-engineer и Futurist по разрешению `commands/roast.md` строка 109 не используют severity-категории.)

## Cross-cutting concerns

### CC-1: `updated` content-hash паттерн без external state — broken в pre-B1 (3 роли)

- **B-1**: last-known hash негде жить; pre-B1 — либо frontmatter (circular, contradicts privacy), либо behavior undefined.
- **H-4**: «just compute hash» с iceberg — где хранится, какой алгоритм нормализации (CRLF/BOM), когда compute, транзакционность file vs state-store.
- **F-5**: ломается на batch-операциях — rename/reformat/migration не двигают content-hash, но `updated:` должен бы.

**Underlying issue**: pre-B1 либо отказаться от auto-managed `updated:` (документировать как «manual в v1, auto в v1.x после B1»), либо принять «mtime fallback с известными sync caveats» (в нарушение research §6 рекомендации). ADR должен явно выбрать одну позицию вместо implicit «B1 решит».

### CC-2: Hand-rolled write — claim about preserve-verbatim противоречит реальности (2 роли)

- **B-2**: «hand-rolled template» + «preserve unknown verbatim в alphabetical order» противоречат для non-trivial YAML значений (multi-line scalars, anchors, nested). zetto либо silently re-formats, либо падает на nested YAML.
- **H-2/H-6**: «не пере-эмитит если known fields не изменились» = hand-rolled diff, не template; quoting state machine для каждого условия; ~30–50 unit tests, мультипликативно растёт с new fields.

**Underlying issue**: либо честно зафиксировать в ADR «zetto-managed frontmatter не сохраняет комменты, не делает byte-level preserve unknown nested YAML — только flat scalars», либо специфицировать YAML-lexer для byte-level diff (~500–1000 LOC, не «hand-rolled template»). Текущий язык overpromises.

### CC-3: Concurrency и atomicity write — silent (1 роль)

- **B-3**: pre-alpha CLI/TUI — между read и write нет file lock, нет CAS-проверки на mtime. Параллельный процесс (TUI в одном терминале + `zetto retitle` в другом, или vim параллельно) перезаписывает файл, partial write оставляет corrupt frontmatter.

**Underlying issue**: ADR должен описать concurrency-policy: `atomic-write-file` (уже зафиксирован в ADR-0002), CAS на mtime/hash перед write, либо явное «zetto предполагает single-writer; concurrent writes — undefined behavior». Без этого — потерянные правки и corrupt files.

### CC-4: `aliases:`-резолвер — temporary vs canonical (2 роли)

- **F-1**: становится permanent zetto-side обязательством; через 2 года имеет собственный contract.
- **B-4**: case-insensitive collision DOS — vault 5000 заметок с `aliases: [TODO]` в каждой третьей даёт 8.5M lint warnings.

**Underlying issue**: ADR должен зафиксировать в одну сторону — либо «aliases — постоянная часть contract» (с upfront-инвестициями в индекс, лимиты на N collision, полноценный resolver), либо «aliases — preserve verbatim только; resolver выключен в v1» (per pragmatist H-3 предложение редукции).

### CC-5: First-day experience и onboarding — silent (2 роли)

- **H-5**: заметка без `title:` блокирует все mutate-операции; user-frustration на первом capture.
- **J-1, J-2, J-3, J-4, J-5, J-6, J-7, J-8, J-9, J-10**: documentation gaps создают barrier для нового читателя.

**Underlying issue**: ADR должен описать `zetto new`-template: что в skeleton-файле включено по умолчанию (`title: <slug-derived>`?), prompt-on-empty, auto-derive from H1, или что-то ещё. Plus explicit cross-reference в шапке к discovery/research/design.

### CC-6: `x-*` namespace — design без PKM precedent (1 роль)

- **F-2**: становится «онбординг-налогом» — каждый user первый раз спрашивает «что это»; в первые 6–12 месяцев требует постоянной защиты в прозе.

**Underlying issue**: ADR должен включить «rationale block» с 1–2 рассмотренными альтернативами префикса (`zetto:`-prefix, `_`-prefix, free-form vendor-extensions). Не сейчас аргументировать в воздух — зафиксировать на будущее, чтобы дискуссия не открывалась с нуля.

### CC-7: Defer-list (description/status/type/prev/next/cssclasses) → forever-deferred (1 роль)

- **F-3**: defer без trigger condition становится permanent; пользователи де-факто используют `x-status`, `x-description` — promotion в standard set создаёт breaking change в семантике.

**Underlying issue**: ADR должен либо ввести trigger conditions для каждого deferred поля («3 случая `x-status` в реальном vault — promote»), либо явно «defer = non-goal до format-v2».

## Recommended path

**Apply findings, then proceed to Document.** Decision (Альтернатива A — lean-set canonical schema) **не оспорена** ни одной ролью. Cross-cutting CC-1, CC-2, CC-3, CC-4, CC-5 указывают на текстовые правки или architecture-level clarifications (concurrency-policy, write-strategy honest scope, aliases-permanence). CC-6, CC-7 — мелкие дополнения для будущей ясности.

Конкретные правки в ADR-0004 (по приоритету):

1. **CC-1 / B-1, H-4, F-5**: явно зафиксировать в § Standard optional fields `updated:` — pre-B1 поведение определено («manual в v1, auto в v1.x после B1»). Trigger для активации auto-управления.
2. **CC-3 / B-3**: добавить § Concurrency and atomicity — `atomic-write-file` per ADR-0002, single-writer assumption, CAS на mtime для обнаружения concurrent edit.
3. **CC-2 / B-2, H-2, H-6**: переформулировать «hand-rolled write» — explicit scope: «zetto-managed write для known fields в фиксированном порядке + flat preserve unknown scalars; nested unknown YAML — best-effort через `serde_yaml::to_string`, может потерять comments/anchors».
4. **CC-4 / F-1, B-4**: добавить в § Open questions явно «aliases-resolver — постоянная часть on-disk contract». Плюс лимиты на alias-collision (например, N matches > 100 — alias-rule disabled с lint info `zetto/alias-collision-saturation`).
5. **CC-5 / H-5**: § `zetto new` skeleton template — `title: ` (empty placeholder) + prompt при first save; auto-derive from H1 если body содержит heading. ADR-0003 §empty-slug lifecycle уже описывает похожий механизм — extend на title.
6. **CC-6 / F-2**: rationale block для `x-*` choice (alternatives: `zetto:`, `_`, free-form).
7. **CC-7 / F-3**: trigger conditions для defer-list или explicit «defer = non-goal до format-v2».
8. **F-4** (read→write идемпотентность invariant): test-обязательный invariant в § Implementation.
9. **F-6** (`zetto/*` namespace reserved): одна строка в § Lint rules.
10. **F-7, F-9** (`gray_matter` vendor risk + Obsidian regress): в § Risks accepted.
11. **J-1..J-12** (clarity): inline definitions, Reviews trail, cross-references.

После применения — proceed to Document (ADR-0004). Re-roast не требуется.

## Per-role outputs

- [01-devil-advocate.md](./01-devil-advocate.md) — 8 findings (B-1 … B-8); 3 high, 3 medium, 2 low
- [02-pragmatist.md](./02-pragmatist.md) — 7 findings (H-1 … H-7); 2 high, 4 medium, 1 low
- [03-junior-engineer.md](./03-junior-engineer.md) — 12 findings (J-1 … J-12); severity не назначен
- [04-compliance-officer.md](./04-compliance-officer.md) — 6 findings (C-1 … C-6); 0 high, 2 medium (C-2, C-5), 4 low (C-1, C-3, C-4, C-6)
- [05-futurist.md](./05-futurist.md) — 9 findings (F-1 … F-9); 6 structural (high-confidence), 3 trend (medium/low confidence)

**Total**: ~42 findings across 5 roast roles.
