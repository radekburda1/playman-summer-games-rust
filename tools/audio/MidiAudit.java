import javax.sound.midi.MidiMessage;
import javax.sound.midi.MidiSystem;
import javax.sound.midi.Sequence;
import javax.sound.midi.ShortMessage;
import javax.sound.midi.Track;
import java.nio.file.Paths;
import java.util.Locale;
import java.util.Set;
import java.util.TreeSet;

/** MIDI metadata only; does not open an audio device or synthesizer. */
public class MidiAudit {
    public static void main(String[] args) throws Exception {
        if (args.length != 1) {
            throw new IllegalArgumentException("Usage: MidiAudit sound-directory");
        }
        System.out.println("resource,sequence_seconds,program_changes,note_on_events");
        for (int id = 0; id < 12; id++) {
            Sequence sequence = MidiSystem.getSequence(Paths.get(args[0], id + ".mid").toFile());
            int notes = 0;
            Set<String> programs = new TreeSet<>();
            for (Track track : sequence.getTracks()) {
                for (int i = 0; i < track.size(); i++) {
                    MidiMessage message = track.get(i).getMessage();
                    if (message instanceof ShortMessage event) {
                        if (event.getCommand() == ShortMessage.NOTE_ON && event.getData2() > 0) {
                            notes++;
                        }
                        if (event.getCommand() == ShortMessage.PROGRAM_CHANGE) {
                            programs.add(event.getChannel() + ":" + event.getData1());
                        }
                    }
                }
            }
            System.out.printf(Locale.ROOT, "%d,%.6f,%s,%d%n", id,
                    sequence.getMicrosecondLength() / 1000000., String.join(";", programs), notes);
        }
    }
}
