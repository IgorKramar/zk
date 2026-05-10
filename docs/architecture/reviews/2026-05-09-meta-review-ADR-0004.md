# Meta-review: ADR-0004 (Frontmatter convention)

**Target**: `/Users/user/Work/self/zk/docs/architecture/decisions/0004-frontmatter-convention.md`
**Date**: 2026-05-10
**Plugin version**: archforge 0.4.0-rc3
**Specification source**: `templates/adr-template.md`, `commands/adr.md`, `commands/roast.md` step 6, `skills/architect/SKILL.md` § Language and terminology.

## Summary

ADR-0004 структурно соответствует Nygard-шаблону `templates/adr-template.md` плюс набор Nygard-extended секций (`## Privacy and security considerations`, `## Open questions`, `## Reviews trail`), консистентных с прецедентом ADR-0002/0003 в этом же проекте. Идентификаторы (ADR-NNNN, decision-map IDs A3/A4/A5/B1/C2a/D4, имена крейтов, syntax-формы полей frontmatter, lint-rule names) сохранены без перевода. Cross-references резолвятся, индексы во всех четырёх местах (ARCHITECTURE.md §5, decision-map A3, `docs/architecture/README.md`, `docs/architecture/decisions/README.md`) обновлены. Расхождений уровня high не обнаружено; перечисленные ниже находки — medium/low, в основном фиксация Nygard-extended элементов и мелкие неконсистентности с шаблоном.

## Findings

### M-1: Шаблон не предписывает секцию `## Privacy and security considerations`
**Category**: template conformance
**Severity**: low
**Where**: ADR-0004 строки 271–282 (`## Privacy and security considerations`).
**The divergence**: `templates/adr-template.md` (строки 1–69) перечисляет только секции Context, Decision, Consequences (с подсекциями Easier/Harder/Risks accepted) и Alternatives considered. Секция `## Privacy and security considerations` не предписана. Это Nygard-extended секция, добавленная как проектная конвенция (присутствует и в ADR-0002 §Privacy, и в ADR-0003 §Privacy — то есть прецедент в проекте есть).
**Suggested fix**: ничего не править в этом ADR. Для in-project консистентности — задокументировать в `docs/architecture/decisions/README.md`, что проект zetto использует Nygard-шаблон с тремя дополнительными секциями (Privacy, Open questions, Reviews trail).

### M-2: Шаблон не предписывает `## Open questions` и `## Reviews trail`
**Category**: template conformance
**Severity**: low
**Where**: ADR-0004 строки 284–294 (`## Open questions`), 296–303 (`## Reviews trail`).
**The divergence**: те же замечания, что в M-1. Ни Open questions, ни Reviews trail в `templates/adr-template.md` не предписаны. `commands/roast.md` шаг 6 говорит «If the artifact has a `## Reviews` section (or you want to add one), append a line» — то есть имя секции командой не зафиксировано (Reviews / Reviews trail — оба допустимы), но добавление само по себе разрешено и трактуется как opt-in расширение. Open questions — целиком проектная конвенция.
**Suggested fix**: то же, что в M-1 — задокументировать проектное расширение Nygard-шаблона на уровне README в `decisions/`. ADR-0004 трогать не требуется.

### M-3: Имя секции `Reviews trail` vs предписанный командой вариант `Reviews`
**Category**: template conformance
**Severity**: low
**Where**: строка 296 (`## Reviews trail`).
**The divergence**: `commands/roast.md` шаг 6 (строка 132) предлагает имя секции `## Reviews`. ADR-0004 использует `## Reviews trail`. Команда формулирует имя как пример («or you want to add one»), не как жёсткое требование, поэтому это не нарушение, но точечная неконсистентность с языком команды и с inter-project узнаваемостью (другой `meta-reviewer`-агент будет искать секцию `## Reviews` для извлечения trail-записей).
**Suggested fix**: оставить «Reviews trail» — узнаваемее в проекте; либо переименовать в «Reviews» для соответствия языку команды. Решение — на уровне zetto-конвенции, не блокер.

