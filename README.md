**English** · [Русский](./README.ru.md)

# zk

A terminal-native CLI/TUI for Zettelkasten-style knowledge management, written in Rust.

> ### ⚠️ Status: pre-alpha, architectural design phase
>
> The working tree is intentionally empty. The project is in a structured architectural redesign before code is written: strategy, architecture, decision map, and adversarial ideation are being settled first. See [`STRATEGY.md`](./STRATEGY.md), [`ARCHITECTURE.md`](./ARCHITECTURE.md), and [`docs/architecture/decision-map.md`](./docs/architecture/decision-map.md) for the current state.

## A note on the name

This project shares the name `zk` with [zk-org/zk](https://github.com/zk-org/zk), an established Go-based Zettelkasten CLI (~2.6k stars, active May 2026). They are different projects with different goals:

- **zk-org/zk** is methodology-agnostic: orphan notes, deep folder hierarchies, and free-form tags are all permitted.
- **zk (this project)** enforces research-grounded Zettelkasten constraints (atomic notes, link-before-save, fixed ID schema, no folders-as-taxonomy) at write time, not after the fact.

The name will be revisited in an upcoming ADR (see decision **D3** in [`decision-map.md`](./docs/architecture/decision-map.md)). Until that ADR lands, `zk` is the working name.

## What this is

A CLI/TUI for terminal-native engineers who already live in vim+tmux+git and want their knowledge graph as plain markdown alongside their code — instead of in a separate GUI like Obsidian. Behavior is dictated by proven Zettelkasten patterns (Luhmann, Ahrens): atomic notes, link-before-save, fixed ID schema, no folders-as-taxonomy, no tags-as-link-replacement. The tool composes with `$EDITOR`, ripgrep, fzf, and git rather than reimplementing them.

Read [`STRATEGY.md`](./STRATEGY.md) for the full framing.

## Reading order

If you want to understand where this is heading, read in this order:

1. [`STRATEGY.md`](./STRATEGY.md) — what zk is, who it serves, key metrics, tracks of work.
2. [`ARCHITECTURE.md`](./ARCHITECTURE.md) — system summary, quality attributes (with latency budget breakdown), constraints, anti-patterns, open questions.
3. [`docs/architecture/decision-map.md`](./docs/architecture/decision-map.md) — open architectural decisions in four groups (on-disk contract, engine internals, surface/UX, interop & ecosystem) with dependencies and a proposed order.
4. [`docs/ideation/`](./docs/ideation/) — outputs of brainstorming and idea-filtering passes that feed into decisions.
5. [`docs/architecture/decisions/`](./docs/architecture/decisions/) — accepted ADRs. None yet — the first will resolve project naming; subsequent ones close the on-disk contract group.
6. [`docs/architecture/research/`](./docs/architecture/research/) — discovery, design, and observation reports that produced the decisions.

## Why the working tree is empty

A previous implementation existed in this repository — `Cargo.toml`, `src/{cli,commands,notes,tags,templates,tui,editor,config}`, with UUID IDs, YAML frontmatter, an in-memory HashMap graph, search by tags / title / content / regex / glob, and editor delegation to vim/nvim/code/emacs. The last real commit (`293a1e4 feat: add tui style`, November 2024) is still in the git history.

That implementation was a spike — useful for learning, but the choices it embedded were made before `STRATEGY.md` was written. After articulating the strategy (research-grounded constraints, lintable methodology, terminal-native distribution model), the prior implementation no longer matches the intended product. It is intentionally not in the working tree; the redesign starts from architecture, not from code refactoring.

The git history remains a useful prior-art source for discovery work. To inspect a prior file:

```sh
git show HEAD:src/notes/store.rs | less
git show HEAD:src/cli/mod.rs | less
```

Commits before `293a1e4` are part of the historical record; nothing prior to the architectural reset is being treated as a contract.

## Project scaffolding

This repository uses two complementary Claude Code plugins to drive the architectural and feature work:

- **`archforge`** — architecture cycle (Discover → Research → Design → Decide → Document → Review). All architectural decisions live in `docs/architecture/`.
- **`compound-engineering`** — feature workflow (Brainstorm → Plan → Work → Review → Compound). Feature-level artifacts live in `docs/{ideation,brainstorms,plans,solutions}/`.

The interleaving rules (when each cycle hands off to the other) live in [`AGENTS.md`](./AGENTS.md).

## Contributing

Not yet open for outside contributions — the architectural work is structured and one-author at this stage. Once the first ADRs are accepted and the on-disk format spec (`format-v1`) is published, this section will be expanded with a setup guide and a contribution flow.

If the project is interesting in its current state, the most useful feedback is on the open questions in `decision-map.md`.

## License

[MIT](./LICENSE) © 2026 Igor Kramar.

The on-disk format specification (`format-v1`) will receive its own licensing decision when published — see decision **A5** in [`decision-map.md`](./docs/architecture/decision-map.md).
