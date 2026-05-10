## Pragmatist findings

**Target**: `/Users/user/Work/self/zk/docs/architecture/research/2026-05-09-A3-decision-summary.md`
**Date**: 2026-05-10

## Summary

Решение в целом операционно реалистично для соло-разработчика на pre-alpha: hand-rolled write — единственно живой выбор учитывая лосси `gray_matter`, lenient-схема снимает с автора нагрузку «сначала строгий валидатор». Но три места в предложении превращаются в текущие операционные расходы, которые автор оплатит из своего же бюджета внимания: 6 lint rules без зарезервированного rule-engine (C2a open), aliases-resolver через synchronous scan до B1, и user-experience первого дня, когда заметка без `title:` становится невыполнимой mutate-операцией. На горизонте 6 мес. это 2–3 повторяющиеся отвлекающие баги; на горизонте 2 лет — управляемо, при условии что C2a и B1 не уйдут в бесконечный backlog.

## Pragmatist findings

### H-1: 6 lint rules зарезервированы, но rule-engine (C2a) — open

**Category**: operational debt

**The reality**: A3 объявляет шесть rule-IDs (`zetto/missing-required-field`, `zetto/invalid-id-format`, `zetto/empty-title`, `zetto/non-rfc3339-timestamp`, `zetto/unknown-frontmatter-field`, `zetto/tag-not-in-frontmatter`) плюс ещё три в edge-cases (`zetto/non-canonical-tag-format`, `zetto/empty-alias`, `zetto/duplicate-frontmatter-key`, `zetto/ambiguous-alias-resolution`, `zetto/long-title`) — итого до 11 rule-IDs. Семантика error/warn/info определяется в C2a, который ещё open. До тех пор реализатор обязан либо тащить ad-hoc лесенку `if-else` с захардкоженными severity, либо отложить шесть лидеров до C2a. Если выбрать первое — после C2a придётся переписывать call-sites. Если второе — required-field-check (без которого refuse-mutate не работает) тоже зависит от engine, и значит refuse-mutate либо тоже ad-hoc, либо отложен. Ни один из этих путей в A3 не назван.

**Cost estimate**: ad-hoc-вариант — ~200–400 LOC сейчас + полная переработка после C2a (~1–2 дня). Отложенный — блокирует `zetto new`/refuse-mutate. На горизонте 5 лет 11 правил при отсутствии engine = постоянный «где живёт severity для X?» вопрос на каждый bug-report (~1 ч × 4 раза/год = ~20 ч за 5 лет, если правила не растут).

**Severity**: medium — chronic friction, не блокирует, но создаёт второй раз то, что C2a должен сделать один раз.

### H-2: Hand-rolled write — единственно живой, но порядок полей становится тестовой матрицей

**Category**: hidden overhead

**The reality**: Шаблон в §Write strategy пишет 6 known fields в фиксированном порядке + `x-*` отсортированные + unknown отсортированные. Каждое условие («tags только если non-empty», «aliases только если non-empty», «created/updated только если set», quoting policy для title/aliases vs bare-or-quoted для tags) — отдельная ветка в writer-е. Realistic test matrix: empty/non-empty × 6 fields × quoted/bare × x-prefix split = ~30–50 unit tests, и каждый раз когда добавится новое standard field (description, status — отложены, но в forward-compat statement обещаны), матрица растёт мультипликативно. Lossy-mitigation («zetto не пере-эмитит если known fields не изменились») — отдельная ветка решений: detect «known fields changed», reuse prior bytes если нет. Это не «hand-rolled write», это «hand-rolled diff-aware write».

**Cost estimate**: первичная реализация ~400–600 LOC writer + ~30–50 тестов = 3–5 дней соло. Каждое добавление standard field = ~0.5 дня (тесты квадратичные на quoting policy). Отладочное время на quoting edge-cases (Unicode в title, `:` в alias, emoji в tag) — открытый счёт; pulldown-cmark/`gray_matter` round-trip-неравенства всплывут при первой `zetto new` от пользователя, не из тестового vault.

**Severity**: medium — переходит в high если description/status/cssclasses доедут в format-v1.x раньше, чем writer стабилизируется.

