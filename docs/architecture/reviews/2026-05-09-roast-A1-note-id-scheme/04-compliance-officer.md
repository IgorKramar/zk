# Compliance-officer findings: A1 — Note ID scheme + filename layout

- **Target**: `docs/architecture/research/2026-05-09-A1-decision-summary.md`
- **Date**: 2026-05-09
- **Role**: compliance-officer (regulatory / privacy / security)
- **Disclaimer**: I am not a lawyer. The findings below identify questions a compliance or security reviewer would raise. Final regulatory determinations require qualified counsel.

## Summary

Артефакт описывает соло-инструмент, работающий с локальным git-репозиторием одного пользователя; крупные регуляторные рамки (GDPR, HIPAA, 152-ФЗ, PCI) к нему напрямую не применяются — нет data subjects кроме самого автора. `ARCHITECTURE.md` §4 это явно фиксирует: «Compliance — N/A». Тем не менее остаются гигиенические дыры по линии «filename как канал утечки»: ULID-timestamp в имени файла раскрывает время создания заметки при любой публикации/расшаривании; title-derived slug превращает sensitive title в имя файла, которое всплывает в `git log`, shell history, swap-файлах, индексах rg/fzf и опубликованных форках.

## Applicable regulations and standards

- **GDPR / 152-ФЗ / HIPAA / CCPA / PCI-DSS** — **не применяются**. Пользователь = data subject = controller = processor в одном лице. Нет третьих лиц, нет обработки чужих персональных данных, нет cross-border processing, нет cloud-зависимости (прямо запрещена в `ARCHITECTURE.md` §7).
- **Employer data classification policies** — могут применяться transitively, если пользователь ведёт в zetto рабочие заметки под NDA или в регулируемой индустрии. Это не регуляторный вопрос к проекту, но дизайнерский.
- **OWASP / общие принципы information hygiene для local-first tools** — рамка, в которой имеют смысл findings ниже.

## Findings

### C-1: ULID-timestamp в filename и frontmatter — раскрытие времени создания при любом расшаривании

**Category**: PII flow / metadata leakage

**The gap**: ULID embeds 48-bit ms-precision Unix timestamp в первых 10 chars Crockford-base32. Любой человек с открытой спецификацией ULID может декодировать timestamp из filename за секунды. Артефакт §«Канонический ID» подаёт sortability как *feature*, но не упоминает обратную сторону: при публикации заметок (блог-пост, public git repo, gist, передача файла коллеге, скриншот `ls`) каждое имя файла декодируется в timestamp с точностью до миллисекунды. Это раскрывает паттерны работы пользователя: когда он бодрствует, когда работает над чем, в какие даты накапливал заметки по конкретной теме.

**Where in the artifact**: §«Канонический ID» (sortability presented as benefit), §«Filename layout»; silent on metadata-leakage trade-off.

**Severity**: medium — не нарушение, но осознанный design choice, последствия которого не проговорены и который попадёт в `format-v1`, то есть отозвать нельзя без major-version migration.

**What would close this**: явная запись в артефакте — «ULID-timestamp в filename декодируется тривиально; пользователь публикующий vault принимает раскрытие creation-times. Документировать в README/format-v1 spec».

### C-2: Title-derived slug в filename — sensitive title попадает в git log, shell history, swap, fzf cache

**Category**: PII flow / leakage через filename surface

**The gap**: §«Slug normalization» описывает алгоритм title → slug, но молчит о surface-площади получившегося имени файла. Если пользователь создаёт заметку с title `Therapy session 2026-04-11 with Dr. Smith` или `NDA review — ACME Corp Q3 numbers`, slug-крейт честно превращает это в `therapy-session-2026-04-11-with-dr-smith.md` или `nda-review-acme-corp-q3-numbers.md`. Это имя файла далее всплывает в:

- `git log --name-only`, `git log --stat`, `git diff` — навсегда в истории репо.
- Shell history (`zetto open <filename>`, `vim <filename>`).
- Swap-файлах vim/nvim.
- fzf/rg ignore/cache, ctags, LSP indexes.
- Скриншотах/screen-share `ls`/`tree`.
- В remote-копиях git (origin, fork, mirror).

Артефакт §«Charset» обосновывает ASCII-only чисто tooling-аргументом, но не упоминает обратную аргументацию: ASCII-slug делает sensitive title *более* грепаемым.

**Where in the artifact**: §«Filename layout» / §«Slug normalization»; §«ID generation» step 3.

**Severity**: medium — для соло-инструмента это поведение пользователя, не bug. Но артефакт фиксирует это как поведение по умолчанию (`zetto new` всегда генерирует slug из title, нет opt-out), что создаёт failure mode «пользователь не знал, что title станет filename».

