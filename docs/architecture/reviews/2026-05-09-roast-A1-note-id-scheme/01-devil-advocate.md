## Devil-advocate findings: ADR-0002 (A1 — ULID + slug filename layout)

- **Target**: `docs/architecture/research/2026-05-09-A1-decision-summary.md`
- **Date**: 2026-05-09
- **Role**: devil-advocate (adversarial)

## Summary

Решение опирается на два хрупких допущения, которые не отражены в документе. Первое: filename и frontmatter `id:` рассинхронизируются, и не зафиксировано, что считать истиной, когда пользователь вмешается руками или столкнётся с merge-conflict от git. Второе: lookup `[[ULID]]` через filename-glob падает в degenerate-режим (full FS-scan) ровно в тех случаях, которые документ объявляет «edge», но эти случаи возникают по умолчанию — empty-slug fallback, manual editing, sync-конфликты. Atомичность retitle-операции (rename файла + edit frontmatter title) не определена вообще.

## Attacks (B-1 … B-8)

### B-1: Retitle — это две независимые операции без атомарности

**Type**: concurrency bug / failure mode

**The attack**: Раздел «Поведение при retitle» описывает: (1) меняется frontmatter `title`, (2) zetto регенерирует slug, (3) `std::fs::rename` переименовывает файл. Но edit frontmatter и rename файла — **две отдельные FS-операции**. Если процесс прервался между ними (Ctrl-C, OOM-killer, сбой питания, vim crashed после `:w` но до post-write hook) — на диске остаётся файл со старым slug и новым title во frontmatter. На следующем запуске zetto не знает: это рассинхрон от crash, или пользователь намеренно отредактировал title, не желая rename? Документ говорит «atomic-write-file used only for body-edits», то есть для frontmatter edits атомарность не гарантируется вовсе. Усугубляется тем, что edit обычно делает *vim*, а не zetto — у zetto нет шанса обернуть это в транзакцию.

**Where in the artifact**: «Поведение при retitle», шаги не пронумерованы и не описана последовательность; «Атомарность: `std::fs::rename` достаточна для slug-rename» — но rename — только третий шаг из трёх.

**Severity**: high (silent inconsistency между filename и frontmatter в долгоживущем vault с тысячами заметок).

### B-2: Lookup `[[ULID]]` через filename-glob ломается, когда filename — `<ULID>.md` без слэша

**Type**: edge case / logical inconsistency

**The attack**: Документ заявляет «Резолюция: O(1) через filename-glob `<ULID>-*.md` или `<ULID>.md`». Это уже **два glob-паттерна**, и `<ULID>-*.md` matches `<ULID>-anything.md`, но не `<ULID>.md` — это разные glob-выражения. Реализация должна сначала пробовать один, потом другой (или комбинировать в `<ULID>*` — но тогда захватываются ULIDs, начинающиеся одинаково, что для ULID-monotonic в одной мс **технически возможно**: monotonic-режим инкрементирует random-часть, оставляя timestamp-prefix общим, и две заметки в одной мс получают ULIDs, чей общий prefix может перекрыться при коротком glob). То есть «O(1) lookup» — на самом деле «попробовать два паттерна, если оба не match — fallback», и этот fallback не описан. Что если пользователь руками удалил slug из filename, оставив `<ULID>-.md` или `<ULID>-foo` (без `.md`)? Glob промахивается молча.

**Where in the artifact**: «Линкование», «Резолюция: O(1) через filename-glob».

**Severity**: medium (broken backlinks при manual filename edit или partial state).

### B-3: Frontmatter — source of truth, но zetto не валидирует его

**Type**: hidden assumption

**The attack**: Документ говорит «Хранение: YAML frontmatter поле `id`. Это source of truth» и «Ручное изменение `id:` пользователем — нарушение контракта; backlinks `[[ULID]]` ломаются». Но **ничего не сказано о том, что zetto проверяет инвариант** «frontmatter `id` совпадает с ULID-prefix filename». Если они разошлись — а они разойдутся (при copy-paste заметки руками, при git merge с конфликтом, при том что пользователь скопировал шаблон и забыл заменить ID) — какой источник побеждает? Документ заявляет frontmatter, но **lookup идёт через filename-glob**. То есть в реальности filename — практический source of truth для резолва, а frontmatter — источник для записи. Два конфликтующих контракта на один и тот же объект.

