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
    "h": 360.5,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "rec_pill",
    "x": 76.5,
    "y": 3.0,
    "w": 50.0,
    "h": 14.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "rec_dot",
    "x": 82.5,
    "y": 7.5,
    "w": 5.0,
    "h": 5.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
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
    "id": "flash",
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
    "id": "flash_control",
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
    "id": "flash_icon",
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
      "path": "assets/flash_icon.svg",
      "sha256": "5211f51b665e15c3bfee8ce90c3b06e1f48e558fb05c03814f4ac48c2ca59fae",
      "method": "reference_svg",
      "reference_sha256": "2a29e00345c2342050fb887f4686fdb33676c9bab44aac0ab2c0877d9ca36190",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
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
    "id": "rec_still",
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
    "id": "rec_still_control",
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
    "id": "rec_still_bg",
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
    "id": "rec_still_icon",
    "x": 31.6,
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
      "path": "assets/rec_still_icon.svg",
      "sha256": "b8ad79c82876fe97e02090d1330cb0c6f4fd861f8d47e39ea0154c2f44f45ac5",
      "method": "reference_svg",
      "reference_sha256": "2a29e00345c2342050fb887f4686fdb33676c9bab44aac0ab2c0877d9ca36190",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
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
    "id": "shutter_stop",
    "x": 95.0,
    "y": 339.35,
    "w": 13.0,
    "h": 13.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "rec_pause",
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
    "id": "rec_pause_control",
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
    "id": "rec_pause_bg",
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
    "id": "rec_pause_icon",
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
      "path": "assets/rec_pause_icon.svg",
      "sha256": "e2e8edc5d1f50b1c93ce0fadb6bb581e0c37d04f68f8930731c77d1495456c7e",
      "method": "reference_svg",
      "reference_sha256": "2a29e00345c2342050fb887f4686fdb33676c9bab44aac0ab2c0877d9ca36190",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "rec_timer",
    "x": 90.5,
    "y": 5.802,
    "w": 22.046,
    "h": 9.347999999999999,
    "text": "00:07",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 7.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  }
]
```