**What would close this**: одно из (а) опция в config `filename_slug: from_title | none` с дефолтом sensible; (б) явная нотация в README warning; (в) поддержка ULID-only filename как first-class option.

### C-3: Frontmatter `title:` хранится plain text — не проблема в isolation, но артефакт о ней не говорит

**Category**: PII at rest / encryption posture

**The gap**: §«ID generation» step 5 пишет frontmatter `title: <title>` через `serde_yaml`. Title и body заметки лежат plain text на диске. Это правильное проектное решение для plain-markdown-first продукта, но артефакт молчит о том, что *это* и есть encryption-posture: at-rest шифрование делегировано пользователю (FileVault/LUKS/dm-crypt/BitLocker), а не предоставляется приложением.

**Where in the artifact**: §«ID generation» step 5; §«Контракт публичного формата».

**Severity**: low — silent on, не wrong. Но в `format-v1` фиксируется plain-text frontmatter без opt-in encryption.

**What would close this**: одна строчка в артефакте: «encryption-at-rest = ответственность пользователя (FS-level); zetto хранит plain text. Любой механизм sync/share — отдельный ADR с encryption clauses».

### C-4: `git rm` заметки — content остаётся в git history; артефакт молчит о deletion semantics

**Category**: data retention / right-to-erasure analogue

**The gap**: Артефакт описывает rename, но не описывает семантику удаления заметки. Что происходит при `zetto delete <ULID>`, и что должен делать пользователь, чтобы реально стереть заметку? Стандартное `git rm <file>` + `git commit` удаляет файл из tip, но содержимое остаётся в history — навсегда восстанавливаемо. Если пользователь публикует репо и потом удаляет sensitive-заметку — она остаётся в git history. Дополнительно: ULID — глобально уникальный идентификатор, который теоретически можно использовать как correlation ID между публикациями.

**Where in the artifact**: silent — нет §«deletion» / §«lifecycle» в значении «note deletion semantics».

**Severity**: medium — прямой gap в lifecycle, которого артефакт обязан коснуться, потому что фиксирует ID-схему как `format-v1` контракт.

**What would close this**: либо отметить в §«Open questions» новый узел «note deletion semantics — отдельный ADR», либо добавить параграф «zetto не гарантирует удаление из git history; для полного стирания требуется `git filter-repo` / BFG; пользователь несёт ответственность».

### C-5: Slug-deunicode-pass и оригинал в frontmatter — false sense of «PII removed from filename»

**Category**: data minimization / consent UX

**The gap**: §«Slug normalization» документирует transliterate non-ASCII → ASCII (`Заметка о ULID` → `zametka-o-ulid`). Незадокументированный side-effect: пользователь может *думать*, что title не попал в filename. На самом деле transliterated form тривиально reversible через тот же deunicode-словарь, и оригинал title сохранён full-fidelity в frontmatter `title:`.

**Where in the artifact**: §«Slug normalization» — рядом примеры, но нет ноты «оригинал title сохраняется в frontmatter в полном виде».

**Severity**: low — это документация-фикс.

**What would close this**: explicit note: «slug — это UX-affordance для filename, не privacy-механизм. Title в frontmatter сохраняется full-fidelity и индексируется/грепается».

### C-6: Multi-device git sync — cross-machine clock-skew документирован, но fingerprint-leakage через ULID — нет

**Category**: PII flow / device fingerprinting

**The gap**: §«ID monotonicity и concurrency» описывает, что ULID на разных устройствах генерируются от собственных wall-clock с документированным skew-caveat. Но 80-bit random-часть ULID — это per-process random; через анализ распределения random-части в большой выборке можно определить число устройств, использовавшихся для ведения заметок.

**Where in the artifact**: §«ID monotonicity и concurrency».

**Severity**: low — теоретическая поверхность, для соло-use невидимая. Включаю как silent-on-finding только потому, что артефакт фиксируется в `format-v1`.

**What would close this**: ничего, кроме признания «device-fingerprint через random-распределение ULID существует, но не митигируется — это compatibility cost spec-compliant ULID».

## What's well-handled

- **`ARCHITECTURE.md` §4 явно объявляет «Compliance — N/A»**. Это правильная и честная позиция.
- **No cloud sync as dependency** — устраняет огромный класс privacy-проблем.
- **Plain markdown on disk** — minimization principle.
- **ID never changes after creation** — корректная inviolable-identifier semantics.
- **Single-author by design** — закрывает класс multi-author access-control вопросов.

## Areas not evaluated

- Что попадает в `zetto.config` или другие state-файлы — артефакт A1 их не касается.
- Frontmatter schema целиком — отложен в A3.
- Поведение при ошибках I/O / partial writes.
- Logging/telemetry stance — артефакт о нём не говорит вовсе.
