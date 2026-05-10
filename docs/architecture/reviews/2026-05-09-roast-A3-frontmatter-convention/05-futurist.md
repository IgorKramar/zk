## Futurist findings

- **Target**: `docs/architecture/research/2026-05-09-A3-decision-summary.md`
- **Date**: 2026-05-10
- **Role**: futurist (long-horizon)
- **Horizon**: 1 to 3 years (до 2028)
- **Confidence note**: Findings под Structural — high-confidence (структурные неизбежности при росте). Findings под Trend — speculative, опираются на сигналы из экосистемы, явно помечены.

## Summary

A3 фиксирует разумный, additively-расширяемый контракт для format-v1, но три из его допущений приобретают долговременный вес: (1) `aliases:`-резолвер фактически становится постоянным zetto-side обязательством, не временной мерой против бага Obsidian 1.12.7; (2) `x-*` namespace, не имея PKM-прецедента, превращается из расширяемой ниши в «объясняем на каждом онбординге» pattern; (3) defer-list (description/status/type/prev/next/cssclasses) при темпах личного проекта почти наверняка остаётся «forever-deferred», и format-v1 застывает в текущем виде. Не катастрофы — структурные дрейфы, которые лучше зафиксировать сознательно сейчас.

## Structural findings (high-confidence)

### F-1: `aliases:`-резолвер становится канонической, а не временной мерой

**Type**: inertia / adjacent decisions
**Horizon**: 1–2 года

**The drift**: A3 описывает zetto-side alias-резолвер как обходной путь под D4=read-write на фоне регресса Obsidian 1.12.7. Но как только zetto начинает поддерживать собственный резолвер с case-insensitive lookup, alias-collision lint, ULID-creation-time tiebreak и интеграцией с B1-индексом — это становится отдельной поверхностью с собственной семантикой. Даже если Obsidian починит свой резолвер в 2027, к тому моменту zetto уже будет иметь свой contract (правила нормализации, поведение при коллизии, кэш-инвалидацию). Удалять его дороже, чем содержать; семантические расхождения создадут D4-edge-cases.

**Mitigation**: явно зафиксировать в ADR-0004, что alias-резолвер zetto — **постоянная** часть on-disk contract, не временная заглушка под Obsidian-баг.

### F-2: `x-*` namespace без PKM-прецедента приобретает «онбординг-налог»

**Type**: codebase aging / team
**Horizon**: 6–18 месяцев

**The drift**: A3 заимствует `x-` prefix из OpenAPI/JSON Schema, явно отмечая что в PKM нет прецедента. Каждый новый пользователь увидит `x-skip-updated`, `x-content-hash` и спросит «что это». В zetto-документации придётся постоянно объяснять. Через 1 год укореняется как часть identity, но в первые 6–12 месяцев — поверхность для путаницы.

**Mitigation**: добавить «rationale block» с 1–2 контр-альтернативами (`zetto:`-prefix, `_`-prefix, vendor-extensions без префикса) — чтобы будущие обсуждения не переоткрывались с нуля.

### F-3: Defer-list для description/status/type/prev/next/cssclasses становится «forever-deferred»

**Type**: codebase aging / inertia
**Horizon**: 2–3 года

**The drift**: A3 откладывает 6 additive полей до «появления конкретного use case». В личном проекте за 2 года эти use cases скорее всего не появятся явно — они проявятся как ad-hoc применение `x-*` для тех же целей (пользователь напишет `x-status: draft`). Когда A3 будет ревизироваться в 2028, окажется что де-факто пользователи уже использовали `x-status`, и продвигать в standard set — миграционная работа + breaking change в семантике.

**Mitigation**: один из двух — (a) явные триггеры promotion («3 случая `x-status` в реальном vault — promote»), или (b) явное «defer = non-goal до format-v2».

### F-4: Hand-rolled writer становится structural anchor, дороже всего format-v1

**Type**: inertia
**Horizon**: 1–2 года

**The drift**: hand-rolled writer с фиксированным порядком полей и собственной quoting policy через 1 год накопит edge cases (CRLF vs LF, BOM, Unicode, multi-line YAML). К 2 годам становится самым stable, но и самым «нельзя трогать»: любое изменение — diff во всех заметках. Если A4 или A5 потребуют менять схему — переписывать writer страшнее, чем переписывать парсер.

**Mitigation**: добавить test-обязательный invariant: «для любой valid v1 заметки `read → write` идемпотентен (byte-for-byte для известных полей)». Дешёвый guard, делает inertia осознанной.

### F-5: `updated:` content-hash паттерн ломается на batch-операциях

**Type**: scale shifts
**Horizon**: при первой batch-операции

