# Design: A2 — Link representation — alternatives

- **Date**: 2026-05-09
- **Cycle**: A2, deep
- **Inputs**: discovery (round 1, 7 leans все приняты), research digest 2026-05-09
- **Status**: design phase, ожидает подтверждения альтернативы A или pivot

## Зафиксированные предпосылки (из ADR-0002 + 7 leans Discovery)

- ID в линке = ULID (canonical 26-char Crockford); никакого slug/filename внутри `[[]]`.
- Frontmatter `id:` — source of truth; filename — ergonomic projection.
- Парсер — `pulldown-cmark` 0.13.3 с `Options::ENABLE_WIKILINKS` (нативная поддержка `[[ID]]` и `[[ID|display]]`, `LinkType::WikiLink { has_pothole }`).
- Display-text: `[[ID|display]]` опционально + render-time fallback на frontmatter `title` если display нет.
- Embeds (`![[ID]]`), anchor refs (`[[ID#Heading]]`), block refs (`[[ID#^block-id]]`) — defer в v2 (parser уже принимает синтаксис, но zetto не использует/не пишет).
- External URLs: canonical markdown `[text](https://...)`. Wikilinks reserved internal-only.
- Resolver behavior на broken/invalid: lint-rule `zetto/no-broken-link` + literal preserved; `zetto open <broken>` → error.

Эти leans почти полностью определяют альтернативу A (рекомендованную). Альтернативы B и C ниже представлены для проверки, что выбор честный.

## Альтернативы

### A. Wikilink-primary с canonical markdown read-compat

**Write** (что zetto генерирует при создании линка):
- Internal: `[[ULID]]` или `[[ULID|display]]`. zetto автоматически вставляет display-text если пользователь явно указал, иначе оставляет `[[ULID]]` и render берёт title из frontmatter.
- External: `[text](https://...)`. zetto никогда не пишет `[[https://...]]`.

**Read** (что zetto распознаёт при парсинге):
- `[[ULID]]` / `[[ULID|display]]` — primary internal-link.
- `[text](path.md)` — recognized для backward-compat и для контента импортированного из Obsidian/CommonMark-source. Резолвер извлекает ULID-prefix из filename target-а regex-ом `^[0-9A-HJKMNP-TV-Z]{26}` и резолвит через ULID; если match нет — fallback на стандартный filename match.
- `[[https://...]]` (внешний URL внутри wikilink) — lint-rule `zetto/external-url-as-wikilink` flag-ит (parser принимает, мы запрещаем post-parse).
- `![[ID]]` — lint-rule `zetto/embed-not-supported-in-v1` flag-ит; render заменяет на literal без HTML-image.
- `[[ID#anchor]]` / `[[ID#^block]]` — parser отдаёт `dest_url == "ID#anchor"`; v1 lint flag-ит как «not supported in v1, defer to v2»; render — literal.

**Resolver** (как `[[ULID]]` превращается в filepath):
- `dest_url.split_once('#')` — отделяет ID от опционального suffix.
- ULID-validation: regex `^[0-9A-HJKMNP-TV-Z]{26}$` на ID-часть.
- File lookup: filename glob `<ULID>-*.md` или `<ULID>.md` (см. ADR-0002).
- На broken/invalid: lint flag, но render-time линк остаётся literal.

**Render** (как линк отображается):
- `[[ULID]]` → если display-text есть в `[[ULID|display]]`, рендерит display. Иначе — синхронно (в v1) читает frontmatter `title` target-а, рендерит title. Если target не резолвится — рендерит ULID literal в «broken» style.
- `[text](path.md)` → стандартный CommonMark-render (text как label).

### B. Markdown-only canonical

**Write**: zetto генерирует только `[text](<ULID>-<slug>.md)`. Internal линки — стандартный CommonMark.

**Read**: только canonical markdown. Wikilinks `[[X]]` либо игнорируются (parser-без-extension парсит `[[X]]` как literal text), либо — lint flag «non-canonical syntax».

**Resolver**: filename match с ULID-prefix-extract как в A.

