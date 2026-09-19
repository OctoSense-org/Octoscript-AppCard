# Mate 70 Air Camera — UX map

Source: HUAWEI Mate 70 Air (SUP-AL90, HarmonyOS 6.1.0.135, API 24, display
1320 × 2760 px, 3.25 px/vp → 406 vp wide), camera app `com.huawei.hmos.camera`
6.1.6.152, captured over `hdc` on 18 Sep 2026 (`snapshot_display` screenshots +
`uitest dumpLayout` trees; capture tooling and raw captures in the session
scratchpad `mate/`). Every number below is measured from the layout dumps and
expressed in the **artboard**: the phone's vp grid with the 42.5 vp status bar
and the 30.8 vp home-indicator strip removed, i.e. exactly the pipeline's
406 × 776 logical artboard (`y_art = (y_px − 138) / 3.25`, `x_art = x_px / 3.25`).

## 1. Screen anatomy (portrait)

| Region | y (vp) | Notes |
| --- | --- | --- |
| Tab bar (top chrome) | 0 – 36 | black; left button + right pill, contents depend on mode |
| Preview 4:3 | 39.4 – 580.3 | 406 × 541; Photo, Portrait, Night, Snapshot, Pro, Super macro, High-res, Aperture, Light painting, Panorama |
| Preview 16:9 | 39.4 – 760.6 | 406 × 721; Video, Slow-mo, Time-lapse (chrome overlays the preview) |
| Param bar (mode extras) | 513.5 – 561.5 | 60 × 48 items at x = 0 (left) and x = 340 (right): icon 20 at top, 12 pt label below |
| Quick zoom bar | 497.5 – 577.5 | 40-tall pill centred at x = 203; items 40 × 40: `W 1x 2x 3x 5x` (5 items, 200 wide), `1x 2x 3x` (120), `1x 2x` (80), `1x 2x 4x` (pano), `W 1x` (front) |
| Mode bar | 576.6 – 632.6 | horizontal list, 52.3 vp pitch, selected label always centred at x = 203, 16 pt, selected bold white, others white, edge items fade to grey; red dot 8 vp at (199, 618.5) under the selected label |
| Foot bar | 647.7 – 719.7 | thumbnail 44 ⌀ at (53, 670); shutter 64 ⌀ at (171, 660); right button 44 ⌀ at (309, 670) |
| Treasure-box handle | 724.6 – 756.6 | 60 × 32 chevron-up (`︿`) centred at x = 203 |

Pro mode inserts a 60-tall parameter strip at y 501.8 – 561.8 and pushes the
zoom bar up to 421.5 – 501.5.

Colours (sampled): chrome black `#000000`; pills `#111111`–`#1f1f1f`
(right pill `#111`, left round button `#151515`, resolution pill `#272727`);
selected zoom chip `#dcdcdc` with `#4f4f4f` text; zoom pill = black at 30 %
over the preview; mode red dot `#e2362c`; video record disc `#ea3323`;
treasure box `#202020` with `#333` icon circles; popup panels = black at
≈45 % with 24 vp corners; filter menu `#2f3034`; selected segment chips pure
white with black glyphs.

Fonts: HarmonyOS Sans. Mode labels 16 pt (bold when selected); zoom chips
13 pt bold; param labels 12 pt; tab-bar text 14 pt; panel titles 16 pt;
segment labels 13 pt; toasts 12 pt; intro card title 24 pt bold, body 12 pt.

## 2. Modes

Mode bar order (rear camera): **夜景 · 闪拍 · 人像 · 拍照 · 录像 · 专业 · 更多**.
Tap a label or swipe the bar horizontally to change mode; the list scrolls so
the selected label sits at the centre. Entering a More sub-mode replaces the
whole bar with a pill `<mode name> ×` (90 × 30 at (158, 590), 4-char names
104 wide) whose × returns to 拍照 (`COMPONENT_ID_MORE_MODE_BACK`). The first
visit to 专业 / 延时摄影 / 超清全景 (and other rich modes) shows an intro card
(§6) that must be dismissed.

