# Quran CLI 📖

A beautiful, high-performance Terminal User Interface (TUI) for reading the Quran, built with Rust and Ratatui. Designed for developers and terminal enthusiasts who want a distraction-free reading experience.

![Quran CLI Preview](https://github.com/whoisyurii/quran-cli/raw/main/assets/preview.png)

## ✨ Features

- **🚀 Fast Navigation**: Browse through all 114 Surahs with a real-time reactive interface.
- **📖 Scripture Pane**: Beautifully formatted English translation with Quranic text.
- **🔍 Live Search**: Instant full-text search across the entire Quran with Ayah jumping.
- **🎨 Themes**: Multiple curated color palettes:
  - `Slate` (Classic Dark)
  - `Emerald` (Nature Green)
  - `Sand` (Warm Desert)
  - `Night` (Deep OLED Black)
- **📏 Responsive**: Adapts to any terminal size with automatic text wrapping and scroll capping.
- **🔌 Offline First**: All data is bundled in the binary—no internet connection required.

## 📥 Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
# Clone the repository
git clone https://github.com/whoisyurii/quran-cli
cd quran-cli

# Build and install
cargo install --path .
```

## 🚀 Usage

### Terminal User Interface (TUI)
Simply run the command to open the interactive browser:
```bash
quran
```

**Keybindings:**
- `h/l` or `Left/Right`: Switch between Surah list and Scripture.
- `j/k` or `Up/Down`: Navigate through Surahs or scroll verses.
- `/`: Open Search mode.
- `t`: Cycle through themes.
- `Esc` or `q`: Quit.

### CLI Commands
- **Search**: `quran search "mercy"`
- **Read**: `quran read 2:255`
- **Random**: `quran random`

## 🛠️ Tech Stack
- **Rust**: Language
- **Ratatui**: TUI Framework
- **Crossterm**: Terminal backend
- **Serde**: Data serialization

## 📄 License
Licensed under the [MIT License](LICENSE).
