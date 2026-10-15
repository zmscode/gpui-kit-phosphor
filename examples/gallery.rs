//! A small gallery of Phosphor icons in every weight.
//!
//! ```sh
//! cargo run --example gallery --features all-weights
//! ```

use gpui_kit::assets::Assets;
use gpui_kit::component::{ActiveTheme as _, Icon, Sizable as _, StyledExt as _};
use gpui_kit::*;
use gpui_kit_phosphor::{PhosphorAssets, PhosphorIcon, Weight};

const ICONS: &[PhosphorIcon] = &[
    PhosphorIcon::House,
    PhosphorIcon::Gear,
    PhosphorIcon::MagnifyingGlass,
    PhosphorIcon::Heart,
    PhosphorIcon::Star,
    PhosphorIcon::Bell,
    PhosphorIcon::ChatCircle,
    PhosphorIcon::Folder,
    PhosphorIcon::Trash,
    PhosphorIcon::Lightning,
];

struct Gallery;

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .p_6()
            .gap_4()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .children(Weight::ALL.into_iter().map(|weight| {
                div()
                    .h_flex()
                    .gap_4()
                    .items_center()
                    .child(div().w_20().text_sm().child(weight.as_str()))
                    .children(
                        ICONS
                            .iter()
                            .map(move |icon| Icon::new(icon.weight(weight)).large()),
                    )
            }))
            .child(
                // Bare icons render at the current text size and color.
                div()
                    .h_flex()
                    .gap_2()
                    .items_center()
                    .text_color(cx.theme().primary)
                    .child(PhosphorIcon::Sparkle.duotone())
                    .child("Bare icons follow the text style"),
            )
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(PhosphorAssets.with(Assets))
        .run(|cx| {
            gpui_kit::init(cx);
            gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| Gallery))
                .expect("failed to open window");
        });
}
