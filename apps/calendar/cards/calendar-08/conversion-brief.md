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
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "done_calendars",
    "x": 153.0,
    "y": 25.0,
    "w": 44.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "done_calendars_surface",
    "x": 153.0,
    "y": 25.0,
    "w": 44.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "done_calendars_control",
    "x": 153.0,
    "y": 25.0,
    "w": 44.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "done_calendars_label",
    "x": 153.0,
    "y": 28.489500000000003,
    "w": 21.0,
    "h": 11.956,
    "text": "\u5b8c\u6210",
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
    "id": "calendars_group",
    "x": 10.0,
    "y": 65.0,
    "w": 183.0,
    "h": 105.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_family",
    "x": 10.0,
    "y": 67.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "toggle_family_surface",
    "x": 10.0,
    "y": 67.0,
    "w": 183.0,
    "h": 25.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_family_control",
    "x": 10.0,
    "y": 67.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "toggle_family_check",
    "x": 16.0,
    "y": 74.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/toggle_family_check.svg",
      "sha256": "9706b555dd2c028aae681bc4382ae03e1bb0a40be2842593c5414945daabb386",
      "method": "reference_svg",
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "toggle_family_dot",
    "x": 33.0,
    "y": 76.5,
    "w": 6.0,
    "h": 6.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_family_line",
    "x": 33.0,
    "y": 92.0,
    "w": 155.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_family_label",
    "x": 44.0,
    "y": 69.9895,
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
    "id": "toggle_family_sub",
    "x": 44.0,
    "y": 81.034,
    "w": 37.198,
    "h": 9.574,
    "text": "\u4e0e Sam \u5171\u4eab",
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
    "id": "toggle_work",
    "x": 10.0,
    "y": 93.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "toggle_work_surface",
    "x": 10.0,
    "y": 93.0,
    "w": 183.0,
    "h": 25.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_work_control",
    "x": 10.0,
    "y": 93.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "toggle_work_check",
    "x": 16.0,
    "y": 100.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/toggle_work_check.svg",
      "sha256": "08794cca4e88ce6d0812d3522179f12292ca9065f33f98f2f9df89e50cb266d8",
      "method": "reference_svg",
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "toggle_work_dot",
    "x": 33.0,
    "y": 102.5,
    "w": 6.0,
    "h": 6.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_work_line",
    "x": 33.0,
    "y": 118.0,
    "w": 155.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_work_label",
    "x": 44.0,
    "y": 100.10000000000001,
    "w": 21.0,
    "h": 11.7775,
    "text": "\u5de5\u4f5c",
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
    "id": "toggle_personal",
    "x": 10.0,
    "y": 119.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "toggle_personal_surface",
    "x": 10.0,
    "y": 119.0,
    "w": 183.0,
    "h": 25.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_personal_control",
    "x": 10.0,
    "y": 119.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "toggle_personal_check",
    "x": 16.0,
    "y": 126.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/toggle_personal_check.svg",
      "sha256": "637115c01f028119a9483b7a910b874cedac11486eef1bd70427a2508601d4c9",
      "method": "reference_svg",
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "toggle_personal_dot",
    "x": 33.0,
    "y": 128.5,
    "w": 6.0,
    "h": 6.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_personal_line",
    "x": 33.0,
    "y": 144.0,
    "w": 155.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_personal_label",
    "x": 44.0,
    "y": 126.0575,
    "w": 21.0,
    "h": 11.82,
    "text": "\u4e2a\u4eba",
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
    "id": "toggle_birthdays",
    "x": 10.0,
    "y": 145.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "toggle_birthdays_surface",
    "x": 10.0,
    "y": 145.0,
    "w": 183.0,
    "h": 25.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_birthdays_control",
    "x": 10.0,
    "y": 145.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "toggle_birthdays_check",
    "x": 16.0,
    "y": 152.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/toggle_birthdays_check.svg",
      "sha256": "6eae36f3e0a6a8ea1814030eebd24913caefe547f575d6a68622c8bf231bf233",
      "method": "reference_svg",
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "toggle_birthdays_dot",
    "x": 33.0,
    "y": 154.5,
    "w": 6.0,
    "h": 6.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "toggle_birthdays_label",
    "x": 44.0,
    "y": 148.06600000000003,
    "w": 21.0,
    "h": 11.726500000000001,
    "text": "\u751f\u65e5",
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
    "id": "toggle_birthdays_sub",
    "x": 44.0,
    "y": 159.07600000000002,
    "w": 22.0,
    "h": 9.532,
    "text": "\u5df2\u9690\u85cf",
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
    "id": "sync_group",
    "x": 10.0,
    "y": 182.0,
    "w": 183.0,
    "h": 32.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "sync_icon",
    "x": 18.0,
    "y": 192.5,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/sync_icon.svg",
      "sha256": "3f46bec658d92aa319ecba3f3c61db594278759bf0cda8ca77c0117161beb82f",
      "method": "reference_svg",
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
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
      "sha256": "4591784cd98a28f3f45006092e6225954c2cc72c9f2a8cd9d44d4f6a30e9139e",
      "method": "reference_svg",
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
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
      "sha256": "b4e3170d50ee08fb636d51da785fc1733a9b65358eb2e6d625793c1784b5a368",
      "method": "reference_svg",
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
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
      "reference_sha256": "c15eda187bff1e029d67ac17792fbb9a5f7128017dcfc33ca4080a9a1eb32930",
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
    "id": "nav_title",
    "x": 93.0,
    "y": 28.812500000000004,
    "w": 21.0,
    "h": 11.65,
    "text": "\u65e5\u5386",
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
    "id": "section_icloud",
    "x": 18.0,
    "y": 54.16,
    "w": 23.695,
    "h": 9.258500000000002,
    "text": "iCloud",
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
    "id": "sync_status",
    "x": 36.0,
    "y": 186.96999999999997,
    "w": 56.095,
    "h": 10.915,
    "text": "\u5df2\u540c\u6b65 \u00b7 09:41",
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
    "id": "sync_devices",
    "x": 36.0,
    "y": 199.05200000000002,
    "w": 104.224,
    "h": 9.706,
    "text": "2 \u53f0\u8bbe\u5907\uff1aAlex \u00b7 \u624b\u673a\uff0cSam \u00b7 \u684c\u9762",
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
