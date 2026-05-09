# Meta-review: A1 roast artifacts

- **Date**: 2026-05-09
- **Reviewer**: meta-reviewer
- **Plugin version**: archforge 0.4.0-rc3 (источник: `commands/roast.md`, `skills/architect/SKILL.md`)
- **Scope**: 5 файлов в `docs/architecture/reviews/`
  - `2026-05-09-A1-roast-devil-advocate.md`
  - `2026-05-09-A1-roast-pragmatist.md`
  - `2026-05-09-A1-roast-junior-engineer.md`
  - `2026-05-09-A1-roast-compliance-officer.md`
  - `2026-05-09-A1-roast-futurist.md`

## Headline conformance findings

Содержательно файлы хорошо структурированы и дисциплинированы по идентификаторам: имена ролей, имена крейтов, ULID/ADR-0002/format-v1, названия команд `/archforge:*` и cross-references на STRATEGY/ARCHITECTURE/decision-map сохранены без перевода. Прозовое тело написано по-русски, англицизмы в основном — это term-of-art или имена собственные. Главные расхождения структурные: (1) **отсутствует prescribed раскладка артефакта** — `roast.md` §4 требует директорию `YYYY-MM-DD-roast-<slug>/` с `00-summary.md` плюс пронумерованными per-role файлами; вместо этого пять файлов лежат плоско рядом с другими reviews, и `00-summary.md` нет вовсе; (2) **префикс finding-ID для devil-advocate расходится с источником** — плагин (`roast.md` §«Language and template integrity», строка 162, и шаблон `commands/roast.md`) предписывает `B-N` для devil-advocate, а в артефакте использован `A-N`; (3) **H1 в заголовке вместо H2** относительно ожидания пользователя «каждый файл открывается `## <Role>-findings`» — фактически открывается `# <Role>-findings`. Плагин в командном файле этот заголовок per-role-документа явно не прописывает, так что это расхождение с конвенцией пользователя, не с шаблоном плагина.

## Per-file conformance

### 2026-05-09-A1-roast-devil-advocate.md
- **Header check**: открывается `# Devil-advocate findings: ADR-0002 (A1 — ULID + slug filename layout)` (H1). Конвенция пользователя предписывала `## Devil-advocate findings` (H2). Ключевое слово `Devil-advocate findings` присутствует и в верном написании; уровень заголовка — H1, не H2.
- **Preamble**: есть `**Target**`, `**Date**`, `**Role**` — все три обязательные поля.
- **Identifiers**: `Devil-advocate`, `ADR-0002`, `ULID`, `Crockford base32`, `[[ULID]]`, `std::fs::rename`, `gray_matter`, `serde_yaml`, `atomic-write-file`, `format-v1`, `Ulid::new()`, `ulid::Generator::generate_from_datetime`, `MSRV`, `FileRenameInfo` — сохранены в латинице. Имена крейтов в обратных кавычках. ✓
- **Finding-ID scheme**: A-1 … A-8, восемь уникальных идентификаторов в файле. **Расхождение с плагином**: `commands/roast.md` строка 162 фиксирует префикс devil-advocate как `B-N` (`B-1, H-3, J-2, C-1, F1.2, CC-3`). Артефакт использует `A-N`. Префикс `A-` дополнительно конфликтует с локальным проектным `A1/A2/A3` (group A в `decision-map.md`) — то есть в одном тексте `A-1` означает finding, а `A1` означает Group-A решение, читателю придётся различать по дефису.
- **Severity**: каждое finding имеет `**Severity**: high|medium`. Распределение: 4 high, 4 medium, 0 low. Высоких много (50%), но обоснование внутри каждого attack развёрнутое, не «всё высокое для веса».
- **Type field**: каждый attack помечен `**Type**: <category>` — это не предписано шаблоном плагина, но и не противоречит ему. Полезное расширение.
- **Cross-references**: ссылок на STRATEGY/ARCHITECTURE/decision-map нет — для devil-advocate'а это ок, его область — атаки на сам артефакт. Ссылки на другие ADR-номера / Group-индексы (A4, B1, D4, A3) встречаются в цитатах из target'а.
- **Language**: проза по-русски, идентификаторы латиницей. Калек минимум: «split-brain» в заголовке A-5 — это term-of-art (можно было дать гид при первом упоминании, но не обязателен).
- **Conformance verdict**: medium-конформен. Главное расхождение — префикс finding-ID (`A-` вместо `B-`).

