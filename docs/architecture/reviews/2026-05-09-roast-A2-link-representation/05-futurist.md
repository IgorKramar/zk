## Futurist findings

- **Target**: `docs/architecture/research/2026-05-09-A2-decision-summary.md`
- **Date**: 2026-05-09
- **Role**: futurist (long-horizon)
- **Horizon**: 1 to 3 years (2026 → 2028)
- **Confidence note**: F-1..F-6 — высокая уверенность (структурный дрейф). F-7..F-9 — спекуляция, с явно названными сигналами и допущениями.

## Summary

Решение в его центральной части — wikilinks-как-синтаксис и pulldown-cmark-как-парсер — стареет хорошо: и формат, и крейт устойчиво поддерживаются, и оба тренда подкреплены экосистемой PKM. Главный долгосрочный дрейф — не в выборе синтаксиса, а в трёх местах вокруг него: «defer-в-v2»-стратегия эмпирически часто оборачивается «defer навсегда», синхронный frontmatter scan становится квази-постоянным контрактом, если B1 откладывается, а позиционирование D4 (Obsidian-compat) натыкается на асимметрию ID-based / title-based резолвера, которая с годами превращается из мелкого зазора в архитектурный шов.

## Structural findings (high-confidence)

### F-1: «Defer в v2» для embeds/anchors/block-refs становится квази-постоянным состоянием

**Type**: codebase aging / inertia
**Horizon**: 1.5–2 года

**The drift**: ADR прямо опирается на «forward-compat free, потому что parser принимает синтаксис уже сейчас». Это технически верно, но в OSS-проектах со соло-разработчиком отложенные фичи реализуются позднее заявленного значительно чаще, чем по плану. К 2028-му с высокой вероятностью v1 vault содержит сотни заметок с `![[...]]`, `[[X#H]]`, `[[X#^id]]`, каждая из которых — lint warn, и lint warn перестаёт восприниматься. Шесть lint-правил `zetto/{embed,anchor,block-ref}-not-supported-in-v1` к 2028 году либо игнорируются (warn-fatigue), либо выключены пользователями вручную.

**Mitigation**: добавить строкой в раздел «Deferred в v2» явный *trigger condition*, при котором отложенные фичи поднимаются на следующий цикл (например: «embeds — когда первая заметка в собственном vault использует `![[...]]` ≥10 раз»; «block-refs — когда B1 закрыт»). Это превращает defer-without-condition в defer-with-condition.

### F-2: Resolver-сложность копится через aliases, slug-rename, broken-link recovery

**Type**: codebase aging
**Horizon**: 1–2 года

**The drift**: Сейчас resolver описан как 4-шаговый алгоритм для wikilink + 2-шаговый для markdown-link. К 2028 на тот же resolver почти неизбежно лягут: aliases (если A3 их добавит, что вероятно для D4=read-write), slug-rename detection и follow, case-insensitive сопоставление для импорта Obsidian-материалов, multi-vault-aware lookup. Каждый из шагов добавляет один слой ветвлений. «Один простой резолвер» — это не свойство задачи, а свойство момента; в 2 года такой резолвер обычно содержит 8–12 шагов, нетривиальный appeal/disambig, и собственный test corpus.

**Mitigation**: добавить «resolver — это extension point, не одна функция» в раздел Render/Resolver. Конкретно — подсказка в ADR-0003 о том, что resolver состоит из ordered passes (`external` → `ulid` → `filename` → `aliases` → ...), и что добавление нового pass — это локальное изменение, не переписывание.

### F-3: Synchronous frontmatter scan становится постоянным, если B1 откладывается

**Type**: scale / inertia
**Horizon**: при росте vault до ≈1000–2000 заметок, или когда B1 откладывается на третий ADR-цикл подряд

