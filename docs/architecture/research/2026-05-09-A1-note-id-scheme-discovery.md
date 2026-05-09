# Discovery: A1 — Note ID scheme

- **Date**: 2026-05-09
- **Cycle scale**: deep
- **Decision-map ref**: A1 — на‑диске контракт, наибольший blast radius (блокирует A2/A3/A4/A5/B1/B2/C4/D1/D4)
- **Predecessor**: ADR-0001 (Project name)
- **Status**: round 1

## Проблема

Заметка в zetto — это plain markdown на диске. Прежде чем закрывать любые другие решения on-disk-контракта (A2 link representation, A3 frontmatter, A4 layout, A5 versioning), нужно ответить: *как заметка идентифицируется?* — чем именно одна заметка отличается от другой и как другая на неё ссылается.

Ответ диктует: имя файла, формат wikilink/markdown-link, поведение при rename, стабильность обратных ссылок, сложность парсера, скорость индексации, поведение в `ls`/`fzf`/git diff и эргономику любого UX-сценария, который касается ссылок.

## Силы и драйверы

1. **Стабильность.** ID должен переживать переименования, разделения, склейки заметок. Если ID привязан к заголовку — каждый retitle ломает обратные ссылки.
2. **Читаемость.** Терминальный пользователь ежедневно делает `ls`, `fzf`, `rg` по каталогу. UUID v4 (`550e8400-e29b-41d4-a716-446655440000`) визуально шумен; человеко-читаемый slug — полезен для мышечной памяти.
3. **Сортируемость.** Хронологическое или Luhmann‑иерархическое упорядочение в `ls -1` помогает «листать» граф без TUI.
4. **Бесколлизионность.** На одной машине, между машинами (sync через git), и в гипотетическом multi-author будущем (deferred) — ID не должны сталкиваться. Auto-generated формат (UUID/ULID) даёт это бесплатно; человеко-выбираемый — требует регистра.
5. **Краткость.** Линк `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]` визуально тяжёл против `[[zk1]]` или `[[on-fixed-ids]]`. Краткость влияет на читаемость самих заметок и на UX fuzzy-link-picker.
6. **Capture latency.** Из бюджета `ARCHITECTURE.md` §2.1: ID generation должен укладываться в <50 ms. Большинство ID-форматов укладываются в наносекунды — ограничение слабое.
7. **Парсер графа.** Формат ID определяет regex/grammar для extracting links. Чем более «свой» формат — тем больше работы для B2 (parser).
8. **Toolchain interop.** Имя файла должно дружить с `rg`, `fzf`, `xargs`, vim-плагинами. Никаких пробелов, спецсимволов, экзотики; `[a-z0-9-]` идеально.
9. **Migration cost.** Однажды выбранный ID-формат фиксируется через A5 (Format versioning). Bump — это переписывание всех файлов и всех ссылок. Хочется один правильный выбор.
10. **Anticipation D4 (Obsidian compat).** Если zetto будет читать существующий Obsidian-vault или ему будет читать его vault — Obsidian резолвит `[[Title]]` по заголовку, не по ID. Жёсткая ID-схема может закрыть путь к D4 = read-write.

## Связывающие ограничения

Из `STRATEGY.md`, `ARCHITECTURE.md`:

- **Plain markdown на диске.** Файл = заметка. Никакой проприетарной БД (anti-pattern в ARCHITECTURE §7).
- **Notes-as-code.** Файл должен «дружить» с git, `vim`, `rg`. Это исключает имена с пробелами, спецсимволами, нестандартной кодировкой.
- **Toolchain interop как track в STRATEGY.** Композиция с CLI-цепочкой важнее собственной фичевости.
- **Latency budget из ARCHITECTURE §2.1.** ID generation, parsing, lookup укладываются в свои бюджеты — слабое ограничение.
- **A5 (Format versioning) как замыкающий ADR группы A.** ID-схема становится частью замороженного публичного контракта `format-v1`. Изменение в `format-v2` требует migration tool.
- **Single-author / single-vault в v1** (`ARCHITECTURE.md` §7 anti-pattern, ideation rejection F3.5). Multi-author/multi-vault — deferred. ID не обязан быть глобально уникален в этой версии.
- **Anti-pattern: папки-как-таксономия.** ID-схема не должна полагаться на каталоги для разрешения.

## Прецеденты и сообщество (snapshot 2025–2026)

| Схема | Где | Сильное | Слабое |
|---|---|---|---|
| **Luhmann hierarchical** (`1a2b3c`) | Аналоговый Zettelkasten Лумана; некоторые ручные практики | компактен; передаёт структурную близость | бессмыслен в цифре с FTS; форум Zettelkasten называет timestamp-аналог «useless» |
| **Timestamp prefix** (`202605091230`) | neuron, ранний Roam, ручные практики | сортируется хронологически; auto-generated | визуально шумный; community-feedback явно негативный; ID без relational signal |
| **UUID v4** (`550e8400-e29b-...`) | прежний zetto (`metadata.rs`) | глобально уникален; collision-safe | нечитаемый; несортируемый; 36 символов; community «не использует в filename» |
| **ULID** (`01J9XQK7ZBV5G2D8X3K7C4M9P0`) | новые проекты дизайна 2024–2026 | time-prefixed (sortable); 26 символов; Crockford base32 (нет O/0/I/1 путаницы); collision-safe | всё ещё длинный; нечитаемый без подсказки |
| **Slug-only** (`on-fixed-ids`) | Foam, Logseq для отображения; Obsidian для resolving | максимально читаемый; ID = title | rename ломает все обратные ссылки; коллизии между slug-aмн |
| **Hybrid: ULID + slug** (`01J9XQK7ZB-on-fixed-ids.md`) | community ideation 2025–2026; рекомендация Phase-1 web research | ULID — стабильный канон, slug — эргономика для `ls`/`fzf`; rename только slug, ID константен | имя файла длинное; сложнее парсить |
| **Hybrid: UUID/ULID frontmatter + slug filename** | org-roam частично, Obsidian с UID-плагинами | filename полностью человеко-читаемый; ID живёт в frontmatter | две источника правды — синхронизация |
| **Dot-notation hierarchical** (`project.meeting.2024-05-01.md`) | Dendron (EOL Feb 2023) | передаёт структуру; читаемо в `ls` | rename верхнего уровня каскадирует; коммьюнити-приёмники не появились |
| **Content-hash** (`a3f5b9.md`) | Roam-related экспериментальные | стабилен относительно содержимого | меняется при любом edit — не ID, а версия |

