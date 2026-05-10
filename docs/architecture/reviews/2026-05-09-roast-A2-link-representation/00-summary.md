# Roast: A2 — Link representation

**Target**: `docs/architecture/research/2026-05-09-A2-decision-summary.md`
**Date**: 2026-05-09
**Roles run**: Devil-advocate, Pragmatist, Junior-engineer, Compliance-officer, Futurist

## Headline findings

- **Devil-advocate**: B-1 — «Forward-compat free» это семантическая ловушка отложенного связывания: в v1 пользователь пишет `![[X]]`, `[[X#H]]`, `[[X#^id]]` под прикрытием lint warn; в момент v2 release эти литералы получают активную семантику и меняют рендер существующего корпуса без миграции и diff-а.
- **Pragmatist**: H-2 — шесть lint-правил без C2a engine — это 1.5–3 недели соло-работы как полноценный sub-product, не «делта к парсеру»; ADR оценивает их как «free» благодаря парсеру, но без engine они не сработают.
- **Junior-engineer**: J-1 — «Альтернатива A» упоминается в шапке без секции Alternatives considered и без перечня leans; читатель вынужден открывать discovery/design.
- **Compliance-officer**: C-1 — `[[ULID]]` в опубликованной заметке передаёт читателю **существование других ULID-ов** + их **ms-точное creation time** (через декодирование Crockford-prefix-а); A2 silent на этот побочный эффект, в отличие от ADR-0002 § Privacy.
- **Futurist**: F-1 — «Defer в v2» эмпирически становится квази-постоянным состоянием в OSS-проектах со соло-разработчиком; шесть `*-not-supported-in-v1` lint-правил к 2028 либо игнорируются (warn-fatigue), либо выключены вручную.

## Severity counts

| Role | High | Medium | Low |
|---|---|---|---|
| Devil-advocate | 2 (B-1, B-2) | 5 (B-3, B-4, B-5, B-6, B-9) | 2 (B-7, B-8) |
| Pragmatist | 1 conditional (H-4) | 4 (H-1, H-2, H-5, H-6) | 2 (H-3, H-7) |
| Compliance-officer | 0 | 2 (C-1, C-2) | 4 (C-3, C-4, C-5, C-6) |
| Junior-engineer | — | — | — |
| Futurist | — | — | — |

(Junior-engineer и Futurist по разрешению `commands/roast.md` строка 109 не используют severity-категории; junior — clarity-сортировка; futurist — structural confidence + trend confidence.)

## Cross-cutting concerns

### CC-1: «Defer в v2» — семантическая ловушка отложенного связывания (3 роли)

- **B-1**: parser принимает синтаксис → пользователь пишет `![[X]]` / `[[X#H]]` / `[[X#^id]]` → в v2 эти литералы получают активную семантику без diff-а или migration tool
- **F-1**: defer-without-trigger становится defer-навсегда; lint warn → warn-fatigue → выключают в config
- **H-3**: «forward-compat free» вводит в заблуждение про вероятность v2; три фичи отложены одновременно ≈ never

**Underlying issue**: ADR должен (a) ввести explicit trigger conditions для каждой отложенной фичи (по образу ADR-0002 § Implementation Out-of-repo follow-ups); (b) зафиксировать политику для v1→v2: либо migration tool обязателен (даже если изменения — только семантика рендера), либо принять, что v2 поведение — opt-in через config flag.

### CC-2: Synchronous frontmatter scan нарушает latency budget (4 роли)

- **B-2**: на 500 заметках × 10 wikilinks = O(N²) FS reads на render; ARCHITECTURE.md §2.1 фиксирует <500 ms — нарушается на ≈200 заметках
- **B-6**: TUI preview pane заметки с 30 wikilinks без display = 30 disjoint disk reads + 30 YAML parse; на NFS/sshfs — секунды
- **H-1**: synchronous scan не имеет явного потолка; нарушает бюджет «Index lookup <100 ms»
- **F-3**: B1 на «Уровне 2», блокирован A1/A2/A4; для соло-проекта 12–18 месяцев типично; де-факто sync scan может стать постоянным

