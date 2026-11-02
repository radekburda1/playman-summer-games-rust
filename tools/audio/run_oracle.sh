#!/bin/sh
# Execute the unmodified JAR in a disposable emulator session; no audio device is opened.
set -eu
PROJECT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
REFERENCE_DIR=${REFERENCE_DIR:-"$PROJECT_DIR/../reference-runner"}
REFERENCE_DIR=$(CDPATH= cd -- "$REFERENCE_DIR" && pwd)
JAVA=${PLAYMAN_JAVA:-java}
JAR=${1:?Usage: run_oracle.sh game.jar output-directory}
OUT=${2:?Usage: run_oracle.sh game.jar output-directory}
JAR=$(python3 -c 'from pathlib import Path; import sys; print(Path(sys.argv[1]).resolve())' "$JAR")
mkdir -p "$OUT"
OUT=$(CDPATH= cd -- "$OUT" && pwd)
mkdir -p "$OUT/classes"
"$JAVA" -jar "$REFERENCE_DIR/lib/ecj.jar" -21 -cp "$REFERENCE_DIR/lib/freej2me_plus.jar" -d "$OUT/classes" "$PROJECT_DIR/tools/audio/AudioOracle.java" "$PROJECT_DIR/tools/audio/MidiAudit.java"
SESSION=$(mktemp -d "$OUT/session-XXXXXX")
(cd "$SESSION" && "$JAVA" -Dfile.encoding=ISO_8859_1 -Djava.awt.headless=true -cp "$OUT/classes:$REFERENCE_DIR/lib/freej2me_plus.jar" AudioOracle "$JAR" "$OUT/original-cues.csv")
"$JAVA" -cp "$OUT/classes" MidiAudit "$PROJECT_DIR/assets/sounds" > "$OUT/midi.csv"