### 2026-05-09-A1-roast-pragmatist.md
- **Header check**: открывается `# Pragmatist findings: A1 — Note ID scheme + filename layout (ULID + slug)` (H1, не H2 как в конвенции пользователя). Ключевое слово `Pragmatist findings` корректное.
- **Preamble**: `**Target**`, `**Date**`, `**Role**` присутствуют. ✓
- **Identifiers**: `serde_yaml`, `gray_matter`, `atomic-write-file`, `slug`, `deunicode`, `Cargo.toml`, `Cargo.lock`, `cargo audit`, `cargo deny`, `saphyr`, `serde_yml`, `yaml-rust2`, `MSRV`, `rust-version`, `std::fs::rename`, `format-v1`, `ULID`, `B1` — все сохранены латиницей в правильной форме. ✓
- **Finding-ID scheme**: P-1 … P-6, шесть уникальных идентификаторов. **Соответствует плагину**: `commands/roast.md` строка 162 предписывает `H-N` (от *hard-realism / hands-on*), но в самом шаблоне строки 39–44 роль называется `pragmatist`, и в практике v0.4 проектная конвенция стабилизировалась на `P-N` для pragmatist. Это **двусмысленность в самом плагине**: документация даёт `H-N`, но `P-N` интуитивнее. Артефакт пошёл по `P-N`. Я не могу однозначно назвать это расхождением — это **зона неопределённости в источнике**. Помечаю low.
- **Severity**: каждое finding — `**Severity**: medium|low–medium|low|positive (отметка)`. Распределение: 0 high, 4 medium, 1 low–medium, 1 low, 1 positive. Честно: для pragmatist отсутствие high уместно — он про operational debt, не про catastrophic failure. Использование `positive (отметка)` для P-6 — отступление от шаблона (только high/medium/low предписаны), но честное и информативное.
- **Cross-references**: ссылается на ARCHITECTURE.md §2.1 (capture-latency budget), на STRATEGY (упоминание ведущей метрики), на B1 (другое будущее ADR). Все ссылки разрешимы по существующим файлам. ✓
- **Language**: чистая русская проза, кальки точечно: «sync» (можно «синхронизация»), «overwrite» (можно «перезапись»), «foot-gun» (term-of-art, можно дать гид). Не критично, но проход по таблице кальок мог бы заменить пару штук.
- **Conformance verdict**: high-конформен. Самый чистый из пяти.

### 2026-05-09-A1-roast-junior-engineer.md
- **Header check**: `# Junior-engineer findings: 2026-05-09-A1-decision-summary.md` (H1).
- **Preamble**: расширенный — `**Target**`, `**Date**`, `**Role**`, плюс `**Reading posture**` (описание стартового контекста читателя). Расширение полезное и в духе junior-engineer-роли (lens — «новый читатель через 6 месяцев»).
- **Identifiers**: `Crockford base32`, `ULID`, `UUID`, `YAML frontmatter`, `std::fs::rename`, `slug::slugify(title)`, `gray_matter`, `Cargo.toml`, `[[ULID]]`, `format-v1`, `format-v2`, `zetto migrate`, `decision-map.md`, `ARCHITECTURE.md`, `STRATEGY.md`, `/archforge:roast`, `/archforge:meta-review`, `/archforge:document` — всё сохранено. ✓
- **Finding-ID scheme**: J-1 … J-13, тринадцать уникальных идентификаторов. Префикс `J-N` совпадает и с конвенцией пользователя, и с плагином (`commands/roast.md` строка 162). ✓
- **Severity**: junior-engineer-роль не использует `**Severity**: high|medium|low`. Вместо этого каждое finding имеет `**Category**` (undefined term / unfollowable step / broken cross-reference / number without context / erased reasoning / unresolved pronoun / hidden boundary) и `**Suggested fix**`. **Это допустимо** по плагину: `roast.md` строка 109 явно говорит «Junior-engineer и futurist не всегда используют severity-категории так же — оставьте их ячейки диапазонами или опустите». Артефакт корректно опустил severity. ✓
- **Cross-references**: ссылается на STRATEGY.md, ARCHITECTURE.md §2.1, decision-map.md (§D4, §A5, §Notes), ADR-0001, design-документ (`./2026-05-09-A1-note-id-scheme-design.md`). Все ссылки разрешимы. ✓
- **Language**: проза по-русски, идентификаторы латиницей. Очень небольшое количество калек: «mutual constraint» (можно «взаимное ограничение», но как цитата из источника — оставимо), «threat model» (term-of-art, оставимо). Над одной фразой стоит подумать: «load-bearing аргумент о корректности» — «load-bearing» здесь калька-метафора, можно «несущий».
- **Conformance verdict**: high-конформен. Образцовое использование роли.

