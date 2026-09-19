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
    "id": "more_panel",
    "x": 8.0,
    "y": 118.0,
    "w": 187.0,
    "h": 167.0,
    "role": "layout",
    "native_candidates": [
      "View"
    ]
  },
  {
    "id": "more_edit",
    "x": 14.45,
    "y": 124.45,
    "w": 16.0,
    "h": 16.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_edit_control",
    "x": 14.45,
    "y": 124.45,
    "w": 16.0,
    "h": 16.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_edit_icon",
    "x": 16.45,
    "y": 126.45,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_edit_icon.svg",
      "sha256": "1f2212aef7e7046cbdd9c67730fbc865f21f2b8980f2271e3f529e86abaf92d5",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_info",
    "x": 172.55,
    "y": 124.45,
    "w": 16.0,
    "h": 16.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_info_control",
    "x": 172.55,
    "y": 124.45,
    "w": 16.0,
    "h": 16.0,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_info_icon",
    "x": 174.55,
    "y": 126.45,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_info_icon.svg",
      "sha256": "464057134560328fe4b2638b58ae794f65ea3399e6ac34188268b37fd46204e3",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_supermacro",
    "x": 15.2,
    "y": 146.45,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_supermacro_control",
    "x": 15.2,
    "y": 146.45,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_supermacro_icon",
    "x": 35.95,
    "y": 154.29999999999998,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_supermacro_icon.svg",
      "sha256": "2b374aa66274359c87b7f22b0392e853d733d75dbfd25f601080414481bbb6b5",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_supermacro_label",
    "x": 27.95,
    "y": 168.86599999999999,
    "w": 32.0,
    "h": 10.545,
    "text": "\u8d85\u7ea7\u5fae\u8ddd",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 7.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "more_highres",
    "x": 74.7,
    "y": 146.45,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_highres_control",
    "x": 74.7,
    "y": 146.45,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_highres_icon",
    "x": 95.45,
    "y": 154.29999999999998,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_highres_icon.svg",
      "sha256": "14ef0bad9e44a1115b9c39958381f0a007ad437ceab937319d44b7b6bc3afa69",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_highres_label",
    "x": 90.95,
    "y": 168.85899999999998,
    "w": 25.0,
    "h": 10.53275,
    "text": "\u9ad8\u50cf\u7d20",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 7.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "more_slowmo",
    "x": 134.2,
    "y": 146.45,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_slowmo_control",
    "x": 134.2,
    "y": 146.45,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_slowmo_icon",
    "x": 154.95,
    "y": 154.29999999999998,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_slowmo_icon.svg",
      "sha256": "10a47fa0fbf9a957b3339c821104261b2e940b7f1188cb557d985dc9cac0e02d",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_slowmo_label",
    "x": 150.45,
    "y": 168.894,
    "w": 25.0,
    "h": 10.503,
    "text": "\u6162\u52a8\u4f5c",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 7.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "more_aperture_mode",
    "x": 15.2,
    "y": 190.95,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_aperture_mode_control",
    "x": 15.2,
    "y": 190.95,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_aperture_mode_icon",
    "x": 35.95,
    "y": 198.79999999999998,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_aperture_mode_icon.svg",
      "sha256": "7557a879344a4ac03f30a4fd3ee0982d6f4ab0bc151441accbaf5a61bfb324b3",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_aperture_mode_label",
    "x": 31.45,
    "y": 213.394,
    "w": 25.0,
    "h": 10.503,
    "text": "\u5927\u5149\u5708",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 7.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "more_timelapse",
    "x": 74.7,
    "y": 190.95,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_timelapse_control",
    "x": 74.7,
    "y": 190.95,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_timelapse_icon",
    "x": 95.45,
    "y": 198.79999999999998,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_timelapse_icon.svg",
      "sha256": "8883a26fbe706d206e0826b40e58121d2f9d621be36d7b4a90de05eb4665225f",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_timelapse_label",
    "x": 87.45,
    "y": 213.394,
    "w": 32.0,
    "h": 10.545,
    "text": "\u5ef6\u65f6\u6444\u5f71",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 7.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "more_lightpaint",
    "x": 134.2,
    "y": 190.95,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_lightpaint_control",
    "x": 134.2,
    "y": 190.95,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_lightpaint_icon",
    "x": 154.95,
    "y": 198.79999999999998,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_lightpaint_icon.svg",
      "sha256": "85cf6ab74640ad46a2a06cb00bc4f00298f04eeed7ce76e40d197370fac0419d",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_lightpaint_label",
    "x": 146.95,
    "y": 213.36599999999999,
    "w": 32.0,
    "h": 10.559,
    "text": "\u6d41\u5149\u5feb\u95e8",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 7.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "more_panorama",
    "x": 15.2,
    "y": 235.45,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_panorama_control",
    "x": 15.2,
    "y": 235.45,
    "w": 53.5,
    "h": 38.5,
    "role": "button",
    "native_candidates": [
      "Button",
      "KitButton"
    ]
  },
  {
    "id": "more_panorama_icon",
    "x": 35.95,
    "y": 243.29999999999998,
    "w": 12.0,
    "h": 12.0,
    "role": "icon",
    "native_candidates": [
      "Svg",
      "Icon",
      "Image"
    ],
    "asset": {
      "path": "assets/more_panorama_icon.svg",
      "sha256": "40f1ca87fead3a8fd4006365dc47cd733577ba8d227914106af2c4e5cc808f68",
      "method": "reference_svg",
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  },
  {
    "id": "more_panorama_label",
    "x": 27.95,
    "y": 257.817,
    "w": 32.0,
    "h": 10.58,
    "text": "\u8d85\u6e05\u5168\u666f",
    "font_src": "self:resources/service/NotoSansSC-Medium.ttf",
    "size": 7.0,
    "weight": 500,
    "role": "text",
    "native_candidates": [
      "Label",
      "TextFlow"
    ]
  },
  {
    "id": "mode_photo",
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
    "id": "mode_photo_control",
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
    "id": "mode_photo_label",
    "x": 15.050000000000011,
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
    "id": "mode_video_control",
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
    "id": "mode_video_label",
    "x": 41.2,
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
    "id": "mode_pro_control",
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
    "id": "mode_pro_label",
    "x": 67.35,
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
    "id": "mode_more_control",
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
    "id": "mode_more_label",
    "x": 93.5,
    "y": 294.964,
    "w": 20.0,
    "h": 11.559999999999999,
    "text": "\u66f4\u591a",
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
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
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
      "reference_sha256": "ecb0e2a20e1f213ad4ca5489ae4fdca9f08034ef0e6d97af51e936b3b8e1cc45",
      "fit": "stretch",
      "clip": true,
      "notes": "Storyboard icon: the same vector paths the reference was rendered from; no text or raster"
    }
  }
]
```
