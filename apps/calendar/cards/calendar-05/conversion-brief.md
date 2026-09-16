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
    "id": "calendar_card",
    "x": 12.0,
    "y": 75.0,
    "w": 179.0,
    "h": 131.0,
    "role": "card",
    "native_candidates": [
      "View",
      "TaskplanProjectCard",
      "CamoTrackRow"
    ]
  },
  {
    "id": "calendar_card_icon_tile",
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
    "id": "calendar_card_icon",
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
      "path": "assets/calendar_card_icon.svg",
      "sha256": "a770c5049093a0a39fda365e21393679d7bb9d24b9548950e6e272361b91070d",
      "method": "reference_svg",
      "reference_sha256": "fd33bc1301ea51b1470cbf4a9811f41593620fa06ffaeb7396ea72a0c52da03f",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "ack_sync",
    "x": 22.0,
    "y": 154.0,
    "w": 75.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "ack_sync_surface",
    "x": 22.0,
    "y": 154.0,
    "w": 75.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ack_sync_control",
    "x": 22.0,
    "y": 154.0,
    "w": 75.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "ack_sync_label",
    "x": 48.25,
    "y": 158.83499999999998,
    "w": 26.5,
    "h": 11.1325,
    "text": "\u77e5\u9053\u4e86",
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
    "id": "undo_event",
    "x": 104.0,
    "y": 154.0,
    "w": 75.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "undo_event_surface",
    "x": 104.0,
    "y": 154.0,
    "w": 75.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "undo_event_control",
    "x": 104.0,
    "y": 154.0,
    "w": 75.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "undo_event_label",
    "x": 134.0,
    "y": 158.9175,
    "w": 19.0,
    "h": 10.9675,
    "text": "\u64a4\u9500",
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
    "id": "open_calendar",
    "x": 22.0,
    "y": 180.0,
    "w": 100.0,
    "h": 16.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_calendar_surface",
    "x": 22.0,
    "y": 180.0,
    "w": 100.0,
    "h": 16.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_calendar_control",
    "x": 22.0,
    "y": 180.0,
    "w": 100.0,
    "h": 16.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "open_calendar_label",
    "x": 22.0,
    "y": 183.172,
    "w": 46.0,
    "h": 10.468,
    "text": "\u5728\u65e5\u5386\u4e2d\u6253\u5f00",
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
    "id": "calendar_card_kicker",
    "x": 46.0,
    "y": 88.37400000000001,
    "w": 45.912000000000006,
    "h": 9.992999999999999,
    "text": "\u65e5\u5386 \u00b7 \u5df2\u540c\u6b65",
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
    "id": "card_title",
    "x": 22.0,
    "y": 109.97900000000001,
    "w": 37.0,
    "h": 14.405999999999999,
    "text": "\u94a2\u7434\u8bfe",
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
    "id": "card_when",
    "x": 22.0,
    "y": 125.508,
    "w": 94.083,
    "h": 10.146,
    "text": "9\u670825\u65e5 \u5468\u4e94 \u00b7 16:00\u201317:00",
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
    "id": "card_source",
    "x": 22.0,
    "y": 135.81550000000001,
    "w": 88.916,
    "h": 10.0515,
    "text": "\u5bb6\u5ead\u65e5\u5386 \u00b7 \u6765\u81ea Alex \u00b7 \u624b\u673a",
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
      "reference_sha256": "fd33bc1301ea51b1470cbf4a9811f41593620fa06ffaeb7396ea72a0c52da03f",
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
      "reference_sha256": "fd33bc1301ea51b1470cbf4a9811f41593620fa06ffaeb7396ea72a0c52da03f",
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
      "reference_sha256": "fd33bc1301ea51b1470cbf4a9811f41593620fa06ffaeb7396ea72a0c52da03f",
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
    "text": "09:42",
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
