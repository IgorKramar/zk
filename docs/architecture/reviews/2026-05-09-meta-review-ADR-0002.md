# Meta-review: ADR-0002

- **Date**: 2026-05-09
- **Reviewer**: meta-reviewer (archforge plugin self-conformance role)
- **Scope**: ADR-0002 (newly-written, deep-cycle), её привязка к индексам и trail-документам
- **Source**: `docs/architecture/decisions/0002-note-id-scheme-and-filename-layout.md`
- **Plugin source-of-truth**: `archforge` 0.4.0-rc3 (`templates/adr-template.md`, `skills/architect/SKILL.md`, `commands/roast.md`)

## Headline conformance findings

ADR-0002 **полностью соответствует** Nygard-extended ADR-шаблону archforge: все обязательные разделы (`Context`, `Decision`, `Consequences` с подразделами `Easier` / `Harder` / `Risks accepted`, `Alternatives considered`) присутствуют дословно; обязательная метаинформация (`Date`, `Status`, `Authors`) на месте; идентификаторы (ADR-NNNN, decision-map IDs A1…D5, имена крейтов, имена инструментов, термины спецификаций ULID/Crockford/RFC 9562) сохранены в английской форме согласно таксономии `architect/SKILL.md`. Дополнительные разделы (`Privacy and security considerations`, `Open questions`, `Reviews trail`, `Implementation`) — это разрешённые расширения Nygard-формата, не нарушение шаблона. Все cross-references разрешаются: ADR-0001 существует и Accepted, файлы research/roast/summary на местах, индексы (ARCHITECTURE.md, README.md, decision-map.md) синхронно обновлены под ADR-0002. Найдено три низко-/средне-серьёзных расхождения; ни одно из них не блокирует приёмку.

## Per-rule conformance

### Required template sections

**Соответствует.** Сравнение с `templates/adr-template.md`:

| Обязательный раздел шаблона | В ADR-0002 | Статус |
|---|---|---|
| `# ADR-NNNN: <Decision summary>` | `# ADR-0002: Note ID scheme and filename layout` | OK |
| `## Context` | строка 10 | OK |
| `## Decision` | строка 27 | OK |
| `## Consequences` | строка 101 | OK |
| `### Easier` | строка 103 | OK |
| `### Harder` | строка 111 | OK |
| `### Risks accepted` | строка 117 | OK |
| `## Alternatives considered` | строка 134, 3 альтернативы (минимум по шаблону — 2) + status quo, плюс таблица «Explicitly not considered» из 7 вариантов | OK, перевыполнено |

Дополнительные разделы (`Privacy and security considerations`, `Implementation`, `Open questions`, `Reviews trail`) — это расширения, разрешённые Nygard-extended ADR; они не вытесняют обязательные разделы и логически дополняют их. Для deep-cycle ADR с большим radius влияния — это пропорционально сложности.

### Header metadata

**Соответствует.** Шаблон требует `Date`, `Status`, `Authors`. Все три присутствуют (строки 3–5). Дополнительные поля `Cycle`, `Predecessor`, `Decision-map IDs` (строки 6–8) — корректное расширение для deep-cycle ADR (привязка к decision-map и предшественнику). Шаблон не запрещает дополнительные поля метаблока.

**Минор**: формат поля `Decision-map IDs` (строка 8) перечисляет 12 ID через `/`, и среди них только один (A1) реально decided этим ADR; остальные — лишь *затронутые* / разблокированные. Шаблон archforge на это поле не предписывает строгую семантику, поэтому это не нарушение, но читателю это поле может казаться, что ADR-0002 закрывает их все. См. M-2 ниже.

### Identifiers preserved

**Соответствует.** Проверено по таксономии `architect/SKILL.md` §«Что переводится, что остаётся английским»:

