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
  task
    add     # Add a new task
    check   # Check/uncheck a task
    todo    # Lists all unchecked tasks
    list    # List all tasks
    edit    # Edit a task
    delete  # Delete a task
  export
    entries # Export all your activities to a md file
    tasks   # Export all your tasks to a md file

  help      # Print this message or the help of the given subcommand(s)

Options:
  -h, --help    # Print help
  -V, --version # Print version
```

## Where it saves?

wokjo saves a couple files in `ProjectDirs` default folder

- Linux: `~/.local/share/wokjo/entries.jsonl` and `~/.local/share/wokjo/tasks.jsonl`
- Windows: `C:\Users\<username>\AppData\Roaming\wokjo\data\entries.jsonl` and `C:\Users\<username>\AppData\Roaming\wokjo\data\tasks.jsonl`
- macOS: `/Users/<username>/Library/Application Support/wokjo/entries.jsonl` and `/Users/<username>/Library/Application Support/wokjo/tasks.jsonl`
