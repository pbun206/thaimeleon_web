# Thaimeleon

![Gif demo of Thaimeleon palettes](https://codeberg.org/thairanaru/thaimeleon/raw/branch/main/assets/gifs/demo.gif)

An opinionated color scheme generator that uses a wallpaper input!

* Supports for light themes ☀️ and dark themes 🌑

* Generates [a forty-five color palette](./palette.md), ensuring accessibility for every TUI and GUI situation! 

* Uses [k-means](https://en.wikipedia.org/wiki/K-means_clustering), [weighted ward's method of minimum variance](https://en.wikipedia.org/wiki/Ward%27s_method), and [Oklab](https://bottosson.github.io/posts/oklab/) to determine representative background colors and foreground colors of a wallpaper

* Uses [Oklch](https://bottosson.github.io/posts/oklab/) to ensure perpetual chromatic clarity 🌈

* In addition to json, toml, yaml, and ron, Thaimeleon can generate a rhai file to allow perfect integration with [Yolk](https://elkowar.github.io/yolk/) ⚙️

* Thaimeleon uses Rust 🦀

* Configuration file to change contrast to how you like it

* Enable cache to make Thaimeleon run at lightning speed ⚡

* Although not compliant to, this project uses a formula built on Andrew Somers' [DPS Contrast](https://github.com/Myndex/deltaphistar/tree/master), in order to retain reasonable contrast

## Installation

### From crates.io (Stable Version)

With cargo installed, do the following command in your shell:    
```bash
cargo install thaimeleon
```

Make sure the cargo binary location is in PATH

### Compile from source (Unstable Version)

Git version currently lack features that the stable version has and vice-verse due to an ongoing rewrite. There is no documentation for all the changes yet.

The biggest one is that the command line tool has been renamed to "thaimel".

With cargo and git installed, do the following commands in your shell:

```bash
git clone https://codeberg.org/thairanaru/thaimeleon.git
cd thaimeleon
cd thaimel
cargo install --path .
```

## Usage

The recommended method of using Thaimeleon is to generate a palette file, which is then processed by a dot file manager. For reference, in my wallpaper changer script, I have:

```bash
    thaimeleon "$selected_wallpaper" -w ~/.config/yolk/chameleon.rhai 
    yolk sync
```

However, thaimeleon will adjust contents by the file extension. Currently, json, toml, yaml, rhai, and ron are supported. 

For details of the color palette, read [here](palette.md).

To enable experimental [base16](https://github.com/chriskempson/base16/tree/main?tab=readme-ov-file) output, add the `--base16` or `-b` flag. Note that Thaimeleon isn't designed around base 16 (because Thaimeleon color palette is superior) nor I have tested results.

For screenshots, add the `--print-pretty-palette` or `-p` flag. 

## Configuration

Thaimeleon can be configured in `~/.config/thaimeleon/thaimeleon.toml`. Generate a default config with `thaimeleon --generate-config-file` and read the default config file for documentation. I recommend turning on cache once you get settled. It does have options to configure Thaimeleon's algorithm but caution! Once you start changing the defaults, you now hold responsibility for accessibility. For a more detailed understanding, check out [here](https://codeberg.org/thairanaru/thaimeleon/src/branch/main/how-it-works.md) for information how the algorithm works.

# Road Map

Here are some things planned for next release:

- [ ] Thaimeleon library 
    - [x] Replace distance formulas with CIED2000
    - [x] ~~Hue data from [UW's color dataset](https://github.com/uwdata/color-naming-in-different-languages/)~~
    - [ ] ~~Expand Thaimeleon's palette to support color schemes that flips foreground and background for colored surfaces~~
        - [ ] ~~Make a condition for Thaimeleon to filp polarity for colored surfaces~~
    - [ ] Material you comptiable output
    - [ ] Document extensions of Thaimeleon palette
- [ ] Thaimeleon CLI
    - [x] Refactor code
- [x] Add css output
- [x] Rewrite config integration
    - [x] In particular, adding marcos
- [ ] Add debug/verbose printing
<!-- - [ ] Better clustering algorithm -->

# Contributing

Pull requests are welcomed, though be warned that I am entirely self taught and haven't programmed since I was 12, so the codebase might be difficult to work with.

The algorithm is constructed largely through experimentation and personal taste rather than scientific theory. In other words, feel free to play around with it. Check out [here](https://codeberg.org/thairanaru/thaimeleon/src/branch/main/how-it-works.md) for information how the algorithm works.

Suggestions for dot file manager/theming templating integration are also welcomed. As you see, I only use yolk.

# Licensing 

With the exception of the dps_contrast module in `src/contrast.rs`, this project is licensed under [the AGPL v3](LICENSES/AGPLv3.md).

The dps_contrast module is licensed under [the DPS CONTRAST (DELTA PHI STAR) LICENSE, a modified version of the AGPL v3 license](LICENSES/DPS_CONTRAST_LICENSE.md). All rights of the formula to Andrew Somers.

For the licenses of the images shown in the demo, check out [LICENSES/licenses.md](./LICENSES/license.md)

# Similar projects to try out

- [Matugen](https://github.com/InioX/matugen) - uses Material You which uses HCT, so this should be good. I haven't tried this personally yet, but I don't see how anything could go wrong with that approach.
