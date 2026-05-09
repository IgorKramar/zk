# Research digest: Rust ecosystem for ULID + slug + frontmatter (zetto A1)

- **Date**: 2026-05-09
- **Cycle**: A1 (Note ID scheme), deep
- **Source**: archforge:researcher agent, 13 web queries
- **Status**: input для Phase 3 Design

## Headline finding

Recommended baseline: **`ulid` 1.2.1 (dylanhart) + `slug` (Stebalien, deunicode-backed) + `gray_matter` для read / `serde_yaml` для round-trip write + `std::fs::rename` для slug-rename + `atomic-write-file` для записи содержимого**. ULID, Crockford encoding и кириллический slugging — well-served и стабильны; открытые риски — (a) Windows rename-atomicity edge cases, (b) deunicode читает кандзи как путунхуа-пиньинь, и (c) **отсутствие установленного Rust-PKM прецедента** ровно с этим filename-pattern (`<ULID>-<slug>.md` + ULID канонический в frontmatter). Каждый компонент по отдельности зрелый.

## Summary by topic

### 1. ULID-крейт

`ulid` 1.2.1 (dylanhart, март 2025) — канонический выбор: 16,6M downloads, три релиза в начале 2025, monotonic-API через `Generator::generate_from_datetime` (инкрементирует случайную часть в той же миллисекунде, на overflow — error). Optional features: `serde`, `uuid`, `rkyv`, `bytes`, `rand` (^0.9). Альтернативы (`rusty_ulid` — стейл с 2023; `mr_ulid` — sacrificial monotonicity; `ferroid` — 288M ULIDs/s, для backend-ID-сервисов, overkill для CLI) не нужны.

**MSRV ulid 1.2.1**: не выявлен в search; проверить `Cargo.toml` напрямую при пиннинге. Вероятно ≥1.65.

### 2. Crockford base32

