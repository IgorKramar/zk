# Meta-review: ADR-0003

- **Date**: 2026-05-09
- **Reviewer**: meta-reviewer (archforge plugin self-conformance role)
- **Scope**: ADR-0003 (newly-written, deep-cycle, A2)
- **Source**: `docs/architecture/decisions/0003-link-representation.md`
- **Plugin source-of-truth**: archforge 0.4.0-rc3 (`templates/adr-template.md`, `skills/architect/SKILL.md`, `commands/adr.md`, `commands/roast.md`)

## Headline conformance findings

ADR-0003 в целом **соответствует** шаблону `templates/adr-template.md` и расширениям Nygard, прописанным в плагине: обязательные секции на месте, header-метаданные присутствуют, идентификаторы (`ADR-NNNN`, имена крейтов, API-имена `pulldown-cmark`, IDs `decision-map.md`) сохранены без перевода, прозу прошёл терминологический проход (на «обзервабилити»/«деплой»/«латенси» совпадений нет). Значимые расхождения — структурные: секция `## Alternatives considered` нарушает прескриптивный шаблон (использует bullet-list вместо `### 1. … ### 2. … ### 3. Status quo`) и расположена **выше** `## Decision`, а не ниже `## Consequences`. Индексы (`ARCHITECTURE.md` §5/§6, `decision-map.md`, `docs/architecture/README.md`) обновлены; `docs/architecture/decisions/README.md` отсутствует — но это repo-wide artefact (та же ситуация при ADR-0001 и ADR-0002), не специфика этого ADR.

## Per-rule conformance

### Required template sections

`templates/adr-template.md` фиксирует порядок: `# ADR-NNNN: <title>` → metadata → `## Context` → `## Decision` → `## Consequences` (`### Easier` / `### Harder` / `### Risks accepted`) → `## Alternatives considered` (минимум 2 альтернативы, включая status quo).

- `# ADR-0003: Link representation` ✓
- `## Context` ✓ (строка 11)
- `## Decision` ✓ (строка 33), но размещена **после** `## Alternatives considered` (строка 23) — **расхождение порядка секций (medium severity)**.
- `## Consequences` ✓ (строка 127), все три подсекции `### Easier` / `### Harder` / `### Risks accepted` на месте, заголовки **verbatim English**.
- `## Alternatives considered` ✓ (строка 23), но **формат расходится с шаблоном** (см. M-2 ниже).
- Расширения Nygard (`## Privacy and security considerations`, `## Implementation`, `## Open questions, отложенные в смежные ADR / v2`, `## Reviews trail`) — допустимы; шаблон явно не запрещает дополнительные секции.

### Header metadata

`**Date**`, `**Status**`, `**Authors**` — все на месте, корректные значения. Расширения (`Cycle`, `Predecessor`, `Decided`, `Affects`) — допустимы как Nygard-extended.

### Identifiers preserved

Полное соответствие. Проверено по всем категориям таксономии `architect/SKILL.md`:

- **B. Software/library**: `pulldown-cmark`, Obsidian, Logseq, Foam, Dendron, mdbook, pandoc, Hugo, vim, fzf, rg, git, BFG, `git filter-repo` — сохранены в латинской форме.
- **C. Standard abbreviations**: `ULID`, `ABI`, `LSP`, `TUI`, `CSS`, `HTML`, `FS`, `UTF-8`, `URL`, `CLI`, `PKM` — сохранены.
- **E. Artifact IDs**: `ADR-0001/0002/0003`, decision-map IDs (`A2/A3/A4/A5/B1/B2/C2a/C3/C4/C5/D1/D4`), finding IDs (`F-2`, `B-3..B-9`, `CC-1..CC-7`, `F-1`, `F-4`) — все в латинице.
- **F. Plugin template section names**: `Status`, `Context`, `Decision`, `Consequences`, `Alternatives considered`, `Easier`, `Harder`, `Risks accepted` — verbatim English.
- **API-имена**: `Tag::Link`, `LinkType::WikiLink`, `Options::ENABLE_WIKILINKS`, `has_pothole`, `dest_url`, `Event::Start`, `Event::Text`, `Tag::Image`, `Tag::CodeBlock`, `Code`, `Path::file_stem`, `Parser::new_ext`, `split_once`, `unwrap_or` — все сохранены.
- **Syntax forms**: `[[ID]]`, `[[ID|display]]`, `![[X]]`, `[[ID#H]]`, `[[ID#^block-id]]` — literally.
- **Lint-rule names** + `recommended-luhmann` preset name — сохранены.

### Language pass

Проза в русском; сканирование на калькированные формы из таблицы `architect/SKILL.md`:

- «обзервабилити», «деплой/деплоймент», «латенси/лейтенси», «перформанс», «резильентность», «фейловер», «скейлинг», «провижининг», «спанить», «бридж» — **не найдено**.
- Английские term-of-art сохранены в латинице с гидом или в общеизвестной форме: `forward-compat`, `weak forward-compat`, `term-of-art`, `breaking change`.
- Гибридные формы вида `defer-with-trigger`, `defer-state`, `out-of-the-box`, `read-write`, `read-only`, `read-compat`, `forward-compat`, `back-compat` — допустимая категория H term-of-art с гидом.
- «mitigation» как «митигирует/митигация», «inline-ит target body», «leak-ить» — гибридное склеивание; **L-1 ниже** — кандидаты на расширение калька-таблицы.

