# 🐟 Fishtank TUI

A retro-styled terminal user interface (TUI) aquarium simulator built in Rust. Your very own Tamagotchi-style fish that lives in your terminal!

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## ✨ Features

- 🐠 **5 Unique Fish Species** - Goldfish, Betta, Guppy, Neon Tetra, Angelfish
- 🌙 **Day/Night Cycle** - Tank dims at night, fish sleep (12h real = 24h game time)
- 💧 **Water Quality System** - Manage Purity, pH, and Temperature to keep fish healthy
- ⏰ **Persistent World** - Fish age even when you're away (with smart 24hr catch-up)
- 🖼️ **Beautiful ASCII Graphics** - Clean dark tank with retro aesthetics and varied fish sprites
- 💾 **Auto-Save** - Never lose your progress (saves every 30 seconds)
- 🎮 **Casual Friendly** - Check in every 4-8 hours depending on species
- 🔋 **Lightweight** - Minimal resource usage (<10MB RAM, 825KB binary)

## 🎮 Gameplay

Your fish has needs that must be maintained:

- **🍽️ Hunger** - Feed every 6-8 hours (press `F`)
- **😊 Happiness** - Affected by care and interaction
- **❤️ Health** - Degrades if hunger/happiness are low
- **⚡ Energy** - Fish rest when tired

Fish live for approximately **30 days** if well cared for. Neglect will eventually lead to death (permanent).

### Offline Progression

When you close the app, your fish continues aging. However:
- Maximum 24 hours of decay is applied (even if away longer)
- Fish won't die while you're away - you'll have a chance to recover
- Get clear warnings when returning if fish is in danger

## 📦 Installation

### Requirements

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- A terminal (Linux, macOS, Windows)

### Quick Install

```bash
git clone https://github.com/yourusername/fishtank-TUI.git
cd fishtank-TUI
./install.sh
```

The binary will be installed to `~/.local/bin/fishtank`.

Make sure `~/.local/bin` is in your PATH:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Manual Build

```bash
cargo build --release
./target/release/fishtank
```

## 🐟 Fish Species

| Species | Sprite | Hunger Rate | Traits |
|---------|--------|-------------|--------|
| 🟡 **Goldfish** | `><>` | Normal (3.5/hr) | Balanced, classic |
| 🔵 **Betta** | `>∫>` | Slow (2.5/hr) | Flowing fins, territorial |
| 🟢 **Guppy** | `>°>` | Fast (4.5/hr) | Small, active, cheerful |
| 🔴 **Neon Tetra** | `>->` | Normal (3.0/hr) | Sleek, schools well |
| ⚪ **Angelfish** | `>^>` | Normal (3.0/hr) | Graceful, slow-moving |

## 🌙 Day/Night Cycle

The game features an accelerated time system where **1 real hour = 2 game hours**.

- **Day (06:00 - 18:00):** Bright tank, active fish 🌞
- **Night (18:00 - 06:00):** Dim blue tank, fish rest to regain energy 🌙

## 💧 Water Quality

Maintain your tank to keep fish healthy!

- **Purity:** Decreases over time and when feeding. Keep it above 80% for bonus health.
- **pH:** Ideal is 7.0. Extremes allow disease.
- **Temperature:** Ideal is 24-26°C. Fluctuates day/night.

## 🎯 Controls


| Key | Action |
|-----|--------|
| `N` | Cycle species & add fish (up to 3) |
| `F` | Feed all fish |
| `W` | Clean tank (Water change) |
| `R` | Restart tank (remove all fish) |
| `C` | Clear notification messages |
| `Q` or `ESC` | Quit (auto-saves) |

## 📁 Files

- **Save File**: `~/.config/fishtank/save.json`
- **Binary Location**: `~/.local/bin/fishtank` (after install)

## 🚀 Roadmap

This is the MVP (v0.1.0) with basic features. Future updates planned:

- [ ] Multiple fish species (Betta, Guppy, Neon Tetra)
- [ ] Breeding and genetics system
- [ ] Mini-games for interaction
- [ ] Decorations and tank customization
- [ ] XP and progression system
- [ ] Custom themes
- [ ] Water quality management
- [ ] Equipment (filters, heaters, lights)

## 🛠️ Development

```bash
# Run in debug mode
cargo run

# Run tests
cargo test

# Check code
cargo clippy
```

## 📄 License

MIT License - see [LICENSE](LICENSE) file

## 🤝 Contributing

Contributions welcome! This project is designed to be beginner-friendly.

## 🐛 Troubleshooting

**Terminal too small?**
- Minimum recommended size: 80x24
- Resize your terminal window

**Fish died immediately?**
- This might happen if you were away for a very long time
- Press `N` to add a new fish

**Can't find the binary after install?**
- Make sure `~/.local/bin` is in your PATH
- Try running: `~/.local/bin/fishtank` directly

## ❤️ Credits

Created with love for retro terminal aesthetics and virtual pet nostalgia.

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - Amazing TUI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation

---

**Start your virtual aquarium journey today! 🐠✨**