`ulid` крейт **тащит свой Crockford-encoder внутри** — ULID-spec не RFC-4648 + Crockford alphabet, generic encoder выдаст неправильные строки (`063...` вместо `01G...`, см. `ulid/spec` issue #81). Standalone `crockford` или `fast32` крейты не нужны при использовании `ulid`.

### 3. Slug normalization

`slug` (Stebalien, depends on `deunicode ^1`) — канонический: `[a-z0-9-]`, никаких leading/trailing `-`, никаких double `-`. Поведение по языкам:

- **Кириллица**: deterministic char-by-char. «Заметка» → `zametka`. Сильный fit для русско-язычных vault-ов.
- **Кандзи (китайский/японский Han)**: explicit limitation — мапятся на путунхуа-pronunciation, illegible для японского читателя. «猫» (кошка) даст путунхуа-чтение, не japanese. Для японско-тяжёлых vault-ов нужен `kakasi`/`lindera`; для смешанных с редкими японскими — `slug` приемлем с задокументированной caveat.
- **Хирагана/катакана**: «ノート» → `noto`-style romaji.
- **Эмодзи**: `🦄☣` → `unicorn-face-biohazard` → после trim в `slug` → `unicorn-face-biohazard`. Конкретно `🔥` → `fire`.
- **Unknown chars**: `deunicode` выдаёт `[?]`-placeholder, `slug` его убирает. Title целиком из неподдерживаемых символов → пустой slug. **Edge case: empty slug — нужен fallback (filename = только ULID).**
- License: MIT/Apache-2.0.

### 4. Frontmatter parsing

`gray_matter` — самый идиоматичный 2025–2026 выбор: Rust-port js `gray-matter`, поддерживает YAML/JSON/TOML, custom delimiters, `parse_with_struct::<T>` для прямого serde-десериала. Используется `mdbook-frontmatter`. Maintainers: Hanchin Hsieh + Knut Magnus Aasrud.

**`pulldown-cmark-frontmatter` (Khonsu Labs) — НЕ подходит**: требует frontmatter в fenced code-блоке (` ```yaml `), а не `---`-delimiter — это **ломает Obsidian/Logseq/Foam interop**. Учитывая D4 (Obsidian-compat в decision-map), pulldown-cmark-frontmatter — wrong tool.

`serde_yaml` + manual `---`-detection — работает, но переизобретает парсинг и спотыкается на edge cases (Windows CRLF, BOM, blank lines). Reading через `gray_matter`, writing — `serde_yaml` для serialization frontmatter-struct, body — opaque text concatenation после.

**Caveat:** `gray_matter` write-back fidelity (preservation comments / key-order) — likely lossy. Если потом важна сохранность пользовательских комментариев в frontmatter — switch на YAML AST (`saphyr`, `yaml-rust2`). Phase-2 risk.

### 5. Filesystem rename semantics

`std::fs::rename` — cross-platform, **Windows — foot-gun**: atomic NTFS rename через `FILE_RENAME_POSIX_SEMANTICS` исторически использовался непоследовательно; PR #138133 (2025) пропатчил stdlib пробовать `FileRenameInfo` сначала, fallback на `FileRenameInfoEx` для Windows Server NTFS. Старее Windows 10 1607 → `ERROR_INVALID_PARAMETER`. **`std::fs::rename` overwrites target если он существует** — потенциальный hazard для zetto, если slug-collision возможен.

`renamore::rename_exclusive` — atomic non-overwriting rename (Linux glibc≥2.28 через `renameat2`, Windows-аналоги). Для slug-rename — приемлемо `std::fs::rename` (collision при разных ULID-префиксах vanishingly unlikely); для body-edits — `atomic-write-file` (write-temp-then-rename).

**Git rename detection**: git **не хранит rename-ы**, восстанавливает их из delete+add по 50% similarity (configurable `-M<n>`). Slug-rename без body-edit → 100% similarity → exact-rename fast-path с git 2.31. Работает одинаково с `git mv` и filesystem rename + `git add -A`. Failure-mode: rename + heavy edit в одном коммите. Совет — slug-rename отдельным коммитом или полагаться на `git log --follow`.

### 6. Empirical precedent для `<ULID>-<slug>.md`

**No findings.** Surveyed Rust PKM tools: `terror/zk` (Obsidian-style), `settle` (Obsidian-compatible), `zettelkasten-cli` (July 2025), `Trangar/zettelkasten` — **никто публично не документирует ULID+slug в filename + ULID канонический в frontmatter**. Большинство — либо timestamp-IDs (`20250915064516`), либо filename-as-ID (Obsidian-convention). Closest analog — Obsidian-плагины с stable-ID-prefix.

**Implication:** zetto делает defensible но не well-trodden путь. Upside: prior-art не противоречит. Obsidian-экосистема примет layout fine — Obsidian use filename как link-target по умолчанию, но поддерживает frontmatter-aliased links через `aliases:` field.

### 7. ULID sortability — confirmation

Подтверждено: первые 48 бит — millisecond Unix epoch (UTC), big-endian, Crockford base32 lexicographic order = numeric order. **String-sort = chronological sort**. Edge cases:

- **DST**: irrelevant (Unix epoch — UTC).
- **Wall-clock backward drift** (NTP correction): monotonic generator выдаёт ID с прошлым timestamp + incremented random, держит strict monotonicity ценой tiny "future"-drift.
- **Counter overflow в одной ms**: spec mandates fail на 80-bit-random overflow. Expected ULIDs/ms before overflow ≈ 2^40. Для CLI — non-issue.
- **Year-2086 / year-10889**: irrelevant.
- **Cross-machine ordering**: ULID timestamps от wall-clock каждой машины → между машинами с skewed clocks порядок «approximately chronological, modulo per-device clock skew». Документированный caveat.

## Caveats (для ADR consequences)

- `ulid` 1.2.1 MSRV не верифицирован; проверить перед pin.
- `slug` crate latest version не зафиксирован в search; стабильность подразумевает low churn, не abandonment. Verify при adoption.
- `gray_matter` write-back fidelity unverified — likely lossy для комментариев и порядка ключей. Phase-2 risk.
- Mixed-language slug (русский + японский + emoji в одном title) → может дать ugly mix путунхуа-piiнyin + кириллицы. Тест перед commit; consider config-knob для fallback на ULID-only filename при low slug-quality.
- `zettelkasten-cli` (July 2025) не inspected directly — если использует ULID+slug, prior-art стоит проверить.

## Cite

См. полный список из 30 источников в выводе researcher-агента (transcript run `bb8a9081`-equivalent). Ключевые:

1. `crates.io/crates/ulid` (1.2.1, март 2025)
2. `github.com/ulid/spec` issue #81 — Crockford encoding mismatch
3. `crates.io/crates/slug` + `kornelski/deunicode`
4. `crates.io/crates/gray_matter`
5. `rust-lang/rust` PR #138133 (2025) — Windows atomic rename fix
6. `crates.io/crates/renamore` + `crates.io/crates/atomic-write-file`
7. `git-scm.com/docs/git-diff` — rename detection
8. `byteaether.github.io/2025/...` — ULID reliability post