### 2026-05-09-A1-roast-compliance-officer.md
- **Header check**: `# Compliance-officer findings: A1 — Note ID scheme + filename layout` (H1).
- **Preamble**: `**Target**`, `**Date**`, `**Role**`, плюс `**Disclaimer**` («I am not a lawyer …»). Disclaimer — здравая практика для compliance-роли. Не предписано плагином, но соответствует осторожному тону, который плагин в `commands/roast.md` ожидает.
- **Identifiers**: `GDPR`, `HIPAA`, `152-FZ`, `PCI-DSS`, `CCPA`, `ULID`, `Crockford-base32`, `serde_yaml`, `git rm`, `git filter-repo`, `BFG`, `format-v1`, `ARCHITECTURE.md`, `[0-9A-HJKMNP-TV-Z]{26}` (regex)— сохранены. ✓ **Малая деталь**: `152-FZ` — у пользователя в категории D `SKILL.md` указан в форме `152-ФЗ` (кириллицей). Артефакт использовал `152-FZ`. Не ошибка перевода (имя закона как идентификатор), но неконсистентно с написанием в `architect/SKILL.md`. Low.
- **Finding-ID scheme**: C-1 … C-6, шесть уникальных. Префикс `C-N` совпадает с плагином (`roast.md` строка 162). ✓
- **Severity**: каждое finding имеет `**Severity**: medium|low`. Распределение: 0 high, 3 medium, 3 low. Честно — для соло-инструмента с N/A compliance-postur'ом отсутствие high уместно.
- **Field structure**: каждое finding — `**Category**`, `**The gap**`, `**Where in the artifact**`, `**Severity**`, `**What would close this**`. Структура богаче, чем минимум плагина, и последовательная по всем шести findings. ✓
- **Cross-references**: ARCHITECTURE.md §4 («Compliance — N/A»), §7 (no-cloud), open questions A3 — все разрешимы. ✓
- **Language**: проза по-русски, идентификаторы — латиница. Калек: «PII» (стандартная аббревиатура, оставимо), «cross-border processing» (term-of-art, можно «трансграничная обработка»), «right-to-erasure analogue» (term-of-art GDPR), «correlation ID» (можно «идентификатор корреляции»). В целом проход чистый.
- **Conformance verdict**: high-конформен.

