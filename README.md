# Celestia

A command-line tool for managing Minecraft servers.

## Install

Install from GitHub using cargo:

```bash
cargo install --git https://github.com/nicdgonzalez/celestia.git celestia
```

### Shell Autocomplete

Use the `completions` subcommand to generate the script for your preferred
shell, then redirect the output to the necessary path:

> [!NOTE]\
> You need to rerun this snippet each time you update the `celestia` binary.

```bash
# NOTE: This example assumes bash.
output="${XDG_DATA_HOME:-$HOME/.local/share}"/bash-completion/completions/celestia
mkdir --parents "$(dirname "$output")"
celestia completions bash > "$output"
source "$output"
```

See `celestia completions --help` for a list of supported shells.