**The drift**: ADR честно помечает synchronous frontmatter scan как «temporary v1 решение, заменится при B1». Но `decision-map.md` показывает, что B1 заблокирован A1/A2/A4 и расположен на «Уровне 2», а его предпочтительная форма (event-sourced index из находки O-4) — нетривиальная инженерная работа. В соло-OSS-проекте «Уровень 2 + нетривиально» часто означает 12–18 месяцев. Тем временем render каждой заметки с display-text fallback читает весь frontmatter всех target-заметок. На 100 заметках — невидимо; на 500 — заметно при batch-операциях; на 2000+ — нарушает бюджет capture latency.

**Mitigation**: одна строка — «если scan превышает 50 ms p50 при N≥500 заметок, B1 поднимается приоритетом независимо от других зависимостей». Альтернатива — дешёвый interim: in-memory cache title-by-id, заполняемый при первом scan и инвалидируемый по mtime. Это не B1, это 50 строк кода, и даёт 1–2 года breathing room.

### F-4: ADR-0002 § Format versioning anchor — A2 уже его расходует

**Type**: adjacent decisions / inertia
**Horizon**: при первом v2-bump (1–3 года)

**The drift**: ADR-0002 заявляет «ID-rendering в линке — implementation detail, не public ABI; меняется в любом link-syntax-ADR без поднятия major-версии format-v1». A2 это и делает. Но как только v2 добавит anchors/block-refs/embeds, у формата появятся *семантические* линки, которые старая версия zetto не понимает (она их рендерит literal). Это значит, что v2 — *не* строгий superset v1 в смысле user-visible поведения: один и тот же файл, обработанный v1-binary и v2-binary, даёт разные результаты на embed-формах.

**Mitigation**: одна формулировка в § Forward-compat statement: «v1-binary рендерит embed/anchor/block-ref как literal с lint warn; v2-binary рендерит как inline/jump/transclude. Файлы остаются валидными в обе стороны, но visible behavior расходится». Это честнее чем «forward-compat free».

### F-5: D4 (Obsidian-compat) — асимметрия ID-based vs title-based резолвера становится швом

**Type**: adjacent decisions / inertia
**Horizon**: в момент закрытия D4 (вероятно 2027), и далее — постоянно

**The drift**: Obsidian резолвит wikilinks **по filename/title**, не по ID. zetto генерирует `[[ULID]]`, opacity которого для Obsidian-пользователя ≈ 100% — `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]` без display-text не превратится в осмысленный заголовок ни в каком Obsidian preview. Это значит:
- D4=read-only — работает: zetto использует filename match как 2-й fallback в markdown-link branch.
- D4=read-write — структурная проблема: zetto-генерированные `[[ULID]]` в Obsidian отображаются как литералы; Obsidian-генерированные `[[Note Title]]` zetto не резолвит.
- D4=strict-checker — уже исключён в ADR-0002 § Open questions.

К 2028 D4 должен быть закрыт, и какой бы вариант ни выбрали, асимметрия резолвера становится частью контракта.

**Mitigation**: явно зафиксировать в § Open questions, что A2 принимает решение, *совместимое только с D4=полностью свой формат или D4=read-only*. Если D4 в итоге пойдёт в read-write — A2 нужно будет амендировать.

### F-6: Onboarding нового мейнтейнера зависит от знания трёх неявных контрактов

**Type**: team / codebase aging
**Horizon**: 2 года (в личном OSS «новый мейнтейнер» = сам автор через 18 месяцев забывший контекст)

**The drift**: Чтобы корректно работать с этим резолвером в 2028, инженеру нужно одновременно держать в голове: (1) что filename — source of truth для резолва, frontmatter — source of truth для ID; (2) что parser принимает syntax, который resolver игнорирует, и это не баг; (3) что external-URL detection и ULID validation идут regex-ом до файлового lookup. Эти три факта правильны, но не очевидны из кода.

**Mitigation**: один комментарий-якорь в коде резолвера, ссылающийся на ADR-0003 § Resolver, и одна invariant-проверка в тестах: external-URL `[[https://...]]` → lint flag, *не* file lookup.

