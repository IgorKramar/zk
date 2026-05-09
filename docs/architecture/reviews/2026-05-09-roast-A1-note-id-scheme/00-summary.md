# Roast: A1 — Note ID scheme + filename layout

**Target**: `docs/architecture/research/2026-05-09-A1-decision-summary.md`
**Date**: 2026-05-09
**Roles run**: Devil-advocate, Pragmatist, Junior-engineer, Compliance-officer, Futurist

## Headline findings

- **Devil-advocate**: B-4 — empty-slug fallback не edge case, а **default state** (`zetto new` присваивает ULID до того, как пользователь напечатает title), что аннулирует UX-аргумент против Альтернативы C — половина vault де-факто остаётся `<ULID>.md`.
- **Pragmatist**: P-1 — `serde_yaml` **архивирован автором David Tolnay в марте 2024** и помечен «no longer maintained»; миграция на `serde_yml`/`saphyr` неизбежна, но не упомянута в декларации.
- **Junior-engineer**: J-9 — `format-v1` упоминается как уже существующий контракт, но A5 (Format versioning policy) ещё **open** в decision-map; читатель пытается найти определение, его нет.
- **Compliance-officer**: C-2 — title-derived slug в filename превращает sensitive title (например, `Therapy session 2026-04-11 with Dr. Smith`) в имя файла, всплывающее в `git log`, shell history, swap-файлах, fzf-cache, скриншотах и опубликованных форках — surface намного шире, чем артефакт признаёт.
- **Futurist**: F-3 — `format-v1` как «frozen» переоценивает дисциплину одного автора; эмпирически в pre-alpha-проектах формат живёт 6–12 месяцев до первого болезненного нарушения; migration tool становится обязательным артефактом, а не «событием, которого избегаем».

## Severity counts

| Role | High | Medium | Low |
|---|---|---|---|
| Devil-advocate | 4 (B-1, B-3, B-4, B-8) | 4 (B-2, B-5, B-6, B-7) | 0 |
| Pragmatist | 0 | 4 (P-1, P-3, P-4) | 2 (P-2 low–medium, P-5) |
| Compliance-officer | 0 | 3 (C-1, C-2, C-4) | 3 (C-3, C-5, C-6) |
| Junior-engineer | — | — | — |
| Futurist | — | — | — |

(Junior-engineer и Futurist по разрешению `commands/roast.md` строка 109 не используют severity-категории; junior сортирует по «undefined term / erased reasoning / unfollowable step / broken cross-reference», futurist — по «structural / trend» с явным confidence-rating.)

## Cross-cutting concerns

Findings, поднятые ≥2 ролями, указывающие на одну underlying issue. Когда независимые перспективы сходятся — issue реальная.

### CC-1: Retitle-атомарность и multi-machine sync (3 роли)

- **Devil-advocate B-1**: rename файла + edit frontmatter — две независимые FS-операции; без транзакции — рассинхрон при crash.
- **Devil-advocate B-8**: git pull/merge на двух машинах после параллельного retitle + body-edit ломает backlinks молча.
- **Pragmatist P-2**: `std::fs::rename` атомарен на одной машине, но не семантически между машинами; ~1 ручное вмешательство в год; нужен runbook.
- **Futurist F-2**: `git mv` накапливает rename-history; через год `git log --follow` единственный способ проследить заметку; экспорт vault в read-only снапшот теряет URL-стабильность.

**Underlying issue**: rename — load-bearing операция, которая описана как «достаточна», но фактически распадается на несколько failure-mode'ов в разных временных горизонтах. ADR должен явно описать (a) recovery-procedure при partial state; (b) rename-as-canonical-permalink — rename = ULID, не filename; (c) пользовательский runbook для sync-конфликта.

### CC-2: Frontmatter vs filename — source of truth (2 роли)

