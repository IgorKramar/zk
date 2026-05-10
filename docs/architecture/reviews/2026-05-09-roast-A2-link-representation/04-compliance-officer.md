## Compliance-officer findings

**Target**: `/Users/user/Work/self/zk/docs/architecture/research/2026-05-09-A2-decision-summary.md`
**Date**: 2026-05-09
**Disclaimer**: я не юрист. Замечания ниже — это вопросы, которые задал бы compliance- или security-ревьюер. Окончательные регуляторные суждения требуют квалифицированного совета.

### Summary

zetto — локальный single-user CLI/TUI без сети, без облака, без third-party data sharing; подавляющее большинство compliance-категорий (cross-border transfers, GDPR-ишные обязанности контролёра, инцидент-реагирование, sub-processors) попросту вне периметра — `ARCHITECTURE.md` §4 явно фиксирует «Compliance — N/A». Однако формат wikilink-линков с ULID-таргетами выводит на поверхность отдельный, узкий, но реальный класс рисков **share-time information leakage**: при публикации одной заметки наружу её link-graph непреднамеренно сообщает читателю существование, ULID-timestamp-ы и иногда title-фрагменты других — нешареных — заметок. Этот класс рисков ADR-0002 уже частично документирует для filename, но A2-decision-summary его для линков не повторяет и не расширяет.

### Applicable regulations and standards

- **GDPR / 152-FZ / CCPA и пр.**: формально не применимы — продукт не обрабатывает чужие персональные данные, single-user, локально, без передачи третьим лицам. Это явно зафиксировано в `ARCHITECTURE.md` §4 (Compliance N/A) и согласуется со `STRATEGY.md` (terminal-native инженер ведёт собственную базу знаний). Регуляторного периметра у zetto-как-инструмента нет.
- **Применимая «privacy hygiene»**: данные пользователя могут включать его собственные персональные/чувствительные сведения (медицинские, юридические, NDA-материалы, цитаты из приватной переписки). Здесь zetto — не контролёр, а tool, но trade-off-ы формата напрямую влияют на то, что произойдёт **в момент, когда пользователь решит шарить заметку наружу** (gist, blog, репозиторий, отправка коллеге). Именно эта поверхность остаётся областью compliance-ревью.
- **OWASP ASVS V7 (logging) / V8 (data protection)** — релевантно к lint-сообщениям и render-fallback-у как к каналам утечки. Не сертификация, а baseline-практика.

### Findings

#### C-1: ULID-таргеты в `[[ULID]]` раскрывают timestamp-метаданные «соседних» заметок при шаринге

**Category**: PII flow / data minimization
**The gap**: A2 specифицирует, что zetto генерирует именно `[[ULID]]` (а не `[[slug]]` и не human-readable алиас). ULID — это time-prefixed identifier; первые 48 бит декодируются тривиально в ms Unix epoch UTC (см. ADR-0002 § Sortability, § Privacy and security considerations). Когда пользователь публикует одну заметку наружу (gist, blog post, peer review), её raw markdown содержит wikilinks вида `[[01J9XQK7ZBV5G2D8X3K7C4M9P0]]` — ссылки на заметки, которые наружу не выходят. Каждая такая ссылка передаёт читателю: (а) факт существования другой заметки, (б) её приблизительное время создания с точностью до миллисекунды, (в) последовательность создания (lexicographic = chronological). Для большинства пользователей это не проблема. Для тех, чей vault содержит NDA-, медицинский, юридический или security-чувствительный материал, это реальная утечка метаданных, **не упомянутая в A2**. ADR-0002 § Privacy упоминает аналогичный риск для filename, но A2 ни наследует это рассуждение, ни добавляет его явно для link-syntax — между тем рендер `[[ULID]]` в опубликованном HTML/markdown сохраняет ULID полностью.

**Where in the artifact**: § «Wikilink-syntax v1» (определяет `[[ULID]]` как primary write-form), § «Render» (не описывает render-режим для export/share, по умолчанию ULID-литерал остаётся видимым при отсутствии резолва или у читателя без vault-а).

