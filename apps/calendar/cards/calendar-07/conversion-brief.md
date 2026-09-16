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
      "reference_sha256": "fb82337e809629acf41c094618eba987d6b79662be05c8d91f28dd7d453b205f",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "back_editor",
    "x": 6.0,
    "y": 25.0,
    "w": 50.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "back_editor_surface",
    "x": 6.0,
    "y": 25.0,
    "w": 50.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "back_editor_control",
    "x": 6.0,
    "y": 25.0,
    "w": 50.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "back_editor_icon",
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
      "path": "assets/back_editor_icon.svg",
      "sha256": "1027440064a9b8cfc64637059559b54eb6408674274992d1ce108ed1365b6875",
      "method": "reference_svg",
      "reference_sha256": "fb82337e809629acf41c094618eba987d6b79662be05c8d91f28dd7d453b205f",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "back_editor_label",
    "x": 18.0,
    "y": 28.532000000000004,
    "w": 38.0,
    "h": 11.854,
    "text": "\u65b0\u5efa\u65e5\u7a0b",
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
    "id": "slot_13",
    "x": 10.0,
    "y": 84.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_13_surface",
    "x": 10.0,
    "y": 84.0,
    "w": 183.0,
    "h": 28.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "slot_13_control",
    "x": 10.0,
    "y": 84.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_13_label",
    "x": 20.0,
    "y": 93.36500000000001,
    "w": 54.830000000000005,
    "h": 10.4515,
    "text": "13:00 \u2013 14:00",
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
    "id": "slot_14",
    "x": 10.0,
    "y": 117.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_14_surface",
    "x": 10.0,
    "y": 117.0,
    "w": 183.0,
    "h": 28.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "slot_14_control",
    "x": 10.0,
    "y": 117.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_14_label",
    "x": 20.0,
    "y": 121.86500000000002,
    "w": 54.830000000000005,
    "h": 10.4515,
    "text": "14:00 \u2013 15:00",
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
    "id": "slot_14_why",
    "x": 20.0,
    "y": 133.03400000000002,
    "w": 58.0,
    "h": 9.597999999999999,
    "text": "\u4e0e\u300c\u56e2\u961f\u540c\u6b65\u300d\u51b2\u7a81",
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
    "id": "slot_15",
    "x": 10.0,
    "y": 150.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_15_surface",
    "x": 10.0,
    "y": 150.0,
    "w": 183.0,
    "h": 28.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "slot_15_control",
    "x": 10.0,
    "y": 150.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_15_label",
    "x": 20.0,
    "y": 159.365,
    "w": 54.830000000000005,
    "h": 10.4515,
    "text": "15:00 \u2013 16:00",
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
    "id": "slot_16",
    "x": 10.0,
    "y": 183.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_16_surface",
    "x": 10.0,
    "y": 183.0,
    "w": 183.0,
    "h": 28.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "slot_16_control",
    "x": 10.0,
    "y": 183.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_16_check",
    "x": 175.0,
    "y": 191.5,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/slot_16_check.svg",
      "sha256": "9706b555dd2c028aae681bc4382ae03e1bb0a40be2842593c5414945daabb386",
      "method": "reference_svg",
      "reference_sha256": "fb82337e809629acf41c094618eba987d6b79662be05c8d91f28dd7d453b205f",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "slot_16_label",
    "x": 20.0,
    "y": 192.365,
    "w": 54.830000000000005,
    "h": 10.4515,
    "text": "16:00 \u2013 17:00",
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
    "id": "slot_17",
    "x": 10.0,
    "y": 216.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_17_surface",
    "x": 10.0,
    "y": 216.0,
    "w": 183.0,
    "h": 28.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "slot_17_control",
    "x": 10.0,
    "y": 216.0,
    "w": 183.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "slot_17_label",
    "x": 20.0,
    "y": 225.365,
    "w": 54.830000000000005,
    "h": 10.4515,
    "text": "17:00 \u2013 18:00",
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
    "id": "picker_title",
    "x": 10.0,
    "y": 49.044,
    "w": 52.0,
    "h": 15.388,
    "text": "\u9009\u62e9\u65f6\u95f4",
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
    "id": "picker_sub",
    "x": 10.0,
    "y": 64.8155,
    "w": 77.6905,
    "h": 10.0775,
    "text": "9\u670824\u65e5 \u5468\u56db \u00b7 \u5bb6\u5ead\u65e5\u5386",
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
    "id": "picker_hint",
    "x": 50.5,
    "y": 255.034,
    "w": 106.0,
    "h": 10.048,
    "text": "\u51b2\u7a81\u65f6\u6bb5\u4e0d\u53ef\u9009\u62e9\uff1b\u65e5\u7a0b\u4e0d\u4f1a\u81ea\u52a8\u91cd\u53e0",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 6.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  }
]
```
