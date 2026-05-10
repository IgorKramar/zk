## Junior-engineer findings

- **Target**: `docs/architecture/research/2026-05-09-A3-decision-summary.md`
- **Date**: 2026-05-10
- **Role**: junior-engineer (clarity)
- **Reading posture**: впервые читаю проект. Под рукой `STRATEGY.md`, `ARCHITECTURE.md`, `decision-map.md`, ADR-0001/0002/0003. Не читал A3 discovery/research/design.

## Summary

Документ структурно аккуратен, но систематически зависит от трёх внешних источников, которых у читателя нет: A3 discovery («7 leans приняты»), A3 research digest («§1», «§6», «§7») и A3 design («Альтернатива A», «B-альтернатива»). Документ ссылается через короткие хвосты «per research §6» без пересказа цитируемой логики. Самый болезненный пробел: «hand-rolled write» стратегия и `updated:` content-hash паттерн обоснованы только ссылкой на research §7 / §6 — фактическое рассуждение «почему именно так» в этот документ не попало.

## Clarity findings

### J-1: «Альтернатива A», «B-альтернатива», «7 leans приняты» — отсылки в шапке, которые не разворачиваются нигде в документе

**Category**: broken cross-reference / erased reasoning

**The gap**: Шапка: «Inputs: discovery (7 leans приняты), research digest, design (Альтернатива A выбрана)». Дальше: «отложены в B-альтернативу». Какая «Альтернатива A»? Какие «7 leans»? Какая «B-альтернатива» и чем отличалась?

**Suggested fix**: 1–2 предложения в начале «Декомпозиции»: «Альтернатива A = YAML-frontmatter с двумя обязательными + четырьмя стандартными опциональными; Альтернатива B = более широкий required-set (description/status/cssclasses)». Добавить «Reviews trail» с путями — как в ADR-0002/0003.

### J-2: «research §1», «research §6», «research §7» — параграфы, которых читатель не видит

**Category**: broken cross-reference

**The gap**: «Caveat (per research §1)», «Pattern (per research §6)», «Write strategy — hand-rolled fixed order (per research §7)». Каждая из этих ссылок несёт нетривиальное утверждение (Obsidian регресс, content-hash паттерн, lossy round-trip).

**Suggested fix**: Либо вшить полный путь к research digest рядом с первой ссылкой, либо в одну фразу пересказать вывод цитируемого раздела прямо в documenta.

### J-3: «D4=read-write через Obsidian native» — что такое D4 и его варианты

**Category**: undefined term

**The gap**: «D4=read-write через Obsidian native не работает out-of-the-box». Что D4 формально, какие там варианты целиком и в каком документе они впервые перечислены — не сказано ни в одном ADR.

**Suggested fix**: Добавить одно предложение при первом упоминании D4: «D4 = Obsidian-vault compatibility posture (см. `decision-map.md`). Варианты: own format / read-only / read-write через aliases / strict-checker (исключён в ADR-0002).»

### J-4: «C2a (rule engine)» — повторяющийся шибболет, который не разворачивается

**Category**: undefined term / unstated assumption

**The gap**: Документ многократно резервирует «6 lint rules имена резервированы здесь, semantics в C2a» и упоминает «C2a (rule engine architecture, open)». Что C2a, что preset, что значит severity — нужно открывать decision-map.md.

**Suggested fix**: Либо в шапке указать «C2a = Methodology rule engine architecture (см. `decision-map.md`, open)», либо опустить упоминания C2a из мест, где не несёт смысла, оставив только в § «Open questions».

### J-5: `recommended-luhmann` и `lenient` — это пресеты чего? Кто их выбирает?

**Category**: undefined term / unstated assumption

**The gap**: «severity warn в `recommended-luhmann` preset, off в `lenient` preset». Где конфигурируется (CLI-флаг? `~/.zetto`/`zettorc`?), какой default? Откуда имя «luhmann»?

**Suggested fix**: При первом использовании уточнить: «`recommended-luhmann` и `lenient` — два встроенных preset-а lint-rule severity (см. C2a; default — `recommended-luhmann`); пользователь переключает через `.zettorc` поле `lint.preset`».

### J-6: «B1» как зависимость alias-резолвера — что это

**Category**: undefined term / unstated assumption

**The gap**: «scan frontmatter `aliases:` всех заметок vault-а через index (B1) или synchronous scan в v1». «B1 (graph index)» — без раскрытия. В A3 ни разу не разворачивается.

**Suggested fix**: При первом использовании в скобках: «B1 = graph index strategy (см. ADR-0003 §B1 trigger; `decision-map.md`, open)».

### J-7: Поле `format:` упомянуто, но «format-v1» / «format-v2» — что это формально