**Severity**: medium — это не регуляторное нарушение (zetto не обрабатывает чужие PII), но это design choice, который тихо передаёт метаданные при первом use case шаринга. Для personal pre-alpha — приемлемо; для документации формата — должно быть зафиксировано как known characteristic.

**What would close this**: одна-две строчки в § Privacy/Security ADR-0003 (по образу ADR-0002 § Privacy and security considerations): «`[[ULID]]` в опубликованной заметке раскрывает creation-time связанной заметки; для share-сценариев планируется (отдельным ADR) export-mode, заменяющий ULID на opaque hash или вшивающий display-text inline. В v1 — known limitation, ответственность пользователя».

#### C-2: Display-text в `[[ULID|display]]` — канал утечки title-фрагментов чужих заметок

**Category**: PII flow / data minimization
**The gap**: § «Render» определяет приоритет display-text: (1) inline display из `[[ULID|display]]`, (2) frontmatter `title` цели, (3) ULID literal. Случай (2) делает следующее: при render-е заметки A, в которой стоит `[[ULID-of-B]]`, zetto **читает frontmatter заметки B и подставляет её title в render заметки A**. Если A пошла наружу (export to HTML, paste-to-blog, share with reviewer), а B — нет, то title B (потенциально содержащий имя, NDA-кодовое слово, медицинский диагноз) утекает в материализованный текст заметки A. Это нетривиальное поведение для пользователя, который написал «`[[ULID-of-B]]`» как лаконичный bare reference, ожидая, что наружу уйдёт ULID, а не raw title. Случай (1) — `[[ULID|display]]` — пользователь явно набрал display, и это его осознанный выбор; здесь риска нет. Опасность именно в bare-форме `[[ULID]]` + render-fallback на frontmatter title.

**Where in the artifact**: § «Render», шаги 1–3 приоритета display-text — silent на тему, что fallback через frontmatter target-а — это cross-note data flow, материализующийся в опубликованный артефакт.

**Severity**: medium — в personal-use ловушки нет, но это нарушает least-surprise-принцип: пользователь видит в исходнике `[[01J9X...]]`, при export через любой render-pipeline получает «My salary negotiation with Acme». Документировать или дать контракт на render-без-fallback в export-режиме.

**What would close this**: упомянуть это явно в § Privacy в ADR-0003 и зафиксировать как open question для export-pipeline (когда он появится в отдельном ADR): export-mode должен либо рендерить ULID literal, либо требовать explicit display-text у каждого линка перед публикацией. Сейчас — known limitation.

#### C-3: lint-сообщение `zetto/no-broken-link` цитирует ULID — leak-channel при `git log` / CI / shared output

**Category**: audit / logging hygiene
**The gap**: § «Lint rules» определяет правило `zetto/no-broken-link` (severity warn) для unresolvable target-ов. По формулировке в § «Resolver» шаг 4: «Не найдено — lint flag `zetto/no-broken-link`; render literal в broken-style». Текст диагностики не специфицирован, но естественная форма — «error: link `[[01J9X...]]` points to unknown note». Если пользователь запускает `zetto lint` в CI (например, GitHub Actions на публичном репозитории, где сами заметки не закоммичены, а закоммичен только subset, и CI lintит — натянутый, но не невозможный сценарий), либо paste-ит вывод lint-а в issue/PR/Slack/gist — ULID broken-target утекает. Аналогично, `zetto lint` output может попасть в shell history и terminal scrollback на shared workstation. Для personal-use это микрориск; для документации формата — это **класс log-channel exposure**, который OWASP ASVS V7 рекомендует проектировать с самого начала, а не закрывать post-hoc.

Дополнительно: правила `zetto/anchor-not-supported-in-v1` и `zetto/block-ref-not-supported-in-v1` в их естественной форме diagnostic тоже будут содержать ULID. То же самое — `zetto/non-ulid-wikilink-target` потенциально цитирует не-ULID-токен, который пользователь использовал как «временный slug» (может содержать NDA-имя).

**Where in the artifact**: § «Lint rules» (таблица 6 правил), § «Resolver» шаг 4 — silent на формат diagnostic-сообщений.

**Severity**: low — не нарушение, а упущение в logging hygiene; для single-user CLI редко срабатывает. Стоит зафиксировать как convention.

