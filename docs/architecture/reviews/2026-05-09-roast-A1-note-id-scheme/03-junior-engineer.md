# Junior-engineer findings: 2026-05-09-A1-decision-summary.md

- **Target**: `docs/architecture/research/2026-05-09-A1-decision-summary.md`
- **Date**: 2026-05-09
- **Role**: junior-engineer (clarity)
- **Reading posture**: знакомлюсь с проектом впервые. Есть STRATEGY.md, ARCHITECTURE.md, decision-map.md и ADR-0001. Только что прочитал summary. Discovery/research-файлы не открывал. Утро понедельника, спросить некого.

## Summary

Документ в целом читается: формат ID и его обоснование переданы понятно для инженера, который уже различает ULID/UUID и знаком с YAML frontmatter. Главная проблема — он плотно завязан на **внешние ID-якоря (A2, A5, B1, D4, F3.5, O-8, X4, ideation references)**, которые в `decision-map.md` существуют, но в самом summary не объясняются вообще; и на **жаргон Rust/PKM/Zettelkasten/git-VCS-internals**, который местами вводится без определения. Хуже всего — раздел про `std::fs::rename` атомарность с непрослеживаемой ссылкой на «PR #138133 в 2025», которую читатель не сможет разрешить без раскопок.

## Clarity findings

### J-1: «Crockford base32» введён без определения и без указания, почему это важно

**Category**: undefined term

