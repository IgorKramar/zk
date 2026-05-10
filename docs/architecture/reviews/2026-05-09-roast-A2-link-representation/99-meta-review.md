# Meta-review: roast A2 — Link representation

**Target**: `/Users/user/Work/self/zk/docs/architecture/reviews/2026-05-09-roast-A2-link-representation/` (6 файлов: `00-summary.md`, `01-devil-advocate.md`, `02-pragmatist.md`, `03-junior-engineer.md`, `04-compliance-officer.md`, `05-futurist.md`)
**Date**: 2026-05-10
**Plugin version**: archforge 0.4.0-rc3
**Spec source**: `commands/roast.md` строки 80–124 (структура summary), 156–167 (template integrity), 162 (finding-ID prefixes)

## Summary

Артефакты в целом конформны: все шесть файлов на месте, структура `00-summary.md` соответствует прескриптивной, identifiers сохранены повсеместно (ADR-NNNN, decision-map IDs, crate names, API names, syntax forms), русская проза с английскими структурными заголовками выдержана, все ссылки разрешаются. Главный дефект — **систематическое расхождение по prefix-у Pragmatist finding-ID**: в `02-pragmatist.md` используется `P-N` (что согласуется с заданием пользователя), но `commands/roast.md` строка 162 предписывает Pragmatist-у prefix `H-N`. Это противоречие между заданием пользователя и строкой 162 спеки фиксируется как **M-1 high**: артефакты разошлись со спекой плагина (хотя последовательны внутри себя). Остальные находки — низкого/среднего веса: один сломанный per-role файл (`03-junior-engineer.md` начинается с `## Junior-engineer findings` без header-метаданных, без `# Title`), несколько мелких header-format inconsistencies, и одна структурная странность с `04-compliance-officer.md` / `05-futurist.md` — они тоже начинаются без `# Title` header.

## Findings

### M-1: Pragmatist finding-ID prefix `P-N` расходится со спекой `H-N`

**Category**: identifier preservation / template conformance
**Severity**: high
**Where**:
- `02-pragmatist.md` строки 12, 26, 50, 69, 88, 102, 118 — все findings нумерованы `P-1`..`P-7`
- `00-summary.md` строки 10, 19–25, 39–43, 56–61, 89–96, 107 — все cross-references на pragmatist findings используют `P-N`
**The divergence**: `commands/roast.md` строка 162 явно фиксирует: «Finding IDs stay in their Latin form (`B-1`, `H-3`, `J-2`, `C-1`, `F1.2`, `CC-3`)». То есть Pragmatist должен использовать prefix `H-N` (от «Hard reality» — операционная жёсткость). Артефакт использует `P-N` (от «Pragmatist»). Это identifier-divergence, не cosmetic: tooling, читающее severity table или cross-cutting concerns, не найдёт ожидаемый `H-N` шаблон. Задание пользователя в этом meta-review запросе фиксирует `P-N` как ожидаемый — но это в свою очередь расходится со спекой плагина, и meta-reviewer следует строке 162 командного файла как single source of truth.
**Suggested fix**: либо (а) переименовать все `P-N` → `H-N` в `02-pragmatist.md` и в cross-references `00-summary.md` (механическая правка sed-ом), либо (б) явно зафиксировать в `commands/roast.md` строка 162, что для Pragmatist допустимы оба prefix — но это правка спеки, не артефакта, и должна быть отдельным изменением плагина. Пока спека неизменна — правильный fix: переименовать.

### M-2: `03-junior-engineer.md`, `04-compliance-officer.md`, `05-futurist.md` стартуют без `# Title` header

