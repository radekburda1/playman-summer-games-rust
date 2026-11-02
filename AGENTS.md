# Agent instructions

## Initialize before working

Read `DEVELOPER.md`, `README.md`, and `docs/CURRENT-STATE.md`, then inspect
`git status`, the working diff, and relevant untracked files before editing.
The state document records the latest handoff, asset pipeline, verification,
known limitations, and local browser workflow. Confirm its claims against the
current checkout; snapshots can become stale.

Use this existing checkout. Preserve all uncommitted work, including untracked
custom assets and implementation files. Do not stage, commit, push, reset, or
regenerate reference fixtures unless explicitly requested. Commit preparation
means review, cleanup, validation, and a suggested message unless the user also
requests the commit itself.

## Implementation

Preserve the dependency boundaries documented in `DEVELOPER.md`. Simulation
must remain independent of graphics, platform, application, filesystem, and
device APIs. Cosmetic appearance must not change scoring, physics, input RNG,
or replay mechanics. Preserve original artwork and default reference pixels.

Custom heads have no neck extensions. Menu portraits must be transparent
head-only previews. Check generated sheet segmentation, all required views,
transforms, and animation anchors; valid assets and passing tests alone do not
establish visual correctness. Follow the available image-generation skill for
raster generation/edits, keep runtime assets inside the repository, and record
the prompts and provenance in `assets/custom/README.md`.

## Verification and communication

Run checks appropriate to the change; commands and existing toolchain setup are
documented in `docs/CURRENT-STATE.md`. Inspect appearance changes in the live
game. Distinguish automated checks, live browser observations, user-confirmed
audio, and unverified native/event behavior. Do not claim complete parity from
build success. Do not overwrite oracle fixtures to hide failures.

Work autonomously on authorized reversible changes. Keep progress updates
concise. Do not delegate to subagents unless the user or applicable instructions
explicitly request it.

## Keep the handoff current

Update `docs/CURRENT-STATE.md` when meaningful work changes implementation,
verification results, known limitations, or the next task. Keep current behavior
clear and remove superseded claims rather than accumulating contradictory
notes. Put lasting architecture details in `DEVELOPER.md`, asset provenance in
`assets/custom/README.md`, and durable agent rules here. A state snapshot is
context, not authorization to invent new work.
