# asusctl for ASUS ROG

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

`rog-control-center` presents a single, consistent dark card-based interface:

- **Dashboard** — one page for the everyday controls: performance profile, GPU mode, display toggles, battery charge limit, keyboard lighting and Slash lighting, each with live status.
- **System** — hardware monitor plus platform profile, EPP and PPT (CPU/GPU power limit) tuning.
- **Aura** — keyboard lighting modes with per-colour HSV pickers and per-zone power behaviour.
- **AniMe Matrix** — brightness, display and built-in animation settings for equipped models.
- **Slash** — A-cover LED strip brightness, animation and visibility triggers.
- **Fans** — per-profile, per-fan custom fan curves on an interactive graph.
- **GPU** — Integrated/Hybrid/Ultimate switching, reserved GPU memory, XG Mobile LED.
- **Battery** — charge limit and battery health information.
- **Settings** — tray, autostart, notifications and the ROG/Armoury key global shortcut.

The whole interface is built from a shared design system: all colours, spacing, corner radii and font sizes come from the `Theme` global in
[`rog-control-center/ui/widgets/theme.slint`](rog-control-center/ui/widgets/theme.slint), and pages are composed from common widgets (cards, section headers, info banners, slider/toggle/dropdown rows) in `rog-control-center/ui/widgets/`. If you want to adjust the look of the app, start there — no page-local colour hex values are used.

## Hardware and kernel compatibility

### Supported laptops

`asusctl` supports most ASUS gaming laptops equipped with a USB keyboard. To verify device compatibility, run `lsusb` and check for entries matching:

```plain
Bus 001 Device 002: ID 0b05:1866 ASUSTek Computer, Inc. N-KEY Device
```

or

```plain
Bus 003 Device 002: ID 0b05:19b6 ASUSTek Computer, Inc. [unknown]
```