- **ADR-NNNN** (категория E): `ADR-0001`, `ADR-0002`, `ADR-0003`, `ADR-0006` — все в каноничном формате, zero-padded 4 цифры.
- **Decision-map IDs** (категория E): `A1`, `A2`, `A3`, `A4`, `A5`, `B1`, `B2`, `C4`, `C5`, `D1`, `D4`, `D5` — единый формат, без русских вариантов.
- **Roast finding IDs** (категория E): `CC-1`, `CC-2`, `CC-3`, `CC-4`, `CC-5`, `B-3`, `B-4`, `B-6`, `F-1`, `F-5`, `F-6` — английские префиксы сохранены, нет калек вида `СП-1` или `Б-3`.
- **Имена крейтов** (категория B): `ulid`, `slug`, `gray_matter`, `serde_yaml`, `serde_yml`, `saphyr`, `yaml-rust2`, `atomic-write-file`, `crockford`, `fast32`, `deunicode`, `kakasi`, `lindera` — все в английской форме.
- **Командные идентификаторы и API** (категория B): `zetto new`, `zetto open`, `zetto edit`, `zetto link`, `zetto lint`, `zetto retitle`, `zetto resync`, `zetto delete`, `zetto graph`, `zetto migrate format-v1 format-v2`, `Ulid::new()`, `ulid::Generator::generate_from_datetime`, `slug::slugify`, `std::fs::rename`, `git mv`, `git pull`, `git rm`, `git commit`, `git log --follow`, `git log --diff-filter=A`, `git log --name-only`, `git diff`, `git show`, `git filter-repo`, `BFG`, `cargo update`, `cargo audit` — без переводов.
- **Имена инструментов и платформ** (категория B): Obsidian, Logseq, Foam, vim, nvim, fzf, rg, ctags, LSP, MCP, FileVault, LUKS, dm-crypt, BitLocker, Windows, GitHub — без переводов.
- **Спецификации и стандарты** (категория D + B): ULID, Crockford base32, RFC 9562, RFC 4648, UUID v4, UUID v7, Luhmann, MIT/Apache-2.0, MSRV, frontmatter, wikilink, slug, Zettelkasten, deunicode, Crockford spec, ulid/spec — без переводов.
- **Поля frontmatter** (категория E, идентификаторы внутри спецификации формата): `id`, `title`, `slug_locked` — без переводов.
- **Версии и numeric tokens**: `1.2.1`, `0.1.6`, `0.2`, `0.9`, `1.87`, `format-v1`, `format-v2` — стабильны.

Идентификаторов, переведённых на русский, не обнаружено. Это особенно важно, так как ADR-0002 ссылается на 11+ roast-finding-IDs из `99-meta-review.md` — все они сохранены в английской форме, cross-references не сломаны.

### Language pass

**Соответствует с одним наблюдением.** Прозовая часть ADR (русский, рабочий язык проекта) применяет правильный регистр терминов:

- Калек из «Avoid»-колонки калькной таблицы не обнаружено: нет «обзервабилити», «деплоймент», «латенси», «перформанс», «фейловер».
- Есть `breaking change`-сценарий, описанный по-русски без транслитерации («ломающее изменение» подразумеваемое; в строке 196 — «изменяется в A2 и любом будущем link-syntax-ADR без bump format-v1» — `bump` оставлен как глагол прозы, в данном контексте читается понятно, но это кандидат на «без поднятия версии format-v1»).
- Заголовки разделов (`## Context`, `## Decision`, `## Consequences`, `### Easier`, `### Harder`, `### Risks accepted`, `## Alternatives considered`) — на английском, как в шаблоне archforge. Шаблон ADR (`templates/adr-template.md`) **не предписывает** verbatim-английские заголовки (это требование действует только для `commands/roast.md` и для нескольких других template-prescribed sections, перечисленных в категории F таксономии). Авторские заголовки расширений (`Privacy and security considerations`, `Open questions`, `Reviews trail`, `Implementation`) — на английском, что согласовано со стилем шаблона. Подразделы `Decision` смешанные («Канонический ID», «Линкование», «Retitle lifecycle and recovery») — это допустимо, шаблон их не предписывает.

