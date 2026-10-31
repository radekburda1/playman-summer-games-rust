import org.recompile.mobile.*;
import javax.microedition.lcdui.*;
import javax.imageio.ImageIO;
import java.nio.file.*;
import java.lang.reflect.*;
import java.util.*;
/** Isolated probes of original methods; never modifies the original JAR. */
public class HurdlesOracle {
 static Class<?> game;
 static Field field(String name,Class<?> type)throws Exception {for(Field f:game.getDeclaredFields())if(f.getName().equals(name)&&f.getType()==type){f.setAccessible(true);return f;}throw new NoSuchFieldException(name+type);}
 static Method method(String name,Class<?> result,Class<?>... args)throws Exception {for(Method m:game.getDeclaredMethods())if(m.getName().equals(name)&&m.getReturnType()==result&&Arrays.equals(m.getParameterTypes(),args)){m.setAccessible(true);return m;}throw new NoSuchMethodException(name);}
 static void call(String name,int... values)throws Exception {Class<?>[] types=new Class<?>[values.length];Arrays.fill(types,int.class);Object[] a=new Object[values.length];for(int i=0;i<a.length;i++)a[i]=values[i];method(name,void.class,types).invoke(null,a);}
 static int value(String name)throws Exception {return field(name,int.class).getInt(null);}
 public static void main(String[] args)throws Exception {
  try {
   Path out=Paths.get(args[1]);Files.createDirectories(out);
   MobilePlatform p=new MobilePlatform(240,320);Mobile.setPlatform(p,()->{});Mobile.maskIndex=0;Mobile.sound=false;p.setPainter(()->java.util.concurrent.locks.LockSupport.parkNanos(16_666_667L));
   if(!p.load(Paths.get(args[0]).toAbsolutePath().toUri().toString()))throw new Exception("load failed");Mobile.maskIndex=0;Mobile.sound=false;p.loader.start();
   for(int n=0;n<100&&(Mobile.getDisplay()==null||Mobile.getDisplay().getCurrent()==null);n++)Thread.sleep(100);
   game=Mobile.getDisplay().getCurrent().getClass();
   for(int key:new int[]{-7,53,-6,-6,-6,-6}){MobilePlatform.keyPressed(key);Thread.sleep(70);MobilePlatform.keyReleased(key);Thread.sleep(850);}
   field("p",boolean.class).setBoolean(null,true);Thread.sleep(250);Mobile.isPaused=false;Thread.sleep(100);
   System.err.println("oracle: engine ready");call("O");

   for(int bank=0;bank<18;bank++)call("c",bank);
   int[] remap=Arrays.copyOf((int[])field("u",int[].class).get(null),29);for(int n=0;n<4;n++){remap[21+n]=794+n;remap[25+n]=612+n;}
   field("s",int[].class).set(null,remap);field("aD",int.class).setInt(null,100);

   field("aM",int.class).setInt(null,2);field("aP",int.class).setInt(null,1);field("aN",int.class).setInt(null,1);field("aO",int.class).setInt(null,0);field("aZ",int.class).setInt(null,0);field("bc",int.class).setInt(null,5);field("ba",int.class).setInt(null,11);
   Class<?> profileType=Class.forName("e",true,game.getClassLoader());
   Object profile=((Object[])field("a",java.lang.reflect.Array.newInstance(profileType,0).getClass()).get(null))[0];
   ef(profile,"a",int[].class).set(profile,remap);ef(profile,"d",int.class).setInt(profile,100);ef(profile,"c",int.class).setInt(profile,0);
   call("aU");
   Class<?> entity=Class.forName("c",true,game.getClassLoader());
   Object athlete=((Object[])field("b",java.lang.reflect.Array.newInstance(entity,0).getClass()).get(null))[0];
   Method init=method("a",void.class,int.class,entity,long.class,int.class,int.class);
   Method step=method("a",void.class,entity,boolean.class);Method input=method("c",void.class,entity,int.class,boolean.class);
   Image image=Image.createImage(240,320);Graphics graphics=image.getGraphics();int errors=0;
   for(int id:new int[]{8192,8195,8196,8197,8200,8201,8202,8204,8206,8207,8208,16384}) {
     field("b",Graphics.class).set(null,graphics);field("s",int[].class).set(null,remap);graphics.setClip(0,0,240,320);graphics.setColor(0xFFFFFF);graphics.fillRect(0,0,240,320);call("a",0,0,240,320);call("a",id,0,0,0,120,160,1024,0);
     ImageIO.write(((PlatformImage)image).getCanvas(),"png",out.resolve("asset-"+id+".png").toFile());
   }
   field("b",Graphics.class).set(null,graphics);graphics.setClip(0,0,240,320);graphics.setColor(0xffffff);graphics.fillRect(0,0,240,320);call("a",8201,0,0,0,-257,173,1024,0);ImageIO.write(((PlatformImage)image).getCanvas(),"png",out.resolve("asset-negative-8201.png").toFile());
   for(String scenario:new String[]{"clean","wrong","incomplete","boundary","dive","late-dive","idle"}) {
    init.invoke(null,0,athlete,12345L,0,2);field("bE",int.class).setInt(null,3);field("bG",int.class).setInt(null,0);
    if(scenario.equals("clean")) {
      for(int stage=0;stage<3;stage++) {
        field("ba",int.class).setInt(null,stage==0?10:11);field("bE",int.class).setInt(null,stage==2?2:1);field("bF",int.class).setInt(null,0);field("ch",int.class).setInt(null,0);
        field("b",Graphics.class).set(null,graphics);field("s",int[].class).set(null,remap);graphics.setClip(0,0,240,320);call("a",0,0,240,320);call("ba");
        ImageIO.write(((PlatformImage)image).getCanvas(),"png",out.resolve("start-stage-"+stage+".png").toFile());
        Files.writeString(out.resolve("start-stage-"+stage+".json"),"{\"ba\":"+(stage==0?10:11)+",\"stage\":"+(stage==2?2:1)+",\"stage_ms\":0}");
      }
      field("ba",int.class).setInt(null,11);field("bE",int.class).setInt(null,3);
    }
    StringBuilder trace=new StringBuilder("tick,input,pos,speed,slot,mistakes,previous_mistakes,fallen,dive,seq0,seq1,seq2,camera_x,camera_y,camera_z,camera_angle\n");
    boolean dived=false;
    for(int tick=0;tick<1400;tick++) {
     int pos=ef(athlete,"c",int.class).getInt(athlete);int slot=ef(athlete,"l",int.class).getInt(athlete);int[] sequence=(int[])ef(athlete,"b",int[].class).get(athlete);
     int key=-1;
     if(tick%7==0&&slot<3)key=sequence[slot]^(scenario.equals("wrong")?1:0);
     if(scenario.equals("incomplete")&&slot==2)key=-1;
     if(scenario.equals("boundary")&&tick%2==0&&slot<3)key=sequence[slot];
     if(scenario.equals("idle"))key=-1;
     if(pos>=0x780000)key=-1;
     if(!dived&&((scenario.equals("dive")&&pos>=0x640000)||(scenario.equals("late-dive")&&pos>=0x740000))){key=10;dived=true;}
     if(key>=0)input.invoke(null,athlete,key==10?103:key==0?102:104,false);
     field("bF",int.class).setInt(null,(tick+1)*32);field("ch",int.class).setInt(null,tick*32);step.invoke(null,athlete,false);
     field("b",Graphics.class).set(null,graphics);field("s",int[].class).set(null,remap);graphics.setClip(0,0,240,320);call("a",0,0,240,320);
     try{call("ba");}catch(Throwable e){if(errors++==0)e.printStackTrace();}
     trace.append(tick+","+key);for(String name:new String[]{"c","b","l","m","n","o","f"})trace.append(","+ef(athlete,name,int.class).getInt(athlete));
     for(int v:sequence)trace.append(","+v);for(String name:new String[]{"bw","bx","by","bz"})trace.append(","+value(name));trace.append("\n");
     if(tick%40==0||pos>=0x780000){ImageIO.write(((PlatformImage)image).getCanvas(),"png",out.resolve(scenario+"-"+tick+".png").toFile());Files.writeString(out.resolve(scenario+"-"+tick+".json"),"{\"tick\":"+tick+",\"pos\":"+ef(athlete,"c",int.class).getInt(athlete)+",\"speed\":"+ef(athlete,"b",int.class).getInt(athlete)+",\"fallen\":"+ef(athlete,"o",int.class).getInt(athlete)+"}");}
     if(pos>=0x780000&&ef(athlete,"b",int.class).getInt(athlete)<0&&ef(athlete,"p",int.class).getInt(athlete)>1200)break;
    }
    Files.writeString(out.resolve(scenario+".csv"),trace);
   }
   System.out.println("Hurdles oracle exported: "+out+" render errors="+errors);System.exit(0);
  }catch(Throwable e){e.printStackTrace();System.exit(1);}
 }

 static Field ef(Object o,String name,Class<?> type)throws Exception {for(Field f:o.getClass().getDeclaredFields())if(f.getName().equals(name)&&f.getType()==type){f.setAccessible(true);return f;}throw new NoSuchFieldException(name);}
}
