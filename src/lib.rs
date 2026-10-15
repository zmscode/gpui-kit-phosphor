//! [Phosphor](https://phosphoricons.com) icons for [GPUI Kit](https://gpui-kit.com).
//!
//! All 1,512 Phosphor icons in all six weights, as a [`PhosphorIcon`] enum that
//! plugs into GPUI Kit's `Icon` through the [`IconNamed`] trait, plus asset
//! sources that serve the SVGs.
//!
//! ```no_run
//! use gpui_kit::*;
//! use gpui_kit::assets::Assets;
//! use gpui_kit::component::Icon;
//! use gpui_kit_phosphor::{PhosphorAssets, PhosphorIcon, Weight};
//!
//! struct Example;
//!
//! impl Render for Example {
//!     fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
//!         div()
//!             .flex()
//!             .gap_2()
//!             .child(Icon::new(PhosphorIcon::Gear))                 // regular
//!             .child(Icon::new(PhosphorIcon::Heart.fill()))         // fill
//!             .child(Icon::new(PhosphorIcon::House.weight(Weight::Bold)).size_6())
//!             .child(PhosphorIcon::Star.duotone())                  // no `Icon` needed
//!     }
//! }
//!
//! fn main() {
//!     // Phosphor first, then GPUI Kit's own component icons.
//!     gpui_kit::application()
//!         .with_assets(PhosphorAssets.with(Assets))
//!         .run(|cx| { /* ... */ });
//! }
//! ```
//!
//! # Choosing what gets embedded
//!
//! Depending on this crate does not put every SVG into your binary. Only the
//! icons an asset source refers to are embedded:
//!
//! - [`PhosphorAssets`] embeds every icon of each weight enabled as a Cargo
//!   feature. Only `regular` is on by default; enable `thin`, `light`, `bold`,
//!   `fill`, `duotone`, or `all-weights` as needed. Each weight adds roughly
//!   800 KiB of SVG data.
//! - [`phosphor_assets!`] generates a source holding only the icons you list,
//!   in any weight, regardless of features. Use it to ship a handful of icons.
//!
//! Requesting an icon whose weight is not embedded by the registered source
//! simply renders nothing, the same as any other missing asset in GPUI.
//!
//! Asset paths have the form `phosphor/{weight}/{name}.svg`, so they never
//! collide with GPUI Kit's `icons/…` paths.

#[cfg(feature = "gpui-fast")]
extern crate gpui_fast as gpui;

use std::borrow::Cow;

use gpui::{
    AnyElement, App, AssetSource, Element as _, IntoElement, RenderOnce, SharedString, Styled,
    ViewElement, Window, svg,
};
pub use gpui_kit_assets::IconNamed;

include!(concat!(env!("OUT_DIR"), "/phosphor.rs"));

// Compile-check the README's examples.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Prefix shared by every asset path this crate serves.
pub const PATH_PREFIX: &str = "phosphor/";

/// A Phosphor icon weight (style).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Weight {
    Thin,
    Light,
    #[default]
    Regular,
    Bold,
    Fill,
    Duotone,
}

impl Weight {
    /// Every weight, lightest first.
    pub const ALL: [Weight; 6] = [
        Weight::Thin,
        Weight::Light,
        Weight::Regular,
        Weight::Bold,
        Weight::Fill,
        Weight::Duotone,
    ];

    /// The lowercase weight name used in asset paths, e.g. `"bold"`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Weight::Thin => "thin",
            Weight::Light => "light",
            Weight::Regular => "regular",
            Weight::Bold => "bold",
            Weight::Fill => "fill",
            Weight::Duotone => "duotone",
        }
    }

    /// Parse a lowercase weight name.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|weight| weight.as_str() == name)
    }

    /// Whether [`PhosphorAssets`] embeds this weight in the current build.
    pub const fn is_embedded(self) -> bool {
        match self {
            Weight::Thin => cfg!(feature = "thin"),
            Weight::Light => cfg!(feature = "light"),
            Weight::Regular => cfg!(feature = "regular"),
            Weight::Bold => cfg!(feature = "bold"),
            Weight::Fill => cfg!(feature = "fill"),
            Weight::Duotone => cfg!(feature = "duotone"),
        }
    }
}

impl std::fmt::Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Weight {
    type Err = ParseError;

    /// Parse a lowercase weight name, e.g. `"bold"`.
    fn from_str(name: &str) -> Result<Self, ParseError> {
        Self::from_name(name).ok_or(ParseError(()))
    }
}

/// Returned when parsing an unknown icon or weight name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseError(());

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown Phosphor icon or weight name")
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for PhosphorIcon {
    /// The Phosphor name, e.g. `arrow-up-right`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for PhosphorIcon {
    type Err = ParseError;

    /// Parse a Phosphor name, e.g. `"arrow-up-right"`.
    fn from_str(name: &str) -> Result<Self, ParseError> {
        Self::from_name(name).ok_or(ParseError(()))
    }
}

impl std::fmt::Display for Phosphor {
    /// The name and weight, e.g. `gear (bold)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.icon, self.weight)
    }
}