**Where in the artifact**: «Канонический ID — Хранение» против «Линкование — Резолюция».

**Severity**: high (data integrity — backlink резолвится «не туда», тихо).

### B-4: Empty-slug fallback порождает скрытый класс коллизий

**Type**: edge case

**The attack**: Fallback на `<ULID>.md` при пустом slug-е активируется не только для CJK кандзи и emoji, но и для **любого title, который при создании ещё не вписан** (capture flow: пользователь жмёт `zetto new`, ULID присваивается, а title пишется потом в редакторе). Документ говорит в шаге 2 алгоритма: «если пустой — slug пустой, fallback на ULID-only filename». То есть **большинство свежесозданных заметок начинают жизнь как `<ULID>.md`**. Затем пользователь пишет title и сохраняет — и вот здесь срабатывает retitle-логика. Но retitle-логика срабатывает на change в frontmatter, а если zetto не watcher — кто триггерит regenерацию slug? Пользователь должен явно вызывать `zetto retitle`? Не сказано. Скорее всего — половина vault-а навсегда останется в форме `<ULID>.md`, и весь UX-аргумент против альтернативы C («стена ID в `ls`») аннулируется на практике.

**Where in the artifact**: «ID generation на `zetto new`» шаг 2; раздел «Поведение при retitle» не упоминает initial-title-flow.

**Severity**: high (UX-аргумент решения против C обнуляется в реальном workflow).

### B-5: ULID-monotonic в одном процессе vs split-brain между процессами

**Type**: concurrency bug

**The attack**: «Внутри одного `zetto`-процесса: используем `Ulid::new()`, который на overflow random-части в одной мс возвращает ID с инкрементированной random-частью (monotonic в spec)». Но `Ulid::new()` из крейта `ulid` 1.2.1 — это *не* monotonic-генератор. Monotonic поведение требует `ulid::Generator::generate_from_datetime` со стейтом (см. research digest §1: «monotonic-API через `Generator::generate_from_datetime`»). `Ulid::new()` каждый вызов — независим, monotonicity между двумя `Ulid::new()` подряд *не гарантирована*. Это фактическая ошибка в decision-summary. Дополнительно: zetto-CLI — это short-lived процессы (`zetto new` запускается, делает работу, выходит). Между двумя `zetto new` подряд состояние Generator-а теряется. Monotonic-гарантия в принципе недоступна для CLI-shape без external persistent state. Документ декларирует свойство, которого реализация иметь не будет.

**Where in the artifact**: «ID monotonicity и concurrency», первый bullet.

**Severity**: medium (документ обещает invariant, который implementation не даст; ловится первым же читателем кода).

### B-6: Manual `id:` edit ломает backlinks молча, без detection

**Type**: adversarial scenario / failure mode

**The attack**: «Ручное изменение `id:` пользователем — нарушение контракта; backlinks `[[ULID]]` ломаются». Хорошо, контракт нарушен. Но **что zetto делает, когда обнаруживает нарушение?** Ничего не сказано. Сценарий: пользователь копирует заметку (`cp note.md note-copy.md`) — у двух файлов одинаковый `id:` во frontmatter, но filename-prefix разный (если он не переименовал) или одинаковый (если переименовал). Два файла с одним ULID. Резолв `[[ULID]]` через filename-glob теперь возвращает два кандидата. Документ не описывает поведение. Скорее всего — первый по сортировке win-ит, второй становится «заметка-зомби», на которую никто не сошлётся. Это нормальный сценарий для пользователя, который привык к plain-text workflow («хочу склонировать заметку»).

**Where in the artifact**: пробел в разделе «Жизненный цикл»; нет раздела «инварианты, которые zetto проверяет».

**Severity**: medium (silent dup, обнаруживается только при попытке открыть линк).

