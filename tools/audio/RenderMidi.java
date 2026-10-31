import javax.sound.midi.*;
import javax.sound.sampled.*;
import java.nio.file.*;
import java.util.*;
/** Offline General MIDI rendering with the JDK's software synthesizer. */
public final class RenderMidi {
 public static void main(String[] args) throws Exception {
  Files.createDirectories(Paths.get(args[1]));
  for(int id=0;id<12;id++) {
   Sequence sequence=MidiSystem.getSequence(Paths.get(args[0],id+".mid").toFile());
   Synthesizer synth=MidiSystem.getSynthesizer();
   AudioFormat format=new AudioFormat(22050,16,2,true,false);
   AudioInputStream stream=(AudioInputStream)synth.getClass().getMethod("openStream",AudioFormat.class,Map.class).invoke(synth,format,Map.of("interpolation","sinc","reverb",true,"chorus",true));
   Receiver receiver=synth.getReceiver();
   ArrayList<MidiEvent> events=new ArrayList<>();
   for(Track track:sequence.getTracks())for(int i=0;i<track.size();i++)events.add(track.get(i));
   events.sort(Comparator.comparingLong(MidiEvent::getTick));
   long previousTick=0;double micros=0;int tempo=500000;
   for(MidiEvent event:events) {
    long tick=event.getTick();
    micros+=(tick-previousTick)*(sequence.getDivisionType()==Sequence.PPQ?(double)tempo/sequence.getResolution():1000000.0/(sequence.getDivisionType()*sequence.getResolution()));
    previousTick=tick;
    MidiMessage message=event.getMessage();
    if(message instanceof MetaMessage meta) {
     if(meta.getType()==0x51){byte[] b=meta.getData();tempo=((b[0]&255)<<16)|((b[1]&255)<<8)|(b[2]&255);}
    } else receiver.send(message,(long)micros);
   }
   long frames=(long)((micros/1000000.0+2)*format.getFrameRate());
   AudioSystem.write(new AudioInputStream(stream,format,frames),AudioFileFormat.Type.WAVE,Paths.get(args[1],id+".wav").toFile());
   receiver.close();synth.close();
   System.out.printf(Locale.ROOT,"%d %.3f seconds%n",id,micros/1000000.0);
  }
 }
}