impl PhosphorIcon {
    /// Look an icon up by its Phosphor name, e.g. `"address-book"`.
    pub fn from_name(name: &str) -> Option<Self> {
        BY_NAME
            .binary_search_by(|(candidate, _)| (*candidate).cmp(name))
            .ok()
            .map(|index| BY_NAME[index].1)
    }

    /// This icon in the given weight.
    pub const fn weight(self, weight: Weight) -> Phosphor {
        Phosphor { icon: self, weight }
    }

    /// This icon in the thin weight.
    pub const fn thin(self) -> Phosphor {
        self.weight(Weight::Thin)
    }

    /// This icon in the light weight.
    pub const fn light(self) -> Phosphor {
        self.weight(Weight::Light)
    }

    /// This icon in the regular weight (the same as the bare icon).
    pub const fn regular(self) -> Phosphor {
        self.weight(Weight::Regular)
    }

    /// This icon in the bold weight.
    pub const fn bold(self) -> Phosphor {
        self.weight(Weight::Bold)
    }

    /// This icon in the fill weight.
    pub const fn fill(self) -> Phosphor {
        self.weight(Weight::Fill)
    }

    /// This icon in the duotone weight.
    pub const fn duotone(self) -> Phosphor {
        self.weight(Weight::Duotone)
    }

    /// The asset path of the regular weight, e.g. `phosphor/regular/gear.svg`.
    pub fn path(self) -> SharedString {
        self.regular().path()
    }
}

/// A Phosphor icon in a specific [`Weight`].
///
/// Build one with [`PhosphorIcon::weight`] or a shorthand such as
/// [`PhosphorIcon::bold`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Phosphor {
    pub icon: PhosphorIcon,
    pub weight: Weight,
}

impl Phosphor {
    pub const fn new(icon: PhosphorIcon, weight: Weight) -> Self {
        Self { icon, weight }
    }

    /// The asset path, e.g. `phosphor/bold/gear.svg`.
    pub fn path(self) -> SharedString {
        format!(
            "{PATH_PREFIX}{}/{}.svg",
            self.weight.as_str(),
            self.icon.name()
        )
        .into()
    }

    /// Parse an asset path produced by [`Phosphor::path`].
    pub fn from_path(path: &str) -> Option<Self> {
        let rest = path.strip_prefix(PATH_PREFIX)?.strip_suffix(".svg")?;
        let (weight, name) = rest.split_once('/')?;
        Some(Self::new(
            PhosphorIcon::from_name(name)?,
            Weight::from_name(weight)?,
        ))
    }

    /// The SVG bytes, if this icon's weight is enabled as a Cargo feature.
    ///
    /// Useful with `Icon::default().data(..)` when no asset source is wanted.
    /// Calling this refers to every icon of the enabled weights.
    pub fn svg(self) -> Option<&'static [u8]> {
        embedded_entry(self).map(|(_, bytes)| bytes)
    }
}

impl From<PhosphorIcon> for Phosphor {
    fn from(icon: PhosphorIcon) -> Self {
        icon.regular()
    }
}

impl IconNamed for PhosphorIcon {
    fn path(self) -> SharedString {
        PhosphorIcon::path(self)
    }
}

impl IconNamed for Phosphor {
    fn path(self) -> SharedString {
        Phosphor::path(self)
    }
}

// Like `gpui_kit_assets::IconName`, both types can be used directly as
// children, sized and colored by the surrounding text style. Wrap them in
// `gpui_kit::component::Icon` for themed sizes and transformations.
fn render_icon(path: SharedString, window: &mut Window) -> impl IntoElement {
    let text_style = window.text_style();
    svg()
        .path(path)
        .flex_shrink_0()
        .size(text_style.font_size.to_pixels(window.rem_size()))
        .text_color(text_style.color)
}

// Written out instead of `#[derive(IntoElement)]` so the impls don't depend on
// how the derive spells GPUI's crate path; this mirrors what it generates.
impl IntoElement for PhosphorIcon {
    type Element = ViewElement<Self>;

    #[track_caller]
    fn into_element(self) -> Self::Element {
        ViewElement::new(self)
    }

    #[track_caller]
    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }
}

impl IntoElement for Phosphor {
    type Element = ViewElement<Self>;

    #[track_caller]
    fn into_element(self) -> Self::Element {
        ViewElement::new(self)
    }

    #[track_caller]
    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }
}

impl RenderOnce for PhosphorIcon {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        render_icon(self.path(), window)
    }
}

impl RenderOnce for Phosphor {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        render_icon(self.path(), window)
    }
}

impl From<PhosphorIcon> for AnyElement {
    fn from(icon: PhosphorIcon) -> Self {
        icon.into_any_element()
    }
}

impl From<Phosphor> for AnyElement {
    fn from(icon: Phosphor) -> Self {
        icon.into_any_element()
    }
}

