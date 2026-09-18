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
      "reference_sha256": "385bab7c6e6ed3a5f23167f1361ed912ca53e4c5e56621c5e0e95bfa6bab6186",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "top_pill",
    "x": 96.0,
    "y": 0.0,
    "w": 101.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "flash",
    "x": 101.0,
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
    "x": 101.0,
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
    "x": 109.0,
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
      "reference_sha256": "385bab7c6e6ed3a5f23167f1361ed912ca53e4c5e56621c5e0e95bfa6bab6186",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "live_photo",
    "x": 127.0,
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
    "id": "live_photo_control",
    "x": 127.0,
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
    "id": "live_photo_icon",
    "x": 135.0,
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
      "path": "assets/live_photo_icon.svg",
      "sha256": "aef383e35f6c947c95bea76f56b18309dcd686417107a5441c2030383dbb7f0d",
      "method": "reference_svg",
      "reference_sha256": "385bab7c6e6ed3a5f23167f1361ed912ca53e4c5e56621c5e0e95bfa6bab6186",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "filter",
    "x": 153.0,
    "y": 0.0,
    "w": 34.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "filter_control",
    "x": 153.0,
    "y": 0.0,
    "w": 34.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "filter_frame",
    "x": 155.0,
    "y": 2.5,
    "w": 30.0,
    "h": 13.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "filter_icon",
    "x": 158.0,
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
      "reference_sha256": "385bab7c6e6ed3a5f23167f1361ed912ca53e4c5e56621c5e0e95bfa6bab6186",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "filter_label",
    "x": 168.5,
    "y": 4.78975,
    "w": 14.5,
    "h": 8.9665,
    "text": "\u539f\u8272",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 5.25,
    "weight": 700,
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
    "id": "mode_night_control",
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
    "id": "mode_night_label",
    "x": 15.050000000000011,
    "y": 294.97200000000004,
    "w": 20.0,
    "h": 11.504,
    "text": "\u591c\u666f",
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
    "id": "mode_snapshot",
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
    "id": "mode_snapshot_control",
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
    "id": "mode_snapshot_label",
    "x": 41.2,
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
    "id": "mode_portrait_control",
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
    "id": "mode_portrait_label",
    "x": 67.35,
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
    "id": "mode_photo_control",
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
    "id": "mode_photo_label",
    "x": 93.5,
    "y": 294.988,
    "w": 20.0,
    "h": 11.559999999999999,
    "text": "\u62cd\u7167",
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
    "id": "mode_video",
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
    "id": "mode_video_control",
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
    "id": "mode_video_label",
    "x": 119.65,
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
    "id": "mode_pro_control",
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
    "id": "mode_pro_label",
    "x": 145.8,
    "y": 295.004,
    "w": 20.0,
    "h": 11.48,
    "text": "\u4e13\u4e1a",
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
    "id": "mode_more",
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
    "id": "mode_more_control",
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
    "id": "mode_more_label",
    "x": 171.95,
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
      "reference_sha256": "385bab7c6e6ed3a5f23167f1361ed912ca53e4c5e56621c5e0e95bfa6bab6186",
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
      "reference_sha256": "385bab7c6e6ed3a5f23167f1361ed912ca53e4c5e56621c5e0e95bfa6bab6186",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  }
]
```
