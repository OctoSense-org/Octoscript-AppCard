#!/usr/bin/env python3
"""The Camera storyboard: the Mate 70 Air camera app's screens as one logical
layout on the 406×776 artboard, measured from the phone (docs/ux-map.md).
The same node list renders the atlas (HTML → Chromium) and authors the native
contracts, so reference and contract agree by construction. Coordinates are
the phone's vp grid minus the status bar (see the UX map).

Scenes: text carries both locales; the atlas is rendered in Chinese, the
device's language and the contract's source language.
"""
from fontTools.ttLib import TTFont
from fontTools.pens.boundsPen import BoundsPen
from pathlib import Path
import html as html_mod

ROOT = Path(__file__).resolve().parents[1]
KIT_FONTS = ROOT.parents[2] / 'octoscript-makepad/apps/kit-host/resources/service'
FONTS = {400: ROOT / 'fonts/NotoSansSC-Regular.ttf', 500: KIT_FONTS / 'NotoSansSC-Medium.ttf', 700: KIT_FONTS / 'NotoSansSC-Bold.ttf'}
FONT_SRC = {400: 'self:resources/service/NotoSansSC-Regular.ttf', 500: 'self:resources/service/NotoSansSC-Medium.ttf', 700: 'self:resources/service/NotoSansSC-Bold.ttf'}
ARTBOARD = (406, 776)

# Palette (sampled from the device, docs/ux-map.md §1)
BLACK = '000000'; WHITE = 'ffffff'; PILL = '111111'; PILL2 = '1f1f1f'; ROUND = '151515'; RESPILL = '272727'
PREVIEW = '3a3a3c'; PREVIEW2 = '2c2c2e'          # the viewfinder placeholder (a live camera texture at runtime)
CHIP = 'dcdcdc'; CHIP_TEXT = '4f4f4f'; ZOOM_PILL = '0000004d'; GREY = '727272'; RED = 'e2362c'; REC = 'ea3323'
BOX = '202020'; BOX_CIRCLE = '333333'; PANEL = '00000073'; PANEL_SEG = 'ffffff26'; MENU = '2f3034'; DIVIDER = '46474b'
PRO_BAR = '00000080'; INK_DIM = 'c8c8c8'; DOT = '808080'; TOAST = '00000080'; BLUE = '4a90ff'