**Category**: template conformance
**Severity**: medium
**Where**:
- `03-junior-engineer.md` строка 1 — начинается сразу с `## Junior-engineer findings`, метаданные (Target/Date/Role) — bullet-список ниже
- `04-compliance-officer.md` строка 1 — начинается с `## Compliance-officer findings`, метаданные — отдельные строки
- `05-futurist.md` строка 1 — начинается с `## Futurist findings`, метаданные — bullet-список
**The divergence**: `commands/roast.md` явной верхней `# <Title>` структуры для per-role файлов не предписывает прямо (строки 156–167 говорят про section headers «verbatim English»), но `01-devil-advocate.md` строка 1 показывает работающий паттерн: `# Devil's advocate: A2 — Link representation`. Это создаёт inconsistency между файлами одного roast-а: один-два файла имеют `# Title` header, остальные — нет. Read-first surface страдает: `cat 03-junior-engineer.md | head -1` возвращает заголовок секции, а не имя документа.
**Suggested fix**: добавить `# Junior-engineer: A2 — Link representation`, `# Compliance-officer: A2 — Link representation`, `# Futurist: A2 — Link representation` как первую строку соответствующих файлов. Существующее содержимое сдвинуть на одну H-уровень ниже не нужно — `## Summary`, `## Junior-engineer findings` остаются как есть.

### M-3: `02-pragmatist.md` не использует header `## Pragmatist findings` — секция называется иначе

**Category**: template conformance
**Severity**: medium
**Where**: `02-pragmatist.md` строка 10 — `## Pragmatist findings` присутствует, корректно. Однако файл также содержит секции `## What's understated in the proposal` (строка 130), `## What's missing entirely` (строка 140), `## What's actually realistic` (строка 151), которых нет в template.
**The divergence**: `commands/roast.md` per-role template не строго предписывает фиксированный набор секций для pragmatist — стр. 156 говорит о «each role's own template headers». Строго говоря, нарушения нет: дополнительные секции допустимы, базовый `## Pragmatist findings` есть. Но `01-devil-advocate.md` использует другой набор пост-findings секций (`## Strongest single attack`, `## Gaps in your own analysis`), и эта роль-специфичная вариация не унифицирована между файлами одного roast-а.
**Suggested fix**: оставить как есть — это не нарушение спеки, а разнообразие per-role форматов; зафиксировать как наблюдение, не как требование к правке. Если в будущем плагин формализует набор секций для каждой роли (текущая спека этого не делает), привести к этому набору.

### M-4: `## Severity counts` — junior/futurist строки помечены `—`, но текстовый комментарий ссылается на правильную строку спеки

**Category**: template conformance
**Severity**: low
**Where**: `00-summary.md` строки 22–25 (severity table) и строка 25 (комментарий)
**The divergence**: спека `commands/roast.md` строка 109 явно разрешает: «Junior-engineer and futurist don't always use severity categories the same way — leave their cells as ranges or omit». Артефакт использует `—` (em-dash) в каждой ячейке + добавляет комментарий-ссылку на строку 109. Это **корректно** — попадает в «omit». Но finding фиксирую как low-severity-наблюдение: если строгий downstream-парсер ожидает числовые ячейки или пустые ячейки, em-dash может его смутить. Уточнения от спеки нет.
**Suggested fix**: оставить как есть. Правка не нужна.

### M-5: `00-summary.md` cross-cutting concerns используют structure `### CC-N: <название>` — не отдельный CC-N counter в severity table

**Category**: template conformance
**Severity**: low
**Where**: `00-summary.md` строки 29, 37, 47, 55, 63, 70, 77 — cross-cutting concerns пронумерованы `CC-1`..`CC-7`. В severity table (строки 17–25) колонки cross-cutting нет.
**The divergence**: спека (строки 100–110) задаёт severity table только для per-role finding counts. Cross-cutting concerns в табличке отсутствуют by design: они не имеют независимой severity, они композитные. Артефакт это соблюдает. Identifier `CC-N` использован корректно (строка 162 спеки явно его разрешает). Это **не divergence**, а подтверждение конформности.
**Suggested fix**: правка не нужна. Перевести в раздел «What conforms».

### M-6: `01-devil-advocate.md` — `B-6` имеет severity «medium-high», `02-pragmatist.md` — `P-4` имеет «high, если ... ; medium, если...», но summary table показывает их как фиксированные категории