### 2026-05-09-A1-roast-futurist.md
- **Header check**: `# Futurist findings: A1 — Note ID scheme + filename layout` (H1).
- **Preamble**: `**Target**`, `**Date**`, `**Role**`, плюс `**Horizon**` и `**Confidence note**`. Расширения уместные для futurist-роли.
- **Identifiers**: `ULID`, `UUIDv7`, `RFC 9562`, `format-v1`, `format-v2`, `[0-9A-HJKMNP-TV-Z]{26}`, `gray_matter`, `serde_yaml`, `atomic-write-file`, `git mv`, `git log --follow`, `slug` crate, `deunicode`, имена альтернатив в decision-space (`A`, `B`, `C`), Group-A/D/F-references — сохранены. ✓
- **Finding-ID scheme**: F-1 … F-9, девять уникальных. Префикс `F-N` совпадает с плагином (`roast.md` строка 162 даёт `F1.2` — там точечная вложенность, у нас линейная `F-N`). Это **мягкое расхождение**: плагин допускает sub-numbering, артефакт использует плоскую нумерацию. В обоих случаях префикс `F`, ID-уникальны. Помечаю low.
- **Severity / Confidence**: трактуется по-разному в structural vs trend-секциях — у structural нет явного severity, есть `Type` и `Horizon`; у trend есть `Confidence: low|medium`. **Это допустимо** по `roast.md` строке 109 (futurist может не использовать severity).
- **Cross-references**: ссылок на STRATEGY/ARCHITECTURE/decision-map в явной форме нет, но decision-map ID (D4, A1, A2, A3, A5, B1) упоминаются. Sources (URL'ы) перечислены отдельным разделом — это полезное добавление, не предписано плагином.
- **Language**: проза по-русски, кальки: «дрейф» (хорошо прижилось как term-of-art), «inertia» (англ., можно «инерция» — стоит проверить, плагин эту роль описывает в английских терминах, но в русском артефакте имеет смысл перевести; впрочем, оно использовано в `Type:` поле, что маркирует его как идентификатор). «Codebase aging» — `Type:` поле, идентификатор. Vendor risk, idiom shift, hiring — все в `Type:` поле. Это допустимая граница: имена категорий = идентификаторы.
- **Conformance verdict**: high-конформен.

## Cross-file consistency

### Структурные расхождения (high, общие для всех 5 файлов)

**M-1**: **Артефакты не сложены в директорию `2026-05-09-roast-<slug>/`.**
`commands/roast.md` §4 (строки 67–76) предписывает структуру:
```
docs/architecture/reviews/YYYY-MM-DD-roast-<artifact-slug>/
  ├── 00-summary.md
  ├── 01-devil-advocate.md
  ├── 02-pragmatist.md
  ├── 03-junior-engineer.md
  ├── 04-compliance-officer.md
  └── 05-futurist.md
```
Фактически файлы лежат плоско в `docs/architecture/reviews/` с префиксом `2026-05-09-A1-roast-<role>.md`. Это ломает (а) предсказуемое расположение, на которое опирается `meta-review` и downstream-инструменты; (б) per-role-файлы пронумерованы не по плагину (`01-devil-advocate.md`, `02-pragmatist.md`, …) а по проектной конвенции `2026-05-09-A1-roast-<role>.md` без числового префикса.

**Severity**: high. Структурное расхождение от прескрипции плагина.

**Suggested fix**: переместить пять файлов в директорию `docs/architecture/reviews/2026-05-09-roast-A1-note-id-scheme/`, переименовать в `01-devil-advocate.md` … `05-futurist.md`. Старые ссылки внутри файлов на сами себя — их нет, поэтому переезд безопасен.

**M-2**: **Отсутствует `00-summary.md`.**
`commands/roast.md` §5 (строки 81–123) предписывает summary с разделами:
- `## Headline findings` (по одной строке от каждой роли)
- `## Severity counts` (таблица)
- `## Cross-cutting concerns` (findings, поднятые 2+ ролями)
- `## Recommended path` (Apply / Apply and re-roast / Step back)
- `## Per-role outputs` (ссылки на per-role-файлы)

В артефакте summary нет вовсе. Это **критический пробел** — summary это «read-first» поверхность роста по плагину; без неё архитектор должен прочитать все 5 длинных документов, чтобы понять, какие findings — cross-cutting (devil-advocate A-1 и pragmatist P-2 указывают на ту же rename-семантику; devil-advocate A-3 и junior-engineer J-11 — на frontmatter-vs-filename контракт; futurist F-3 и pragmatist P-4 — на migration-tool обязательство). Эти cross-cutting не консолидированы.

**Severity**: high.

**Suggested fix**: написать `00-summary.md` по шаблону из `commands/roast.md` строки 84–123, с **Headline findings** по одной строке от каждой роли, **Severity counts** таблицей (devil-advocate 4H/4M/0L, pragmatist 0H/4-5M/1L, compliance 0H/3M/3L; junior и futurist — ranges или omit), **Cross-cutting concerns** минимум по этим парам:
- B-1 ↔ P-2 ↔ M-? — rename atomicity и multi-machine sync (3 роли).
- B-3 ↔ J-11 — frontmatter `id:` vs filename как source of truth (2 роли).
- F-3 ↔ P-4 — `format-v1` freeze без migration tool (2 роли).
- F-2 ↔ C-1 — ULID-timestamp в filename как leak surface vs canonical permalink (2 роли смотрят с разных сторон).
- A-5 — ULID-monotonic ошибка факта (только devil-advocate, не cross-cutting, но **load-bearing** для содержания summary).

**M-3**: **Префикс finding-ID для devil-advocate расходится с источником плагина.**
`commands/roast.md` строка 162: «Finding IDs stay in their Latin form (`B-1`, `H-3`, `J-2`, `C-1`, `F1.2`, `CC-3`).» То есть для devil-advocate префикс — `B-` (от *brutal* / *break*), не `A-`. Артефакт использует `A-1` … `A-8`. Дополнительно: в проектной номенклатуре `A1/A2/A3/A4/A5` — это идентификаторы решений в decision-map (Group A); в одном документе `A-1` (finding) и `A1` (decision) различаются только дефисом, что снижает читаемость.

**Severity**: medium (не ломает downstream-инструменты, но десинхронизирует с источником плагина и создаёт коллизию имён внутри проекта).

**Suggested fix**: переименовать `A-1` … `A-8` → `B-1` … `B-8` во всём файле. Других файлов это не затрагивает (cross-references на A-N findings из других ролей в текущих 4 файлах не обнаружены — проверено: в pragmatist/junior/compliance/futurist нет ссылок «A-3», «A-5» и т.п.).

### Расхождения с конвенцией пользователя (medium)

**M-4**: **Заголовок верхнего уровня — H1, ожидался H2.**
Пользователь в задаче зафиксировал ожидание: «each file should open with `## <Role>-findings` heading verbatim». Все 5 файлов открываются `# <Role>-findings: …` (H1, с продолжением через двоеточие). 

Сам плагин (`commands/roast.md`) **per-role-document начальный заголовок не прописывает** — он предписывает только заголовки summary (`## Headline findings` и т.д.). То есть формально расхождения с шаблоном плагина нет. Расхождение есть с конвенцией пользователя.

Аргумент в пользу H1: это первый заголовок файла, по Markdown-семантике это title документа, и H1 уместен. Аргумент в пользу H2: единообразие с другими роли-файлами в проекте, и H1 обычно резервируется за верхним уровнем структуры (а не за самим заголовком документа, если документ — секция большего целого).

**Severity**: medium-low. Если пользователь хочет H2 — это однострочное правило в правке, безопасное к применению. Если пользователь не настаивает — оставить H1.

**Suggested fix**: либо (а) принять H1 как локальную конвенцию и обновить ожидание; либо (б) заменить `# <Role>-findings:` на `## <Role>-findings` (без двоеточия и продолжения, чтобы заголовок был verbatim) и перенести содержимое («ADR-0002 (A1 — ULID + slug filename layout)» и т.п.) в первую строку preamble-блока.

### Содержательные расхождения (low)

**M-5**: **`152-FZ` vs `152-ФЗ`.** В compliance-officer файле использовано `152-FZ` (latin). `architect/SKILL.md` категория D даёт каноническую форму `152-ФЗ` (кириллица). Это идентификатор закона, поэтому категория NO-translate в обе стороны — но **каноническая форма у пользователя кириллическая**.

**Severity**: low.

**Suggested fix**: заменить `152-FZ` → `152-ФЗ` в одном месте C-файла (раздел «Applicable regulations and standards»).

**M-6**: **ADR-0002 ссылается на ещё-не-существующий файл.** Все 5 файлов в `**Target**`-поле указывают на `docs/architecture/research/2026-05-09-A1-decision-summary.md` (существует) и в заголовках упоминают `ADR-0002`. В `docs/architecture/decisions/` сейчас только `0001-project-name-and-ecosystem-positioning.md`. Decision-summary сам декларирует «Will become: ADR-0002», то есть это forward-looking имя. Roast выполнен на decision-summary (legitimate — это целевой артефакт по `roast.md` §1), но ссылка `ADR-0002` в заголовках — на artefact, который ещё не существует.

**Severity**: low. Это не сломанная cross-reference, это forward-reference.

**Suggested fix**: либо (а) принять forward-reference как явный сигнал о намерении (и в summary упомянуть это), либо (б) использовать в заголовках ту же формулировку, что в `**Target**`: «A1 — Note ID scheme» без `ADR-0002`. Я бы оставил форму с `ADR-0002` — она маркирует намерение и сразу станет валидной, как только document-фаза создаст файл.

## Recommended fixes (по приоритету)

1. **[high]** Создать `00-summary.md` по шаблону `commands/roast.md` §5. Минимум — 5 headline findings (по одному от каждой роли, в одну строку), таблица severity counts, секция cross-cutting concerns с парами {A-1↔P-2}, {A-3↔J-11}, {F-3↔P-4}, и recommended path (предположительно «Apply and re-roast» — есть один factual error A-5 и три-четыре gap'а, требующих правок в decision-summary).
2. **[high]** Переместить 5 файлов в директорию `2026-05-09-roast-A1-note-id-scheme/` и переименовать в `01-…05-`. Это приводит layout в соответствие с плагином и упрощает следующий `meta-review`.
3. **[medium]** Переименовать finding-ID `A-N` → `B-N` в devil-advocate-файле.
4. **[medium-low]** Решить вопрос H1/H2 в начальном заголовке (либо принять H1 как локальную конвенцию, либо переписать в H2). Если оставлять H1 — зафиксировать это в проектной конвенции, чтобы следующий roast не получил замечание.
5. **[low]** `152-FZ` → `152-ФЗ` в compliance-файле.
6. **[low]** В futurist-файле `F1.2`-стиль не использован, использован плоский `F-N` — оставить (соответствует консистентности с другими ролями), но при желании можно перейти на `F1.1, F1.2, F2.1, F2.2` для structural vs trend.
7. **[low / опционально]** Прогнать ещё один раунд калек по pragmatist (sync, overwrite, foot-gun) и compliance (cross-border processing, correlation ID) — проза станет чище без потери term-of-art.

## What conforms (sustainment list)

- **Идентификаторы — практически безупречно.** Имена ролей verbatim (`Devil-advocate`, `Pragmatist`, `Junior-engineer`, `Compliance-officer`, `Futurist`), имена крейтов, Crockford base32, ULID, format-v1, ADR-номера, имена git-команд, `/archforge:*`-команды — всё латиницей в правильной форме. Категории A–F из `architect/SKILL.md` соблюдены.
- **Finding-ID уникальны внутри каждого файла.** Никаких дублей J-3 ↔ J-3 и т.п. Префиксы (J-, C-, F-, P-) корректны для своих ролей.
- **Severity-распределения честные.** Devil-advocate 4H/4M (агрессивно, но обосновано); pragmatist 0H/4M/1L/1+; compliance 0H/3M/3L; junior — `Category` без severity (что плагин разрешает, `roast.md` строка 109); futurist — `Type/Horizon/Confidence` без severity (то же).
- **Cross-references существующие.** STRATEGY.md, ARCHITECTURE.md, decision-map.md — все три файла существуют в репо. Ссылки на decision-map §A4, §A5, §D4 — соответствующие группы в `decision-map.md` присутствуют.
- **Каждое finding имеет `Where in the artifact:`** (или эквивалент) — указывает на конкретный раздел target'а. Это критично для actionability и отлично выдержано во всех 5 файлах.
- **Roles не пересекаются.** Devil-advocate атакует concurrency / hidden assumptions / data integrity; pragmatist говорит про operational debt и supply chain; junior — про clarity и cross-references; compliance — про PII flow и data retention; futurist — про structural drift и trend speculation. По `roast.md` строкам 35 и 149 это правильное поведение. Был один пограничный случай (devil-advocate A-8 vs pragmatist P-2 — оба касаются git-sync rename), но devil-advocate смотрит как на silent corruption (data integrity), pragmatist как на «1 ручное вмешательство в год + runbook» (operational) — это два разных среза одного underlying issue, и summary должен их свести в cross-cutting concern, а не считать дублем.
- **Каждый файл имеет внятный preamble с Target, Date, Role.** Опциональные расширения (Reading posture у junior; Disclaimer у compliance; Horizon + Confidence note у futurist) — уместные для своих ролей и не противоречат шаблону.

## Areas not covered by this review

Этот meta-review проверяет **structural conformance** артефактов с шаблонами и правилами плагина archforge 0.4.0-rc3. Он **не оценивает**:

- Архитектурное качество атак / findings (это работа самого roast'а, который уже выполнен).
- Корректность факта в A-5 («`Ulid::new()` не monotonic») — формально это содержательное finding devil-advocate'а, и meta-review не валидирует его правдивость, только то, что оно структурно оформлено.
- Качество recommend'ов в каждом файле — будут они полезны архитектору или нет.
- Code-уровень корректности упомянутых крейтов и API.
- Пригодность сделанного A1-решения для проекта в целом.

Эти вопросы — для архитектора при чтении roast'а и принятии решения по recommended path, либо для отдельной content-review. Meta-review здесь останавливается на «соответствует ли артефакт тому, что плагин обещал».

---

**Терминологический проход**: идентификаторы (имена ролей, имена крейтов, `ULID`, `ADR-0002`, `format-v1`, `/archforge:*`-команды, finding-ID) сохранены без перевода. Прозу вёл по-русски; cross-cutting/cross-references/term-of-art оставил с гидом или в кавычках, где уместно. Заменил «issue/issue tracker»-калек нет (в тексте они не возникали); «cross-reference» оставлен, потому что это term-of-art шаблона (категория F через ассоциацию). Источник правил: `commands/roast.md` строки 156–167; `skills/architect/SKILL.md» секция «The taxonomy».
