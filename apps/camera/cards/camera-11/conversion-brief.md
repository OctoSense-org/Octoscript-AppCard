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
      "reference_sha256": "e438ab15b2d228245724aceae8efec5cc3eb2e8d9bc41de194dd814ce35c656a",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "top_pill",
    "x": 148.0,
    "y": 0.0,
    "w": 49.0,
    "h": 18.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
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
      "reference_sha256": "e438ab15b2d228245724aceae8efec5cc3eb2e8d9bc41de194dd814ce35c656a",
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
      "reference_sha256": "e438ab15b2d228245724aceae8efec5cc3eb2e8d9bc41de194dd814ce35c656a",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "ruler_bar",
    "x": 0.0,
    "y": 248.75,
    "w": 203.0,
    "h": 40.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick0",
    "x": 19.625,
    "y": 272.5,
    "w": 0.75,
    "h": 7.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick1",
    "x": 26.416666666666668,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick2",
    "x": 33.208333333333336,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick3",
    "x": 40.0,
    "y": 272.5,
    "w": 0.75,
    "h": 7.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick4",
    "x": 46.79166666666667,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick5",
    "x": 53.583333333333336,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick6",
    "x": 60.375,
    "y": 272.5,
    "w": 0.75,
    "h": 7.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick7",
    "x": 67.16666666666667,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick8",
    "x": 73.95833333333334,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick9",
    "x": 80.75,
    "y": 272.5,
    "w": 0.75,
    "h": 7.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick10",
    "x": 87.54166666666667,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick11",
    "x": 94.33333333333334,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_marker",
    "x": 100.75,
    "y": 269.0,
    "w": 1.5,
    "h": 10.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick13",
    "x": 107.91666666666667,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick14",
    "x": 114.70833333333334,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick15",
    "x": 121.5,
    "y": 272.5,
    "w": 0.75,
    "h": 7.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick16",
    "x": 128.29166666666669,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick17",
    "x": 135.08333333333334,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick18",
    "x": 141.875,
    "y": 272.5,
    "w": 0.75,
    "h": 7.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick19",
    "x": 148.66666666666669,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick20",
    "x": 155.45833333333334,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick21",
    "x": 162.25,
    "y": 272.5,
    "w": 0.75,
    "h": 7.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick22",
    "x": 169.04166666666669,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick23",
    "x": 175.83333333333334,
    "y": 274.5,
    "w": 0.75,
    "h": 4.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler_tick24",
    "x": 182.625,
    "y": 272.5,
    "w": 0.75,
    "h": 7.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "ruler",
    "x": 11.5,
    "y": 247.4,
    "w": 180.0,
    "h": 40.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "ruler_control",
    "x": 11.5,
    "y": 247.4,
    "w": 180.0,
    "h": 40.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "mode_night",
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
    "id": "mode_night_control",
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
    "id": "mode_night_label",
    "x": 41.2,
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
    "id": "mode_snapshot_control",
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
    "id": "mode_snapshot_label",
    "x": 67.35,
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
    "id": "mode_portrait_control",
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
    "id": "mode_portrait_label",
    "x": 93.5,
    "y": 294.99600000000004,
    "w": 20.0,
    "h": 11.514666666666667,
    "text": "\u4eba\u50cf",
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
    "id": "mode_photo",
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
    "id": "mode_photo_control",
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
    "id": "mode_photo_label",
    "x": 119.65,
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
    "id": "mode_video_control",
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
    "id": "mode_video_label",
    "x": 145.8,
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
    "id": "mode_pro_control",
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
    "id": "mode_pro_label",
    "x": 171.95,
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
      "reference_sha256": "e438ab15b2d228245724aceae8efec5cc3eb2e8d9bc41de194dd814ce35c656a",
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
      "reference_sha256": "e438ab15b2d228245724aceae8efec5cc3eb2e8d9bc41de194dd814ce35c656a",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "ruler_title",
    "x": 94.5,
    "y": 249.05299999999997,
    "w": 18.0,
    "h": 10.643,
    "text": "\u7f8e\u80a4",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 7.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "ruler_min",
    "x": 18.23,
    "y": 258.592,
    "w": 7.54,
    "h": 8.608,
    "text": "0",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "ruler_mid",
    "x": 99.73,
    "y": 258.66999999999996,
    "w": 7.54,
    "h": 8.530000000000001,
    "text": "5",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "ruler_max",
    "x": 179.46,
    "y": 258.592,
    "w": 11.08,
    "h": 8.608,
    "text": "10",
    "font_src": "self:resources/service/NotoSansSC-Bold.ttf",
    "size": 6.0,
    "weight": 700,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  }
]
```
