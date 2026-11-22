# Working on MUON

A swarm needs units it can swap and compare, so MUON designs one learning unit behind a common interface; HENA's swarm is made of many such units. The `muon-kernel` crate re-exports MONAD's types. The unit coordinator and model adapters are still to be built. The first stock implementation can start on today's kernel types; MONAD's mathematical research remains paused. Later innovations and stage acceptance keep their own scope and evidence requirements.

Start with [README.md](README.md), the [unit design](docs/index.md) and the [build guide](docs/building.md). `kernel/` holds Rust, `paper/` the draft and shared definitions, and `protos/` the proposed language interface.

## Build and check

The kernel fetches MONAD's kernel at the commit `kernel/Cargo.toml` pins. Run from this root:

```sh
cargo check --manifest-path kernel/Cargo.toml
make -C paper all
```

The paper needs Make, Bash, a TeX installation with `pdflatex` and `bibtex`, and `pdftotext` for the rendered-text check. Read the build guide's paper checks and inspect the PDF. Compilation checks the re-export; the [research statement](paper/sections/00_problem_statement.tex) defines proposed work and does not establish a working unit.

<!-- BEGIN shared repository rules -->

- Review a source file above 1,000 lines and split it by concern before 1,500.
- Keep Rust tests under `tests/`; keep Go's `_test.go` files beside their package.
- Finish a goal with the codebase the same size or smaller unless a requested feature requires growth. Preserve the behaviour, tests, cases and examples.
- Every engine must build from a fresh clone on Linux and macOS.
- Keep generated API reference and build output out of Git; change their source or generator.
- Write comments and API documentation for the code's reader. A crate or module comment first says what it is for in the system, what it builds on and what builds on it, then its purpose, inputs, results and useful edge cases, with the purpose written problem-first in the present tense: the problem it solves and what solves it today. One plain line after that opening says what is not built yet, as the problem now in front of it, and a negative appears only where a reader would expect the opposite: a limit stated as a property of what exists stays ("it runs on loopback with no caller authentication"), while a negative measured against the finished design is a hedge. Keep them self-contained, with links inside the repository.
- Push each repository as one commit whose whole message is `init`, with Harsh as author and committer under the account the folder uses and no `Co-Authored-By` or `Claude-Session` trailers; later commits stay short and plain.
- Describe the present design and research. The repository neither links nor ships lineage: earlier papers, predecessor versions, history tags and the commits behind them.
- Public repository files, including generated text, link within the repository or to public addresses their readers can open.
- A README is the repository's front door: it carries the product account for the technical reader who arrives there, opening with the product's plain name and the problem it solves, in the words its published pages use.
- Name every operation, command, tool, type, field and variable so a reader without its author's context can tell what it does, what it enables and when to use it; docstrings say the same. Keep full-field record schemas for the engine's consistency, named in plain words like everything else, and author each surface's schemas separately, one per intent, in its caller's terms: SDKs and APIs, the command line, the language server and the agent protocols (MCP, A2A, ACP) are different surfaces with different callers, and the agent protocols compose intents over the operations the others expose.
- Write each Markdown paragraph on one source line. Run the repository's formatter with `proseWrap: never` on changed Markdown.

<!-- END shared repository rules -->
