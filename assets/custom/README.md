# Brother character assets

## Burdas — 2026-11-03

Runtime `burdas-heads.png` was generated with built-in image_gen,
transparent_background=true. `burdas-reference.png` is the user's identity photo;
`burdas-style-reference.png` is the user's original-character lineup reference,
both copied unchanged. Four neckless views: right profile, three-quarter right,
front, rear. Runtime sizes34x35,31x36,29x36,28x31. Both transparent menu sizes
(25x25/50x50) use the frontal head. Rear-turn animations use the three-quarter
view, as for the other custom characters. Standard male body/uniform/height.
All six sprites append at BASE+43, preserving previous atlas coordinates.

Generated source: `/Users/Radek.Burda/.codex/generated_images/01a19e70-a6c1-72f3-87ff-0d545c49e17f/exec-308d79fe-4799-401d-96f9-12bb37aafd93.png`.
Exact prompt:

> Create a production transparent pixel-art HEAD sprite sheet for Burdas, the man in IMAGE 1. IMAGE 1 is identity reference; IMAGE 2 is visual style reference of the game's original character lineup ONLY. Preserve recognizable short dark-brown slightly spiky hair, high/receding hairline and broad forehead, gray-blue eyes, brown eyebrows, prominent straight nose, thin light-brown mustache and very light jaw stubble (NOT a full beard), fair warm skin, small confident friendly smile. Match the charming outlined, bold-shaded pixel-art heads in image 2, a 2008 mobile athletics game, simple 3-tone shading and deliberate chunky pixels suitable for nearest-neighbor reduction to roughly32x35 pixels. Exactly FOUR isolated HEAD ONLY views in ONE horizontal row, evenly spaced equal cells: strict RIGHT-facing profile, three-quarter RIGHT, full FRONT, full REAR. Consistent head size/identity across views. Absolutely NO NECK in any view: ends at chin/jaw, rear at skull base just under ears. No neck stub, shoulders, collar, clothes, body, hat or accessories. Generous transparent margins and gutters, no touching/clipping. Actual transparent background, not a checkerboard. No background, text, labels, flags, UI or frame. Front head will also be the transparent selection portrait.

## Head-turn and shorts integration — 2026-11-03

Brother, Kubak and Radek now use their existing three-quarter head whenever the
original animation requests a rear view. Both head-remapping entry points share
this rule, so starts and celebrations keep recognizable faces visible. Rear
source views and atlas cells are preserved; no new image generation this pass.

Brother's hip and shorts-bearing upper-leg pieces (virtual slots 16,23–26) now use
original widths. The former 1.45x scaling made the black shorts project into a
point during the celebration jump. Other limb widths, belly artwork and joint
positions are unchanged. This is atlas integration of the existing artwork.

## Current shoulder correction — 2026-11-03

Runtime uses `brother-torsos-shoulders.png`. Built-in image_gen edited the upper
third of `brother-torsos-low-belly.png`, using `brother-torsos-v2.png` as a compact
open-shoulder reference. Transparent_background=true. Four views remain; runtime
samples the profile and supplies all four lean poses and eight transforms. The
low round belly and bottom waistband remain; the enclosed tall shoulder loop is
replaced by a shallow open arm socket. No original game artwork is overwritten.

The old enclosed hole was measured at 25.5% width/17.35% height within its alpha
bounds, rather than the original guessed 28%/22%. The edited open socket is fitted
at 18%/17%; bottom waistband remains 47%/94%. Front width is 1.5x. Rig joints, body
length, leg attachment and 96x96 transparent cells are preserved. Tests check the
arm socket is transparent at each arm joint and the hem stays near each hip.
Thin downsampled waistband shading may occupy only a few gray pixels.

Generated source: `/Users/Radek.Burda/.codex/generated_images/01a19e70-a6c1-72f3-87ff-0d545c49e17f/exec-d1f7480b-a9cb-490d-ad4c-79e4a7d034e3.png`.
Exact built-in edit prompt:

> Precise pixel-art asset edit. IMAGE 1 is edit target. IMAGE 2 is reference ONLY for its compact open shoulder/arm socket. Edit ONLY UPPER THIRD of the four torsos in image 1. Keep its low round fat belly, lower two thirds silhouette, red/white colors, black bottom waistband, all four views and overall bounds unchanged. Replace the tall closed circular shoulder hoop on profile and three-quarter views with the short OPEN shoulder shape like image 2: shoulder strap close to body, shallow side arm socket opening to LEFT edge in right-facing profile, NO enclosed round hole, NO upward/backward loop or spike. A compact neckline connecting to the near shoulder like original mobile athletics game singlet. No extra fabric tab behind shoulder. Keep chest small, belly very round and low, existing waist width and bottom baseline unchanged. Four isolated headless neckless armless legless torso views in one horizontal row: right profile, three-quarter right, front, rear. Chunky outlined pixel art, no neck head arms hands legs skin shadows text labels. True transparent background.

## Previous low-belly source — retained for provenance

`brother-torsos-low-belly.png` was the edit target above. Its source and prompt:

Source: `/Users/Radek.Burda/.codex/generated_images/01a19e70-a6c1-72f3-87ff-0d545c49e17f/exec-fa78a601-cf7e-4184-b521-e0d24bc7c97b.png`.
Edit target: `brother-torsos.png`. Exact prompt:

