## Compliance-officer findings

**Target**: `/Users/user/Work/self/zk/docs/architecture/research/2026-05-09-A3-decision-summary.md` (станет ADR-0004)
**Date**: 2026-05-10
**Disclaimer**: я не юрист. Находки ниже — вопросы, которые поднял бы compliance/security-ревьюер. Окончательные регуляторные определения требуют квалифицированного консультанта.

## Summary

zetto — personal-CLI с явным compliance-out-of-scope statement в `ARCHITECTURE.md` §4 (single user, plain text в user-owned git repo, никаких third-party data flows, никакой обработки чужих PII). Применимых юрисдикционных рамок (GDPR/152-ФЗ/HIPAA/CCPA) **нет**, потому что нет data subjects, отличных от самого пользователя; нет controller/processor-отношений; нет cross-border transfer; нет third-party providers внутри scope. Регуляторная поверхность отсутствует. Однако в схеме A3 есть несколько **leak-channel вопросов** на уровне «если пользователь однажды опубликует/расшарит vault или одну заметку», которые ADR должен документировать как пользовательские caveats — это не compliance issue в строгом смысле, но это категория, которой compliance-роль занимается, поэтому findings ниже формулируются именно так.

## Applicable regulations and standards

Учитывая зафиксированный scope (single user, локальный git repo, no network sync zetto-side, no third-party processing):

- **GDPR / 152-ФЗ / HIPAA / CCPA / LGPD / PIPEDA / PDPA** — **не применимы**. Нет data subjects вне самого пользователя; пользователь как data controller своих собственных заметок не подпадает под эти регуляции (household-exemption / personal-use carve-outs).
- **SOC2 / ISO 27001 / PCI-DSS** — не применимы, нет corporate context.
- **Export-control / dual-use** (EAR/Wassenaar) — не применимы; zetto не содержит криптографии beyond standard library (никаких хешей в A3 не реализуется самим zetto helper-ом для пользователя, content-hash для `updated` обсуждается только как mechanism — см. C-4).

Регуляторная экспозиция этой ADR — **нулевая в текущем scope**. Findings ниже относятся к категории «privacy hygiene при будущем opt-in share-сценарии», не к compliance violation.

## Findings

### C-1: Aliases как leak channel альтернативных идентичностей

**Category**: PII flow

**The gap**: §«Standard optional fields» вводит `aliases: [<string>, ...]` как user-controlled список альтернативных названий заметки. Пользователь может ввести туда code-names, nicknames, pseudonyms, бывшие имена людей, кодовые названия проектов под NDA, alias-ы клиентов. При share одной заметки наружу (export, gist, paste) frontmatter c `aliases:` уносится **верблюдом** — title leak channel из ADR-0003 §Privacy CC-2 расширяется N-кратно: каждый alias — независимая утечка ассоциации «эта заметка известна также как X». Privacy-секция A3 это упоминает («те же caveats как `title:`»), но не отмечает специфический риск **множественных** альтернативных идентичностей в одном поле, что качественно отличается от одиночного `title:`.

**Where in the artifact**: §«Standard optional fields» (определение `aliases:`); §«Privacy and security considerations» bullet 3.

**Severity**: low (nothing in current scope — single user, локально; гипотетическая утечка при будущем share).

**What would close this**: добавить в Privacy-секцию ADR-0004 одно предложение: «`aliases:` накапливает множественные альтернативные human-readable формы; при share одной заметки публикуются **все** alias-ы, а не только canonical title. Future export-mode (отдельный ADR) должен предложить redaction policy конкретно для `aliases:` field — например, сохранять только alias, использованный в текущем link-context».

### C-2: x-* preserve-verbatim не предупреждает о хранении секретов

**Category**: PII flow / data minimization

**The gap**: §«Custom user/extension fields» определяет `x-*` как «preserved verbatim, никогда не lint-flag-ятся». Privacy-секция содержит одну строку «Custom `x-*` fields — пользователь несёт ответственность; zetto не sanitize». Это формально корректно, но недостаточно: пользователь ergonomically воспримет `x-*` как «private namespace для моих данных» и может туда положить API-tokens (`x-openai-key`), credentials, медицинские identifier-ы, NDA-материалы. zetto не sanitize при export, не warn-ит при `zetto lint`, не выводит при `zetto info` predicate-list «эти поля начинаются с `x-` — они уйдут наружу при любом share». Compliance-аудитор личного vault-а (если когда-либо понадобится — например, при увольнении, при litigation-hold от employer-а) обнаружит, что `x-*` поля были фактическим scratch-pad-ом для секретов.

**Where in the artifact**: §«Custom user/extension fields»; §«Privacy and security considerations» bullet 4 (упоминание есть, но slim).

**Severity**: medium (вероятный паттерн использования, который ADR не предотвращает и не помечает).