# Icons: 28×28 stroked paths (stroke 1.8, round caps); `status` is 56×28.
SVG = {
 'ai_compose': '<rect x="8" y="6" width="14" height="14" rx="2"/><path d="M6 24l16-16M7 21l1.5 2.5M4.5 23.5l2.5-2.5"/><path d="M8 24l1 2 1-2 2-1-2-1-1-2-1 2-2 1z" fill="currentColor"/>',
 'ai_compose_on': '<rect x="8" y="6" width="14" height="14" rx="2"/><path d="M8 24l1 2 1-2 2-1-2-1-1-2-1 2-2 1z" fill="currentColor"/><path d="M12 10h6v6"/>',
 'flash_auto': '<path d="M13 4 7 16h5l-1 8 7-12h-5l1-8z" fill="currentColor"/><path d="M18 24l3-8 3 8M19.2 21.5h3.6" stroke-width="1.6"/>',
 'flash_off': '<path d="M15 4 9 16h5l-1 8 7-12h-5l1-8z"/><path d="M6 6l16 16"/>',
 'flash_on': '<path d="M15 4 9 16h5l-1 8 7-12h-5l1-8z" fill="currentColor"/>',
 'flash_torch': '<path d="M9 13a5 5 0 0 1 10 0c0 3-2 3-2 6h-6c0-3-2-3-2-6z"/><path d="M12 22h4M12.5 25h3M14 3v3M5 13H2M26 13h-3M6.2 5.2l2.1 2.1M21.8 5.2l-2.1 2.1"/>',
 'live_off': '<circle cx="14" cy="14" r="5"/><circle cx="14" cy="14" r="9" stroke-dasharray="1.4 2.2"/><path d="M6 6l16 16"/>',
 'live_on': '<circle cx="14" cy="14" r="5"/><circle cx="14" cy="14" r="9" stroke-dasharray="1.4 2.2"/>',
 'film': '<rect x="3" y="7" width="22" height="14" rx="2"/><path d="M3 11h22M3 17h22M7 7v4M12 7v4M17 7v4M22 7v4M7 17v4M12 17v4M17 17v4M22 17v4"/>',
 'film_solid': '<rect x="3" y="7" width="22" height="14" rx="2" fill="currentColor"/><path d="M3 11h22M3 17h22" stroke="#111"/>',
 'petal': '<circle cx="14" cy="14" r="2.5"/><path d="M14 4v6M14 18v6M4 14h6M18 14h6M7 7l4 4M17 17l4 4M21 7l-4 4M7 21l4-4"/>',
 'switch_camera': '<path d="M9 9h3l2-3h4l2 3h3a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2z" fill="currentColor"/><circle cx="16" cy="15" r="3" fill="#111"/><path d="M5 22c1 3 5 4 9 4M23 22c-1 3-5 4-9 4M12 27l2-1-2-1"/>',
 'video_camera': '<rect x="3" y="8" width="15" height="12" rx="3" fill="currentColor"/><path d="M18 12l7-4v12l-7-4z" fill="currentColor"/>',
 'beauty': '<circle cx="14" cy="15" r="8"/><path d="M8 12c2-4 10-4 12 0M11 16h.5M16.5 16h.5M11 19c2 1.5 4 1.5 6 0"/><path d="M22 5l1 2 2 1-2 1-1 2-1-2-2-1 2-1z" fill="currentColor"/>',
 'aperture': '<circle cx="14" cy="14" r="10"/><path d="M14 4v10l8.7 5M14 14l-8.7 5M5.3 9l8.7 5 8.7-5"/>',
 'aperture_solid': '<circle cx="14" cy="14" r="10" fill="currentColor"/><circle cx="14" cy="14" r="4" fill="#111"/><path d="M14 4v6M14 18v6M5.3 9l5.2 3M17.5 16l5.2 3M22.7 9l-5.2 3M10.5 16l-5.2 3" stroke="#111"/>',
 'info': '<circle cx="14" cy="14" r="10"/><path d="M14 12v7M14 8.5v.5"/>',
 'moon': '<path d="M16 4a10 10 0 1 0 8 14 8 8 0 0 1-8-14z" fill="currentColor"/><path d="M21 5l1 2 2 1-2 1-1 2-1-2-2-1 2-1z" fill="currentColor"/>',
 'shutter_s': '<path d="M18 8c-1-2-8-2-8 2s8 2 8 6-7 4-8 2" stroke-width="2.2"/>',
 'gear': '<circle cx="14" cy="14" r="3.5"/><path d="M14 4v3m0 14v3M4 14h3m14 0h3M6.9 6.9l2.1 2.1m10 10 2.1 2.1M21.1 6.9 19 9m-10 10-2.1 2.1"/>',
 'vision': '<path d="M6 11V8a2 2 0 0 1 2-2h3M17 6h3a2 2 0 0 1 2 2v3M22 17v3a2 2 0 0 1-2 2h-3M11 22H8a2 2 0 0 1-2-2v-3"/><circle cx="14" cy="14" r="4"/><path d="M8 4l.8 1.6L10.4 6.4 8.8 7.2 8 8.8 7.2 7.2 5.6 6.4 7.2 5.6z" fill="currentColor"/>',
 'exposure': '<circle cx="14" cy="14" r="6"/><path d="M14 8a6 6 0 0 1 0 12z" fill="currentColor"/><path d="M14 3v3M14 22v3M3 14h3M22 14h3M6.2 6.2l2.1 2.1M19.7 19.7l2.1 2.1M21.8 6.2l-2.1 2.1M8.3 19.7l-2.1 2.1"/>',
 'sun': '<circle cx="14" cy="14" r="5"/><path d="M14 3v3M14 22v3M3 14h3M22 14h3M6.2 6.2l2.1 2.1M19.7 19.7l2.1 2.1M21.8 6.2l-2.1 2.1M8.3 19.7l-2.1 2.1"/>',
 'watermark': '<circle cx="14" cy="9" r="4"/><path d="M10 12l-2 6h12l-2-6M6 18v4h16v-4"/>',
 'ratio': '<rect x="4" y="5" width="20" height="18" rx="3"/>',
 'grid': '<path d="M10 4v20M18 4v20M4 10h20M4 18h20M6 6l16 16"/>',
 'macro': '<path d="M14 14c-4 0-5-4-5-7 3 0 5 2 5 7zM14 14c4 0 5-4 5-7-3 0-5 2-5 7zM14 14v10M14 24c-3 0-6-2-6-5 3 0 6 2 6 5zM14 24c3 0 6-2 6-5-3 0-6 2-6 5z"/>',
 'highres': '<circle cx="13" cy="13" r="8"/><path d="M19 19l5 5M10 10h2v2h-2zM14 10h2v2h-2zM10 14h2v2h-2zM14 14h2v2h-2z" fill="currentColor"/>',
 'slowmo': '<path d="M14 4a10 10 0 1 1-8.7 5" /><circle cx="14" cy="14" r="2.5" fill="currentColor"/><circle cx="10" cy="14" r="2.5" fill="currentColor" opacity=".6"/><circle cx="6.5" cy="14" r="2.5" fill="currentColor" opacity=".3"/><path d="M6.2 6.2l1 1M4 10l1.2.6" stroke-dasharray="1 2"/>',
 'timelapse': '<circle cx="14" cy="14" r="10"/><path d="M14 14l3-5M5 9l1 .5M4.5 13h1M5 17l1-.5" stroke-dasharray="1.2 2"/><circle cx="14" cy="14" r="1.4" fill="currentColor"/>',
 'lightpaint': '<path d="M4 11h10M6 15h8M8 19h6"/><path d="M20 6l1.2 3 3 1.2-3 1.2-1.2 3-1.2-3-3-1.2 3-1.2z" fill="currentColor"/><path d="M22 18l.7 1.6 1.6.7-1.6.7-.7 1.6-.7-1.6-1.6-.7 1.6-.7z" fill="currentColor"/>',
 'panorama': '<path d="M4 6c6 2 14 2 20 0v16c-6-2-14-2-20 0z"/><path d="M6 18l5-5 4 4 3-3 4 4"/><circle cx="18" cy="10" r="1.5" fill="currentColor"/>',
 'chevron_up': '<path d="m6 18 8-8 8 8"/>',
 'chevron_down': '<path d="m6 10 8 8 8-8"/>',
 'check': '<path d="m6 14 6 6L23 8"/>',
 'close': '<path d="m7 7 14 14M21 7 7 21"/>',
 'pencil': '<path d="M5 23l1-5L18 6l4 4L10 22z"/><path d="M15 9l4 4"/>',
 'reset': '<path d="M6 14a8 8 0 1 0 3-6.2M6 5v4h4"/>',
 'stabilise': '<path d="M10 24V12a2 2 0 0 1 4 0v-3a2 2 0 0 1 4 0v3a2 2 0 0 1 4 0v8a6 6 0 0 1-6 6h-1a5 5 0 0 1-5-5zM10 15l-3-3a2 2 0 0 1 3-3"/><path d="M5 5l18 18"/>',
 'sliders': '<path d="M4 9h20M4 19h20"/><circle cx="10" cy="9" r="2.5" fill="#111"/><circle cx="18" cy="19" r="2.5" fill="#111"/>',
 'night_enhance': '<path d="M16 4a10 10 0 1 0 8 14 8 8 0 0 1-8-14z" fill="currentColor"/>',
 'camera': '<path d="M9 9h3l2-3h4l2 3h3a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2z"/><circle cx="14" cy="15" r="3"/>',
 'pause': '<path d="M10 7v14M18 7v14" stroke-width="3"/>',
 'metering': '<rect x="4" y="8" width="20" height="12" rx="2"/><path d="M10 11l-2 3 2 3M18 11l2 3-2 3"/>',
 'back': '<path d="m17 4-10 10 10 10"/>',
 'arrow_left': '<path d="M24 14H8M13 8l-6 6 6 6" stroke-width="2.4"/>',
 'arrow_right': '<path d="M4 14h16M15 8l6 6-6 6" stroke-width="2.4"/>',
 'status': '<path d="M2 17v4m4-8v8m4-12v12m4-16v16M19 10q6-6 12 0m-9 3q3-3 6 0m-4 3h2M36 8h15v11H36zm17 3v5"/>',
}

