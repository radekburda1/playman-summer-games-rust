# Brother character assets

## Radek

`radek-heads.png` was generated with built-in image_gen, transparent_background=true, using the supplied Radek portrait as identity reference. Four neckless views supply gameplay; the same frontal head supplies both transparent menu sizes. Standard body and uniform. Prompt:

Generate a production pixel-art HEAD sprite sheet based on this man, Radek. Preserve his recognizable bald shaved head (NO hair, NO cap), bright blue eyes, warm light skin, light short brown stubble beard, dark eyebrows and broad friendly toothy smile. Exactly FOUR isolated HEAD ONLY views in ONE horizontal row: strict right-facing profile, three-quarter right, full front, full rear. NO NECK in any view: silhouette ends at chin/jaw, rear ends at skull base just below ears. No neck stubs, no shoulders, no shirt, no body. Compact rounded heads like 2008 Playman mobile athletics game. Chunky low-resolution pixel outlines, simple 3-tone shading suitable for nearest neighbor reduction to about 30x35 pixels. Consistent identity and scale across all four views. Four evenly spaced cells with generous fully transparent gutters and padding. Actual transparent background; no reference UI, frame, labels or text. Frontal view must work as transparent head-selection portrait.

Current torso correction replaces the procedural belly described below. `brother-torsos.png` contains four purpose-drawn transparent views, generated with built-in image_gen. The red singlet covers a continuous round belly. Native sizes are 48x40, 48x36, 52x34 and 52x34; original animation centers are preserved. The former exposed-midriff pixel fill is removed.

Torso generation prompt:

Create a transparent pixel-art game asset sheet: exactly FOUR isolated TORSO ONLY sprites in ONE horizontal row of four equal cells. Views left to right strict right-facing side profile, three-quarter right, frontal, rear. A very round stocky adult male athlete's torso with a big comical pot belly, red athletics singlet with white vertical side stripe, black shorts waistband at base. The singlet fits over the ENTIRE round belly (no bare skin belly), a continuous smoothly rounded pear-shaped body from narrow shoulders to bulging middle then tapered waist. Clear rounded silhouette, no horizontal cut, no detached oval, no flat bowl shape. HEADLESS, NO neck, NO arms, NO hands, NO legs, NO feet, NO face: shoulders and armholes only, waist ends at shorts waistband. Same torso height all four views. Side torso width about 90% of torso height; front torso width about 110% height. Chunky low resolution outlined pixel art matching a 2008 Playman mobile athletics game, 3-tone shading, thick dark outline, simple bold red and white uniform. Each torso centered with generous transparent padding and baseline aligned. Actual transparent background. No labels, text, props, UI, shadows or ground.

Current neckless revision (built-in image_gen edit, transparent background):

Edit this existing four-view pixel-art head sprite sheet. Preserve the same recognizable man, full brown beard, bald scalp under a BLACK BACKWARD cap, same four views in same horizontal row: right profile, three-quarter right, front, rear. Remove ALL NECK from every sprite: each silhouette ends at jaw/chin/beard; no skin neck stub extending below or behind the jaw. Rear view ends just below the ears at skull base, no long nape/neck extension. Compact rounded head silhouettes like original 2008 Playman mobile game character heads. Preserve face, smile, eyebrows, beard, cap orientation (brim behind head in ALL views including rear), low resolution outlined pixel art. No shoulders, body, collar, text, frame or extra objects. Four isolated heads, evenly spaced cells, generous transparent margins. Actual transparent background. Front head must also be suitable as transparent character-selection portrait. Keep head size consistent across views.

The menu uses the transparent frontal head from this sheet, without shoulders or an opaque portrait rectangle.

`brother-reference.png` is the user's supplied reference, copied unchanged. The
reference is retained for provenance; the selector now uses the transparent frontal head from the sprite sheet.
`brother-heads.png` was generated with the built-in image_gen tool using that
reference. The four transparent cells provide profile, three-quarter, frontal and
back head views. Rendering downsamples each alpha-bounded cell to game resolution
and generates the eight original sprite transforms. The original male skin/body
bank supplies limbs; the torso has a rounded belly profile and broader limbs.
This changes presentation only, with unchanged equipment hooks and mechanics.

Source prompt (built-in image_gen, transparent background):

Create a production sprite sheet for a 2008 mobile pixel-art athletics game.
Reference image is the identity reference ONLY: adult man with warm light/tan
skin, brown eyes, thick dark eyebrows, full brown beard, black backward baseball
cap with back strap visible, friendly competitive smile. Preserve recognizable
face. Output exactly FOUR isolated HEAD ONLY sprites in a single horizontal row
of FOUR equally sized square cells. Left to right: (1) strict right-facing
profile looking right, (2) three-quarter right-facing view, (3) full frontal face
looking at viewer, (4) back-of-head view showing black cap and back neck. Each
head includes tiny neck stub and NO shoulders or body. Same size and alignment:
heads centered in each cell, neck at same baseline, generous transparent padding,
no clipping or touching. Consistent low-resolution chunky pixel art with dark
outlines and simple 3-tone shading, approximately 32x40 pixel apparent design per
head, suited to nearest-neighbor downsizing. Truly transparent background. No
frame, no UI, no arrows, no text, no numerals. This is an asset sheet, not a mockup.