**What would close this**: явный документированный warning в ADR-0004 + предложить будущий lint rule (резервирование имени, не семантика — semantics in C2a) `zetto/x-field-resembles-secret` (info severity, эвристика на имена `*-key`, `*-token`, `*-secret`, `*-password`, `*-credential`, длина значения, entropy). Альтернатива: явно зафиксировать «`x-*` поля являются частью plain-text заметки и не имеют privacy-семантики; для секретов используйте OS keychain / `pass` / 1Password CLI» — текстом, в Privacy-секции.

### C-3: Aliases-resolver — implicit cross-note read access

**Category**: authn-authz / trust boundary

**The gap**: §«`aliases` resolver behavior» определяет: «scan frontmatter `aliases:` всех заметок vault-а через index (B1) или synchronous scan в v1». Это **read-all-notes-frontmatter-on-resolve**: каждый раз, когда zetto резолвит `[[X]]` через alias-pass, он читает frontmatter каждой заметки в vault-е. В single-user scope это zero-cost от compliance-perspective. Но если vault когда-либо разделяется (multi-author shared vault, team Zettelkasten — отвергнутый сценарий per `STRATEGY.md`, но `decision-map.md` содержит D4), это становится authorization question: «может ли пользователь A выполнить link-resolve, который читает frontmatter заметок пользователя B?». ADR не помечает aliases-resolver как «опирается на zero ACL boundary» и не фиксирует, что при изменении этого допущения резолвер придётся перепроектировать. ADR-0002 §Open questions упоминает «multi-author author-attribution» как F-5 — родственный вопрос; A3 его косвенно затрагивает, но не cross-references.

**Where in the artifact**: §«`aliases` resolver behavior»; §Open questions B1.

**Severity**: low (multi-author явно out-of-scope в STRATEGY; finding профилактический).

**What would close this**: одно предложение в Privacy/security-секции: «alias-резолвер читает frontmatter всех заметок vault-а; в single-user scope это безопасно. Если когда-либо рассматривается multi-author shared vault (out-of-scope STRATEGY), alias-резолвер требует переоценки ACL — frontmatter `aliases:` не может рассматриваться как public-readable для всех authors».

### C-4: Content-hash в frontmatter — отвергнутый paths, но не запрещён explicitly

**Category**: PII flow / data minimization

**The gap**: §«updated» описывает паттерн «compute content-hash тела (без frontmatter), compare с last-known hash». Privacy-секция указывает «**Recommendation**: cache hash в external state (sqlite в B1), не в frontmatter». Это **recommendation**, не binding decision — `x-*` fields preserve-verbatim, и если пользователь (или extension plugin) положит `x-content-hash: <64-hex-chars>`, schema его примет. 64-char hex (предположительно SHA-256 или blake3) тела заметки — это **fingerprint, который при vault-share позволяет third party verify, что у них имеется именно эта версия body**, без раскрытия body. Это canonical-correlation surface: если две версии vault-а опубликованы (например, разные ветки git), content-hash в frontmatter позволяет корреляцию заметок между ветками даже после rename, retitle, частичной редакции metadata. Для single-user scope нет угрозы; для гипотетического share — это plus correlation. ADR это упоминает в Privacy bullet 5, но как «загрязняет user-visible data», не как correlation-surface.

**Where in the artifact**: §«updated» (mechanism); §«Privacy and security considerations» bullet 5.

**Severity**: low (recommendation against уже есть, риск гипотетический).

**What would close this**: усилить recommendation до **decision**: «zetto **никогда не записывает** content-hash в frontmatter; cache живёт исключительно в external state (sqlite/B1). `x-content-hash` явно reserved-name и lint-flag-ится `zetto/reserved-x-name` (warn) если пользователь вручную добавил». Альтернатива: оставить как recommendation, но зафиксировать в Privacy-секции одну строку про correlation-surface specifically.

### C-5: Tags могут содержать sensitive labels — нет mention в Privacy

**Category**: PII flow

**The gap**: §«Standard optional fields» определяет `tags: [<string>, ...]` как «facets». Не указано никаких ограничений на содержимое tag-string. Пользователь может положить туда `medical`, `legal-NDA-acmecorp`, `financial-tax-2026`, `mental-health-therapy`, имена клиентов, диагнозы, имена ответчиков по делам. Privacy-секция A3 **полностью молчит** про tags — упомянуты `title`, `created`, `aliases`, `x-*`, `updated/content-hash`, но не `tags`. Tag-pane (если C3 introduces TUI tag view) — самый частый share-vector в PKM (скриншоты, презентации «как я организую заметки»). Категория тегов часто sensitive именно потому, что отражает таксономию пользователя, не контент. ADR-0002/0003 Privacy-секции тоже не обсуждают tags (они там не введены), так что A3 — единственное место, где tags формально появляются как часть schema.

