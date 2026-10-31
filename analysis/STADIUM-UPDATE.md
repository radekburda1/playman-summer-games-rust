# Dash stadium layers

The original `d.E(int)` repeats scene 8204 (grandstand/crowd) at depth -20480 with no extra spacing and an animation interval of 120 ms. Rust previously inserted 120 pixels of spacing and always selected frame zero. It now repeats without gaps and interpolates the four original frames. Scenery uses a presentation clock, so spectators continue animating while waiting for the start confirmation.

The omitted scene 8206 supplies the advertising strip below the grandstand. Its horizontal depth is -18432, while its vertical anchor is inherited from the grandstand projection, matching `d.g(8206,-18432,0,bB)`. The omitted scene 8200 repeats at depth 512 and supplies the foreground Playman/Summer Games branding. The gameplay shortcut footer was removed to leave those lower assets visible; keyboard controls are unchanged.

The upper HUD, character customization and remaining full-scene differences are outside this change. These fixes were traced from the original renderer and supplied visual comparison; a new live synchronized frame comparison has not been performed.

Finish-area update: all six lanes now receive the original number sprites at finish minus 384 world units and three inset marking rows at finish minus 1600, with 416-unit spacing. Markings use the original 2x2 pixel line offsets and appear past camera x=21120. The dash omits the full-width 90 m line as in Java. Sprites 711 and 712 (photo-finish equipment and bell) use original bottom-center anchors at the projected finish point at depth -2126, with 32 screen pixels between them.

Start official and lane separators: the six-lane track now explicitly draws each white boundary so far-lane separators survive rounding of thin color bands. The original starter scene 16386 is rendered after the athlete at world (512,0,512), with frame 0 while waiting, frames 1/2 during Ready/Set, and the original 3–7 firing transition after Go (32 ms hold followed by 200 ms interpolations).
