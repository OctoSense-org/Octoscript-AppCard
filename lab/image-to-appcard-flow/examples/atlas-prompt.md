# One-atlas service application storyboard prompt template

This is an authoring template for an external image-generation step. Replace
the placeholders before generation and retain the exact submitted text. The
atlas intake CLI does not generate images, verify a model provider, or approve
the resulting design.

```text
Create ONE original image containing {SCENE_COUNT: 8–12} complete mobile UX
screens for {SCENARIO}. Generate every screen together in this single image.
Do not generate separate images for individual states.

Requested image size: {REQUESTED_WIDTH} × {REQUESTED_HEIGHT} pixels
Requested quality: high
Grid: {COLUMNS} columns × {ROWS} rows, in chronological reading order
Shared logical artboard: 406 × 776 for every screen

Keep every screen equally sized and fully visible. Keep all screen rectangles
separate, with generous, consistent gutters and no overlapping frames. Place
scene numbers and captions ONLY in the gutters, outside the screen bounds.
Do not add perspective, device bezels, slanted mockups, or cropped screen edges.
Actual pixel boundaries will be measured after generation; do not assume that
the model obeyed the requested size or grid perfectly.

Visual language: modern, calm, warm ivory, sage green, light mist blue, plenty
of whitespace, clear Chinese labels and quiet native controls. Use neutral
service names with no retailer branding. Keep typography, app icons, corner
radii, button styles, wallpaper, data and recurring artwork consistent across
every scene.

Keep two kinds of experience distinct:
- Application interiors use familiar native navigation and content layouts
- Desktop Service App Cards belong to the calendar, payment, logistics,
  booking or other service app that owns the represented data and action

Do not introduce chat messages, assistant bubbles, a conversational approval
panel or an assistant mascot. A service card is an actionable system surface.
The card body opens its owning application. A small source link opens the
relevant original event or service record.

Use the exact approved fixture throughout:
{FIXTURE: people, dates including weekdays/time zone, location, product,
amount and currency, booking and calendar ownership, service status}

Depict these scenes in order, with one complete screen per entry:
{SCENES: scene number; app or desktop; visible card owners; exact visible copy;
data state; enabled/disabled controls; what the user has already authorized;
what is still waiting for user action; expected result of each visible action}

State consistency rules:
- A calendar card may report an event already added by an authorized source
- Confirming such a card acknowledges the update; it does not add a duplicate
- Undoing the calendar event does not cancel an independent service booking
- Payment stays pending until the user explicitly chooses to pay
- Cancelling a pending payment request is not a refund or service cancellation
- Repeated events update the same service identity rather than create duplicates
- Conflicting appointment slots remain visibly disabled

Render all interface text and controls clearly enough for later measurement.
Photographs and illustrations can be reused as isolated artwork regions;
labels, buttons, status indicators and data-driven graphics will be rebuilt
as actual native Makepad components in a separate, reviewed design pipeline.
```

After generation, record the original atlas and this exact completed prompt
as immutable inputs. Measure each actual screen rectangle in the atlas's
stored pixel matrix, then create a manifest with `schema_version: 1`, an `id`,
`artboard: [406, 776]`, and a `generation` object containing `atlas`, `prompt`,
`provider`, optional `model`, `requested_size: [width, height]`, and
`quality: "high"`. Atlas and prompt paths are relative to the project root.

Provide 8–12 `scenes`, each with a unique string `id`, unique `design_id`,
integer `crop: [x, y, width, height]`, and `surface: "app"` or `"desktop"`.
Additional scene metadata is retained as declared context, not converted into
review or native-mapping approval. Crops may touch but must not overlap.

```sh
python3 lab/image-to-appcard-flow/atlas.py \
  --manifest /absolute/project/storyboard.json \
  --project /absolute/project \
  --output /absolute/project/intakes/first-atlas
```

The receipt records requested and measured dimensions separately. It preserves
the source bytes and hashes, exact lossless crops, and uniformly fitted 2×
references with reversible transforms. A repeated call with identical inputs
returns the existing intake without writing; any changed input or modified
artifact requires a new output directory. Native reconstruction and Studio
verification are subsequent steps, never implied by successful intake.
