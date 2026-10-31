# Menu and number feedback update

Menus now use the original extracted bitmap fonts (font tables 3–6 in resource `i`), sport icons and white/red styling. This covers event selection, instructions, pause, results and tournament completion. Header margins and the personal-record border were checked in the running WebAssembly build in Edge. The event menu supports 4/6 navigation and 5 selection, including keypad equivalents. The pause menu exposes resume, retry, sound and main menu actions.

The running prompt renderer follows `d.a(c,int,int)` and `d.i(int,int,int)` in the decompiled original: correct input draws sprites 474–476 for 200 ms; the next standard running prompt bounces for 100 ms; incorrect input draws sprite 477 for 300 ms. Feedback stays on the pressed side or consumed hurdle sequence slot. Inputs ignored in the final dash section do not generate a false highlight. The simulation clock drives these effects, so pausing freezes their age.

Validation: 17 tests pass, strict Clippy passes, native release and WebAssembly release builds succeed. The new five-test suite is `tests/menu_feedback.rs`. Event-menu navigation, instructions and a running event were visually checked in Edge. Native interactive execution and full synchronized Java/Rust traces remain unverified; existing parity limits still apply.

Prompt-anchor correction: original race drawing records the prompt position after `d.B(athlete_position)` and before applying the early running camera offset. Rust now uses that unshifted race camera for running prompts, while retaining the offset camera for athlete and stadium rendering. A regression test verifies stable prompt x coordinates throughout the first 20 metres. Later perspective changes and the original short correct-input bounce remain intact.