**Render**: стандартный CommonMark; text label из markdown-link, не из frontmatter.

**Pros vs A**: pure CommonMark; работает в любом mdbook/Hugo/pandoc-renderer без extension; парсер ENABLE_WIKILINKS не нужен; D4 (Obsidian-compat) read-only поверх Obsidian-vault — markdown-стиль линков Obsidian тоже поддерживает.

**Cons vs A**: длинные линки в исходнике (`[on fixed IDs](01J9XQK7ZBV5G2D8X3K7C4M9P0-on-fixed-ids.md)` против `[[01J9XQK7ZBV5G2D8X3K7C4M9P0|on fixed IDs]]` или просто `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]`); rename slug ломает текстовый label на read (хотя ULID-prefix-extract его исцеляет на target side); Obsidian-compat read-write ослабевает (Obsidian primary syntax — wikilinks).

### C. Full-Obsidian-superset (wikilinks + embeds + anchors + block-refs в v1)

Расширение A: всё из A плюс:
- Embeds `![[ULID]]` — рендерятся как inline-блок (контент target-заметки включается в текущей).
- Anchor refs `[[ULID#Heading]]` — резолвятся в `<a href="...#heading-anchor">`. Требует heading-stable seed (slugify heading text).
- Block refs `[[ULID#^block-id]]` — требует auto-generation block-IDs при write (Obsidian convention `^[a-z0-9]{6}` per-paragraph).

**Pros vs A**: max D4-compat (Obsidian users могут импортировать vault с минимальными правками); богатый PKM-функционал из коробки.

**Cons vs A**: ×3 parser-rules + render-pass complexity; block-IDs требуют отдельного auto-id-схемы (yet another decision внутри A2); heading-stable contract (если пользователь меняет heading text — anchor-ref ломается, нужна heading-id-stability через дополнительный slug); migration tool для embeds (если v2 их меняет) — гарантированная боль; CI для всех трёх features в pre-alpha. **Существенно** утяжеляет первый release.

## Trade-off matrix

| Сила | A: wikilink+markdown-read | B: markdown-only | C: full-Obsidian-superset |
|---|---|---|---|
| Brevity в исходнике | ✓✓ `[[ID]]` или `[[ID|t]]` | ✗ длинный markdown-link | ✓✓ |
| Стабильность через rename | ✓✓✓ ULID anchor; rename не трогает линк | ✓✓ ULID-prefix-extract на read | ✓✓✓ |
| Toolchain interop (pandoc/mdbook без extension) | ⚠ wikilinks — нестандарт CommonMark | ✓✓✓ pure CommonMark | ✗ embeds/anchors не работают вне zetto |
| D4 read-only (zetto читает Obsidian-vault) | ✓ (markdown-read есть) | ✓✓ | ✓✓✓ |
| D4 read-write (Obsidian читает zetto-vault) | ✓ wikilinks работают | ⚠ markdown-links Obsidian понимает, но primary syntax — wikilinks; aliases-resolution в Obsidian было сломано до 1.12.7 | ✓✓✓ |
| Parser complexity | medium (`Options::ENABLE_WIKILINKS` + post-parse lint) | low (vanilla `pulldown-cmark`) | high (rules для embeds/anchors/blocks + render-pass) |
| Implementation effort v1 | medium | low | high |
| Lint rule scope | small (`no-broken-link`, `external-url-as-wikilink`, `embed-not-supported-in-v1`, `anchor-not-supported-in-v1`) | smaller (`no-broken-link` only) | large (всё из A + embed-render-rules + anchor-stability + block-id-uniqueness) |
| Forward-compat в v2 | ✓✓ parser уже принимает `#suffix` и `![[...]]`; v2 = добавить resolver-логику | ⚠ wikilinks потребуют новый ADR (изменение синтаксиса) | N/A (всё уже в v1) |
| Migration cost при изменении синтаксиса | medium (user content на wikilinks) | low (markdown — стандарт; vanilla migration) | high (embeds/anchors/blocks все user-written) |

