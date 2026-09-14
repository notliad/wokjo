# wokjo

A CLI minimalist work journal

## Installation

### Build & Install

```bash
git clone https://github.com/notliad/wokjo.git
cd wokjo
cargo install --path .
```

### Direct Install

```bash
curl -fsSL https://raw.githubusercontent.com/notliad/wokjo/main/install.sh | bash
```

## Usage

```bash
wokjo <COMMAND>

Commands:
  add       # Log a new work activity
  edit      # Edit an entry
  delete    # Delete an entry
  today     # Show today's activities
  yesterday # Show yesterday's activities
  day       # Show activities from that day
  list      # List all logged activities
  week      # Show activities from the current week
  help      # Print this message or the help of the given subcommand(s)

Options:
  -h, --help # Print help
```

## Where it saves?

wokjo saves a single file in `ProjectDirs` default folder

- Linux: `~/.local/share/wokjo/entries.jsonl`
- Windows: `C:\Users\<username>\AppData\Roaming\wokjo\data\entries.jsonl`
- macOS: `/Users/<username>/Library/Application Support/wokjo/entries.jsonl`
