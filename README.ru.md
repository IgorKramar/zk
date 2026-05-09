[English](./README.md) · **Русский**

# zetto

Terminal-native CLI/TUI для ведения базы знаний в формате Zettelkasten, на Rust.

> Прежнее имя — `zk`. Переименован в [ADR-0001](./docs/architecture/decisions/0001-project-name-and-ecosystem-positioning.md) (2026-05-09), чтобы избежать коллизии с [zk-org/zk](https://github.com/zk-org/zk).

> ### ⚠️ Состояние: pre-alpha, фаза архитектурного проектирования
>
> Рабочее дерево намеренно пустое. Проект сейчас в фазе структурированного архитектурного проектирования до того, как написан код: сначала фиксируются стратегия, архитектура, карта решений и проходит adversarial-ideation. Текущее состояние — в [`STRATEGY.md`](./STRATEGY.md), [`ARCHITECTURE.md`](./ARCHITECTURE.md) и [`docs/architecture/decision-map.md`](./docs/architecture/decision-map.md).

## Замечание об имени

Проект переименован из `zk` в `zetto` в [ADR-0001](./docs/architecture/decisions/0001-project-name-and-ecosystem-positioning.md), чтобы избежать путаницы с [zk-org/zk](https://github.com/zk-org/zk) — действующим Go-CLI для Zettelkasten в той же нише (~2,6k★, активный в мае 2026). У проектов разные цели:

- **zk-org/zk** методологически нейтрален: сироты-заметки, глубокие папочные иерархии и свободные теги — всё разрешено.
- **zetto (этот проект)** принуждает к доказанным паттернам Zettelkasten (atomic notes, link-before-save, фиксированная ID-схема, отказ от папок-как-таксономии) на этапе записи, а не post factum.

Имя `zetto` — производное от немецкого *Zettel* (карточка), корня самой Zettelkasten-методологии. Если ты пришёл по поиску или ссылке, где упомянут `zk` до мая 2026 года — это тот же репозиторий (автоматический redirect с `IgorKramar/zk`).

## Что это

CLI/TUI для terminal-native инженеров, которые уже живут в vim+tmux+git и хотят, чтобы их граф знаний жил рядом с кодом как plain markdown — а не в отдельном GUI-приложении вроде Obsidian. Поведение продукта диктуется доказанными паттернами Zettelkasten (Luhmann, Ahrens): atomic notes, link-before-save, фиксированная ID-схема, отказ от папок-как-таксономии и тегов-как-замены-ссылок. Инструмент композируется с `$EDITOR`, ripgrep, fzf и git, а не переизобретает их.

Полный контекст — в [`STRATEGY.md`](./STRATEGY.md).

## Порядок чтения

Если хочется понять, куда это идёт, читай в этом порядке:

1. [`STRATEGY.md`](./STRATEGY.md) — что такое zetto, кому он, ключевые метрики, треки работы.
2. [`ARCHITECTURE.md`](./ARCHITECTURE.md) — системная сводка, нефункциональные требования (включая разложение бюджета задержек по этапам), ограничения, анти-паттерны, открытые вопросы.
3. [`docs/architecture/decision-map.md`](./docs/architecture/decision-map.md) — открытые архитектурные решения в четырёх группах (контракт на диске, внутренности движка, поверхность/UX, interop и экосистема) с зависимостями и предложенным порядком.
4. [`docs/ideation/`](./docs/ideation/) — результаты brainstorming-проходов и фильтрации идей, питающие решения.
5. [`docs/architecture/decisions/`](./docs/architecture/decisions/) — принятые ADR-ы. Пока ноль — первый разрешит имя проекта, последующие закроют группу контракта на диске.
6. [`docs/architecture/research/`](./docs/architecture/research/) — discovery-, design- и observation-отчёты, из которых выросли решения.

## Почему рабочее дерево пустое

В этом репозитории была прежняя реализация — `Cargo.toml`, `src/{cli,commands,notes,tags,templates,tui,editor,config}`, с UUID-ID, YAML-frontmatter, графом на HashMap в памяти, поиском по тегам/заголовку/содержимому/regex/glob, делегированием редактору vim/nvim/code/emacs. Последний реальный коммит (`293a1e4 feat: add tui style`, ноябрь 2024) сохранён в git-истории.

Та реализация — это спайк (пробный код): полезный как опыт, но выбор, в неё заложенный, был сделан до того, как написан `STRATEGY.md`. После того как стратегия артикулирована (research-grounded constraints, методология как lint, terminal-native модель распространения), прежняя реализация перестала соответствовать предполагаемому продукту. Она намеренно не в рабочем дереве; редизайн начинается от архитектуры, а не от рефакторинга кода.

Git-история остаётся полезным источником prior-art для discovery-работы. Чтобы посмотреть прежний файл:

```sh
git show HEAD:src/notes/store.rs | less
git show HEAD:src/cli/mod.rs | less
```

Коммиты до `293a1e4` — часть исторического следа; ничего из периода до архитектурного reset-а не трактуется как контракт.

## Каркас проекта

Репозиторий использует два дополняющих плагина Claude Code, ведущих архитектурную и фичевую работу:

- **`archforge`** — архитектурный цикл (Discover → Research → Design → Decide → Document → Review). Все архитектурные решения живут в `docs/architecture/`.
- **`compound-engineering`** — рабочий цикл уровня фичи (Brainstorm → Plan → Work → Review → Compound). Артефакты уровня фичи живут в `docs/{ideation,brainstorms,plans,solutions}/`.

Правила чередования (когда один цикл передаёт другому) — в [`AGENTS.md`](./AGENTS.md).

## Вклад

Пока не открыто для внешних PR — архитектурная работа структурированная и одно-авторская. После того как первые ADR-ы будут приняты и опубликована спецификация формата (`format-v1`), эта секция будет расширена руководством по setup-у и потоком вкладов.

Если проект интересен в текущем состоянии — самая полезная обратная связь сейчас по открытым вопросам в `decision-map.md`.

## Лицензия

[MIT](./LICENSE) © 2026 Igor Kramar.

Лицензия для спецификации формата на диске (`format-v1`) — отдельное решение, которое будет принято при публикации спецификации; см. запись **A5** в [`decision-map.md`](./docs/architecture/decision-map.md).