**Category**: erased reasoning / unstated assumption

**The gap**: «A3 расширяет format-v1 spec contract additive way». «`format: 1` — НЕ требуется (per [ADR-0002 § Format versioning anchor]). format-v1 — implicit». Полная спецификация format-v1 не существует на момент чтения.

**Suggested fix**: Одно предложение в начале §«Forward-compat»: «format-v1 spec — будет зафиксирован в A5; A3 описывает frontmatter-часть. Additive change = добавление нового optional поля без удаления/переинтерпретации существующих.»

### J-8: «schema lenient» — что такое схема и каков её default

**Category**: hidden boundary

**The gap**: В одной фразе: «схема lenient (preserve unknown verbatim, lint warn для unprefixed unknown)». В декомпозиции: «Schema strictness — lenient». Это название режима? Default? Один из нескольких?

**Suggested fix**: «Schema strictness — это поведение парсера на нестандартные поля. zetto v1 поддерживает один режим — lenient (название совпадает с lint-preset, но это разные настройки). Strict-режим (reject unknown) не реализован в v1.»

### J-9: Code-snippet `Frontmatter` struct — `extra: BTreeMap`, `serde(flatten)` — обоснование «зачем» опущено

**Category**: erased reasoning

**The gap**: В § Validation pipeline дан Rust-snippet с `#[serde(flatten)] extra: BTreeMap<String, serde_yaml::Value>`. Связь read→write через `extra` map нигде не сформулирована.

**Suggested fix**: Один комментарий в snippet: «`BTreeMap` обеспечивает alphabetical iteration на write (см. Write strategy); `serde_yaml::Value` сохраняет original YAML AST поля для verbatim render.»

### J-10: «zetto не пере-эмитит frontmatter если zetto-known fields не изменились» — что считается «изменением»?

**Category**: unfollowable instruction / hidden boundary

**The gap**: «zetto не пере-эмитит frontmatter если zetto-known fields не изменились». Какая операция zetto это контролирует? Пользователь правит файл в vim; zetto не наблюдает edit. Где хранится last-known?

**Suggested fix**: Одно предложение: «zetto на каждом запуске сравнивает текущие значения известных полей с последней версией в external state (sqlite в B1; до B1 — write-pass вызывается только при mutate-операциях zetto-команд, не на FS-watcher событиях).»

### J-11: Edge case `tags: tag1` (scalar) — «`gray_matter` обычно coerce-ит» — насколько твёрдо «обычно»?

**Category**: unstated assumption

**The gap**: «`gray_matter` обычно coerce-ит scalar to single-element list». Слово «обычно» в архитектурном документе означает: автор не уверен или поведение зависит от опции.

**Suggested fix**: Заменить «обычно» на конкретику: «`gray_matter` (через `serde_yaml`) coerce-ит scalar в `Vec<String>` через `#[serde(default, deserialize_with = …)]` custom helper; на failure — lint error `zetto/invalid-tags-format`».

### J-12: «Inline `#tag` в body — recognized at read для derived view» — какая derived view?

**Category**: undefined term

**The gap**: «recognized at read для derived view (lint surface для C2a `zetto/tag-not-in-frontmatter`)». «Derived view» — это в zetto такая концепция? Это TUI-view, CLI-команда `zetto tags`?

**Suggested fix**: «Inline `#tag` в body распознаётся при чтении и попадает в derived `zetto tags`-output (CLI-команда), но не записывается обратно в `tags:` array заметки.»

## What's well-documented

- **§Edge cases** — конкретные YAML-кейсы и ожидаемое поведение перечислены систематически.
- **§Lint rules-таблица** — `Rule ID / Description / Default severity` дают полный поверхностный обзор. Имена префиксированы `zetto/`.
- **Шаблон write-strategy** между `---` ограничителями — буквальный шаблон файла.
- **§Privacy and security considerations** — каждый bullet идентифицирует конкретный leak-channel.
- **§Open questions** — каждый отложенный вопрос имеет адресата (A4, A5, B1, C2a, D4).

## Where I gave up

- **`updated:` content-hash логика в pre-B1**. Где хранится last-known hash? «Не во frontmatter» (per Privacy §) — но B1 ещё не существует. Тогда v1 этой логики не имеет вообще?
- **§Validation pipeline шаги 4–5 vs §Schema strictness** — оба раздела описывают reaction на «invalid format» и «unknown field», но разными словами и в разном порядке.
- **`aliases` resolver pass 3 «через index (B1) или synchronous scan в v1»** — какой именно «synchronous scan» в pre-B1? Чтение каждой `.md` на каждом `[[X]]` lookup? Это O(N) на link, что для render-fallback ADR-0003 уже было названо триггером B1.
