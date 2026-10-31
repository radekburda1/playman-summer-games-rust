# Event transition effects

Original `d.r(int)` tiles sprites 109–118 in 24-pixel steps, selecting the white overlay from a clamped 16-bit progress value. Event states 10 and 12 drive it over 400 ms. The Rust start/finish overlays now use those original images rather than an invented opacity curve.

Original header helper `d.a(int,String,int,boolean,boolean,boolean,boolean)` moves the icon and red line left and the title right as visibility decreases. Its easing helper `d.c(a,b,t,d)` transforms progress using `(4*t*t/d-t)/3` before interpolation. Rust uses that curve and the original 400 ms duration. The player name strip moves from the left and the numeric result panel from the right. Start and finish confirmations retain the outgoing screen for 400 ms before advancing. Escape cancels pending transitions.

The wash is drawn over the stadium before UI, so the moving overlays stay crisp. Camera, athlete simulation, countdown and stored race result remain separate from transition timing. Full original menu choreography and a live synchronized visual comparison remain unfinished.

Start-confirmation correction: original state 10 invokes the white wash only when entering (`bb < 400`), not when leaving for state 11. Rust now skips the outgoing wash on AwaitStart; confirming the start slides UI away over the unchanged stadium. The initial scene-entry dissolve and idle Start/5 prompt blink remain separate.