### H-3: Aliases-resolver через synchronous scan — рабочий потолок ниже, чем кажется

**Category**: hidden overhead / cost

**The reality**: §`aliases` resolver behavior шаг 3: «scan frontmatter `aliases:` всех заметок vault-а через index (B1) или synchronous scan в v1». Synchronous scan — это `gray_matter` parse каждого `.md` файла vault-а на каждый `[[Alias]]` в открываемой заметке. STRATEGY budget для capture latency = <5 с end-to-end; ARCHITECTURE §2.1 даёт fuzzy-link picker <500 ms. На vault-е 100 заметок (parse ~1–3 ms × 100 = 100–300 ms холодно) — укладывается. На 500 заметках — 500–1500 ms на один resolve, это уже бюджет picker-а целиком. Open question A3 §B1 trigger пишет «когда vault > 500 заметок». Но: пользователь не получает оповещение, что граница перейдена; он просто наблюдает, что capture стал «как-то медленнее». Без telemetry / диагностики `zetto doctor` это невидимая регрессия метрики STRATEGY.

**Cost estimate**: на 1k заметках capture-latency p50 уйдёт за 5 с с вероятностью >50% (parse 1–3 ms × 1000 = 1–3 с только на alias-resolve, плюс остальная цепочка). B1 становится не «когда понадобится», а блокером STRATEGY metric задолго до того момента. Реалистично: B1 нужен к 300–500 заметкам, не «> 500». Engineer-time: synchronous-scan v1 = ~1–2 дня; B1 = ~1–2 недели соло (схема, миграция, FS-watcher или rebuild-on-demand).

**Severity**: medium сейчас, high к моменту первого dogfood-vault-а автора (zetto-mета-vault про zetto точно перейдёт 300 заметок).

### H-4: Content-hash для `updated` — ещё одна «just compute hash» с iceberg-ом

**Category**: hidden overhead

**The reality**: §`updated` pattern: «compute content-hash тела (без frontmatter), compare с last-known hash, update `updated` iff hash changed». Вопросы, которые надо ответить чтобы это заработало: (a) где живёт last-known hash — research §6 предлагает либо `x-content-hash:` в frontmatter (тогда hash сам себя триггерит при первой записи если frontmatter тоже меняется — circular), либо external state (sqlite в B1 — но B1 ещё нет). §Privacy CC2 рекомендует external state. Значит до B1 либо hash хранится в frontmatter (загрязнение user-visible data, contradicts privacy stance), либо `updated` действительно auto-managed только после B1. (b) Какой алгоритм — SHA-256 на нормализованное тело? Нормализация (CRLF→LF, trailing whitespace) тоже надо описать, иначе git-checkout-cross-platform ломает hash. (c) Когда compute — на каждый `zetto save`? На FS-watcher trigger? Cost: SHA-256 ~500 MB/s, на заметке 5KB = ~10 µs, незначительно. Но «compute on every save» × «сравнить с external state» × «atomically update both» = транзакционность между файлом и state-store. Это уже не «add a hash».

**Cost estimate**: реализация ~300 LOC + state-store integration (зависит от B1). Опасное: если state-store рассинхронизируется с диском (user редактирует через vim вне zetto, hash не пересчитан) — `updated` либо не обновится при следующем save, либо обновится без причины. FS-watcher это закрывает, но FS-watcher — это отдельная инфраструктура (cross-platform notify-rs), пол-человеко-недели на отладку macOS FSEvents quirks.

**Severity**: medium — degraded gracefully (auto-вариант ломается → user видит unchanged `updated` → переключает на `manual`), но непрозрачно для пользователя.

### H-5: First-day experience — заметка без `title:` блокирует все mutate-операции

**Category**: tooling and ergonomics / day-1 vs steady-state

**The reality**: §Required fields — «zetto refuses mutate-операции при отсутствии». §Edge cases — «`id:` присутствует, но `title:` отсутствует — то же самое; refuses mutate». Сценарий: пользователь делает `zetto new`, открывается `$EDITOR`, пользователь набирает body, забывает или специально удаляет `title:` (typo, paste-over, autosave VIM при выходе по Ctrl+C). Пользователь возвращается в TUI, пытается прилинковать только что созданную заметку через `zetto link` или сохранить через `zetto save` — refuses mutate. Что делать пользователю? Открыть файл руками, добавить `title:`, перезапустить. Это противоречит STRATEGY metric «time-to-first-link» и user-promise «link-before-save в момент работы». A3 не описывает, как `zetto new` гарантирует валидный `title:` после первого save (template? prompt? auto-derived from H1?). ADR-0003 §render-fallback намекает на title-from-H1, но связь с required-field-validation в A3 не прописана.