| Mode | Tab bar left | Tab bar right pill | Param bar | Zoom | Shutter | Right foot button |
| --- | --- | --- | --- | --- | --- | --- |
| 拍照 Photo | AI 构图 toggle (24, 8, 20 ⌀ glyph in 36 ⌀ round button) | flash · live photo · filter chip `原色` | (low light: `夜景增强` right, with a `4s` badge) | W 1x 2x 3x 5x | white ring + white disc | switch camera |
| 夜景 Night | AI 构图 | filter (film icon only) | `S 自动` left, `超级夜景` right | W…5x | dotted ring + white disc | switch camera |
| 闪拍 Snapshot | — | flash · live photo · film | — | W…5x | white | switch camera |
| 人像 Portrait | AI 构图 | filter chip `原色`; ⓘ at (362, 65) | `美肤 5` left, `F2.4` right (front camera: `美颜 开/关` left, `F16` right) | 1x 2x 3x (front: W 1x) | white | switch camera |
| 录像 Video | `1080p │ 30 fps` pill | stabilisation · flash · film (front: flash only) | `美肤 0` left | W…5x (front: W 1x) | white ring + red disc | switch camera |
| 专业 Pro | `JPG` pill | flash · film | pro strip `M ISO S EV· AF· WB·` | W…5x (raised) | white | **video-camera icon** (switches to Pro video) |
| 更多 More | — | — | grid panel over the preview (§5) | — | — | — |
| 超级微距 Super macro | — | flash · live · film | `AF 自动` left | W…5x | white | video-camera icon |
| 高像素 High-res | — | flash · live · film; toast “AI 超清适合拍摄光照充足的静物和风景，拍摄时请持稳手机” | — | (hidden) | dotted ring | (none) |
| 慢动作 Slow-mo | `1080p │ 240 fps` | flash · film | `8x` right | 1x 2x | video (red disc) | switch camera |
| 大光圈 Aperture | — | film | `F2.4` right | 1x 2x 3x | white | video-camera icon |
| 延时摄影 Time-lapse | `1080p │ 30 fps` | film; ⓘ + exposure ☼ on the preview's right edge | `自动` right (sliders glyph) | W…5x | dotted ring + red disc | switch camera |
| 流光快门 Light painting | — | film | strip of 4 sample tiles: 车水马龙 · 光绘涂鸦 · 丝绢流水 · 绚丽星轨 (84.6 sq at y 498.5, selected has a white frame) | — | dotted ring | (none) |
| 超清全景 Panorama | — | — ; ⓘ | `横向` right (direction) + guide overlay (§7) | 1x 2x 4x | white | (none) |

Shutter glyphs: *white* = 3 vp white ring, 5 vp gap, 48 ⌀ white disc;
*dotted* = 24 white dots on the ring instead of the solid ring; *video* =
white ring, white disc, 26 ⌀ red centre; *time-lapse* = dotted ring + red centre.

## 3. Top chrome controls

- **Flash** (`TAB_BAR_FLASH`, 20 ⌀ glyph at (229, 8)): tap opens a transient
  panel (16, 52) 374 × 104: title 闪光灯 (28, 70), segmented control (28, 94.5)
  350 × 50 with four 87-wide options `⚡A 自动 · ⚡̸ 关闭 · ⚡ 打开 · ☼ 常亮`;
  the selected one is a white 84 × 46 chip with black glyph and 13 pt label.
  Tapping an option applies it, closes the panel and updates the tab-bar
  glyph (A / off / on / torch). The panel auto-dismisses after ~1 s of no
  interaction and on any outside tap.
- **Live photo** (`TAB_BAR_LIVE_PHOTO_TAB_BAR`, (281, 8)): toggle; on → glyph
  becomes the concentric “live” rings, toast `动态照片已开启` for ~1.5 s at the
  top of the preview; off → crossed glyph.