## Trend findings (speculative)

### F-7: pulldown-cmark 0.13.x — устойчивая поддержка на 2028 горизонте

**Type**: technology lifecycle / vendor risk
**Confidence**: medium-high

**Signals**:
- Релизный темп: 0.13.0 (февраль 2025) → 0.13.3 (март 2026) — паттерн зрелого крейта
- Maintainership распределён (Pozo, Howell, Salmi, Geisler) — не single-point-of-failure
- Downstream-экосистема: pulldown-cmark-to-cmark v22 от декабря 2025 (mdBook/zola остаются на 0.13 API)
- CommonMark 0.31 поддерживается; следующий major spec не анонсирован

**The drift**: к 2028 pulldown-cmark 0.13 (или 0.14 с лёгким API delta) скорее всего остаётся стандартом. **API-break при 0.13 → 0.14** — ожидаем точечный рефакторинг.

### F-8: CommonMark spec не поглотит wikilinks к 2028; markdown-2 не появится

**Type**: idiom shift
**Confidence**: medium

**Signals**:
- CommonMark spec 0.31.2 (январь 2024) — последний релиз; никакого 0.32 в 2025-2026
- Wikilinks остаются в Proposed Extensions wiki второй год подряд
- Реализации wikilinks разрозненны и взаимно-несовместимы

**The drift**: к 2028 wikilinks — устоявшийся **de-facto** стандарт через Obsidian-влияние, но не **de-jure** часть CommonMark.

### F-9: Hiring-market для Rust + markdown PKM — нерелевантный риск для соло-проекта

**Type**: hiring market
**Confidence**: low (применимо только при изменении team-context)

**The drift**: пропуск. При сохранении соло-режима этот риск нулевой; при появлении контрибьюторов — Rust 2028 остаётся mainstream.

## What's likely to age well

- **Wikilink-primary с ULID-target** — синтаксически устойчиво. Если бы выбор был title-based, F-5 стал бы внутренним противоречием.
- **External-URL detection до ULID-validation до file-lookup** — порядок шагов резолвера правильный с точки зрения safety.
- **Parser-accepts-syntax forward-compat для embeds** — 0-cost инженерное решение, сохраняющее опциональность v2.
- **Lint rules как первая партия `recommended-luhmann` preset** — встраивает A2 в C2a-flow, давая всему link-handling-у единый surface.
- **Опора на ADR-0002 для базовой инфраструктуры** — A2 не дублирует риски A1, что упрощает аудит.

## What's worth deciding now to defer pain

- **F-1**: добавить trigger conditions в § Deferred в v2.
- **F-2**: одной строкой зафиксировать «resolver — ordered passes, расширяемый локально».
- **F-3**: ввести метрический trigger («synchronous scan >50 ms p50 при N≥500 → B1 разблокируется внепланово») и/или дешёвый in-memory cache как interim.
- **F-4**: переформулировать «forward-compat free» как «weak forward-compat: parser, не renderer».
- **F-5**: явно записать в § Open questions / D4 что A2 совместима только с D4 ∈ {own format, read-only}.
- **F-6**: anchor-комментарий в коде резолвера + один регрессионный тест на порядок шагов.

## Sources

- [pulldown-cmark releases on GitHub](https://github.com/pulldown-cmark/pulldown-cmark/releases)
- [pulldown-cmark on docs.rs](https://docs.rs/crate/pulldown-cmark/latest)
- [CommonMark Spec](https://spec.commonmark.org/)
- [CommonMark Proposed Extensions wiki](https://github.com/commonmark/commonmark-spec/wiki/Proposed-Extensions)
- [Obsidian Help — Internal links](https://help.obsidian.md/Linking+notes+and+files/Internal+links)
- [Title As Link Text — Obsidian plugin](https://www.obsidianstats.com/plugins/title-as-link-text)
