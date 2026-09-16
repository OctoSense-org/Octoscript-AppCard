#!/usr/bin/env python3
"""The Calendar storyboard: one logical layout that both the atlas render and the
native contracts are derived from, so the measured reference and the authored
tree agree by construction. Coordinates are the 406×776 logical artboard.

Every scene is a list of nodes: stacks (surfaces), single-line texts, buttons
(a stack + label + native control), and vector icons. Text carries both
locales; the atlas is rendered in Chinese, the contract's source language.
"""
from fontTools.ttLib import TTFont
from fontTools.pens.boundsPen import BoundsPen
from pathlib import Path
import html as html_mod

ROOT = Path(__file__).resolve().parents[1]
KIT_FONTS = ROOT.parents[2] / 'octoscript-makepad/apps/kit-host/resources/service'
FONTS = {400: ROOT / 'fonts/NotoSansSC-Regular.ttf', 700: KIT_FONTS / 'NotoSansSC-Bold.ttf'}
FONT_SRC = {400: 'self:resources/service/NotoSansSC-Regular.ttf', 700: 'self:resources/service/NotoSansSC-Bold.ttf'}
ARTBOARD = (406, 776)

# iOS-like palette
INK = '1c1c1e'; GRAY = '8e8e93'; GRAY2 = 'c7c7cc'; LINE = 'e5e5ea'; PAGE = 'ffffff'; GROUP = 'f2f2f7'
RED = 'ff3b30'; BLUE = '0a84ff'; GREEN = '34c759'; ORANGE = 'ff9500'; PURPLE = 'af52de'; WHITE = 'ffffff'
DESK = 'e9edf3'; DESK2 = 'dfe6ef'; CARD = 'ffffff'
CAL = {'family': RED, 'work': BLUE, 'personal': GREEN, 'birthdays': PURPLE}

