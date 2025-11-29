# Celestia

A collection of tools for managing Minecraft servers.

This project has two interfaces: a command-line interface, and a web dashboard.

The command-line interface is designed for developers and contains convenient
commands for managing servers programatically. The web dashboard aims to be
more user friendly and convenient; it requires a Minecraft server plugin to be
installed to receive real-time updates from the Minecraft world.

> [!NOTE]\
> The only server provider currently supported is [PaperMC].

## Installation

**Requirements**:

| Requirement | Version | Reason                            |
| :---------- | :------ | :-------------------------------- |
| cargo       | 1.93.0  | Build the project                 |
| tmux        | 3.5a    | Attach/detach from server console |
| java        | 21.0.8  | Run the Minecraft server          |
| gradle      | 9.1.0   | Build the Minecraft server plugin |
| sqlx        | 0.8.6   | Prepare the database              |

Install from GitHub using cargo:

```bash
cargo install --git https://github.com/nicdgonzalez/celestia -- celestia
```

## Quickstart

Create the database:

```bash
sqlx database create
```

Start the web dashboard:

> [!TIP]\
> This will block the current process; run this command in a separate window.

```bash
# TODO: Not implemented yet.
# celestia dashboard [--hostname 0.0.0.0 | --external] --port 1140 [--open]
```

Create a new Minecraft server and add the Celestia plugin:

> [!NOTE]\
> `--name` is optional and refers to how the server will be displayed in the
> web dashboard.

```bash
# TODO: Not implemented yet.
# celestia new [--path ./smp] [--name "Survival Multiplayer"]
```

Or, if you already have an existing server:

```bash
# TODO: Not implemented yet.
# cd /path/to/minecraft/server
# celestia init [--name "Survival Multiplayer"]
```

Start the Minecraft server:

```bash
# TODO: Not implemented yet.
# celestia start
```

List of all the available subcommands:

```bash
celestia --help
```

[papermc]: https://papermc.io/
