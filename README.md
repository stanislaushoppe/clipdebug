# Clipboard workaround for Wayland/Hyprland 

there is a Bug for some of us, where you can't copy text from X11 applications to Wayland application
for example if you copy something from a xfreerdp to kitty or to a Folder sometimes kitty  wayland application will crash or hang for several seconds.

Using this tool the clipboard gets monitored, and if there is some clipboard changes from x11 applications, the data will simply transfered to wl-copy.

## Features

- 🔍 Monitors X11 Application clipboard changes 
- 📋 Copy the right Data to wl-copy
- ⚡ Asynch implementation for better Performance
- 🔢 Shows Hex form of the Clipboard-Data only in -debug mode
- 🎨 ASCII-Preview only in -debug mode

## Compile

```bash
# clone or download the project using git clone
cd clipdebug

# Dependencies installieren und kompilieren
cargo build --release
```

## Usage

```bash
# Debug-Version starten
cargo run

# Oder die kompilierte Binary verwenden
./target/release/clipdebug
```

## tested on

- Arch Linux with Wayland + Hyprland
- Rust 1.70 or higher
- wl-clipboard installed. 
- tested without xclip.
- tested copy files to a folder from xfreerdp3 with +clipboard option rdp connection.

you can autostart it using exec-once in Hyprland.

## End

Press`Ctrl+C` to end it.