**Category**: cross-reference integrity
**Severity**: low
**Where**:
- `01-devil-advocate.md` строка 70 — B-6 severity «medium-high»
- `02-pragmatist.md` строка 84 — P-4 severity «high, если импорт vault входит в реалистичный сценарий первого года; medium, если ... »
- `00-summary.md` строка 19 — Devil-advocate `2 high (B-1, B-2)` — не учитывает B-6
- `00-summary.md` строка 20 — Pragmatist `1 conditional (P-4)` — учитывает условность корректно, но комментарием в скобках, не отдельной колонкой
**The divergence**: спека не предписывает работу с conditional severity. Summary author разрешил это разными способами для двух ролей: Pragmatist получил пометку `conditional`, Devil-advocate (B-6 medium-high) — не получил, B-6 поставлен в medium bucket в строке 19. Это inconsistency внутри summary, не нарушение спеки.
**Suggested fix**: для последовательности — либо добавить B-6 как `1 conditional` к Devil-advocate row, либо снять `conditional` пометку с P-4. Не критично.

### M-7: Cross-reference на «J-7» в `00-summary.md` строка 58 указывает на CC-4

**Category**: cross-reference integrity
**Severity**: low
**Where**:
- `00-summary.md` строка 58 — `**J-7**: cross-reference на «ADR-0002 § Methodology engine architecture» — секции с этим именем нет`
- `03-junior-engineer.md` строки 64–70 — J-7: «`recommended-luhmann` preset — где определён». Текст финдинга: «Я открыл ADR-0002 — секции `Methodology engine architecture` там нет».
**The divergence**: cross-reference корректна по сути (J-7 действительно про отсутствующую секцию ADR-0002), но формулировка в summary («cross-reference на «ADR-0002 § Methodology engine architecture» — секции с этим именем нет») оборачивается meta-cross-reference: summary говорит, что сама ссылка broken, и это правда — но это уже не finding J-7, это retransmission. Не technical error, но потенциальный source of confusion. Verified: в `decisions/0002-note-id-scheme-and-filename-layout.md` секции `Methodology engine architecture` действительно нет (можно убедиться `grep -n 'Methodology engine architecture' decisions/0002-*.md`).
**Suggested fix**: правка не нужна — finding передан корректно.

## What conforms

Артефакты конформны по большинству критериев. Конкретно:

1. **Полный набор файлов**: 6 файлов (1 summary + 5 ролей) согласно `commands/roast.md` строки 68–76. Никаких пропусков.
2. **`00-summary.md` структура**: все пять прескриптивных секций присутствуют **verbatim English**:
   - `## Headline findings` (строка 7) — каждая роль одной строкой ✓
   - `## Severity counts` (строка 15) — табличка с пятью ролями ✓
   - `## Cross-cutting concerns` (строка 27) ✓
   - `## Recommended path` (строка 83) — выбран один из трёх вариантов («Apply findings, then proceed to Document») ✓
   - `## Per-role outputs` (строка 104) — links to all five files ✓
3. **Header metadata в `00-summary.md`**: `**Target**`, `**Date**`, `**Roles run**` — все три присутствуют (строки 3–5) и в правильном формате.
4. **Per-role finding sections**: каждый файл содержит `## <Role>-findings` heading verbatim:
   - `## Devil-advocate findings` (`01-devil-advocate.md` строка 10) ✓
   - `## Pragmatist findings` (`02-pragmatist.md` строка 10) ✓
   - `## Junior-engineer findings` (`03-junior-engineer.md` строка 1) — обозначение «Clarity findings» строка 12 — это секция-вариант, не замена; основной заголовок есть ✓
   - `## Compliance-officer findings` — отсутствует напрямую; в `04-compliance-officer.md` секция называется `### Findings` (строка 17). **Это marginal divergence**, см. ниже M-8.
   - `## Futurist findings` — частично, в `05-futurist.md` использованы `## Structural findings` (строка 13) и `## Trend findings` (строка 74). **Marginal divergence**, см. M-9.
5. **Finding-ID prefixes (за вычетом M-1)**:
   - Devil-advocate `B-1`..`B-9` ✓ (соответствует строке 162)
   - Junior-engineer `J-1`..`J-11` ✓
   - Compliance-officer `C-1`..`C-6` ✓
   - Futurist `F-1`..`F-9` ✓
   - Cross-cutting `CC-1`..`CC-7` ✓