_fonts = {}
def font(weight):
    if weight not in _fonts: _fonts[weight] = TTFont(str(FONTS[weight]))
    return _fonts[weight]

def metrics(weight, text):
    """Ink box (em), advance (em), ascent, descent of `text` in the bundled font."""
    f = font(weight); glyphs = f.getGlyphSet(); cmap = f.getBestCmap(); units = f['head'].unitsPerEm
    cursor = 0; bounds = []
    for ch in text:
        name = cmap.get(ord(ch), '.notdef'); pen = BoundsPen(glyphs); glyphs[name].draw(pen)
        if pen.bounds:
            x0, y0, x1, y1 = pen.bounds; bounds.append((x0 + cursor, y0, x1 + cursor, y1))
        cursor += f['hmtx'].metrics[name][0]
    box = [min(b[0] for b in bounds), min(b[1] for b in bounds), max(b[2] for b in bounds), max(b[3] for b in bounds)]
    return [v / units for v in box], cursor / units, f['hhea'].ascent / units, f['hhea'].descent / units

class Scene:
    def __init__(self, n, id, title_cn, title_en, surface='app'):
        self.n = n; self.id = id; self.title = {'cn': title_cn, 'en': title_en}; self.surface = surface
        self.nodes = []; self.controls = {}; self.cards = []
        self.stack('screen', 0, 0, 406, 776, bg=BLACK, parent='page')
    def stack(self, id, x, y, w, h, bg=None, radius=0, border=None, parent='screen'):
        self.nodes.append(dict(kind='stack', id=id, x=x, y=y, w=w, h=h, bg=bg, radius=radius, border=border, parent=parent)); return id
    def text(self, id, cn, en, x, y, w, h, size, weight=400, color=WHITE, align='left', parent='screen'):
        self.nodes.append(dict(kind='text', id=id, cn=cn, en=en, x=x, y=y, w=w, h=h, size=size, weight=weight, color=color, align=align, parent=parent)); return id
    def icon(self, id, kind, x, y, w, h, color=WHITE, parent='screen'):
        self.nodes.append(dict(kind='icon', id=id, icon=kind, x=x, y=y, w=w, h=h, color=color, parent=parent)); return id
    def control(self, id, x, y, w, h, enabled=True, parent='screen', label=None):
        """A tappable area; children drawn with parent=id belong to it."""
        self.stack(id, x, y, w, h, parent=parent)
        self.nodes.append(dict(kind='control', id=id + '_control', x=x, y=y, w=w, h=h, enabled=enabled, parent=id))
        self.controls[id] = dict(event=id, text_ids=[label] if label else [], source_bounds=[x, y, w, h], enabled=enabled)
        return id
    def node(self, id):
        return next(n for n in self.nodes if n['id'] == id)
    # ---- camera chrome --------------------------------------------------------------
    def preview(self, ratio='4:3', dim=False):
        h = 541 if ratio == '4:3' else 721
        self.stack('viewfinder', 0, 39.4, 406, h, bg=PREVIEW2 if dim else PREVIEW)
        return 39.4 + h
    def round_button(self, id, kind, cx, cy, d=36, bg=ROUND, icon=20, color=WHITE, parent='screen'):
        self.control(id, cx - d / 2, cy - d / 2, d, d, parent=parent)
        self.stack(id + '_bg', cx - d / 2, cy - d / 2, d, d, bg=bg, radius=d / 2, parent=id)
        self.icon(id + '_icon', kind, cx - icon / 2, cy - icon / 2, icon, icon, color, parent=id)
        return id
    def top_left_round(self, id, kind):
        return self.round_button(id, kind, 34, 18)
    def top_pill(self, items, x0=None, w=None, chip=None):
        """The right pill: [(id, icon)] 20-px glyphs spaced 52 apart; optional filter chip (id, text)."""
        n = len(items) + (1 if chip else 0)
        w = w or (52 * len(items) + (78 if chip else 0) + 20)
        x0 = 394 - w if x0 is None else x0
        self.stack('top_pill', x0, 0, w, 36, bg=PILL, radius=18)
        x = x0 + 10
        for id, kind in items:
            self.control(id, x, 0, 52, 36)
            self.icon(id + '_icon', kind, x + 16, 8, 20, 20, parent=id)
            x += 52
        if chip:
            id, cn = chip
            self.control(id, x, 0, 68, 36, label=id + '_label')
            self.stack(id + '_frame', x + 4, 5, 60, 26, bg=None, radius=13, border=WHITE, parent=id)
            self.icon(id + '_icon', 'film', x + 10, 8, 20, 20, parent=id)
            self.text(id + '_label', cn, cn, x + 31, 5, 30, 26, 10.5, 700, WHITE, 'left', parent=id)
    def res_pill(self, res='1080p', fps='30 fps'):
        self.stack('res_pill', 12, 0, 104, 36, bg=RESPILL, radius=18)
        self.control('video_res', 12, 0, 56, 36, label='video_res_label')
        self.text('video_res_label', res, res, 24, 0, 42, 36, 13, 500, parent='video_res')
        self.stack('res_sep', 68, 10, 1, 16, bg=GREY)
        self.control('video_fps', 70, 0, 46, 36, label='video_fps_label')
        self.text('video_fps_label', fps, fps, 76, 0, 40, 36, 13, 500, parent='video_fps')
    def zoom_bar(self, items, selected, y=517.5):
        w = 40 * len(items); x0 = 203 - w / 2
        self.stack('zoom_pill', x0, y, w, 40, bg=ZOOM_PILL, radius=20)
        for i, label in enumerate(items):
            id = f'zoom_{label.replace("x", "").lower()}'
            x = x0 + i * 40
            self.control(id, x, y, 40, 40, label=id + '_label')
            if label == selected:
                self.stack(id + '_chip', x, y, 40, 40, bg=CHIP, radius=20, parent=id)
            self.text(id + '_label', label, label, x, y, 40, 40, 13, 700, CHIP_TEXT if label == selected else WHITE, 'center', parent=id)
    def param(self, id, kind, cn, en, side='right', y=513.5):
        x = 340 if side == 'right' else 0
        self.control(id, x, y, 60, 48, label=id + '_label')
        self.icon(id + '_icon', kind, x + 20, y + 6, 20, 20, parent=id)
        self.text(id + '_label', cn, en, x, y + 28, 60, 20, 12, 500, WHITE, 'center', parent=id)
    def mode_bar(self, modes, selected):
        """Labels 52.3 apart, the selected one centred at x 203; edge labels fade."""
        i0 = [m[0] for m in modes].index(selected)
        for i, (id, cn, en) in enumerate(modes):
            cx = 203 + (i - i0) * 52.3
            if cx < 0 or cx > 406: continue
            x = cx - 26
            self.control('mode_' + id, x, 576.6, 52, 56, label='mode_' + id + '_label')
            dist = abs(i - i0)
            color = WHITE if dist <= 2 else (GREY if dist == 3 else '3c3c3c')
            self.text('mode_' + id + '_label', cn, en, x, 588.6, 52, 24, 16, 700 if i == i0 else 500, color, 'center', parent='mode_' + id)
        self.stack('mode_dot', 199, 618.5, 8, 8, bg=RED, radius=4)
    def sub_mode_pill(self, cn, en, four=False):
        w = 104 if four else 90; x = 203 - w / 2
        self.control('sub_mode_close', x, 589.8, w, 30, label='sub_mode_label')
        self.stack('sub_mode_bg', x, 589.8, w, 30, bg=PILL2, radius=15, border='ffffff66', parent='sub_mode_close')
        self.text('sub_mode_label', cn, en, x + 13, 596.6, w - 45, 16.3, 14, 700, parent='sub_mode_close')
        self.icon('sub_mode_x', 'close', x + w - 29, 596.9, 16, 16, parent='sub_mode_close')
    def shutter(self, kind='photo', cx=203, cy=691.7, d=64):
        r = d / 2
        self.control('shutter', cx - r, cy - r, d, d)
        if kind in ('dotted', 'timelapse'):
            import math
            for k in range(24):
                a = 2 * math.pi * k / 24
                self.stack(f'shutter_dot{k}', cx + (r - 1.5) * math.cos(a) - 1.5, cy + (r - 1.5) * math.sin(a) - 1.5, 3, 3, bg=WHITE, radius=1.5, parent='shutter')
        else:
            self.stack('shutter_ring', cx - r, cy - r, d, d, bg=WHITE, radius=r, parent='shutter')
            self.stack('shutter_gap', cx - r + 3, cy - r + 3, d - 6, d - 6, bg=BLACK, radius=r - 3, parent='shutter')
        inner = d - 16
        self.stack('shutter_disc', cx - inner / 2, cy - inner / 2, inner, inner, bg=WHITE, radius=inner / 2, parent='shutter')
        if kind in ('video', 'timelapse'):
            self.stack('shutter_rec', cx - 13, cy - 13, 26, 26, bg=REC, radius=13, parent='shutter')
    def foot(self, shutter='photo', right='switch', thumb=(53.2, 669.8, 44), shutter_pos=(203, 691.7, 64), right_pos=(330.8, 691.8, 44)):
        x, y, d = thumb
        self.control('thumbnail', x, y, d, d)
        self.stack('thumbnail_ring', x, y, d, d, bg=WHITE, radius=d / 2, parent='thumbnail')
        self.stack('thumbnail_img', x + 1.5, y + 1.5, d - 3, d - 3, bg='b9b7b1', radius=d / 2 - 1.5, parent='thumbnail')
        self.stack('thumbnail_line', x + 3, y + d / 2 - 1, d - 6, 2, bg='2a2a2a', parent='thumbnail')
        cx, cy, sd = shutter_pos
        self.shutter(shutter, cx, cy, sd)
        if right:
            rx, ry, rd = right_pos
            id = 'switch_camera' if right == 'switch' else 'pro_video'
            self.control(id, rx - rd / 2, ry - rd / 2, rd, rd)
            self.stack(id + '_bg', rx - rd / 2, ry - rd / 2, rd, rd, bg='131313', radius=rd / 2, border=WHITE, parent=id)
            self.icon(id + '_icon', 'switch_camera' if right == 'switch' else 'video_camera', rx - 12, ry - 12, 24, 24, parent=id)
    def handle(self, up=True, y=724.6):
        self.control('treasure_handle', 172.9, y, 60, 32)
        self.icon('treasure_handle_icon', 'chevron_up' if up else 'chevron_down', 191, y + 4, 24, 24, INK_DIM, parent='treasure_handle')
    def info_button(self):
        self.control('info', 352, 55.5, 40, 40)
        self.icon('info_icon', 'info', 362, 65.5, 20, 20, parent='info')
    def segment(self, id, x, y, w, h, options, selected, two_line=False, parent='screen'):
        """A segmented control: translucent track, white chip on the selected option."""
        self.stack(id + '_track', x, y, w, h, bg=PANEL_SEG, radius=h / 2, parent=parent)
        ow = w / len(options)
        for i, (oid, icon, cn, en) in enumerate(options):
            ox = x + i * ow
            self.control(oid, ox, y, ow, h, label=oid + '_label', parent=parent)
            sel = oid == selected
            if sel:
                self.stack(oid + '_chip', ox + 2, y + 2, ow - 4, h - 4, bg=WHITE, radius=(h - 4) / 2, parent=oid)
            color = BLACK if sel else WHITE
            if icon:
                self.icon(oid + '_icon', icon, ox + ow / 2 - 10, y + 6, 20, 20, color, parent=oid)
                self.text(oid + '_label', cn, en, ox, y + 29, ow, 16, 13, 500, color, 'center', parent=oid)
            elif two_line:
                a, b = cn.split('\n'); ae, be = en.split('\n')
                self.text(oid + '_label', a, ae, ox, y + 8, ow, 18, 13, 500, color, 'center', parent=oid)
                self.text(oid + '_label2', b, be, ox, y + 26, ow, 18, 13, 500, color, 'center', parent=oid)
            else:
                self.text(oid + '_label', cn, en, ox, y, ow, h, 14, 500, color, 'center', parent=oid)

