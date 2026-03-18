# quran-cli

A beautiful Quran TUI for developers. Read the Quran in your terminal.

Inspired by [christ-cli](https://github.com/whoisyurii/christ-cli).

## Features

- **Offline Support**: Bundled English translation (Saheeh International).
- **Search**: Search through the Quran's translation.
- **Random**: Get a random Ayah.
- **Reference Support**: Read specific Ayahs by reference (e.g., `2:255`).

## Usage

### Read an Ayah
```sh
cargo run -- read 2:255
```

### Search
```sh
cargo run -- search "faith"
```

### Random Ayah
```sh
cargo run -- random
```

## Next Steps

- **TUI**: Implement a full-screen interactive browser similar to `christ-cli`.
- **Arabic Support**: Add original Arabic text with RTL support.
- **More Translations**: Support more languages via an online API (e.g., AlQuran.cloud).
- **Themes**: Modern, Islamic-inspired color palettes.

## License

MIT
