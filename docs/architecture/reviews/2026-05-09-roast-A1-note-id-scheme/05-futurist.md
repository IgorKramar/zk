# Futurist findings: A1 — Note ID scheme + filename layout

- **Target**: `docs/architecture/research/2026-05-09-A1-decision-summary.md`
- **Date**: 2026-05-09
- **Role**: futurist (long-horizon)
- **Horizon**: 1 to 3 years
- **Confidence note**: Structural drift findings are high-confidence; trend findings are speculative and named accordingly.

## Summary

A1 принимает на себя три долгосрочных обязательства разной природы: формат ID (структурно стабильный, но с поднимающейся стандартной альтернативой), filename layout (стабильный для terminal-native пользователя, но становящийся помехой по мере диверсификации surface'ов) и заморозку этих двух решений как public spec. Самый консеквентный дрейф — давление UUIDv7 на ULID к 2027–2028 годам и риск, что `format-v1` придётся ослабить, не успев накопить достаточно пользователей, чтобы заморозка стала ценной.

## Structural findings (high-confidence)

### F-1: ULID становится якорем в той же мере, в какой `<ULID>-<slug>.md` встречается в чужих сторонних обвязках

**Type**: inertia
**Horizon**: 1.5–3 года, при условии что zetto преодолеет порог >100 пользователей с публичными vault-репозиториями

**The drift**: Решение фиксирует ULID не только в frontmatter (легко мигрируется), но и в filename и в синтаксисе wikilink `[[<ULID>]]`. Как только пользователи начинают писать сторонние скрипты (rg-обёртки, vim-биндинги, dataview-подобные запросы), regex `[0-9A-HJKMNP-TV-Z]{26}` для распознавания ID попадает в чужой код, который Igor не контролирует. Любая будущая миграция формата ID становится не только rewrite заметок, но и «нужно объявить deprecation в ecosystem», что для одного автора посильно только если ecosystem ещё мал.

**Mitigation**: добавить в спецификацию `format-v1` правило, что ID-rendering в линке рассматривается как имплементационная деталь резолвера, а не как часть публичного контракта; либо явно зафиксировать обратное и принять последствия.

### F-2: Slug-rename через `std::fs::rename` создаёт дрейф git-истории, который ужесточается с возрастом vault

**Type**: codebase aging
**Horizon**: 1–2 года для vault-ов с >300 заметок

**The drift**: Каждый retitle = `git mv` (переименование плюс update-index). Через год работы typical vault накапливает сотни переименований; `git log --follow` становится единственным способом проследить историю заметки. Если в 2027–2028 пользователь захочет, например, экспортировать vault в read-only снапшот сайта (mdbook/Hugo), URL-стабильность теряется на каждом retitle.

**Mitigation**: явно зафиксировать в `format-v1`, что **канонический URL/permalink заметки = ULID, не filename**. Это переносит ответственность за стабильную ссылку на ULID и оставляет slug чисто эргономической проekцией. Сейчас implicit; стоит сделать explicit.

### F-3: `format-v1` как заморозка переоценивает дисциплину одного автора

**Type**: codebase aging / inertia
**Horizon**: 12–18 месяцев после публикации v0.1

**The drift**: A5 ещё не решён, но A1 уже декларирует, что layout «фиксируется как часть format-v1». Эмпирически в pre-alpha-проектах одного автора первая публичная версия формата живёт 6–12 месяцев до первого болезненного нарушения. При фиксации сейчас два сценария к 2028: (а) нашлось что-то лучшее → нужен `format-v2` + migration tool, что съедает 1–2 квартала самостоятельной работы; (б) Igor избегает миграции и накапливает «компромиссы поверх v1», которые в сумме образуют негласный v1.5.

**Mitigation**: смягчить язык от «frozen» к «stable but versioned»; перенести акцент с «изменение требует мажорной версии» на «миграция дёшева, потому что версия номеруется и migration tool — обязательный артефакт каждого bump-а».

### F-4: Контрибьюторы в 2027 не должны знать ULID, но обязаны знать про Crockford

**Type**: team / hiring
**Horizon**: момент, когда появится первый внешний contributor

**The drift**: Большая часть кода будет обращаться к ID как к opaque string. Это хорошо. Но в любой нетривиальной обработке (sortability, проверка валидности ID, парсинг линка) контрибьютору придётся понять: (а) почему именно `[0-9A-HJKMNP-TV-Z]{26}`, а не общий base32; (б) почему нельзя использовать generic crockford crate (см. issue #81); (в) как этот regex взаимодействует с filename. Это три скрытых tribal-knowledge артефакта.

**Mitigation**: добавить в `format-v1` (или в README исходника) короткий раздел «ID validation regex и почему именно такой» с явными ссылками на ulid spec issue #81 и Crockford-таблицу.

### F-5: Multi-author scenario не закрыт, но ULID без node-component делает миграцию в него простой подменой

**Type**: adjacent decisions
**Horizon**: если/когда multi-author размораживается

**The drift**: ULID глобально уникален бесплатно, и это правильное решение для будущей кооперации. Но А1 не закладывает поле `author:` или аналог в frontmatter (отдано в A3). К 2028, если возникает реальный multi-author use case, attribution будет восстанавливаться из git blame, что хрупко при сквошах и patch-import.

**Mitigation**: ничего в А1 — это законно зона А3. Но сделать заметку в open questions, что A3 должен зарезервировать имя для author-attribution.

### F-6: Obsidian-compat (D4) как strict checker несовместим с фиксированным `<ULID>-<slug>.md` filename

**Type**: adjacent decisions / inertia
**Horizon**: момент решения D4, ориентировочно после Group A

**The drift**: D4-вариант (d) — strict checker поверх существующего Obsidian-vault — предполагает, что zetto читает чужие filenames, не диктуя их. Текущая фиксация `<ULID>-<slug>.md` как часть format-v1 закрывает D4(d) на уровне контракта.

**Mitigation**: добавить одну строку в раздел «Open questions» о том, что A1 неявно ограничивает D4 вариантами (a) полностью свой, (b) read-only, (c) read-write — и исключает (d) strict checker over arbitrary vault.

## Trend findings (speculative)

### F-7: UUIDv7 поднимется как «правильный по умолчанию» к 2027, ULID останется нишевым

**Type**: technology lifecycle / idiom shift
**Confidence**: medium

**Signals**:
- RFC 9562 опубликован в мае 2024, формализовал UUIDv7
- PostgreSQL 18 (релиз 2025) — встроенная генерация UUIDv7
- Java native поддержка RFC 9562 в октябре 2025
- Python 3.14+ обновил `uuid` модуль с UUIDv7
- arXiv 2509.08969 (сентябрь 2025) — ULID всё ещё выигрывает в network overhead и generation speed, но UUIDv7 — стандарт с лучшим database support
- `ulid` crate dylanhart: 1.2.1 март 2025, открытые issues с 2023 года не закрыты — крейт работает, но velocity замедлилась

**The drift**: К 2028 «нормальная по умолчанию» рекомендация для time-prefixed ID на новом проекте — UUIDv7. ULID останется в нишах human-facing readability. Для zetto это не критично — его выбор ULID по основанию readability как раз попадает в ту нишу, где ULID объективно лучше (26 chars vs 36, Crockford vs hex). Но в 2028 объяснять выбор станет дороже.

**What would change my mind**: появление IETF-черновика на ULID или присоединение ULID к RFC 9562 как версии 8/9; либо явное замедление UUIDv7.

### F-8: `slug` crate Stebalien — кандидат на тихую смерть к 2028

**Type**: vendor risk
**Confidence**: medium

**Signals**:
- последняя активность на lib.rs — август 2024
- зависимость `deunicode` обновлялась в апреле 2025 (живая)
- `slug` сам — тонкая обёртка над `deunicode`
- альтернативы (`slugify`, прямое использование `deunicode` + ручная нормализация) тривиальны

**The drift**: 1–2 года крейт продолжит работать; в 2028 возможен (а) тихий fork, (б) переход на прямой вызов `deunicode` + 30 строк нормализации внутри zetto.

### F-9: AI-assisted file navigation в редакторах в 2027–2028 делает префикс из 26 chars менее раздражающим

**Type**: idiom shift
**Confidence**: low

**Signals**: рост LLM-интеграций в Helix/Zed/nvim в 2025–2026; fuzzy-finder-эволюция в сторону semantic search

**The drift**: Аргумент «стена ID в `ls`» (которым в design отвергнут вариант C) частично теряет силу к 2028, если терминальная навигация в среднем перемещается от `ls`+глаз к picker+matcher. Это не делает решение A неправильным, но **снижает его относительное преимущество над C**.

## What's likely to age well

- **ULID в frontmatter как source of truth, filename как projection.** Этот разрез переживает любое изменение filename convention.
- **`gray_matter` для чтения и `serde_yaml` для записи.** YAML frontmatter — de-facto стандарт plain-markdown PKM.
- **Empty-slug fallback (`<ULID>.md` без хвостового `-`).** Маленькое, но правильное решение.
- **`atomic-write-file` для body-edits.** Нерезонирующее, durable решение.
- **Чёткое отделение link semantics в A2 от ID scheme в A1.** Декомпозиция оставляет дверь для эволюции линкования, не трогая ID.

## What's worth deciding now to defer pain

- **F-1**: добавить в `format-v1` явное правило, что **regex ULID и rendering ID в линке — implementation detail, не public ABI**.
- **F-2**: зафиксировать в `format-v1`, что **canonical permalink заметки = ULID**, filename — проекция.
- **F-3**: переименовать «frozen format-v1» в «stable, versioned format-v1». Migration tool — обязательный артефакт каждого bump.
- **F-4**: добавить раздел «ID validation: regex and why» в `format-v1` или в исходный код модуля ID.
- **F-5**: открыть TODO в decision-map для A3: зарезервировать имя для author-attribution.
- **F-6**: добавить в open questions A1 одну строку: «A1 неявно сужает D4 — strict checker over arbitrary vault несовместим с фиксированным filename layout».

## Sources

- [Time-Sortable Identifiers Explained: UUIDv7, ULID, and Snowflake Compared](https://www.authgear.com/post/time-sortable-identifiers-uuidv7-ulid-snowflake/)
- [arXiv 2509.08969 — A Comparative Analysis of Identifier Schemes](https://arxiv.org/abs/2509.08969)
- [What is a UUID? Complete Guide to RFC 9562](https://dogenerator.com/en-us/blog/what-is-a-uuid-the-complete-guide-to-rfc-9562-and-modern-unique-identifiers)
- [dylanhart/ulid-rs](https://github.com/dylanhart/ulid-rs)
- [Stebalien/slug-rs](https://github.com/Stebalien/slug-rs)
- [Wikilink resolution does not honor frontmatter aliases (Obsidian 1.12.7)](https://forum.obsidian.md/t/wikilink-resolution-does-not-honor-frontmatter-aliases-1-12-7/113902)