- **Devil-advocate B-3**: декларация говорит «frontmatter — source of truth», но lookup `[[ULID]]` идёт через filename-glob — два конфликтующих контракта на один объект; copy-paste заметки даёт два файла с одним `id:`.
- **Junior-engineer J-11**: «backlinks `[[ULID]]` ломаются» — под каким сценарием? Если `id:` поменян, но filename не переименован, резолв через filename-glob их не ломает.

**Underlying issue**: ADR должен явно зафиксировать инвариант («frontmatter `id` совпадает с ULID-prefix filename») и описать поведение zetto при detected divergence (refuse to operate? warn? auto-correct?).

### CC-3: format-v1 «freeze» нереалистична для one-author pre-alpha (3 роли)

- **Pragmatist P-4**: format-v1 контракт зафиксирован, миграционного инструмента нет; первая невинная идея, требующая format-v2, ловит автора без миграционного инструмента под давлением.
- **Junior-engineer J-9**: format-v1 в декларации звучит как уже существующая сущность; A5 (Format versioning) ещё open.
- **Futurist F-3**: «frozen» переоценивает дисциплину; через 12–18 месяцев один из двух сценариев — либо format-v2 с миграцией (1–2 квартала работы), либо неявный v1.5 в виде накопленных компромиссов.

**Underlying issue**: переименовать «frozen» → «stable, versioned»; зафиксировать в ADR trigger для migration tool («tool пишется тогда, когда первая идея, требующая format-v2, проходит фазу Decide») и сделать его обязательным артефактом каждого bump.

### CC-4: Empty-slug как первичный, а не граничный сценарий (2 роли)

- **Devil-advocate B-4**: `zetto new` присваивает ULID *до* того, как пользователь напечатает title; большинство свежесозданных заметок начинают как `<ULID>.md` без slug-а; UX-аргумент против Альтернативы C аннулируется.
- **Pragmatist P-5**: CJK fallback — open question, но в operationally-значимом месте; редкий, но создаёт inconsistent UX («почему эта заметка без slug?»).

**Underlying issue**: ADR должен явно описать lifecycle slug: когда первая запись срабатывает retitle-flow (на первое сохранение body после набора title? Watcher? Manual `zetto retitle`?) — без этого кушать UX-обоснование Альтернативы A нельзя.

### CC-5: Filename как канал утечки — title и timestamp (1 роль, но 2 finding'а)

- **Compliance-officer C-1**: ULID-timestamp в filename декодируется тривиально; публикация vault раскрывает creation-times.
- **Compliance-officer C-2**: title-derived slug всплывает в git log / shell history / swap / fzf cache / скриншотах.

**Underlying issue**: filename — публичная поверхность, и формат `<ULID>-<slug>.md` навсегда фиксирует и timestamp, и slugified title как наблюдаемые. ADR должен включить «Privacy-and-Security Considerations» подсекцию с (a) явным признанием утечки timestamp; (b) рекомендацией для sensitive-title-сценариев; (c) опционально — config-knob для ULID-only filename как opt-in.

### CC-6: Фактическая ошибка про monotonicity (1 роль, но точный)

- **Devil-advocate B-5**: декларация утверждает, что `Ulid::new()` monotonic в одной мс — это ложь (`Generator::generate_from_datetime` со стейтом нужен; `Ulid::new()` каждый вызов независим). Между двумя `zetto new` процессами monotonicity недоступна без external persistent state.

**Underlying issue**: исправить декларацию — либо удалить monotonicity-claim целиком (для personal-CLI с <100 заметок/сек он не нужен), либо описать как «между двумя ULID, сгенерированными в одном `zetto`-процессе, monotonicity недоступна; для CLI это не блокер».

## Recommended path

**Apply findings, then proceed to Document.** Решение по сути (Альтернатива A — `<ULID>-<slug>.md` filename) **не оспорено** ни одной из 5 ролей. Ни один cross-cutting не предлагает пересмотр самого выбора. Все blockers — addressable текстом ADR + минимальной реализационной дисциплиной.

Конкретные правки в ADR (по приоритету):