**Where in the artifact**: §«Standard optional fields» (`tags`); §«Privacy and security considerations» — silence.

**Severity**: medium (tags объявлены здесь впервые в schema, и privacy-perspective не зафиксирован; будущие документы будут наследовать эту тишину).

**What would close this**: добавить в Privacy-секцию ADR-0004 bullet: «`tags:` array — user-controlled facet labels; могут содержать sensitive labels (медицинские, юридические, финансовые категории). При share одной заметки tags уносятся в frontmatter; при скриншотах TUI tag-pane — visible. Zetto не sanitize и не классифицирует. Тот же caveat что для `title:`, помноженный на N тегов».

### C-6: Updated timestamp — work-pattern leak surface не указан

**Category**: PII flow

**The gap**: §«updated» — auto-managed RFC 3339 UTC timestamp. Privacy-секция упоминает `created:` (creation-time leak duplicate ULID), но **не упоминает `updated:`**. `updated:` — качественно другая утечка: серия `updated:` значений по vault-у (после share или git-history exposure) даёт **временной профиль работы пользователя** — когда пользователь активен (часы, дни недели), какие темы редактируются вместе во время одной сессии (correlated `updated:` timestamps в один день/час), периоды отпусков и болезни. Для single-user-локально — нет утечки; для git-published vault (open-source second brain, что является одним из patterns в Zettelkasten community) — это behavioral analytics surface. Также: content-hash gating (update timestamp iff body changed) **не покрывает** frontmatter-edits — но это очевидно из дизайна; не finding.

**Where in the artifact**: §«updated»; §«Privacy and security considerations» — silence на `updated`.

**Severity**: low (известный паттерн PKM, ADR-0002 уже зафиксировал creation-time leak).

**What would close this**: одно предложение в Privacy-секции: «`updated:` timestamps накапливают временной профиль работы пользователя над vault-ом. При публикации vault git-history или export — раскрывают паттерн активности. Это аналог git commit timestamps, и не считается дополнительным leak surface beyond git itself; пользователь, который скрывает commit timestamps (`GIT_AUTHOR_DATE`/rebase), должен также применить redaction к `updated:`».

## What's well-handled

- §«Privacy and security considerations» **существует** и явно cross-reference-ит ADR-0002/ADR-0003 §Privacy. Это редкость для personal-CLI ADR.
- Разграничение `x-*` (silent preserve) vs unprefixed unknown (lint-warn) даёт пользователю явный «extension namespace», который не загрязняет linter; это разумная UX-hygiene даже без compliance-impact.
- Recommendation против хранения content-hash в frontmatter (Privacy bullet 5) показывает, что роль авторов уже думала о metadata pollution.
- §«Forward-compat statement» фиксирует additive-only minor bumps — это снижает риск, что будущий privacy-relevant field (например, `author:`) попадёт в schema без отдельного review.
- ADR-0002 предупреждение про CC-5 (note deletion → git history) распространяется на frontmatter без переформулировки — наследуется автоматически.
- Schema strictness lenient — preserve unknown verbatim — означает, что zetto **не теряет** custom fields, которые пользователь добавил для своей privacy-tooling (например, custom `x-redacted: true` flag, который user-script затем использует для export-фильтрации). Это privacy-compatible default.

## Areas I couldn't evaluate

- **Будущий export/share-mode** (отдельный ADR, упомянут в Privacy-секции как «future»). Все finding-и C-1, C-2, C-5 имеют формулировку «при share» — реальная серьёзность зависит от того, как export-mode будет sanitize frontmatter. Если export-mode по умолчанию strip-нет `aliases`, `tags`, `x-*` — finding-и C-1/C-2/C-5 закрываются автоматически. Если export-mode передаст frontmatter as-is — finding-и становятся material.
- **Multi-author scenario (D4 read-write)** — explicitly out-of-scope per STRATEGY; C-3 в этом контексте профилактический. Если D4 когда-либо разморозится, требуется отдельный compliance review aliases-resolver.
- **Конкретный hash-algorithm** для content-hash в `updated` — ADR не фиксирует (sha-256? blake3? xxhash?). Это не privacy-question в строгом смысле, но fingerprint-strength определяет collision-resistance — что является security property. Out of scope for compliance role в текущем scope (single-user); упомянуто для записи.

Terminology pass: применён к prose; identifiers (`x-*`, ULID, RFC 3339, ADR-0002/0003/0004, A3, B1, C2a, D4, GDPR, 152-ФЗ, finding IDs `C-1`…`C-6`, section headers, lint rule names) сохранены. Замены: единичные («рекомендация», «предостережение», «привязка», «утечка», «надёжность», «корреляция», «снимок», «профиль», «политика»). Identifiers preserved.