**Подсчёт ✓✓✓**: A — 1, B — 1, C — 3. **✗**: A — 0, B — 1, C — 1.
**Качественно**: B простейший и pure-CommonMark, но проигрывает A в brevity и D4 read-write; C максимально feature-rich, но утяжеляет v1 на ~3× и заставляет принимать decisions, не нужные сейчас (heading stability, block-ID schema).

## Lean (моя рекомендация)

**Альтернатива A.** Reasoning:

1. **Все 7 принятых leans Discovery один-в-один соответствуют A.** Pivot на B потребует пересмотра leans Q1/Q3; pivot на C потребует пересмотра leans Q4/Q5.
2. **Forward-compat** через `pulldown-cmark` 0.13.3 — `[[ID#suffix]]` синтаксис уже принимается parser-ом, dest_url содержит suffix; в v2 resolver просто добавит split-on-hash. Embeds — `Tag::Image{LinkType::WikiLink}` уже в parser; v1 lint-flag, v2 — render-логика. **Никакой migration в v2 не требуется** — это выигрыш A над B (B потребует ADR на переход к wikilinks при необходимости brevity).
3. **D4 (Obsidian-compat)**: A даёт read-write Obsidian-compat при минимальных правках. Strict-checker variant D4 уже исключён ADR-0002; остальные варианты (read-only, read-write, mutually-exclusive) с A совместимы.
4. **Парсер уже выбран в ADR-0002** (`pulldown-cmark`). `Options::ENABLE_WIKILINKS` — single-line изменение. Стоимость A vs B на парсер-стороне ≈ нулевая.
5. **Empirical: ни одного Rust PKM с зрелым `[[ID|display]]` в 2026** (research digest §7). zetto будет в относительно пустой нише — но не на пустом месте, потому что Obsidian/Foam/Logseq/Dendron этот pattern уже стабилизировали в JS/TS-экосистеме.

**Cons A**, которые принимаем явно:
- Wikilinks — нестандарт CommonMark; pandoc/mdbook без plugin их не отрендерят. Митигация: zetto write умеет emit-ить markdown-стиль `[text](path.md)` опцией для export-pipelines; primary writing-format остаётся wikilinks.
- Render display-text fallback требует frontmatter scan target-а (O(N) cold). В v1 — синхронный scan; B1 (graph index) когда будет — заменит на index lookup.

## Explicitly not considered

| Variant | Reason rejected |
|---|---|
| **Org-mode `[[id:UUID][text]]`** | Иной экосистемный formats; CommonMark-несовместим; STRATEGY-выбор markdown в ADR-0002. |
| **Pure tag-graph (no link syntax, ссылки через теги)** | STRATEGY anti-pattern «теги ≠ замены ссылок»; полностью противоречит подходу. |
| **`[text](#ULID)` URL-fragment style** | Non-standard interpretation; `#ULID` обычно — anchor внутри текущего документа, не cross-doc reference. Сломает pandoc/mdbook. |
| **Free-form text resolved-at-runtime** (любая строка, матчится → линк) | Too magical; breaks WYSIWYG; STRATEGY-anti-pattern «продукт диктуется доказанными паттернами», free-form resolution — анти-паттерн в PKM-лите. |
| **Mediawiki-style `[[ID#section|display]]` с full anchor support в v1** | Это вариант C, см. выше. Отвергнут по сложности и не-нужности в v1. |

## После выбора альтернативы

После подтверждения Альтернативы A (или модификации):

- Phase 4 (Decide) → `2026-05-09-A2-decision-summary.md`, формализующий full link-syntax-контракт + concrete edge cases (broken-link rendering, embed-detection, external-URL detection, ULID-validation regex, render-fallback-priority).
- Phase 5 (Roast → Meta-review) — обязательно для deep cycle.
- Phase 6 (Document) → ADR-0003 + amendments в `decision-map.md` (A2 → decided), `ARCHITECTURE.md` §5 + §6 (Q2 закрыт), `docs/architecture/README.md` ADR index.
- A3 (Frontmatter convention) разблокирован полностью — A1+A2 теперь оба известны, frontmatter может быть зафиксирован.