1. **CC-6 / B-5** (factual error). Удалить monotonicity-claim или переформулировать как «monotonicity не гарантируется между процессами; для personal-CLI этого достаточно». Минимальная правка, но обязательная — иначе первый читатель кода найдёт ложь.
2. **CC-1 / B-1, B-8, P-2, F-2** (rename как обязательство). Добавить раздел «Retitle lifecycle и recovery»: явная последовательность шагов, recovery-procedure при partial state, runbook для multi-machine sync-конфликта, явная фиксация «canonical permalink = ULID» в format-v1.
3. **CC-2 / B-3** (source-of-truth). Зафиксировать инвариант явно. Описать поведение при detected divergence: zetto refuses to operate, выводит diagnostic; пользователь решает руками.
4. **CC-4 / B-4** (empty-slug lifecycle). Описать когда срабатывает retitle-flow: либо первое сохранение body с пустым/изменённым title в frontmatter, либо явный `zetto retitle <ULID>`; либо периодическая sweep-команда. Без этого UX-обоснование Альтернативы A слабое.
5. **CC-3 / P-4, J-9, F-3** (format-v1 freeze). Перефразировать «frozen» → «stable, versioned»; trigger для migration tool в текст ADR; ссылка на A5 как пока-open зависимость.
6. **P-1** (`serde_yaml` deprecated). Добавить в Consequences «Risks accepted»: упомянуть deprecation upstream и план миграции на `serde_yml`/`saphyr` при первом security advisory или MSRV-конфликте.
7. **CC-5 / C-1, C-2** (privacy). Добавить «Privacy and security considerations» подсекцию: явное признание timestamp-leak, title-as-PII risk, opt-in для ULID-only filename.
8. **B-7 / P-1** (MSRV). Зафиксировать `rust-version` в ADR как требование (минимум — Rust 1.X с PR #138133 для Windows rename atomicity).
9. **B-6** (manual edit detection). Описать в ADR Open Questions новый узел: «invariant validation — что zetto делает, когда `id:` field в frontmatter не совпадает с ULID-prefix filename».
10. **F-1** (regex как public ABI). Добавить в format-v1 spec: «ULID-rendering в линке — implementation detail, не public ABI».
11. **F-5, F-6** (forward-looking). Добавить в Open Questions: A3 должен зарезервировать имя для `author:`-field; A1 неявно сужает D4 (исключает strict-checker variant).
12. **Junior-engineer J-1, J-2, J-3, J-4, J-5, J-6, J-7, J-8, J-10, J-12, J-13** (clarity). Все эти gap'ы фиксятся текстом ADR-0002 — определения терминов, явные cross-references, относительные пути к research/design-документам.

После применения этих правок — **proceed to Document**. Re-roast не требуется: ни одно cross-cutting не указывает на архитектурный пересмотр.

## Per-role outputs

- [01-devil-advocate.md](./01-devil-advocate.md) — 8 findings (B-1 … B-8); 4 high, 4 medium, 0 low
- [02-pragmatist.md](./02-pragmatist.md) — 6 findings (P-1 … P-6); 0 high, 4 medium, 2 low (P-6 — positive)
- [03-junior-engineer.md](./03-junior-engineer.md) — 13 findings (J-1 … J-13); severity не назначен (по разрешению `roast.md` 109)
- [04-compliance-officer.md](./04-compliance-officer.md) — 6 findings (C-1 … C-6); 0 high, 3 medium, 3 low
- [05-futurist.md](./05-futurist.md) — 9 findings (F-1 … F-9); 6 structural (high-confidence), 3 trend (medium/low confidence)
- [99-meta-review.md](./99-meta-review.md) — plugin-conformance check; 6 findings (M-1 … M-6); 2 high (структурные расхождения с шаблоном — поправлены в этом roast), 1 medium (rename A-N → B-N — поправлено), 3 low

**Total**: 42 findings across 5 roast roles + 6 meta-review findings = 48 findings reviewed.
