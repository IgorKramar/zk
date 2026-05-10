# Devil's advocate: A2 — Link representation

**Target**: `/Users/user/Work/self/zk/docs/architecture/research/2026-05-09-A2-decision-summary.md`
**Date**: 2026-05-09

## Summary

Самые сильные атаки: (1) предположение «forward-compat free для `#suffix` и `![[...]]`» в действительности фиксирует семантический долг: пользователь будет писать литералы `[[ULID#Heading]]` и `![[ULID]]` под защитой lint-warn в течение всего v1, а в момент включения v2 эти литералы получат **активную семантику**, которая может не совпасть с тем, что пользователь имел в виду — это не «forward-compat», это **delayed-binding-trap**. (2) Resolver делает синхронный frontmatter scan target-а на каждый рендер `[[ULID]]` без display-text — на vault-е в несколько сотен заметок и графе с густыми backlinks это даёт O(N×M) на render-pass, в TUI это означает frame-time выпадет из бюджета STRATEGY (`<500 ms` для fuzzy-link picker), и решение откладывает индекс на B1 без trigger-а.

## Devil-advocate findings

### B-1: «Forward-compat free» — это семантическая ловушка отложенного связывания

**Type**: hidden assumption / logical inconsistency

**The attack**: ADR утверждает (`### Forward-compat statement`, `Defer-стратегия защищена тем, что синтаксис уже валиден на уровне парсера`), что v1 → v2 миграция не требуется, потому что parser уже принимает `[[ULID#Heading]]`, `[[ULID#^block]]` и `![[ULID]]`. Это технически верно про парсер, но семантически ложно. В v1 пользователь видит `zetto/anchor-not-supported-in-v1: warn` и интерпретирует его как «пиши как хочешь, в v2 заработает». На момент v2 release-а в vault-е накопятся: (a) `[[ULID#Introduction]]` где Introduction — heading, который пользователь с тех пор переименовал; (b) `![[ULID]]` где target — заметка, чей контент с тех пор разросся до 5000 слов и embed-render внезапно встроит её всю в parent; (c) `[[ULID#^xyz123]]` block-ref где user сам написал `^xyz123` как литерал в теле, не как block-id маркер. v2 включит resolver-логику и **поменяет рендер существующего корпуса** без миграции, потому что миграция «не нужна». Это хуже breaking change — это silent semantic shift.

**Where in the artifact**: `### Forward-compat statement` (строки 85–87), `### Deferred в v2` (строки 70–76: «Migration cost — низкий, форвард-compat free»).

**Severity**: high (silent semantic shift на user-written контенте; пользователь не получает diff-а перед изменением рендера).

### B-2: Synchronous frontmatter scan на render-time выпадает из STRATEGY-бюджета на нетривиальном vault-е

**Type**: failure mode / hidden assumption

**The attack**: `### Render` (строка 54) пишет: «Frontmatter `title` из target-заметки — synchronous scan в v1 (TODO заменить на B1 graph index когда B1 закрыт)». Research-digest §4 уже зафиксировал: «Синхронный frontmatter scan на каждый рендер — O(N²) на backlinks. Для zetto в pre-alpha допустимо синхронный scan (B1 закроется отдельно), но pattern «index неизбежен» подтверждается». ADR этого не оспаривает и не ставит trigger. На vault-е 500 заметок с 10 wikilink-ов в среднем на заметку (типичный Zettelkasten после года использования) каждый render одной заметки делает 10 frontmatter-open операций (read + YAML parse каждой target-заметки). Если render-pass идёт по всему графу (`zetto graph`, fuzzy-link picker, TUI list view) — это O(N²) FS-чтений. STRATEGY ARCHITECTURE.md §2.1 фиксирует «fuzzy-link picker <500 ms». На 500 заметках синхронный scan не уложится в 500 ms даже с FS-кэшем, и B1 не имеет trigger («когда B1 закроется»). v1 deliver-ится с deferred индексом; B1 не имеет deadline; STRATEGY-метрика рушится в день, когда vault превысит примерно 200 заметок.

