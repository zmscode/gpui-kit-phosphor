# gpui-kit-phosphor

[![CI](https://github.com/zmscode/gpui-kit-phosphor/actions/workflows/ci.yml/badge.svg)](https://github.com/zmscode/gpui-kit-phosphor/actions/workflows/ci.yml) [![Docs](https://docs.rs/gpui-kit-phosphor/badge.svg)](https://docs.rs/gpui-kit-phosphor/) [![Crates.io](https://img.shields.io/crates/v/gpui-kit-phosphor.svg)](https://crates.io/crates/gpui-kit-phosphor) [![License](https://img.shields.io/crates/l/gpui-kit-phosphor.svg)](#license)

[Phosphor](https://phosphoricons.com) icons for [GPUI Kit](https://gpui-kit.com):
all 1,512 icons in all six weights (thin, light, regular, bold, fill, duotone).

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zmscode/gpui-kit-phosphor/main/docs/images/weights-dark.png">
  <img alt="Ten Phosphor icons in each of the six weights" src="https://raw.githubusercontent.com/zmscode/gpui-kit-phosphor/main/docs/images/weights-light.png" width="620">
</picture>

`PhosphorIcon` implements GPUI Kit's `IconNamed` trait, so it works anywhere
GPUI Kit takes an icon: `Icon::new(...)`, buttons, inputs, tags, menus, and so on.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zmscode/gpui-kit-phosphor/main/docs/images/components-dark.png">
  <img alt="GPUI Kit buttons, a search input and tags using Phosphor icons" src="https://raw.githubusercontent.com/zmscode/gpui-kit-phosphor/main/docs/images/components-light.png" width="580">
</picture>

Icons take the surrounding text color, any size, and duotone keeps its
two-tone look in every color:

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zmscode/gpui-kit-phosphor/main/docs/images/styling-dark.png">
  <img alt="A duotone planet at six sizes, and fill hearts and duotone lightning bolts in six colors" src="https://raw.githubusercontent.com/zmscode/gpui-kit-phosphor/main/docs/images/styling-light.png" width="580">
</picture>

## Usage

```toml
[dependencies]
gpui-kit = "0.7.3"
gpui-kit-phosphor = { version = "0.1", features = ["fill"] } # `regular` is on by default
```

Register the Phosphor asset source along with GPUI Kit's own icons:

```rust,no_run
use gpui_kit::*;
use gpui_kit::assets::Assets;
use gpui_kit::component::{Icon, Sizable as _};
use gpui_kit_phosphor::{PhosphorAssets, PhosphorIcon, Weight};

struct Example;

impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .child(Icon::new(PhosphorIcon::Gear))                       // regular weight
            .child(Icon::new(PhosphorIcon::Heart.fill()).large())       // shorthand per weight
            .child(Icon::new(PhosphorIcon::House.weight(Weight::Fill))) // or pick at runtime
            .child(PhosphorIcon::MagnifyingGlass)                       // bare: follows the text style
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(PhosphorAssets.with(Assets)) // Phosphor first, then GPUI Kit's icons
        .run(|cx| {
            gpui_kit::init(cx);
            gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| Example))
                .expect("failed to open window");
        });
}
```

Phosphor names map to PascalCase variants: `arrow-up-right` → `PhosphorIcon::ArrowUpRight`.
Look up names at runtime with `PhosphorIcon::from_name("arrow-up-right")` or
`"arrow-up-right".parse::<PhosphorIcon>()` (`Weight` parses the same way), and
iterate the catalog with `PhosphorIcon::ALL`.

## What gets embedded

Depending on the crate does not put every SVG into your binary. Only icons an
asset source refers to are embedded:

| Source             | Embeds                                                                                 |
| ------------------ | -------------------------------------------------------------------------------------- |
| `PhosphorAssets`   | All 1,512 icons of each weight enabled as a Cargo feature (~800 KiB of SVG per weight) |
| `phosphor_assets!` | Only the icons you list, in any weight, regardless of features                         |

Weight features: `thin`, `light`, `regular` (default), `bold`, `fill`,
`duotone`, and `all-weights`. Asking for a weight the registered source does not
embed draws nothing, the same as any other missing GPUI asset; check
`Weight::is_embedded()` if you choose weights dynamically.

To ship just a handful of icons:

```rust,no_run
use gpui_kit_phosphor::phosphor_assets;

phosphor_assets!(pub AppIcons, {
    regular: [Gear, House, MagnifyingGlass],
    fill: [Heart, Star],
});

// gpui_kit::application().with_assets(AppIcons.with(gpui_kit::assets::Assets))
```

Sources chain with `.with(...)`, e.g. `AppIcons.with(PhosphorAssets).with(Assets)`.
`Layered(a, b)` combines any two `AssetSource`s the same way.

Asset paths look like `phosphor/bold/gear.svg`, so they never collide with GPUI
Kit's `icons/...` paths. If you don't want an asset source at all,
`PhosphorIcon::Gear.bold().svg()` returns the raw bytes for
`Icon::default().data(...)` (for weights enabled as features).

## Compatibility

| gpui-kit-phosphor | gpui-kit | GPUI snapshot     | Phosphor                      |
| ----------------- | -------- | ----------------- | ----------------------------- |
| 0.1               | 0.7.3    | `gpui-pre =0.3.9` | core `main` @ 2b75f3a (2.1.1) |

The crate depends on the exact GPUI snapshot `gpui-kit-assets` uses so their
types line up. If you enable `gpui-kit`'s `gpui-fast` feature, enable this
crate's `gpui-fast` feature too.

## Gallery

```sh
cargo run --example gallery --features all-weights
```

The images in this README are rendered by GPUI itself, offscreen, by
[`examples/readme_images.rs`](examples/readme_images.rs). Regenerate them after
changing the icons or the showcase:

```sh
cargo run --example readme_images --features all-weights
```

## Tests

```sh
cargo test --features all-weights
```

On macOS this includes `tests/rendering.rs`, which draws icons with GPUI's
headless Metal renderer (no window is shown) and checks the pixels. Set
`PHOSPHOR_SNAPSHOT=grid.png` to also save a grid of every weight.

## Updating the icons

```sh
script/sync-phosphor.sh          # latest phosphor-icons/core main
script/sync-phosphor.sh v2.0.8   # or a specific tag
```

`build.rs` regenerates `PhosphorIcon` from `assets/` at build time; no network
access is needed to build.

## License

Crate code: Apache-2.0 (matching GPUI Kit). Phosphor icons: MIT, see
[LICENSE-PHOSPHOR](LICENSE-PHOSPHOR).
