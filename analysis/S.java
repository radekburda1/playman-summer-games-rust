/*
 * Decompiled with CFR 0.152.
 * 
 * Could not load the following classes:
 *  javax.microedition.lcdui.Display
 *  javax.microedition.lcdui.Displayable
 *  javax.microedition.midlet.MIDlet
 */
import javax.microedition.lcdui.Display;
import javax.microedition.lcdui.Displayable;
import javax.microedition.midlet.MIDlet;

/*
 * Duplicate member names - consider using --renamedupmembers true
 */
public final class S
extends MIDlet {
    public static d a;
    public static Display a;

    public final void startApp() {
        a.setCurrent((Displayable)a);
        d.p = false;
    }

    public final void pauseApp() {
        if (a != null) {
            a.hideNotify();
        }
        this.notifyPaused();
        d.p = true;
    }

    public final void destroyApp(boolean bl) {
        if (a != null) {
            d.a();
        }
        if ((a = null) != null) {
            a.setCurrent(null);
        }
        a = null;
        this.notifyDestroyed();
    }

    public final void a() {
        this.destroyApp(false);
    }

    public S() {
        a = Display.getDisplay((MIDlet)this);
        try {
            a = new d(this);
            d.a.start();
            return;
        }
        catch (Exception exception) {
            Exception exception2 = exception;
            exception.printStackTrace();
            return;
        }
    }
}