**Underlying issue**: ADR должен (a) добавить metric-trigger «scan >50 ms p50 при N≥500 → B1 разблокируется внепланово»; (b) опционально включить interim in-memory cache title-by-id (50 строк кода, 1–2 года breathing room); (c) явно зафиксировать какой кусок capture-latency budget из §2.1 расходуется на render-fallback.

### CC-3: Resolver edge cases и расширяемость (4 роли)

- **B-3**: ULID-validation regex `^[0-9A-HJKMNP-TV-Z]{26}$` принимает out-of-range timestamps (`ZZZZZZZ...`); case-sensitivity contract не зафиксирован
- **B-4**: markdown-link `[text](../inbox/01J9X...)` — `^`-anchor regex не сматчит; basename-extract шаг не описан
- **B-5**: external-URL detection regex неполный (`file:`, `javascript:`, `data:`, custom schemes — не ловятся); потенциальная sanitization-проблема для glob input
- **F-2**: resolver-сложность будет копиться (aliases, slug-rename detection, case-insensitive Obsidian import, multi-vault) — «один простой резолвер» становится 8–12 шагов через 2 года

**Underlying issue**: ADR должен (a) уточнить ULID-validation: regex + range check на timestamp; (b) добавить basename-extract шаг в markdown-link branch; (c) расширить external-URL detection до allow-list или явно ограничить interesting schemes; (d) зафиксировать «resolver — ordered passes, добавление нового pass — локальное изменение».

### CC-4: Lint rules — engine отсутствует, severity-семантика не определена (3 роли)

- **H-2**: 6 lint-правил без C2a engine — sub-product работа, не «делта»; имена резервируются, реализация — впереди
- **J-7**: cross-reference на «ADR-0002 § Methodology engine architecture» — секции с этим именем нет; preset `recommended-luhmann` упомянут, но не определён
- **J-10**: severity `error` для `external-url-as-wikilink` — что значит error в zetto? Не определено до C2a

**Underlying issue**: ADR должен (a) явно сказать «этот ADR резервирует имена правил и предлагаемые severity defaults; семантика error/warn и engine architecture — в C2a, который open»; (b) переименовать footnote на ADR-0002 — там нет такой секции, должно быть «см. C2a в decision-map».

### CC-5: Privacy / metadata leakage при шаринге (1 роль, 2 угла)

- **C-1**: ULID в `[[ULID]]` раскрывает creation-times связанных заметок при публикации одной заметки наружу
- **C-2**: render-fallback на frontmatter title подставляет title чужой заметки в render публикуемой → potential title-leakage если связанная заметка не шарится

**Underlying issue**: ADR должен унаследовать § Privacy and security considerations подход из ADR-0002 — по крайней мере короткой подсекцией. Конкретно: (a) явное признание «`[[ULID]]` декодируется в creation time»; (b) отметка про render-fallback и future export-mode (render literal vs frontmatter scan).

### CC-6: Forward-compat обещает больше, чем парсер даёт (2 угла)

- **F-4**: ADR-0002 § Format versioning anchor говорит «ID-rendering — implementation detail, не public ABI»; A2 расходует это, но v1-binary и v2-binary рендерят embed/anchor/block-ref по-разному → файлы валидны в обе стороны, но visible behavior расходится
- **B-1**: «forward-compat free» технически верно (parser не падает), семантически ложно (рендер меняется)

**Underlying issue**: ADR должен переформулировать «forward-compat free» → «weak forward-compat: parser, не renderer»; явно описать diff поведения v1 vs v2 для трёх отложенных синтаксисов.

### CC-7: D4 (Obsidian-compat) — асимметрия ID-based vs title-based резолвера (1 роль)