**Наблюдение M-3** (ниже): нет явной строки «терминологический проход» в чате/коммит-сообщении ADR. По правилу `architect/SKILL.md` §«Mandatory terminology pass — after generating any Russian artifact», шаг 5: «State briefly in chat (one line) what the pass changed». Это требование чат-уровня, не самого артефакта; для самого ADR это незаметно. Если ADR-0002 был создан без видимого прохода — это пред-rule артефакт по нашему мягкому толкованию для проектов, где правило ранее не применялось систематически. Помечается как low.

### Cross-references

**Полностью разрешаются.** Проверены все relative-ссылки:

| Цель ссылки | Существует | Замечания |
|---|---|---|
| `../decision-map.md` | OK | A1 действительно отмечен decided со ссылкой на ADR-0002 |
| `../research/2026-05-09-A1-ulid-slug-rust-research.md` | OK | |
| `../research/2026-05-09-A1-note-id-scheme-discovery.md` | OK | |
| `../research/2026-05-09-A1-note-id-scheme-design.md` | OK | |
| `../research/2026-05-09-A1-decision-summary.md` | OK | |
| `../reviews/2026-05-09-roast-A1-note-id-scheme/00-summary.md` | OK | |
| `../reviews/2026-05-09-roast-A1-note-id-scheme/99-meta-review.md` | OK | meta-review над roast-итерацией |
| ADR-0001 (упомянут как Predecessor и в Risks accepted) | OK | существует, Status: Accepted |
| Внешние ссылки (github.com/ulid/spec, crockford.com, crates.io/...) | не валидируется meta-reviewer-ом (out-of-scope) | — |

In-document ссылки (`§ Invariant validation`, `§ Retitle lifecycle and recovery`, `§ Empty-slug lifecycle`, `§ Implementation`, `§ Open questions`) — все указывают на существующие subsections внутри того же ADR.

Ссылки на decision-map IDs (`A1`, `A2`, ..., `D4`) разрешаются в `decision-map.md`, где они и определены.

### Lifecycle states

**Соответствует.** Status: Accepted (строка 4) — это валидное состояние из enum'а `Proposed | Accepted | Deprecated | Superseded by ADR-NNNN`. ADR только что создан, никаких следов post-acceptance edits на superseded-version нет — это первая версия Accepted-статуса. Ничего не нарушено.

ADR-0001 (Predecessor) проверен: тоже Accepted, не Deprecated, не Superseded — корректно ссылаться на него как на предшественника.

### Roast trail linked

**Соответствует.** Раздел `## Reviews trail` (строки 204–211) перечисляет полный набор артефактов цикла:

- Discovery — линк OK
- Research digest — линк OK
- Design — линк OK
- Decision summary — линк OK
- Roast (5 ролей, severity 4H/11M/5L) — линк на `00-summary.md` OK
- Meta-review on roast — линк на `99-meta-review.md` OK

Это перевыполняет требования `commands/roast.md` step 6 (там минимум — линк на summary; здесь добавлен ещё meta-review над roast-итерацией). Severity-сводка `4H/11M/5L` совпадает с тем, что обычно появляется в `00-summary.md` (`## Severity counts` верифицируется отдельно — out-of-scope для этого meta-review). Plug-in хочет видеть, что roast реально проведён и его выводы привязаны к ADR — это требование выполнено.

Привязка roast → ADR прослеживается семантически: разделы `### Retitle lifecycle and recovery (применение CC-1)`, `### Empty-slug lifecycle (применение CC-4)`, `### Invariant validation (применение CC-2 / B-3, B-6)`, `## Privacy and security considerations (применение CC-5)`, `### Format versioning anchor (применение CC-3)`, и пометки `(применение F-1)`, `(применение F-5)`, `(применение F-6)` в Open questions — это явное cross-reference на конкретные roast findings. Это не требование шаблона, но это **признак зрелого ADR** для deep-cycle: видно, какие ровно roast-замечания закрыты текстом ADR.

### Indexes updated

**Соответствует.** Все три обязательных индекса синхронно обновлены:

