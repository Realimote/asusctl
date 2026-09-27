# asusctl for ASUS ROG

**English** | [简体中文](README.zh.md)

> **About this repository** — a UI remake of [upstream asusctl](https://github.com/OpenGamingCollective/asusctl).
> `rog-control-center` has been redesigned around a dark, card-based "cyber" theme with top-tab navigation, hand-drawn
> controls and Lucide line icons. The daemon (`asusd`), the CLI (`asusctl`) and the supporting libraries track upstream;
> the D-Bus interface is unchanged, so either side can be updated independently.

<p align="center">
  <a href="https://www.patreon.com/bePatron?u=7602281"><img src="extra/icons/patreon-button.svg" width="190" height="32" alt="Become a Patron" /></a>
  <a href="https://ko-fi.com/V7V5CLU67"><img src="extra/icons/ko-fi-button.svg" width="190" height="32" alt="Support me on Ko-fi" /></a>
  <a href="https://asus-linux.org/"><img src="extra/icons/rog-logo-button.svg" width="190" height="32" alt="Asus Linux Website" /></a>
  <a href="https://discord.gg/B8GftRW2Hd"><img src="extra/icons/discord-button.svg" width="190" height="32" alt="Discord" /></a>
</p>

`asusctl` is a system control utility for Linux designed primarily for ASUS ROG, TUF and ProArt laptops, with reduced functionality available for non-ASUS hardware.

> [!WARNING]
> **Kernel requirement:** many features are developed alongside Linux kernel updates. If an expected feature is missing, make sure you are running the latest stable kernel, or a kernel containing the required patches. TDP control in particular requires the `asus-armoury` driver, mainline since Linux 6.19.

![ROG Control Center](docs/assets/shared/rog-control-center.png)

## Components

| Component | Description |
| :--- | :--- |
| `asusd` | System daemon exposing hardware control over D-Bus |
| `rog-control-center` | Graphical interface (dashboard, fan curves, Aura, GPU modes, …) with tray integration |
| `asusctl` | Command-line client for `asusd` |
| `asusd-user` | Per-user daemon for AniMe Matrix and related user services |
| `asus-shutdown` | Shutdown helper that safely applies deferred GPU firmware settings |

## The control center UI

`rog-control-center` presents a single, consistent dark card-based interface. Pages are reached from a top tab bar; the **Lighting** tab opens the Aura page and carries a segmented switch between the Aura, AniMe Matrix and Slash pages, and the Dashboard's section links jump straight to the fan-curve, Aura and Slash pages.

| | |
| :--- | :--- |
| ![Fan curves](docs/assets/shared/rog-control-center-fan-curve.png) | ![Lighting](docs/assets/shared/rog-control-center-lighting.png) |
| **Fans** — per-profile, per-fan custom curves on a gradient area chart with draggable nodes | **Lighting** — effect, colour and animation controls, with the power-zone editor behind **Power Settings** |

- **Dashboard** — one page for the everyday controls: performance profile, GPU mode, display toggles, battery charge limit, keyboard lighting and Slash lighting, each with live status. Related sections sit side by side on a half-width grid.
- **System** — hardware monitor plus platform profile, EPP and PPT (CPU/GPU power limit) tuning, with a separate **Advanced** panel for throttle-policy tuning.
- **Aura** — keyboard lighting modes with per-colour HSV pickers and per-zone power behaviour.
- **AniMe Matrix** — brightness, display and built-in animation settings for equipped models.
- **Slash** — A-cover LED strip brightness, animation and visibility triggers.
- **Fans** — per-profile, per-fan custom fan curves on an interactive graph.
- **GPU** — Integrated/Hybrid/Ultimate switching, reserved GPU memory, XG Mobile LED.
- **Battery** — charge limit and battery health information.
- **Settings** — tray, autostart, notifications and the ROG/Armoury key global shortcut.

### Design system

All colours, spacing, corner radii and font sizes come from the `Theme` global in
[`rog-control-center/ui/widgets/theme.slint`](rog-control-center/ui/widgets/theme.slint) — no page-local colour hex values are used, and each page picks one of the per-page accent colours so sections stay visually distinguishable.

Three things are worth knowing before styling a page:

- **Slint's `Palette` is read-only** apart from `color-scheme`, so the standard `ComboBox`/`Slider`/`Switch` widgets cannot be re-themed. Everything that has to match the design is drawn by hand and lives in
  [`rog-control-center/ui/widgets/cyber.slint`](rog-control-center/ui/widgets/cyber.slint) (metric tiles, option cards, segmented controls, tab strips, toggles, sliders, dropdowns) — build pages from those.
- **Line icons** live in `rog-control-center/ui/icons/` (Lucide, ISC). They are rasterised by `slint-build` at compile time and tinted at run time through `Image.colorize`, so a single asset serves every accent colour.
- **Widths must be explicit** inside a section card. Slint measures the cross axis of a nested layout from its content, so `horizontal-stretch` alone cannot fill a row; use the `content-width` / `card-inner-width` pair as the Dashboard does.

### Display server support

> [!NOTE]
> X11 is officially unsupported. Users who require it may compile the GUI with X11 support enabled using `make X11=1` or `cargo build --release --features "rog-control-center/x11"`. Operation on unmaintained display servers remains the responsibility of the user.

## Implemented features

Feature availability depends on upstream kernel support and hardware capabilities.

- **Power and performance:** platform profiles with per-profile EPP and AC/battery policy switching, custom fan curves, PPT/CPU/GPU power-limit sliders, GPU MUX toggling (2022+ models)
- **Lighting:** built-in LED modes, per-key RGB, AniMe Matrix displays (G14, M16, Strix Scar 16/18), Slash lighting
- **System:** battery charge limits and health reporting, POST audio toggle, dGPU power notifications, global shortcuts through the desktop portal

Keyboard backlight support relies on the hardware mappings in [`rog-aura/data/aura_support.ron`](rog-aura/data/aura_support.ron) (installed to `/usr/share/asusd/aura_support.ron`). See the [rog-aura README](rog-aura/README.md) for details.

### Service management

`asusctl` uses `udev` rules to start its services when hardware is detected. On distributions such as Fedora or Ultramarine, enable the services manually after installation:

```sh
sudo systemctl enable --now asusd.service
sudo systemctl enable --now asus-shutdown.service
```

On Pop!_OS, disable the `system76-power` GNOME extension and its service to avoid power-profile conflicts.

Full per-distribution guides, including immutable Fedora variants, Bazzite, NixOS and PikaOS, live in the [documentation book](https://asus-linux.org/) (`docs/` in this repository).

## Building

The repository is a Cargo workspace. Everything can be driven through the `Makefile`, which wraps `cargo` and applies the right flags.

### Dependencies

| Need | Why |
| :--- | :--- |
| Rust (stable, from [rustup.rs](https://rustup.rs/)) | The pinned toolchain is in `rust-toolchain.toml` and is installed automatically |
| `gettext` (`msgfmt`) | `rog-control-center/build.rs` compiles the `.po` catalogues into `.mo` files at build time |
| C toolchain, `clang`/`llvm`, `pkg-config`, `cmake` | Native dependencies of the workspace crates |
| GTK3 / at-spi2 / cairo | Slint's windowing backend and accessibility |

On Arch Linux:

```sh
sudo pacman -S --needed --asdeps git cmake clang pkg-config libzip rust openssl gettext
```

### Build and install

```sh
make                 # release build of the default members (cargo build --release --locked)
make DEBUG=1         # development build (dev profile, much faster to iterate on)
make X11=1           # add the X11 backend to the GUI
make STRIP_BINARIES=1  # strip the release binaries

sudo make install    # install binaries, desktop file, icons, hardware data and translations
sudo make uninstall  # remove them again
```

Useful variants:

```sh
# Stage into a package root instead of the live system (this is what the
# distribution packages do):
make DESTDIR=/tmp/asusctl-pkg install

# Build without network access, using the versions pinned in Cargo.lock:
make FROZEN=1

# Regenerate the gettext template after adding or changing @tr() strings:
make translate        # needs: cargo install slint-tr-extractor --version 1.17.1

make clean            # cargo clean (removes ./target)
make distclean        # also removes .cargo and vendor/
```

After installing, remove any leftover configuration in `/etc/asusd/`. For package installations, use your distribution's package manager instead.

### Packaging

**Arch Linux** — the in-repo PKGBUILD builds and installs through `make`:

```sh
cd distro-packaging
makepkg -si          # or: makepkg -s --noconfirm  in CI
```

**Debian / Ubuntu** — `rog-control-center/Cargo.toml` carries `[package.metadata.deb]` for [`cargo-deb`](https://github.com/kornelski/cargo-deb), and the rest of the workspace installs through the Makefile. The deb metadata refers to the file layout produced by `make DESTDIR=… install`, so stage that first:

```sh
make DESTDIR=/tmp/asusctl-pkg install     # produces the /usr tree to be packaged
```

**Fedora / openSUSE / anything else** — stage with `DESTDIR` as above and feed the result to `rpmbuild`, `fpm`, or your distribution's build system. The full binary/program/data split is defined by the `install-*` targets in the [Makefile](Makefile).

## Development

Daemon-side architecture patterns are documented in [docs/dev/design-patterns.md](docs/dev/design-patterns.md), user and distribution documentation in `docs/` (an mdBook), and the CLI reference in [MANUAL.md](MANUAL.md).

### Running the UI without hardware

`rog-control-center` has a demo mode that runs the full interface with representative fake data — no `asusd`, no dbus, no ASUS hardware required. It is the fastest way to work on the UI:

```sh
cargo build -p rog-control-center
./target/debug/rog-control-center --demo

# Open directly on a specific page (0 = dashboard … 9 = about)
ROGCC_DEMO_PAGE=1 ./target/debug/rog-control-center --demo
```

All controls are interactive and update only the demo state.

To preview the workspace translations instead of the ones installed system-wide, set `RUST_TRANSLATIONS=1`.

### AniMe Matrix simulator

An SDL2-based simulator is included for testing matrix rendering without hardware:

```sh
cargo build --package rog_simulators
./target/debug/anime_sim
```

Restart `asusd` after starting the simulator to attach it to the simulated display.

### Tests and linting

```sh
cargo test
cargo clippy
```

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening pull requests.

## License

This project is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](LICENSE).

---

ASUS and ROG are registered trademarks of ASUSTeK Computer Inc. in the United States and other jurisdictions. References to ASUS products, services, or trademarks within this repository do not constitute or imply endorsement, sponsorship, or recommendation by ASUSTeK Computer Inc. Trademarks are used solely for hardware identification purposes.

---

## AI Disclaimer

We do not accept code blindly written with just AI or "vibecoding". We encourage use of AI for finding bugs and as a tool used to assist development, but all of these must be verified by a human as AI makes mistakes and gives false bug reports as well. For further details, refer to [our contribution policy](./CONTRIBUTING.md)
