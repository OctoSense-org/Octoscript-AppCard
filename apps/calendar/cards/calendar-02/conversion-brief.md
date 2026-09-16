# Conversion brief

This is an implementation brief, not a claim that these instructions were used
to generate the existing reference. Keep the submitted image prompt unchanged.

Apply MAPPING-RULES.md and mapping-rules.json. Resolve every needs_review/unknown
region. Prefer a matching native kit component, then built-in Makepad widgets,
then a reusable custom widget for missing behavior. Use SVG or cropped Image
assets only for artwork. Never substitute a chart or control with an asset.

For new image generation, include the exact text, font files/family/weights,
layout hierarchy, dimensions, spacing, colors, chart samples/units/domains and
selected control states. Preserve a separate machine-readable manifest. Request
complex illustrations as separate assets, or clearly bounded artwork-only regions
with no overlaid UI text. Do not invent missing numerical values from a mockup.

After generation, measure the actual reference. Requested layout is not measured
evidence. Inspect through Makepad's built-in HTTP instrument with a standalone
release binary; hidden windows support automated tests. See
`lab/core/NATIVE-INSTRUMENT.md`. Run semantic, geometry and visual checks;
legacy Studio capture/gate adapters require their own evidence schema.

```json
[
  {
    "id": "page",
    "x": 0,
    "y": 0,
    "w": 406,
    "h": 776,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "screen",
    "x": 0.0,
    "y": 0.0,
    "w": 203.0,
    "h": 388.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "status_icons",
    "x": 163.0,
    "y": 6.5,
    "w": 28.0,
    "h": 7.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/status_icons.svg",
      "sha256": "416cbc07c261422b8a0e80634980633e68841fc39b3db3879a8337f85587a1e0",
      "method": "reference_svg",
      "reference_sha256": "da183738dfa292b7c533a45de09a86807fbe1a3cb5c3a4cf8db4be13704ee406",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "back_month",
    "x": 6.0,
    "y": 25.0,
    "w": 33.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "back_month_surface",
    "x": 6.0,
    "y": 25.0,
    "w": 33.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "back_month_control",
    "x": 6.0,
    "y": 25.0,
    "w": 33.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "back_month_icon",
    "x": 7.0,
    "y": 28.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/back_month_icon.svg",
      "sha256": "1027440064a9b8cfc64637059559b54eb6408674274992d1ce108ed1365b6875",
      "method": "reference_svg",
      "reference_sha256": "da183738dfa292b7c533a45de09a86807fbe1a3cb5c3a4cf8db4be13704ee406",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "back_month_label",
    "x": 18.0,
    "y": 29.0165,
    "w": 17.2175,
    "h": 11.378,
    "text": "9\u6708",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 8.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "add_event",
    "x": 173.0,
    "y": 25.0,
    "w": 22.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "add_event_surface",
    "x": 173.0,
    "y": 25.0,
    "w": 22.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "add_event_control",
    "x": 173.0,
    "y": 25.0,
    "w": 22.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "add_event_icon",
    "x": 178.5,
    "y": 28.5,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/add_event_icon.svg",
      "sha256": "2e94041b38e4ca076ffbcc668f0674f094a6b8b9dd4e90a3a6fd119d7f81cea7",
      "method": "reference_svg",
      "reference_sha256": "da183738dfa292b7c533a45de09a86807fbe1a3cb5c3a4cf8db4be13704ee406",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "open_dentist",
    "x": 44.0,
    "y": 83.0,
    "w": 149.0,
    "h": 48.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_dentist_surface",
    "x": 44.0,
    "y": 83.0,
    "w": 149.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dentist_control",
    "x": 44.0,
    "y": 83.0,
    "w": 149.0,
    "h": 48.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_dentist_card",
    "x": 44.0,
    "y": 83.0,
    "w": 149.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dentist_bar",
    "x": 44.0,
    "y": 83.0,
    "w": 2.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dentist_label",
    "x": 53.0,
    "y": 89.8295,
    "w": 21.0,
    "h": 11.642916666666666,
    "text": "\u7259\u533b",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 8.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dentist_loc",
    "x": 53.0,
    "y": 102.8675,
    "w": 30.0,
    "h": 9.988125,
    "text": "\u65e5\u51fa\u7259\u79d1",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dentist_range",
    "x": 53.0,
    "y": 115.485,
    "w": 42.870000000000005,
    "h": 8.9335,
    "text": "10:30 \u2013 11:15",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_team_sync",
    "x": 44.0,
    "y": 143.0,
    "w": 149.0,
    "h": 48.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_team_sync_surface",
    "x": 44.0,
    "y": 143.0,
    "w": 149.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_team_sync_control",
    "x": 44.0,
    "y": 143.0,
    "w": 149.0,
    "h": 48.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_team_sync_card",
    "x": 44.0,
    "y": 143.0,
    "w": 149.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_team_sync_bar",
    "x": 44.0,
    "y": 143.0,
    "w": 2.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_team_sync_label",
    "x": 53.0,
    "y": 149.48100000000002,
    "w": 38.0,
    "h": 11.9985,
    "text": "\u56e2\u961f\u540c\u6b65",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 8.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_team_sync_loc",
    "x": 53.0,
    "y": 162.8675,
    "w": 30.0,
    "h": 10.032,
    "text": "\u89c6\u9891\u4f1a\u8bae",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_team_sync_range",
    "x": 53.0,
    "y": 175.485,
    "w": 42.870000000000005,
    "h": 8.9335,
    "text": "14:00 \u2013 14:45",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dinner",
    "x": 44.0,
    "y": 203.0,
    "w": 149.0,
    "h": 48.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_dinner_surface",
    "x": 44.0,
    "y": 203.0,
    "w": 149.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dinner_control",
    "x": 44.0,
    "y": 203.0,
    "w": 149.0,
    "h": 48.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_dinner_card",
    "x": 44.0,
    "y": 203.0,
    "w": 149.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dinner_bar",
    "x": 44.0,
    "y": 203.0,
    "w": 2.0,
    "h": 48.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dinner_label",
    "x": 53.0,
    "y": 209.48100000000002,
    "w": 51.8805,
    "h": 12.0155,
    "text": "\u4e0e Sam \u665a\u9910",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 8.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dinner_loc",
    "x": 53.0,
    "y": 222.861,
    "w": 81.8765,
    "h": 9.9995,
    "text": "\u83b2\u82b1\u53a8\u623f \u00b7 \u6765\u81ea Sam \u684c\u9762",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dinner_range",
    "x": 53.0,
    "y": 235.485,
    "w": 42.870000000000005,
    "h": 8.9335,
    "text": "19:00 \u2013 20:30",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "now_line",
    "x": 44.0,
    "y": 78.5,
    "w": 149.0,
    "h": 1.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "tab_bar",
    "x": 0.0,
    "y": 347.0,
    "w": 203.0,
    "h": 41.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "tab_today",
    "x": 10.0,
    "y": 350.0,
    "w": 61.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "tab_today_icon",
    "x": 34.5,
    "y": 352.0,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/tab_today_icon.svg",
      "sha256": "fae6f2960edcdfc4069a9d562bcec85c0105f25bd880a22ed38f7c911404f57a",
      "method": "reference_svg",
      "reference_sha256": "da183738dfa292b7c533a45de09a86807fbe1a3cb5c3a4cf8db4be13704ee406",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "tab_today_control",
    "x": 10.0,
    "y": 350.0,
    "w": 61.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "tab_today_label",
    "x": 35.0,
    "y": 365.73949999999996,
    "w": 15.0,
    "h": 9.114999999999998,
    "text": "\u4eca\u5929",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 5.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "tab_calendars",
    "x": 71.0,
    "y": 350.0,
    "w": 61.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "tab_calendars_icon",
    "x": 95.5,
    "y": 352.0,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/tab_calendars_icon.svg",
      "sha256": "9ac047537118e5d992c968c85fb465685ef0e60c7661fa49555a95341d0ab2a2",
      "method": "reference_svg",
      "reference_sha256": "da183738dfa292b7c533a45de09a86807fbe1a3cb5c3a4cf8db4be13704ee406",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "tab_calendars_control",
    "x": 71.0,
    "y": 350.0,
    "w": 61.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "tab_calendars_label",
    "x": 96.0,
    "y": 366.04749999999996,
    "w": 15.0,
    "h": 8.774000000000001,
    "text": "\u65e5\u5386",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 5.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "tab_inbox",
    "x": 132.0,
    "y": 350.0,
    "w": 61.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "tab_inbox_icon",
    "x": 156.5,
    "y": 352.0,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/tab_inbox_icon.svg",
      "sha256": "6ac382742f14fe1a62c939d03e6c49e8cbdf27512b0ff336dd758275a70d74e8",
      "method": "reference_svg",
      "reference_sha256": "da183738dfa292b7c533a45de09a86807fbe1a3cb5c3a4cf8db4be13704ee406",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "tab_inbox_control",
    "x": 132.0,
    "y": 350.0,
    "w": 61.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "inbox_badge",
    "x": 146.0,
    "y": 350.0,
    "w": 9.0,
    "h": 9.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "tab_inbox_label",
    "x": 154.25,
    "y": 365.756,
    "w": 20.5,
    "h": 9.087499999999999,
    "text": "\u6536\u4ef6\u7bb1",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 5.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "inbox_badge_count",
    "x": 148.8775,
    "y": 350.8225,
    "w": 7.244999999999999,
    "h": 8.0755,
    "text": "1",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 5.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "tab_line",
    "x": 0.0,
    "y": 347.0,
    "w": 203.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "home_indicator",
    "x": 69.5,
    "y": 381.0,
    "w": 64.0,
    "h": 2.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "status_time",
    "x": 14.0,
    "y": 6.614999999999998,
    "w": 24.1375,
    "h": 9.76,
    "text": "09:41",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 7.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_title",
    "x": 10.0,
    "y": 49.608,
    "w": 75.964,
    "h": 14.752,
    "text": "9\u670824\u65e5 \u5468\u56db",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 12.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_sub",
    "x": 10.0,
    "y": 64.8285,
    "w": 50.975500000000004,
    "h": 10.045,
    "text": "\u4eca\u5929 \u00b7 3 \u9879\u65e5\u7a0b",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dentist_start",
    "x": 10.0,
    "y": 86.774,
    "w": 22.795,
    "h": 9.376000000000001,
    "text": "10:30",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 7.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dentist_end",
    "x": 10.0,
    "y": 97.718,
    "w": 18.988,
    "h": 8.475999999999999,
    "text": "11:15",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_team_sync_start",
    "x": 10.0,
    "y": 146.774,
    "w": 22.795,
    "h": 9.376000000000001,
    "text": "14:00",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 7.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_team_sync_end",
    "x": 10.0,
    "y": 157.71800000000002,
    "w": 18.988,
    "h": 8.475999999999999,
    "text": "14:45",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dinner_start",
    "x": 10.0,
    "y": 206.774,
    "w": 22.795,
    "h": 9.376000000000001,
    "text": "19:00",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 7.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dinner_end",
    "x": 10.0,
    "y": 217.64000000000001,
    "w": 18.988,
    "h": 8.554,
    "text": "20:30",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "now_line_label",
    "x": 10.0,
    "y": 75.25099999999999,
    "w": 18.7675,
    "h": 8.224,
    "text": "09:41",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 5.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  }
]
```
