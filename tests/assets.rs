#[cfg(not(feature = "gpui-fast"))]
use gpui::AssetSource;
#[cfg(feature = "gpui-fast")]
use gpui_fast::AssetSource;
use gpui_kit_assets::IconNamed;
use gpui_kit_phosphor::{Phosphor, PhosphorAssets, PhosphorIcon, Weight, phosphor_assets};

phosphor_assets!(Selected, {
    regular: [Gear, House],
    fill: [Heart],
});

#[test]
fn catalog_is_complete_and_named() {
    assert_eq!(PhosphorIcon::ALL.len(), 1512);
    assert_eq!(PhosphorIcon::AddressBook.name(), "address-book");
    for &icon in PhosphorIcon::ALL {
        assert_eq!(PhosphorIcon::from_name(icon.name()), Some(icon));
    }
    assert_eq!(PhosphorIcon::from_name("not-an-icon"), None);
}

#[test]
fn paths_round_trip() {
    assert_eq!(
        IconNamed::path(PhosphorIcon::Gear).to_string(),
        "phosphor/regular/gear.svg"
    );
    assert_eq!(
        IconNamed::path(PhosphorIcon::ArrowUpRight.bold()).to_string(),
        "phosphor/bold/arrow-up-right.svg"
    );
    for weight in Weight::ALL {
        let icon = PhosphorIcon::MagnifyingGlass.weight(weight);
        assert_eq!(Phosphor::from_path(&icon.path()), Some(icon));
    }
    assert_eq!(Phosphor::from_path("icons/gear.svg"), None);
    assert_eq!(Phosphor::from_path("phosphor/heavy/gear.svg"), None);
}

#[test]
#[cfg(feature = "regular")]
fn default_source_serves_regular_only() {
    let svg = PhosphorAssets
        .load("phosphor/regular/gear.svg")
        .unwrap()
        .expect("regular weight is a default feature");
    assert!(svg.starts_with(b"<svg"));

    let bold = PhosphorAssets.load("phosphor/bold/gear.svg").unwrap();
    assert_eq!(bold.is_some(), cfg!(feature = "bold"));
    assert!(PhosphorAssets.load("icons/gear.svg").unwrap().is_none());

    let listed = PhosphorAssets.list("phosphor/regular/").unwrap();
    assert_eq!(listed.len(), 1512);
    assert!(PhosphorAssets.list("icons/").unwrap().is_empty());
}

#[test]
fn selected_source_serves_only_listed_icons() {
    assert!(
        Selected
            .load("phosphor/regular/gear.svg")
            .unwrap()
            .is_some()
    );
    assert!(Selected.load("phosphor/fill/heart.svg").unwrap().is_some());
    assert!(
        Selected
            .load("phosphor/regular/heart.svg")
            .unwrap()
            .is_none()
    );
    assert_eq!(Selected.list("phosphor/").unwrap().len(), 3);
}

#[test]
fn source_serves_only_enabled_weights() {
    for weight in Weight::ALL {
        let path = PhosphorIcon::Gear.weight(weight).path();
        let loaded = PhosphorAssets.load(&path).unwrap();
        assert_eq!(loaded.is_some(), weight.is_embedded(), "{path}");
        assert_eq!(
            PhosphorIcon::Gear.weight(weight).svg().is_some(),
            weight.is_embedded()
        );
        let listed = PhosphorAssets.list(&format!("phosphor/{weight}/")).unwrap();
        assert_eq!(listed.len(), if weight.is_embedded() { 1512 } else { 0 });
    }
}

#[test]
fn names_parse_and_display() {
    assert_eq!("arrow-up-right".parse(), Ok(PhosphorIcon::ArrowUpRight));
    assert!("ArrowUpRight".parse::<PhosphorIcon>().is_err());
    assert_eq!(PhosphorIcon::ArrowUpRight.to_string(), "arrow-up-right");
    assert_eq!("duotone".parse(), Ok(Weight::Duotone));
    assert!("heavy".parse::<Weight>().is_err());
    assert_eq!(PhosphorIcon::Gear.bold().to_string(), "gear (bold)");
}

#[test]
#[cfg(feature = "regular")]
fn layered_falls_back() {
    let layered = Selected.with(PhosphorAssets);
    assert!(layered.load("phosphor/fill/heart.svg").unwrap().is_some());
    assert!(
        layered
            .load("phosphor/regular/acorn.svg")
            .unwrap()
            .is_some()
    );
    let listed = layered.list("phosphor/regular/").unwrap();
    assert_eq!(listed.len(), 1512, "duplicates are merged");
}