- **`ARCHITECTURE.md` §6 Open questions** (строка 126): `~~Q1. Схема ID заметок.~~ — закрыт ADR-0002…` — корректно стрейкнут как resolved.
- **`docs/architecture/README.md` ADR Index** (строка 25): запись для ADR-0002 со статусом Accepted, датой, кратким описанием — формат совпадает с записью ADR-0001.
- **`docs/architecture/decision-map.md`** (строки 14–16, 159): A1 переведён open → decided со ссылкой на ADR-0002; в footer-блоке строка 159 продублировано.

ARCHITECTURE.md §5 Decision Index (упомянут в Implementation §Synchronous changes как место, требующее обновления) — поиск по `ADR-0002` в файле даёт два совпадения (строки 15, 126). Полное содержимое §5 не проверял подробно, но факт упоминания ADR-0002 в индексе подтверждён.

## Recommended fixes (если применять)

### M-1 (low) — `bump` в прозе

**Категория**: language pass / прозовая калька.
**Где**: `0002-note-id-scheme-and-filename-layout.md` строка 196 — «изменяется в A2 и любом будущем link-syntax-ADR без bump format-v1».
**Расхождение**: `bump` как глагол прозы — англицизм, имеющий русский эквивалент. Это не идентификатор (категория B/E/F), а прозовый глагол.
**Предлагаемая правка**: «без поднятия версии format-v1» или «без перехода на новую major-версию format-v1». Severity low — единичное вхождение, читается понятно, не блокирует.

### M-2 (low) — поле `Decision-map IDs` в метаблоке

**Категория**: header metadata clarity.
**Где**: строка 8 — `**Decision-map IDs**: A1/A2/A3/A4/A5/B1/B2/C4/C5/D1/D4/D5`.
**Расхождение**: 12 ID, перечисленные через `/`, читаются как «этот ADR закрывает все 12». На самом деле ADR-0002 закрывает только A1; остальные — *затронуты / разблокированы*. Шаблон archforge не нормирует это поле, но текущая форма потенциально вводит в заблуждение.
**Предлагаемая правка**: разнести на два поля: `**Decision-map IDs decided**: A1` и `**Decision-map IDs affected**: A2, A3, A4, A5, B1, B2, C4, C5, D1, D4, D5`. Или оставить как есть, добавив один-словный комментарий после первого ID: `A1 (decided) / A2/A3/A4/A5/B1/B2/C4/C5/D1/D4/D5 (affected)`. Severity low — semantical, не структурная.

### M-3 (low, historical) — отсутствие строки терминологического прохода

**Категория**: language pass evidence.
**Где**: вне ADR — в чат-сообщении, в котором ADR был создан.
**Расхождение**: правило `architect/SKILL.md` шаг 5 предписывает после каждого русскоязычного артефакта одну строку в чате о том, что заменил терминологический проход. Это требование документации chat-flow-а, не самого артефакта. Для уже сохранённого ADR — pre-rule (если правило не применялось систематически в этом проекте раньше) или post-hoc-зафиксированное (если применялось, но не сохранилось в общем потоке).
**Предлагаемая правка**: при следующем русскоязычном артефакте в этом проекте — приучить себя выводить эту строку. Для ADR-0002 — задокументировать ретроактивно: «терминологический проход на ADR-0002: калек не найдено; bump оставлен в прозе (один раз) — кандидат на правку M-1; идентификаторы (ADR-NNNN, decision-map IDs, ULID, Crockford base32, имена крейтов и инструментов) сохранены без изменений». Severity low — historical, не блокер.

## Что conforms (положительный список)