- **F-5**: A2 принимает решение, *совместимое только с D4 ∈ {own format, read-only}*; D4=read-write требует пересмотра A2 (Obsidian резолвит wikilinks по filename/title; zetto — по ULID)

**Underlying issue**: явно зафиксировать в § Open questions, что A2 неявно сужает D4. Если D4 в итоге пойдёт в read-write — ADR-0003 пересматривается, не просто расширяется.

## Recommended path

**Apply findings, then proceed to Document.** Decision (Альтернатива A — wikilink-primary) не оспорена ни одной ролью. Cross-cutting CC-1, CC-2, CC-3, CC-4 указывают на текстовые правки + механические дополнения в ADR; CC-5, CC-6, CC-7 — про дополнительные подсекции (Privacy, weak-forward-compat, D4-implication).

Конкретные правки в ADR-0003 (по приоритету):

1. **CC-1 / B-1, F-1**: добавить trigger conditions в § Deferred в v2 для каждой отложенной фичи (embeds, anchors, block-refs).
2. **CC-2 / B-2, B-6, H-1, F-3**: добавить metric-trigger «scan >50 ms p50 при N≥500 → B1 разблокируется внепланово»; опционально in-memory cache как interim.
3. **CC-3 / B-3, B-4, B-5, F-2**: уточнить resolver edge cases (ULID-range check + case-sensitivity, markdown-link basename-extract, external-URL schema-allow-list); зафиксировать «resolver — ordered passes».
4. **CC-4 / H-2, J-7, J-10**: явно сказать «этот ADR резервирует имена правил; semantic engine — в C2a (open)»; исправить broken cross-reference на «ADR-0002 § Methodology engine architecture» → «C2a в decision-map».
5. **CC-5 / C-1, C-2**: добавить § Privacy and security considerations (короче, чем в ADR-0002, но наследующая принцип).
6. **CC-6 / F-4, B-1**: переформулировать «forward-compat free» → «weak forward-compat: parser, не renderer»; явно описать v1-vs-v2 visible-behavior delta.
7. **CC-7 / F-5**: явная нота в § Open questions: A2 совместима только с D4 ∈ {own format, read-only}; D4=read-write требует амендмента A2.
8. **J-1, J-2, J-3**: добавить секцию «Alternatives considered (выжимка из design)» с тремя альтернативами и leans-перечнем; заменить «research §N» на полу-предложения.
9. **J-5, J-6, J-8, J-9, J-11** (clarity): объяснить `has_pothole` при первом упоминании; «broken style» — implementation-detail-marker; format-v1 — будущая спецификация (A5 open); `#`-suffix-семантика — явная отсылка вверх; sync scan — оценить кусок latency budget.
10. **B-7, B-8**: edge cases для display-text с markdown внутри + multi-pipe; декаплинг lint-severity и render-behavior.
11. **B-9**: явное поведение wikilinks внутри inline-code и markdown-link-text (parser принимает; lint игнорирует или флагает).
12. **H-4** (Obsidian-vault import): runbook одна строка — «первый импорт даст тысячи lint warn; ожидаемое поведение, не баг».

После применения — **proceed to Document** (ADR-0003). Re-roast не требуется.

## Per-role outputs

- [01-devil-advocate.md](./01-devil-advocate.md) — 9 findings (B-1 … B-9); 2 high, 5 medium, 2 low
- [02-pragmatist.md](./02-pragmatist.md) — 7 findings (H-1 … H-7); 1 conditional-high (H-4), 4 medium, 2 low
- [03-junior-engineer.md](./03-junior-engineer.md) — 11 findings (J-1 … J-11); severity не назначен
- [04-compliance-officer.md](./04-compliance-officer.md) — 6 findings (C-1 … C-6); 0 high, 2 medium, 4 low
- [05-futurist.md](./05-futurist.md) — 9 findings (F-1 … F-9); 6 structural (high-confidence), 3 trend (medium/low confidence)

**Total**: 42 findings across 5 roast roles.
