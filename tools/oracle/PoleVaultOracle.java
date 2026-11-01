import org.recompile.mobile.*;
import javax.microedition.lcdui.*;
import javax.imageio.ImageIO;
import java.nio.file.*;
import java.lang.reflect.*;
import java.util.*;
/** Isolated probes of original methods; never modifies the original JAR. */
public class PoleVaultOracle {
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

   field("aM",int.class).setInt(null,3);field("aP",int.class).setInt(null,1);field("aN",int.class).setInt(null,1);field("aZ",int.class).setInt(null,0);field("bc",int.class).setInt(null,10);field("ba",int.class).setInt(null,11);field("bd",int.class).setInt(null,0);field("bi",int.class).setInt(null,12345);
   Class<?> profileType=Class.forName("e",true,game.getClassLoader());
   Object profile=((Object[])field("a",java.lang.reflect.Array.newInstance(profileType,0).getClass()).get(null))[0];ef(profile,"a",int[].class).set(profile,remap);ef(profile,"d",int.class).setInt(profile,100);ef(profile,"c",int.class).setInt(profile,0);
   call("bE");call("bH");
   Class<?> entity=Class.forName("c",true,game.getClassLoader());
   Object athlete=field("e",entity).get(null);
   Method step=method("e",void.class,entity,boolean.class), down=method("h",void.class,entity,int.class,boolean.class), up=method("i",void.class,entity,int.class,boolean.class);
   System.err.println("pole prepare length="+((short[])field("L",short[].class).get(null)).length);
   Image image=Image.createImage(240,320);Graphics graphics=image.getGraphics();
   for(String scenario:effects?new String[]{"clean"}:new String[]{"clean","early","late","no-plant","no-release","plant-release","wrong-run"}) {
    call("bH");athlete=field("e",entity).get(null);for(String reset:new String[]{"E","D","A"})ef(athlete,reset,int.class).setInt(athlete,0);boolean pressed=false,released=false;int landed=-1;
    if(effects){field("a",java.util.Random.class).set(null,new java.util.Random(54321L));call("ae");}
    StringBuilder particles=new StringBuilder("tick,marker,anchor_x,anchor_y,x,y,vx,vy,origin_x,origin_y,born,lifetime\n");
    StringBuilder csv=new StringBuilder("tick,input,phase,phase_ms,pos,speed,height,vertical,expected,r,s,t,u,v,w,x,y,C,B,E,D,z,A,held,camera_offset,camera_x,camera_y,camera_z,camera_angle\n");
    for(int tick=0;tick<1400;tick++) {
     int phase=ef(athlete,"o",int.class).getInt(athlete),key=-1;
     if(phase==1&&tick%7==0){key=ef(athlete,"d",int.class).getInt(athlete)^(scenario.equals("wrong-run")?1:0);down.invoke(null,athlete,key==0?102:104,false);}
     if(!pressed&&phase==2&&!scenario.equals("no-plant")&&ef(athlete,"v",int.class).getInt(athlete)+ef(athlete,"x",int.class).getInt(athlete)>=(scenario.equals("early")?-500000:scenario.equals("late")?0:-100000)){pressed=true;key=10;down.invoke(null,athlete,103,false);}
     if(pressed&&!released&&scenario.equals("plant-release")&&phase==2&&key!=10){key=11;released=true;up.invoke(null,athlete,103,false);}
     if(pressed&&!released&&!scenario.equals("no-release")&&(scenario.equals("early")?phase>=3&&phase<=4:phase==4)&&ef(athlete,"D",int.class).getInt(athlete)>=(scenario.equals("early")?3500:4800)){released=true;key=11;up.invoke(null,athlete,103,false);}
     step.invoke(null,athlete,false);
     if(ef(athlete,"o",int.class).getInt(athlete)==7&&landed<0){int height=ef(athlete,"z",int.class).getInt(athlete);call("j",0,0,height>0?height/855:-1);}
     field("ch",int.class).setInt(null,tick*32);field("bb",int.class).setInt(null,tick*32);
     csv.append(tick+","+key);for(String name:new String[]{"o","p","c","b","k","j","d","r","s","t","u","v","w","x","y","C","B","E","D","z","A"})csv.append(","+ef(athlete,name,int.class).getInt(athlete));csv.append(","+(ef(athlete,"b",boolean.class).getBoolean(athlete)?1:0)+","+value("bP"));
     if(effects)call("af");
     field("b",Graphics.class).set(null,graphics);field("s",int[].class).set(null,remap);graphics.setClip(0,0,240,320);call("a",0,0,240,320);call("bL");
     if(effects) {
       call("ag");
       Class<?> particleType=Class.forName("b",true,game.getClassLoader());
       Object[] pool=(Object[])field("a",java.lang.reflect.Array.newInstance(particleType,0).getClass()).get(null);
       for(Object part:pool)if(ef(part,"a",int.class).getInt(part)==1) {
         particles.append(tick+","+value("bQ")+","+value("aE")+","+value("aF"));
         for(String name:new String[]{"b","c","d","e","f","g","i","j"})particles.append(","+ef(part,name,int.class).getInt(part));particles.append("\n");
       }
     }
     for(String name:new String[]{"bw","bx","by","bz"})csv.append(","+value(name));csv.append("\n");
     if(tick%40==0)ImageIO.write(((PlatformImage)image).getCanvas(),"png",out.resolve(scenario+"-"+tick+".png").toFile());
     if(ef(athlete,"o",int.class).getInt(athlete)==7){if(landed<0)landed=tick;if(tick-landed>40)break;}
    }
    Files.writeString(out.resolve(scenario+".csv"),csv);
    if(effects)Files.writeString(out.resolve("timing-particles.csv"),particles);
   }
   System.out.println("Pole Vault oracle exported: "+out);System.exit(0);
  }catch(Throwable e){e.printStackTrace();System.exit(1);}
 }
 static Field ef(Object o,String name,Class<?> type)throws Exception {for(Field f:o.getClass().getDeclaredFields())if(f.getName().equals(name)&&f.getType()==type){f.setAccessible(true);return f;}throw new NoSuchFieldException(name);}
}
