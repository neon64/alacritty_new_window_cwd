# Open a new Alacritty window in the same directory with Sway

**DEPRECATED**: this has now been integrated into Alacritty itself [#1971](https://github.com/alacritty/alacritty/pull/1971). You can add this to your config:

```yml
key_bindings:
  - { key: N,        mods: Control|Shift,    action: SpawnNewInstance    }
```

## Motivation

This project was a quick response to [this issue with Alacritty](https://github.com/jwilm/alacritty/issues/808).

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