MODES = [('night', '夜景', 'Night'), ('snapshot', '闪拍', 'Snapshot'), ('portrait', '人像', 'Portrait'), ('photo', '拍照', 'Photo'), ('video', '录像', 'Video'), ('pro', '专业', 'Pro'), ('more', '更多', 'More')]

def storyboard():
    scenes = []
    # 01 · Photo ---------------------------------------------------------------------
    s = Scene(1, 'camera-01', '拍照', 'Photo'); scenes.append(s)
    s.preview('4:3')
    s.top_left_round('ai_compose', 'ai_compose')
    s.top_pill([('flash', 'flash_off'), ('live_photo', 'live_off')], chip=('filter', '原色'))
    s.zoom_bar(['W', '1x', '2x', '3x', '5x'], '1x')
    s.mode_bar(MODES, 'photo')
    s.foot('photo', 'switch')
    s.handle()
    # 02 · Flash panel -----------------------------------------------------------------
    s = Scene(2, 'camera-02', '闪光灯', 'Flash'); scenes.append(s)
    s.preview('4:3')
    s.top_left_round('ai_compose', 'ai_compose')
    s.top_pill([('flash', 'flash_off'), ('live_photo', 'live_off')], chip=('filter', '原色'))
    s.stack('flash_panel', 16, 52, 374, 104, bg=PANEL, radius=24); s.cards.append('flash_panel')
    s.text('flash_title', '闪光灯', 'Flash', 28, 66, 200, 24, 16, 500, parent='flash_panel')
    s.segment('flash_seg', 28, 94.5, 350, 50, [('flash_auto', 'flash_auto', '自动', 'Auto'), ('flash_off_opt', 'flash_off', '关闭', 'Off'), ('flash_on', 'flash_on', '打开', 'On'), ('flash_torch', 'flash_torch', '常亮', 'Torch')], 'flash_off_opt', parent='flash_panel')
    s.zoom_bar(['W', '1x', '2x', '3x', '5x'], '1x')
    s.mode_bar(MODES, 'photo')
    s.foot('photo', 'switch')
    s.handle()
    # 03 · Filter menu -----------------------------------------------------------------
    s = Scene(3, 'camera-03', '色彩风格', 'Color style'); scenes.append(s)
    s.preview('4:3')
    s.top_left_round('ai_compose', 'ai_compose')
    s.top_pill([('flash', 'flash_off'), ('live_photo', 'live_off')], chip=('filter', '原色'))
    s.stack('filter_menu', 186, 44, 204, 416, bg=MENU, radius=16)
    styles = [('style_original', 'film', '原色', 'Original'), ('style_vivid', 'film_solid', '鲜艳', 'Vivid'), ('style_bright', 'film_solid', '明快', 'Bright'), ('style_mono', 'film', '黑白', 'Mono'),
              ('style_natural', 'petal', '自然', 'Natural'), ('style_filmic', 'petal', '胶片', 'Film'), ('style_cinema', 'petal', '电影', 'Cinema'), ('style_anime', 'petal', '动漫', 'Anime')]
    for i, (id, icon, cn, en) in enumerate(styles):
        y = 48 + i * 50.5
        s.control(id, 190, y, 196, 50.5, label=id + '_label')
        s.icon(id + '_icon', icon, 202, y + 15.4, 20, 20, parent=id)
        s.text(id + '_label', cn, en, 230, y + 14, 112, 22, 16, 400, parent=id)
        if i == 0: s.icon(id + '_check', 'check', 350, y + 13.2, 24, 24, parent=id)
        if i < 7: s.stack(id + '_line', 202, y + 50, 176, 1, bg=DIVIDER)
    s.zoom_bar(['W', '1x', '2x', '3x', '5x'], '1x')
    s.mode_bar(MODES, 'photo')
    s.foot('photo', 'switch')
    s.handle()
    # 04 · Portrait --------------------------------------------------------------------
    s = Scene(4, 'camera-04', '人像', 'Portrait'); scenes.append(s)
    s.preview('4:3')
    s.top_left_round('ai_compose', 'ai_compose')
    s.top_pill([], chip=('filter', '原色'))
    s.info_button()
    s.param('beauty', 'beauty', '美肤 5', 'Skin 5', 'left')
    s.param('aperture', 'aperture_solid', 'F2.4', 'F2.4', 'right')
    s.zoom_bar(['1x', '2x', '3x'], '1x')
    s.mode_bar(MODES, 'portrait')
    s.foot('photo', 'switch')
    s.handle()
    # 05 · Video -----------------------------------------------------------------------
    s = Scene(5, 'camera-05', '录像', 'Video'); scenes.append(s)
    s.preview('16:9')
    s.res_pill('1080p', '30 fps')
    s.top_pill([('stabilise', 'stabilise'), ('flash', 'flash_off'), ('filter', 'film')])
    s.param('beauty', 'beauty', '美肤 0', 'Skin 0', 'left')
    s.zoom_bar(['W', '1x', '2x', '3x', '5x'], '1x')
    s.mode_bar(MODES, 'video')
    s.foot('video', 'switch')
    s.handle()
    # 06 · Recording -------------------------------------------------------------------
    s = Scene(6, 'camera-06', '录像中', 'Recording'); scenes.append(s)
    s.preview('16:9')
    s.stack('rec_pill', 153, 6, 100, 28, bg=TOAST, radius=14)
    s.stack('rec_dot', 165, 15, 10, 10, bg=REC, radius=5)
    s.text('rec_timer', '00:07', '00:07', 181, 6, 64, 28, 14, 500)
    s.top_pill([('flash', 'flash_off')])
    s.zoom_bar(['W', '1x', '2x', '3x', '5x'], '1x')
    s.control('rec_still', 53.2, 669.8, 44, 44)
    s.stack('rec_still_bg', 53.2, 669.8, 44, 44, bg='131313', radius=22, border=WHITE, parent='rec_still')
    s.icon('rec_still_icon', 'camera', 63.2, 679.8, 24, 24, parent='rec_still')
    s.control('shutter', 171, 659.7, 64, 64)
    s.stack('shutter_ring', 171, 659.7, 64, 64, bg=WHITE, radius=32, parent='shutter')
    s.stack('shutter_gap', 174, 662.7, 58, 58, bg=BLACK, radius=29, parent='shutter')
    s.stack('shutter_stop', 190, 678.7, 26, 26, bg=REC, radius=6, parent='shutter')
    s.control('rec_pause', 308.8, 669.8, 44, 44)
    s.stack('rec_pause_bg', 308.8, 669.8, 44, 44, bg='131313', radius=22, border=WHITE, parent='rec_pause')
    s.icon('rec_pause_icon', 'pause', 318.8, 679.8, 24, 24, parent='rec_pause')
    # 07 · Pro -------------------------------------------------------------------------
    s = Scene(7, 'camera-07', '专业', 'Pro'); scenes.append(s)
    s.preview('4:3')
    s.control('pro_format', 12, 0, 52, 36, label='pro_format_label')
    s.stack('pro_format_bg', 12, 0, 52, 36, bg=ROUND, radius=18, parent='pro_format')
    s.text('pro_format_label', 'JPG', 'JPG', 12, 0, 52, 36, 13, 700, WHITE, 'center', parent='pro_format')
    s.top_pill([('flash', 'flash_off'), ('filter', 'film')])
    s.info_button()
    s.control('pro_exposure_effect', 352, 99.2, 40, 40)
    s.icon('pro_exposure_effect_icon', 'sun', 362, 109.2, 20, 20, parent='pro_exposure_effect')
    s.zoom_bar(['W', '1x', '2x', '3x', '5x'], '1x', y=441.5)
    s.stack('pro_bar', 0, 501.8, 406, 60, bg=PRO_BAR)
    pro = [('pro_metering', 'M', None, 'metering'), ('pro_iso', 'ISO', '50', None), ('pro_shutter', 'S', '1/125', None), ('pro_ev', 'EV•', '0', None), ('pro_af', 'AF•', 'AF-C', None), ('pro_wb', 'WB•', 'AWB', None)]
    for i, (id, label, value, icon) in enumerate(pro):
        x = 12 + i * 66.7
        s.control(id, x, 506.8, 48, 50.2, label=id + '_label')
        s.text(id + '_label', label, label, x, 508.8, 48, 20, 15, 700, WHITE, 'center', parent=id)
        if value: s.text(id + '_value', value, value, x, 529.8, 48, 16, 12, 500, WHITE, 'center', parent=id)
        if icon: s.icon(id + '_icon', icon, x + 16, 530.8, 16, 16, parent=id)
        if id == 'pro_wb': s.stack(id + '_underline', x + 10, 546.8, 28, 1.5, bg=WHITE, parent=id)
    s.mode_bar(MODES, 'pro')
    s.foot('photo', 'video')
    s.handle()
    # 08 · Night -----------------------------------------------------------------------
    s = Scene(8, 'camera-08', '夜景', 'Night'); scenes.append(s)
    s.preview('4:3', dim=True)
    s.top_left_round('ai_compose', 'ai_compose')
    s.top_pill([('filter', 'film')])
    s.info_button()
    s.param('night_shutter', 'shutter_s', '自动', 'Auto', 'left')
    s.node('night_shutter').update(x=6.2); s.node('night_shutter_icon').update(x=26.2); s.node('night_shutter_label').update(x=6.2)
    s.param('night_sub_mode', 'moon', '超级夜景', 'Super night', 'right')
    s.zoom_bar(['W', '1x', '2x', '3x', '5x'], '1x')
    s.mode_bar(MODES, 'night')
    s.foot('dotted', 'switch')
    s.handle()
    # 09 · More ------------------------------------------------------------------------
    s = Scene(9, 'camera-09', '更多', 'More'); scenes.append(s)
    s.preview('4:3', dim=True)
    s.stack('more_panel', 16, 236, 374, 334, bg='000000a0', radius=24)
    s.control('more_edit', 28.9, 248.9, 32, 32)
    s.icon('more_edit_icon', 'pencil', 32.9, 252.9, 24, 24, parent='more_edit')
    s.control('more_info', 345.1, 248.9, 32, 32)
    s.icon('more_info_icon', 'info', 349.1, 252.9, 24, 24, parent='more_info')
    more = [('supermacro', 'macro', '超级微距', 'Super macro'), ('highres', 'highres', '高像素', 'High-res'), ('slowmo', 'slowmo', '慢动作', 'Slow-mo'),
            ('aperture_mode', 'aperture', '大光圈', 'Aperture'), ('timelapse', 'timelapse', '延时摄影', 'Time-lapse'), ('lightpaint', 'lightpaint', '流光快门', 'Light painting'), ('panorama', 'panorama', '超清全景', 'Panorama')]
    for i, (id, icon, cn, en) in enumerate(more):
        r, c = divmod(i, 3); x = 30.4 + c * 119; y = 292.9 + r * 89
        s.control('more_' + id, x, y, 107, 77, label='more_' + id + '_label')
        s.icon('more_' + id + '_icon', icon, x + 41.5, y + 15.7, 24, 24, parent='more_' + id)
        s.text('more_' + id + '_label', cn, en, x, y + 45.6, 107, 18, 14, 500, WHITE, 'center', parent='more_' + id)
    s.mode_bar(MODES, 'more')
    s.foot('photo', 'switch')
    s.handle()
    # 10 · Treasure box ----------------------------------------------------------------
    s = Scene(10, 'camera-10', '百宝箱', 'Toolbox'); scenes.append(s)
    s.preview('4:3')
    s.top_left_round('ai_compose', 'ai_compose')
    s.top_pill([('flash', 'flash_off'), ('live_photo', 'live_off')], chip=('filter', '原色'))
    s.foot('photo', 'switch', thumb=(77, 533, 40), shutter_pos=(203, 552.8, 52), right_pos=(309, 553, 40))
    s.stack('box', 16, 586.5, 374, 204, bg=BOX, radius=24); s.cards.append('box')
    s.handle(up=False, y=586.5)
    box = [('box_settings', 'gear', '设置', 'Settings'), ('box_vision', 'vision', '小艺视觉', 'Celia vision'), ('box_xmage', 'film', 'XMAGE 风格', 'XMAGE style'), ('box_exposure', 'exposure', '曝光', 'Exposure'),
           ('box_ai', 'ai_compose', 'AI 辅助构图', 'AI composition'), ('box_watermark', 'watermark', '水印', 'Watermark'), ('box_ratio', 'ratio', '照片比例', 'Aspect ratio'), ('box_flash', 'flash_off', '闪光灯', 'Flash')]
    for i, (id, icon, cn, en) in enumerate(box):
        r, c = divmod(i, 4); x = 32 + c * 85.5; y = 622.5 + r * 72
        s.control(id, x, y, 85.5, 72, label=id + '_label', parent='box')
        s.stack(id + '_circle', x + 22.75, y + 4, 40, 40, bg=BOX_CIRCLE, radius=20, parent=id)
        s.icon(id + '_icon', icon, x + 32.75, y + 14, 20, 20, parent=id)
        if id == 'box_ratio': s.text(id + '_glyph', '4:3', '4:3', x + 32.75, y + 16, 20, 16, 7, 700, WHITE, 'center', parent=id)
        s.text(id + '_label', cn, en, x - 6, y + 48, 97.5, 16, 11, 400, WHITE, 'center', parent=id)
    s.stack('box_page_active', 183, 761.5, 20, 6, bg=WHITE, radius=3, parent='box')
    s.stack('box_page_other', 213, 761.5, 6, 6, bg=DOT, radius=3, parent='box')
    # 11 · Beauty ruler ----------------------------------------------------------------
    s = Scene(11, 'camera-11', '美肤', 'Skin smoothing'); scenes.append(s)
    s.preview('4:3')
    s.top_left_round('ai_compose', 'ai_compose')
    s.top_pill([], chip=('filter', '原色'))
    s.info_button()
    s.stack('ruler_bar', 0, 497.5, 406, 80, bg=PRO_BAR)
    s.text('ruler_title', '美肤', 'Skin', 153, 498, 100, 20, 14, 700, WHITE, 'center')
    for id, label, x in [('ruler_min', '0', 30), ('ruler_mid', '5', 193), ('ruler_max', '10', 356)]:
        s.text(id, label, label, x, 517, 20, 16, 12, 700, WHITE, 'center')
    for k in range(25):
        x = 40 + k * (326 / 24); tall = k % 3 == 0
        if k == 12:
            s.stack('ruler_marker', x - 1.5, 538, 3, 20, bg=REC, radius=1.5)
        else:
            s.stack(f'ruler_tick{k}', x - 0.75, 545 if tall else 549, 1.5, 14 if tall else 8, bg=WHITE if tall else INK_DIM, radius=0.75)
    s.control('ruler', 23, 494.8, 360, 80)
    s.mode_bar(MODES, 'portrait')
    s.foot('photo', 'switch')
    s.handle()
    # 12 · Panorama --------------------------------------------------------------------
    s = Scene(12, 'camera-12', '超清全景', 'Panorama'); scenes.append(s)
    s.preview('4:3')
    s.info_button()
    s.stack('pano_band', 40, 63.4, 328, 80, bg='ffffff26')
    s.stack('pano_frame', 142, 63.4, 122, 80, bg=None, border=WHITE)
    s.icon('pano_arrow_l', 'arrow_left', 92, 89.4, 28, 28)
    s.icon('pano_arrow_r', 'arrow_right', 288, 89.4, 28, 28)
    s.stack('pano_line_l', 60, 103, 32, 1.5, bg=WHITE); s.stack('pano_line_r', 316, 103, 32, 1.5, bg=WHITE)
    s.stack('pano_target', 185, 292, 36, 36, bg=None, radius=18, border=WHITE)
    s.stack('pano_target_in', 194, 301, 18, 18, bg=WHITE, radius=9)
    s.param('pano_dir', 'exposure', '横向', 'Horizontal', 'right')
    s.node('pano_dir_icon').update(icon='sun')
    s.zoom_bar(['1x', '2x', '4x'], '1x')
    s.sub_mode_pill('超清全景', 'Panorama', four=True)
    s.foot('photo', None)
    s.handle()
    return scenes

