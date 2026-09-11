# Security Policy

cyberplug shells out to the `omarchy plugin` CLI and fetches the community
registry over HTTPS (see [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for
what each file is allowed to touch). If you find a security issue — anything
from a shell-injection risk in how a plugin id/URL is passed to `omarchy`, to
an unsafe fallback in `cyberplug-toggle`'s binary bootstrap — please report
it privately rather than opening a public issue.

## Reporting a vulnerability

Email **darkstardevx@gmail.com** (primary) or, as a backup,
**cybercore.sh@gmail.com**. Include:

- the affected file/commit and a minimal repro or PoC
- what you'd expect to happen instead
- how you'd rate the impact (your best guess is fine)

Expect an acknowledgement within a few days. Please don't include exploit
details in a public GitHub issue or PR until a fix has shipped.

## Supported versions

Only the latest tagged release and `main` are supported. There's no LTS
branch at this stage of the project.