## Corrective edit: bald scalp and backward cap

The current sheet was edited with built-in image_gen using the previous sheet as
edit target and the user's additional bald reference photo as identity support.
The game portrait/reference crop and fuller body were left unchanged.

Corrective prompt:

EDIT image 1, the existing four-view pixel-art game head sprite sheet. Image 2 is
the real person's BALD scalp reference. Make only these corrections, preserving
the same recognizable face, full brown beard, eyebrows, black cap, pixel-art
style, four cells in one horizontal row, relative head sizes, neck baselines and
transparent background. (1) The man is BALD under his cap. Remove all scalp hair,
side scalp hair and nape hair in every view. Bare shaved light/tan skin must be
visible behind ears, above the beard, around the hat opening and at the back of
neck. Keep his full facial beard; distinguish beard on jaw/chin from scalp/nape
hair. (2) The cap is worn BACKWARDS in ALL FOUR views. Preserve backward cap in
the first three views: adjustment strap/opening over the forehead and brim
pointing behind the head. Correct the fourth RIGHTMOST rear-view sprite: seen
from behind the viewer must see the cap's BRIM projecting toward the viewer at
the lower rear edge of the cap, NOT an adjustment strap/opening on the back of
the head. The adjustment strap is at his forehead, hidden from this rear view.
Remove the wrongly positioned rear strap/opening, remove the brown
hair/beard-like patch on rear scalp/nape, show bare shaved skin under the
backwards brim. No hair on back of head. No extra props, no shoulders, no labels
or text, no background, no UI. Keep FOUR isolated sprites: right profile,
three-quarter right, frontal, rear. This is a precise corrective asset edit,
not a redesign.

## Kubak

Current neckless correction uses built-in image_gen edit with transparent background. All four views end at jaw/skull base. Standard body and supplied selector portrait are preserved.

Corrective prompt:

Edit this existing FOUR-view pixel-art head sheet for Kubak. Preserve his identity exactly: swept textured light-brown hair, bright blue eyes, short brown beard, friendly smile, no hat. Preserve four heads in one horizontal row, right profile / three-quarter right / front / rear, same order and style. Remove ALL NECK extensions from every view. Heads must end at chin/jaw/beard, with no skin stub below or behind the jaw. Rear view ends at skull base just below ears, no projecting nape or neck. Compact isolated HEAD ONLY silhouettes like original Playman mobile game sprites. Keep beard, haircut, facial proportions and pixel outlines. No shoulders, collar, body, text or frame. Four evenly spaced cells, generous transparent padding, consistent scale. Actual transparent background.

`kubak-reference.png` is a lossless PNG conversion of the supplied portrait.
The selector samples (210,105,420,580). `kubak-heads.png` was generated using
built-in image_gen in transparent-background mode with the portrait reference.
Its four cells are right profile, three-quarter right, front and back. Kubak
uses the standard male body and uniform with no width transformation.

Source prompt:

Create a production transparent pixel-art HEAD sprite sheet for a 2008 mobile athletics game, based on the man in the reference portrait. His name is Kubak. Preserve his recognizable features: fair/light warm skin, bright blue eyes, swept-up textured light-brown/dark-blond hair, short neatly trimmed light-brown beard and mustache, friendly toothy smile. NO hat. HEADS ONLY including tiny neck stubs: no shoulders, no hoodie, no body. Exactly FOUR isolated views in ONE horizontal row of four evenly spaced cells, left to right: strict RIGHT-facing profile, three-quarter RIGHT-facing, full FRONT face, full BACK of head showing matching short hair and neck. Keep each head the same size and baseline, centered with generous transparent padding, no touching/clipping. Match chunky low-resolution outlined pixel art suitable for nearest-neighbor reduction to roughly 32 by 36 game pixels per head; simple three-tone shading, clear silhouette. Preserve consistent hairline, haircut and beard in all views; no beard on rear scalp. Genuinely transparent background. No text, no labels, no frames, no arrows, no UI or extra props. This is an asset sheet, not a mockup.

## Cartoon body revision

The code-generated brother torso now has a 2.6x maximum belly width, a short jersey over a shaded exposed midriff and waistband. Limbs use 1.45x width. Custom atlas cells are 96 pixels to contain the broader silhouettes. Original joint hooks and simulation remain unchanged. The supplied cartoon example guides proportions only.

Kubak menu correction: both preview sizes now sample the transparent neckless frontal head, not the supplied opaque portrait crop. Reference image is retained for provenance only.
