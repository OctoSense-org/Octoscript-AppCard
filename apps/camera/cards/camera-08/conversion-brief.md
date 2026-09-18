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
    "id": "ai_compose",
    "x": 8.0,
    "y": 0.0,
    "w": 18.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "ai_compose_control",
    "x": 8.0,
    "y": 0.0,
    "w": 18.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "ai_compose_bg",
    "x": 8.0,
    "y": 0.0,
    "w": 18.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ai_compose_icon",
    "x": 12.0,
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
      "path": "assets/ai_compose_icon.svg",
      "sha256": "1d3b9e87f8b9032632f7522284e4a565b23f7ecae193757ad790b5249ad10072",
      "method": "reference_svg",
      "reference_sha256": "f460fe3c1f3f361c2c0322a060a72ed7f5b092de39d742c37a29ce71489278a3",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "top_pill",
    "x": 161.0,
    "y": 0.0,
    "w": 36.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
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
      "reference_sha256": "f460fe3c1f3f361c2c0322a060a72ed7f5b092de39d742c37a29ce71489278a3",
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
      "reference_sha256": "f460fe3c1f3f361c2c0322a060a72ed7f5b092de39d742c37a29ce71489278a3",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "night_shutter",
    "x": 3.1,
    "y": 256.75,
    "w": 30.0,
    "h": 24.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "night_shutter_control",
    "x": 0.0,
    "y": 256.75,
    "w": 30.0,
    "h": 24.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "night_shutter_icon",
    "x": 13.1,
    "y": 259.75,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/night_shutter_icon.svg",
      "sha256": "dd15771a2b24e31600b978d4b1de37693b04eea6fc33c711a48666b8c090cbf6",
      "method": "reference_svg",
      "reference_sha256": "f460fe3c1f3f361c2c0322a060a72ed7f5b092de39d742c37a29ce71489278a3",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "night_shutter_label",
    "x": 12.1,
    "y": 271.28999999999996,
    "w": 16.0,
    "h": 9.58,
    "text": "\u81ea\u52a8",
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
    "id": "night_sub_mode",
    "x": 170.0,
    "y": 256.75,
    "w": 30.0,
    "h": 24.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "night_sub_mode_control",
    "x": 170.0,
    "y": 256.75,
    "w": 30.0,
    "h": 24.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "night_sub_mode_icon",
    "x": 180.0,
    "y": 259.75,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/night_sub_mode_icon.svg",
      "sha256": "00dbf45f5204fabbed891f07dbbe0a8e13f55cf277f4d60df3fe57bfd88a1f76",
      "method": "reference_svg",
      "reference_sha256": "f460fe3c1f3f361c2c0322a060a72ed7f5b092de39d742c37a29ce71489278a3",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "night_sub_mode_label",
    "x": 173.0,
    "y": 271.25399999999996,
    "w": 28.0,
    "h": 9.634,
    "text": "\u8d85\u7ea7\u591c\u666f",
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
    "id": "zoom_pill",
    "x": 51.5,
    "y": 258.75,
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
    "y": 258.75,
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
    "y": 258.75,
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
    "y": 264.7675,
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
    "y": 258.75,
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
    "y": 258.75,
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
    "y": 258.75,
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
    "y": 264.7675,
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
    "y": 258.75,
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
    "y": 258.75,
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
    "y": 264.683,
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
    "y": 258.75,
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
    "y": 258.75,
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
    "y": 264.683,
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
    "y": 258.75,
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
    "y": 258.75,
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
    "y": 264.7675,
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
    "id": "mode_night",
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
    "id": "mode_night_control",
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
    "id": "mode_night_label",
    "x": 93.5,
    "y": 294.916,
    "w": 20.0,
    "h": 11.6,
    "text": "\u591c\u666f",
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
    "id": "mode_snapshot",
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
    "id": "mode_snapshot_control",
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
    "id": "mode_snapshot_label",
    "x": 119.65,
    "y": 295.036,
    "w": 20.0,
    "h": 11.424,
    "text": "\u95ea\u62cd",
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
    "id": "mode_portrait",
    "x": 140.8,
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
    "x": 140.8,
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
    "x": 145.8,
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
    "x": 166.95,
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
    "x": 166.95,
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
    "x": 171.95,
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
    "id": "shutter_dot0",
    "x": 116.0,
    "y": 345.1,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot1",
    "x": 115.48036885090829,
    "y": 349.04699043781346,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot2",
    "x": 113.95688740771268,
    "y": 352.725,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot3",
    "x": 111.53337841309485,
    "y": 355.8833784130949,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot4",
    "x": 108.375,
    "y": 358.30688740771274,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot5",
    "x": 104.69699043781344,
    "y": 359.8303688509083,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot6",
    "x": 100.75,
    "y": 360.35,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot7",
    "x": 96.80300956218656,
    "y": 359.8303688509083,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot8",
    "x": 93.125,
    "y": 358.30688740771274,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot9",
    "x": 89.96662158690515,
    "y": 355.8833784130949,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot10",
    "x": 87.54311259228732,
    "y": 352.725,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot11",
    "x": 86.01963114909171,
    "y": 349.04699043781346,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot12",
    "x": 85.5,
    "y": 345.1,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot13",
    "x": 86.01963114909171,
    "y": 341.1530095621866,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot14",
    "x": 87.54311259228732,
    "y": 337.475,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot15",
    "x": 89.96662158690515,
    "y": 334.31662158690517,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot16",
    "x": 93.125,
    "y": 331.89311259228737,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot17",
    "x": 96.80300956218656,
    "y": 330.36963114909173,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot18",
    "x": 100.75,
    "y": 329.85,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot19",
    "x": 104.69699043781344,
    "y": 330.36963114909173,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot20",
    "x": 108.375,
    "y": 331.8931125922873,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot21",
    "x": 111.53337841309485,
    "y": 334.31662158690517,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot22",
    "x": 113.95688740771268,
    "y": 337.475,
    "w": 1.5,
    "h": 1.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_dot23",
    "x": 115.48036885090829,
    "y": 341.1530095621866,
    "w": 1.5,
    "h": 1.5,
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
    "id": "switch_camera",
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
    "id": "switch_camera_control",
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
    "id": "switch_camera_bg",
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
    "id": "switch_camera_icon",
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
      "path": "assets/switch_camera_icon.svg",
      "sha256": "81bb2e5a07e63fd6049f90fbf8a98ede7bdd3b6ff515067fe7bcc17a7a3e1221",
      "method": "reference_svg",
      "reference_sha256": "f460fe3c1f3f361c2c0322a060a72ed7f5b092de39d742c37a29ce71489278a3",
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
      "reference_sha256": "f460fe3c1f3f361c2c0322a060a72ed7f5b092de39d742c37a29ce71489278a3",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  }
]
```