- **Filter chip** (`TAB_BAR_CUSTOM_FILTER_CARD`, (333, 0) 49 × 36): a white
  1 vp rounded outline with a 16 vp film glyph and the current style name
  (10.5 pt). Tap → a menu at (186, 44) 204 × 416 (`#2f3034`, 16 vp corners)
  listing 8 styles in 50.5-tall rows: 原色 ✓ · 鲜艳 · 明快 · 黑白 (film glyph)
  and 自然 · 胶片 · 电影 · 动漫 (petal glyph); 20 vp glyph at x 202, 16 pt label
  at x 230, 24 vp check at x 350 on the selected row, 1 vp dividers.
  Selecting a row closes the menu and rewrites the chip text.
- **AI 辅助构图** (left round button, photo/night/portrait): toggle; on → the
  glyph loses its slash and a 2-line toast `AI 辅助构图已开启，可在 1x、2x 焦段中生效，风光、建筑、人像类场景推荐构图概率更高` shows.
- **Video resolution / frame rate** (`1080p │ 30 fps` pill, x 12 – ~115):
  tap either half → panel (16, 52) 374 × 190: 视频分辨率 segments
  `4K 16:9 · 1080p 全屏 · 1080p 16:9 (selected) · 720p 16:9` (two-line 13 pt
  labels, 87 × 50 each) and 视频帧率 segments `30 fps (selected) · 60 fps`
  (170 × 50). Selection rewrites the pill.
- **Stabilisation** (video, (277, 8) 25 × 20 hand glyph): toggle.
- **JPG** (pro, image 28 × 20 at (24, 8)): tap cycles the output format
  (JPG / RAW+JPG …).
- **ⓘ** (362, 65.5, 20 ⌀) in Portrait / Pro / Time-lapse / Panorama: opens
  the mode's intro card (§6).

## 4. Preview gestures

- Tap: focus brackets 80 × 80 (`COMPONENT_ID_PREVIEW_FOCUSBOX_1`) at the
  point, a ☼ exposure handle 20 vp to the left of the box, and a 2 × 64
  exposure track on the right edge (x 398). Dragging vertically on the track
  or ☼ changes EV. First time: a tip card `开启相机智控，可通过长触保持曝光和对焦 / 去开启`.
- Long press: AE/AF lock (needs 相机智控); Pro mode also long-presses
  EV·/AF·/WB· labels to lock (hint `长按 EV·、AF·、WB· 带点图标可锁定参数`).
- Pinch: continuous zoom; the quick-zoom bar shows the live value.
- Long press on the zoom bar: roulette dial (`DefaultIndex-RouletteZoomView`).
- Horizontal swipe anywhere on the preview: previous / next mode.
- Swipe up from the handle: treasure box (§5). Swipe down: close it.

## 5. Panels

**Treasure box** (`DefaultTreasureBox`): sheet (16, 586.5) 374 × 204,
`#202020`, 24 vp corners, chevron-down handle 60 × 32 at (173, 586.5).
While open the mode bar hides, the zoom bar hides, and the foot bar shrinks
and rises: thumbnail 40 ⌀ at (77, 533), shutter 52 ⌀ at (177, 527), switch
40 ⌀ at (289, 533). Content is a 2-page swiper (indicator at y 758.5: 20 × 6
white pill = current, 6 ⌀ grey dot = other), 4 × 2 grid of 85.5 × 72 cells
(x 32 / 117.5 / 203 / 288.5, y 622.5 / 694.5): 40 ⌀ `#333` circle with a
20 vp glyph, 11 pt label at y + 48.

- Page 1: 设置 · 小艺视觉 · XMAGE 风格 · 曝光 / AI 辅助构图 · 水印 · 照片比例 (`4:3`) · 闪光灯
- Page 2: 动态照片 · 参考线

设置 opens the full settings page (Navigation push); 曝光 / 照片比例 / 闪光灯 /
XMAGE open the same segmented panels as the tab bar; AI 辅助构图 / 动态照片 /
参考线 / 水印 are toggles (glyph loses its slash when on).

