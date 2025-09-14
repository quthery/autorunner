# autorunner

A simple file watcher that automatically restarts processes when files or directories change.

## Installation

### Build from source

```bash
git clone https://github.com/quthery/autorunner 
cd autorunner
cargo build --release
```

### Install to system path (macOS/Linux)

```bash
# Build and install to /usr/local/bin
cargo build --release
sudo cp target/release/autorunner /usr/local/bin/
```

Or build and install in one command:
```bash
cargo install --path . --root /usr/local
```

## Usage

```bash
autorunner --path <FILE_OR_DIRECTORY> --command <COMMAND>
```

**Options:**
- `-p, --path`: Path to file or directory to watch
- `-c, --command`: Command to execute and restart on changes

The tool monitors the specified path (file or directory) for changes and automatically kills and restarts the running process whenever modifications are detected. For directories, it recursively watches all files within them.