#[allow(unused_variables, unreachable_patterns)]
fn embedded_entry(icon: Phosphor) -> Option<(&'static str, &'static [u8])> {
    match icon.weight {
        #[cfg(feature = "thin")]
        Weight::Thin => Some(thin(icon.icon)),
        #[cfg(feature = "light")]
        Weight::Light => Some(light(icon.icon)),
        #[cfg(feature = "regular")]
        Weight::Regular => Some(regular(icon.icon)),
        #[cfg(feature = "bold")]
        Weight::Bold => Some(bold(icon.icon)),
        #[cfg(feature = "fill")]
        Weight::Fill => Some(fill(icon.icon)),
        #[cfg(feature = "duotone")]
        Weight::Duotone => Some(duotone(icon.icon)),
        _ => None,
    }
}

/// Serves every icon of the weights enabled as Cargo features.
///
/// Combine it with GPUI Kit's own `Assets` (or any other source) using
/// [`PhosphorAssets::with`]:
///
/// ```no_run
/// # use gpui_kit_phosphor::PhosphorAssets;
/// gpui_kit::application().with_assets(PhosphorAssets.with(gpui_kit::assets::Assets));
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct PhosphorAssets;

impl PhosphorAssets {
    /// Serve Phosphor icons first and fall back to `fallback` for other paths.
    pub fn with<B: AssetSource>(self, fallback: B) -> Layered<Self, B> {
        Layered(self, fallback)
    }
}

impl AssetSource for PhosphorAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(Phosphor::from_path(path)
            .and_then(Phosphor::svg)
            .map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        // Cheap rejection for the common case of listing another source's paths.
        if !(path.starts_with(PATH_PREFIX) || PATH_PREFIX.starts_with(path)) {
            return Ok(Vec::new());
        }
        Ok(Weight::ALL
            .into_iter()
            .filter(|weight| weight.is_embedded())
            .flat_map(|weight| {
                PhosphorIcon::ALL
                    .iter()
                    .map(move |icon| icon.weight(weight))
            })
            .map(Phosphor::path)
            .filter(|candidate| candidate.starts_with(path))
            .collect())
    }
}

/// Two asset sources layered together: `load` tries the first source, then the
/// second; `list` merges both.
#[derive(Clone, Copy, Debug, Default)]
pub struct Layered<A, B>(pub A, pub B);

impl<A, B> Layered<A, B> {
    /// Add another fallback below the existing ones.
    pub fn with<C: AssetSource>(self, fallback: C) -> Layered<Self, C> {
        Layered(self, fallback)
    }
}

impl<A: AssetSource, B: AssetSource> AssetSource for Layered<A, B> {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        match self.0.load(path)? {
            Some(bytes) => Ok(Some(bytes)),
            None => self.1.load(path),
        }
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        let mut paths = self.0.list(path)?;
        paths.extend(self.1.list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

// Public only so `phosphor_assets!` resolves in downstream crates.
#[doc(hidden)]
pub mod __private {
    pub use crate::Layered;
    pub use crate::embedded::*;
    pub use gpui::{AssetSource, Result, SharedString};
}

/// Generate an asset source holding only the listed icons.
///
/// Icons are grouped by weight; any weight works, whatever Cargo features
/// are enabled. Only the listed SVGs end up in the binary.
///
/// ```no_run
/// use gpui_kit_phosphor::{phosphor_assets, PhosphorIcon};
///
/// phosphor_assets!(pub AppIcons, {
///     regular: [Gear, House, MagnifyingGlass],
///     fill: [Heart, Star],
/// });
///
/// // Phosphor icons, then GPUI Kit's component icons.
/// # fn main() {
/// gpui_kit::application().with_assets(AppIcons.with(gpui_kit::assets::Assets));
/// # }
/// ```
///
/// Rendering an icon that was not listed draws nothing. A misspelled icon or
/// weight is a compile error.
#[macro_export]
macro_rules! phosphor_assets {
    ($vis:vis $name:ident, { $($weight:ident : [$($icon:ident),* $(,)?]),* $(,)? }) => {
        #[derive(Clone, Copy, Debug, Default)]
        $vis struct $name;

        impl $name {
            const ENTRIES: &'static [(&'static str, &'static [u8])] = &[
                $($($crate::__private::$weight::$icon,)*)*
            ];

            /// Serve these icons first and fall back to `fallback` for other paths.
            #[allow(dead_code)]
            $vis fn with<B: $crate::__private::AssetSource>(self, fallback: B) -> $crate::__private::Layered<Self, B> {
                $crate::__private::Layered(self, fallback)
            }
        }

        impl $crate::__private::AssetSource for $name {
            fn load(&self, path: &str) -> $crate::__private::Result<::std::option::Option<::std::borrow::Cow<'static, [u8]>>> {
                ::std::result::Result::Ok(
                    Self::ENTRIES
                        .iter()
                        .find(|(candidate, _)| *candidate == path)
                        .map(|(_, bytes)| ::std::borrow::Cow::Borrowed(*bytes)),
                )
            }

            fn list(&self, path: &str) -> $crate::__private::Result<::std::vec::Vec<$crate::__private::SharedString>> {
                ::std::result::Result::Ok(
                    Self::ENTRIES
                        .iter()
                        .map(|(candidate, _)| *candidate)
                        .filter(|candidate| candidate.starts_with(path))
                        .map(::std::convert::Into::into)
                        .collect(),
                )
            }
        }
    };
}