# --- HTML render of the storyboard ---------------------------------------------
def css_color(v):
    v = v.lstrip('#')
    if len(v) == 8:
        return f'rgba({int(v[0:2], 16)},{int(v[2:4], 16)},{int(v[4:6], 16)},{int(v[6:8], 16) / 255:.3f})'
    return '#' + v

def scene_html(s, interactive=False):
    parts = []; controls = []
    for n in s.nodes:
        x, y, w, h = n['x'], n['y'], n['w'], n['h']
        base = f'left:{x}px;top:{y}px;width:{w}px;height:{h}px;'
        tag = (lambda attr: f' data-node="{n["id"]}"{attr}') if interactive else (lambda attr: '')
        if n['kind'] == 'control':
            if interactive: controls.append(f'<div class="c" data-control="{n["id"][:-8]}" style="{base}"></div>')
            continue
        if n['kind'] == 'stack':
            if not n['bg'] and not n['border'] and not interactive: continue
            st = base + (f'background:{css_color(n["bg"])};' if n['bg'] else '') + f'border-radius:{n["radius"]}px;' + (f'box-shadow:inset 0 0 0 1px {css_color(n["border"])};' if n['border'] else '')
            parts.append(f'<div class="s"{tag("")} style="{st}"></div>')
        elif n['kind'] == 'text':
            st = base + f'line-height:{h}px;font-size:{n["size"]}px;font-weight:{n["weight"]};color:{css_color(n["color"])};text-align:{n["align"]};'
            parts.append(f'<div class="t"{tag(f" data-text={chr(34)}{n[chr(105)+chr(100)]}{chr(34)}")} style="{st}">{html_mod.escape(n["cn"])}</div>')
        elif n['kind'] == 'icon':
            vb = '0 0 56 28' if n['icon'] == 'status' else '0 0 28 28'
            parts.append(f'<svg class="i"{tag("")} style="{base};color:{css_color(n["color"])}" viewBox="{vb}" fill="none" stroke="{css_color(n["color"])}" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">{SVG[n["icon"]]}</svg>')
    return ''.join(parts + controls)