**More** (更多 in the mode bar): a translucent panel over the preview
(black ≈45 %): pencil (edit) button 32 × 32 at (29, 249), ⓘ at (345, 249);
3-column grid at (30, 293) with 107 × 77 cells (columns 30 / 149.5 / 268.5,
rows 293 / 382 / 471): 24 vp glyph at (+41.6, +15.7), 14 pt label at +45.6.
Order: 超级微距 · 高像素 · 慢动作 / 大光圈 · 延时摄影 · 流光快门 / 超清全景.
The mode bar shows 拍照 录像 专业 **更多**. Tapping a cell enters that sub-mode.

**More – edit** (pencil): panel gains `×` (29, 249), hint `拖动图标可调整布局`
centred, reset ↺ (293, 249) and ✓ (345, 249); cells get a `#ffffff33` pill
background and can be dragged; the mode bar becomes a row of five 62 × 28
chips 闪拍 · 人像 · 拍照 · 录像 · 专业 (y 582.5, x 27 / 101 / 174 / 248 / 322) —
the ones on the main bar are white, the rest grey (drag a More item onto the
bar to promote it).

## 6. Intro card (`MODE_DETAIL_VERTICAL_CARD`)

Shown on the first entry to a mode and from ⓘ: a 328 × 542 image card at
(39, 111) with 24 vp corners over a dimmed preview; close button 28 ⌀ at
(331, 119) (black 50 %, white ×); bottom caption 328 × 92 at (39, 561):
title 24 pt bold at (55, 573), `更多介绍` pill 64 × 26 at (287, 576), 2-line
12 pt description at (55, 609). Back or × dismisses it.

## 7. Mode-specific overlays

- **Rulers** share one layout in the 80-tall strip replacing the zoom bar
  (y 497.5): a 14 pt title centred at y ≈ 504, end/mid labels above, ticks
  every 1/24 of the 332-wide track (long every third), the current value a
  3 × 20 red marker. Auto-capable rulers (ISO, night shutter, time-lapse,
  macro focus) add a 32 ⌀ red `A` chip at the left (x 50) that is filled when
  auto is active. Pro ISO shows the ruler above the pro strip (y 413.5,
  labels `50 · 500 · 4000 · 409600`) and a red dot under the active label.
  Slow-mo's rate ruler shows the value large (`8x`, 52 pt) with `240 帧/秒`
  under it; time-lapse adds three 48 × 48 option buttons (interval / auto /
  speed) at y 603 in place of the mode bar; super macro's focus ruler has
  near/far glyphs at its ends.
- **Treasure-box second level** (照片比例, 曝光, XMAGE 风格): the sheet keeps
  its size; a header row at y 590.5 with back `‹` (20, 40 ⌀), the title
  centred (16 pt) and close `×` (346); content below: 照片比例 = three 40 ⌀
  option circles `4:3 · 1:1 · 全屏` (selected white) at y 664.6 with labels;
  曝光 = a −4 … +4 ruler at y 648.6 and an EV readout `0.0` with mini ticks
  that appears in the top pill; XMAGE = three dials (色温 / 饱和度 / 明暗)
  with back / reset / close.
- **小艺视觉** opens a separate full-screen scanner page: close `×` (16, 1.5)
  and settings `⋯` (350, 1.5), tip `扫一扫 / 支持识别二维码/动植物/商品` at y 57,
  a `轻触照亮` torch chip, a 64 ⌀ capture button at (171, 661.5), a gallery
  button at (31, 669.5) and a bottom tab row `识文 · 翻译 · 扫一扫 · 扫描`.
- **Portrait 美肤 / 美颜**: tapping `美肤 5` replaces the zoom bar with a
  ruler (0.3, 497.5) 406 × 80: title 美肤 centred at y 500 (14 pt bold), end
  labels `0 · 5 · 10` at y 520 (x 40 / 203 / 366), 25 ticks between x 40 and
  366 (every third tick tall), the current value a 3 × 20 red marker; drag
  to set 0–10. On the front camera the control is a toggle `美颜 开/关` which
  opens the dialog 美颜状态: two sample photos (关 / 开 radio buttons) and 确定.
