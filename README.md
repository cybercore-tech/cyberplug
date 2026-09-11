# cyberplug

Terminal plugin manager for Omarchy's Quattro shell — discover, install, enable,
configure, and profile your plugins without leaving the keyboard.

Ships as an Omarchy bar widget. One command installs the widget **and** the
manager binary.

## Install

```bash
omarchy plugin add https://github.com/darkstardevx/cyberplug.git --enable
```

Click the plug icon on the bar. That's it.

The plugin checkout includes a bundled `cyberplug` binary for `linux-x86_64`.
On other architectures the launcher downloads a GitHub release asset, or builds
from source with `cargo` if needed.

## Remove

```bash
omarchy plugin remove io.github.darkstardevx.cyberplug
```

## Use

```bash
cyberplug   # optional: run the TUI from any terminal after copying the binary
```

Or just use the bar icon. Inside the TUI:

### Main screen

```
j/k, ↑/↓    move
/           filter installed plugins
a           add plugin by git url
e           enable (pick placement: left, center, right)
d           disable
x           remove (confirms)
s           settings (if the plugin has any)
D           discover — browse the community registry
P           profile — export or import your whole setup
u           update selected
U           update all
q, Esc      quit
```

### Discover screen

Browses the live community registry from
[plugins.omarchy.org](https://plugins.omarchy.org).

```
j/k, ↑/↓    move
h/l, ←/→    switch category tab
ENTER       install — then walks into placement
r           force-refresh the registry
q, Esc      back
```

## What it wraps

Every action shells out to the real `omarchy plugin` CLI. cyberplug reads
`omarchy plugin catalog` and calls `enable` / `disable` / `add` / `remove` /
`update` for state changes.

## Developer install (rebuild binary)

```bash
git clone https://github.com/darkstardevx/cyberplug.git
cd cyberplug
./install.sh
```

That rebuilds a release binary into `bin/linux-$(arch)/cyberplug` (and optionally
`~/.local/bin` for CLI use outside the bar).

## Docs

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/SETTINGS.md](docs/SETTINGS.md)
- [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)

## Requirements

Omarchy with the Quattro shell. Internet optional after install on x86_64
(bundled binary). Discover needs network (falls back to the last cached registry).

## License

MIT
