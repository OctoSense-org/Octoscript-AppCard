Camera — one-atlas storyboard brief (authored render, not a generated image)

This atlas is not the output of an image model. It is a deterministic render of
scripts/design.py: the storyboard HTML (source/storyboard.html) is produced from
the same logical layout that the native contracts are authored from, then
rasterised by headless Chromium at 2× (source/render_atlas.mjs). The intake
freezes the actual PNG bytes; contracts do not read the pixels back.

Subject: the HUAWEI Mate 70 Air camera app (HarmonyOS 6.1, app 6.1.6.152),
whose every screen was captured on the device over hdc (screenshot + uitest
layout dump) on 18 Sep 2026 and measured into the 406×776 artboard (the
phone's 406-vp width; the status bar and the home-indicator strip removed).
docs/ux-map.md records the geometry, colours, controls and interaction logic.

Scenes (all portrait, rear camera unless noted):
01 拍照 Photo · 02 闪光灯 flash panel · 03 色彩风格 style menu · 04 人像 Portrait ·
05 录像 Video · 06 录像中 Recording · 07 专业 Pro · 08 夜景 Night · 09 更多 More grid ·
10 百宝箱 toolbox sheet · 11 美肤 skin ruler · 12 超清全景 Panorama.

The viewfinder is a flat placeholder surface in the storyboard; at runtime it
is the live camera texture (Makepad video input), never a raster.
Palette, type and control sizes: see docs/ux-map.md §1. Font: the bundled
Noto Sans SC stands in for HarmonyOS Sans (not redistributable).