6. **Identifiers preserved** повсеместно:
   - ADR-IDs: `ADR-0001`, `ADR-0002`, `ADR-0003` ✓
   - decision-map IDs: `A1`, `A2`, `A3`, `A4`, `A5`, `B1`, `C2a`, `C3`, `C4`, `C5`, `D4` — все встречаются в latin form, не локализованы ✓
   - Crate names: `pulldown-cmark`, `gray_matter` ✓
   - API names: `Tag::Link`, `Tag::Image`, `LinkType::WikiLink`, `Options::ENABLE_WIKILINKS`, `has_pothole`, `dest_url` ✓
   - Syntax forms: `[[ID]]`, `[[ID|display]]`, `![[X]]`, `[[X#H]]`, `[[X#^id]]` ✓
   - Lint-rule IDs: `zetto/no-broken-link`, `zetto/external-url-as-wikilink`, `zetto/non-ulid-wikilink-target`, `zetto/embed-not-supported-in-v1`, `zetto/anchor-not-supported-in-v1`, `zetto/block-ref-not-supported-in-v1` ✓
   - Tool/regulation names: `vim`, `tmux`, `git`, `Obsidian`, `Crockford`, `GDPR`, `152-FZ`, `CCPA`, `OWASP ASVS` — все без перевода ✓
7. **Russian-language discipline**: prose в русском с английскими structural identifiers — правильное поведение по `architect/SKILL.md` taxonomy A–J. Ни в одном файле section header не локализован (нет `## Главные находки` / `## Перекрёстные проблемы` / etc.).
8. **Severity-counts honest**: не all-high. Compliance-officer честно ставит 0 high (в personal-use threat model действительно не должно быть high). Junior/futurist используют разрешённый omit per строка 109 спеки.
9. **Recommended path**: выбран один из трёх (`Apply findings, then proceed to Document`) и обоснован конкретным списком правок.
10. **Cross-references resolve**: `STRATEGY.md`, `ARCHITECTURE.md`, `decision-map.md`, ADR-0001, ADR-0002 — все ссылочные файлы существуют. Per-role files cross-link корректно через `./0N-rolename.md`.
11. **Terminology pass evidence**: каждый из четырёх per-role файлов (`01`, `02`, `04`) явно фиксирует terminology pass note в конце. `03-junior-engineer.md` и `05-futurist.md` — без явной footer-ноты, но prose в обоих чистая (см. M-10 ниже).

### M-8: `04-compliance-officer.md` использует `### Findings` вместо `## Compliance-officer findings`

**Category**: template conformance
**Severity**: low
**Where**: `04-compliance-officer.md` строка 17 — `### Findings`. Должно быть `## Compliance-officer findings` для соответствия pattern остальных ролей и заданию meta-review запроса.
**The divergence**: финдинги пронумерованы корректно (`#### C-1`..`#### C-6`), но без role-name-prefix в section header. Downstream tooling, ищущий regex `## (Devil-advocate|Pragmatist|Junior-engineer|Compliance-officer|Futurist) findings`, не найдёт этого файла.
**Suggested fix**: переименовать `### Findings` → `## Compliance-officer findings`. Уровень H3 → H2 — приведёт к консистентности с другими ролями (где findings — H2). Сабхедеры `#### C-N` останутся H4, что логично под H2.

### M-9: `05-futurist.md` разбивает findings на `## Structural findings` и `## Trend findings`, но единого `## Futurist findings` header нет

**Category**: template conformance
**Severity**: low
**Where**: `05-futurist.md` строка 13 (`## Structural findings (high-confidence)`), строка 74 (`## Trend findings (speculative)`). Заголовка `## Futurist findings` нет.
**The divergence**: разделение на structural vs trend семантически осмысленно (commands/roast.md table в строке 43 явно описывает futurist lens как «Structural drift ... and trend speculation»). Но если downstream tooling ищет verbatim `## Futurist findings`, оно не найдёт. Это marginal divergence — формально findings есть, формально категории даже более информативны, но нарушает pattern предсказуемого header-name.
**Suggested fix**: либо (а) добавить `## Futurist findings` как H2 wrapper над двумя существующими H2 (но тогда внутренние нужно понизить до H3), либо (б) переименовать первую существующую секцию в `## Futurist findings — Structural (high-confidence)` (сохранение seek-by-prefix паттерна). Вариант (а) предпочтительнее.

