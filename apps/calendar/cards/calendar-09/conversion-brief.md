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
    "id": "sync_card",
    "x": 12.0,
    "y": 75.0,
    "w": 179.0,
    "h": 134.0,
    "role": "card",
    "native_candidates": [
      "View",
      "TaskplanProjectCard",
      "CamoTrackRow"
    ]
  },
  {
    "id": "sync_card_icon_tile",
    "x": 22.0,
    "y": 84.0,
    "w": 18.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "sync_card_icon",
    "x": 25.5,
    "y": 87.5,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/sync_card_icon.svg",
      "sha256": "ecff35688dc4e6b71ac169aaf0d9fdf0c14fad161696f5d74c4eab6e74f2ce89",
      "method": "reference_svg",
      "reference_sha256": "2726bd46f4cc402772eff0f3b25e2a384c2638ffd27b35abb50789e3f97e58ca",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "sync_now",
    "x": 22.0,
    "y": 166.0,
    "w": 75.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "sync_now_surface",
    "x": 22.0,
    "y": 166.0,
    "w": 75.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "sync_now_control",
    "x": 22.0,
    "y": 166.0,
    "w": 75.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "sync_now_label",
    "x": 44.5,
    "y": 170.81249999999997,
    "w": 34.0,
    "h": 11.14,
    "text": "\u7acb\u5373\u540c\u6b65",
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
    "id": "open_calendar",
    "x": 104.0,
    "y": 166.0,
    "w": 75.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_calendar_surface",
    "x": 104.0,
    "y": 166.0,
    "w": 75.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_calendar_control",
    "x": 104.0,
    "y": 166.0,
    "w": 75.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_calendar_label",
    "x": 126.5,
    "y": 170.96999999999997,
    "w": 34.0,
    "h": 10.93,
    "text": "\u6253\u5f00\u65e5\u5386",
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
    "id": "sync_card_kicker",
    "x": 46.0,
    "y": 88.348,
    "w": 39.412000000000006,
    "h": 10.018999999999998,
    "text": "\u540c\u6b65 \u00b7 \u6700\u65b0",
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
    "id": "sync_title",
    "x": 22.0,
    "y": 109.77000000000001,
    "w": 70.0,
    "h": 14.527,
    "text": "\u4e24\u53f0\u8bbe\u5907\u4e00\u81f4",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 11.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "device_phone",
    "x": 22.0,
    "y": 127.172,
    "w": 69.366,
    "h": 10.44,
    "text": "Alex \u00b7 \u624b\u673a \u00b7 09:41",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 7.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "device_desktop",
    "x": 22.0,
    "y": 138.172,
    "w": 70.353,
    "h": 10.44,
    "text": "Sam \u00b7 \u684c\u9762 \u00b7 09:40",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 7.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "pending_count",
    "x": 22.0,
    "y": 149.8415,
    "w": 93.6025,
    "h": 10.038499999999999,
    "text": "0 \u9879\u5f85\u540c\u6b65\u66f4\u6539 \u00b7 \u7b2c 12 \u6b21\u540c\u6b65",
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
    "id": "sync_foot",
    "x": 22.0,
    "y": 192.256,
    "w": 103.0,
    "h": 9.2305,
    "text": "\u66f4\u6539\u53ea\u4f1a\u5728\u4f60\u64cd\u4f5c\u540e\u53d1\u9001\uff0c\u4e0d\u4f1a\u81ea\u52a8\u6392\u7a0b",
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
    "id": "dock",
    "x": 12.0,
    "y": 345.0,
    "w": 179.0,
    "h": 33.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "dock_calendar",
    "x": 20.0,
    "y": 348.0,
    "w": 50.0,
    "h": 27.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "dock_calendar_tile",
    "x": 35.0,
    "y": 349.0,
    "w": 20.0,
    "h": 17.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "dock_calendar_icon",
    "x": 40.0,
    "y": 351.5,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/dock_calendar_icon.svg",
      "sha256": "fae6f2960edcdfc4069a9d562bcec85c0105f25bd880a22ed38f7c911404f57a",
      "method": "reference_svg",
      "reference_sha256": "2726bd46f4cc402772eff0f3b25e2a384c2638ffd27b35abb50789e3f97e58ca",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "dock_calendar_control",
    "x": 20.0,
    "y": 348.0,
    "w": 50.0,
    "h": 27.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "dock_calendar_label",
    "x": 40.0,
    "y": 366.725,
    "w": 14.0,
    "h": 8.34,
    "text": "\u65e5\u5386",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 5.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "dock_mail",
    "x": 78.0,
    "y": 348.0,
    "w": 50.0,
    "h": 27.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "dock_mail_tile",
    "x": 93.0,
    "y": 349.0,
    "w": 20.0,
    "h": 17.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "dock_mail_icon",
    "x": 98.0,
    "y": 351.5,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/dock_mail_icon.svg",
      "sha256": "5700c00c4c613834993bb1a26f0a30a90310de366905c06f79adcf819de15be8",
      "method": "reference_svg",
      "reference_sha256": "2726bd46f4cc402772eff0f3b25e2a384c2638ffd27b35abb50789e3f97e58ca",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "dock_mail_control",
    "x": 78.0,
    "y": 348.0,
    "w": 50.0,
    "h": 27.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "dock_mail_label",
    "x": 98.0,
    "y": 366.485,
    "w": 14.0,
    "h": 8.594999999999999,
    "text": "\u90ae\u4ef6",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 5.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "dock_settings",
    "x": 136.0,
    "y": 348.0,
    "w": 50.0,
    "h": 27.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "dock_settings_tile",
    "x": 151.0,
    "y": 349.0,
    "w": 20.0,
    "h": 17.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "dock_settings_icon",
    "x": 156.0,
    "y": 351.5,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/dock_settings_icon.svg",
      "sha256": "dac6f2b7e7bbd08975b93c59d8636db41c1d32f0e5a0f0429f4c55d1bcd2d176",
      "method": "reference_svg",
      "reference_sha256": "2726bd46f4cc402772eff0f3b25e2a384c2638ffd27b35abb50789e3f97e58ca",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "dock_settings_control",
    "x": 136.0,
    "y": 348.0,
    "w": 50.0,
    "h": 27.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "dock_settings_label",
    "x": 156.0,
    "y": 366.57,
    "w": 14.0,
    "h": 8.524999999999999,
    "text": "\u8bbe\u7f6e",
    "font_src": "self:resources/service/NotoSansSC-Regular.ttf",
    "size": 5.0,
    "weight": 400,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "desk_date",
    "x": 12.0,
    "y": 15.829999999999998,
    "w": 48.167500000000004,
    "h": 10.585,
    "text": "9\u670824\u65e5 \u5468\u56db",
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
    "id": "desk_time",
    "x": 12.0,
    "y": 30.275999999999996,
    "w": 52.33,
    "h": 17.823999999999998,
    "text": "09:44",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 18.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  }
]
```
