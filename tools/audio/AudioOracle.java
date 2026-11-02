import org.recompile.mobile.Mobile;
import org.recompile.mobile.MobilePlatform;
import javax.microedition.media.Player;
import java.lang.reflect.Array;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.reflect.Proxy;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;

/** Observe Player.start calls from the unmodified JAR in an isolated emulator. */
public class AudioOracle {
    private static Class<?> game;
    private static int tick;
    private static String scenario;
    private static final StringBuilder trace = new StringBuilder("scenario,tick,resource\n");

    private static Field field(Class<?> type, String name, Class<?> fieldType) throws Exception {
        for (Field field : type.getDeclaredFields()) {
            if (field.getName().equals(name) && field.getType() == fieldType) {
                field.setAccessible(true);
                return field;
            }
        }
        throw new NoSuchFieldException(name + fieldType);
    }

    private static Field field(String name, Class<?> type) throws Exception {
        return field(game, name, type);
    }

    private static Method method(String name, Class<?>... types) throws Exception {
        for (Method method : game.getDeclaredMethods()) {
            if (method.getName().equals(name) && method.getReturnType() == void.class
                    && Arrays.equals(method.getParameterTypes(), types)) {
                method.setAccessible(true);
                return method;
            }
        }
        throw new NoSuchMethodException(name);
    }

    private static void call(String name, int... values) throws Exception {
        Class<?>[] types = new Class<?>[values.length];
        Arrays.fill(types, int.class);
        method(name, types).invoke(null, Arrays.stream(values).boxed().toArray());
    }

    private static void set(String name, int value) throws Exception {
        field(name, int.class).setInt(null, value);
    }

    private static void entity(Object entity, String name, int value) throws Exception {
        field(entity.getClass(), name, int.class).setInt(entity, value);
    }

    private static int entity(Object entity, String name) throws Exception {
        return field(entity.getClass(), name, int.class).getInt(entity);
    }

    private static void observePlayers() throws Exception {
        call("d"); // Initialize the original sound bank.
        short[] mapping = (short[]) field("a", short[].class).get(null);
        Object[] pool = (Object[]) field("a", Object[].class).get(null);
        for (int resource = 0; resource < 27; resource++) {
            final int id = resource;
            pool[mapping[1214 + resource]] = Proxy.newProxyInstance(
                    Player.class.getClassLoader(), new Class<?>[] {Player.class},
                    (proxy, method, args) -> {
                        if (method.getName().equals("start")) {
                            trace.append(scenario).append(',').append(tick)
                                    .append(',').append(id).append('\n');
                        }
                        Class<?> result = method.getReturnType();
                        if (result == long.class) return 0L;
                        if (result == int.class) return 400;
                        if (result == boolean.class) return false;
                        return null;
                    });
        }
        ((int[]) field("j", int[].class).get(null))[0] = 1;
    }

    private static void initialize(String jar) throws Exception {
        MobilePlatform platform = new MobilePlatform(240, 320);
        Mobile.setPlatform(platform, () -> {});
        Mobile.sound = false;
        platform.setPainter(() -> java.util.concurrent.locks.LockSupport.parkNanos(16_666_667L));
        if (!platform.load(Paths.get(jar).toAbsolutePath().toUri().toString())) {
            throw new IllegalStateException("Could not load original JAR");
        }
        platform.loader.start();
        for (int attempt = 0; attempt < 100
                && (Mobile.getDisplay() == null || Mobile.getDisplay().getCurrent() == null);
                attempt++) {
            Thread.sleep(100);
        }
        game = Mobile.getDisplay().getCurrent().getClass();
        for (int key : new int[] {-7, 53, -6, -6, -6, -6}) {
            MobilePlatform.keyPressed(key);
            Thread.sleep(70);
            MobilePlatform.keyReleased(key);
            Thread.sleep(850);
        }
        field("p", boolean.class).setBoolean(null, true);
        Thread.sleep(300);
        Mobile.isPaused = false;
        call("O");
        for (int bank = 0; bank < 18; bank++) call("c", bank);
        observePlayers();
        field("z", int[].class).set(null, new int[24]);
        field("x", int[].class).set(null, new int[36]);
        field("y", int[].class).set(null, new int[6]);
    }