Devices with these hardware IDs typically work without extra configuration. Features such as AniMe Matrix, LED controls and Slash displays work regardless of laptop make, but newer models may require explicit support — see [Laptop support requests](#laptop-support-requests).

Battery charge thresholds use generic kernel interfaces and work on non-ASUS hardware; platform and fan controls require the ASUS `asus-nb-wmi` or `asus-armoury` drivers.

### Display server support

> [!NOTE]
> X11 is officially unsupported. Users who require it may compile the GUI with X11 support enabled using `cargo build --features "rog-control-center/x11"`. Operation on unmaintained display servers remains the responsibility of the user.

## Implemented features

Feature availability depends on upstream kernel support and hardware capabilities.

- **Power and performance:** platform profiles with per-profile EPP and AC/battery policy switching, custom fan curves, PPT/CPU/GPU power-limit sliders, GPU MUX toggling (2022+ models)
- **Lighting:** built-in LED modes, per-key RGB, AniMe Matrix displays (G14, M16, Strix Scar 16/18), Slash lighting
- **System:** battery charge limits and health reporting, POST audio toggle, dGPU power notifications, global shortcuts through the desktop portal

Keyboard backlight support relies on the hardware mappings in [`rog-aura/data/aura_support.ron`](rog-aura/data/aura_support.ron) (installed to `/usr/share/asusd/aura_support.ron`). See the [rog-aura README](rog-aura/README.md) for details.

## Installation

Pre-built packages are available in several distribution repositories — check your package manager before building from source.

| Distribution | Source | Install |
| :--- | :--- | :--- |
| **Ultramarine / Nobara** | Official repositories | `sudo dnf install asusctl` |
| **Fedora** | [Terra](https://terrapkg.com/) | `sudo dnf install asusctl` |
| **openSUSE** | [OBS](https://download.opensuse.org/repositories/home:/luke_nukem:/asus/) | Add the OBS repository |
| **Arch Linux** | [OGC Arch repository](https://github.com/OpenGamingCollective/ogc-arch-packaging) | See the OGC Arch guide |
| **Nix / NixOS** | Nixpkgs | `nix-env -iA nixpkgs.asusctl` |
| **Solus** | Official repositories | `sudo eopkg install asusctl` |

### Service management

`asusctl` uses `udev` rules to start its services when hardware is detected. On distributions such as Fedora or Ultramarine, enable the services manually after installation:

```sh
sudo systemctl enable --now asusd.service
sudo systemctl enable --now asus-shutdown.service
```

On Pop!_OS, disable the `system76-power` GNOME extension and its service to avoid power-profile conflicts.

Full per-distribution guides, including immutable Fedora variants, Bazzite, NixOS and PikaOS, live in the [documentation book](https://asus-linux.org/) (`docs/` in this repository).

### Building from source

A Rust toolchain from [rustup.rs](https://rustup.rs/) (stable) is required, plus the usual build tools.

#### Arch Linux

```sh
sudo pacman -S git cmake clang pkg-config libzip rust openssl
make
sudo make install
```

#### Fedora

```sh
sudo dnf install git make cmake clang-devel libxkbcommon-devel systemd-devel expat-devel pcre2-devel libzstd-devel gtk3-devel rust cargo
make
sudo make install
```

#### openSUSE

```sh
sudo zypper in -t pattern devel_basis
sudo zypper in rustup make cmake clang-devel libxkbcommon-devel systemd-devel expat-devel pcre2-devel libzstd-devel
make
sudo make install
```

#### Debian / Ubuntu / Pop!_OS (unsupported)

```sh
sudo apt install make cargo gcc pkg-config openssl libasound2-dev cmake build-essential python3 \
  libfreetype6-dev libexpat1-dev libxcb-composite0-dev libssl-dev libx11-dev libfontconfig1-dev \
  curl libclang-dev libudev-dev libinput-dev libxkbcommon-dev libgbm-dev
make
sudo make install
```

### Upgrading

```sh
sudo systemctl daemon-reload && sudo systemctl restart asusd
```

### Uninstalling

```sh
sudo systemctl disable --now asusd.service asus-shutdown.service
sudo make uninstall
sudo systemctl daemon-reload
```

Then remove any leftover configuration in `/etc/asusd/`. For package installations, use your distribution's package manager instead.

## Development

The repository is a Cargo workspace. Daemon-side architecture patterns are documented in [docs/dev/design-patterns.md](docs/dev/design-patterns.md), user and distribution documentation in `docs/` (an mdBook), and the CLI reference in [MANUAL.md](MANUAL.md).

### Running the UI without hardware

`rog-control-center` has a demo mode that runs the full interface with representative fake data — no `asusd`, no dbus, no ASUS hardware required. It is the fastest way to work on the UI:

```sh
cargo build -p rog-control-center
./target/debug/rog-control-center --demo

# Open directly on a specific page (0 = dashboard … 9 = about)
ROGCC_DEMO_PAGE=1 ./target/debug/rog-control-center --demo
```

All controls are interactive and update only the demo state.

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

## Laptop support requests

To request support for unlisted hardware, open an issue on the [issue tracker](https://github.com/OpenGamingCollective/asusctl/issues).

- **PPT sliders:** see [issue #124](https://github.com/OpenGamingCollective/asusctl/issues/124).
- **Keyboard backlight:** test layout changes locally in `/usr/share/asusd/aura_support.ron` (or `rog-aura/data/aura_support.ron` in this repository); once they work for your model, open a pull request with the updated mapping.

## License

This project is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](LICENSE).

---

ASUS and ROG are registered trademarks of ASUSTeK Computer Inc. in the United States and other jurisdictions. References to ASUS products, services, or trademarks within this repository do not constitute or imply endorsement, sponsorship, or recommendation by ASUSTeK Computer Inc. Trademarks are used solely for hardware identification purposes.

---

## AI Disclaimer

We do not accept code blindly written with just AI or "vibecoding". We encourage use of AI for finding bugs and as a tool used to assist development, but all of these must be verified by a human as AI makes mistakes and gives false bug reports as well. For further details, refer to [our contribution policy](./CONTRIBUTING.md)
