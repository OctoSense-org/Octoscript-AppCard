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
    "id": "viewfinder",
    "x": 0.0,
    "y": 19.7,
    "w": 203.0,
    "h": 270.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pro_format",
    "x": 6.0,
    "y": 0.0,
    "w": 26.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_format_control",
    "x": 6.0,
    "y": 0.0,
    "w": 26.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_format_bg",
    "x": 6.0,
    "y": 0.0,
    "w": 26.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pro_format_label",
    "x": 12.656,
    "y": 4.933,
    "w": 16.688,
    "h": 8.992,
    "text": "JPG",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "top_pill",
    "x": 135.0,
    "y": 0.0,
    "w": 62.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "flash",
    "x": 140.0,
    "y": 0.0,
    "w": 26.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "flash_control",
    "x": 140.0,
    "y": 0.0,
    "w": 26.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "flash_icon",
    "x": 148.0,
    "y": 4.0,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/flash_icon.svg",
      "sha256": "5211f51b665e15c3bfee8ce90c3b06e1f48e558fb05c03814f4ac48c2ca59fae",
      "method": "reference_svg",
      "reference_sha256": "a4e18f07968c8fd3fe41aff1a6160f4b0921606f774dc4e484653e0f0848c034",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "filter",
    "x": 166.0,
    "y": 0.0,
    "w": 26.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "filter_control",
    "x": 166.0,
    "y": 0.0,
    "w": 26.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "filter_icon",
    "x": 174.0,
    "y": 4.0,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/filter_icon.svg",
      "sha256": "6eee1ff89ed03065ba6ae219e4d934625e55c610c272be5d38274a4f781b3d5b",
      "method": "reference_svg",
      "reference_sha256": "a4e18f07968c8fd3fe41aff1a6160f4b0921606f774dc4e484653e0f0848c034",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "info",
    "x": 176.0,
    "y": 27.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "info_control",
    "x": 176.0,
    "y": 27.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "info_icon",
    "x": 181.0,
    "y": 32.75,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/info_icon.svg",
      "sha256": "464057134560328fe4b2638b58ae794f65ea3399e6ac34188268b37fd46204e3",
      "method": "reference_svg",
      "reference_sha256": "a4e18f07968c8fd3fe41aff1a6160f4b0921606f774dc4e484653e0f0848c034",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "pro_exposure_effect",
    "x": 176.0,
    "y": 49.6,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_exposure_effect_control",
    "x": 176.0,
    "y": 49.6,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_exposure_effect_icon",
    "x": 181.0,
    "y": 54.6,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/pro_exposure_effect_icon.svg",
      "sha256": "e76281c7a5b1c3472f9081b9278eb6c897d3e7123edac1722b63d53719b0fc48",
      "method": "reference_svg",
      "reference_sha256": "a4e18f07968c8fd3fe41aff1a6160f4b0921606f774dc4e484653e0f0848c034",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "zoom_pill",
    "x": 51.5,
    "y": 220.75,
    "w": 100.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "zoom_w",
    "x": 51.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_w_control",
    "x": 51.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_w_label",
    "x": 58.52625,
    "y": 226.7675,
    "w": 9.9475,
    "h": 8.8165,
    "text": "W",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "zoom_1",
    "x": 71.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_1_control",
    "x": 71.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_1_chip",
    "x": 71.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "zoom_1_label",
    "x": 77.756,
    "y": 226.7675,
    "w": 11.488,
    "h": 8.8165,
    "text": "1x",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "zoom_2",
    "x": 91.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_2_control",
    "x": 91.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_2_label",
    "x": 97.756,
    "y": 226.683,
    "w": 11.488,
    "h": 8.901,
    "text": "2x",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "zoom_3",
    "x": 111.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_3_control",
    "x": 111.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_3_label",
    "x": 117.756,
    "y": 226.683,
    "w": 11.488,
    "h": 8.992,
    "text": "3x",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "zoom_5",
    "x": 131.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_5_control",
    "x": 131.5,
    "y": 220.75,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "zoom_5_label",
    "x": 137.756,
    "y": 226.7675,
    "w": 11.488,
    "h": 8.907499999999999,
    "text": "5x",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "pro_bar",
    "x": 0.0,
    "y": 250.9,
    "w": 203.0,
    "h": 30.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pro_metering",
    "x": 6.0,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_metering_control",
    "x": 6.0,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_metering_icon",
    "x": 14.0,
    "y": 265.4,
    "w": 8.0,
    "h": 8.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/pro_metering_icon.svg",
      "sha256": "75ab26917a5239c00e0e1edd02b0ef129c55a9f69523ba7530b104a50afd9e3d",
      "method": "reference_svg",
      "reference_sha256": "a4e18f07968c8fd3fe41aff1a6160f4b0921606f774dc4e484653e0f0848c034",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "pro_metering_label",
    "x": 14.80125,
    "y": 255.1125,
    "w": 10.3975,
    "h": 9.557500000000001,
    "text": "M",
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
    "id": "pro_iso",
    "x": 39.35,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_iso_control",
    "x": 39.35,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_iso_label",
    "x": 44.885000000000005,
    "y": 255.01500000000004,
    "w": 16.93,
    "h": 9.76,
    "text": "ISO",
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
    "id": "pro_iso_value",
    "x": 47.93,
    "y": 265.01599999999996,
    "w": 10.84,
    "h": 8.584,
    "text": "50",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 6.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "pro_shutter",
    "x": 72.7,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_shutter_control",
    "x": 72.7,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_shutter_label",
    "x": 82.36,
    "y": 255.01500000000004,
    "w": 8.68,
    "h": 9.76,
    "text": "S",
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
    "id": "pro_shutter_value",
    "x": 76.69,
    "y": 264.722,
    "w": 20.02,
    "h": 9.874,
    "text": "1/125",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 6.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "pro_ev",
    "x": 106.05000000000001,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_ev_control",
    "x": 106.05000000000001,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_ev_label",
    "x": 109.67250000000001,
    "y": 255.1125,
    "w": 20.755,
    "h": 9.557500000000001,
    "text": "EV\u2022",
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
    "id": "pro_ev_value",
    "x": 116.34,
    "y": 265.01599999999996,
    "w": 7.42,
    "h": 8.584,
    "text": "0",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 6.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "pro_af",
    "x": 139.4,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_af_control",
    "x": 139.4,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_af_label",
    "x": 143.0525,
    "y": 255.1125,
    "w": 20.695,
    "h": 9.557500000000001,
    "text": "AF\u2022",
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
    "id": "pro_af_value",
    "x": 144.827,
    "y": 265.01599999999996,
    "w": 17.146,
    "h": 8.584,
    "text": "AF-C",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 6.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "pro_wb",
    "x": 172.75,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_wb_control",
    "x": 172.75,
    "y": 253.4,
    "w": 24.0,
    "h": 25.1,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_wb_underline",
    "x": 177.75,
    "y": 273.4,
    "w": 14.0,
    "h": 0.75,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pro_wb_label",
    "x": 175.015,
    "y": 255.1125,
    "w": 23.47,
    "h": 9.557500000000001,
    "text": "WB\u2022",
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
    "id": "pro_wb_value",
    "x": 178.201,
    "y": 265.09399999999994,
    "w": 17.098,
    "h": 8.422,
    "text": "AWB",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 6.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "mode_portrait",
    "x": 10.050000000000011,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_portrait_control",
    "x": 10.050000000000011,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_portrait_label",
    "x": 15.050000000000011,
    "y": 295.044,
    "w": 20.0,
    "h": 11.418,
    "text": "\u4eba\u50cf",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 8.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "mode_photo",
    "x": 36.2,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_photo_control",
    "x": 36.2,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_photo_label",
    "x": 41.2,
    "y": 295.036,
    "w": 20.0,
    "h": 11.463999999999999,
    "text": "\u62cd\u7167",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 8.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "mode_video",
    "x": 62.349999999999994,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_video_control",
    "x": 62.349999999999994,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_video_label",
    "x": 67.35,
    "y": 295.044,
    "w": 20.0,
    "h": 11.44,
    "text": "\u5f55\u50cf",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 8.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "mode_pro",
    "x": 88.5,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_pro_control",
    "x": 88.5,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_pro_label",
    "x": 93.5,
    "y": 294.94,
    "w": 20.0,
    "h": 11.616,
    "text": "\u4e13\u4e1a",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 8.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "mode_more",
    "x": 114.65,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_more_control",
    "x": 114.65,
    "y": 288.3,
    "w": 26.0,
    "h": 28.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_more_label",
    "x": 119.65,
    "y": 295.012,
    "w": 20.0,
    "h": 11.463999999999999,
    "text": "\u66f4\u591a",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 8.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "mode_dot",
    "x": 99.5,
    "y": 309.25,
    "w": 4.0,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "thumbnail",
    "x": 26.6,
    "y": 334.9,
    "w": 22.0,
    "h": 22.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "thumbnail_control",
    "x": 26.6,
    "y": 334.9,
    "w": 22.0,
    "h": 22.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "thumbnail_ring",
    "x": 26.6,
    "y": 334.9,
    "w": 22.0,
    "h": 22.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "thumbnail_img",
    "x": 27.35,
    "y": 335.65,
    "w": 20.5,
    "h": 20.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "thumbnail_line",
    "x": 28.1,
    "y": 345.4,
    "w": 19.0,
    "h": 1.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter",
    "x": 85.5,
    "y": 329.85,
    "w": 32.0,
    "h": 32.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "shutter_control",
    "x": 85.5,
    "y": 329.85,
    "w": 32.0,
    "h": 32.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "shutter_ring",
    "x": 85.5,
    "y": 329.85,
    "w": 32.0,
    "h": 32.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_gap",
    "x": 87.0,
    "y": 331.35,
    "w": 29.0,
    "h": 29.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_disc",
    "x": 89.5,
    "y": 333.85,
    "w": 24.0,
    "h": 24.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pro_video",
    "x": 154.4,
    "y": 334.9,
    "w": 22.0,
    "h": 22.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_video_control",
    "x": 154.4,
    "y": 334.9,
    "w": 22.0,
    "h": 22.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "pro_video_bg",
    "x": 154.4,
    "y": 334.9,
    "w": 22.0,
    "h": 22.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "pro_video_icon",
    "x": 159.4,
    "y": 339.9,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/pro_video_icon.svg",
      "sha256": "fcd18d5cf4b85f8c467b8adfc55deb27bc5566f687ea7cc710e5bbb196430231",
      "method": "reference_svg",
      "reference_sha256": "a4e18f07968c8fd3fe41aff1a6160f4b0921606f774dc4e484653e0f0848c034",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "treasure_handle",
    "x": 86.45,
    "y": 362.3,
    "w": 30.0,
    "h": 16.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "treasure_handle_control",
    "x": 86.45,
    "y": 362.3,
    "w": 30.0,
    "h": 16.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "treasure_handle_icon",
    "x": 95.5,
    "y": 364.3,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/treasure_handle_icon.svg",
      "sha256": "ba2aab369353e7c3ac04967d8f527f7f0ae0b97ccbe5e79ab43017835c9fccbb",
      "method": "reference_svg",
      "reference_sha256": "a4e18f07968c8fd3fe41aff1a6160f4b0921606f774dc4e484653e0f0848c034",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  }
]
```
