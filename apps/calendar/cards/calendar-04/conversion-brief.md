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
      "reference_sha256": "1955f30bbd57f4dd0ad00ed9915b0109863b12a2a7bad47344538e83d8cc77af",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "cancel_editor",
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
    "id": "cancel_editor_surface",
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
    "id": "cancel_editor_control",
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
    "id": "cancel_editor_label",
    "x": 6.0,
    "y": 28.557500000000005,
    "w": 21.0,
    "h": 11.828499999999998,
    "text": "\u53d6\u6d88",
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
    "id": "save_event",
    "x": 159.0,
    "y": 27.0,
    "w": 36.0,
    "h": 15.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "save_event_surface",
    "x": 159.0,
    "y": 27.0,
    "w": 36.0,
    "h": 15.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "save_event_control",
    "x": 159.0,
    "y": 27.0,
    "w": 36.0,
    "h": 15.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "save_event_label",
    "x": 169.5,
    "y": 29.439999999999998,
    "w": 19.0,
    "h": 11.004999999999999,
    "text": "\u6dfb\u52a0",
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
    "id": "title_group",
    "x": 10.0,
    "y": 54.0,
    "w": 183.0,
    "h": 50.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "title_line",
    "x": 18.0,
    "y": 79.0,
    "w": 167.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "time_group",
    "x": 10.0,
    "y": 113.0,
    "w": 183.0,
    "h": 75.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "allday_switch",
    "x": 165.0,
    "y": 119.0,
    "w": 25.5,
    "h": 15.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "allday_knob",
    "x": 166.0,
    "y": 120.0,
    "w": 13.5,
    "h": 13.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "allday_line",
    "x": 18.0,
    "y": 138.0,
    "w": 167.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pick_time",
    "x": 10.0,
    "y": 140.0,
    "w": 183.0,
    "h": 23.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pick_time_surface",
    "x": 10.0,
    "y": 140.0,
    "w": 183.0,
    "h": 23.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pick_time_control",
    "x": 10.0,
    "y": 140.0,
    "w": 183.0,
    "h": 23.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "start_chip",
    "x": 96.0,
    "y": 144.0,
    "w": 89.0,
    "h": 15.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pick_time_label",
    "x": 18.0,
    "y": 146.0575,
    "w": 21.0,
    "h": 11.8625,
    "text": "\u5f00\u59cb",
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
    "id": "start_value",
    "x": 107.36875,
    "y": 146.82999999999998,
    "w": 70.2625,
    "h": 10.585,
    "text": "9\u670825\u65e5 \u5468\u4e94  16:00",
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
    "id": "start_line",
    "x": 18.0,
    "y": 164.0,
    "w": 167.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "end_chip",
    "x": 147.0,
    "y": 169.0,
    "w": 38.0,
    "h": 15.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "meta_group",
    "x": 10.0,
    "y": 197.0,
    "w": 183.0,
    "h": 50.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pick_calendar",
    "x": 10.0,
    "y": 199.0,
    "w": 183.0,
    "h": 23.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pick_calendar_surface",
    "x": 10.0,
    "y": 199.0,
    "w": 183.0,
    "h": 23.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pick_calendar_control",
    "x": 10.0,
    "y": 199.0,
    "w": 183.0,
    "h": 23.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "calendar_dot",
    "x": 146.0,
    "y": 208.0,
    "w": 5.0,
    "h": 5.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "calendar_chevron",
    "x": 181.0,
    "y": 206.5,
    "w": 8.0,
    "h": 8.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/calendar_chevron.svg",
      "sha256": "173ecccd4e15dcdca80c6086908e78139e01ce9ec919f593d7b24146601fee9e",
      "method": "reference_svg",
      "reference_sha256": "1955f30bbd57f4dd0ad00ed9915b0109863b12a2a7bad47344538e83d8cc77af",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "pick_calendar_label",
    "x": 18.0,
    "y": 205.48250000000002,
    "w": 21.0,
    "h": 11.378,
    "text": "\u65e5\u5386",
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
    "id": "calendar_value",
    "x": 154.0,
    "y": 204.98950000000002,
    "w": 21.0,
    "h": 11.913499999999999,
    "text": "\u5bb6\u5ead",
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
    "id": "calendar_line",
    "x": 18.0,
    "y": 223.0,
    "w": 167.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "alert_chevron",
    "x": 181.0,
    "y": 230.5,
    "w": 8.0,
    "h": 8.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/alert_chevron.svg",
      "sha256": "173ecccd4e15dcdca80c6086908e78139e01ce9ec919f593d7b24146601fee9e",
      "method": "reference_svg",
      "reference_sha256": "1955f30bbd57f4dd0ad00ed9915b0109863b12a2a7bad47344538e83d8cc77af",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "sync_hint",
    "x": 10.0,
    "y": 256.0,
    "w": 183.0,
    "h": 20.0,
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
    "id": "nav_title",
    "x": 84.5,
    "y": 28.438500000000005,
    "w": 38.0,
    "h": 12.032499999999999,
    "text": "\u65b0\u5efa\u65e5\u7a0b",
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
    "id": "title_field",
    "x": 18.0,
    "y": 62.591499999999996,
    "w": 29.5,
    "h": 11.8455,
    "text": "\u94a2\u7434\u8bfe",
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
    "id": "location_field",
    "x": 18.0,
    "y": 86.5575,
    "w": 63.5,
    "h": 11.854,
    "text": "\u5730\u70b9\u6216\u89c6\u9891\u901a\u8bdd",
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
    "id": "allday_label",
    "x": 18.0,
    "y": 119.97250000000003,
    "w": 21.0,
    "h": 11.8965,
    "text": "\u5168\u5929",
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
    "id": "end_label",
    "x": 18.0,
    "y": 171.0575,
    "w": 21.0,
    "h": 11.828499999999998,
    "text": "\u7ed3\u675f",
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
    "id": "end_value",
    "x": 156.6325,
    "y": 172.17499999999998,
    "w": 22.735000000000003,
    "h": 9.692499999999999,
    "text": "17:00",
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
    "id": "alert_label",
    "x": 18.0,
    "y": 229.07450000000003,
    "w": 21.0,
    "h": 11.811499999999999,
    "text": "\u63d0\u9192",
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
    "id": "alert_value",
    "x": 128.757,
    "y": 229.0235,
    "w": 51.242999999999995,
    "h": 11.888,
    "text": "\u63d0\u524d 30 \u5206\u949f",
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
    "id": "sync_hint_text",
    "x": 60.76775,
    "y": 261.348,
    "w": 85.4645,
    "h": 10.018999999999998,
    "text": "\u4fdd\u5b58\u540e\u4f1a\u540c\u6b65\u5230 Sam \u7684\u684c\u9762",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.5,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  }
]
```
