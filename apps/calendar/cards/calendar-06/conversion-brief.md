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
    "id": "invite_card",
    "x": 12.0,
    "y": 75.0,
    "w": 179.0,
    "h": 150.0,
    "role": "card",
    "native_candidates": [
      "View",
      "TaskplanProjectCard",
      "CamoTrackRow"
    ]
  },
  {
    "id": "invite_card_icon_tile",
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
    "id": "invite_card_icon",
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
      "path": "assets/invite_card_icon.svg",
      "sha256": "609914f6b50fabf79fe88ff741ffa6062d2b0145fe14c868107aa06ebc3b9f7c",
      "method": "reference_svg",
      "reference_sha256": "3421040fdf7cddb3d6281e878cf9f0aa9fd20234d72039dbc73f288326ddde25",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "accept_invite",
    "x": 22.0,
    "y": 165.0,
    "w": 50.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "accept_invite_surface",
    "x": 22.0,
    "y": 165.0,
    "w": 50.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "accept_invite_control",
    "x": 22.0,
    "y": 165.0,
    "w": 50.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "accept_invite_label",
    "x": 39.5,
    "y": 169.8575,
    "w": 19.0,
    "h": 11.1025,
    "text": "\u63a5\u53d7",
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
    "id": "maybe_invite",
    "x": 77.0,
    "y": 165.0,
    "w": 50.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "maybe_invite_surface",
    "x": 77.0,
    "y": 165.0,
    "w": 50.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "maybe_invite_control",
    "x": 77.0,
    "y": 165.0,
    "w": 50.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "maybe_invite_label",
    "x": 94.5,
    "y": 169.9175,
    "w": 19.0,
    "h": 10.975,
    "text": "\u5f85\u5b9a",
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
    "id": "decline_invite",
    "x": 132.0,
    "y": 165.0,
    "w": 47.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "decline_invite_surface",
    "x": 132.0,
    "y": 165.0,
    "w": 47.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "decline_invite_control",
    "x": 132.0,
    "y": 165.0,
    "w": 47.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "decline_invite_label",
    "x": 148.0,
    "y": 169.8875,
    "w": 19.0,
    "h": 10.9225,
    "text": "\u62d2\u7edd",
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
    "y": 192.0,
    "w": 120.0,
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
    "y": 192.0,
    "w": 120.0,
    "h": 16.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "open_calendar_control",
    "x": 22.0,
    "y": 192.0,
    "w": 120.0,
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
    "y": 195.172,
    "w": 60.0,
    "h": 10.454,
    "text": "\u5728\u65e5\u5386\u4e2d\u67e5\u770b\u5f53\u5929",
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
    "id": "invite_card_kicker",
    "x": 46.0,
    "y": 88.361,
    "w": 54.4205,
    "h": 9.9995,
    "text": "\u9080\u8bf7 \u00b7 \u6765\u81ea Sam",
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
    "id": "invite_title",
    "x": 22.0,
    "y": 109.94600000000001,
    "w": 48.0,
    "h": 14.384,
    "text": "\u5468\u672b\u8fdc\u8db3",
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
    "id": "invite_when",
    "x": 22.0,
    "y": 125.207,
    "w": 94.083,
    "h": 10.447,
    "text": "9\u670826\u65e5 \u5468\u516d \u00b7 09:00\u201312:00",
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
    "id": "invite_where",
    "x": 22.0,
    "y": 135.81550000000001,
    "w": 65.412,
    "h": 10.0515,
    "text": "\u897f\u5c71\u6b65\u9053 \u00b7 \u5bb6\u5ead\u65e5\u5386",
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
    "id": "invite_note",
    "x": 22.0,
    "y": 148.05200000000002,
    "w": 101.85400000000001,
    "h": 9.706,
    "text": "\u63a5\u53d7\u540e\u4f1a\u52a0\u5165\u4f60\u7684\u65e5\u5386\uff0c\u5e76\u901a\u77e5 Sam",
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
      "reference_sha256": "3421040fdf7cddb3d6281e878cf9f0aa9fd20234d72039dbc73f288326ddde25",
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
      "reference_sha256": "3421040fdf7cddb3d6281e878cf9f0aa9fd20234d72039dbc73f288326ddde25",
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
      "reference_sha256": "3421040fdf7cddb3d6281e878cf9f0aa9fd20234d72039dbc73f288326ddde25",
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
    "text": "09:43",
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