**The drift**: Per-нота content-hash compare работает идеально для интерактивного потока. Ломается семантически в трёх сценариях: (1) batch-rename при смене ID-схемы — content тела не меняется, но `updated:` должен ли обновляться? (2) global tag reformatting — frontmatter changed, content не changed, hash тот же → `updated:` НЕ обновляется → пользователь не знает что тег теперь другой; (3) format-v1.x → format-v1.y migration tool — все заметки переписаны, content-hash скорее всего не меняется. К 2 годам появится хотя бы один такой сценарий.

**Mitigation**: одна строка — «batch-операции, инициированные zetto-tool (миграции, mass-rename, reformat), пропускают auto-update `updated:` или явно его обновляют через флаг».

### F-6: Lint-rule namespace `zetto/*` блокирует будущий plugin host

**Type**: adjacent decisions
**Horizon**: 2–3 года, conditional on plugin host (deferred per decision-map)

**The drift**: A3 резервирует 6 правил под `zetto/*`. Когда plugin host реально появится, его рамка вынуждена либо переопределить «`zetto/*`» (сейчас «built-in»), либо ввести параллельный namespace (`@user/*`?). Маленькое решение, но создаёт неявную констрейнту на C2a.

**Mitigation**: одна строка — «`zetto/*` reserved for built-in rules; future third-party rule namespacing — в C2a». Явное отложенное решение.

## Trend findings (speculative)

### F-7: `gray_matter` Rust crate — vendor risk medium

**Type**: technology lifecycle / vendor risk
**Confidence**: medium

**Signals**: основной maintained fork — yuchanns/gray-matter-rs; последний значимый refresh — июль 2025; альтернативы (`yaml-front-matter`, `rust-frontmatter`) явно неподдерживаемые (последние коммиты 2021); `serde_yaml` deprecated архивирован в 2024, форки разрозненны.

**The drift**: к 2027–2028 либо `gray_matter` сохранит статус-кво, либо community сместится на новый де-факто crate. zetto-write hand-rolled (per A3) — правильный hedge, изолирует write path. zetto-read остаётся exposed: при breaking change в `gray_matter` zetto-read должен будет либо мигрировать, либо принять fork.

### F-8: «теги-как-facets, не link-replacement» — мейнстрим в 2027–2028

**Type**: idiom shift
**Confidence**: low

**Signals**: за 2024–2026 в Obsidian/Logseq community сдвиг — посты «stop using tags as folders», популярность Properties view, общий нарратив «tags ≠ structure». Тренд **поддерживает** позицию zetto.

**The drift**: к 2028 «tags as facets» позиция zetto более нормализована. Этот аспект A3 опережает тренд — позитивный finding.

### F-9: Obsidian 1.12.7 регресс резолвера alias может не быть починен в 2026 году

**Type**: vendor risk / regulatory drift
**Confidence**: medium

**Signals**: bug-репорт открыт ~май 2026; Obsidian closed-source; история past alias bugs (2023, 2025) показывает, что резолвер aliases — known fragile area. Нет канала влияния со стороны zetto.

**The drift**: даже если 1.12.7 починят в 2026, гарантии что в 1.13.x не будет нового регресса нет. Аргумент за F-1 (резолвер становится permanent), не временная мера.

## What's likely to age well

- **Two-required-field minimum (`id` + `title`)** — минимально-возможный обязательный контракт. По принципу «smallest forced commitment ages best».
- **Lenient strictness + lint warnings** — идиоматический подход 2025–2026 (Rust ecosystem: rustfmt/clippy warn-by-default). Эта позиция становится более mainstream.
- **Hand-rolled write strategy** — изолирует zetto от vendor risk на write path (см. F-7).
- **Additive minor format-v1.x bump policy** — стандарт semver применённый к данным.
- **`tags:` as facets stance** — см. F-8, тренд поддерживает позицию.
- **Forward-compat anchor (no `format:` field в v1)** — наследует from ADR-0002, работает.

## What's worth deciding now to defer pain

Каждое — одна-две строки в ADR-0004, не пере-дизайн.

- **F-1**: «zetto-side alias resolver — постоянная часть on-disk contract, независимая от состояния Obsidian-резолвера».
- **F-2**: «rationale block» с 1–2 рассмотренными альтернативами префикса.
- **F-3**: явные триггеры promotion для каждого deferred поля или явное «defer = non-goal до format-v2».
- **F-4**: test-обязательный invariant: «read → write идемпотентен byte-for-byte».
- **F-5**: «batch-операции имеют explicit `updated:` mode (skip / set-now)».
- **F-6**: «`zetto/*` rule namespace reserved for built-in; third-party — в C2a».

## Sources

- [gray_matter on lib.rs](https://lib.rs/crates/gray_matter)
- [yuchanns/gray-matter-rs](https://github.com/yuchanns/gray-matter-rs)
- [Wikilink resolution does not honor frontmatter aliases (1.12.7)](https://forum.obsidian.md/t/wikilink-resolution-does-not-honor-frontmatter-aliases-1-12-7/113902)
- [Obsidian 1.12.7 Mobile changelog](https://obsidian.md/changelog/2026-03-23-mobile-v1.12.7/)
