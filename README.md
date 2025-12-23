# Quick Search

A highly performant and easy to use search bar built in Rust.

---

## Functionalities
- **Open applications**
- **Open paths** (Directories are opened with the file manager, files are opened using the default applications)
- **Open URL's**
- **Do math**
- **Search on the web** (fallback option)
- **Run commands** (Disabled by default, can be enabled in settings)
- **Save history** (Can be turned off in settings)

---

## Prerequisites

- **Linux**
- **One of the following Tiling Window Managers:**
    - Hyprland
    - Sway
    - i3
    - Awesome
    - Bspwm
    - Dwm
    - Herbstluftwm
    - Leftwm
    - Qtile
    - River
    - Xmonad
    - (Might work on other ones too but its not confirmed and will not be tested)

---

## Installation

### 1. Download

Download the latest executable from
**[GitHub Releases](https://github.com/Lucop1911/quick_search/releases/latest)**

---

### 2. Make the file executable

Open a terminal in the directory where you downloaded the file and run:

```bash
chmod +x quick_search
```

---

### 3. Move the binary to a `bin` directory

You can install it **locally (recommended)** or **globally**.

#### Option A: Local install (no root required)

```bash
mkdir -p ~/.local/bin
mv quick_search ~/.local/bin/
```

Make sure `~/.local/bin` is in your `$PATH`:

```bash
echo $PATH | grep -q "$HOME/.local/bin" || echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

Restart your shell afterward.

#### Option B: Global install (system-wide)

```bash
sudo mv quick_search /usr/local/bin/
```

---

### 4. Verify installation

Run:

```bash
quick_search
```

If the application opens, the installation was successful.

---

### 5. Add a keybind in your Window Manager

Add a keybinding in your window manager configuration to launch **Quick Search**.

Below are some common examples — adjust as needed.

#### i3 / Sway

```conf
bindsym $mod+d exec quick_search
```

Reload the config:

```bash
$mod+Shift+r
```

---

#### Hyprland

```conf
bind = SUPER, D, exec, quick_search
```

Reload Hyprland:

```bash
hyprctl reload
```

---

#### bspwm (sxhkd)

```conf
super + d
    quick_search
```

Reload sxhkd:

```bash
pkill -USR1 -x sxhkd
```

---

#### AwesomeWM

```lua
awful.key({ modkey }, "d",
    function () awful.spawn("quick_search") end,
    {description = "open quick search", group = "launcher"})
```

Restart AwesomeWM.

---

## License

This project is licensed under the **Apache License 2.0**.
See [LICENSE](LICENSE) for details.