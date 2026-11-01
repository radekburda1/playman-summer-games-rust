import org.recompile.mobile.*;
import javax.microedition.lcdui.*;
import javax.imageio.ImageIO;
import java.nio.file.*;
import java.lang.reflect.*;
import java.util.*;
/** Isolated probes of original methods; never modifies the original JAR. */
public class JavelinOracle {
 static Class<?> game;
 static Field field(String name,Class<?> type)throws Exception {for(Field f:game.getDeclaredFields())if(f.getName().equals(name)&&f.getType()==type){f.setAccessible(true);return f;}throw new NoSuchFieldException(name+type);}
 static Method method(String name,Class<?> result,Class<?>... args)throws Exception {for(Method m:game.getDeclaredMethods())if(m.getName().equals(name)&&m.getReturnType()==result&&Arrays.equals(m.getParameterTypes(),args)){m.setAccessible(true);return m;}throw new NoSuchMethodException(name);}
 static void call(String name,int... values)throws Exception {Class<?>[] types=new Class<?>[values.length];Arrays.fill(types,int.class);Object[] a=new Object[values.length];for(int i=0;i<a.length;i++)a[i]=values[i];method(name,void.class,types).invoke(null,a);}
 static int value(String name)throws Exception {return field(name,int.class).getInt(null);}
 public static void main(String[] args)throws Exception {
  try {
   boolean effects=args.length>2&&args[2].equals("--effects");
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

   field("aM",int.class).setInt(null,4);field("aP",int.class).setInt(null,1);field("aN",int.class).setInt(null,1);field("aZ",int.class).setInt(null,0);field("bc",int.class).setInt(null,10);field("ba",int.class).setInt(null,11);field("bd",int.class).setInt(null,0);field("bi",int.class).setInt(null,12345);
   Class<?> profileType=Class.forName("e",true,game.getClassLoader());Object profile=((Object[])field("a",java.lang.reflect.Array.newInstance(profileType,0).getClass()).get(null))[0];ef(profile,"a",int[].class).set(profile,remap);ef(profile,"d",int.class).setInt(profile,100);ef(profile,"c",int.class).setInt(profile,0);
   call("bq");call("bs");
   Files.writeString(out.resolve("controls.json"),"{\"fire_skips_flight\":"+method("a",boolean.class,int.class,int.class).invoke(null,103,196616)+",\"five_skips_flight\":"+method("a",boolean.class,int.class,int.class).invoke(null,53,196616)+"}");
   Class<?> entity=Class.forName("c",true,game.getClassLoader());
   Method step=method("c",void.class,entity,boolean.class),down=method("f",void.class,entity,int.class,boolean.class),up=method("g",void.class,entity,int.class,boolean.class);
   Image image=Image.createImage(240,320);Graphics graphics=image.getGraphics();
   for(String scenario:effects?new String[]{"valid"}:new String[]{"valid","early","late","foul","missed","wrong-run","idle","skip"}) {
    call("bt");Object athlete=field("d",entity).get(null);for(String reset:new String[]{"r","s","t","u","v","x","A","B","C","z"})ef(athlete,reset,int.class).setInt(athlete,0);
    if(effects){call("ae");field("a",java.util.Random.class).set(null,new java.util.Random(54321L));}
    StringBuilder particles=new StringBuilder("tick,preset,x,y,vx,vy,origin_x,origin_y,born,lifetime\n");
    StringBuilder csv=new StringBuilder("tick,input,phase,phase_ms,pos,speed,height,vertical,expected,r,s,t,u,v,x,A,B,C,z,foul,camera_x,camera_y,camera_z,camera_angle\n");int done=-1;
    for(int tick=0;tick<1200;tick++) {
     int phase=ef(athlete,"o",int.class).getInt(athlete),pos=ef(athlete,"c",int.class).getInt(athlete),speed=ef(athlete,"b",int.class).getInt(athlete),key=-1;
     int duration=scenario.equals("early")?320:scenario.equals("late")?1600:1088;
     if((phase==1||phase==2)&&tick%7==0&&!scenario.equals("idle")){key=ef(athlete,"d",int.class).getInt(athlete)^(scenario.equals("wrong-run")?1:0);down.invoke(null,athlete,key==0?102:104,false);}
     if(phase==3&&!scenario.equals("missed")&&!scenario.equals("idle")&&pos>=-15*speed+(scenario.equals("foul")?65536:-65536)-duration/32*speed){key=10;down.invoke(null,athlete,103,false);}
     if(phase==4&&ef(athlete,"p",int.class).getInt(athlete)>=duration){key=11;up.invoke(null,athlete,103,false);}
     if(scenario.equals("skip")&&phase==6){key=12;call("O",103);}
     int previous=ef(athlete,"o",int.class).getInt(athlete);
     if(effects&&(previous==4||previous==5))field("bM",int.class).setInt(null,value("bM")+1);
     if(effects)field("ch",int.class).setInt(null,tick*32);
     step.invoke(null,athlete,false);field("ch",int.class).setInt(null,tick*32);field("bb",int.class).setInt(null,tick*32);
     int state=ef(athlete,"o",int.class).getInt(athlete);
     if((state==7||state==8)&&done<0){done=tick;int distance=ef(athlete,"z",int.class).getInt(athlete);call("j",0,0,ef(athlete,"a",boolean.class).getBoolean(athlete)?-1:distance*100/88576);}
     if(effects) {
       if(previous==5&&state==6){int angle=ef(athlete,"v",int.class).getInt(athlete)>>11;call("a",value("aE"),value("aF"),2,0,0,0,0,angle*angle>>10,-1,-1);}
       call("af");
     }
     field("b",Graphics.class).set(null,graphics);field("s",int[].class).set(null,remap);graphics.setClip(0,0,240,320);call("a",0,0,240,320);call("bx");
     if(effects) {
       call("ag");Class<?> particleType=Class.forName("b",true,game.getClassLoader());Object[] pool=(Object[])field("a",java.lang.reflect.Array.newInstance(particleType,0).getClass()).get(null);
       for(Object part:pool){int kind=ef(part,"a",int.class).getInt(part);if(kind>=0){particles.append(tick+","+kind);for(String name:new String[]{"b","c","d","e","f","g","i","j"})particles.append(","+ef(part,name,int.class).getInt(part));particles.append("\n");}}
     }
     csv.append(tick+","+key);for(String name:new String[]{"o","p","c","b","k","j","d","r","s","t","u","v","x","A","B","C","z"})csv.append(","+ef(athlete,name,int.class).getInt(athlete));csv.append(","+(ef(athlete,"a",boolean.class).getBoolean(athlete)?1:0));for(String name:new String[]{"bw","bx","by","bz"})csv.append(","+value(name));csv.append("\n");
     if(tick%40==0)ImageIO.write(((PlatformImage)image).getCanvas(),"png",out.resolve(scenario+"-"+tick+".png").toFile());
     if(done>=0&&tick-done>40)break;
    }
    Files.writeString(out.resolve(scenario+".csv"),csv);
    if(effects)Files.writeString(out.resolve("particles.csv"),particles);
   }
   System.out.println("Javelin oracle exported: "+out);System.exit(0);
  }catch(Throwable e){e.printStackTrace();System.exit(1);}
 }
 static Field ef(Object o,String name,Class<?> type)throws Exception {for(Field f:o.getClass().getDeclaredFields())if(f.getName().equals(name)&&f.getType()==type){f.setAccessible(true);return f;}throw new NoSuchFieldException(name);}
}
