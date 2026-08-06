> [!WARNING]
> This project is no longer maintained as Modrinth now has a shortcut feature. 

# mrlaunch

Launch Modrinth App profiles from cli on Windows.

This cli is a thin wrapper around Modrinth's `theseus` crate.

## Commands

- `mrlaunch list`
- `mrlaunch run <profile_path>`
- `mrlaunch run <profile_path> --wait`

## Binary types:

- `mrlaunch.exe`: normal console binary (use for `list`, `--wait` or debugging)
- `mrlaunchw.exe`: tiny no-console wrapper that starts sibling `mrlaunch.exe` hidden. This will hide the console window that would normally appear using the `mrlaunch.exe` binary (use for Stream Deck, hotkeys or similar)

## Optional app id override:

- `mrlaunch --app-id com.modrinth.ModrinthApp list`
- or set env var: `MRLAUNCH_APP_ID`

## Updating to latest Modrinth launcher code

```powershell
cargo update -p theseus
```
