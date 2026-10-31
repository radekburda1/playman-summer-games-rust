# Dash starting scene

Reference: `reference-runner/captures/manual-1793460930584.png` and its state JSON (camera x=0, y=1792, distance=2048, perspective=8192).

Recovered from original `d.aR()`: each of six lane centers is `(-6 + lane) * 333 + 166`. Sprite 709 is drawn at the projected start point minus 6 horizontally, plus 5 vertically, with Java ME RIGHT|BOTTOM anchoring. Sprite 710 uses the same baseline and an additional `56 + lane * 28 / 6` horizontal offset. The Rust scene now uses these original sprites, coordinates and anchors. Equipment remains on the track as the camera moves away, and is omitted once the starting area is outside the original camera threshold.

A separate AwaitStart screen now follows instructions, retries and subsequent attempts. It renders the stationary starting camera and pose, NEXT UP header and Start/5 prompt. Only confirmation begins Ready and its countdown; the simulation does not step while waiting. This also avoids playing the countdown cue before confirmation.

This change addresses the starting equipment and confirmation flow. Full visual parity, including the original foreground official, remains unfinished.

The later `manual-1793461319147` frame shows the hidden phase of the waiting prompt. The original global blink flag toggles when more than 300 ms have passed (`d.j()`); the Start label, 5 prompt and selected-lane shade now share a 300 ms on / 300 ms off cycle. A separate presentation timer advances while waiting, leaving the camera, countdown and simulation untouched.
