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
    "h": 118.0,
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
      "reference_sha256": "d5fe079d7362afeee6360dde7f39bacf17fcb09420edf3017d818995be157fe5",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "undo_delete",
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
    "id": "undo_delete_surface",
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
    "id": "undo_delete_control",
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
    "id": "undo_delete_label",
    "x": 44.5,
    "y": 158.80499999999998,
    "w": 34.0,
    "h": 11.192499999999999,
    "text": "\u64a4\u9500\u5220\u9664",
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
    "id": "open_calendar_surface",
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
    "id": "open_calendar_control",
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
    "id": "open_calendar_label",
    "x": 126.5,
    "y": 158.96999999999997,
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
    "id": "calendar_card_kicker",
    "x": 46.0,
    "y": 88.3285,
    "w": 45.912000000000006,
    "h": 10.032,
    "text": "\u65e5\u5386 \u00b7 \u5df2\u5220\u9664",
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
    "id": "removed_title",
    "x": 22.0,
    "y": 109.94600000000001,
    "w": 65.963,
    "h": 14.373,
    "text": "\u4e0e Sam \u665a\u9910",
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
    "id": "removed_when",
    "x": 22.0,
    "y": 125.508,
    "w": 94.083,
    "h": 10.146,
    "text": "9\u670824\u65e5 \u5468\u56db \u00b7 19:00\u201320:30",
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
    "id": "removed_note",
    "x": 22.0,
    "y": 135.8285,
    "w": 141.4685,
    "h": 10.552,
    "text": "\u5728 Alex \u00b7 \u624b\u673a \u4e0a\u5220\u9664\uff1bSam \u7684\u526f\u672c\u5df2\u540c\u6b65\u66f4\u65b0",
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
      "reference_sha256": "d5fe079d7362afeee6360dde7f39bacf17fcb09420edf3017d818995be157fe5",
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
      "reference_sha256": "d5fe079d7362afeee6360dde7f39bacf17fcb09420edf3017d818995be157fe5",
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
      "reference_sha256": "d5fe079d7362afeee6360dde7f39bacf17fcb09420edf3017d818995be157fe5",
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
    "text": "09:45",
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