    private static void probeDash(Class<?> entityType) throws Exception {
        set("aM", 0);
        set("aP", 1);
        set("aN", 1);
        set("aO", 0);
        set("aZ", 0);
        set("ba", 11);
        set("bc", 5);
        set("bi", 12345);
        call("aK");
        call("aO");
        Class<?> arrayType = Array.newInstance(entityType, 0).getClass();
        Object athlete = ((Object[]) field("a", arrayType).get(null))[0];
        scenario = "dash-countdown";
        for (tick = 0; tick < 160; tick++) call("aP");

        scenario = "dash-finish";
        tick = 0;
        set("bs", 3);
        set("ba", 11);
        entity(athlete, "c", 0x640000 - 1000);
        entity(athlete, "b", 16000);
        call("aP");
    }

    private static void probeLongJump(Class<?> entityType) throws Exception {
        // These boundaries share the setup used by the existing Long Jump oracle.
        set("aM", 1);
        set("bc", 10);
        call("bd");
        int action = ((short[]) field("a", short[].class).get(null))[11];
        Method input = method("d", entityType, int.class, boolean.class);
        Method step = method("b", entityType, boolean.class);
        for (String name : new String[] {"jump-valid", "jump-early", "jump-foul"}) {
            set("ba", 11);
            set("bb", 1000);
            field("G", boolean.class).setBoolean(null, true); // Suppress result transitions.
            call("bg");
            Object athlete = field("c", entityType).get(null);
            scenario = name;
            tick = 0;
            entity(athlete, "o", 3);
            entity(athlete, "c", name.equals("jump-foul") ? 1 : -65536);
            entity(athlete, "b", 28000);
            input.invoke(null, athlete, action, true);
            for (tick = 0; tick < 90; tick++) {
                boolean stretch = name.equals("jump-early")
                        ? tick == 3 : entity(athlete, "j") <= 650;
                if (entity(athlete, "o") == 4 && stretch) {
                    input.invoke(null, athlete, action, true);
                }
                step.invoke(null, athlete, true);
            }
        }
    }

    private static void probeResults() throws Exception {
        set("aM", 0);
        set("bc", 5);
        set("aI", 0);
        set("aJ", 0);
        set("aZ", 0);
        set("bd", 0);
        set("v", 32);
        for (String name : new String[] {"result-good", "result-poor", "result-good-record"}) {
            scenario = name;
            tick = 0;
            set("ba", 12);
            field("E", boolean.class).setBoolean(null, name.endsWith("record"));
            field("F", boolean.class).setBoolean(null, false);
            ((int[]) field("z", int[].class).get(null))[3] = name.equals("result-poor") ? 1282 : 965;
            call("K");
            method("a", boolean.class).invoke(null, true);
            for (tick = 1; tick <= 40; tick++) call("L");
        }
    }

    public static void main(String[] args) {
        try {
            if (args.length != 2) {
                throw new IllegalArgumentException("Usage: AudioOracle game.jar output.csv");
            }
            initialize(args[0]);
            Class<?> entityType = Class.forName("c", true, game.getClassLoader());
            probeDash(entityType);
            probeLongJump(entityType);
            Files.writeString(Paths.get(args[1]), trace.toString());
            trace.setLength(0);
            trace.append("scenario,tick,resource\n");
            probeResults();
            Files.writeString(Paths.get(args[1]).toAbsolutePath().getParent().resolve("result-cues.csv"), trace.toString());
            System.out.print(trace);
            System.exit(0);
        } catch (Throwable error) {
            error.printStackTrace();
            System.exit(1);
        }
    }
}
