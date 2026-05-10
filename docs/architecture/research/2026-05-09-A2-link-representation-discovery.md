# Discovery: A2 — Link representation

- **Date**: 2026-05-09
- **Cycle scale**: deep
- **Decision-map ref**: A2 — link representation; mutually constrained с A1 (см. `decision-map.md` §Notes)
- **Predecessor**: ADR-0002 (Note ID scheme + filename layout)
- **Status**: round 1

## Проблема

ADR-0002 зафиксировал, что **identifier внутри линка — ULID**. Это однозначно отвечает «что» в линке, но не отвечает на «как» — какой синтаксис обёртки, что попадает в этот синтаксис помимо ULID, как обрабатывать display-text, embeds, anchor refs, block refs.

Этот ADR (будет ADR-0003) фиксирует **полный link-syntax-контракт** zetto: какие формы линка `zetto` распознаёт при парсинге, какие пишет при создании, и что делает резолвер при нарушенном/невалидном линке.

## Силы и драйверы

1. **Brevity** в исходной заметке. Markdown-стиль `[on fixed IDs](01J9XQK7ZBV5G2D8X3K7C4M9P0-on-fixed-ids.md)` визуально шумен; wikilink `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]` короче, но всё ещё 28 chars. Display-text-форма `[[01J9X|on fixed IDs]]` ещё компактнее.
2. **Стабильность через rename**. ADR-0002 даёт slug-rename, ULID константен — линки на ULID живут. Линки на filename / slug — ломаются после retitle.
3. **Toolchain interop**. Markdown-стиль `[text](path)` нативно работает в pandoc, mdbook, Hugo, любом CommonMark-renderer-е. Wikilinks — нестандартное расширение CommonMark; pulldown-cmark v0.10+ поддерживает wikilinks как opt-in extension; comrak — нет; статические site-генераторы (mdbook etc.) — частично, через плагины.
4. **D4 (Obsidian-vault compat)**. Obsidian primary syntax — wikilinks `[[Title]]` (резолвится по filename / aliases). Markdown-style links Obsidian тоже поддерживает. Решение в A2 определяет, насколько zetto-vault будет совместим в read-write-режиме D4.
5. **Render-time vs parse-time identity**. Если `[[ULID]]` рендерится как title — нужен lookup при render (frontmatter scan или index). Если рендерится как ULID literal — выглядит криво в preview. Display-text форма `[[ULID|text]]` снимает trade-off.
6. **Парсер сложность**. Обработка одного синтаксиса проще, чем двух. CommonMark-only — самый простой парсер (pulldown-cmark из коробки). CommonMark + wikilinks — pulldown-cmark + extension. CommonMark + wikilinks + embeds + anchors + block-refs — близко к Obsidian-style парсеру, существенно сложнее.
7. **External URL handling**. `[text](https://...)` — стандарт. `[[https://...]]` — невалидный wikilink в большинстве PKM-tools. Чёткое разделение «внутренний линк = wikilink, внешний = markdown» снимает ambiguity.
8. **Anchor / block-refs (forward feature)**. Anchor `[[ULID#Heading]]` требует heading-stable семантики (если пользователь меняет heading text — ссылка ломается). Block-ref `[[ULID#^para-id]]` требует отдельной auto-id-схемы для параграфов (Obsidian's `^xyz123` block IDs).
9. **Embeds (`![[ULID]]`)**. Включает контент другой заметки в текущую — в Obsidian/Logseq популярна; в pandoc / static-site-generator — не стандартно; реализация требует render-pass, который проходит по графу.
10. **format-v1 spec contract**. ADR-0002 §«Format versioning anchor» зафиксировал: «ID-rendering в линке — implementation detail, не public ABI». Это означает, что **link syntax можно эволюционировать без bump format-v1**. Но user-written контент в репозитории будет писаться этим syntax-ом — практически migration-cost существует, даже если spec формально позволяет менять.

## Связывающие ограничения (из STRATEGY / ARCHITECTURE / ADR-0001 / ADR-0002)

- **identifier в линке = ULID** (ADR-0002).
- **frontmatter `id:` canonical, filename — projection** (ADR-0002).
- **Plain markdown на диске** (ARCHITECTURE.md §7).
- **Anti-pattern «не переизобретать ripgrep/fzf/git»** — но markdown parser приходится иметь свой (выбран pulldown-cmark в ADR-0002 § Crate dependencies).
- **STRATEGY-метрика capture latency <5s** включает время linking (см. ARCHITECTURE.md §2.1: fuzzy-link picker <500 ms). Парсер должен быть быстрым.
- **Anti-pattern «folders-as-taxonomy»** — markdown-стиль линка не должен заставлять писать `../another-folder/X.md`-пути; либо absolute (от root vault), либо identifier-based.
- **A1 + A2 mutual constraint** — link syntax влияет на filename layout interpretation и обратно. ADR-0002 уже зафиксировал filename, A2 должен с ним согласоваться.

## Прецеденты и сообщество (snapshot 2025–2026)

| PKM-tool | Wikilink syntax | Display-text | Embeds | Anchors | Block refs | External URLs |
|---|---|---|---|---|---|---|
| **Obsidian** | `[[Title]]`, `[[ID]]` через aliases | `[[ID|text]]` | `![[ID]]` | `[[ID#Heading]]` | `[[ID#^block]]` | `[text](https://)` |
| **Logseq** | `[[Title]]` | нет alias-syntax (через property) | `{{embed [[Title]]}}` | `[[Title#Heading]]` | `((block-id))` (separate syntax) | `[text](https://)` |
| **Foam** | `[[Title]]`, `[[Title|alias]]` | `[[T|alias]]` | not in v1 | not in v1 | not in v1 | `[text](https://)` |
| **org-roam** | `[[id:UUID][text]]` (org-mode native) | вшит | `#+INCLUDE:` | `[[id:UUID::*Heading]]` | not native | `[[https://...]]` |
| **zk-org/zk** (Go) | `[[Title]]`, markdown links | `[text](path)` | not in v1 | not in v1 | not in v1 | `[text](https://)` |
| **Dendron** (EOL 2023) | `[[note.path]]`, `[[label|note.path]]` | `[[label|note.path]]` | embed via plugin | `[[note.path#heading]]` | `[[note.path#^block]]` | `[text](https://)` |
| **CommonMark / pandoc / mdbook** | not native | N/A | not native | через `<a name>` или anchors из headings | not native | `[text](https://)` |

Закономерности:
- **Wikilinks `[[X]]`** — de-facto стандарт PKM 2020+. Каждый PKM-tool, кроме pure-CommonMark generators, поддерживает wikilinks как primary internal-link syntax.
- **Markdown-style `[text](path)`** — universal fallback / external-URL standard.
- **Display-text** через `|`-разделитель — Obsidian convention, принят Foam, Dendron. Logseq использует другой механизм (property).
- **Embeds, anchors, block refs** — в Obsidian/Dendron stable; в Foam/zk-org/zk — отсутствуют в v1; в Logseq — отдельный block-ref-syntax.
- **`pulldown-cmark` v0.10+ (2025)** имеет нативную wikilinks extension, парсит `[[X]]` и `[[X|text]]` как `Tag::Link(LinkType::WikiLink, ...)`. Это снимает основной парсерный cost.

## Открытые вопросы

Чтобы продвинуться к Design (3 альтернативы + explicitly-not-considered), нужно зафиксировать ответы:

1. **Primary internal-link syntax.** Wikilinks `[[ID]]` / canonical markdown `[text](filename)` / оба распознаются параллельно?
   - *Lean*: wikilinks `[[ID]]` primary; canonical markdown поддерживается распознаванием при чтении (read), но НЕ генерируется zetto при создании (write). Это даёт «zetto-canonical → wikilinks», совместимость на чтение со старым контентом.

2. **Что внутри `[[ ]]`.** ULID literal / slug / filename без `.md` / mixed (resolve через любой из них)?
   - *Lean*: ULID literal только. ADR-0002 это уже зафиксировал.

3. **Display-text механизм.** `[[ID|display]]` (pipe-разделитель) / нет display-text (рендерится title из frontmatter) / через frontmatter `aliases:` поле?
   - *Lean*: `[[ID|display]]` опционально + render-time fallback на title из frontmatter, если display нет. Это даёт читабельность сразу при создании линка (display-text), и читабельность при render (auto-title), и стабильность ID если retitle.

4. **Embeds (`![[ID]]`)** в v1?
   - *Lean*: defer в v2. Не блокирующий feature; добавляет render-pass complexity.

5. **Anchor refs (`[[ID#Heading]]`) и block refs (`[[ID#^block-id]]`)** в v1?
   - *Lean*: defer оба в v2. Heading-stable семантика — отдельный contract; block-IDs — отдельная auto-id-схема. Не запрещаем синтаксис парсингом (parser просто не понимает `#`-suffix), но zetto не пишет такие линки сам.

6. **Resolver behavior** при `[[ID]]` где ID невалиден (не 26-char Crockford) или валиден но не найден в vault?
   - *Lean*: `zetto lint` flag-ит broken links явно (rule `zetto/no-broken-link`); render-time — линк остаётся `[[ID]]` literal без auto-correct; `zetto open <broken-ID>` — error, не silent-create.

7. **External URLs** — формат?
   - *Lean*: canonical markdown `[text](https://...)`. Wikilinks резервируются за internal-only. Чёткое разделение.

## Заметки

- A2 решает only **link syntax**; resolver-логика, индексация backlinks, эффективность fuzzy-link picker — это B1/B2/D1/C4. A2 их касается косвенно: если A2 = wikilinks, парсер B2 должен включать pulldown-cmark wikilinks extension.
- Q4/Q5 (embeds, anchors, block refs) — это «расширения» над core link syntax. Defer-стратегия даёт минимальный v1 без закрытия дверей. Если в 2027 окажется, что embeds — must-have, добавление `![[ID]]` parser-rule — единичный feature, не migration.
- Q3 (display-text) самый интересный с UX-стороны: display-text vs auto-title. Compromise — поддерживать оба, render выбирает по приоритету (display > auto-title > ULID literal).
- Q6 (resolver behavior) важен для STRATEGY-метрики orphan-ratio: «сирота» — это заметка без исходящих линков; «broken link» — линк, идущий в никуда. Эти две концепции должны быть разведены в lint-engine (C2a).
- ADR-0002 §Format versioning anchor: link syntax можно менять без bump format-v1. Но user content замёрзнет — миграция реальна. Делаем выбор раз; меняем только если новая форма строго лучше.

## Что выйдет из ответов

После закрытия Q1–Q7:

- 3 альтернативы для Design — обычно (A) wikilinks-only-primary с markdown-fallback, (B) markdown-only (CommonMark purist), (C) full-Obsidian-superset с embeds/anchors/blocks в v1.
- Возможно research-pass на pulldown-cmark wikilinks extension state в 2026 (если выбрана альтернатива A или C).
- Decide → Roast → Meta-review → ADR-0003.