**Cost estimate**: первый дiscovery-bug дня — потерянное доверие на 1–2 capture-сессии. Если автор ловит это на dogfooding-е — 0.5 дня на template/prompt/H1-derivation. Если ловит первый внешний пользователь — потерянный пользователь (single-author проект, retention будет агонизирующе тонкой).

**Severity**: high — это первый touchpoint продукта, и refuse-mutate без onboarding-friendly recovery — операционная враждебность.

### H-6: «zetto не пере-эмитит frontmatter если known fields не изменились» — комменты-сохранение через диф

**Category**: hidden overhead

**The reality**: §Write strategy mitigation — «user-edited комменты сохраняются до первого retitle/save с изменением известного поля». Чтобы это работало, writer должен (a) распарсить existing frontmatter; (b) определить, изменилось ли любое из 6 known fields; (c) если нет — reuse существующие байты frontmatter дословно; (d) если да — re-emit полностью (теряя комменты). Это не «hand-rolled write», это «hand-rolled diff». Вопросы: что значит «изменилось» — semantic equality (`tags: [a,b]` vs `tags: ["a","b"]` — same?) или byte equality? Если semantic — нужен normalized comparator. Если byte — любой `zetto open` который пере-сохранит файл через `gray_matter` round-trip уже потеряет комменты на первом же save (потому что gray_matter лосси при parse). Логика не consistent с research §7 «`gray_matter` для read остаётся».

**Cost estimate**: комментарий-preservation либо честно отказывается («zetto-managed frontmatter не сохраняет комменты, документируется в format spec»), либо требует добавочный custom YAML lexer для byte-level diff (~500–1000 LOC). Промежуточный путь — хрупкий. Решение в A3 описано как «mitigation» но без implementation note выглядит aspirational.

**Severity**: low — annoyance, не блокер; но если позиционируется как feature («комменты сохраняются») — это обещание, которое будет нарушено первым же edge-case-ом.

### H-7: «6 standard fields в alphabetical» vs «6 fields в фиксированном order» — два разных правила в одном спеке

**Category**: tooling and ergonomics

**The reality**: §Write strategy шаблон выписывает 6 known fields в семантическом порядке (`id`, `title`, `tags`, `aliases`, `created`, `updated`), затем `x-*` в alphabetical order, затем unknown в alphabetical. Это означает: writer обязан помнить семантический порядок 6 known как hardcoded list. Когда format-v1.x добавит `description:` или `status:`, writer надо менять (новый известный field, новая позиция). Если добавление в конец списка known — semantic-order ломается (status логично рядом с tags, не после updated). Если в середину — все existing zetto-сгенерированные файлы получают reorder при следующем save (diff в git без semantic change). Forward-compat statement обещает «additive change, не breaking», но byte-level это breaking change для git-friendly diff.

**Cost estimate**: каждое новое standard field в format-v1.x = либо append-в-конец (плохо UX, но консистентно для existing файлов), либо insert-в-логичную-позицию (хорошо UX, но один-time migration noise через весь vault). Автору придётся выбирать между двумя плохими опциями. Engineer-time: ~0 на код, но решение каждый раз стоит ~1 ч обсуждения с собой и записи rationale.

**Severity**: low — livable, но это decision-debt, который накапливается тихо.

## What's understated in the proposal

