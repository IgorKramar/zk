# Meta-review: 2026-05-09-roast-A3-frontmatter-convention

- **Date**: 2026-05-10
- **Reviewer**: meta-reviewer (archforge plugin self-conformance role)
- **Scope**: 6 файлов в `docs/architecture/reviews/2026-05-09-roast-A3-frontmatter-convention/`
- **Plugin source-of-truth**: archforge 0.4.0-rc3 (`commands/roast.md` строки 80–124, 156–167)

## Summary

Roast-каталог в высокой степени соответствует шаблону: все 6 файлов на месте, prescribed английские заголовки (`## Headline findings`, `## Severity counts`, `## Cross-cutting concerns`, `## Recommended path`, `## Per-role outputs`) сохранены, finding-IDs следуют схеме `B-N / H-N / J-N / C-N / F-N / CC-N`, идентификаторы (crate names, ADR-NNNN, YAML-ключи, regulation names) не переведены, проза на русском с явной нотой terminology-pass в каждом per-role файле. Самая существенная расходимость — численная ошибка в severity-таблице summary (Compliance counts инвертированы) плюс несколько мелких template-расхождений в per-role файлах.

## Findings

### M-1 (medium): Severity-counts для Compliance-officer не совпадают с per-role файлом

**Where**: `00-summary.md` строка 21 (severity table) и строка 101 (per-role link).
**The divergence**: Таблица заявляет «Compliance-officer | 0 | 4 (estimated) | 2 (estimated)». Фактически в `04-compliance-officer.md`: C-1 low, C-2 medium, C-3 low, C-4 low, C-5 medium, C-6 low → **0 high / 2 medium / 4 low**. Числа инвертированы.
**Fix**: Заменить ячейку на `0 | 2 | 4` и убрать «estimated».

### M-2 (low): `01-devil-advocate.md` имеет дублирующийся `## Devil-advocate findings` заголовок

**Where**: `01-devil-advocate.md` строки 1 и 10.
**The divergence**: После terminology-нормализации (перл-pass H1→H2) файл начинается `## Devil-advocate findings`, затем содержит метаданные и `## Summary`, затем повторно `## Devil-advocate findings` на строке 10 (исходно был body-section в материале агента).
**Fix**: Удалить второй `## Devil-advocate findings` или заменить на `## Findings` нижним уровнем.

### M-3 (low): `03-junior-engineer.md` использует `## Clarity findings` вместо canonical

**Where**: `03-junior-engineer.md` строка 12.
**The divergence**: Файл правильно начинается `## Junior-engineer findings`, затем body-section называется `## Clarity findings` (синоним lens role-name, но не canonical identifier из `roast.md`).
**Fix**: Переименовать в `## Findings` нижним уровнем для консистентности с другими per-role файлами.

### M-4 (low): Headline-finding для Compliance-officer в summary не имеет finding-ID prefix

**Where**: `00-summary.md` строка 12.
**The divergence**: Все остальные роли в headline начинают с finding-ID: «Devil-advocate: B-1 — …», «Pragmatist: H-5 — …». Compliance-officer (строка 12) пропускает: «Compliance-officer: предупреждения о title-leak …».
**Fix**: Добавить prefix наиболее значимого finding'а — например, `C-2` (sensitive content в `x-*` без sanitize) или `C-5` (tags-leak unaddressed в Privacy-секции).

### M-5 (informational): Severity table cells `—` для Junior-engineer/Futurist разрешены

**Where**: `00-summary.md` строки 22–23.
**The conformance**: `roast.md` строка 109 явно разрешает omit для junior/futurist; explanatory note (строка 25) корректно ссылается. Conforms.

### M-7 (low): `02-pragmatist.md` terminology-pass note содержит «дiff» (mixed-script)

**Where**: `02-pragmatist.md` строка 107.
**The divergence**: terminology-pass note: «"дiff" оставлен исходно как "диф" в одном месте». Слово содержит латинскую `i` посередине кириллических букв. В body файла термин на самом деле появляется как `diff` или `диф` целиком.
**Fix**: В note заменить `дiff` на `diff` или `диф` (один из, не mixed).

## What conforms

- **Все 6 файлов присутствуют** per `roast.md` строки 71–76.
- **`00-summary.md` имеет все 5 prescribed-секций** verbatim english.
- **Per-role файлы (5 из 5) имеют canonical `## <Role>-findings` heading** на строке 1.
- **Finding-ID prefixes полностью соответствуют spec'у**: B-1..B-8, H-1..H-7 (correct H- per spec, не P-N), J-1..J-12, C-1..C-6, F-1..F-9, CC-1..CC-7. Latin form, не калькированы.
- **Идентификаторы не переведены**: `gray_matter`, `serde_yaml`, `pulldown-cmark`, `saphyr`, ULID, RFC 3339, GDPR, 152-ФЗ, ADR-0002/0003/0004, YAML keys (`tags:`, `aliases:`, `created:`, `updated:`, `id:`, `title:`, `x-*`), Obsidian, decision-map IDs.
- **Cross-references resolve**: B-N/H-N/J-N/C-N/F-N в summary находят соответствующие findings в per-role файлах.
- **Severity-counts честные**: 5 high из ~42 findings; компонентное распределение реалистично.
- **Recommended path** использует canonical «Apply findings, then proceed to Document».
- **Каждый per-role файл имеет terminology-pass note**, фиксирующую identifier-preservation.
- **Cross-cutting (CC-1..CC-7) ссылается на findings из 2+ ролей**.

## Areas not covered

- Substantive correctness конкретных findings (`gray_matter` coerce behavior, etc.) — роль `devil-advocate`/тестов.
- Cost estimates в pragmatist findings — роль `pragmatist`, не template-conformance.
- Compliance-content correctness — disclaimer на месте; финальное определение требует юриста.
- Future drift предсказания — содержательная роль `futurist`.
- Качество прозы — meta-reviewer не литературный редактор.