### M-4: Подсекция `### Explicitly not considered` внутри `## Alternatives considered` — не часть шаблона
**Category**: template conformance
**Severity**: low
**Where**: строки 258–269 (`### Explicitly not considered`).
**The divergence**: `templates/adr-template.md` предписывает только H3-нумерованные альтернативы (`### 1.`, `### 2.`, `### 3. Status quo`). Подсекция «explicitly not considered» в виде таблицы — проектное расширение, чтобы зафиксировать варианты, которые не дошли до полноценной альтернативы. Прецедент в ADR-0002 (есть такая же подсекция). Пользовательский запрос явно говорит «`## Explicitly not considered` table — Nygard-extended OK», то есть это известное проектное расширение.
**Suggested fix**: оставить как есть — паттерн консистентен внутри проекта. Альтернатива: добавить заметку в `docs/architecture/decisions/README.md` о том, что подсекция «Explicitly not considered» — конвенция проекта.

### M-5: Поле метаданных `**Predecessors**` (мн.ч.) расходится с ADR-0002 `**Predecessor**` (ед.ч.)
**Category**: lifecycle integrity / cross-document consistency
**Severity**: low
**Where**: строка 7 (`**Predecessors**: ADR-0002 (...), ADR-0003 (...)`); сравните с ADR-0002 строка 7 (`**Predecessor**: ADR-0001`).
**The divergence**: шаблон требует только `**Date**`, `**Status**`, `**Authors**`. Поле Predecessor(s) — расширение проекта. Несогласованность ед./мн. между ADR-0002 (`Predecessor`) и ADR-0004 (`Predecessors`) — мелкая, но накапливается при росте корпуса ADR (грeppable consistency страдает: `grep -r '**Predecessor**'` пропустит ADR-0004).
**Suggested fix**: выбрать одну форму (рекомендую `**Predecessors**` мн.ч., потому что поле может содержать список) и применить ретроспективно к ADR-0002. Не обязательно прямо сейчас — можно зафиксировать выбор в README конвенции и применять в новых ADR.

### M-6: Severity-сводка в Reviews trail — нестандартный формат «5H/9M/7L»
**Category**: cross-reference integrity
**Severity**: low
**Where**: строка 302 (`Roast (5 roles, severity: 5H/9M/7L)`).
**The divergence**: `commands/roast.md` шаг 6 предлагает шаблон строки `Roast (5 roles, severity: H/M/L counts)` — то есть три отдельных счёта. Сжатый формат «5H/9M/7L» (без пробелов, slash-separator) — компактный вариант той же информации, не противоречит, но не дословный. Проверка чисел: roast-summary показывает devil-advocate 3H/3M/2L, pragmatist 2H/4M/1L, compliance-officer 0H/2M/4L → суммирование trois ролей даёт 5H/9M/7L (junior-engineer без severity, futurist использует другую категоризацию — это согласуется с note в `commands/roast.md` строка 109). Числа сходятся.
**Suggested fix**: оставить компактный формат как проектную конвенцию. Если требуется пол-формальное соответствие шаблону команды — заменить на `severity: 5 high / 9 medium / 7 low`.

### M-7: Языковой пас — отсутствует one-line note в самой странице
**Category**: language pass
**Severity**: low (исторический, по правилам meta-reviewer для pre-rule артефактов)
**Where**: ADR-0004 целиком.
**The divergence**: `skills/architect/SKILL.md` § Mandatory terminology pass (строка 237) предписывает по итогам генерации русскоязычного артефакта произнести одно-двухстрочную сводку в чате о том, что заменено и какие идентификаторы оставлены. Артефакт сам по себе такой записи не содержит (что нормально — note живёт в чате, не в документе), и проверить из артефакта факт прохода нельзя. Само тело документа не содержит явных калек из таблицы «Avoid» (нет «обзервабилити», «деплоймент», «перформанс», «латенси», «трейсинг», «фейловер») и идентификаторы сохранены без перевода — косвенное свидетельство, что языковой пас прошёл.
**Suggested fix**: ничего в самом ADR не править. На будущее — фиксировать one-line terminology-сводку в commit-message или в чат-выводе при принятии ADR, чтобы meta-reviewer мог её увидеть постфактум.