**What would close this**: одно предложение в ADR-0003 — «Lint-сообщения проектируются так, что сами по себе не являются leak-channel более полным, чем уже видимый источник: ULID цитируется, потому что он уже находится в исходнике заметки; non-ULID-токены пользователя цитируются опционально, режим `--quiet-targets` ограничивает diagnostic до позиции (file:line:col) без литерала». Альтернатива — простая фиксация: «lint-сообщения цитируют ULID-literal-таргет; пользователь, передающий вывод lint-а наружу, ответственен за scrubbing». Любая из двух формулировок закрывает gap.

#### C-4: render-fallback через synchronous frontmatter scan создаёт implicit cross-note read-path в render-time

**Category**: trust boundary / data flow
**The gap**: § «Render» шаг 2 — synchronous scan target-frontmatter для подстановки title (TODO заменить на B1-index). Это означает, что при любом render-е заметки zetto открывает на чтение **другие заметки** (точнее, их frontmatter region). На уровне unix-permissions это безопасно (single-user, тот же uid/gid), но создаёт два менее очевидных следствия: (а) sandboxed-execution-окружения, в которых zetto запускается с restricted FS-scope (например, поверх per-note container/jail), будут видеть «inexplicable» open-вызовы вне области; (б) если в будущем zetto обзаведётся per-note ACL-моделью (например, в multi-author cycle, который сейчас anti-pattern, но в decision-map не запрещён навсегда), render-fallback станет implicit privilege escalation — A читает B-frontmatter без явной grant. Сейчас это не проблема, потому что multi-user/ACL out of scope; но это решение **тихо устанавливает архитектурное допущение, что любой реndered note имеет full read-access к frontmatter всех target-ов**, и это допущение должно быть либо зафиксировано как контракт, либо окружено явным caveat.

**Where in the artifact**: § «Render» шаг 2, парная заметка про B1 в § «Open questions, отложенные».

**Severity**: low — для current scope (single-user, no ACL) это не риск, а архитектурный тихий контракт. Поднимать перед multi-author cycle.

**What would close this**: одна строка в § Forward-compat statement или § Render — «Render-fallback допускает full read-access render-engine-а к frontmatter всех linked target-ов; multi-author cycle потребует ревизии этого контракта».

#### C-5: Embeds (`![[ULID]]`), отложенные в v2 — будущая структурная утечка контента, заслуживает превентивного caveat

**Category**: PII flow / data minimization (forward-looking)
**The gap**: § «Deferred в v2» планирует embed-render-pass, который inline-ит target-content. Это качественно отличается от C-1 и C-2: embed материализует **тело** target-заметки внутрь embedding-заметки. При шаринге embedding-заметки наружу её render-output содержит полный текст embedded заметки. Пользователь, читающий исходник embedding-заметки, видит лаконичное `![[01J9X...]]` и может не осознавать, что при export наружу уйдёт весь body цели. Это в точности тот класс ловушек, на котором основано множество incident-постмортемов в product-формате (Notion-strip-publish, Slack-public-channel-export). A2 говорит «migration cost — низкий, форвард-compat free» о parser-уровне, но **privacy-level migration не free**: embed-семантика полностью меняет модель угроз для share. ADR-0003 — естественное место зафиксировать это превентивно, до того как v2-cycle для embeds уже наберёт momentum.

**Where in the artifact**: § «Deferred в v2», подпункт «Embeds `![[ULID]]`».

**Severity**: low (поскольку deferred, не реализуется в v1) — но уровень severity повышается до medium в момент, когда v2-cycle для embeds открывается. Лучше зафиксировать caveat сейчас.

**What would close this**: одна строка в § Deferred / v2: «Embed-семантика расширяет модель угроз share-time leakage: render-output embedding-заметки содержит body цели полностью. v2-ADR для embeds **обязан** описать export-mode behavior (render literal `![[X]]` vs inline body) до реализации».

#### C-6: external-URL tracking в индексе — потенциальная PII-поверхность, неупомянутая в A2