### B-7: Slug-rename атомарность на Windows опирается на патч 2025 г.

**Type**: hidden assumption

**The attack**: «atomic на Windows ≥10 1607 после PR #138133 в 2025». Это допущение, что (a) пользователь на Windows 10 ≥ build 1607 (около 60% Windows-пользователей в 2025; остаток — на 1507/RS1, server 2016 без updates, или на старой 1607 без PR-fix); (b) `std::fs::rename` уже использует новую path code. Research digest сам пишет: «PR #138133 (2025) пропатчил stdlib пробовать `FileRenameInfo` сначала, fallback на `FileRenameInfoEx`». Это **stdlib patch** — нужен Rust toolchain версии, в который этот патч уже вошёл. Какой MSRV? Документ MSRV не фиксирует. Если zetto компилируется старым `rustc`, поведение rename — old, foot-gun. На Windows вне 1607 — `ERROR_INVALID_PARAMETER`, и zetto падает. Документ не упоминает MSRV, не упоминает Windows-пользователей, не описывает что делает CLI при rename failure (откатить frontmatter title? оставить рассинхронизированным?).

**Where in the artifact**: «Атомарность: `std::fs::rename` достаточна … atomic на Windows ≥10 1607».

**Severity**: medium (broken на части Windows-инсталляций; rename failure-handling undefined).

### B-8: Git merge-конфликт во frontmatter `id:` поле — silent corruption

**Type**: failure mode (multi-machine sync)

**The attack**: «Cross-machine guarantee: ULID глобально уникален бесплатно … sync через git между машинами не требует никакой координации». Это про создание разных заметок на разных устройствах. Но ничего не сказано про **edit одной и той же заметки на двух устройствах в офлайне**. Сценарий: машина A retitle-ит заметку, slug меняется, filename изменился с `01J9X-old.md` на `01J9X-new.md`, frontmatter `title:` тоже обновился. Машина B параллельно сделала body-edit на `01J9X-old.md`. Git pull/merge: на машине B файл с одним именем, на машине A — с другим. Git видит это как rename + body conflict, или как delete+add — зависит от similarity heuristic. Если heuristic не сработает (большой body diff), машина A получает свою версию `01J9X-new.md`, а машина B видит `01J9X-old.md` как deleted. Backlinks обоих устройств указывают на ULID — резолв падает на одной из машин. Документ говорит «не требует координации», но координация неявно требуется — пользователь должен либо retitle делать в одной машине, либо разрешать merge вручную.

**Where in the artifact**: «Cross-machine guarantee»; раздел «Поведение при retitle» не учитывает distributed-сценарий.

**Severity**: high (silent broken backlinks после git pull, обнаруживаются спустя дни).

## Strongest single attack

**B-4: Empty-slug fallback — это не edge, это default state.** На `zetto new` ULID присваивается до того, как пользователь напечатает title. Если zetto не запускает retitle-flow после первой записи title — а из документа не видно, чтобы запускал — половина vault-а перманентно остаётся в форме `<ULID>.md`. Главный аргумент документа против альтернативы C («стена ID в `ls` режет terminal-native UX») аннулируется в практике: vault де-факто становится C, только с опциональными slug-ами для тех заметок, которые пользователь явно ретайтлил. Это не «edge case с CJK» — это первичный workflow.

## Gaps in own analysis

- Производительность ULID generation на embedded/ARM: research digest упоминает «ns generation», но не верифицировано на slow ARM где-то на cheap VPS, который single-author может использовать. Не атакую, потому что artifact достаточно конкретен.
- Поведение `slug` крейта на title из 100% emoji: research digest даёт примеры, но edge-case «title ровно один emoji, который deunicode не знает» → empty slug → попадает в B-4, отдельно не атакую.
- Frontmatter parsing fidelity при manual edit с unusual YAML (multiline strings, anchors, alias): `gray_matter` write-back lossy — атакует A3 frontmatter convention, не этот ADR.
- Capture latency budget на slow disk: «<50 ms» — оптимистичное допущение, но пограничный случай не дотягивает до атаки на проектное решение.