### Cross-references

Все ссылки разрешены:

- `ADR-0001`, `ADR-0002` — существуют ✓
- 4 research-файла + 2 reviews-файла A2 — существуют ✓
- `STRATEGY.md`, `ARCHITECTURE.md` — упоминаются с § references; § существуют (§2, §5, §6 в ARCHITECTURE.md проверены).
- Внутренние § references (B1 trigger, Deferred, Forward-compat, Privacy) — все секции существуют в ADR-0003.
- ADR-0002 references (`§ Crate dependencies`, `§ Format versioning anchor`, `§ Privacy and security considerations`) — все § существуют (orchestrator проверил вручную, grep строки 176/193/127 ADR-0002).

### Lifecycle states

- `Status: Accepted` — корректно для свеже-decided deep-cycle ADR ✓.
- ADR-0002 (предшественник) не помечен как `Superseded by ADR-0003` — корректно (ADR-0003 расширяет, не заменяет).
- Никаких edits-to-superseded не наблюдается.

### Roast trail linked

`commands/roast.md` step 6 предписывает запись `- YYYY-MM-DD — Roast (5 roles, severity: H/M/L counts) — [link]`. ADR-0003 строка 192:

```
- 2026-05-09 — Roast (5 roles, severity: 3H/13M/8L) — [00-summary.md](../reviews/2026-05-09-roast-A2-link-representation/00-summary.md)
```

Формат **соответствует** ✓. Строка с meta-review on roast — допустимое расширение.

### Indexes updated

- `ARCHITECTURE.md §5 Decision Index` — запись для ADR-0003 ✓
- `ARCHITECTURE.md §6 Open Questions` — Q2 закрыт через strikethrough + reference ✓
- `decision-map.md` A2 — `Status: decided` со ссылкой на ADR-0003 ✓; `Suggested order` пересобран ✓
- `docs/architecture/README.md` — запись для ADR-0003 ✓
- `docs/architecture/decisions/README.md` — **отсутствует** в проекте; **M-3 ниже** (repo-wide, не специфика ADR-0003).

## Recommended fixes

### M-1 (medium): Порядок секций нарушает шаблон

`## Alternatives considered` (строка 23) расположена **между** `## Context` и `## Decision`. Шаблон фиксирует: Context → Decision → Consequences → Alternatives.

**Fix**: переместить блок `## Alternatives considered` ниже `## Consequences` (после `### Risks accepted`, до `## Privacy and security considerations`).

### M-2 (medium): Формат `## Alternatives considered` не соответствует шаблону

Шаблон (строки 49–69 `templates/adr-template.md`) предписывает структуру `### 1. <name>` / `### 2. <name>` / `### 3. Status quo / do nothing`, каждая с описанием опции, её сильных/слабых сторон и причины отклонения (2–4 строки).

Текущий формат — bullet-list `**A.** … **B.** … **C.** …` без подсекций H3 и без явного **Status quo** варианта.

**Fix**: переписать секцию в `### 1. Wikilink-primary с canonical markdown read-compat (выбрана)` / `### 2. Markdown-only canonical` / `### 3. Full-Obsidian-superset` / `### 4. Status quo — отложить A2 до B1` (или аналогичная status-quo-формулировка).

### M-3 (medium, repo-wide): Отсутствует `docs/architecture/decisions/README.md`

`commands/adr.md` step 5 предписывает обновлять `docs/architecture/decisions/README.md`. В проекте этого файла нет — индекс ведётся в `docs/architecture/README.md`. Та же ситуация была при ADR-0001 и ADR-0002. Не специфика этого ADR.

**Fix (опциональный, repo-wide)**: создать `decisions/README.md` со списком ADR-0001/0002/0003. Альтернативно — принять текущую конвенцию «индекс в `docs/architecture/README.md`» и зафиксировать её отдельным решением.

### L-1 (low, non-blocking): Кандидаты на расширение калька-таблицы

Гибриды «inline-ит», «leak-ить», «митигирует/митигация» — judgment-call, не нарушение. Если решено принять — упомянуть пользователю и предложить дополнить таблицу в `architect/SKILL.md`.

## Verdict

**CONFORMING with minor template-order divergences.**

Артефакт соответствует обязательным правилам плагина по идентификаторам, метаданным, языковому проходу, статусу, обновлению индексов (за вычетом отсутствующего `decisions/README.md`, который — общая проблема репо), линку на roast trail. Расхождения с шаблоном — на уровне порядка секций (M-1) и формата `Alternatives considered` (M-2). Оба — точечные текстовые правки, не требуют пересборки решения и не блокируют принятие. Архитектурное содержание не оценивалось (не моя роль).

## Areas not covered by this review

- Архитектурная корректность решения (роль `architect`, `roast/devil-advocate`).
- Качество альтернатив и полнота их сравнения (роль `roast/devil-advocate`, `pragmatist`).
- Реалистичность trigger conditions для defer-фич (роль `futurist`, `pragmatist`).
- Корректность приватность-анализа (роль `compliance-officer`).
- Ясность для новичка через 6 месяцев (роль `junior-engineer`).
- Соответствие STRATEGY.md и реальным метрикам пользователя (роль `architect` в Discover).
- Проверка edits-to-accepted через `git diff` (требует git access; статический проход не делает).
