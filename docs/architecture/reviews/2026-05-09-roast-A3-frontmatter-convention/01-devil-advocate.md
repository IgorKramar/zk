## Devil-advocate findings

**Target**: `/Users/user/Work/self/zk/docs/architecture/research/2026-05-09-A3-decision-summary.md`
**Date**: 2026-05-10

## Summary

Решение содержит две тяжёлые системные дыры. Первая — `updated`-через-content-hash требует постоянного state (last-known hash), который не определён где живёт; без него algorithm недетерминирован при первом save после внешнего edit. Вторая — hand-rolled write при наличии preserve-verbatim полей (unknown, `x-*`) фактически невозможен без round-trip через парсер: zetto обязан re-emit unknown ключи, и для этого ему нужен YAML-emitter с теми же edge-cases, которые `gray_matter` теряет. Проект записал «hand-rolled simple template» там, где нужен полноценный YAML serializer.

## Devil-advocate findings

### B-1: «Update iff hash changed» недетерминирован при первом save после внешнего edit
**Type**: hidden assumption / failure mode

**The attack**: Pattern «compute content-hash тела, compare с last-known hash, update `updated` iff changed» (research §6, summary §Standard optional `updated:`) предполагает существование «last-known hash». При первом save заметки после `git pull`, после внешнего редактирования (Obsidian, vim напрямую), после restore из backup — last-known hash либо отсутствует, либо stale. Что zetto делает? Если consider-as-unchanged → `updated` врёт (тело изменилось внешне, timestamp не двинулся). Если consider-as-changed → каждый pull генерирует frontmatter-only churn в git. Документ не выбирает.

**Where in the artifact**: decision-summary §Standard optional fields, `updated:` bullet — упоминает «last-known hash» без указания, где он живёт. Privacy section §`updated:` content-hash рекомендует «cache hash в external state (sqlite в B1), не в frontmatter» — но B1 ещё open ADR. До B1 zetto не имеет external state, и frontmatter-cache явно отвергнут. Поведение в v1 не зафиксировано.

**Severity**: high (frontmatter-only diff churn в git OR systematically wrong `updated` timestamps — выбор между двумя bad outcomes делается каждый save).

### B-2: Hand-rolled write требует полноценного YAML emitter, который ADR называет «template»
**Type**: logical inconsistency

**The attack**: Decision говорит «zetto собирает YAML строкой через шаблон» (§Write strategy) и одновременно требует preserve unknown поля verbatim в alphabetical order. Preserve-verbatim для произвольного YAML значения — это roundtrip serialize, не «template». Что zetto эмитит для `mood: {complex: nested, list: [1, 2, 3]}`? Multi-line scalars (`|`, `>`)? Anchors/aliases (`&foo` / `*foo`)? Unicode-escapes? Ключи с пробелами? Если zetto re-emit-ит через `serde_yaml::to_string` — теряется verbatim contract (re-formatting). Если хранит raw substring — нужен парсер с position info, которого `gray_matter` не предоставляет. «Hand-rolled template» как технология этот кейс не покрывает.

**Where in the artifact**: §Write strategy утверждает «hand-rolled fixed order» и одновременно «unknown без prefix in alphabetical order, preserved verbatim». Эти два требования противоречат друг другу для non-trivial YAML значений.

**Severity**: high (либо unknown поля silently re-formatted (нарушение preserve-verbatim contract), либо zetto падает на nested YAML, либо требуется YAML library, которая в ADR не выбрана).

### B-3: Concurrent write — два процесса zetto обнуляют друг друга
**Type**: concurrency bug

**The attack**: Pre-alpha CLI/TUI — пользователь может запустить `zetto retitle X` в одном терминале и иметь TUI открытый в другом, или vim открытый параллельно. zetto читает файл → парсит → строит новый frontmatter → пишет. Между read и write нет file lock, нет CAS-проверки на mtime/hash. Параллельный процесс (или Obsidian, или ручной vim-save) перезаписывает файл; zetto-write затирает изменения. На Linux/macOS file write не атомарный — partial write оставляет corrupt frontmatter (отсутствующий closing `---`).

**Where in the artifact**: §Write strategy полностью молчит о concurrency, atomic write, file locking. Edge cases section не упоминает.

**Severity**: high (потеря пользовательских правок, corrupt files в случае partial write).

### B-4: `aliases:` resolver — case-insensitive collision DOS
**Type**: adversarial scenario / edge case

**The attack**: §`aliases` resolver behavior говорит «case-insensitive по умолчанию» и «при multiple matches lint flag + zetto предпочитает первый по ULID-creation-time». Vault в 5000 заметок с user, который ввёл `aliases: ["TODO"]` в каждой третьей заметке (легко происходит при import). Каждый `[[TODO]]` wikilink лезет в alias-scan (synchronous per ADR-0003 без B1), находит ~1700 matches, вычисляет ULID-creation-time-min на all из них. Linter в `zetto check` запускается над всем vault → 1700 lint-warnings на каждой заметке × 5000 заметок = 8.5M warnings в output. Терминал, диск, retention лога, всё ломается. Наивная реализация O(N²) без B1 — это часы на check-pass.

**Where in the artifact**: §`aliases` resolver behavior; B1 в open questions без даты. ADR не вводит никаких ограничений на длину aliases-array, на total alias count, на N collision threshold.

**Severity**: medium (degraded `zetto check`, может перейти в high при vault > 1000 заметок).

### B-5: `tags: tag1` scalar coercion — Obsidian и `gray_matter` расходятся
**Type**: hidden assumption / edge case

