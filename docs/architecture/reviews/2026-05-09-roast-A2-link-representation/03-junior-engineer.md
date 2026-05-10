## Junior-engineer findings

- **Target**: `docs/architecture/research/2026-05-09-A2-decision-summary.md`
- **Date**: 2026-05-09
- **Role**: junior-engineer (clarity)
- **Reading posture**: впервые открываю этот документ. Под рукой `STRATEGY.md`, `ARCHITECTURE.md`, `decision-map.md`, ADR-0001 и ADR-0002. Discovery/research/design A2 НЕ читал. Утро понедельника, спросить некого.

## Summary

Документ читается достаточно плотно, если уже знаешь, что такое wikilink и как устроен `pulldown-cmark`, но активно опирается на материалы, которых у читателя нет: «Альтернатива A», «research §1/§2/§5/§8», «7 leans» из discovery упомянуты в шапке как inputs, но решение само по себе нигде не пересказывает, что они содержали. Самый крупный пробел — секция «Wikilink-syntax v1» начинается с готового вывода («zetto генерирует/читает …») без единой фразы о том, *какую проблему это решает* и *что такое wikilink в контексте zetto* — все категории cross-cutting (CC-1, CC-4, F-1 и т.д.) из ADR-0002 имели прозу-якорь, тут её нет.

## Clarity findings

### J-1: «Альтернатива A» в шапке — без раскрытия, что это

**Category**: broken cross-reference / erased reasoning

**The gap**: В строке `Inputs` стоит «design (Альтернатива A выбрана)». Дальше по документу единственный намёк на альтернативы — секция «Alternatives considered» отсутствует целиком. В ADR-0002 эта секция есть и в decision summary A1 она тоже подразумевается; здесь её нет, и читатель не знает, что было A, B, C.

**What I tried**: Прочитал ARCHITECTURE.md и decision-map.md (A2 описан как open question с forces, без перечня вариантов). Discovery/design-файлы по условию задачи я не открывал, но именно туда меня вынуждает идти ссылка «Альтернатива A» — иначе я не пойму, *почему* wikilink-primary, а не canonical-markdown-primary.

**Suggested fix**: Добавить однопараграфную секцию «Alternatives considered (выжимка из design)» с тремя альтернативами и одной строкой про каждую — как сделано в ADR-0002.

### J-2: «7 leans приняты» — без перечня

**Category**: broken cross-reference

**The gap**: В шапке `Inputs: discovery (7 leans приняты), research digest, design …`. Дальше leans нигде не перечислены. Читатель не знает, что зафиксировано как «принято» до этого ADR.

**Suggested fix**: Добавить bullet-список «Leans, принятые в discovery (вход в это решение):» с 7 пунктами одной строкой каждый. Это снимает зависимость от discovery-файла.

### J-3: «research §1», «research §2», «research §5», «research §8» — указатели в неоткрытый документ

**Category**: broken cross-reference

**The gap**: В заголовках секций встречается «(применение research §1)», «(применение research §2 + §5)», «(применение research §2 + §8)», «(применение research §2)». Читатель, у которого нет research digest на руках, не понимает, какие именно тезисы оттуда применены.

**Suggested fix**: Либо заменить «§N» на полупредложение по существу («…в соответствии с research-выводом X: pulldown-cmark поддерживает wikilinks нативно с 0.13»), либо вынести «Что взято из research digest» отдельным блоком в конце с парой строк на каждый § — как именно это применяется ниже.

### J-4: «back-compat с imported content» — что такое imported content

**Category**: undefined term

**The gap**: «zetto **читает** оба wikilink-формата плюс canonical markdown `[text](filename.md)` (для back-compat с imported content)». Что такое «imported content», в zetto не введено нигде раньше — нет ни импортирующей команды, ни перечня поддерживаемых источников.

**Suggested fix**: Заменить на конкретику: «для совместимости с заметками, импортированными из Obsidian-vault или mdbook (см. D4 в decision-map)» — или явно сказать, что подразумеваются произвольные внешние markdown-файлы.

### J-5: `has_pothole` — термин из API библиотеки, не объяснён

**Category**: undefined term

**The gap**: «`has_pothole = false` → `[[X]]` form. `has_pothole = true` → `[[X|display]]` form». Слово `has_pothole` появляется без введения. Это нестандартный термин (буквально «pothole» — выбоина); читатель, не помнящий API `pulldown-cmark`, должен догадаться, что это «есть ли пайп-разделитель».

**Suggested fix**: Добавить одну строку: «`has_pothole` — поле из `pulldown-cmark`, true если внутри wikilink есть разделитель `|` (pipe), отделяющий display-text».

### J-6: «broken style» — в HTML и в TUI — не пояснено, как именно

**Category**: number/spec without context

**The gap**: «render literal в broken-style» и далее в render-секции: «визуально — broken style (CSS class в HTML render; в TUI — отдельный colour)». Какой CSS-class? Какой colour? Это контракт с темами или внутренняя деталь?

