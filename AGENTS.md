# AGENTS.md - quran

## Repository

`quran`, a command-line tool that prints Quranic text for a reference query,
such as `quran 2:255` or `quran 2:12-16`. 

- `crates/quran/`: the `quran` CLI.
- Reference grammar: `ai/specs/reference-grammar/spec.md`.

## Collaboration

- The user is responsible for the project's quality. Help them make sound
  judgments; engage until the reasoning is sound and they agree.
- Collaborate on high-impact decisions. Use balanced judgment for day-to-day
  work. When unsure, ask clarifying questions.
- Use straightforward, simple and direct language. No hyperbole.

## Working

- Use `ai/` as the agent workspace. Read `ai/agents/workspace.md` for
  locations and file naming; its paths override skill defaults.
- Before changing a file, read its notes in `ai/comments/<path>.md` if
  present. Record non-obvious facts about a file there, not in code comments.
- Treat `my/`, if present, as the human workspace. Read it for context; edit
  it only when the user asks.
- Delegate substantive implementation and research to dedicated subagents.
  Sequence dependent tasks; run independent tasks in parallel.
- Track pending work in `ai/backlog.md`. Move finished items to `ai/done.md`.
- Read `ai/glossary.md` before choosing terminology or filename keywords.
  Maintain it, and harmonize affected documents when a definition changes.
- One logical change per commit, using Conventional Commits.
- Identify yourself in documents you author: your agent name and model,
  as `@agent (model)`, for example `@codex (GPT-6.1-Sol)`. Follow the
  authorship and changelog rules in `ai/agents/workspace.md`.

## Development

Enter the environment with `devenv shell`. Build with `cargo build -p quran`;
test with `cargo nextest run`.

## Content

- Never alter the Quranic text. Ship source files verbatim.
- Comply with each source's licence and credit it clearly.
- Fail on invalid references and data mismatches; never silently repair or
  clamp them.

## Writing

- Prefer terse prose. Cite primary sources and distinguish findings,
  recommendations, assumptions and open questions.