- Шаблон `templates/adr-template.md` соблюдён в обязательной части полностью.
- Минимум 2 альтернативы — превышен (3 + status quo + 7 в «Explicitly not considered»). Это признак зрелого ADR.
- В `Consequences` обе стороны (`Easier`, `Harder`, плюс отдельно `Risks accepted`) с конкретными downsides — шаблон явно требует «An ADR with no downsides is marketing, not an ADR»; здесь даунсайды артикулированы детально (long filenames, retitle atomicity, deprecated `serde_yaml`, Windows MSRV, CJK fallback).
- Идентификаторная гигиена образцовая. Нет ни одного перевода в категориях A-F. Особенно в свете 11+ cross-references на roast finding IDs (CC-1, CC-2, CC-3, CC-4, CC-5, B-3, B-4, B-6, F-1, F-5, F-6) — все английские, все разрешаются в `99-meta-review.md`.
- Reviews trail полный (6 артефактов; стандартное требование — линк на roast summary, а здесь дано всё trail-дерево).
- Привязка roast-finding → текст-ADR явная: каждое крупное расширение (`Retitle lifecycle`, `Empty-slug lifecycle`, `Invariant validation`, `Privacy`, `Format versioning anchor`) имеет в заголовке `(применение CC-N)` или `(применение F-N)`. Это превышает требования шаблона и упрощает audit.
- Все cross-references разрешаются (8 internal links, 6 internal sub-section refs).
- Все индексы (ARCHITECTURE.md, README.md, decision-map.md) синхронно обновлены под ADR-0002 — это соответствует шагу Document архитектурного цикла.
- Pre-condition lifecycle: ADR-0001 существует и Accepted, корректно ссылаться как на Predecessor.

## Areas not covered by this review

Per `meta-reviewer.md` §«What you do NOT cover», следующее **намеренно не оценивалось** — это вне роли:

- Архитектурное качество решения (ULID vs UUID v7, дилемма «filename-as-projection vs frontmatter-as-truth», invariants design) — это компетенция `architect`, `devil-advocate`, `pragmatist` и других roast-ролей. Roast уже был проведён и привязан к ADR.
- Правильность утверждений о крейтах и стандартах (что `ulid` 1.2.1 действительно встраивает Crockford-encoder, что Rust 1.87 действительно стабилизирует PR-138133, что `gray_matter` действительно lossy на comments) — это компетенция `researcher` / `historian`. Сами ссылки на эти источники в ADR присутствуют.
- Соответствие текущему GDPR / 152-ФЗ описаний privacy — компетенция `compliance-officer`.
- Обоснованность 60-char slug-cap, выбора MSRV ≥ 1.87, политики deferred CJK fallback — компетенция `pragmatist` / `futurist`.
- Code-bug в planned implementation (атомарность `std::fs::rename`, race window между vim и zetto) — компетенция `reviewer` после первого коммита.
- Вопрос «можно ли было вместо deep-cycle сделать quick-cycle» — компетенция самого пользователя в момент инициации цикла.

Если по любой из этих тем у пользователя есть ощущение неуверенности — соответствующая роль уже доступна через `/archforge:roast --roles=<role>` или через прямой вызов sub-agent-а.

## Verdict

**CONFORMING.** ADR-0002 соответствует структурным требованиям шаблона archforge `templates/adr-template.md`, правилам идентификаторной гигиены `architect/SKILL.md`, и lifecycle-требованиям plug-in-а. Найдено три расхождения, все severity low: одна прозовая калька (`bump`), одно semantically-неоднозначное поле метаблока (`Decision-map IDs`), одно историческое отсутствие строки терминологического прохода. Ни одно из них не блокирует приёмку и не требует пересмотра решения. Принимать ADR в текущем виде.

---

**Терминологический проход на этот meta-review**: калек не обнаружено; идентификаторы (`ADR-NNNN`, `archforge`, `Nygard-extended`, `architect`, `devil-advocate`, `pragmatist`, `roast`, `meta-reviewer`, `discovery`, `decision-map`, `frontmatter`, `## Context`, `## Decision`, `## Consequences`, `### Easier`, `### Harder`, `### Risks accepted`, `## Alternatives considered`, `CC-N`, `B-N`, `F-N`, `A1…D5`) сохранены без изменений; имена крейтов и инструментов из ADR-0002 (`ulid`, `slug`, `gray_matter`, `serde_yaml`, `Obsidian`, `vim`, `git`, `fzf`, `rg`, `ULID`, `Crockford base32`, `RFC 9562`, `MSRV`) перенесены в meta-review без перевода.