**The attack**: §Edge cases говорит «`tags: tag1` (scalar вместо list) — `gray_matter` обычно coerce-ит scalar to single-element list». «Обычно» — это не guarantee. Behavior зависит от того, как `gray_matter` (через `serde_yaml`) обрабатывает scalar при ожидании `Vec<String>`. По стандартному `serde_yaml` поведению — это **type error**, не coercion: `Vec<String>` deserialize из scalar `"tag1"` падает. Тогда zetto получает parse error, заметка с `tags: tag1` (написанная Obsidian-пользователем, который пропустил bracket-syntax) refuses mutate. Alternative: zetto получает silent coerce, но Obsidian intepretирует тот же текст как раздельный токен `"tag1"`. Behavior fork непредсказуем.

**Where in the artifact**: §Edge cases, `tags: tag1` row. ADR полагается на «обычно» вместо тестового evidence от конкретной версии `gray_matter` 0.2.

**Severity**: medium (interop-break с Obsidian-vault, в котором scalar tags исторически приняты).

### B-6: Дублирующиеся ключи — last-wins ломает round-trip и open security door
**Type**: edge case / adversarial scenario

**The attack**: §Edge cases говорит «два `tags:` — `gray_matter`/`serde_yaml` обычно takes last; zetto не fix-ит, lint flag warn». Это означает: пользователь видит в файле два `tags:` блока, lint warn говорит «duplicate». Пользователь делает `zetto retitle` — zetto перезаписывает frontmatter в canonical form. Что emitted? Только last из двух (per parse) или only first? Discovery silent. Какой бы выбор не был — пользовательские tags из other блока silently dropped. Adversarial vector: import-tool генерирует два `tags:` — один с публичными, второй с приватными; zetto-write дропает один → leak public-only OR обнаружение приватных (зависит от order). Lint warn недостаточен — это data loss.

**Where in the artifact**: §Edge cases, duplicate keys row.

**Severity**: medium (silent data loss tags-данных при retitle/save).

### B-7: `update_timestamp: off` + `x-skip-updated: true` — конфликт policy
**Type**: logical inconsistency

**The attack**: Decision определяет два orthogonal opt-out: vault-level config-knob `update_timestamp: auto | manual | off` и per-note `x-skip-updated: true`. Что побеждает в комбинации `update_timestamp: auto` + `x-skip-updated: true`? Очевидно note-override. Но `update_timestamp: manual` означает «пользователь сам ставит updated» — что делает `x-skip-updated: true` поверх manual? «Manual but не пиши»? «Manual но zetto не trigger reminders»? Обратное: `update_timestamp: off` + отсутствие `x-skip-updated` — старое значение `updated:` сохраняется, или удаляется? Спецификация описывает opt-out примитив, но не их semantics в комбинациях.

**Where in the artifact**: §Standard optional fields, `updated:` bullet.

**Severity**: low (annoyance, разные пользователи получат разное поведение, но не data loss).

### B-8: ULID regex принимает невалидные timestamps
**Type**: edge case

**The attack**: `^[0-9A-HJKMNP-TV-Z]{26}$` (§Required fields, summary `id`) валидирует Crockford-base32 charset и длину. Не валидирует, что decoded timestamp находится в pre-defined ULID range (timestamp ≤ 2^48-1 ms ≈ year 10889). Lint rule `zetto/invalid-id-format` упоминает «out-of-range timestamp» в скобках, но regex этого не enforce — нужен decode-step. Hand-crafted ULID с first character `8` (или выше) парсится regex как valid, но decoded timestamp overflow-ит u48. ULID library поведение в этом случае не специфицировано (panic / wrap / error). Если zetto использует regex как primary check, timestamp-extract (для ADR-0002 Folge-sequence) работает на «valid» ULID и возвращает мусор.

**Where in the artifact**: §Required fields, lint rule `zetto/invalid-id-format`. Regex и semantic check (out-of-range) перечислены вместе, но это два разных шага.

**Severity**: low (low real-world frequency, но open для adversarial inputs из imports).

## Strongest single attack

**B-1**: «Update `updated` iff content-hash changed» — это algorithm с памятью, для которого decision не определяет, где память хранится. Каждый пользовательский pull / external edit / restore-from-backup поставит zetto в позицию «должен ли update timestamp». Без зафиксированного state-storage zetto будет либо мусорить git frontmatter-only diff-ами, либо систематически lying в `updated:`. Это в самом core promise standard optional field.

## Gaps in your own analysis

- **`gray_matter` 0.2 actual behavior** на scalar-vs-list, duplicate keys, multiline scalars не verified от source — только community claims в research digest. Нужен exec-test перед finalize, я не могу запустить.
- **`x-content-hash` field уже упомянут как «не в frontmatter»** в privacy section, но `x-skip-updated` упомянут как `x-*` extension — coherence между этими двумя examples не атаковал глубже.
- **Folgezettel `prev`/`next` (отложены в B-альтернативу)** — не атакую, потому что они вне scope текущего ADR. Если будут добавлены в format-v1.x — это отдельный roast.
- **C2a rule engine semantics** — все 6 lint rules имеют «error/warn/info» severity, но decision явно ссылается «семантика — в C2a (open)». Атаковать undefined в C2a не имеет смысла.

Terminology pass: применён к prose; identifiers (ULID, zetto, ADR-0002/0003/0004, `gray_matter`, `serde_yaml`, `x-*`, `tags:`, `aliases:`, `created:`, `updated:`, finding IDs `B-1..B-8`) сохранены.
