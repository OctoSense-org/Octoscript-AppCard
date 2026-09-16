Calendar — one-atlas storyboard brief (authored render, not a generated image)

This atlas is not the output of an image model. It is a deterministic render of
scripts/design.py: the storyboard HTML (source/storyboard.html) is produced from
the same logical layout that the native contracts are authored from, then
rasterised by headless Chromium at 2× (source/render_atlas.mjs). The intake
freezes the actual PNG bytes; contracts do not read the pixels back.

Scenario: an iOS-style Calendar for one household. Two devices share the same
calendar service: "Alex · Phone" (the app screens) and "Sam · Desktop" (the
desktop service cards). Every change is an explicit user action on one device,
recorded by the calendar service and synced to the other; nothing is scheduled
or confirmed on the user's behalf.

Fixture (all dates 2026, +08:00; "today" is Thu Sep 24, 09:41):
- Dentist, Thu Sep 24 10:30–11:15, Personal, Sunrise Dental
- Team sync, Thu Sep 24 14:00–14:45, Work, video call
- Dinner with Sam, Thu Sep 24 19:00–20:30, Family, Lotus Kitchen — created by
  Sam · Desktop and synced to the phone at 09:40
- New event Piano lesson, Fri Sep 25 16:00–17:00, Family — created on the phone
- Invitation from Sam: Weekend hike, Sat Sep 26 09:00–12:00, Family, West Hill
- Calendars: Family (shared with Sam), Work, Personal, Birthdays (hidden)

Shared artboard 406 × 776 per screen; grid 4 columns × 3 rows, reading order,
captions only in the gutters. Rendered at 2× → 812 × 1552 px per screen.

Scenes:
01 app · Month: September, today ring on 24, dots on 24/25/26, today's list
02 app · Day: Thu Sep 24 agenda with the three events and the now line
03 app · Event: Dinner with Sam — time, place, alert, notes, sync note, Delete
04 app · New Event: Piano lesson editor with Starts/Ends, Calendar, Alert, Add
05 desktop · Calendar · Synced: Piano lesson arrived from Alex · Phone — Got it / Undo
06 desktop · Invitation from Sam: Weekend hike — Accept / Maybe / Decline
07 app · Choose a time: 14:00 slot disabled because it conflicts with Team sync
08 app · Calendars: Family, Work, Personal, Birthdays toggles; sync status
09 desktop · Sync · Up to date: both devices, 0 pending — Sync now / Open
10 desktop · Calendar · Removed: Dinner deleted on the phone — Undo delete / Open

State rules: a synced card reports a change already made on the other device;
acknowledging it does not duplicate the event. Undo affects the named calendar
operation only. Accepting an invitation adds the event and answers Sam;
declining does not delete Sam's event. Conflicting slots stay disabled; the
service never double-books the same calendar.