GUTTER = 48; CAPTION = 34; COLS = 4
def atlas_layout(scenes):
    w, h = ARTBOARD[0] * 2, ARTBOARD[1] * 2; crops = []
    for i, _ in enumerate(scenes):
        r, c = divmod(i, COLS)
        crops.append([GUTTER + c * (w + GUTTER), GUTTER + CAPTION + r * (h + GUTTER + CAPTION), w, h])
    rows = (len(scenes) + COLS - 1) // COLS
    return crops, [COLS * w + (COLS + 1) * GUTTER, rows * h + (rows + 1) * GUTTER + rows * CAPTION]

def atlas_html(scenes):
    crops, (aw, ah) = atlas_layout(scenes)
    faces = ''.join(f'@font-face{{font-family:"Noto Sans SC";src:url("{FONTS[w].resolve().as_uri()}");font-weight:{w}}}\n' for w in (400, 500, 700))
    head = f'''<!doctype html><html><head><meta charset="utf-8"><style>
{faces}html,body{{margin:0;background:#d7dbe2}}
body{{width:{aw // 2}px;height:{ah // 2}px;position:relative;font-family:"Noto Sans SC";-webkit-font-smoothing:antialiased}}
.screen{{position:absolute;width:{ARTBOARD[0]}px;height:{ARTBOARD[1]}px;overflow:hidden;background:#000}}
.s,.t,.i{{position:absolute;box-sizing:border-box}}
.t{{white-space:nowrap;overflow:visible}}
.cap{{position:absolute;font-size:12px;color:#3c4452;line-height:{CAPTION}px}}
</style></head><body>'''
    body = []
    for s, crop in zip(scenes, crops):
        x, y = crop[0] // 2, crop[1] // 2
        body.append(f'<div class="cap" style="left:{x}px;top:{y - CAPTION}px">{s.n:02d} · {html_mod.escape(s.title["cn"])} / {html_mod.escape(s.title["en"])}</div>')
        body.append(f'<div class="screen" style="left:{x}px;top:{y}px">{scene_html(s)}</div>')
    return head + ''.join(body) + '</body></html>', crops, (aw, ah)

if __name__ == '__main__':
    scenes = storyboard()
    for s in scenes:
        ids = [n['id'] for n in s.nodes]
        dup = {i for i in ids if ids.count(i) > 1}
        if dup: raise SystemExit(f'{s.id}: duplicate ids {sorted(dup)}')
    html, crops, size = atlas_html(scenes)
    out = ROOT / 'source/storyboard.html'; out.parent.mkdir(exist_ok=True); out.write_text(html)
    print(len(scenes), 'scenes', size, crops[0], sum(len(s.controls) for s in scenes), 'controls')