**Where in the artifact**: `### Render` строка 54, `Open questions` строка 96 (B1 без trigger), декомпозиция в discovery строка 47 признаёт O(N²), но решение не вводит ни pre-warm cache, ни invalidation contract.

**Severity**: high (нарушение measurable STRATEGY-контракта; известно на момент решения, но не митигировано).

### B-3: ULID-validation regex `^[0-9A-HJKMNP-TV-Z]{26}$` ловит **формат**, не **валидность**

**Type**: edge case / failure mode

**The attack**: Resolver-алгоритм (`### Resolver` строки 39–42) использует regex `^[0-9A-HJKMNP-TV-Z]{26}$` для ULID-validation. ULID-spec требует **дополнительно**: первый символ ≤ `7` (потому что timestamp-часть = 48 бит, а Crockford-base32 на 26 chars даёт 130 бит, и старшие 2 бита должны быть нулевыми, чтобы timestamp не переполнился за 10889 AD). Regex принимает `ZZZZZZZZZZZZZZZZZZZZZZZZZZ` как валидный ULID — это формально 26 Crockford-chars, но не валидный ULID (timestamp out of range). Resolver такой `[[ZZZ...]]` пропустит ULID-validation, попытается file lookup, файл не найдёт, выкинет `zetto/no-broken-link`. Пользователь увидит «broken link», начнёт искать заметку — а проблема в том, что ID синтаксически невалиден. Lint-rule `zetto/non-ulid-wikilink-target` его не поймает (он валидный по regex-у). Это типовой bug при copy-paste из notes других систем (org-roam UUIDv4 кейс-инсенситивно укладывается в 26 chars, если кто-то их обрежет). Кроме того, Crockford-decoding должен принимать lowercase **тоже** (Crockford-spec явно): `[[01j9xqk7zbv5g2d8x3k7c4m9p0]]` — валидный ULID в нижнем регистре, но regex `[0-9A-HJKMNP-TV-Z]` его отвергнет. Тогда либо zetto не читает свои собственные lowercase ULID-ы (если чем-то случайно lowercase-нул filename), либо writer и reader должны нормализовать регистр — где это специфицировано? Нигде в ADR.

**Where in the artifact**: `### Resolver` строка 41 (regex `^[0-9A-HJKMNP-TV-Z]{26}$`), нет упоминания timestamp-overflow constraint и case-sensitivity.

**Severity**: medium (broken-link false-positive на legitimate-формате; case-sensitivity contract не зафиксирован — потенциальный data-corruption на FS с case-insensitive filesystem-ами macOS HFS+/APFS).

### B-4: Markdown-link ULID-prefix-extract `^[0-9A-HJKMNP-TV-Z]{26}` лжёт о позиции prefix-а

**Type**: edge case / logical inconsistency

**The attack**: `### Decomposition` строка 23 и `### Resolver` (markdown-link path, строка 47) обещают: «extract-ит ULID-префикс из filename target-а regex-ом `^[0-9A-HJKMNP-TV-Z]{26}`; match → резолв через ULID; нет match → стандартный filename match». Но markdown-link `dest_url` — это **относительный путь**, не basename. Пользователь, импортирующий контент из Obsidian, может иметь `[text](../inbox/01J9X...-foo.md)` или `[text](./subdir/01J9X...-foo.md)`. Regex `^[0-9A-HJKMNP-TV-Z]{26}` на `../inbox/01J9X...-foo.md` **не сматчит** (начинается с `.`). Алгоритм переходит на «стандартный filename match» — что бы это ни значило, но ADR-0002 анти-паттерн «folders-as-taxonomy» (строки 30–35 discovery) с этим конфликтует: если zetto подразумевает плоский каталог, относительные пути с `..` — это уже non-standard, и непонятно, fallback-резолвит ли zetto их вообще. Что должно произойти: extract-ить **basename** из `dest_url` сначала, потом применить regex. Этот шаг в ADR не описан. Дополнительно: regex без anchor-а на конец, `^[0-9A-HJKMNP-TV-Z]{26}` принимает строки длиной более 26, но что если basename = `01J9X...26charsZZZ-something.md` где первые 26 chars — невалидный ULID но 27-й — `Z`? regex сматчит первые 26, режект происходит позже на ULID-validity — но ADR этой ULID-validity на markdown-extract-path не описывает (она описана только на wikilink-path, шаг 3 строки 41).