SVG = {
 'back': '<path d="m17 4-10 10 10 10"/>',
 'chevron': '<path d="m11 5 8 9-8 9"/>',
 'plus': '<path d="M14 5v18M5 14h18"/>',
 'calendar': '<rect x="4" y="5" width="20" height="20" rx="3"/><path d="M4 11h20M9 3v4m10-4v4"/>',
 'list': '<path d="M5 8h18M5 14h18M5 20h18"/>',
 'tray': '<path d="M4 15v7h20v-7M4 15l3-9h14l3 9M4 15h6l2 3h4l2-3h6"/>',
 'pin': '<path d="M14 26S5 17 5 11a9 9 0 0 1 18 0c0 6-9 15-9 15Z"/><circle cx="14" cy="11" r="3"/>',
 'clock': '<circle cx="14" cy="14" r="10"/><path d="M14 8v6l4 3"/>',
 'bell': '<path d="M7 20V12a7 7 0 0 1 14 0v8l2 2H5zM11 24a3 3 0 0 0 6 0"/>',
 'check': '<path d="m6 14 6 6L23 8"/>',
 'sync': '<path d="M23 12a9 9 0 0 0-16-4M5 16a9 9 0 0 0 16 4M7 3v5h5M21 25v-5h-5"/>',
 'people': '<circle cx="10" cy="10" r="4"/><circle cx="19" cy="11" r="3"/><path d="M3 23a7 7 0 0 1 14 0M17 23a5 5 0 0 1 8 0"/>',
 'note': '<path d="M6 4h12l5 5v15H6zM17 4v6h6M10 15h8m-8 4h8"/>',
 'gear': '<circle cx="14" cy="14" r="4"/><path d="M14 3v4m0 14v4M3 14h4m14 0h4M6.2 6.2l2.8 2.8m10 10 2.8 2.8M21.8 6.2 19 9m-10 10-2.8 2.8"/>',
 'mail': '<rect x="3" y="6" width="22" height="17" rx="2"/><path d="m3 7 11 9L25 7"/>',
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
    def __init__(self, n, id, title_cn, title_en, surface):
        self.n = n; self.id = id; self.title = {'cn': title_cn, 'en': title_en}; self.surface = surface
        self.nodes = []; self.controls = {}; self.cards = []
        self.stack('screen', 0, 0, 406, 776, bg=DESK if surface == 'desktop' else PAGE, parent='page')
    def stack(self, id, x, y, w, h, bg=None, radius=0, border=None, parent='screen'):
        self.nodes.append(dict(kind='stack', id=id, x=x, y=y, w=w, h=h, bg=bg, radius=radius, border=border, parent=parent)); return id
    def text(self, id, cn, en, x, y, w, h, size, weight=400, color=INK, align='left', parent='screen'):
        self.nodes.append(dict(kind='text', id=id, cn=cn, en=en, x=x, y=y, w=w, h=h, size=size, weight=weight, color=color, align=align, parent=parent)); return id
    def icon(self, id, kind, x, y, w, h, color=INK, parent='screen'):
        self.nodes.append(dict(kind='icon', id=id, icon=kind, x=x, y=y, w=w, h=h, color=color, parent=parent)); return id
    def button(self, id, cn, en, x, y, w, h, style='filled', size=15, enabled=True, parent='screen', icon=None):
        """A native control. filled: accent surface, white label; outline: bordered; plain: text-only."""
        self.stack(id, x, y, w, h, parent=parent)
        bg = RED if style == 'filled' else (GROUP if style == 'outline' else None)
        self.stack(id + '_surface', x, y, w, h, bg=bg, radius=10 if style != 'plain' else 0, border=(GRAY2 if style == 'outline' else None), parent=id)
        color = WHITE if style == 'filled' else (INK if style == 'outline' else RED)
        if not enabled: color = GRAY2 if style != 'filled' else WHITE
        if cn:
            self.text(id + '_label', cn, en, x, y, w, h, size, 700 if style == 'filled' else 400, color, 'center' if style != 'text' else 'left', parent=id)
        self.nodes.append(dict(kind='control', id=id + '_control', x=x, y=y, w=w, h=h, enabled=enabled, parent=id))
        self.controls[id] = dict(event=id, text_ids=[id + '_label'] if cn else [], source_bounds=[x, y, w, h], enabled=enabled)
        return id
    def card(self, id, x, y, w, h, radius=16, parent='screen'):
        self.stack(id, x, y, w, h, bg=CARD, radius=radius, border=LINE, parent=parent); self.cards.append(id); return id
    # Shared chrome
    def status_bar(self, time='09:41'):
        self.text('status_time', time, time, 28, 12, 60, 20, 15, 700)
        self.icon('status_icons', 'status', 326, 13, 56, 14, INK)
    def home_indicator(self):
        self.stack('home_indicator', 139, 762, 128, 5, bg=INK, radius=3)
    def tab_bar(self, active):
        self.stack('tab_bar', 0, 694, 406, 82, bg='f9f9f9')
        self.stack('tab_line', 0, 694, 406, 1, bg=LINE)
        for i, (id, kind, cn, en) in enumerate([('tab_today', 'calendar', '今天', 'Today'), ('tab_calendars', 'list', '日历', 'Calendars'), ('tab_inbox', 'tray', '收件箱', 'Inbox')]):
            x = 20 + i * 122; color = RED if id == active else GRAY
            self.stack(id, x, 700, 122, 56, parent='tab_bar')
            self.icon(id + '_icon', kind, x + 49, 704, 24, 24, color, parent=id)
            self.text(id + '_label', cn, en, x, 732, 122, 16, 11, 400, color, 'center', parent=id)
            self.nodes.append(dict(kind='control', id=id + '_control', x=x, y=700, w=122, h=56, enabled=True, parent=id))
            self.controls[id] = dict(event=id, text_ids=[id + '_label'], source_bounds=[x, 700, 122, 56], enabled=True)
        self.stack('inbox_badge', 292, 700, 18, 18, bg=RED, radius=9, parent='tab_inbox')
        self.text('inbox_badge_count', '1', '1', 292, 700, 18, 18, 11, 700, WHITE, 'center', parent='tab_inbox')
        self.home_indicator()
    def nav(self, back_id, back_cn, back_en, title_cn=None, title_en=None, right=None):
        if back_id:
            w = 24 + len(back_cn) * 17 + 8
            self.button(back_id, None, None, 12, 50, w, 36, 'plain')
            self.icon(back_id + '_icon', 'back', 14, 56, 22, 22, RED, parent=back_id)
            self.text(back_id + '_label', back_cn, back_en, 36, 50, w - 24, 36, 17, 400, RED, parent=back_id)
            self.controls[back_id]['text_ids'] = [back_id + '_label']
        if title_cn:
            self.text('nav_title', title_cn, title_en, 100, 50, 206, 36, 17, 700, INK, 'center')
        if right:
            id, cn, en, style = right
            self.button(id, cn, en, 306, 50, 88, 36, style, size=17)
    def desktop_clock(self, time):
        self.text('desk_date', '9月24日 周四', 'Thu, Sep 24', 24, 30, 260, 22, 15, 400, '5b6470')
        self.text('desk_time', time, time, 24, 54, 200, 44, 36, 700, '2b3340')
    def dock(self):
        self.stack('dock', 24, 690, 358, 66, bg='f7f9fc', radius=18, border=LINE)
        for i, (id, kind, cn, en) in enumerate([('dock_calendar', 'calendar', '日历', 'Calendar'), ('dock_mail', 'mail', '邮件', 'Mail'), ('dock_settings', 'gear', '设置', 'Settings')]):
            x = 40 + i * 116
            self.stack(id, x, 696, 100, 54, parent='dock')
            self.stack(id + '_tile', x + 30, 698, 40, 34, bg=WHITE, radius=8, parent=id)
            self.icon(id + '_icon', kind, x + 40, 703, 22, 22, RED if kind == 'calendar' else '5b6470', parent=id)
            self.text(id + '_label', cn, en, x, 734, 100, 14, 10, 400, '5b6470', 'center', parent=id)
            self.nodes.append(dict(kind='control', id=id + '_control', x=x, y=696, w=100, h=54, enabled=True, parent=id))
            self.controls[id] = dict(event=id, text_ids=[id + '_label'], source_bounds=[x, 696, 100, 54], enabled=True)
    def card_header(self, card, kind, cn, en, color=RED, x=None, y=None):
        x = x or self.node(card)['x'] + 20; y = y or self.node(card)['y'] + 18
        self.stack(card + '_icon_tile', x, y, 36, 36, bg=color, radius=10, parent=card)
        self.icon(card + '_icon', kind, x + 7, y + 7, 22, 22, WHITE, parent=card)
        self.text(card + '_kicker', cn, en, x + 48, y + 9, 250, 18, 13, 400, GRAY, parent=card)
    def node(self, id):
        return next(n for n in self.nodes if n['id'] == id)

def storyboard():
    scenes = []
    # 1 · Month --------------------------------------------------------------
    s = Scene(1, 'calendar-01', '月视图', 'Month', 'app'); scenes.append(s)
    s.status_bar()
    s.button('back_year', None, None, 12, 50, 80, 36, 'plain'); s.icon('back_year_icon', 'back', 14, 56, 22, 22, RED, parent='back_year')
    s.text('back_year_label', '2026年', '2026', 36, 50, 60, 36, 17, 400, RED, parent='back_year'); s.controls['back_year']['text_ids'] = ['back_year_label']
    s.button('add_event', None, None, 346, 50, 44, 36, 'plain'); s.icon('add_event_icon', 'plus', 357, 57, 22, 22, RED, parent='add_event')
    s.text('month_title', '9月', 'September', 20, 92, 300, 44, 34, 700, RED)
    for i, (cn, en) in enumerate(zip('日一二三四五六', ['S', 'M', 'T', 'W', 'T', 'F', 'S'])):
        s.text(f'weekday_{i}', cn, en, 20 + i * 52, 146, 52, 16, 12, 400, GRAY, 'center')
    s.stack('grid_line', 20, 166, 366, 1, bg=LINE)
    dots = {24: [CAL['personal'], CAL['work'], CAL['family']], 25: [CAL['family']], 26: [CAL['family']]}
    day = 1
    for row in range(5):
        for col in range(7):
            if (row == 0 and col < 2) or day > 30: continue
            x = 20 + col * 52; y = 172 + row * 54
            if day == 24:
                s.stack('today_ring', x + 8, y + 2, 36, 36, bg=RED, radius=18)
                s.button('open_today', str(day), str(day), x, y, 52, 46, 'plain', size=18)
                s.node('open_today_label').update(color=WHITE, weight=700)
            else:
                s.text(f'day_{day}', str(day), str(day), x, y + 2, 52, 36, 18, 400, INK, 'center')
            for k, color in enumerate(dots.get(day, [])):
                s.stack(f'dot_{day}_{k}', x + 26 - 4 * len(dots[day]) + k * 8 - 1, y + 40, 6, 6, bg=color, radius=3)
            day += 1
    s.stack('list_line', 20, 446, 366, 1, bg=LINE)
    s.text('list_title', '9月24日 周四 · 今天', 'Thu, Sep 24 · Today', 20, 458, 300, 20, 13, 700, GRAY)
    rows = [('row_dentist', 'open_dentist', '10:30', '牙医', 'Dentist', 'personal'), ('row_sync', 'open_team_sync', '14:00', '团队同步', 'Team sync', 'work'), ('row_dinner', 'open_dinner', '19:00', '与 Sam 晚餐', 'Dinner with Sam', 'family')]
    for i, (row, control, time, cn, en, cal) in enumerate(rows):
        y = 486 + i * 56
        s.button(control, None, None, 20, y, 366, 50, 'plain')
        s.stack(control + '_bar', 20, y + 8, 4, 34, bg=CAL[cal], radius=2, parent=control)
        s.text(control + '_time', time, time, 36, y + 2, 60, 24, 15, 700, INK, parent=control)
        s.text(control + '_label', cn, en, 100, y + 2, 240, 24, 15, 400, INK, parent=control); s.controls[control]['text_ids'] = [control + '_label']
        s.text(control + '_sub', {'personal': '个人', 'work': '工作', 'family': '家庭 · 来自 Sam'}[cal], {'personal': 'Personal', 'work': 'Work', 'family': 'Family · from Sam'}[cal], 100, y + 26, 240, 18, 12, 400, GRAY, parent=control)
        s.icon(control + '_chevron', 'chevron', 366, y + 16, 16, 16, GRAY2, parent=control)
    s.tab_bar('tab_today')
    # 2 · Day ----------------------------------------------------------------
    s = Scene(2, 'calendar-02', '日视图', 'Day', 'app'); scenes.append(s)
    s.status_bar(); s.nav('back_month', '9月', 'Sep')
    s.button('add_event', None, None, 346, 50, 44, 36, 'plain'); s.icon('add_event_icon', 'plus', 357, 57, 22, 22, RED, parent='add_event')
    s.text('day_title', '9月24日 周四', 'Thursday, Sep 24', 20, 96, 300, 32, 24, 700, INK)
    s.text('day_sub', '今天 · 3 项日程', 'Today · 3 events', 20, 130, 300, 18, 13, 400, GRAY)
    events = [('open_dentist', '10:30', '11:15', '牙医', 'Dentist', '日出牙科', 'Sunrise Dental', 'personal', 172), ('open_team_sync', '14:00', '14:45', '团队同步', 'Team sync', '视频会议', 'Video call', 'work', 292), ('open_dinner', '19:00', '20:30', '与 Sam 晚餐', 'Dinner with Sam', '莲花厨房 · 来自 Sam 桌面', 'Lotus Kitchen · from Sam · Desktop', 'family', 412)]
    for control, t0, t1, cn, en, loc_cn, loc_en, cal, y in events:
        s.text(control + '_start', t0, t0, 20, y, 60, 20, 14, 700, INK)
        s.text(control + '_end', t1, t1, 20, y + 22, 60, 18, 12, 400, GRAY)
        s.button(control, None, None, 88, y - 6, 298, 96, 'plain')
        s.stack(control + '_card', 88, y - 6, 298, 96, bg=CAL[cal] + '22', radius=12, parent=control)
        s.stack(control + '_bar', 88, y - 6, 4, 96, bg=CAL[cal], radius=2, parent=control)
        s.text(control + '_label', cn, en, 106, y + 6, 260, 24, 17, 700, INK, parent=control); s.controls[control]['text_ids'] = [control + '_label']
        s.text(control + '_loc', loc_cn, loc_en, 106, y + 34, 270, 18, 13, 400, GRAY, parent=control)
        s.text(control + '_range', f'{t0} – {t1}', f'{t0} – {t1}', 106, y + 58, 200, 18, 13, 400, CAL[cal], parent=control)
    s.text('now_line_label', '09:41', '09:41', 20, 150, 60, 16, 11, 700, RED)
    s.stack('now_line', 88, 157, 298, 2, bg=RED)
    s.tab_bar('tab_today')
    # 3 · Event detail -------------------------------------------------------
    s = Scene(3, 'calendar-03', '日程详情', 'Event', 'app'); scenes.append(s)
    s.status_bar(); s.nav('back_day', '9月24日', 'Sep 24', right=('edit_event', '编辑', 'Edit', 'text'))
    s.stack('detail_head', 20, 104, 366, 92, bg=CAL['family'] + '1a', radius=14)
    s.stack('detail_bar', 20, 104, 5, 92, bg=CAL['family'], radius=3)
    s.text('event_title', '与 Sam 晚餐', 'Dinner with Sam', 40, 114, 330, 32, 24, 700, INK)
    s.text('event_calendar', '家庭日历 · 与 Sam 共享', 'Family calendar · shared with Sam', 40, 150, 330, 18, 13, 400, GRAY)
    s.stack('detail_group', 20, 214, 366, 214, bg=GROUP, radius=14)
    rows = [('clock', 'detail_when', '2026年9月24日 周四', 'Thu, Sep 24, 2026', '19:00 – 20:30', '19:00 – 20:30'), ('pin', 'detail_where', '莲花厨房', 'Lotus Kitchen', '南京西路 88 号', '88 West Nanjing Rd'), ('bell', 'detail_alert', '提醒', 'Alert', '提前 30 分钟', '30 minutes before'), ('note', 'detail_notes', '备注', 'Notes', '订了靠窗的位子', 'Window table booked')]
    for i, (kind, id, cn, en, sub_cn, sub_en) in enumerate(rows):
        y = 224 + i * 52
        s.icon(id + '_icon', kind, 36, y + 6, 22, 22, GRAY)
        s.text(id, cn, en, 72, y, 300, 22, 15, 400, INK)
        s.text(id + '_sub', sub_cn, sub_en, 72, y + 22, 300, 18, 13, 400, GRAY)
        if i < 3: s.stack(id + '_line', 72, y + 46, 300, 1, bg=LINE)
    s.stack('sync_badge', 20, 446, 366, 44, bg='eef7ff', radius=12)
    s.icon('sync_badge_icon', 'sync', 34, 457, 22, 22, BLUE)
    s.text('sync_note', '由 Sam · 桌面 添加 · 09:40 已同步到本机', 'Added by Sam · Desktop · synced here 09:40', 66, 446, 310, 44, 13, 400, BLUE)
    s.button('delete_event', '删除日程', 'Delete Event', 20, 520, 366, 50, 'outline', size=17); s.node('delete_event_label').update(color=RED)
    s.tab_bar('tab_today')
    # 4 · New event editor ---------------------------------------------------
    s = Scene(4, 'calendar-04', '新建日程', 'New Event', 'app'); scenes.append(s)
    s.status_bar()
    s.button('cancel_editor', '取消', 'Cancel', 12, 50, 80, 36, 'text', size=17)
    s.text('nav_title', '新建日程', 'New Event', 100, 50, 206, 36, 17, 700, INK, 'center')
    s.button('save_event', '添加', 'Add', 318, 54, 72, 30, 'filled', size=15)
    s.stack('title_group', 20, 108, 366, 100, bg=GROUP, radius=14)
    s.text('title_field', '钢琴课', 'Piano lesson', 36, 116, 330, 40, 17, 400, INK)
    s.stack('title_line', 36, 158, 334, 1, bg=LINE)
    s.text('location_field', '地点或视频通话', 'Location or Video Call', 36, 166, 330, 36, 17, 400, GRAY2)
    s.stack('time_group', 20, 226, 366, 150, bg=GROUP, radius=14)
    s.text('allday_label', '全天', 'All-day', 36, 238, 200, 26, 17, 400, INK)
    s.stack('allday_switch', 330, 238, 51, 31, bg=GRAY2 + '80', radius=16); s.stack('allday_knob', 332, 240, 27, 27, bg=WHITE, radius=14)
    s.stack('allday_line', 36, 276, 334, 1, bg=LINE)
    s.button('pick_time', None, None, 20, 280, 366, 46, 'plain')
    s.text('pick_time_label', '开始', 'Starts', 36, 290, 100, 26, 17, 400, INK, parent='pick_time'); s.controls['pick_time']['text_ids'] = ['pick_time_label']
    s.stack('start_chip', 192, 288, 178, 30, bg=GRAY2 + '55', radius=8, parent='pick_time')
    s.text('start_value', '9月25日 周五  16:00', 'Fri, Sep 25   16:00', 192, 288, 178, 30, 15, 400, INK, 'center', parent='pick_time')
    s.stack('start_line', 36, 328, 334, 1, bg=LINE)
    s.text('end_label', '结束', 'Ends', 36, 340, 100, 26, 17, 400, INK)
    s.stack('end_chip', 294, 338, 76, 30, bg=GRAY2 + '55', radius=8)
    s.text('end_value', '17:00', '17:00', 294, 338, 76, 30, 15, 400, INK, 'center')
    s.stack('meta_group', 20, 394, 366, 100, bg=GROUP, radius=14)
    s.button('pick_calendar', None, None, 20, 398, 366, 46, 'plain')
    s.text('pick_calendar_label', '日历', 'Calendar', 36, 408, 100, 26, 17, 400, INK, parent='pick_calendar'); s.controls['pick_calendar']['text_ids'] = ['pick_calendar_label']
    s.stack('calendar_dot', 292, 416, 10, 10, bg=CAL['family'], radius=5, parent='pick_calendar')
    s.text('calendar_value', '家庭', 'Family', 308, 408, 44, 26, 17, 400, GRAY, parent='pick_calendar')
    s.icon('calendar_chevron', 'chevron', 362, 413, 16, 16, GRAY2, parent='pick_calendar')
    s.stack('calendar_line', 36, 446, 334, 1, bg=LINE)
    s.text('alert_label', '提醒', 'Alert', 36, 456, 100, 26, 17, 400, INK)
    s.text('alert_value', '提前 30 分钟', '30 min before', 200, 456, 152, 26, 17, 400, GRAY, 'right')
    s.icon('alert_chevron', 'chevron', 362, 461, 16, 16, GRAY2)
    s.stack('sync_hint', 20, 512, 366, 40, bg='eef7ff', radius=12)
    s.text('sync_hint_text', '保存后会同步到 Sam 的桌面', 'Saving syncs this to Sam · Desktop', 20, 512, 366, 40, 13, 400, BLUE, 'center')
    s.home_indicator()
    # 5 · Desktop card: synced from the phone ---------------------------------
    s = Scene(5, 'calendar-05', '日历已同步', 'Calendar synced', 'desktop'); scenes.append(s)
    s.desktop_clock('09:42')
    s.card('calendar_card', 24, 150, 358, 262)
    s.card_header('calendar_card', 'calendar', '日历 · 已同步', 'Calendar · Synced')
    s.text('card_title', '钢琴课', 'Piano lesson', 44, 218, 320, 30, 22, 700, INK, parent='calendar_card')
    s.text('card_when', '9月25日 周五 · 16:00–17:00', 'Fri, Sep 25 · 16:00–17:00', 44, 250, 320, 20, 14, 400, INK, parent='calendar_card')
    s.text('card_source', '家庭日历 · 来自 Alex · 手机', 'Family calendar · from Alex · Phone', 44, 272, 320, 18, 13, 400, GRAY, parent='calendar_card')
    s.button('ack_sync', '知道了', 'Got it', 44, 308, 150, 40, 'filled', parent='calendar_card')
    s.button('undo_event', '撤销', 'Undo', 208, 308, 150, 40, 'outline', parent='calendar_card')
    s.button('open_calendar', '在日历中打开', 'Open in Calendar', 44, 360, 200, 32, 'text', size=14, parent='calendar_card')
    s.dock()
    # 6 · Desktop card: invitation ---------------------------------------------
    s = Scene(6, 'calendar-06', '日程邀请', 'Invitation', 'desktop'); scenes.append(s)
    s.desktop_clock('09:43')
    s.card('invite_card', 24, 150, 358, 300)
    s.card_header('invite_card', 'people', '邀请 · 来自 Sam', 'Invitation · from Sam', BLUE)
    s.text('invite_title', '周末远足', 'Weekend hike', 44, 218, 320, 30, 22, 700, INK, parent='invite_card')
    s.text('invite_when', '9月26日 周六 · 09:00–12:00', 'Sat, Sep 26 · 09:00–12:00', 44, 250, 320, 20, 14, 400, INK, parent='invite_card')
    s.text('invite_where', '西山步道 · 家庭日历', 'West Hill trail · Family calendar', 44, 272, 320, 18, 13, 400, GRAY, parent='invite_card')
    s.text('invite_note', '接受后会加入你的日历，并通知 Sam', 'Accepting adds it to your calendar and tells Sam', 44, 296, 320, 18, 12, 400, GRAY, parent='invite_card')
    s.button('accept_invite', '接受', 'Accept', 44, 330, 100, 40, 'filled', parent='invite_card'); s.node('accept_invite_surface').update(bg=BLUE)
    s.button('maybe_invite', '待定', 'Maybe', 154, 330, 100, 40, 'outline', parent='invite_card')
    s.button('decline_invite', '拒绝', 'Decline', 264, 330, 94, 40, 'outline', parent='invite_card'); s.node('decline_invite_label').update(color=RED)
    s.button('open_calendar', '在日历中查看当天', 'See that day in Calendar', 44, 384, 240, 32, 'text', size=14, parent='invite_card')
    s.dock()
    # 7 · Time picker with a conflict -------------------------------------------
    s = Scene(7, 'calendar-07', '选择时间', 'Choose a time', 'app'); scenes.append(s)
    s.status_bar(); s.nav('back_editor', '新建日程', 'New Event')
    s.text('picker_title', '选择时间', 'Choose a time', 20, 96, 300, 32, 24, 700, INK)
    s.text('picker_sub', '9月24日 周四 · 家庭日历', 'Thu, Sep 24 · Family calendar', 20, 130, 300, 18, 13, 400, GRAY)
    slots = [('slot_13', '13:00 – 14:00', True, None, None), ('slot_14', '14:00 – 15:00', False, '与「团队同步」冲突', 'Conflicts with “Team sync”'), ('slot_15', '15:00 – 16:00', True, None, None), ('slot_16', '16:00 – 17:00', True, None, None), ('slot_17', '17:00 – 18:00', True, None, None)]
    for i, (id, label, enabled, why_cn, why_en) in enumerate(slots):
        y = 168 + i * 66
        s.button(id, None, None, 20, y, 366, 56, 'plain', enabled=enabled)
        s.node(id + '_surface').update(bg=GROUP if enabled else 'f7f7f9', radius=14)
        s.text(id + '_label', label, label, 40, y + (6 if why_cn else 15), 200, 26, 17, 400, INK if enabled else GRAY2, parent=id); s.controls[id]['text_ids'] = [id + '_label']
        if why_cn: s.text(id + '_why', why_cn, why_en, 40, y + 32, 300, 18, 12, 400, GRAY, parent=id)
        if id == 'slot_16': s.icon(id + '_check', 'check', 350, y + 17, 22, 22, RED, parent=id)
    s.text('picker_hint', '冲突时段不可选择；日程不会自动重叠', 'Conflicting slots stay disabled; nothing double-books', 20, 510, 366, 18, 12, 400, GRAY, 'center')
    s.home_indicator()
    # 8 · Calendars list --------------------------------------------------------
    s = Scene(8, 'calendar-08', '日历列表', 'Calendars', 'app'); scenes.append(s)
    s.status_bar(); s.nav(None, None, None, '日历', 'Calendars', right=('done_calendars', '完成', 'Done', 'text'))
    s.text('section_icloud', 'iCloud', 'iCloud', 36, 108, 200, 18, 13, 400, GRAY)
    s.stack('calendars_group', 20, 130, 366, 210, bg=GROUP, radius=14)
    cals = [('toggle_family', 'family', '家庭', 'Family', '与 Sam 共享', 'Shared with Sam', True), ('toggle_work', 'work', '工作', 'Work', None, None, True), ('toggle_personal', 'personal', '个人', 'Personal', None, None, True), ('toggle_birthdays', 'birthdays', '生日', 'Birthdays', '已隐藏', 'Hidden', False)]
    for i, (id, cal, cn, en, sub_cn, sub_en, on) in enumerate(cals):
        y = 134 + i * 52
        s.button(id, None, None, 20, y, 366, 50, 'plain')
        s.icon(id + '_check', 'check', 32, y + 14, 22, 22, CAL[cal] if on else GROUP, parent=id)
        s.stack(id + '_dot', 66, y + 19, 12, 12, bg=CAL[cal], radius=6, parent=id)
        s.text(id + '_label', cn, en, 88, y + (4 if sub_cn else 12), 200, 26, 17, 400, INK, parent=id); s.controls[id]['text_ids'] = [id + '_label']
        if sub_cn: s.text(id + '_sub', sub_cn, sub_en, 88, y + 28, 200, 18, 12, 400, GRAY, parent=id)
        if i < 3: s.stack(id + '_line', 66, y + 50, 310, 1, bg=LINE, parent=id)
    s.stack('sync_group', 20, 364, 366, 64, bg=GROUP, radius=14)
    s.icon('sync_icon', 'sync', 36, 385, 22, 22, GREEN)
    s.text('sync_status', '已同步 · 09:41', 'Synced · 09:41', 72, 372, 300, 24, 15, 400, INK)
    s.text('sync_devices', '2 台设备：Alex · 手机，Sam · 桌面', '2 devices: Alex · Phone, Sam · Desktop', 72, 398, 300, 18, 12, 400, GRAY)
    s.tab_bar('tab_calendars')
    # 9 · Desktop card: sync status --------------------------------------------
    s = Scene(9, 'calendar-09', '同步状态', 'Sync status', 'desktop'); scenes.append(s)
    s.desktop_clock('09:44')
    s.card('sync_card', 24, 150, 358, 268)
    s.card_header('sync_card', 'sync', '同步 · 最新', 'Sync · Up to date', GREEN)
    s.text('sync_title', '两台设备一致', 'Both devices agree', 44, 218, 320, 30, 22, 700, INK, parent='sync_card')
    s.text('device_phone', 'Alex · 手机 · 09:41', 'Alex · Phone · 09:41', 44, 254, 320, 20, 14, 400, INK, parent='sync_card')
    s.text('device_desktop', 'Sam · 桌面 · 09:40', 'Sam · Desktop · 09:40', 44, 276, 320, 20, 14, 400, INK, parent='sync_card')
    s.text('pending_count', '0 项待同步更改 · 第 12 次同步', '0 pending changes · sync #12', 44, 300, 320, 18, 13, 400, GRAY, parent='sync_card')
    s.button('sync_now', '立即同步', 'Sync now', 44, 332, 150, 40, 'filled', parent='sync_card'); s.node('sync_now_surface').update(bg=GREEN)
    s.button('open_calendar', '打开日历', 'Open Calendar', 208, 332, 150, 40, 'outline', parent='sync_card')
    s.text('sync_foot', '更改只会在你操作后发送，不会自动排程', 'Changes are sent only after you act; nothing is scheduled for you', 44, 384, 320, 18, 11, 400, GRAY, parent='sync_card')
    s.dock()
    # 10 · Desktop card: removed ---------------------------------------------------
    s = Scene(10, 'calendar-10', '日程已删除', 'Event removed', 'desktop'); scenes.append(s)
    s.desktop_clock('09:45')
    s.card('calendar_card', 24, 150, 358, 236)
    s.card_header('calendar_card', 'calendar', '日历 · 已删除', 'Calendar · Removed', GRAY)
    s.text('removed_title', '与 Sam 晚餐', 'Dinner with Sam', 44, 218, 320, 30, 22, 700, INK, parent='calendar_card')
    s.text('removed_when', '9月24日 周四 · 19:00–20:30', 'Thu, Sep 24 · 19:00–20:30', 44, 250, 320, 20, 14, 400, GRAY, parent='calendar_card')
    s.text('removed_note', '在 Alex · 手机 上删除；Sam 的副本已同步更新', 'Deleted on Alex · Phone; Sam’s copy is updated', 44, 272, 320, 18, 13, 400, GRAY, parent='calendar_card')
    s.button('undo_delete', '撤销删除', 'Undo delete', 44, 308, 150, 40, 'filled', parent='calendar_card')
    s.button('open_calendar', '打开日历', 'Open Calendar', 208, 308, 150, 40, 'outline', parent='calendar_card')
    s.dock()
    return scenes

# --- HTML render of the storyboard ---------------------------------------------
def css_color(v):
    return '#' + v

def scene_html(s, interactive=False):
    """The scene as absolutely positioned HTML. With `interactive`, nodes carry
    data-node/data-text/data-control attributes so the browser preview can bind
    the session's native text, enabled state and styles to them."""
    parts = []; controls = []
    for n in s.nodes:
        x, y, w, h = n['x'], n['y'], n['w'], n['h']
        base = f'left:{x}px;top:{y}px;width:{w}px;height:{h}px;'
        tag = (lambda attr: f' data-node="{n["id"]}"{attr}') if interactive else (lambda attr: '')
        if n['kind'] == 'control':
            # Hit areas go last so they sit above every surface and label.
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
            parts.append(f'<svg class="i"{tag("")} style="{base}" viewBox="{vb}" fill="none" stroke="{css_color(n["color"])}" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">{SVG[n["icon"]]}</svg>')
    return ''.join(parts + controls)

GUTTER = 48; CAPTION = 34; COLS = 4
def atlas_layout(scenes):
    """Grid slots in atlas pixels at 2×: [x, y, w, h] per scene, reading order."""
    w, h = ARTBOARD[0] * 2, ARTBOARD[1] * 2; crops = []
    for i, _ in enumerate(scenes):
        r, c = divmod(i, COLS)
        crops.append([GUTTER + c * (w + GUTTER), GUTTER + CAPTION + r * (h + GUTTER + CAPTION), w, h])
    rows = (len(scenes) + COLS - 1) // COLS
    return crops, [COLS * w + (COLS + 1) * GUTTER, rows * h + (rows + 1) * GUTTER + rows * CAPTION]

def atlas_html(scenes):
    crops, (aw, ah) = atlas_layout(scenes)
    regular = FONTS[400].resolve().as_uri(); bold = FONTS[700].resolve().as_uri()
    head = f'''<!doctype html><html><head><meta charset="utf-8"><style>
@font-face{{font-family:"Noto Sans SC";src:url("{regular}");font-weight:400}}
@font-face{{font-family:"Noto Sans SC";src:url("{bold}");font-weight:700}}
html,body{{margin:0;background:#d7dbe2}}
body{{width:{aw // 2}px;height:{ah // 2}px;position:relative;font-family:"Noto Sans SC";-webkit-font-smoothing:antialiased}}
.screen{{position:absolute;width:{ARTBOARD[0]}px;height:{ARTBOARD[1]}px;overflow:hidden}}
.s,.t,.i{{position:absolute;box-sizing:border-box}}
.t{{white-space:nowrap;overflow:visible}}
.cap{{position:absolute;font-size:12px;color:#3c4452;line-height:{CAPTION}px}}
</style></head><body>'''
    body = []
    for s, crop in zip(scenes, crops):
        x, y = crop[0] // 2, crop[1] // 2
        body.append(f'<div class="cap" style="left:{x}px;top:{y - CAPTION}px">{s.n:02d} · {"desktop card" if s.surface == "desktop" else "app"} · {html_mod.escape(s.title["cn"])} / {html_mod.escape(s.title["en"])}</div>')
        body.append(f'<div class="screen" style="left:{x}px;top:{y}px">{scene_html(s)}</div>')
    return head + ''.join(body) + '</body></html>', crops, (aw, ah)

if __name__ == '__main__':
    scenes = storyboard()
    html, crops, size = atlas_html(scenes)
    out = ROOT / 'source/storyboard.html'; out.parent.mkdir(exist_ok=True); out.write_text(html)
    print(len(scenes), 'scenes', size, crops[0], sum(len(s.controls) for s in scenes), 'controls')