**The gap**: «ULID (Crockford base32, 26 chars, time-prefixed)». Дальше идёт замечание «Crockford-кодирование встроено в крейт (использовать внешний `crockford`/`fast32` нельзя — generic encoder выдаёт несовместимые строки, см. `ulid/spec` issue #81)» — это зрелая отсылка к чему-то, чего я не знаю.

**What I tried**: STRATEGY/ARCHITECTURE/decision-map/ADR-0001 — Crockford нигде не упоминается. Из контекста могу догадаться, что это какой-то конкретный алфавит (без `I/L/O/U`?), но не понимаю, в чём отличие от «обычного» base32 и почему generic encoder выдаёт «несовместимые строки».

**Suggested fix**: Одна строка-сноска при первом упоминании: «Crockford base32 — алфавит на 32 символа, исключающий визуально схожие `I/L/O/U`; ULID-spec предписывает именно его, а не RFC 4648 base32, потому что [причина]».

### J-2: «O(1) через filename-glob» — что именно делает резолвер

**Category**: erased reasoning / unfollowable step

**The gap**: «Резолюция: O(1) через filename-glob `<ULID>-*.md` или `<ULID>.md`. Нет необходимости в persistent index для одного lookup».

**What I tried**: Filename-glob по дереву файлов в обычной FS — это O(N) на чтение каталога, а не O(1). Что именно подразумевается под O(1)? Что один-единственный glob-запрос → один FS syscall? Что предполагается flat layout (но A4 ещё открыт)? Если у пользователя 10 000 заметок в одном каталоге, какое будет реальное время lookup-а?

**Suggested fix**: Уточнить: «O(1) в смысле „один FS-вызов для конкретного ULID-префикса“, не „константа независимо от числа заметок“. На flat layout с N заметками readdir остаётся O(N); проблема снимается на уровне A4».

### J-3: «PR #138133 в 2025» — куда смотреть

**Category**: broken cross-reference

**The gap**: «atomic на Windows ≥10 1607 после PR #138133 в 2025». PR в каком репозитории? Rust stdlib? Windows? `atomic-write-file`? Я не могу проверить это утверждение, не зная, куда идти. Это load-bearing аргумент о корректности retitle на Windows.

**Suggested fix**: Указать репозиторий: «PR rust-lang/rust#138133 (стабилизирован в Rust 1.XX)» или дать прямой URL.

### J-4: Идентификаторы A2/A3/A4/A5/B1/D4 используются раньше, чем читателю объяснено, что это за номенклатура

**Category**: undefined term / forced jump-out

**The gap**: «это решается отдельно (B1)», «D4-strict-checker позиции», «B1 (graph index)» — нумерация не вводится. В шапке указано «Will become: ADR-0002» и «Cycle: A1, deep» — но что такое A1 относительно A2/B1, не сказано.

**What I tried**: Открыл `decision-map.md` — там есть Group A / B / C / D с пронумерованными решениями. Но summary должен либо заявить это вверху, либо явно сослаться.

**Suggested fix**: В шапке (после «Cycle: A1, deep») добавить: «Идентификаторы A1/A2/B1/… — индекс открытых решений в [`decision-map.md`](../decision-map.md)».

### J-5: «Alternative A chosen» / «Альтернативы B и C» — A/B/C из чего

**Category**: erased reasoning / broken cross-reference

**The gap**: «design (Alternative A chosen)». Раздел «Альтернативы B и C из design — почему не выбраны» обсуждает «B (slug-only filename)» и «C (ULID-only filename)», но самого «А» как альтернативы в документе нет — оно растворилось в выбранном решении. Также упоминается «full table в `2026-05-09-A1-note-id-scheme-design.md`» без относительного пути.

**Suggested fix**: В начале раздела «Альтернативы B и C» дать одну строку: «Рассматривались три варианта filename layout: A — `<ULID>-<slug>.md` (выбран); B — slug-only; C — ULID-only». И дать относительный путь к design-документу: `./2026-05-09-A1-note-id-scheme-design.md`.

### J-6: «D4-strict-checker позиции» — что это значит

**Category**: undefined term / forced jump-out

**The gap**: «B (slug-only filename): оптимален для D4-strict-checker позиции». Без отсылки за пределы summary я не понимаю, в чём состоит эта «позиция» и почему slug-only filename ей оптимален.

**What I tried**: Открыл `decision-map.md` — D4 это «Obsidian-vault compatibility posture» с четырьмя вариантами (a/b/c/d), четвёртый — «strict checker поверх vault». Связь становится понятной только после этого раунда.

**Suggested fix**: «оптимален для варианта D4 = strict checker поверх Obsidian-vault (zetto не редактирует, только проверяет; см. `decision-map.md` §D4)».

### J-7: Бюджет «< 50 ms» в ID generation — относится к чему из бюджета STRATEGY

**Category**: number without context

**The gap**: «capture latency budget на этом шаге < 50 ms (ULID generation ~ns; slug normalization ~µs; FS write ~ms)». В `ARCHITECTURE.md` §2.1 есть подробный capture-latency budget по этапам. Эти 50 ms из summary относятся к какой строке таблицы? Это «Save»? «Process startup»? Гибрид?

**Suggested fix**: «… < 50 ms — укладывается в строку „Save“ (<300 ms) capture-latency budget из `ARCHITECTURE.md` §2.1».

### J-8: «Capture latency budget на этом шаге» — пропущенное звено в шагах 1–7

**Category**: unfollowable step

**The gap**: Алгоритм ID generation перечисляет 7 шагов, но непонятно, **внутри какого процесса** они происходят. Шаг 7 — «Готово». Что именно «Готово»? Файл на диске и заметка открыта в `$EDITOR`? Или процесс заканчивается до открытия редактора? `zetto new` после этих 7 шагов делает что-то ещё?

**Suggested fix**: После шага 7 одна строка о границе ответственности этого алгоритма: «Шаги 1–7 описывают только генерацию и первичную запись skeleton-файла; spawn `$EDITOR` и последующее редактирование — отдельный поток (см. C4 capture flow)».

### J-9: «format-v1» введён как уже существующий артефакт

**Category**: undefined term / hidden boundary

**The gap**: «ULID + slug filename layout фиксируется как часть `format-v1` (см. A5 в `decision-map.md`)». Дальше: «Mажорной версии `format-v2`», «`zetto migrate format-v1 format-v2`».

**What I tried**: A5 в decision-map.md помечен как **open**. То есть `format-v1` — ещё не существующий контракт; он будет определён, когда A5 закроется. Summary говорит про него как про уже принятую сущность.

**Suggested fix**: «… фиксируется как обязательный inclusion в будущей спецификации `format-v1`, которая будет определена в A5 (сейчас open)».

### J-10: «D4-будущим» в разделе зависимостей — мост через текст

**Category**: unresolved pronoun / hidden boundary

**The gap**: «Зависимость от `gray_matter` обеспечивает совместимость с D4-будущим». Что значит «D4-будущим»? Любым исходом D4? Или конкретно вариантом D4=read-write Obsidian compat?

**Suggested fix**: «… обеспечивает совместимость с любым исходом D4, включая read-write Obsidian-vault compat».

### J-11: «backlinks `[[ULID]]` ломаются» — что значит «ломаются» в этой модели

**Category**: erased reasoning

**The gap**: «Ручное изменение `id:` пользователем — нарушение контракта; backlinks `[[ULID]]` ломаются». Если пользователь поменяет `id:` в frontmatter, но имя файла не переименует — backlinks резолвятся по filename, и они **не ломаются** (пока не запустится retitle с регенерацией). Какой именно сценарий имеется в виду?

**Suggested fix**: «Если пользователь правит `id:` в frontmatter, последующий retitle переименует файл по новому ULID-префиксу; все существующие `[[<старый-ULID>]]` backlinks перестанут резолвиться».

### J-12: «empty-slug fallback» — что именно проверяется

**Category**: unfollowable step

**The gap**: «Empty-slug fallback: `<ULID>.md` (без хвостового `-`). Срабатывает когда title даёт пустой slug — все CJK кандзи, которые `deunicode` не транслитерирует читаемо, чистый emoji, нерасшифровываемые символы». «Не транслитерирует читаемо» — это решение алгоритма `deunicode` или решение `zetto`?

**Suggested fix**: Указать конкретный предикат: «Fallback срабатывает, когда `slug::slugify(title)` возвращает пустую строку либо строку из одних разделителей».

### J-13: Раздел «Что дальше» — список ссылок на команды без объяснения, что это за инструмент

**Category**: undefined term

**The gap**: Ссылки на `/archforge:roast`, `/archforge:meta-review`, `/archforge:document` как на самоочевидные команды. Из контекста ясно, что это какой-то workflow-инструмент, но какой и где он живёт — нет.

**Suggested fix**: Одна строка-сноска: «`/archforge:*` — команды плагина archforge для архитектурного workflow проекта».

## What's well-documented

- **Раздел «Канонический ID»** — формат, хранение, генерация, lifecycle, sortability перечислены отдельно и понятно.
- **Таблица зависимостей** — крейты, версии, лицензии. Стандартный формат.
- **Slug normalization по языкам** с конкретными примерами `Заметка о ULID → zametka-o-ulid`, `ノート → noto`, `🔥 → fire`. Конкретность снимает большинство вопросов.
- **Rejection summary** одной строкой — явно перечисляет, что не выбрано.
- **Edge case rename** (collision между slug-ами разных заметок) явно адресован.

## Where I gave up

- **Раздел «ID monotonicity и concurrency»**: понимаю слова, но не понимаю threat model. «Monotonic в spec» — какой spec? Какой механизм гарантирует, что внутри одного процесса ULID всё ещё растёт? Я могу прочитать crate-doc, но summary должен был указать, какую гарантию я наследую и какую — нет.
- **«Approximately chronological modulo wall-clock skew (документированный caveat)»** — где «документированный»? В `zetto` README, который ещё не написан? В docstring команды? Я не понимаю, кто и когда увидит этот caveat.
- **Связь A1 и A2 («Mutual constraint»)** — `decision-map.md` §Notes говорит, что A1 и A2 имеют mutual constraint и могут быть bundled. Summary решает только A1, но фиксирует «identifier внутри линка — ULID». Я не уверен, может ли A1 быть решён в отрыве от A2.
