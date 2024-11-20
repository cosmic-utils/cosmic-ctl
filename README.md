# cosmic-ctl

CLI for COSMIC Desktop configuration management.

## Overview

cosmic-ctl (short for COSMIC configuration utilities) is a command-line interface for managing the configuration of the COSMIC Desktop.
It allows users to read, write, delete, and backup configuration entries for various components.

## Features

- Write: Add or update a configuration.
- Read: Retrieve a configuration value.
- Delete: Remove a configuration.
- Apply: Write configurations from a JSON file.
- Backup: Backup all configuration entries to a JSON file.

## Installation

### Build from source

You can build this project using Cargo:

```bash
cargo build --release
```

## Usage

### Commands

- Write

```bash
cosmic-ctl write --component <component> --entry <entry> --version <version> <value>
```

- Read

```bash
cosmic-ctl read --component <component> --entry <entry> --version <version>
```

- Delete

```bash
cosmic-ctl delete --component <component> --entry <entry> --version <version>
```

- Apply

```bash
cosmic-ctl apply /path/to/json/file
```

- Backup

```bash
cosmic-ctl backup /path/to/output/json/file
```

# LICENSE

This project is licensed under the `GPL-3.0-or-later` license. See the [LICENSE](LICENSE) for details.