**Category**: PII flow / third-party risk (preventive)
**The gap**: A2 explicitly разделяет external `[text](https://...)` от internal `[[ULID]]`: external никогда не резолвится в vault и проходит как стандартный CommonMark. Это корректно. **Однако** A2 silent на вопрос, попадают ли external URL в граф/индекс zetto (например, для будущей фичи «what-links-here-from-external» или для cross-note URL-дедупликации, которые B1-index естественно мог бы поддерживать). Если такой index появится, он становится отдельной PII-/sensitive-surface: список URL пользовательского интереса (медицинские порталы, dating-сайты, sensitive subreddits, internal-corp URL-ы, выдающие компанию-работодателя). Сам по себе индекс не утечка, но он живёт как derived state на диске (см. anti-patterns в `ARCHITECTURE.md` §7) и расширяет attack-surface при FS-snapshot, time-machine-backup, sync-в-облако-через-iCloud-`~/Library`. A2 не обязан решать это сам, но обязан явно делегировать в B1.

**Where in the artifact**: § «Wikilink-syntax v1» строка про external, § «Resolver» Алгоритм для markdown-link-target-а шаг 1. Silent на indexing posture для external URL.

**Severity**: low — preventive flag к B1, не gap в A2-как-таковом.

**What would close this**: одна строка в § «Open questions, отложенные» — «B1 (graph index) определяет, попадают ли external URLs в persistent index; если да, — рассмотреть privacy-impact derived-state-а в отдельном sub-decision».

### What's well-handled

- **Local-only архитектура**: `ARCHITECTURE.md` §4 фиксирует Compliance N/A честно — нет сети, нет облака, нет third-party data flow, нет sub-processors. Это самая надёжная compliance-позиция, она снимает 80% типичных вопросов автоматически.
- **External vs internal линки разделены чисто**: § «Wikilink-syntax v1» явно отделяет `[[ULID]]` (vault-internal) от `[text](https://...)` (external markdown), и резолвер правилом `zetto/external-url-as-wikilink` (severity error) предотвращает случайное смешение. Это исключает класс bug-ов «zetto пытается резолвить внешний URL в vault и невольно делает HTTP-вызов» — для security-modal такая инъекция была бы серьёзной.
- **Encryption-at-rest и note-deletion явно делегированы пользователю**: ADR-0002 § Privacy. A2 наследует этот контракт по умолчанию и не вводит новых assumption-ов о безопасности диска.
- **Defer-стратегия для embeds защищена parser-валидностью** (§ Forward-compat), что предотвращает класс bug-ов «v1-only-error блокирует чтение v2-валидного файла»; для compliance это важно потому, что recovery-path после accidental v2-syntax не требует ручной правки исходника.

### Areas I couldn't evaluate

- **Будущая интеграция с share/export pipeline**: A2 ничего не говорит о том, как zetto будет генерировать output для публикации (есть ли вообще `zetto export`, `zetto publish`). Все findings C-1, C-2, C-5 предполагают, что какая-то форма export существует или появится; если она навсегда останется out-of-scope (пользователь делает `cat note.md | pandoc` сам), то риски остаются ограниченно gating-доступом пользователя к собственным файлам. Стоит явно сказать в decision-map.
- **MCP-сервер и LSP-сценарии (вне scope ADR-0001)**: если zetto в будущем выпустит MCP-сервер или LSP, render-fallback (C-4) и lint-output (C-3) становятся data-out-of-process channels с другой моделью угроз. Текущий ADR-0003 это не покрывает и не должен — но это естественные cross-references в момент C5 (decision-map).
- **Multi-author cycle**: ARCHITECTURE.md §7 фиксирует single-author by design в v1, но не запрещает навсегда. Все вопросы ACL/per-note-trust-boundary, упомянутые в C-4, активизируются именно тогда. Без знания planned trajectory сложно сказать, превентивный это caveat или преждевременный.

---

Terminology pass: применён к prose (calque-замены: «ревьюер», «утечка», «материализуется», «известное ограничение», «контракт» вместо буквальных кальок). Identifiers preserved: ADR-IDs, Finding IDs (C-1…C-6), section headers, rule IDs (`zetto/no-broken-link`, etc.), crate names, ULID/Crockford terminology, GDPR/CCPA/152-FZ/OWASP-ASVS — оставлены без перевода.