**Suggested fix**: Либо явно сказать «название класса/цвета — implementation detail, не часть public ABI», либо назвать конкретное имя класса (например, `zetto-broken-link`) и оставить пометку «фиксация имени — в C3 (TUI library choice)».

### J-7: `recommended-luhmann` preset — где определён

**Category**: broken cross-reference

**The gap**: «zetto поставляет следующие правила (предсет `recommended-luhmann`, см. ADR-0002 § Methodology engine architecture)». Я открыл ADR-0002 — секции `Methodology engine architecture` там нет. Есть упоминание `recommended-luhmann` мельком в alternatives и в § Format versioning — но не определение пресета.

**Suggested fix**: Заменить на «`recommended-luhmann` — будущий built-in preset, формализуемый в C2a (см. decision-map). Этот ADR резервирует имя, не определяет содержимое».

### J-8: «format-v1» как живой контракт — без точки определения

**Category**: undefined term / hidden boundary

**The gap**: «фиксирует wikilink syntax + display-text + external-URL-разделение как часть `format-v1`». Что такое `format-v1`? ADR-0002 один раз упоминает «stable, versioned компонент будущей спецификации format-v1 (см. A5 в decision-map, статус — open)». То есть format-v1 ещё не существует как документ, но A2 уже в него «фиксирует».

**Suggested fix**: Добавить одно предложение в начале «Forward-compat statement» — «`format-v1` — будущая публичная спецификация формата заметок, фиксируемая отдельным ADR (A5 в decision-map). До её принятия этот ADR резервирует свой вклад в неё авансом».

### J-9: «`#`-suffix-семантика» — какая именно семантика

**Category**: erased reasoning

**The gap**: «`#`-suffix-семантика и embed-семантика — implementation-detail-будущего». Дальше из контекста можно догадаться, что речь про `#Heading` и `#^block-id`, но ровно в этой формулировке семантика не определена — она написана как уже понятная.

**Suggested fix**: Заменить на «семантика `#`-суффикса (anchor refs и block refs, см. выше) и embed-семантика (`![[…]]`, см. выше) — implementation-detail-будущего».

### J-10: Recovery / линт-уровень `error` — кто и где блокирует операцию

**Category**: unfollowable step

**The gap**: В таблице lint rules `zetto/external-url-as-wikilink` имеет default severity = `error`. В одном месте в resolver-алгоритме сказано «render literal». Что значит `error` в zetto — fail сборки? отказ от save? отказ render? просто красный цвет в TUI?

**Suggested fix**: Добавить одну строку под таблицей: «Семантика severity (`error`/`warn`) — фиксируется в C2a (Methodology rule engine architecture). До её определения этот ADR резервирует только имена правил и предлагаемые значения severity по умолчанию».

### J-11: «synchronous scan в v1» — без attestации последствий

**Category**: number without context / hidden boundary

**The gap**: Render-секция: «synchronous scan в v1 (TODO заменить на B1 graph index когда B1 закрыт)». Какой это scan — всех заметок? только target-а? Каков ожидаемый latency-impact на capture-latency budget из ARCHITECTURE.md §2.1?

**Suggested fix**: Добавить одно предложение: «Synchronous scan = open + read frontmatter одного target-файла на каждый wikilink при render. На 1k заметок — N FS-reads за render-pass, что вмещается в <X ms p50 (см. бюджет ARCHITECTURE.md §2.1)».

## What's well-documented

- **Edge cases** — секция аккуратно перечисляет четыре нетривиальных случая (`[[]]`, `[[|display]]`, `[[ID|a|b]]`, `[](path.md)`) с конкретным поведением каждого.
- **Resolver-алгоритмы** для wikilink и для markdown-link разнесены в два пронумерованных списка с явными regex-ами и явными branch-ами — читать удобно.
- **Deferred-в-v2** — три подсекции (embeds, anchors, block refs) с одной и той же структурой «v1 — что; v2 — что; migration cost — какой».
- **Defer-стратегия** в одном предложении объясняет, *почему* defer безопасен («синтаксис уже валиден на уровне парсера»).
- **Open questions, отложенные в смежные ADR** — короткий чёткий список с привязкой к decision-map (A3, B1, C2a, D4).

## Where I gave up

- **«Декомпозиция» как раздел** — заголовок есть, но я не понял, *по какому принципу* выбраны именно эти подразделы (Wikilink-syntax v1 / Parser / Resolver / Render / Lint rules / Deferred / Edge cases / Forward-compat). Это компоненты пайплайна? Этапы реализации? Слои документации?
- **Что именно из «design» решения здесь применено** — документ называет себя «decision summary», но я не уверен, что это: пересказ design-документа, выжимка research-выводов, или draft ADR. Шапка говорит «Will become: ADR-0003», то есть это исходник для ADR. Я не понимаю, какой части жизненного цикла принадлежит этот документ.