**Прежний zetto (git до `293a1e4`)**: UUID v4 в frontmatter (`src/notes/metadata.rs`), filename = slug. Ссылок между заметками не было (или были рудиментарные) — то есть переходные коллизии не проверялись на практике.

**Web research Phase 1 ideation**:
- Сообщество 2024–2026 явно конвергирует на: **UUID/ULID во frontmatter + slug в filename**, либо **ULID-prefix + slug в filename** (где filename = `<ULID>-<slug>.md`).
- Timestamp в filename по-прежнему встречается, но **community-feedback негативный** («visually noisy», «no relational signal»).
- Луман-style ID — почти не воспроизводится в новых tool-ах; считается paper-era artifact.

**Ideation 2026-05-09 (Idea #5)**: предложил `01J9XQK7ZB-on-fixed-ids.md` (ULID-prefix + slug), уверенность 85%, complexity Low/Medium. Это сильный hint в одну сторону.

## Открытые вопросы

Чтобы продвинуться к Design (3 альтернативы), нужно зафиксировать ответы на следующие вопросы. Я даю свою рекомендацию по каждому, но окончательный ответ — за тобой.

1. **Канонический дом ID.** Где именно живёт «истина» — frontmatter, filename, или оба с одним из них как source of truth?
   - *Lean*: frontmatter как source of truth (стабилен через rename), filename как эргономическая проекция.
2. **Формат ID.** ULID / UUID v4 / timestamp / Luhmann / hybrid (ULID + slug в filename) / другое?
   - *Lean*: ULID. Sortable, Crockford base32, 26 символов, community-стандарт.
3. **Генерация ID.** Auto на `zetto new` / user-supplied / hybrid (auto + опциональный override)?
   - *Lean*: auto-only в v1; override как escape-hatch если когда-то понадобится.
4. **Семантика slug-а.** ID *включает* slug (как `01J9X-on-fixed-ids`) / slug — pure filename-эргономика, не часть ID / без slug-а вообще?
   - *Lean*: slug — pure эргономика, не часть ID. Линки используют ID; filename для `ls`/`fzf`.
5. **Cross-machine guarantee.** Vault-локальной уникальности достаточно / нужна глобальная (ULID/UUID дают её бесплатно)?
   - *Lean*: глобальная (ULID/UUID) — бесплатно, и снимает целый класс будущих болей при гипотетическом multi-vault.
6. **Поведение при retitle.** Title меняется → slug меняется (= filename меняется), ID константен / title меняется → ничего не меняется (slug залочен на момент создания) / title и slug всегда синхронны?
   - *Lean*: title → slug автоматически меняется (filename переименовывается), ID константен. Линки `[[ID]]` живут.
7. **Bundling A1+A2.** Решать A1 (ID) и A2 (link syntax) одним bundled ADR / двумя соседними ADR с явной взаимной ссылкой / только A1 сейчас, A2 откладываем?
   - *Lean*: два соседних ADR. A1 (этот) — ID-схема и storage; следующий ADR-0002 — link representation. Bundled делает один ADR неудобоваримо большим.

## Заметки

- Если q5 (cross-machine) ответ «vault-локально достаточно» — открывается дешёвая опция короткого Луман-style ID (`zk1`, `zk2`, `zkn-v1`...). Это вариант, упомянутый в ideation как F6.4 — three-letter human IDs. На бумаге привлекательно (memorable, ergonomic), но требует ручного выбора при создании.
- Если q4 (slug = part of ID) — то A2 (link syntax) сильнее ограничено: линки должны включать slug, и rename slug ломает ссылки. Поэтому slug-as-эргономика выигрывает.
- Anticipation D4 (Obsidian compat): чтобы оставить путь к D4 = read-write, link-syntax должен быть resolvable both ways. Это уйдёт в A2 (следующий ADR), но q4 здесь влияет: если slug — часть ID, то Obsidian резолвит slug, и совместимость возможна; если только ULID — нужен translation layer.
- Q3 lean (auto-only) согласован с STRATEGY metric `capture latency <5s` — пользователь не должен думать об ID при создании.

## Что выйдет из ответов

После того как вопросы 1–7 закрыты:

- Можно сформулировать **3 альтернативы** для phase Design — обычно это будут варианты вокруг выбора формата (ULID vs UUID vs Luhmann) и storage (frontmatter-canonical vs filename-canonical vs both-equivalent).
- Возможна phase **Research** между Discovery и Design, если ответы поднимут version-sensitive вопросы (например, текущее состояние ulid-rs или crockford-base32 крейтов).
- Phase Design → Decide → Roast → Meta-review → ADR (deep cycle).