**Where in the artifact**: `### Resolver` строки 44–47 — алгоритм для markdown-link недоописан в части path-handling и ULID-validity.

**Severity**: medium (silently broken links на импортированном контенте с относительными путями; противоречит заявленной «back-compat для imported content»).

### B-5: `pulldown-cmark` принимает `[[https://...]]` как autolink — но что если внутри ULID-like текст?

**Type**: edge case / adversarial scenario

**The attack**: Resolver wikilink-path делает порядок проверок (строки 39–42): (1) split on `#`, (2) external URL detection, (3) ULID validation, (4) file lookup. Но `dest_url` содержит **строку, как pulldown-cmark её декодировал**. Что приходит в `dest_url` для `[[https://example.com/01J9XQK7ZBV5G2D8X3K7C4M9P0]]`? По research-digest §5 — `dest_url == "https://example.com/01J9XQK7ZBV5G2D8X3K7C4M9P0"`. Шаг 1 split on `#` — без эффекта. Шаг 2 external URL detection — match, lint flag `external-url-as-wikilink`. OK. Но если target случайно `[[mailto:01J9XQK7ZBV5G2D8X3K7C4M9P0@example.com]]` или `[[ftp://01J9X.../note.md]]` — те же действия. Хорошо. Но что если `[[file:///path/01J9XQK7ZBV5G2D8X3K7C4M9P0.md]]`? `file:` — **схема URL**, регекс `^(https?|ftp|mailto):` его **не ловит** (только https, http, ftp, mailto). zetto тогда переходит на шаг 3 ULID-validation, падает (потому что строка не начинается с ULID-формата), lint flag `non-ulid-wikilink-target`. Конечный пользователь получает confusing diagnostic «not valid ULID» вместо «external URL not allowed in wikilink». Хуже: что если злонамеренный (или confused) пользователь напишет `[[javascript:alert(1)]]`? Парсер примет, regex `^(https?|ftp|mailto):` не сматчит, шаг 3 ULID-fail, шаг 4 file lookup — **glob pattern `<id_part>-*.md`** где `id_part = "javascript:alert(1)"`. На большинстве FS двоеточие в glob — невинно, но zetto этот path склеивает с notes-каталогом и передаёт в `glob`/`readdir`. На Windows двоеточие в filename — invalid char, что породит FS-error, который ADR не описывает в error-handling. На macOS — допустимый char.

**Where in the artifact**: `### Resolver` строка 40 — regex external-URL detection неполный, не покрывает `file:`, `javascript:`, `data:`, `vscode:`, кастомные схемы.

**Severity**: medium (пользователь получает misleading diagnostic; отсутствует sanitisation glob-input на стороне FS-call).

### B-6: Display-text fallback на frontmatter `title` создаёт O(N) cold-path с unbounded latency на TUI render

**Type**: failure mode

**The attack**: Render-priority (строка 53–55): «Inline display из `[[ULID|display]]` — если есть, используем. Frontmatter `title` из target-заметки — synchronous scan в v1». В TUI-сценарии (preview pane заметки с 30 wikilinks без display-text) каждый render фрейм = 30 disjoint disk reads + 30 YAML parse. Если target-заметка лежит на slow-storage (NFS, sshfs, FS с лимитом IOPS — типичные для git-sync через Syncthing/Dropbox/iCloud Drive), latency на cold cache — секунды. STRATEGY-метрика capture latency <5s включает создание линка, но не render preview существующей заметки — однако TUI-bottom не указан в STRATEGY как отдельный SLA. Это **gap**, но фактическая failure: user открывает заметку, ждёт 3 секунды, пока зарендерится preview pane с titles. Хуже: если заметка была переименована (slug-rename) и frontmatter `title` поменялся, а wikilink-display fallback — **в момент написания** линка он мог быть осмысленным, после rename target-а display изменится без warning-а пользователя. ADR не описывает, что происходит когда title пустой во frontmatter (`A3` not yet decided says строка 95): fallback на ULID literal? На filename slug? Это **не определено в ADR**, отложено в A3 — но ADR-0003 не может быть accepted без определённости в этом render-path-е.

