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
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
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
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
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
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
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
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
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
    "id": "thumbnail",
    "x": 38.5,
    "y": 266.5,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "thumbnail_control",
    "x": 38.5,
    "y": 266.5,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "thumbnail_ring",
    "x": 38.5,
    "y": 266.5,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "thumbnail_img",
    "x": 39.25,
    "y": 267.25,
    "w": 18.5,
    "h": 18.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "thumbnail_line",
    "x": 40.0,
    "y": 276.0,
    "w": 17.0,
    "h": 1.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter",
    "x": 88.5,
    "y": 263.4,
    "w": 26.0,
    "h": 26.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "shutter_control",
    "x": 88.5,
    "y": 263.4,
    "w": 26.0,
    "h": 26.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "shutter_ring",
    "x": 88.5,
    "y": 263.4,
    "w": 26.0,
    "h": 26.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_gap",
    "x": 90.0,
    "y": 264.9,
    "w": 23.0,
    "h": 23.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "shutter_disc",
    "x": 92.5,
    "y": 267.4,
    "w": 18.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "switch_camera",
    "x": 144.5,
    "y": 266.5,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "switch_camera_control",
    "x": 144.5,
    "y": 266.5,
    "w": 20.0,
    "h": 20.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "switch_camera_bg",
    "x": 144.5,
    "y": 266.5,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "switch_camera_icon",
    "x": 148.5,
    "y": 270.5,
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
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box",
    "x": 8.0,
    "y": 293.25,
    "w": 187.0,
    "h": 102.0,
    "role": "card",
    "native_candidates": [
      "View",
      "TaskplanProjectCard",
      "CamoTrackRow"
    ]
  },
  {
    "id": "box_settings",
    "x": 16.0,
    "y": 311.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_settings_control",
    "x": 16.0,
    "y": 311.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_settings_circle",
    "x": 27.375,
    "y": 313.25,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_settings_icon",
    "x": 32.375,
    "y": 318.25,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/box_settings_icon.svg",
      "sha256": "4c72eea96210c3fd68d69f1266fb739d6c021dd63aafdef935ec5e1f5ff3dd78",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box_settings_label",
    "x": 31.875,
    "y": 335.12699999999995,
    "w": 15.0,
    "h": 8.9775,
    "text": "\u8bbe\u7f6e",
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
    "id": "box_vision",
    "x": 58.75,
    "y": 311.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_vision_control",
    "x": 58.75,
    "y": 311.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_vision_circle",
    "x": 70.125,
    "y": 313.25,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_vision_icon",
    "x": 75.125,
    "y": 318.25,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/box_vision_icon.svg",
      "sha256": "41eb0602af4400103ccdd98985f67757b38dbb87df5f36c36b5fc5b8976177eb",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box_vision_label",
    "x": 69.125,
    "y": 335.02799999999996,
    "w": 26.0,
    "h": 9.082,
    "text": "\u5c0f\u827a\u89c6\u89c9",
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
    "id": "box_xmage",
    "x": 101.5,
    "y": 311.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_xmage_control",
    "x": 101.5,
    "y": 311.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_xmage_circle",
    "x": 112.875,
    "y": 313.25,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_xmage_icon",
    "x": 117.875,
    "y": 318.25,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/box_xmage_icon.svg",
      "sha256": "6eee1ff89ed03065ba6ae219e4d934625e55c610c272be5d38274a4f781b3d5b",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box_xmage_label",
    "x": 107.76375,
    "y": 335.0225,
    "w": 34.2225,
    "h": 9.071,
    "text": "XMAGE \u98ce\u683c",
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
    "id": "box_exposure",
    "x": 144.25,
    "y": 311.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_exposure_control",
    "x": 144.25,
    "y": 311.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_exposure_circle",
    "x": 155.625,
    "y": 313.25,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_exposure_icon",
    "x": 160.625,
    "y": 318.25,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/box_exposure_icon.svg",
      "sha256": "ed0158cf3e2557b5241243435b91ae39167457fbe3bf0f031b367f62bde90113",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box_exposure_label",
    "x": 160.125,
    "y": 335.02799999999996,
    "w": 15.0,
    "h": 9.059999999999999,
    "text": "\u66dd\u5149",
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
    "id": "box_ai",
    "x": 16.0,
    "y": 347.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_ai_control",
    "x": 16.0,
    "y": 347.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_ai_circle",
    "x": 27.375,
    "y": 349.25,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_ai_icon",
    "x": 32.375,
    "y": 354.25,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/box_ai_icon.svg",
      "sha256": "1d3b9e87f8b9032632f7522284e4a565b23f7ecae193757ad790b5249ad10072",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box_ai_label",
    "x": 23.28125,
    "y": 371.02799999999996,
    "w": 32.1875,
    "h": 9.071,
    "text": "AI \u8f85\u52a9\u6784\u56fe",
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
    "id": "box_watermark",
    "x": 58.75,
    "y": 347.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_watermark_control",
    "x": 58.75,
    "y": 347.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_watermark_circle",
    "x": 70.125,
    "y": 349.25,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_watermark_icon",
    "x": 75.125,
    "y": 354.25,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/box_watermark_icon.svg",
      "sha256": "cf6bff5bf8bf2acc659e75f53dbca1d02dcde623e20f05ff576c97293c5ec2a7",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box_watermark_label",
    "x": 74.625,
    "y": 371.039,
    "w": 15.0,
    "h": 9.0545,
    "text": "\u6c34\u5370",
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
    "id": "box_ratio",
    "x": 101.5,
    "y": 347.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_ratio_control",
    "x": 101.5,
    "y": 347.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_ratio_circle",
    "x": 112.875,
    "y": 349.25,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_ratio_icon",
    "x": 117.875,
    "y": 354.25,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/box_ratio_icon.svg",
      "sha256": "b6a11884bf0e04da9e54ef35a2cb0808a1d6a75152f3bcee7557048fc60ec9b4",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box_ratio_glyph",
    "x": 120.24125,
    "y": 356.137,
    "w": 9.2675,
    "h": 6.688000000000001,
    "text": "4:3",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 3.5,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "box_ratio_label",
    "x": 111.875,
    "y": 371.03349999999995,
    "w": 26.0,
    "h": 9.082,
    "text": "\u7167\u7247\u6bd4\u4f8b",
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
    "id": "box_flash",
    "x": 144.25,
    "y": 347.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_flash_control",
    "x": 144.25,
    "y": 347.25,
    "w": 42.75,
    "h": 36.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "box_flash_circle",
    "x": 155.625,
    "y": 349.25,
    "w": 20.0,
    "h": 20.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_flash_icon",
    "x": 160.625,
    "y": 354.25,
    "w": 10.0,
    "h": 10.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/box_flash_icon.svg",
      "sha256": "5211f51b665e15c3bfee8ce90c3b06e1f48e558fb05c03814f4ac48c2ca59fae",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "box_flash_label",
    "x": 157.375,
    "y": 371.02799999999996,
    "w": 20.5,
    "h": 9.059999999999999,
    "text": "\u95ea\u5149\u706f",
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
    "id": "box_page_active",
    "x": 91.5,
    "y": 380.75,
    "w": 10.0,
    "h": 3.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "box_page_other",
    "x": 106.5,
    "y": 380.75,
    "w": 3.0,
    "h": 3.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "treasure_handle",
    "x": 86.45,
    "y": 293.25,
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
    "y": 293.25,
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
    "y": 295.25,
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
      "sha256": "875dc18ee517b8c015e93096c88e45ff05c816e17c0f5129cf6005eb848b30ff",
      "method": "reference_svg",
      "reference_sha256": "2cce6696ea76582820aa14221eeb2bd57ffc2f703152dd53c824107270dae7b6",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  }
]
```
