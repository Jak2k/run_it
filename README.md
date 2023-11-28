# run_it

A simple task runner for running tasks in every project, but with the same command.

## Installation

```bash
cargo install run_it
```

## Usage

### Init

```bash
run init
```

### Run development command

```bash
run dev
```

### Run build command

```bash
run build
```

and so on...

## Configuration

### Config file

```toml
# Tasks.toml
[commands]
dev = "cargo run"
build = "cargo build --release"
<task_name> = "<command>"
```

## Contributing

Feel free to open an issue or a pull request. Any help is appreciated!