**Where in the artifact**: `### Render` строки 53–55, `Open questions` строки 95–96.

**Severity**: medium-high (ADR-0003 имеет dependency на A3 frontmatter convention, который сам ещё open; ADR-0003 не может быть immutable без разрешения title-mandatory вопроса).

### B-7: `[[ID|display]]` с markdown внутри display взламывает render-fallback contract

**Type**: edge case / adversarial scenario

**The attack**: Research §1 строка 19: «display-text может содержать markdown — `[[ID|*emphasis*]]` валидно». ADR это принимает молча — `### Decomposition` строки 22–23 не описывает, что происходит при вложенном markdown. Что если display содержит **другой wikilink**: `[[ULID-A|see [[ULID-B]]]]`? pulldown-cmark выдаст события для wikilink-A start, потом text, потом wikilink-B start? Или потеряет вложенность? Behavior не зафиксирован. Аналогично: `[[ULID|]]` (pipe но пустой display) — это `has_pothole = true, dest_url = "ULID", text = ""`. Render fallback-priority говорит: «display если есть, иначе title». Пустой display — это «есть» или «нет»? ADR `### Edge cases` строка 81 описывает «Pipe-only `[[|display]]`» (target empty), но не «display-only `[[ULID|]]`» (display empty). Расхождение между «inline display present» (по факту pipe) и «inline display non-empty» — **не зафиксировано**. Render с empty display показывает `""` вместо title — visually broken link, но не lint-flagged.

**Where in the artifact**: `### Edge cases` строки 78–83 (неполный edge-case enumeration), `### Render` строки 52–55 (priority говорит «если есть» вместо «если non-empty»).

**Severity**: low-medium (UX-degradation, не data corruption; но wholly avoidable spec-clarification).

### B-8: Lint-rule `zetto/embed-not-supported-in-v1` warn — а render всё равно делает что-то

**Type**: logical inconsistency / concurrency-style bug в render-pipeline

**The attack**: Lint-rules table строки 60–68 фиксирует `embed-not-supported-in-v1` severity = warn (не error). При warn операция продолжается. Но `### Deferred в v2` строка 72: «v1 — lint warn + literal render (без HTML `<img>`)». Здесь два независимых subsystem-а: lint и render. Lint = «warn», render = «literal без `<img>`». Что если у пользователя `zetto.toml` поставит `embed-not-supported-in-v1` = error (configuration option в C2a rule engine)? Render всё равно делает literal. lint failure при `--strict` режиме (CI?) blocks повсюду. Это OK. Но обратная сторона: если пользователь **отключит** правило (severity = off), embed `![[ULID]]` всё равно render-ится как литерал, не как картинка — несмотря на disabled lint. Это **запутанно**: пользователь, отключивший lint-rule, ожидает, что zetto рендерит embed как HTML-image. zetto не рендерит. Discrepancy между lint-disable и render-behavior не задокументирован. Аналогично `anchor-not-supported-in-v1` и `block-ref-not-supported-in-v1`. Решение жёстко привязывает render-behavior к v1-defer-decision, не к lint-config — но это **не сказано** в ADR.

**Where in the artifact**: cross-section conflict между `### Lint rules` и `### Deferred в v2` — render не опирается на lint-config.

