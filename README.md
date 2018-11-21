# Open a new Alacritty window in the same directory with Sway

A quick response to this issue: https://github.com/jwilm/alacritty/issues/808.

No means foolproof. Probably will break everything, but try it anyway.

## Usage instructions

Sway keybinding:

```
set $term ~/Code/Rust/alacritty_new_window_cwd/target/release/alacritty_new_window_cwd
bindsym $mod+Return exec $term
```

Alacritty config

```yaml
- { key: T,        mods: Control,          command: { program: "bash", args: ["-c", "~/Code/Rust/alacritty_new_window_cwd/target/release/alacritty_new_window_cwd"] } }
```