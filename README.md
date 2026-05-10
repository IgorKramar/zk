<div align="right">

**English** · [Русский](./README.ru.md)

</div>

<p align="center">
  <img src="./assets/icon-zetto.svg" alt="zetto" width="160" height="160" />
</p>

<h1 align="center">zetto</h1>

<p align="center">
  <strong>A terminal-native CLI/TUI for Zettelkasten-style knowledge management, in Rust.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/status-pre--alpha-d65d0e?style=flat-square" alt="status: pre-alpha" />
  &nbsp;
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-fabd2f?style=flat-square" alt="license: MIT" /></a>
  &nbsp;
  <img src="https://img.shields.io/badge/Rust-CE422B?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
  &nbsp;
  <a href="./docs/architecture/decisions/"><img src="https://img.shields.io/badge/ADRs-4-928374?style=flat-square" alt="ADRs: 4" /></a>
</p>

<p align="center">
  Previously known as <code>zk</code>. Renamed in <a href="./docs/architecture/decisions/0001-project-name-and-ecosystem-positioning.md">ADR-0001</a> (2026-05-09).
</p>

---

> ### ⚠️ Status: pre-alpha, architectural design phase
>
> The working tree is intentionally empty. The project is in a structured architectural redesign before code is written: strategy, architecture, decision map, and adversarial ideation are settled first.
>
> See [`STRATEGY.md`](./STRATEGY.md), [`ARCHITECTURE.md`](./ARCHITECTURE.md), and [`docs/architecture/decision-map.md`](./docs/architecture/decision-map.md) for the current state.

## Why zetto?

A Zettelkasten tool for terminal-native engineers who already live in vim+tmux+git and want their knowledge graph as plain markdown alongside their code — instead of in a separate GUI like Obsidian.

Behavior is dictated by proven Zettelkasten patterns (Luhmann, Ahrens):

- **Atomic notes.** One concept per note.
- **Link-before-save.** Notes must connect to the existing graph at write time, not later.
- **Fixed ID schema.** Every note has a stable identifier that survives renames.
- **No folders-as-taxonomy.** Structure emerges from links, not directories.
- **No tags-as-link-replacement.** Tags describe; links connect.

The tool composes with `$EDITOR`, ripgrep, fzf, and git rather than reimplementing them.

Read [`STRATEGY.md`](./STRATEGY.md) for the full framing.

## A note on the name

`zetto` derives from German *Zettel* (note card) — the root of the Zettelkasten methodology itself. In Japanese, ゼット (zetto) is also how the letter Z is pronounced; the icon picks up this cultural anchor — ゼ in the center is the syllable closest to "Z".

This project was renamed from `zk` to `zetto` in [ADR-0001](./docs/architecture/decisions/0001-project-name-and-ecosystem-positioning.md) to avoid confusion with [zk-org/zk](https://github.com/zk-org/zk), an established Go-based Zettelkasten CLI in the same niche (~2.6k stars). The two projects have different goals:

|                  | zk-org/zk                                                | **zetto** (this project)                              |
| ---------------- | -------------------------------------------------------- | ----------------------------------------------------- |
| **Methodology**  | agnostic — orphans, deep folders, free tags allowed      | enforced — research-grounded constraints at write time |
| **Stack**        | Go                                                       | Rust                                                  |
| **License**      | GPLv3                                                    | MIT                                                   |

Search results for `zk` from before May 2026 still lead here — GitHub auto-redirects from `IgorKramar/zk`.

## Reading order

If you want to understand where this is heading, read in this order:

1. [`STRATEGY.md`](./STRATEGY.md) — what zetto is, who it serves, key metrics, tracks of work.
2. [`ARCHITECTURE.md`](./ARCHITECTURE.md) — system summary, quality attributes (with latency budget breakdown), constraints, anti-patterns, open questions.
3. [`docs/architecture/decision-map.md`](./docs/architecture/decision-map.md) — open architectural decisions in four groups (on-disk contract, engine internals, surface/UX, interop & ecosystem) with dependencies and a proposed order.
4. [`docs/ideation/`](./docs/ideation/) — outputs of brainstorming and idea-filtering passes that feed into decisions.
5. [`docs/architecture/decisions/`](./docs/architecture/decisions/) — accepted ADRs.
6. [`docs/architecture/research/`](./docs/architecture/research/) — discovery, design, and observation reports.

## Why the working tree is empty

A previous implementation existed in this repository — `Cargo.toml`, `src/{cli,commands,notes,tags,templates,tui,editor,config}`, with UUID IDs, YAML frontmatter, an in-memory HashMap graph, search by tags / title / content / regex / glob, and editor delegation to vim/nvim/code/emacs. The last real commit (`293a1e4 feat: add tui style`, November 2024) is preserved in the git history.

That implementation was a spike — useful for learning, but the choices it embedded were made before `STRATEGY.md` was written. After articulating the strategy (research-grounded constraints, lintable methodology, terminal-native distribution), the prior implementation no longer matches the intended product. It is intentionally not in the working tree; the redesign starts from architecture, not from code refactoring.

To inspect a prior file as historical prior-art:

```sh
git show 293a1e4:src/notes/store.rs | less
git show 293a1e4:src/cli/mod.rs    | less
```

Commits before `293a1e4` are part of the historical record; nothing prior to the architectural reset is treated as a contract.

## Project scaffolding

This repository uses two complementary Claude Code plugins to drive the architectural and feature work:

- **[`archforge`](https://github.com/IgorKramar/archforge-marketplace)** — architecture cycle (Discover → Research → Design → Decide → Document → Review). All architectural decisions live in `docs/architecture/`.
- **`compound-engineering`** — feature workflow (Brainstorm → Plan → Work → Review → Compound). Feature-level artifacts live in `docs/{ideation,brainstorms,plans,solutions}/`.

The interleaving rules (when each cycle hands off to the other) live in [`AGENTS.md`](./AGENTS.md).

## Contributing

Not yet open for outside contributions — the architectural work is structured and one-author at this stage. Once the first ADRs are accepted and the on-disk format spec (`format-v1`) is published, this section will be expanded with a setup guide and a contribution flow.

If the project is interesting in its current state, the most useful feedback is on the open questions in [`decision-map.md`](./docs/architecture/decision-map.md).

## License

[MIT](./LICENSE) © 2026 Igor Kramar.

The on-disk format specification (`format-v1`) will receive its own licensing decision when published — see decision **A5** in [`decision-map.md`](./docs/architecture/decision-map.md).

<p align="center">
  <sub>Built with <a href="https://github.com/anthropics/claude-code">Claude Code</a> ·
  Architecture by <a href="https://github.com/IgorKramar/archforge-marketplace">archforge</a> ·
  Workflow by <a href="https://github.com/EveryInc/compound-engineering">compound-engineering</a></sub>
</p>