### M-10: `03-junior-engineer.md` и `05-futurist.md` — terminology pass note отсутствует

**Category**: language pass evidence
**Severity**: low
**Where**:
- `03-junior-engineer.md` строки 113–116 — заканчивается на «Что именно из «design» решения здесь применено», без terminology pass note
- `05-futurist.md` строки 126–133 — заканчивается на `## Sources` блоком, без terminology pass note
**The divergence**: `architect/SKILL.md` фиксирует, что terminology pass должен оставлять видимый след в каждом артефакте. `01-devil-advocate.md` строка 114, `02-pragmatist.md` строка 167, `04-compliance-officer.md` строка 102 — все три имеют явную footer-строку «Terminology pass: ...». Два файла её не имеют.
**Suggested fix**: добавить одну строку в конце каждого из двух файлов: «Terminology pass: ...; identifiers preserved (J-N IDs, ADR-NNNN, decision-map IDs, ...)» или «Terminology pass: ...; identifiers preserved (F-N IDs, ADR-NNNN, pulldown-cmark version refs, CommonMark)». Это не блокирующее — но meta-reviewer-у важно видеть, что pass run, чтобы отличить «pass run, нечего отметить» от «pass not run».

## Pattern observation

Per-role файлы в этом roast-е варьируются по «полноте» формата от файла к файлу:
- `01-devil-advocate.md` — полный, с `# Title`, `## Summary`, `## Devil-advocate findings`, `## Strongest single attack`, `## Gaps in your own analysis`, terminology footer.
- `02-pragmatist.md` — полный, с `# Title`, `## Summary`, `## Pragmatist findings`, `## What's understated`, `## What's missing`, `## What's actually realistic`, terminology footer.
- `03-junior-engineer.md` — без `# Title`, без terminology footer.
- `04-compliance-officer.md` — без `# Title`, без `## Compliance-officer findings` (есть `### Findings`).
- `05-futurist.md` — без `# Title`, без `## Futurist findings` (есть split structural/trend), без terminology footer.

Pattern: **структура файла деградирует от первого к последнему**, что характерно для batched generation, где первые файлы получают полный template, последние — сокращённый. Это не блокирующее — все findings присутствуют и читаемы — но имеет значение как сигнал для будущих roast-ов: либо template prescription per-role нужно усилить в `commands/roast.md`, либо при генерации проводить unification-pass.

## Areas not covered by this review

Meta-review **не оценивает**:
- архитектурную корректность findings (это работа `roast` ролей и `architect`)
- code-bug-и в проектируемом резолвере (это работа `reviewer` после реализации)
- regulatory-content compliance findings (`compliance-officer` уже это делает; meta-reviewer проверяет только формат/identifier preservation, не содержание)
- futurist trend predictions accuracy (это работа времени, не review-агента)
- ADR-quality для будущего ADR-0003 (это будет работа `meta-reviewer` уже при готовом ADR)

Если architect хочет proceed-нуть к Document phase согласно recommended path в `00-summary.md`, рекомендация meta-review: сначала исправить **M-1** (P-N → H-N) как identifier-divergence, и опционально **M-2/M-8/M-9** как cosmetic/template fixes. Остальные findings — low/observation, не блокируют.

---

Terminology pass: применён к prose (calque-замены: «расхождение» вместо «дайвергенс», «соответствие» вместо «конформанс» где уместно; «footer» / «header» / «prefix» оставлены как technical identifiers). Identifiers preserved: M-N (meta-reviewer finding IDs), B-N / P-N / J-N / C-N / F-N / CC-N (per-role IDs), ADR-NNNN, A1/A2/A3/A4/A5/B1/C2a/C3/C4/C5/D4 (decision-map IDs), `commands/roast.md`, `architect/SKILL.md`, agent role names verbatim English.