### M-8: Терминологические заимствования прозы на грани калькирования
**Category**: language pass
**Severity**: low
**Where**: разрозненно по тексту.
**The divergence**: несколько английских слов в прозе, не входящих в таблицу калек, но имеющих естественные русские эквиваленты:
- «barrier-to-entry» (строка 197) — естественнее «порог входа».
- «out-of-the-box» (строка 207) — естественнее «из коробки» (хотя «из коробки» само калькованное; в техническом контексте обоe приемлемо).
- «anticipation» в составе «D4 anticipation» (строки 199, 230) — здесь работает как ярлык-идентификатор подхода («авансовое включение под D4»), приемлемо как term-of-art с проектным контекстом.
- «render-fallback», «coercion», «iff» — term-of-art-усы, оставлять без перевода допустимо.
Идентификаторы (ADR-NNNN, decision-map IDs, имена крейтов, lint-rule names, syntax-формы полей) сохранены везде корректно. Калек из таблицы (категория I в `architect/SKILL.md`) в тексте нет.
**Suggested fix**: точечно — заменить «barrier-to-entry» → «порог входа». Остальное — на усмотрение автора как term-of-art-баланс.

## What conforms

- **Структура шаблона**. H1 заголовок (`# ADR-0004: Frontmatter convention`), полный метаблок (Date, Status, Authors), и три обязательных раздела `## Context`, `## Decision`, `## Consequences` (с подсекциями `### Easier`, `### Harder`, `### Risks accepted`), и `## Alternatives considered` присутствуют в правильном порядке Context → Decision → Consequences → Alternatives, как предписано `templates/adr-template.md`.
- **Альтернативы**. Четыре альтернативы H3-нумерованного формата (`### 1. Lean-set canonical schema (выбрана)`, `### 2. Maximalist`, `### 3. Minimalist`, `### 4. Status quo — отложить A3 до C2a / B1`) — превышает минимум 2 из шаблона, и `Status quo` явно присутствует как №4. Каждая альтернатива имеет блоки «сильная сторона», «слабая сторона», «lost because» — это богаче, чем минимум шаблона.
- **Consequences с обеими сторонами**. `### Easier` (6 пунктов), `### Harder` (6 пунктов), `### Risks accepted` (6 пунктов) — секция честная, downsides присутствуют без сокрытия (см. `commands/adr.md` discipline: «No downsides» means the decision wasn't honest).
- **Идентификаторы сохранены**. ADR-NNNN-формат корректен (ADR-0001/0002/0003/0004 zero-padded). Decision-map IDs (A3, A4, A5, B1, B2, C2a, C5, D4) везде в латинице. Имена крейтов (`gray_matter`, `serde_yaml`, `pulldown-cmark`, `atomic-write-file`, `ulid`, `chrono`, `serde_yml`, `saphyr`) — английские. Syntax-формы (`tags:`, `aliases:`, `x-*`, `created:`, `updated:`, `id:`, `title:`, `format:`, `cssclasses:`) — verbatim. Lint-rule names (`zetto/missing-required-field`, `zetto/invalid-id-format`, `zetto/empty-title`, `zetto/non-rfc3339-timestamp`, `zetto/invalid-tags-format`, `zetto/unknown-frontmatter-field`, `zetto/tag-not-in-frontmatter`, `zetto/non-canonical-tag-format`, `zetto/empty-alias`, `zetto/duplicate-frontmatter-key`, `zetto/alias-collision-saturation`, `zetto/alias-resolver-disabled-at-scale`, `zetto/long-title`) — все 13 в namespace-форме, английские. Finding IDs из roast (B-1, B-2, B-3, B-4, H-2, H-5, H-6, F-1, F-2, F-3, F-4, F-5, F-6, F-7, CC-1, CC-2, CC-3, CC-4, CC-5, CC-6, CC-7) — преобразование «применение CC-N / B-N / H-N / F-N» в скобках сохраняет латинский формат, не русифицировано. Имена ролей в `commands/roast.md` (Devil-advocate, Pragmatist, Junior-engineer, Compliance-officer, Futurist) в самом ADR не упоминаются по имени, что нормально.
- **Cross-references резолвятся**. Линки на:
  - `../decision-map.md` → существует (`docs/architecture/decision-map.md`).
  - `../research/2026-05-09-A3-frontmatter-convention-discovery.md`, `-research.md`, `-design.md`, `2026-05-09-A3-decision-summary.md` → все 4 файла присутствуют в `docs/architecture/research/`.
  - `../reviews/2026-05-09-roast-A3-frontmatter-convention/00-summary.md` → каталог и файл существуют.
  - `99-meta-review.md` в roast-каталоге → существует.
  - ADR-0001/0002/0003 — упоминаются по имени; файлы в `decisions/` присутствуют.
- **Lifecycle**. Status: Accepted — корректен (per `commands/adr.md` and `decisions/README.md` lifecycle). Дата 2026-05-09 синхронна с date в ARCHITECTURE.md §5, decision-map A3, обоими README ADR-индексами и Reviews trail.
- **Roast-trail линк**. Reviews trail § содержит запись «Roast (5 roles, severity: 5H/9M/7L) — [00-summary.md]» — соответствует `commands/roast.md` шагу 6 (с минимальной перефразировкой формата, см. M-6).
- **Индексы обновлены во всех четырёх местах**:
  - `/Users/user/Work/self/zk/ARCHITECTURE.md` §5 Decision index — содержит ADR-0004 строку с правильным summary, датой, статусом.
  - `/Users/user/Work/self/zk/docs/architecture/decision-map.md` запись A3 — статус **decided** + ссылка на ADR-0004 + одно-строчное summary решения (строки 27–31).
  - `/Users/user/Work/self/zk/docs/architecture/README.md` ADR Index — содержит ADR-0004 строку.
  - `/Users/user/Work/self/zk/docs/architecture/decisions/README.md` Index — содержит ADR-0004 строку.
  Все четыре записи дают консистентное summary решения и одну дату. Это соответствует требованию `decisions/README.md` («ADR попадает в три места одновременно при принятии» — фактически четыре, считая decision-map).
- **Языковой пас**. Проза по-русски, идентификаторы все в исходной форме. Калек из «Avoid»-колонки таблицы калек (`architect/SKILL.md`) не встречается: нет «обзервабилити», «деплоймент», «перформанс», «латенси», «трейсинг», «фейловер», «скейлинг», «кэшинг», «провижининг», «онбординг», «фича», «баг», «хайповый», «сплит», «дашборд». Term-of-art сохранены в исходной форме, как разрешено категорией H («prompt injection», «render-fallback», «content-hash», «hand-rolled», «iff», «verbatim»).
- **Cycle metadata**. Поле `**Cycle**: A3 (decision-map.md), deep` корректно идентифицирует, что это deep-cycle ADR из A3-записи decision-map.

## Areas not covered by this review

- Архитектурное содержание решения — правильно ли выбрана `lean-set canonical schema` вместо maximalist/minimalist; устойчиво ли это решение через 1–3 года. → Это область `roast` (devil-advocate, pragmatist, futurist).
- Operational cost (стоимость поддержки 13 lint rules, hand-rolled write state machine на 30–50 unit tests). → `pragmatist`.
- Privacy / compliance детали (frontmatter как leak surface при share). → `compliance-officer`.
- Ясность для нового читателя (термины «render-fallback», «application of CC-N» без раскрытия). → `junior-engineer`.
- Будущая drift (`updated:`-content-hash паттерн, `aliases:`-resolver как permanent contract). → `futurist`.
- Корректность Rust-сниппета `struct Frontmatter` (например, `BTreeMap<String, serde_yaml::Value>` flatten — компилируется ли). → `reviewer`.
- Lifecycle integrity post-acceptance (был ли ADR изменён после принятия) — `git log/diff` мне доступен, но рассматриваемая версия — single-state snapshot; правильность диффа не проверял.

---

**Терминологический проход**: проза мета-ревью по-русски; идентификаторы (ADR-NNNN, A3/A4/A5/B1/C2a/D4, имена крейтов, lint-rule names, syntax-формы полей frontmatter, Devil-advocate/Pragmatist/Junior-engineer/Compliance-officer/Futurist, finding-IDs B-N/H-N/J-N/C-N/F-N/CC-N, имена шаблонных секций `## Context`/`## Decision`/`## Consequences`/`## Alternatives considered`/`### Easier`/`### Harder`/`### Risks accepted`) сохранены без изменений. Калек из таблицы «Avoid» в `architect/SKILL.md` не использовал.
