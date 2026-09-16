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
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "back_year",
    "x": 6.0,
    "y": 25.0,
    "w": 40.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "back_year_surface",
    "x": 6.0,
    "y": 25.0,
    "w": 40.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "back_year_control",
    "x": 6.0,
    "y": 25.0,
    "w": 40.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "back_year_icon",
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
      "path": "assets/back_year_icon.svg",
      "sha256": "1027440064a9b8cfc64637059559b54eb6408674274992d1ce108ed1365b6875",
      "method": "reference_svg",
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "back_year_label",
    "x": 18.0,
    "y": 28.532000000000004,
    "w": 31.37,
    "h": 11.854,
    "text": "2026\u5e74",
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
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "grid_line",
    "x": 10.0,
    "y": 83.0,
    "w": 183.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "today_ring",
    "x": 118.0,
    "y": 168.0,
    "w": 18.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_today",
    "x": 114.0,
    "y": 167.0,
    "w": 26.0,
    "h": 23.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_today_surface",
    "x": 114.0,
    "y": 167.0,
    "w": 26.0,
    "h": 23.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_today_control",
    "x": 114.0,
    "y": 167.0,
    "w": 26.0,
    "h": 23.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_today_label",
    "x": 121.69,
    "y": 173.638,
    "w": 14.62,
    "h": 10.786,
    "text": "24",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 9.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "dot_24_0",
    "x": 120.5,
    "y": 187.0,
    "w": 3.0,
    "h": 3.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "dot_24_1",
    "x": 124.5,
    "y": 187.0,
    "w": 3.0,
    "h": 3.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "dot_24_2",
    "x": 128.5,
    "y": 187.0,
    "w": 3.0,
    "h": 3.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "dot_25_0",
    "x": 150.5,
    "y": 187.0,
    "w": 3.0,
    "h": 3.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "dot_26_0",
    "x": 176.5,
    "y": 187.0,
    "w": 3.0,
    "h": 3.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "list_line",
    "x": 10.0,
    "y": 223.0,
    "w": 183.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dentist",
    "x": 10.0,
    "y": 243.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_dentist_surface",
    "x": 10.0,
    "y": 243.0,
    "w": 183.0,
    "h": 25.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dentist_control",
    "x": 10.0,
    "y": 243.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_dentist_bar",
    "x": 10.0,
    "y": 247.0,
    "w": 2.0,
    "h": 17.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dentist_chevron",
    "x": 183.0,
    "y": 251.0,
    "w": 8.0,
    "h": 8.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/open_dentist_chevron.svg",
      "sha256": "173ecccd4e15dcdca80c6086908e78139e01ce9ec919f593d7b24146601fee9e",
      "method": "reference_svg",
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "open_dentist_time",
    "x": 18.0,
    "y": 245.61499999999998,
    "w": 24.1375,
    "h": 9.76,
    "text": "10:30",
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
    "id": "open_dentist_label",
    "x": 50.0,
    "y": 245.36749999999998,
    "w": 19.0,
    "h": 10.504375,
    "text": "\u7259\u533b",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 7.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dentist_sub",
    "x": 50.0,
    "y": 256.07,
    "w": 16.0,
    "h": 9.52,
    "text": "\u4e2a\u4eba",
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
    "id": "open_team_sync",
    "x": 10.0,
    "y": 271.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_team_sync_surface",
    "x": 10.0,
    "y": 271.0,
    "w": 183.0,
    "h": 25.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_team_sync_control",
    "x": 10.0,
    "y": 271.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_team_sync_bar",
    "x": 10.0,
    "y": 275.0,
    "w": 2.0,
    "h": 17.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_team_sync_chevron",
    "x": 183.0,
    "y": 279.0,
    "w": 8.0,
    "h": 8.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/open_team_sync_chevron.svg",
      "sha256": "173ecccd4e15dcdca80c6086908e78139e01ce9ec919f593d7b24146601fee9e",
      "method": "reference_svg",
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "open_team_sync_time",
    "x": 18.0,
    "y": 273.615,
    "w": 24.1375,
    "h": 9.76,
    "text": "14:00",
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
    "id": "open_team_sync_label",
    "x": 50.0,
    "y": 272.96999999999997,
    "w": 34.0,
    "h": 10.915,
    "text": "\u56e2\u961f\u540c\u6b65",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 7.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_team_sync_sub",
    "x": 50.0,
    "y": 284.09999999999997,
    "w": 16.0,
    "h": 9.489999999999998,
    "text": "\u5de5\u4f5c",
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
    "id": "open_dinner",
    "x": 10.0,
    "y": 299.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_dinner_surface",
    "x": 10.0,
    "y": 299.0,
    "w": 183.0,
    "h": 25.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dinner_control",
    "x": 10.0,
    "y": 299.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_dinner_bar",
    "x": 10.0,
    "y": 303.0,
    "w": 2.0,
    "h": 17.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_dinner_chevron",
    "x": 183.0,
    "y": 307.0,
    "w": 8.0,
    "h": 8.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/open_dinner_chevron.svg",
      "sha256": "173ecccd4e15dcdca80c6086908e78139e01ce9ec919f593d7b24146601fee9e",
      "method": "reference_svg",
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "open_dinner_time",
    "x": 18.0,
    "y": 301.615,
    "w": 24.1375,
    "h": 9.76,
    "text": "19:00",
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
    "id": "open_dinner_label",
    "x": 50.0,
    "y": 300.96999999999997,
    "w": 45.4975,
    "h": 10.9225,
    "text": "\u4e0e Sam \u665a\u9910",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 7.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "open_dinner_sub",
    "x": 50.0,
    "y": 312.022,
    "w": 50.542,
    "h": 9.585999999999999,
    "text": "\u5bb6\u5ead \u00b7 \u6765\u81ea Sam",
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
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
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
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
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
      "reference_sha256": "3ec37fbd4dff21c19ece077b1d5e37cde3962afaf554cf353a0e235687fe0980",
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
    "id": "month_title",
    "x": 10.0,
    "y": 48.778000000000006,
    "w": 31.03,
    "h": 19.164,
    "text": "9\u6708",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 17.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "weekday_0",
    "x": 20.0,
    "y": 72.984,
    "w": 10.0,
    "h": 9.046,
    "text": "\u65e5",
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
    "id": "weekday_1",
    "x": 46.0,
    "y": 75.03,
    "w": 10.0,
    "h": 4.492,
    "text": "\u4e00",
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
    "id": "weekday_2",
    "x": 72.0,
    "y": 73.434,
    "w": 10.0,
    "h": 8.062,
    "text": "\u4e8c",
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
    "id": "weekday_3",
    "x": 98.0,
    "y": 73.158,
    "w": 10.0,
    "h": 8.5,
    "text": "\u4e09",
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
    "id": "weekday_4",
    "x": 124.0,
    "y": 73.098,
    "w": 10.0,
    "h": 8.8,
    "text": "\u56db",
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
    "id": "weekday_5",
    "x": 150.0,
    "y": 73.158,
    "w": 10.0,
    "h": 8.608,
    "text": "\u4e94",
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
    "id": "weekday_6",
    "x": 176.0,
    "y": 72.606,
    "w": 10.0,
    "h": 9.454,
    "text": "\u516d",
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
    "id": "day_1",
    "x": 72.5025,
    "y": 91.327,
    "w": 8.995000000000001,
    "h": 10.597,
    "text": "1",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_2",
    "x": 98.5025,
    "y": 91.21,
    "w": 8.995000000000001,
    "h": 10.714,
    "text": "2",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_3",
    "x": 124.5025,
    "y": 91.21,
    "w": 8.995000000000001,
    "h": 10.831,
    "text": "3",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_4",
    "x": 150.5025,
    "y": 91.327,
    "w": 8.995000000000001,
    "h": 10.597,
    "text": "4",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_5",
    "x": 176.5025,
    "y": 91.327,
    "w": 8.995000000000001,
    "h": 10.714,
    "text": "5",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_6",
    "x": 20.502499999999998,
    "y": 118.21,
    "w": 8.995000000000001,
    "h": 10.831,
    "text": "6",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_7",
    "x": 46.5025,
    "y": 118.327,
    "w": 8.995000000000001,
    "h": 10.597,
    "text": "7",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_8",
    "x": 72.5025,
    "y": 118.228,
    "w": 8.995000000000001,
    "h": 10.812999999999999,
    "text": "8",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_9",
    "x": 98.5025,
    "y": 118.21,
    "w": 8.995000000000001,
    "h": 10.831,
    "text": "9",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_10",
    "x": 122.005,
    "y": 118.21,
    "w": 13.99,
    "h": 10.831,
    "text": "10",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_11",
    "x": 148.005,
    "y": 118.327,
    "w": 13.99,
    "h": 10.597,
    "text": "11",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_12",
    "x": 174.005,
    "y": 118.21,
    "w": 13.99,
    "h": 10.714,
    "text": "12",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_13",
    "x": 18.005,
    "y": 145.21,
    "w": 13.99,
    "h": 10.831,
    "text": "13",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_14",
    "x": 44.004999999999995,
    "y": 145.327,
    "w": 13.99,
    "h": 10.597,
    "text": "14",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_15",
    "x": 70.005,
    "y": 145.327,
    "w": 13.99,
    "h": 10.714,
    "text": "15",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_16",
    "x": 96.005,
    "y": 145.21,
    "w": 13.99,
    "h": 10.831,
    "text": "16",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_17",
    "x": 122.005,
    "y": 145.327,
    "w": 13.99,
    "h": 10.597,
    "text": "17",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_18",
    "x": 148.005,
    "y": 145.228,
    "w": 13.99,
    "h": 10.812999999999999,
    "text": "18",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_19",
    "x": 174.005,
    "y": 145.21,
    "w": 13.99,
    "h": 10.831,
    "text": "19",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_20",
    "x": 18.005,
    "y": 172.21,
    "w": 13.99,
    "h": 10.831,
    "text": "20",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_21",
    "x": 44.004999999999995,
    "y": 172.21,
    "w": 13.99,
    "h": 10.714,
    "text": "21",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_22",
    "x": 70.005,
    "y": 172.21,
    "w": 13.99,
    "h": 10.714,
    "text": "22",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_23",
    "x": 96.005,
    "y": 172.21,
    "w": 13.99,
    "h": 10.831,
    "text": "23",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_25",
    "x": 148.005,
    "y": 172.21,
    "w": 13.99,
    "h": 10.831,
    "text": "25",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_26",
    "x": 174.005,
    "y": 172.21,
    "w": 13.99,
    "h": 10.831,
    "text": "26",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_27",
    "x": 18.005,
    "y": 199.21,
    "w": 13.99,
    "h": 10.714,
    "text": "27",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_28",
    "x": 44.004999999999995,
    "y": 199.21,
    "w": 13.99,
    "h": 10.831,
    "text": "28",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_29",
    "x": 70.005,
    "y": 199.21,
    "w": 13.99,
    "h": 10.831,
    "text": "29",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "day_30",
    "x": 96.005,
    "y": 199.21,
    "w": 13.99,
    "h": 10.831,
    "text": "30",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 9.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "list_title",
    "x": 10.0,
    "y": 229.2375,
    "w": 65.4315,
    "h": 10.213999999999999,
    "text": "9\u670824\u65e5 \u5468\u56db \u00b7 \u4eca\u5929",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  }
]
```