- **Portrait F2.4**: aperture ruler (same layout, F0.95 … F16).
- **Night `S 自动`**: shutter-time ruler (自动 … 30 s); `超级夜景` cycles the
  night sub-mode.
- **Slow-mo `8x`**: choose 4x / 8x / 32x … (rewrites the fps pill).
- **Time-lapse `自动`**: interval control (自动 / manual sliders).
- **Panorama**: guide at the top of the preview — a 122 × 80 translucent
  band at (142, 63) with a bright frame and ◀ ▶ arrows, a 36 ⌀ target circle
  at (185, 292) that turns yellow while capturing; `横向` toggles the sweep
  direction (horizontal / vertical) and swaps the guide to a 60 × 358 vertical
  band at (24, 63).
- **Low light in Photo**: `夜景增强` badge at (346, 457) with the exposure
  seconds (`4s`) — automatic long exposure suggestion.

## 8. Capture

- Photo shutter: press → the disc flashes, the thumbnail (44 ⌀) animates to
  the new frame. In low light (`夜景增强` shown) the shot is a long exposure:
  the chrome hides, `正在改善拍摄画质，请持稳您的设备` at the top, a large
  countdown `4.1s` in the centre, and the shutter becomes a dotted ring
  around a grey rounded square (cancel) until the exposure ends.
- Video shutter: press → recording state (`COMPONENT_ID_SHUTTER_VIDEO_END_1`):
  the resolution pill is replaced by `● 00:03` (77 × 30 at (165, 3), 20 pt),
  the right pill keeps only the flash, the mode bar hides (the handle stays),
  the zoom bar stays; foot bar: **pause `‖` on the left** (44 ⌀ outlined),
  the shutter ring with a red rounded square, and a **white disc on the
  right** that takes a still while recording. Pause greys the dot and stops
  the timer; the shutter ends the recording and the thumbnail updates.
- Switch camera (44 ⌀, camera-with-arrows glyph): flips the sensor; the
  preview cross-fades. Front camera: mirror preview; Portrait keeps `F16` and
  `美颜 开`; Video keeps `1080p │ 30 fps`, flash becomes screen-fill (`⚡A`).
- Thumbnail: opens the last capture in the gallery viewer (system app),
  Back returns to the same camera state.

## 9. State model (what the replica implements)

```
mode ∈ {night, snapshot, portrait, photo, video, pro, more,
        supermacro, highres, slowmo, aperture, timelapse, lightpainting, panorama}
camera ∈ {rear, front}
flash ∈ {auto, off, on, torch}         livePhoto, aiComposition, stabilisation, grid, watermark: bool
filter ∈ {原色, 鲜艳, 明快, 黑白, 自然, 胶片, 电影, 动漫}
zoom ∈ per-mode set, plus continuous value from pinch / roulette
beauty 0..10 (rear), beautyOn (front), aperture F0.95..F16, nightShutter, slowmoRate ∈ {4x, 8x, 32x}
videoRes ∈ {4K 16:9, 1080p 全屏, 1080p 16:9, 720p 16:9}, videoFps ∈ {30, 60}
pro: metering ∈ {matrix, centre, spot}, iso, shutter, ev, af ∈ {AF-S, AF-C, MF}, wb; locks per EV/AF/WB
overlay ∈ {none, flashPanel, filterMenu, resPanel, treasureBox(page), morePanel, moreEdit,
           introCard(mode), beautyRuler, apertureRuler, nightShutterRuler, focus(x, y), toast(text),
           settings, beautyDialog, galleryViewer}
recording: {off, running(t), paused(t)}
```

Transitions: mode-bar tap/swipe → `mode`, clears overlay, resets zoom to the
mode default (1x); overlays are mutually exclusive; Back closes the top-most
overlay, then leaves a More sub-mode, then exits the app.
