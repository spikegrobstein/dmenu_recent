# dmenu_recent

CLI for managing a basic database for recent items to use with `dmenu`. Supports defining number of recent
entries and deduplication.

Mostly being used for myself. No kind of release. For usage, see:

    cargo run -- --help

## Quickstart

Build the project:

    cargo build --release

Copy the executable to your `PATH`:

    sudo cp target/release/dmenu_recent /usr/local/bin/

Create a `demenu_run_recent` script:

```bash
#!/bin/sh

# a special version of dmenu_run that remembers the most-recently used items
# and shows them first when the menu first pops up

# this is the default file path. needed to prepend the recent items
RECENT_FILE="${1:-$HOME/.dmenu.recent}"

{
  # if RECENT_FILE exists, cat it so its items show up first
  # in the dmenu
  [[ -s "$RECENT_FILE" ]] \
    && cat "$RECENT_FILE"

  dmenu_path
} \
  | dmenu "$@" \
  | dmenu_recent "$RECENT_FILE" \
  | ${SHELL:-"/bin/sh"} &

```

Then, set up your hot-key to use this script rather than the current `dmenu_run`.

That's it.

## License

MIT Licensed. See `LICENSE`.