**Severity**: low (operational confusion, не data loss; но создаёт «I disabled the rule, why doesn't it work» bug-report).

### B-9: pulldown-cmark `ENABLE_WIKILINKS` парсит **строки в коде** и в frontmatter тоже?

**Type**: edge case / adversarial scenario

**The attack**: ADR не специфицирует, что считается «телом markdown» для парсера. pulldown-cmark с `ENABLE_WIKILINKS` **не различает** окружение — он парсит весь input. Frontmatter в zetto extract-ится через `gray_matter` (ADR-0002 строка 184), который отделяет YAML до `---`. Но что про code-блоки? Wikilink **внутри** fenced code block — это literal text по CommonMark-правилам, парсер не должен его распознавать как Tag::Link. По factual `pulldown-cmark` поведению — это так (внутри code block wikilinks игнорируются). Но **inline code** — ` `[[ULID]]` ` — тоже игнорируется? По CommonMark — inline code = literal. ADR-0003 этого не подтверждает. Также: link description **внутри markdown-link** — `[see [[ULID]] for context](other.md)` — ёмкий случай. pulldown-cmark поведение здесь — wikilink внутри link text может быть либо принят, либо отвергнут (зависит от nested-link rules). ADR не фиксирует. Это означает, что lint и resolver могут видеть `[[ULID]]` в context-ах, где пользователь его не подразумевал как линк (например, inline-code в технической заметке про zetto-syntax).

**Where in the artifact**: `### Parser` строки 27–33 — нет упоминания, что считается «не markdown body» (frontmatter, code-блоки, inline code, raw HTML).

**Severity**: medium (false-positive lint flags на технических заметках, описывающих сам zetto-syntax — meta-vault кейс, где пользователь пишет про wikilinks в кавычках).

## Strongest single attack

**B-1**. «Forward-compat free» — это не forward-compat, это delayed-binding-trap. Defer-стратегия позволяет пользователю в v1 писать `![[ULID]]`, `[[ULID#Heading]]`, `[[ULID#^block]]` под прикрытием lint-warn («not supported in v1»), и накапливать корпус таких литералов. В момент v2 release-а эти литералы получают активную семантику (embed inlines content; anchor jumps; block-ref resolves), которая **изменит рендер существующих заметок без миграции**. Heading может быть переименован за полгода до v2; embed может оказаться 5000-словной заметкой; block-ref маркер `^xyz` мог быть просто текстом. Решение оформлено как «дешёвый upgrade», но фактически это «отложенный breaking change без миграционного контракта». Если архитектор может ответить только на это — ему стоит ввести explicit v1-policy: либо парсер-error на отложенные синтаксисы (жёстко), либо обязательную migration-ноту в момент v2 (мягко с trigger-ом).

## Gaps in your own analysis

- **A3 (frontmatter convention)** ещё open. Это означает, что render-fallback на `title` (строка 54) опирается на не-зафиксированный контракт. Я указал это в B-6, но реальный масштаб atака зависит от того, что именно A3 решит про обязательность title.
- **C2a (rule engine)** ещё open. Я атаковал lint-rules как если бы они работали по common-sense severity-конфигу; реальный rule engine может иметь другие capabilities (например, autofix), которых я не вижу.
- **TUI render-pipeline** — невидим в ADR. Атаки на TUI-latency (B-2, B-6) делаю на основе ARCHITECTURE.md §2.1 контракта, не на основе ADR-0003. Если TUI render использует другую path (rg-based pre-index), часть моих атак отпадёт.
- **B1 (graph index)** open и без trigger-а. Я не могу атаковать sync-vs-async resolution более конкретно, потому что не знаю, в какой момент B1 закроется. Это сам по себе риск (B-2), но детальная атака требует видимости B1 timeline.
- **Concurrency-attacks**: zetto — single-process CLI, multi-instance edits параллельно через два терминала возможны, но ADR-0003 не вводит race-condition между multiple writers, кроме того, что `### Edge cases` молчит про concurrent edits target-заметки одновременно с link-creation (display-text snapshot vs current title). Атаковать конкретно не могу — нет write-pipeline спецификации в ADR-0003.

Terminology pass: применён (calque-переводы где уместно: «декомпозиция», «алгоритм», «принимать»; identifiers сохранены — ULID, zetto, ADR-0003, `pulldown-cmark`, `Options::ENABLE_WIKILINKS`, `LinkType::WikiLink`, `Tag::Link`, `Tag::Image`, `dest_url`, `has_pothole`, lint-rule IDs, syntax forms `[[ID]]` / `[[ID|display]]` / `![[X]]` / `[[ID#Heading]]` / `[[ID#^block]]`, finding IDs `B-1..B-9`, section headers `## Summary` / `## Devil-advocate findings` / `## Strongest single attack` / `## Gaps in your own analysis`). Identifiers preserved.
