---
name: lore-lingo-project-standards
description: Enforce LensLingo repository-wide engineering standards for project structure, Vue/TypeScript and Naive UI frontend work, macOS 27 and Windows 11 presentation, Rust/Tauri backend work, SQLite migrations, Git Flow, testing, security, CI, packaging, and releases. Use whenever Codex plans, implements, reviews, tests, commits, or releases changes in the lore-lingo repository.
---

# LensLingo Project Standards

Apply these standards to every repository change. Treat existing source and accepted OpenSpec artifacts as the current architecture; do not invent a parallel structure when a documented boundary already exists.

## Workflow

1. Inspect the requested scope, relevant source files, active OpenSpec changes, and working-tree state.
2. Always read `references/directory-structure.md` before adding or moving files.
3. Load only the task-specific references from the routing table below.
4. State material assumptions when the repository does not yet define a decision.
5. Implement the smallest coherent change that follows the selected standards.
6. Run the required checks from `references/testing-quality.md` in proportion to risk.
7. Re-read `references/git-flow.md` before creating a branch, commit, pull request, tag, or release.

## Reference Routing

| Task scope | Required references |
| --- | --- |
| Any source or directory change | `directory-structure.md`, `testing-quality.md` |
| Vue, TypeScript, CSS, components, composables, Tauri frontend APIs | `frontend.md` |
| Naive UI components, providers, themes, icons, or frontend dependencies | `frontend.md`, `frontend-naive-ui.md`, `security.md` |
| macOS window presentation, Liquid Glass, platform theme, or native material | `frontend.md`, `frontend-naive-ui.md`, `frontend-macos-27.md`, `backend.md` |
| Windows window presentation, Fluent 2, Mica/Acrylic, or platform theme | `frontend.md`, `frontend-naive-ui.md`, `frontend-windows-11.md`, `backend.md` |
| Frontend visual implementation or review | `frontend-visual-validation.md`, `testing-quality.md` plus the applicable platform reference |
| Rust, Tauri commands, native platform APIs, services | `backend.md` |
| SQLite schema, queries, persistence, migrations | `database.md`, `backend.md` |
| Branches, commits, pull requests, tags | `git-flow.md` |
| Dependencies, credentials, capabilities, external input | `security.md` |
| CI, installers, signing, versions, distribution | `release.md`, `git-flow.md`, `security.md` |
| Cross-cutting architecture change | All relevant references plus the active OpenSpec proposal/design/specs |

Read reference files directly from this Skill's `references/` directory. Do not rely on a remembered summary when changing governed code.

## Global Invariants

- Preserve user-owned and unrelated working-tree changes.
- Use Simplified Chinese for all user-facing output and repository documentation. README files MAY be bilingual in Simplified Chinese and English. Keep code identifiers, protocol fields, third-party API literals, and other machine-defined text in their required source language.
- Use the product term "翻译悬浮窗" (`translation overlay`). Treat `lyric`, "歌词条", and "字幕悬浮窗" identifiers as legacy names; do not introduce new uses, and rename existing uses only in a coherent migration.
- Keep Vue free of native handles, raw SQL, and platform framework types.
- Keep Tauri command handlers narrow; put domain behavior in Rust services and operating-system behavior in platform adapters.
- Store no plaintext secrets in source, logs, localStorage, SQLite, fixtures, or screenshots.
- Keep generated output, build artifacts, IDE state, and operating-system files out of Git.
- Use structured errors across process boundaries and avoid leaking native paths or handles.
- Add or update tests for changed behavior; do not represent simulated success as implemented production behavior.
- Update the relevant OpenSpec artifacts when behavior, capability contracts, or architecture changes.
- Prefer an explicit unsupported state over a silent fallback that claims success.

## Conflict Resolution

Apply instructions in this order:

1. User request and repository-local `AGENTS.md` instructions.
2. Accepted OpenSpec capability specifications and active change design.
3. This Skill and its references.
4. Existing local patterns in unaffected code.

When two sources at the same level conflict, stop the affected change, cite both rules, and resolve the decision before implementation.

## Maintaining This Skill

- Keep `SKILL.md` as routing and invariants only.
- Put detailed policy in one-level `references/` files and avoid duplicating rules.
- Update a reference in the same change that establishes a new repository-wide convention.
- Validate the Skill after editing it with the `skill-creator` quick validator.