> Edit this pixel-art torso sprite sheet. Keep exactly four views in one horizontal row, side profile RIGHT, 3/4 right, front, rear, same red singlet white side stripe black waistband, headless armless legless. Main change: make the man VERY FAT WITH A LOW SAGGING POT BELLY, NOT a muscular big chest. Upper chest must be small and almost flat, narrow shoulders; upper third compact. Enormous soft round abdomen in the LOWER TWO THIRDS, maximum forward projection near 75 percent of torso height. Round hanging belly over a low narrow black waistband. Clear pear-shaped silhouette: thin top, huge round bottom, rounded bottom edge, absolutely no pointed or cone shape. Side profile: nearly vertical chest then curve OUT far to RIGHT and DOWN into a large rounded drooping belly. Keep shoulder socket center around x28%,y22% and waistband center around x47%,y94% of profile silhouette bounds, so existing limb attachment points still fit. Four separated transparent cells, whole torsos visible. Simple chunky low resolution mobile-game pixel art with three-tone shading and dark outlines, no muscle shading, no pecs, no abs. No neck head arms hands legs feet shadows text labels. Truly transparent background.

The bottom waistband at 94% is the current hip attachment. The previous 70%
experiment is removed: it stretched the belly down over the legs. A hem-position
regression now checks every lean pose in addition to joint fit and cell clipping.

## Superseded torso rig correction — 2026-11-02

The prior iteration used `brother-torsos-v2.png`, made with the built-in image_gen edit tool
and `transparent_background=true`. The original `brother-torsos.png` is retained.
The revised profile has a rounded covered pot belly, narrow waistband and open
shoulder socket instead of a large enclosed black hole. Only the first profile
cell is consumed: original uniform slots 612–615 are successive lean angles,
not profile/three-quarter/front/rear views. The atlas compositor rotates this
profile by -15, 15, 45 and 75 degrees, nearest-neighbor samples into 52x40,
52x36, 56x34 and 56x32 sprites, then provides all eight original transforms.
Existing transformed anchor offsets and 1.45x limbs remain. The other generated
views are retained as source material, not incorrectly inserted into bent poses.

Source: `/Users/Radek.Burda/.codex/generated_images/01a19e70-a6c1-72f3-87ff-0d545c49e17f/exec-8e215fc7-d336-4b0b-a850-7815891c95f6.png`.
Edit target: existing `brother-torsos.png`. Exact prompt:

> Use case: precise-object-edit. Edit target: attached four torso sheet. Create improved replacement game torso sprite sheet, exactly FOUR isolated headless armless torsos in one horizontal row, views right-facing profile, three-quarter right, front, rear. Same red singlet white side stripe and black shorts waistband. Funny fat adult male athlete, big round low pot belly, compact chest and shoulders, NARROW waist attachment at bottom for short stocky legs. Important: belly projects FORWARD (RIGHT) in profile, back contour nearly straight. Keep roundness but remove inflated beach-ball / sack shape. Remove the huge black outlined circular shoulder hole: use a SMALL shallow arm socket at upper back left in profile, no enclosed black circle, no skin or limbs. Compact shoulders at top taper through broad round belly to narrow bottom waistband. Front and rear broadly rounded but narrow waistband, no deep neckline. No head, neck, arms, hands, legs, feet. Match actual low resolution 2008 mobile pixel sprites, deliberate chunky pixels, simple 3 tone shading, dark 1-2 pixel outlines at intended 40px height, NO gradients. Four well separated consistent scale views with generous transparent gutters, equal cells. Transparent background, no labels shadows or extra items.

The sections below record previous generation iterations; the first section describes
the current torso integration.

## Radek

`radek-heads.png` was generated with built-in image_gen, transparent_background=true, using the supplied Radek portrait as identity reference. Four neckless views supply gameplay; the same frontal head supplies both transparent menu sizes. Standard body and uniform. Prompt:

Generate a production pixel-art HEAD sprite sheet based on this man, Radek. Preserve his recognizable bald shaved head (NO hair, NO cap), bright blue eyes, warm light skin, light short brown stubble beard, dark eyebrows and broad friendly toothy smile. Exactly FOUR isolated HEAD ONLY views in ONE horizontal row: strict right-facing profile, three-quarter right, full front, full rear. NO NECK in any view: silhouette ends at chin/jaw, rear ends at skull base just below ears. No neck stubs, no shoulders, no shirt, no body. Compact rounded heads like 2008 Playman mobile athletics game. Chunky low-resolution pixel outlines, simple 3-tone shading suitable for nearest neighbor reduction to about 30x35 pixels. Consistent identity and scale across all four views. Four evenly spaced cells with generous fully transparent gutters and padding. Actual transparent background; no reference UI, frame, labels or text. Frontal view must work as transparent head-selection portrait.

Earlier torso correction replaced the procedural belly described below. `brother-torsos.png` contains four purpose-drawn transparent views, generated with built-in image_gen. The red singlet covers a continuous round belly. Native sizes are 48x40, 48x36, 52x34 and 52x34; original animation centers are preserved. The former exposed-midriff pixel fill is removed.

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

The superseded code-generated brother torso had a 2.6x maximum belly width, a short jersey over a shaded exposed midriff and waistband. Limbs use 1.45x width. Custom atlas cells are 96 pixels to contain the broader silhouettes. Original joint hooks and simulation remain unchanged. The supplied cartoon example guides proportions only.

Kubak menu correction: both preview sizes now sample the transparent neckless frontal head, not the supplied opaque portrait crop. Reference image is retained for provenance only.
