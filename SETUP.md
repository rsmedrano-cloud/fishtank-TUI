# 🐠 Fishtank - Quick Setup

## Installation Complete! ✅

The fishtank binary is now installed at: `~/.local/bin/fishtank`

Your `.bashrc` has been updated with the PATH.

## How to Run

### Option 1: Current Terminal Session
```bash
export PATH="$HOME/.local/bin:$PATH"
fishtank
```

### Option 2: New Terminal
Just open a new terminal and run:
```bash
fishtank
```

## First Time Playing

1. **Start**: `fishtank`
2. **Add fish**: Press `N` (up to 3 times for 3 fish)
   - 1st fish: "Goldie"
   - 2nd fish: "Bubbles"  
   - 3rd fish: "Splash"
3. **Feed them**: Press `F` (feeds all fish)
4. **Quit**: Press `Q`

## Controls

- `N` - Add new fish (max 3)
- `F` - Feed all living fish
- `C` - Clear messages
- `Q` / `ESC` - Quit and save

## What You'll See

```
┌─────────────────────────────────────┐
│ 🐟 Fishtank          │ 📊 Status    │
├──────────────────────┼──────────────┤
│                      │ 🐟 Goldie    │
│   ><>                │ 🟢 H█████    │
│                      │              │
│        <><           │ 🐟 Bubbles   │
│                      │ 🟢 H█████    │
│  Y                  Y│              │
│  Y      ><>         Y│ Alive: 3/3   │
│ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│              │
├──────────────────────┴──────────────┤
│ [F]eed  [N]ew  [C]lear  [Q]uit      │
└─────────────────────────────────────┘
```

Dark background, no water characters - just fish swimming in space!

## Troubleshooting

**"fishtank: command not found"**
- Run: `export PATH="$HOME/.local/bin:$PATH"`
- Or open a new terminal

**App crashes on start**
- Old save file issue (already fixed!)
- Your old save was backed up to `~/.config/fishtank/save.json.backup`
- Fresh start created automatically

## Save Location

All your fish data is saved at:
```
~/.config/fishtank/save.json
```

Auto-saves every 30 seconds!

---

**Have fun raising your virtual fish! 🐠✨**
