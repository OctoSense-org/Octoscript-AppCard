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
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "back_day",
    "x": 6.0,
    "y": 25.0,
    "w": 58.5,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "back_day_surface",
    "x": 6.0,
    "y": 25.0,
    "w": 58.5,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "back_day_control",
    "x": 6.0,
    "y": 25.0,
    "w": 58.5,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "back_day_icon",
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
      "path": "assets/back_day_icon.svg",
      "sha256": "1027440064a9b8cfc64637059559b54eb6408674274992d1ce108ed1365b6875",
      "method": "reference_svg",
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "back_day_label",
    "x": 18.0,
    "y": 29.0165,
    "w": 35.1525,
    "h": 11.378,
    "text": "9\u670824\u65e5",
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
    "id": "edit_event",
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
    "id": "edit_event_surface",
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
    "id": "edit_event_control",
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
    "id": "edit_event_label",
    "x": 153.0,
    "y": 28.515000000000004,
    "w": 21.0,
    "h": 11.870999999999999,
    "text": "\u7f16\u8f91",
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
    "id": "detail_head",
    "x": 10.0,
    "y": 52.0,
    "w": 183.0,
    "h": 46.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "detail_bar",
    "x": 10.0,
    "y": 52.0,
    "w": 2.5,
    "h": 46.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "detail_group",
    "x": 10.0,
    "y": 107.0,
    "w": 183.0,
    "h": 107.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "detail_when_icon",
    "x": 18.0,
    "y": 115.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/detail_when_icon.svg",
      "sha256": "e6bb59cb36161c3e967fdb5d191c0bd2382a7f180ee8144fb2ce544dc04abfd1",
      "method": "reference_svg",
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "detail_when_line",
    "x": 36.0,
    "y": 135.0,
    "w": 150.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "detail_where_icon",
    "x": 18.0,
    "y": 141.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/detail_where_icon.svg",
      "sha256": "21f575dcf7735032a5cdb8adce4a153ca1cae79f1c4e8c4de7250565b423e946",
      "method": "reference_svg",
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "detail_where_line",
    "x": 36.0,
    "y": 161.0,
    "w": 150.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "detail_alert_icon",
    "x": 18.0,
    "y": 167.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/detail_alert_icon.svg",
      "sha256": "3d08916d7179430398ab4d5382238ae91fad07d1dcf8f77c71edebf0b1427094",
      "method": "reference_svg",
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "detail_alert_line",
    "x": 36.0,
    "y": 187.0,
    "w": 150.0,
    "h": 0.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "detail_notes_icon",
    "x": 18.0,
    "y": 193.0,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/detail_notes_icon.svg",
      "sha256": "82aa9e40a3fd7200fa5bc213dfe4b592ccdb1d5d0c495d0f00d5622d59057465",
      "method": "reference_svg",
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "sync_badge",
    "x": 10.0,
    "y": 223.0,
    "w": 183.0,
    "h": 22.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "sync_badge_icon",
    "x": 17.0,
    "y": 228.5,
    "w": 11.0,
    "h": 11.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/sync_badge_icon.svg",
      "sha256": "f822beb1b0027f61f96023e61a5c74efab224984d9a631560cc6473d095949b5",
      "method": "reference_svg",
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "delete_event",
    "x": 10.0,
    "y": 260.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "delete_event_surface",
    "x": 10.0,
    "y": 260.0,
    "w": 183.0,
    "h": 25.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "delete_event_control",
    "x": 10.0,
    "y": 260.0,
    "w": 183.0,
    "h": 25.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "delete_event_label",
    "x": 84.5,
    "y": 267.0065,
    "w": 38.0,
    "h": 11.888,
    "text": "\u5220\u9664\u65e5\u7a0b",
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
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
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
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
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
      "reference_sha256": "cd3359370450b440f43bdb06cc9557397d91423d6b0a53f0ffb1cb35691397a7",
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
    "id": "event_title",
    "x": 20.0,
    "y": 58.032,
    "w": 71.596,
    "h": 15.315999999999999,
    "text": "\u4e0e Sam \u665a\u9910",
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
    "id": "event_calendar",
    "x": 20.0,
    "y": 74.8155,
    "w": 75.3765,
    "h": 10.0515,
    "text": "\u5bb6\u5ead\u65e5\u5386 \u00b7 \u4e0e Sam \u5171\u4eab",
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
    "id": "detail_when",
    "x": 36.0,
    "y": 112.44,
    "w": 72.3175,
    "h": 10.975,
    "text": "2026\u5e749\u670824\u65e5 \u5468\u56db",
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
    "id": "detail_when_sub",
    "x": 36.0,
    "y": 123.485,
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
    "id": "detail_where",
    "x": 36.0,
    "y": 138.46249999999998,
    "w": 34.0,
    "h": 10.899999999999999,
    "text": "\u83b2\u82b1\u53a8\u623f",
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
    "id": "detail_where_sub",
    "x": 36.0,
    "y": 148.822,
    "w": 46.626999999999995,
    "h": 10.040666666666667,
    "text": "\u5357\u4eac\u897f\u8def 88 \u53f7",
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
    "id": "detail_alert",
    "x": 36.0,
    "y": 164.4775,
    "w": 19.0,
    "h": 10.892499999999998,
    "text": "\u63d0\u9192",
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
    "id": "detail_alert_sub",
    "x": 36.0,
    "y": 174.8415,
    "w": 40.126999999999995,
    "h": 10.032,
    "text": "\u63d0\u524d 30 \u5206\u949f",
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
    "id": "detail_notes",
    "x": 36.0,
    "y": 190.43999999999997,
    "w": 19.0,
    "h": 10.93,
    "text": "\u5907\u6ce8",
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
    "id": "detail_notes_sub",
    "x": 36.0,
    "y": 200.8285,
    "w": 49.5,
    "h": 10.038499999999999,
    "text": "\u8ba2\u4e86\u9760\u7a97\u7684\u4f4d\u5b50",
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
    "id": "sync_note",
    "x": 33.0,
    "y": 229.374,
    "w": 128.48149999999998,
    "h": 9.992999999999999,
    "text": "\u7531 Sam \u00b7 \u684c\u9762 \u6dfb\u52a0 \u00b7 09:40 \u5df2\u540c\u6b65\u5230\u672c\u673a",
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
