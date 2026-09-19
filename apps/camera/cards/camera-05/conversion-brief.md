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
    "id": "res_pill",
    "x": 6.0,
    "y": 0.0,
    "w": 52.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "video_res",
    "x": 6.0,
    "y": 0.0,
    "w": 28.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "video_res_control",
    "x": 6.0,
    "y": 0.0,
    "w": 28.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "video_res_label",
    "x": 12.0,
    "y": 4.959,
    "w": 22.915,
    "h": 10.3245,
    "text": "1080p",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 6.5,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "res_sep",
    "x": 34.0,
    "y": 5.0,
    "w": 0.5,
    "h": 8.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "video_fps",
    "x": 35.0,
    "y": 0.0,
    "w": 23.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "video_fps_control",
    "x": 35.0,
    "y": 0.0,
    "w": 23.0,
    "h": 18.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "video_fps_label",
    "x": 38.0,
    "y": 4.568999999999999,
    "w": 22.3365,
    "h": 10.714500000000001,
    "text": "30 fps",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 6.5,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "top_pill",
    "x": 109.0,
    "y": 0.0,
    "w": 88.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "stabilise",
    "x": 114.0,
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
    "id": "stabilise_control",
    "x": 114.0,
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
    "id": "stabilise_icon",
    "x": 122.0,
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
      "path": "assets/stabilise_icon.svg",
      "sha256": "a2682155dd3dcabf9a4c23bce4df336ad1ece95a6fd7fddc8e37e80e57636924",
      "method": "reference_svg",
      "reference_sha256": "6ddf7526e96bcb2eda6861410a1cdd00f89a3af849c94511bcea0e11b4ab7777",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
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
      "reference_sha256": "6ddf7526e96bcb2eda6861410a1cdd00f89a3af849c94511bcea0e11b4ab7777",
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
      "reference_sha256": "6ddf7526e96bcb2eda6861410a1cdd00f89a3af849c94511bcea0e11b4ab7777",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "beauty",
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
    "id": "beauty_control",
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
    "id": "beauty_icon",
    "x": 10.0,
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
      "path": "assets/beauty_icon.svg",
      "sha256": "061c17585f21dc471c23d0d3acc34b68b1e04093ab2011d9291a282d0bb1ae58",
      "method": "reference_svg",
      "reference_sha256": "6ddf7526e96bcb2eda6861410a1cdd00f89a3af849c94511bcea0e11b4ab7777",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "beauty_label",
    "x": 6.615,
    "y": 271.272,
    "w": 20.77,
    "h": 9.61,
    "text": "\u7f8e\u80a4 0",
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
    "id": "mode_snapshot",
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
    "id": "mode_snapshot_control",
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
    "id": "mode_snapshot_label",
    "x": 15.050000000000011,
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
    "id": "mode_portrait_control",
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
    "id": "mode_portrait_label",
    "x": 41.2,
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
    "id": "mode_photo_control",
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
    "id": "mode_photo_label",
    "x": 67.35,
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
    "id": "mode_video_control",
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
    "id": "mode_video_label",
    "x": 93.5,
    "y": 294.99600000000004,
    "w": 20.0,
    "h": 11.538,
    "text": "\u5f55\u50cf",
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
    "id": "mode_pro",
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
    "id": "mode_pro_control",
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
    "id": "mode_pro_label",
    "x": 119.65,
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
    "id": "mode_more_control",
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
    "id": "mode_more_label",
    "x": 145.8,
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
    "id": "shutter_rec",
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
      "reference_sha256": "6ddf7526e96bcb2eda6861410a1cdd00f89a3af849c94511bcea0e11b4ab7777",
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
      "reference_sha256": "6ddf7526e96bcb2eda6861410a1cdd00f89a3af849c94511bcea0e11b4ab7777",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  }
]
```
