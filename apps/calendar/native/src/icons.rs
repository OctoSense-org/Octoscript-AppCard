//! The storyboard's vector icons (scripts/design.py SVG), served to the
//! renderer from the module's loopback asset server.
pub const ICONS: &[(&str, &str)] = &[
    ("back", r#"<path d="m17 4-10 10 10 10"/>"#),
    ("forward", r#"<path d="m11 4 10 10-10 10"/>"#),
    ("chevron", r#"<path d="m11 5 8 9-8 9"/>"#),
    ("plus", r#"<path d="M14 5v18M5 14h18"/>"#),
    ("calendar", r#"<rect x="4" y="5" width="20" height="20" rx="3"/><path d="M4 11h20M9 3v4m10-4v4"/>"#),
    ("list", r#"<path d="M5 8h18M5 14h18M5 20h18"/>"#),
    ("tray", r#"<path d="M4 15v7h20v-7M4 15l3-9h14l3 9M4 15h6l2 3h4l2-3h6"/>"#),
    ("pin", r#"<path d="M14 26S5 17 5 11a9 9 0 0 1 18 0c0 6-9 15-9 15Z"/><circle cx="14" cy="11" r="3"/>"#),
    ("clock", r#"<circle cx="14" cy="14" r="10"/><path d="M14 8v6l4 3"/>"#),
    ("bell", r#"<path d="M7 20V12a7 7 0 0 1 14 0v8l2 2H5zM11 24a3 3 0 0 0 6 0"/>"#),
    ("check", r#"<path d="m6 14 6 6L23 8"/>"#),
    ("sync", r#"<path d="M23 12a9 9 0 0 0-16-4M5 16a9 9 0 0 0 16 4M7 3v5h5M21 25v-5h-5"/>"#),
    ("people", r#"<circle cx="10" cy="10" r="4"/><circle cx="19" cy="11" r="3"/><path d="M3 23a7 7 0 0 1 14 0M17 23a5 5 0 0 1 8 0"/>"#),
    ("note", r#"<path d="M6 4h12l5 5v15H6zM17 4v6h6M10 15h8m-8 4h8"/>"#),
    ("trash", r#"<path d="M5 8h18M11 8V5h6v3M8 8l1 16h10l1-16M12 12v8m4-8v8"/>"#),
    ("close", r#"<path d="m7 7 14 14M21 7 7 21"/>"#),
];

/// One stroked icon file in the given hex colour.
pub fn svg(name: &str, color: &str) -> Option<String> {
    let body = ICONS.iter().find(|(n, _)| *n == name)?.1;
    Some(format!(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" fill="none" stroke="#{color}" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">{body}</svg>"##))
}