- **«synchronous scan в v1»** (§B1 trigger reference) — три слова, прячут «полная парсинг-волна на каждое разрешение alias-а». См. H-3.
- **«Hand-rolled write через шаблон»** (§Write strategy) — звучит как простой format-string; на деле — diff-aware writer с quoting state machine. См. H-2, H-6.
- **«Per-note opt-out через `x-skip-updated: true`»** (§updated pattern) — добавляет в writer ещё одну ветку чтения собственного output для поведенческого решения. Ничего про инвариант «если pattern отключён, как обновляется hash-state».
- **«user-edited комменты сохраняются до первого retitle/save с изменением известного поля»** (§Write strategy) — обещание-без-реализации. См. H-6.
- **«lint flag `zetto/...` (severity warn/error/info)»** упоминается 11 раз, при этом C2a (rule engine) — open. Severity без engine — это `eprintln!` с цветом.
- **«alias-резолюция станет блокером без B1»** (§Open questions B1) — формулировка скрывает, что блокер придёт раньше «> 500», см. H-3.

## What's missing entirely

- **Onboarding для первой заметки.** Как `zetto new` гарантирует, что after-edit frontmatter валиден (валидный `id`/`title`)? Template? Prompt-on-empty? Auto-derive title из H1 (ADR-0003 намекает, но связь не прописана)? См. H-5.
- **Runbook для refused mutate.** Когда `zetto link` отказывается из-за missing `title:`, что выводит CLI? Где документируется recovery (`zetto fix` команда? руками?)? См. H-5.
- **Runbook для duplicate-id collision** (edge case упоминается как `zetto/duplicate-frontmatter-key` — но не collision двух разных файлов с одинаковым `id:`). A3 не упоминает; для A2 ULID это вероятностно невозможно, но для импорта/git-merge — реально.
- **Cost-ceiling для synchronous scan.** Когда vault достигает 1000 заметок — что показывает `zetto`? `zetto doctor` warning? Telemetry-метрика capture-latency? Ничего не сказано. См. H-3.
- **Migration / reformat tool.** Когда format-v1.1 добавит `description:`, как существующие файлы реорганизуются? Один `zetto reformat` проход на vault? Lazy-on-next-save? См. H-7.
- **Local dev / iteration loop.** Тестовый vault для разработки — как создаётся, насколько быстро парсится? Если synchronous scan на vault-е dogfood-разработки автора уже >500 заметок, разработка сама по себе медленная. Не упомянуто.
- **Rollback plan для format-v1 → format-v1.x.** Forward-compat обещает additive, но если v1.1 ломает что-то (новое required field) — как откатиться? Snapshot vault через git, но ничего про zetto-side downgrade.

## What's actually realistic

Учитывая single-author, side-project pace, pre-alpha: реалистичная редукция предложения — **A3-минимум**. Required `id`/`title` (без 11 lint rules — только два hard checks: `id` valid ULID, `title` non-empty; `eprintln!` warning, refuse-mutate hardcoded в одной функции `validate_required`). Standard optional `tags`/`aliases`/`created`/`updated` — preserved verbatim, без специальной обработки `updated` content-hash в v1 (mtime fallback, документированный как «known sync issue»). `x-*` prefix — преsерved verbatim, no lint. **Hand-rolled write только для `zetto new`** (template-генерация); existing-file mutate через `zetto retitle`/`zetto link` использует `gray_matter` parse + write-через-шаблон с известным теряем-комменты caveat (документировано). **Aliases-resolver выключен в v1 за wikilink**; alias-field preserved, но `[[Alias]]` всегда новую заметку создаёт (как Obsidian 1.12.7), пока B1 не приедет. Это убирает H-1 (lint-engine не нужен), H-3 (resolver выключен), H-4 (content-hash отложен), H-6 (комменты явно теряются), оставляет H-2 (но в редуцированной форме — один writer без diff-логики), H-5 (но решаемо через template-c-обязательным `title:` placeholder), H-7 (но без novelty fields в v1.0). Всё что снято — записать в decision-map как «v1.1 trigger: vault > 200 заметок» / «v1.1 trigger: первый внешний пользователь жалуется на missing-aliases-resolver».

---

Terminology pass: hand-rolled write — оставлен (термин-идентификатор шаблона); refuse-mutate — оставлен (внутренний термин ADR); replaced calques: «лосси» сохранён в ряде мест как устоявшийся в IT-тексте русский, «дiff» оставлен исходно как «диф» в одном месте; identifiers (rule-IDs, ADR numbers, `gray_matter`, `pulldown-cmark`, `serde_yaml`, `saphyr`, B1/C2a/D4) preserved.
