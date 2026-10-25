/*
 * Decompiled with CFR 0.152.
 * 
 * Could not load the following classes:
 *  com.nokia.mid.ui.DeviceControl
 *  com.nokia.mid.ui.DirectUtils
 *  javax.microedition.lcdui.Font
 *  javax.microedition.lcdui.Graphics
 *  javax.microedition.lcdui.Image
 *  javax.microedition.lcdui.game.GameCanvas
 *  javax.microedition.media.Manager
 *  javax.microedition.media.Player
 *  javax.microedition.media.control.VolumeControl
 *  javax.microedition.rms.RecordStore
 */
import com.nokia.mid.ui.DeviceControl;
import com.nokia.mid.ui.DirectUtils;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.Random;
import javax.microedition.lcdui.Font;
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;
import javax.microedition.lcdui.game.GameCanvas;
import javax.microedition.media.Manager;
import javax.microedition.media.Player;
import javax.microedition.media.control.VolumeControl;
import javax.microedition.rms.RecordStore;

/*
 * Duplicate member names - consider using --renamedupmembers true
 */
public final class d
extends GameCanvas
implements Runnable {
    public static int[] a;
    public static int a;
    public static int b;
    public static int c;
    public static boolean a;
    public static final byte[] a;
    public static final byte[] b;
    public static final byte[] c;
    public static final byte[] d;
    public static final byte[] e;
    public static short[][] a;
    public static boolean b;
    public static byte[] f;
    public static byte[] g;
    public static int d;
    public static boolean c;
    public static int e;
    public static int f;
    public static byte[] h;
    public static int g;
    public static Image[] a;
    public static int[] b;
    public static int h;
    public static int i;
    public static int j;
    public static byte[] i;
    public static Player a;
    public static int k;
    public static Object[] a;
    public static boolean d;
    public static Font a;
    public static Font b;
    public static Font c;
    public static Font d;
    public static int l;
    public static int m;
    public static int n;
    public static Graphics a;
    public static int o;
    public static int p;
    public static int q;
    public static int r;
    public static int s;
    public static int t;
    public static int u;
    public static int v;
    public static boolean e;
    public static int w;
    public static boolean f;
    public static int x;
    public static int y;
    public static int z;
    public static int A;
    public static int B;
    public static int[] c;
    public static byte[] j;
    public static byte[] k;
    public static short[] a;
    public static int[] d;
    public static boolean g;
    public static boolean h;
    public static byte a;
    public static boolean i;
    public static int C;
    public static DataInputStream a;
    public static String[] a;
    public static int[] e;
    public static boolean[] a;
    public static boolean[] b;
    public static int D;
    public static boolean j;
    public static Graphics b;
    public static boolean k;
    public static boolean l;
    public static ByteArrayOutputStream a;
    public static DataOutputStream a;
    public static ByteArrayInputStream a;
    public static DataInputStream b;
    public static boolean m;
    public static boolean n;
    public static boolean o;
    public static boolean p;
    public static boolean q;
    public static S a;
    public static Thread a;
    public static d a;
    public static int E;
    public static Object a;
    public static boolean r;
    public static boolean s;
    public static boolean t;
    public static boolean u;
    public static boolean v;
    public static boolean w;
    public static int F;
    public static String a;
    public static int[] f;
    public static int[] g;
    public static int[] h;
    public static int G;
    public static int H;
    public static int I;
    public static int J;
    public static int K;
    public static int L;
    public static int M;
    public static int N;
    public static int O;
    public static int P;
    public static int Q;
    public static int R;
    public static int[] i;
    public static String[] b;
    public static String b;
    public static String c;
    public static short[] b;
    public static short[] c;
    public static short[] d;
    public static short[] e;
    public static int S;
    public static int T;
    public static int U;
    public static int V;
    public static int W;
    public static int X;
    public static int Y;
    public static int Z;
    public static int aa;
    public static int ab;
    public static int ac;
    public static int ad;
    public static int ae;
    public static int af;
    public static int ag;
    public static String d;
    public static boolean x;
    public static int ah;
    public static int ai;
    public static boolean y;
    public static String e;
    public static boolean z;
    public static int aj;
    public static int ak;
    public static int al;
    public static int am;
    public static byte[] l;
    public static byte[] m;
    public static boolean A;
    public static short[][] b;
    public static int[] j;
    public static int an;
    public static short[] f;
    public static short[] g;
    public static Random a;
    public static int[] k;
    public static int ao;
    public static int[] l;
    public static int[] m;
    public static int ap;
    public static int aq;
    public static int ar;
    public static int[] n;
    public static int[] o;
    public static short[] h;
    public static a[] a;
    public static e[] a;
    public static int[] p;
    public static int[] q;
    public static int[] r;
    public static int as;
    public static int at;
    public static int au;
    public static int av;
    public static int aw;
    public static a[] b;
    public static int ax;
    public static int ay;
    public static int az;
    public static int aA;
    public static int aB;
    public static int aC;
    public static boolean B;
    public static int[] s;
    public static int aD;
    public static int[] t;
    public static int[] u;
    public static int[] v;
    public static int[] w;
    public static c a;
    public static boolean C;
    public static boolean D;
    public static c b;
    public static short[] i;
    public static short[] j;
    public static short[] k;
    public static short[] l;
    public static short[] m;
    public static short[] n;
    public static short[] o;
    public static short[] p;
    public static short[] q;
    public static short[] r;
    public static short[] s;
    public static short[] t;
    public static short[] u;
    public static short[] v;
    public static short[] w;
    public static short[] x;
    public static short[] y;
    public static short[] z;
    public static short[] A;
    public static short[] B;
    public static short[] C;
    public static short[] D;
    public static short[] E;
    public static short[] F;
    public static short[] G;
    public static short[] H;
    public static short[] I;
    public static short[] J;
    public static short[] K;
    public static short[] L;
    public static short[] M;
    public static short[] N;
    public static short[] O;
    public static short[] P;
    public static short[] Q;
    public static short[] R;
    public static short[] S;
    public static short[] T;
    public static short[] U;
    public static short[] V;
    public static short[] W;
    public static short[] X;
    public static short[] Y;
    public static short[] Z;
    public static short[] aa;
    public static short[] ab;
    public static short[] ac;
    public static short[] ad;
    public static short[] ae;
    public static b[] a;
    public static int aE;
    public static int aF;
    public static int aG;
    public static int aH;
    public static int aI;
    public static int aJ;
    public static int aK;
    public static int aL;
    public static int aM;
    public static int aN;
    public static int aO;
    public static int aP;
    public static int aQ;
    public static int aR;
    public static int[] x;
    public static int[] y;
    public static int aS;
    public static int aT;
    public static int aU;
    public static byte[] n;
    public static byte b;
    public static int aV;
    public static int aW;
    public static int aX;
    public static int aY;
    public static int aZ;
    public static int ba;
    public static int bb;
    public static int bc;
    public static int bd;
    public static boolean E;
    public static boolean F;
    public static String f;
    public static int be;
    public static int bf;
    public static int bg;
    public static int bh;
    public static int bi;
    public static int bj;
    public static int bk;
    public static int bl;
    public static boolean G;
    public static int bm;
    public static int[] z;
    public static int[] A;
    public static int[] B;
    public static int bn;
    public static int bo;
    public static int bp;
    public static int bq;
    public static int br;
    public static int[] C;
    public static int[] D;
    public static int bs;
    public static int bt;
    public static int bu;
    public static int bv;
    public static c[] a;
    public static int bw;
    public static int bx;
    public static int by;
    public static int bz;
    public static int bA;
    public static int bB;
    public static int bC;
    public static int bD;
    public static int bE;
    public static int bF;
    public static int bG;
    public static int bH;
    public static c[] b;
    public static c c;
    public static c[] c;
    public static int bI;
    public static int bJ;
    public static c d;
    public static c[] d;
    public static int bK;
    public static int bL;
    public static int bM;
    public static int bN;
    public static int bO;
    public static c e;
    public static c[] e;
    public static int bP;
    public static int bQ;
    public static int bR;
    public static int bS;
    public static int bT;
    public static int bU;
    public static int bV;
    public static int bW;
    public static int bX;
    public static int bY;
    public static int bZ;
    public static boolean H;
    public static int ca;
    public static boolean I;
    public static boolean J;
    public static int cb;
    public static int cc;
    public static int cd;
    public static int ce;
    public static int cf;
    public static boolean K;
    public static boolean L;
    public static boolean M;
    public static int cg;
    public static boolean N;
    public static int[] E;
    public static int ch;
    public static int ci;
    public static int cj;
    public static int ck;
    public static int[] F;
    public static int[] G;
    public static int cl;
    public static int cm;
    public static int cn;
    public static boolean O;
    public static boolean P;
    public static boolean Q;
    public static int co;
    public static int cp;
    public static int cq;
    public static int cr;
    public static int cs;
    public static int ct;
    public static int cu;
    public static boolean R;
    public static int cv;
    public static int cw;
    public static int cx;
    public static int cy;
    public static boolean S;
    public static int cz;
    public static int cA;
    public static int cB;
    public static int cC;
    public static boolean T;
    public static int cD;
    public static boolean U;
    public static int[] H;
    public static String[] c;

    private static void b() {
        a = new int[256];
        int n = 0;
        while (n < 256) {
            int n2 = n;
            int n3 = 0;
            while (n3 < 8) {
                n2 = (n2 & 1) != 0 ? 0xEDB88320 ^ n2 >>> 1 : (n2 >>>= 1);
                ++n3;
            }
            d.a[n] = n2;
            ++n;
        }
    }

    private static void a(byte[] byArray, int n) {
        if (a == null) {
            d.b();
        }
        int n2 = 0;
        while (n2 < n) {
            a = a[(a ^ byArray[n2]) & 0xFF] ^ a >>> 8;
            ++n2;
        }
    }

    private static void a(int n, int n2) {
        d.f[n] = (byte)(n2 >>> 24);
        d.f[n + 1] = (byte)(n2 >>> 16 & 0xFF);
        d.f[n + 2] = (byte)(n2 >>> 8 & 0xFF);
        d.f[n + 3] = (byte)(n2 & 0xFF);
    }

    private static void b(byte[] byArray, int n) {
        int n2 = 0;
        while (n2 < n) {
            d.g[d.b++] = byArray[n2];
            ++n2;
        }
    }

    private static void a(int n) {
        d.g[d.b++] = (byte)(n >>> 24);
        d.g[d.b++] = (byte)(n >>> 16);
        d.g[d.b++] = (byte)(n >>> 8);
        d.g[d.b++] = (byte)n;
    }

    private static void c(byte[] byArray, int n) {
        a = -1;
        d.a(n);
        d.b(byArray, 4);
        d.b(f, n);
        d.a(byArray, 4);
        d.a(f, n);
        d.a(~a);
    }

    private static int a(int n) throws Exception {
        int n2;
        int n3;
        int n4;
        int n5;
        b = 0;
        byte[] byArray = new byte[]{-119, 80, 78, 71, 13, 10, 26, 10};
        d.b(byArray, 8);
        int n6 = c ? f : e;
        int n7 = c ? e : f;
        int n8 = d;
        d.a(0, n6);
        d.a(4, n7);
        d.f[8] = 8;
        boolean bl = false;
        if (n8 == 255) {
            bl = true;
        }
        d.f[9] = (a = bl) ? 6 : 3;
        d.f[10] = 0;
        d.f[11] = 0;
        d.f[12] = 0;
        d.c(a, 13);
        if (n8 != 255) {
            n5 = 1;
            while (n5 < a[n8].length) {
                n4 = a[n8][n5];
                n3 = n4 >> 11 & 0x1F;
                d.f[n5 * 3] = (byte)((n3 << 3) + (n3 >> 2));
                n3 = n4 >> 5 & 0x3F;
                d.f[n5 * 3 + 1] = (byte)((n3 << 2) + (n3 >> 4));
                n3 = n4 & 0x1F;
                d.f[n5 * 3 + 2] = (byte)((n3 << 3) + (n3 >> 2));
                ++n5;
            }
            d.c(b, a[n8].length * 3 + 3);
            d.c(c, 1);
        }
        h = n - g;
        d.f[0] = 120;
        d.f[1] = -38;
        d.c(e, 2);
        n5 = 1;
        n4 = 0;
        int n9 = n3 = (n6 + 1) * n7;
        int n10 = 0;
        do {
            int n11;
            n3 = n9;
            if (n8 == 255 && n3 > 1015) {
                n3 = 1015;
            } else if (n3 > 4087) {
                n3 = 4087;
            }
            n9 -= n3;
            if (n8 == 255) {
                n2 = 0;
                n11 = 0;
                while (n11 < n3) {
                    int n12 = 0;
                    if (n10 % (n6 + 1) != 0) {
                        int n13 = g[n];
                        int n14 = g[++n];
                        int n15 = g[++n];
                        int n16 = g[++n];
                        ++n;
                        if (n16 < 0) {
                            n16 += 256;
                        }
                        if (n15 < 0) {
                            n15 += 256;
                        }
                        if (n14 < 0) {
                            n14 += 256;
                        }
                        if (n13 < 0) {
                            n13 += 256;
                        }
                        d.f[n2 + 5] = (byte)n15;
                        n4 += (n5 += n15);
                        n5 %= 65521;
                        n4 %= 65521;
                        d.f[++n2 + 5] = (byte)n14;
                        n4 += (n5 += n14);
                        n5 %= 65521;
                        n4 %= 65521;
                        d.f[++n2 + 5] = (byte)n13;
                        n4 += (n5 += n13);
                        n5 %= 65521;
                        n4 %= 65521;
                        ++n2;
                        n12 = n16;
                    }
                    ++n10;
                    d.f[n2 + 5] = (byte)n12;
                    if (n12 < 0) {
                        n12 += 256;
                    }
                    n4 += (n5 += n12);
                    n5 %= 65521;
                    n4 %= 65521;
                    ++n11;
                    ++n2;
                }
                n3 = n2;
            } else {
                n2 = 0;
                while (n2 < n3) {
                    n11 = 0;
                    if (n10 % (n6 + 1) != 0) {
                        n11 = g[n];
                        ++n;
                    }
                    ++n10;
                    d.f[n2 + 5] = (byte)n11;
                    if (n11 < 0) {
                        n11 += 256;
                    }
                    n4 += (n5 += n11);
                    n5 %= 65521;
                    n4 %= 65521;
                    ++n2;
                }
            }
            d.f[0] = n9 == 0 ? (byte)1 : 0;
            d.f[1] = (byte)(n3 & 0xFF);
            d.f[2] = (byte)(n3 >>> 8 & 0xFF);
            d.f[3] = (byte)((char)-1 - n3 & 0xFF);
            d.f[4] = (byte)((char)-1 - n3 >>> 8 & 0xFF);
            d.c(e, n3 + 5);
        } while (n9 > 0);
        n2 = (n4 << 16) + n5;
        d.a(0, n2);
        d.c(e, 4);
        d.c(d, 0);
        c = b;
        return n;
    }

    private static void c() {
        if (b) {
            return;
        }
        try {
            d.a("p", 0L);
            int n = a.readByte();
            if (n < 0) {
                n += 256;
            }
            a = new short[n][];
            int n2 = 0;
            while (n2 < n) {
                int n3 = a.readByte();
                if (n3 < 0) {
                    n3 += 256;
                }
                d.a[n2] = new short[n3];
                int n4 = 1;
                while (n4 < n3) {
                    d.a[n2][n4] = a.readShort();
                    ++n4;
                }
                ++n2;
            }
            b = true;
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static Image a(byte[] byArray, int n, int n2) {
        Image image = Image.createImage((byte[])byArray, (int)n, (int)n2);
        return image;
    }

    private static void b(int n, int n2) {
        g = n;
        int n3 = 0;
        while (n3 < n2) {
            d.h[n3] = g[n3 + n];
            ++n3;
        }
    }

    private static void a(boolean bl, boolean bl2) {
        c = false;
        int n = 1;
        int n2 = 0;
        int n3 = g + h;
        int n4 = h;
        if (bl) {
            n = -1;
            n3 += a ? (e - 1) * 4 : e - 1;
            if (!bl2) {
                n2 = e * 2;
            }
        }
        if (bl2) {
            n3 += a ? e * (f - 1) * 4 : e * (f - 1);
            if (!bl) {
                n2 = -e * 2;
            }
        }
        int n5 = 0;
        while (n5 < f) {
            int n6 = 0;
            while (n6 < e) {
                d.g[n3] = h[n4];
                if (a) {
                    d.g[n3 + 1] = h[n4 + 1];
                    d.g[n3 + 2] = h[n4 + 2];
                    d.g[n3 + 3] = h[n4 + 3];
                    n3 += n * 4;
                    n4 += 4;
                } else {
                    n3 += n;
                    ++n4;
                }
                ++n6;
            }
            n3 += a ? n2 * 4 : n2;
            ++n5;
        }
    }

    private static void b(boolean bl, boolean bl2) {
        c = true;
        d.g[d.g] = h[2];
        d.g[d.g + 1] = h[3];
        d.g[d.g + 2] = h[0];
        d.g[d.g + 3] = h[1];
        int n = f;
        int n2 = -e * f + 1;
        int n3 = g + h;
        int n4 = h;
        if (bl && bl2) {
            n = -f;
            n2 = f * e - 1;
            n3 += a ? (e * f - 1) * 4 : e * f - 1;
        } else if (bl) {
            n2 = -f * e - 1;
            n3 += a ? (f - 1) * 4 : f - 1;
        } else if (bl2) {
            n2 = f * e + 1;
            n = -f;
            n3 += a ? f * (e - 1) * 4 : f * (e - 1);
        }
        int n5 = 0;
        while (n5 < f) {
            int n6 = 0;
            while (n6 < e) {
                d.g[n3] = h[n4];
                if (a) {
                    d.g[n3 + 1] = h[n4 + 1];
                    d.g[n3 + 2] = h[n4 + 2];
                    d.g[n3 + 3] = h[n4 + 3];
                    n3 += n * 4;
                    n4 += 4;
                } else {
                    n3 += n;
                    ++n4;
                }
                ++n6;
            }
            n3 += a ? n2 * 4 : n2;
            ++n5;
        }
    }

    private static void b(int n) throws Exception {
        int n2 = 0;
        while (n2 < h) {
            d.g[n2 + d.g] = h[n2];
            ++n2;
        }
        switch (n) {
            case 4: {
                d.a(true, false);
                break;
            }
            case 5: {
                d.a(false, true);
                break;
            }
            case 2: {
                d.a(true, true);
                break;
            }
            case 1: {
                d.b(true, false);
                break;
            }
            case 3: {
                d.b(false, true);
                break;
            }
            case 6: {
                d.b(false, false);
                break;
            }
            case 7: {
                d.b(true, true);
            }
        }
        d.a(g);
    }

    private static int a(int n, int n2) {
        int n3 = a[374 + n];
        if (n2 != 0) {
            int n4;
            while ((n4 = b[n3]) != 15) {
                ++n3;
                if (n4 != n2) continue;
                return n3;
            }
        }
        return n3;
    }

    private static int b(int n) {
        n = d.a(n, 0);
        return a[n].getHeight();
    }

    private static int c(int n) {
        n = d.a(n, 0);
        return a[n].getWidth();
    }

    private static void c(int n) {
        int n2 = a[355 + n];
        d.c();
        if (a[n2] != null) {
            return;
        }
        try {
            d.a("g" + n, 0L);
            while (n2 < a[355 + (n + 1)]) {
                d.c(2, n2);
                n2 = d.d(n2);
            }
            d.p();
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static int d(int n) {
        try {
            e = a.readUnsignedShort();
            f = a.readUnsignedShort();
            d = a.readUnsignedByte();
            int n2 = (d == 255 ? e * 4 : e) * f;
            int n3 = 89159 - n2 - 1;
            d.a(n3, n2, g);
            c = false;
            d.b(n3, n2);
            d.a(n3);
            d.a[n] = d.a(g, 0, c);
            byte by = a.readByte();
            int n4 = by & 0xF;
            if (n4 > 0) {
                int[] nArray = new int[n4];
                int n5 = 0;
                while (n5 < n4) {
                    byte by2 = a.readByte();
                    nArray[n5] = by2;
                    ++n5;
                }
                n5 = 0;
                while (n5 < n4) {
                    d.b(nArray[n5]);
                    d.a[n + 1] = d.a(g, 0, c);
                    d.b[n] = nArray[n5];
                    ++n;
                    ++n5;
                }
                d.b[n] = 15;
            }
        }
        catch (Exception exception) {}
        return n + 1;
    }

    private static void a(int n, int n2, int n3, int n4, int n5) {
        int n6 = d.c(n);
        int n7 = d.b(n);
        if (n5 == 1 || n5 == 3 || n5 == 6 || n5 == 7) {
            int n8 = n6;
            n6 = n7;
            n7 = n8;
        }
        n = d.a(n, n5);
        b.drawImage(a[n], n2 -= d.b(n4, n6), n3 -= d.c(n4, n7), 0);
    }

    private static String a(int n) {
        switch (n) {
            case 1: {
                return "audio/midi";
            }
            case 4: {
                return "audio/amr";
            }
            case 3: {
                return "audio/x-wav";
            }
            case 9: {
                return "audio/mpeg";
            }
            case 16: {
                return "audio/mp4";
            }
            case 15: {
                return "audio/aac";
            }
        }
        return null;
    }

    private static void d(int n) {
        int n2;
        if (!d) {
            d.d();
        }
        if ((n2 = (n * 100 >> 6) * 256 >> 8) > 100) {
            n2 = 100;
        } else if (n2 < 0) {
            n2 = 0;
        }
        k = n2;
        int n3 = 0;
        while (n3 < 27) {
            if (a[n3] instanceof Player) {
                try {
                    ((VolumeControl)((Player)a[n3]).getControl("VolumeControl")).setLevel(k);
                }
                catch (Exception exception) {}
            }
            ++n3;
        }
        if (a instanceof Player) {
            try {
                ((VolumeControl)a.getControl("VolumeControl")).setLevel(k);
                return;
            }
            catch (Exception exception) {}
        }
    }

    private static void d() {
        i = new byte[1];
        a = new Object[27];
        int n = 0;
        while (n < 1) {
            d.i[n] = 0;
            ++n;
        }
        d = true;
    }

    private static void e() {
        if (!d) {
            d.d();
        }
        int n = 0;
        while (n < 1) {
            if (i[n] == 1) {
                int n2 = a[1213 + n];
                short s = a[1212 + n];
                int n3 = d[3 + n2];
                d.a("s", (long)n3);
                System.gc();
                int n4 = n2;
                while (n4 < n2 + s) {
                    block13: {
                        d.c(3, n4);
                        byte by = k[907 + n4];
                        int n5 = d[65 + n4];
                        short s2 = a[1214 + n4];
                        if (by != 1 && by != 4 && by != 3 && by != 9 && by != 16 && by != 15 || a[s2] == null) {
                            try {
                                byte[] byArray = new byte[n5];
                                d.a(0, n5, byArray);
                                try {
                                    if (by == 0) {
                                        d.a[s2] = null;
                                        break block13;
                                    }
                                    if (by != 3) {
                                        d.a[s2] = byArray;
                                        break block13;
                                    }
                                    d.a[s2] = Manager.createPlayer((InputStream)new ByteArrayInputStream(byArray), (String)d.a(by));
                                    ((Player)a[s2]).prefetch();
                                    try {
                                        ((VolumeControl)((Player)a[s2]).getControl("VolumeControl")).setLevel(k);
                                    }
                                    catch (Exception exception) {
                                    }
                                }
                                catch (Exception exception) {}
                            }
                            catch (Exception exception) {}
                        }
                    }
                    ++n4;
                }
                d.i[n] = 2;
            }
            ++n;
        }
    }

    private static void e(int n) {
        if (!d) {
            d.d();
        }
        if (i[n] > 0) {
            return;
        }
        d.i[n] = 1;
        if (j[0] != 0) {
            d.e();
        }
    }

    private static void a(int n, int n2, int n3) {
        byte by = k[907 + n];
        if (j[0] == 0 || by == 0) {
            return;
        }
        if (by != 3 && n2 < j && t < i) {
            return;
        }
        if (by != 3) {
            i = n3 == 0 ? Integer.MAX_VALUE : t + d[34 + n] * n3;
            j = n2;
        }
        short s = a[1214 + n];
        if (by == 1 || by == 4 || by == 3 || by == 9 || by == 16 || by == 15) {
            try {
                if (a[s] instanceof Player) {
                    if (n3 != 1) {
                        try {
                            ((Player)a[s]).setLoopCount(n3 == 0 ? -1 : n3);
                        }
                        catch (Exception exception) {}
                    }
                    ((Player)a[s]).setMediaTime(0L);
                    ((Player)a[s]).start();
                } else {
                    if (a != null) {
                        a.close();
                    }
                    a = Manager.createPlayer((InputStream)new ByteArrayInputStream((byte[])a[s]), (String)d.a(by));
                    a.realize();
                    try {
                        ((VolumeControl)a.getControl("VolumeControl")).setLevel(k);
                    }
                    catch (Exception exception) {}
                    a.prefetch();
                    if (n3 != 1) {
                        a.setLoopCount(n3 == 0 ? -1 : n3);
                    }
                    a.setMediaTime(0L);
                    a.start();
                }
                return;
            }
            catch (Exception exception) {}
        }
    }

    private static void f(int n) {
        short s = a[1214 + n];
        byte by = k[907 + n];
        if (!d) {
            return;
        }
        if (by == 1 || by == 4 || by == 3 || by == 9 || by == 16 || by == 15) {
            if (by != 3) {
                i = -2147483647;
            }
            try {
                if (a[s] instanceof Player) {
                    ((Player)a[s]).stop();
                } else if (a != null) {
                    a.close();
                }
                return;
            }
            catch (Exception exception) {}
        }
    }

    private static void g(int n) {
        if (i[n] == 2) {
            int n2 = a[1213 + n];
            short s = a[1212 + n];
            int n3 = n2;
            while (n3 < n2 + s) {
                d.f(n3);
                ++n3;
            }
        }
    }

    private static void f() {
        if (d) {
            int n = 0;
            while (n < 1) {
                d.g(n);
                ++n;
            }
        }
    }

    private static int b(int n, int n2) {
        if ((n & 1) != 0) {
            return n2 >> 1;
        }
        if ((n & 8) != 0) {
            return n2;
        }
        return 0;
    }

    private static int c(int n, int n2) {
        if ((n & 2) != 0) {
            return n2 >> 1;
        }
        if ((n & 0x20) != 0) {
            return n2;
        }
        return 0;
    }

    private static void a(int n, String string, int n2, int n3, int n4) {
        if ((n & 0x7F) >= 3) {
            n2 -= d.b(n4, d.a(n, string));
            n3 -= d.c(n4, d.e(n));
        }
        if ((n & 0x2000) != 0) {
            string = string.toUpperCase();
        }
        if ((n & 0x4000) != 0) {
            string = string.toLowerCase();
        }
        d.a(b, n & 0x7F);
        int n5 = b.getColor();
        b.setColor(l);
        if ((n & 0x400) != 0) {
            if ((n & 0x7F) >= 3) {
                d.a(n & 0x7F, string, n2 - 1, n3);
            } else {
                b.drawString(string, n2 - 1, n3, n4);
            }
            if ((n & 0x7F) >= 3) {
                d.a(n & 0x7F, string, n2, n3 - 1);
            } else {
                b.drawString(string, n2, n3 - 1, n4);
            }
            if ((n & 0x7F) >= 3) {
                d.a(n & 0x7F, string, n2 + 1, n3);
            } else {
                b.drawString(string, n2 + 1, n3, n4);
            }
            if ((n & 0x7F) >= 3) {
                d.a(n & 0x7F, string, n2, n3 + 1);
            } else {
                b.drawString(string, n2, n3 + 1, n4);
            }
        }
        b.setColor(n5);
        if ((n & 0x7F) >= 3) {
            d.a(n & 0x7F, string, n2, n3);
            return;
        }
        b.drawString(string, n2, n3, n4);
    }

    private static int a(int n, String string) {
        if ((n & 0x2000) != 0) {
            string = string.toUpperCase();
        }
        if ((n & 0x4000) != 0) {
            string = string.toLowerCase();
        }
        if ((n & 0x7F) >= 3) {
            return d.b(n & 0x7F, string);
        }
        Font font = d.a(n);
        return font.stringWidth(string);
    }

    private static Font a(int n) {
        switch (n &= 0x7F) {
            case 16: {
                return d;
            }
            case 0: {
                return a;
            }
            case 1: {
                return b;
            }
            case 2: {
                return c;
            }
        }
        return null;
    }

    private static void a(Graphics graphics, int n) {
        if (a == graphics && d.n == s && m == (n &= 0x7F)) {
            return;
        }
        a = graphics;
        d.n = s;
        m = n;
        Font font = d.a(n);
        graphics.setFont(font);
    }

    private static int e(int n) {
        if ((n &= 0x7F) >= 3) {
            return k[903 + (n - 3)];
        }
        switch (n) {
            case 16: {
                return d.getHeight();
            }
            case 0: {
                return a.getHeight();
            }
            case 1: {
                return b.getHeight();
            }
            case 2: {
                return c.getHeight();
            }
        }
        return -1;
    }

    private static void a(int n, int n2, int n3, int n4) {
        o = n;
        p = n2;
        q = n3;
        r = n4;
        d.g();
    }

    private static void g() {
        b.setClip(o, p, q, r);
    }

    private static void b(int n, int n2, int n3, int n4) {
        int n5 = n + n3;
        int n6 = n2 + n4;
        if (n < o) {
            n = o;
        }
        if (n2 < p) {
            n2 = p;
        }
        if (n5 > q + o) {
            n5 = q + o;
        }
        if (n6 > r + p) {
            n6 = r + p;
        }
        b.setClip(n, n2, n5 - n, n6 - n2);
    }

    private static void b(int n, int n2, int n3) {
        if (n3 < n) {
            int n4 = n;
            n = n3;
            n3 = n4;
        }
        b.fillRect(n, n2, n3 - n, 1);
    }

    private static void c(int n, int n2, int n3, int n4) {
        int n5 = 0;
        while (n5 <= 1) {
            int n6 = 0;
            while (n6 <= 1) {
                b.drawLine(n + n5, n2 + n6, n3 + n5, n4 + n6);
                ++n6;
            }
            ++n5;
        }
    }

    private static void h() {
        B = (int)System.currentTimeMillis();
        t = 0;
        x = 0;
        y = 0;
        d.i();
    }

    private static void i() {
        s = 0;
        z = 0;
        v = 0;
        u = 0;
        f = false;
        t = (int)System.currentTimeMillis() - B;
        int n = 0;
        while (n < 4) {
            d.c[n] = 0;
            ++n;
        }
        e = true;
        w = 0;
    }

    private static void j() {
        ++s;
        y = t;
        x = (int)System.currentTimeMillis() - B;
        int n = x - y;
        int n2 = 0;
        while (n2 < 3) {
            d.c[n2] = c[n2 + 1];
            ++n2;
        }
        d.c[3] = x;
        if (z < 4) {
            ++z;
        }
        A = z > 1 ? (x - c[4 - z]) / (z - 1) : 0;
        v = t;
        t += A;
        t += x - t >> 2;
        v = t - v;
        if (n < -1000 || n > 1000) {
            v = 10;
        }
        if (v < 0) {
            v = 0;
        }
        if (d.n && !f) {
            u += v;
        }
        if (t - w > 300) {
            e ^= true;
            w = t;
        }
    }

    private static void k() {
        j = d.a("ropt");
        if (j == null || j.length != 27) {
            j = new byte[27];
            int n = 0;
            while (n < 27) {
                d.j[n] = 0;
                ++n;
            }
            d.j[0] = 1;
            d.j[1] = 1;
            d.j[2] = 0;
        }
        d.j[0] = 0;
        a = j[2];
        d.j[2] = 0;
    }

    private static void l() {
        d.a("ropt", j);
    }

    private static void m() {
        try {
            d.a("i", 0L);
            k = new byte[938];
            d.a(0, 938, k);
            a = new short[1778];
            int n = 0;
            while (n < 1778) {
                d.a[n] = a.readShort();
                ++n;
            }
            d = new int[324];
            n = 0;
            while (n < 324) {
                d.d[n] = a.readInt();
                ++n;
            }
            d.p();
            return;
        }
        catch (IOException iOException) {
            return;
        }
    }

    private static void n() {
        if (o) {
            if (i) {
                String string = a[19];
                int n = 0;
                while (n < 1 + C % 3) {
                    string = string + '.';
                    ++n;
                }
                ++C;
                n = d.a(0, a[19] + "...");
                n += n >> 2;
                int n2 = a.getHeight();
                int n3 = 160 - (n2 >> 1) - n2;
                b.setColor(0);
                b.fillRect(120 - (n >> 1) - 1, n3 + 2, n, n2 << 1);
                b.setColor(0xFFFFFF);
                b.fillRect(120 - (n >> 1), n3, n, n2 << 1);
                b.setColor(0);
                b.drawRect(120 - (n >> 1), n3, n, n2 << 1);
                d.a(0, string, 120 - (d.a(0, a[19] + "...") >> 1), 160, 36);
                i = false;
            } else {
                d.bZ();
            }
            return;
        }
        if (z) {
            d.Z();
            return;
        }
        if (!n) {
            d.E();
            return;
        }
        d.cm();
    }

    private static void c(int n, int n2) {
        if (!g) {
            return;
        }
        d.j();
        d.bX();
    }

    private static void o() {
        i = true;
        d.t();
    }

    public final void hideNotify() {
        if (t && !u) {
            a.keyPressed(a[17] - 50);
        }
        d.f();
        if (n) {
            d.w();
        }
        p = true;
    }

    public final void showNotify() {
        d.q();
        q = true;
        if (t && !u) {
            a.keyPressed(a[17] - 50);
        }
        p = false;
    }

    private static byte[] a(String string) {
        RecordStore recordStore = null;
        byte[] byArray = null;
        try {
            recordStore = RecordStore.openRecordStore((String)string, (boolean)false);
            byArray = recordStore.getRecord(1);
        }
        catch (Exception exception) {}
        try {
            if (recordStore != null) {
                recordStore.closeRecordStore();
            }
        }
        catch (Exception exception) {}
        return byArray;
    }

    private static boolean a(String string, byte[] byArray) {
        RecordStore recordStore = null;
        boolean bl = true;
        try {
            recordStore = RecordStore.openRecordStore((String)string, (boolean)true);
            ByteArrayOutputStream byteArrayOutputStream = new ByteArrayOutputStream();
            byteArrayOutputStream.write(byArray, 0, byArray.length);
            byte[] byArray2 = byteArrayOutputStream.toByteArray();
            if (recordStore.getNumRecords() == 0) {
                recordStore.addRecord(byArray2, 0, byArray2.length);
            } else {
                recordStore.setRecord(1, byArray2, 0, byArray2.length);
            }
        }
        catch (Exception exception) {
            bl = false;
        }
        try {
            if (recordStore != null) {
                recordStore.closeRecordStore();
            }
        }
        catch (Exception exception) {}
        return bl;
    }

    private static void a(String string) {
        try {
            RecordStore.deleteRecordStore((String)string);
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void p() {
        if (a != null) {
            try {
                a.close();
            }
            catch (Exception exception) {}
            a = null;
        }
    }

    private static void a(String string, long l) {
        try {
            d.c(4, (int)l);
            d.p();
            DataInputStream dataInputStream = new DataInputStream(a.getClass().getResourceAsStream("/" + string));
            while (l > 0L) {
                l -= dataInputStream.skip(l);
            }
            a = dataInputStream;
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void a(int n, int n2, byte[] byArray) throws IOException {
        while (n2 > 0) {
            int n3 = a.read(byArray, n, n2);
            n += n3;
            n2 -= n3;
        }
    }

    private static byte[] a(int n, int n2, int n3, byte[] byArray) {
        if (byArray == null || byArray.length < n3) {
            byArray = new byte[n3];
        }
        try {
            d.a("b" + n, (long)n2);
            d.a(0, n3, byArray);
            d.p();
        }
        catch (IOException iOException) {}
        return byArray;
    }

    private static int[] a(int n, int n2, int n3, int[] nArray) {
        if (nArray == null || nArray.length < n3 >> 2) {
            nArray = new int[n3 >> 2];
        }
        byte[] byArray = d.a(n, n2, n3, null);
        int n4 = 0;
        while (n4 < n3) {
            nArray[n4 >> 2] = ((byArray[n4] & 0xFF) << 24) + ((byArray[n4 + 1] & 0xFF) << 16) + ((byArray[n4 + 2] & 0xFF) << 8) + (byArray[n4 + 3] & 0xFF);
            n4 += 4;
        }
        return nArray;
    }

    private static short[] a(int n, int n2, int n3, short[] sArray) {
        if (sArray == null || sArray.length < n3 >> 1) {
            sArray = new short[n3 >> 1];
        }
        byte[] byArray = d.a(n, n2, n3, null);
        int n4 = 0;
        while (n4 < n3) {
            sArray[n4 >> 1] = (short)(((byArray[n4] & 0xFF) << 8) + (byArray[n4 + 1] & 0xFF));
            n4 += 2;
        }
        return sArray;
    }

    private static void h(int n) {
        if (j[1] != 0) {
            try {
                DeviceControl.stopVibra();
                DeviceControl.startVibra((int)50, (long)n);
                return;
            }
            catch (Exception exception) {}
        }
    }

    private static void i(int n) {
        if (a[a[351 + n]] != null) {
            return;
        }
        try {
            d.a("l", (long)d[0]);
            int n2 = 0;
            while (n2 < 219) {
                d.a[n2] = a.readUTF();
                ++n2;
            }
            d.p();
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static String a(String string, String string2, int n) {
        int n2 = string.indexOf("%" + n);
        if (n2 == -1) {
            return string;
        }
        String string3 = string.substring(0, n2) + string2 + string.substring(n2 + 2, string.length());
        return string3;
    }

    private static void q() {
        D = 0;
        int n = 0;
        while (n < 111) {
            d.a[n] = false;
            ++n;
        }
    }

    /*
     * WARNING - Removed try catching itself - possible behaviour change.
     * Enabled aggressive block sorting
     * Enabled unnecessary exception pruning
     * Enabled aggressive exception aggregation
     */
    public final void keyPressed(int n) {
        boolean bl;
        block6: {
            E = (int)System.currentTimeMillis();
            Object object = a;
            synchronized (object) {
                a.notifyAll();
                // MONITOREXIT @DISABLED, blocks:[0, 1] lbl6 : MonitorExitStatement: MONITOREXIT : var2_2
                bl = false;
                if (n + 50 < 111) break block6;
            }
            bl = true;
        }
        boolean bl2 = false;
        if (n < -50) {
            bl2 = true;
        }
        if (bl | bl2) {
            return;
        }
        j = true;
        if (D >= 19) {
            d.q();
        }
        d.e[d.D] = n += 50;
        d.b[d.D] = true;
        ++D;
    }

    public final void keyReleased(int n) {
        boolean bl = false;
        if (n + 50 >= 111) {
            bl = true;
        }
        boolean bl2 = false;
        if (n < -50) {
            bl2 = true;
        }
        if (bl | bl2) {
            return;
        }
        j = true;
        if (D >= 19) {
            d.q();
        }
        d.e[d.D] = n += 50;
        d.b[d.D] = false;
        ++D;
    }

    private static void r() {
        int n = 0;
        while (n < D) {
            int n2 = e[n];
            if (b[n]) {
                d.a[n2] = true;
                if (z) {
                    d.n(n2);
                } else if (!d.n) {
                    d.k(n2);
                } else {
                    d.Z(n2);
                }
            } else if (a[n2]) {
                d.a[n2] = false;
                if (!z && d.n) {
                    d.aa(n2);
                }
            }
            ++n;
        }
        D = 0;
    }

    private static boolean a(int n, int n2) {
        if (n2 < 65536) {
            boolean bl = false;
            if (n == n2) {
                bl = true;
            }
            return bl;
        }
        int n3 = 0;
        while (n3 < n2 >> 16) {
            if (a[(n2 & (char)-1) + n3] == n) {
                return true;
            }
            ++n3;
        }
        return false;
    }

    private static int f(int n) {
        int n2 = n - 98;
        if (n2 < 0 || n2 > 9) {
            return -1;
        }
        return n2;
    }

    private void s() {
        l = false;
        k = true;
        this.repaint();
        this.serviceRepaints();
    }

    public final void paint(Graphics graphics) {
        m = -1;
        if (!k) {
            return;
        }
        k = false;
        DirectUtils.getDirectGraphics((Graphics)graphics);
        b = graphics;
        d.n();
        l = true;
    }

    private static void t() {
        if (!l) {
            return;
        }
        q = false;
        o = true;
        a.s();
        do {
            try {
                Thread.yield();
            }
            catch (Exception exception) {}
        } while (!l);
        o = false;
    }

    private static boolean a(String string) {
        try {
            if (a.platformRequest(string)) {
                d.v();
            }
            return true;
        }
        catch (Exception exception) {
            return false;
        }
    }

    private static void u() {
        if (g) {
            return;
        }
        d.h();
        d.k();
        d.e(0);
        d.x();
        d.ci();
        d.i(0);
        g = true;
        d.c(0);
        h = true;
    }

    /*
     * WARNING - Removed try catching itself - possible behaviour change.
     * Enabled aggressive block sorting
     * Enabled unnecessary exception pruning
     * Enabled aggressive exception aggregation
     */
    public final void run() {
        g = false;
        R = -1;
        l = true;
        d.m();
        d.u();
        m = false;
        n = false;
        o = false;
        q = false;
        z = false;
        i = false;
        p = false;
        d.y();
        d.q();
        while (true) {
            block33: {
                if (a == null) {
                    return;
                }
                r = false;
                do {
                    if (!p) {
                        d.j();
                        d.r();
                        if (a == null) {
                            return;
                        }
                        break block33;
                    }
                    try {
                        Thread.sleep(100L);
                    }
                    catch (Exception exception) {}
                } while (a != null);
                return;
            }
            if (z) {
                q = true;
            } else if (n) {
                if (!d.e()) {
                    if (a == null) {
                        return;
                    }
                    d.ck();
                    d.i();
                    m = false;
                    n = false;
                    q = false;
                    d.y();
                    d.q();
                }
            } else if (!d.a()) {
                if (a == null) {
                    return;
                }
                q = false;
                n = true;
                if (!m) {
                    d.z();
                    d.i();
                    m = true;
                    d.cj();
                } else if (d != null) {
                    d.a(aa, ae, d, af, ag, false, x);
                }
                d.q();
            }
            if (a == null) {
                return;
            }
            if (q) {
                a.s();
                do {
                    j = false;
                    try {
                        Thread.yield();
                    }
                    catch (Exception exception) {}
                } while (!l || j);
                q = false;
            } else {
                if (a == null) {
                    return;
                }
                try {
                    Thread.yield();
                }
                catch (Exception exception) {}
            }
            boolean bl = false;
            if (E + 10000 < (int)System.currentTimeMillis()) {
                bl = true;
            }
            if (!((n ^ true) & r & bl)) continue;
            Object object = a;
            synchronized (object) {
                try {
                    a.wait(10000L);
                }
                catch (Exception exception) {
                    E = (int)System.currentTimeMillis();
                }
            }
        }
    }

    public static void a() {
        a = null;
        d.f();
        d.l();
    }

    private static void v() {
        a.a();
    }

    private static void w() {
        d.f();
        n = false;
        d.y();
    }

    private static void c(int n, int n2, int n3) {
        d.a[1332 + n * 4] = (short)n2;
        d.a[1332 + (n * 4 + 1)] = (short)n3;
    }

    private static void x() {
        t = true;
        M = -1;
        v = false;
        w = false;
        K = -1;
        N = -1;
        u = false;
    }

    private static void y() {
        if (t) {
            d.c(1);
            d.i(1);
            d.j[0] = 0;
            d.a(9);
            return;
        }
        s = false;
        d.A();
        if (m) {
            if (N != -1) {
                d.l(N);
                N = -1;
            } else {
                d.l(2);
            }
            d.f();
            return;
        }
        d.c(2);
        d.i(2);
        if (!w) {
            d.cd();
            w = true;
        }
        d.l(0);
        if (M != -1) {
            d.l(M);
            M = -1;
        }
    }

    private static void z() {
        a = null;
    }

    /*
     * Unable to fully structure code
     */
    private static boolean a() {
        if (d.t) {
            if (!d.u) {
                if (!d.v) {
                    d.ca();
                    d.v = true;
                }
                d.cc();
            }
            d.q = true;
            return true;
        }
        if (!d.s) ** GOTO lbl12
        return false;
lbl-1000:
        // 1 sources

        {
            --d.L;
lbl12:
            // 2 sources

            ** while (d.L > d.H)
        }
lbl13:
        // 2 sources

        while (d.L + d.P <= d.H) {
            ++d.L;
        }
        d.cg();
        d.q = true;
        return true;
    }

    private static boolean a(int n) {
        short s = a[1332 + (n * 4 + 1)];
        if (!u && s == 1) {
            u = true;
            F = n;
            a = d.c(n);
            if (a == null) {
                a = d.a(a[4], a[a[1332 + (n * 4 + 3)]], 1);
            }
            d.a(J == 1 || u ? 3 : 8195, a, 226, 319 - d.e(0) - 1 - 22 - 56, false);
            Q = 0;
            return false;
        }
        return true;
    }

    private static void d(int n, int n2) {
        short s = a[1332 + n * 4];
        short s2 = a[1332 + (n * 4 + 1)];
        if (s == 1 && n2 == 1) {
            d.l(s2);
        }
        if (s == 4 && n2 == 1) {
            d.j[s2] = (byte)(1 - j[s2]);
            d.l();
            if (s2 == 0) {
                d.f();
                d.a(30, 128, 1);
            }
            if (s2 == 1 && j[1] != 0) {
                d.h(200);
            }
        }
        if (s == 5) {
            if (n2 == 1) {
                d.f(s2, 100);
            }
            if (n2 == 2) {
                d.f(s2, 1);
            }
            if (n2 == 3) {
                d.f(s2, -1);
            }
            if (s2 == 0 && j[s2] == 0) {
                d.f();
            }
            d.l();
        }
        if (s == 2 && n2 == 1 && d.a(n)) {
            d.j(n);
        }
        if (s == 3 && n2 == 1 && d.a(n)) {
            d.V(n);
        }
    }

    private static void j(int n) {
        if (n == 11) {
            s = true;
        }
        if (n == 5) {
            d.v();
        }
        if (n == 13) {
            d.C();
        }
        if (n == 14) {
            d.d(F, 1);
            u = false;
        }
        if (n == 15) {
            u = false;
        }
        if (n == 7) {
            d.F();
        }
        if (n == 9) {
            d.j[2] = a;
            if (j[2] == 0) {
                d.j[2] = 64;
            }
            d.j[0] = 1;
            d.d(j[2]);
            d.e();
        }
    }

    private static void k(int n) {
        if (s) {
            return;
        }
        if (u) {
            if (d.a(n, 196625)) {
                d.a(28, 128, 1);
                d.d(14, 1);
            }
            if (d.a(n, 43)) {
                d.a(28, 128, 1);
                d.d(15, 1);
            }
            if (d.a(n, 131085) && --Q < 0) {
                Q = 0;
            }
            if (d.a(n, 131087) && ++Q >= ac - 1) {
                Q = ac - 1;
            }
            return;
        }
        if (t) {
            if (v && d.a(n, 262166)) {
                t = false;
                d.y();
                d.q();
            }
            return;
        }
        switch (J) {
            case 0: 
            case 5: {
                int n2 = 0;
                if (d.a(n, 131085)) {
                    n2 = -1;
                    d.a(27, 128, 1);
                }
                if (d.a(n, 131087)) {
                    n2 = 1;
                    d.a(27, 128, 1);
                }
                while (n2 != 0) {
                    e = true;
                    w = t;
                    if ((H += n2) < 0) {
                        H = I - 1;
                    }
                    if (H >= I) {
                        H = 0;
                    }
                    n2 = 0;
                }
                int n3 = h[H];
                if (a[1332 + (n3 * 4 + 2)] != 0 || !d.a(n, 196625) && !d.a(n, 131098) && !d.a(n, 131100)) break;
                int n4 = 1;
                if (d.a(n, 196625)) {
                    d.a(28, 128, 1);
                } else if (d.a(n, 131098)) {
                    n4 = 2;
                } else if (d.a(n, 131100)) {
                    n4 = 3;
                }
                d.d(n3, n4);
                break;
            }
            case 1: {
                if (d.a(n, 131087)) {
                    ++O;
                    d.a(27, 128, 1);
                }
                if (d.a(n, 131085)) {
                    --O;
                    d.a(27, 128, 1);
                }
                if (O > ad) {
                    O = ad;
                }
                if (O >= 0) break;
                O = 0;
                break;
            }
            case 2: {
                d.p(n, f[G]);
                if (d.a(n, 131085) || d.a(n, 131087)) {
                    e = true;
                    w = t;
                }
                return;
            }
        }
        if (d.a(n, 43)) {
            d.a(29, 128, 1);
            if (G == 0) {
                if (m) {
                    d.d(11, 1);
                } else {
                    d.d(5, 1);
                }
                return;
            }
            d.d(13, 1);
        }
    }

    private static void A() {
        G = -1;
    }

    private static void B() {
        int n = f[G];
        short s = a[1316 + n];
        J = a[1245 + s];
        K = a[1245 + (s + 1)];
        I = 0;
        L = 0;
        P = (319 - d.e(0) - 1 - 22 - 56) / 22;
        P = J == 5 ? (319 - d.e(0) - 1 - 22 - 56) / d.d() : (319 - d.e(0) - 1 - 22 - 56) / 22;
        if (J == 0 || J == 5) {
            int n2 = -1;
            int n3 = 3;
            while (n3 < a[1245 + (s + 2)] + 3) {
                int n4 = a[1245 + (s + n3)];
                if (n4 != -1 && a[1332 + (n4 * 4 + 2)] != 2) {
                    d.h[d.I] = n4;
                    if (n2 == -1 && a[1332 + (n4 * 4 + 2)] == 0) {
                        n2 = I;
                    }
                    ++I;
                }
                ++n3;
            }
            if (a[1332 + (h[H] * 4 + 2)] != 0 && n2 > -1) {
                H = n2;
            }
        } else if (J == 1) {
            String string = "";
            short s2 = a[1245 + (s + 2)];
            short s3 = a[1245 + (s + 3)];
            switch (s2) {
                case 1: {
                    string = a[s3];
                    if (n != 3) break;
                    string = string + "\n\n" + a[212];
                    break;
                }
                case 2: {
                    if (s3 >= 256) break;
                    string = d.b(s3);
                }
            }
            d.a(J == 1 || u ? 3 : 8195, string, 226, 319 - d.e(0) - 1 - 22 - 56, false);
            O = 0;
            ++I;
        } else if (J == 2) {
            d.W(n);
        }
        d.Y(n);
    }

    private static void l(int n) {
        if (G >= 0) {
            d.g[d.G] = H;
        }
        d.f[++d.G] = n;
        H = 0;
        d.B();
    }

    private static void C() {
        H = g[--G];
        d.B();
    }

    private static void c(boolean bl, boolean bl2) {
        if (i) {
            return;
        }
        int n = 320 - (319 - d.e(0)) >> 1;
        int n2 = 320 - n;
        int n3 = n;
        b.setColor(0);
        if (bl) {
            DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(120 - n3, n2 - 1, 120, n2 - n - 1, 120 + n3, n2 - 1, b.getColor() & 0xFFFFFF | 0xFF000000);
        }
        if (bl2) {
            DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(120 - n3, n2 + 1, 120, n2 + n + 1, 120 + n3, n2 + 1, b.getColor() & 0xFFFFFF | 0xFF000000);
        }
        --n2;
        b.setColor(0xFFFFFF);
        if (bl) {
            DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(121 - n3, n2 - 1, 121, n2 - n - 1, 121 + n3, n2 - 1, b.getColor() & 0xFFFFFF | 0xFF000000);
        }
        if (bl2) {
            DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(121 - n3, n2 + 1, 121, n2 + n + 1, 121 + n3, n2 + 1, b.getColor() & 0xFFFFFF | 0xFF000000);
        }
    }

    private static void a(boolean bl, boolean bl2, int n, int n2) {
        if (i) {
            return;
        }
        int n3 = n2 + 11;
        int n4 = n >> 1;
        int n5 = 120 - n4;
        int n6 = 120 + n4;
        boolean bl3 = false;
        if (n5 - 11 < 0) {
            n5 = 11;
            bl3 = true;
        }
        if (n6 + 11 >= 240) {
            n6 = 229;
            bl3 = true;
        }
        if (!bl3 || e) {
            b.setColor(0xFFFFFF);
            if (bl) {
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n5 - 1, n3 - 11, n5 - 11, n3, n5 - 1, n3 + 11, b.getColor() & 0xFFFFFF | 0xFF000000);
            }
            if (bl2) {
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n6 + 1, n3 - 11, n6 + 11, n3, n6 + 1, n3 + 11, b.getColor() & 0xFFFFFF | 0xFF000000);
            }
        }
    }

    private static void e(int n, int n2) {
        if (i) {
            return;
        }
        l = 0;
        b.setColor(0xFF0000);
        if (n != -1) {
            d.a(0, a[n], 3, 319 - d.e(0), 20);
        }
        if (n2 != -1) {
            d.a(0, a[n2], 237, 319 - d.e(0), 24);
        }
    }

    private static void D() {
        l = 0;
        b.setColor(16715817);
        d.a(8196, a[K], 60, 3, 20);
    }

    private static void E() {
        d.a(0, 0, 240, 320);
        if (u) {
            d.ch();
            boolean bl = false;
            l = 0;
            b.setColor(0xFFFFFF);
            d.d(J == 1 || u ? 3 : 8195, 7, 56, Q * ab);
            boolean bl2 = false;
            if (Q > 0) {
                bl2 = true;
            }
            boolean bl3 = false;
            if (Q < ac - 1) {
                bl3 = true;
            }
            d.c(bl2, bl3);
            d.e(2, 3);
            return;
        }
        if (t) {
            d.cb();
            d.e(209, -1);
            return;
        }
        r = true;
        d.ch();
        int n = 56;
        int n2 = -1;
        int n3 = 0;
        boolean bl = false;
        boolean bl4 = false;
        n3 = G > 0 ? 6 : (m ? 9 : 8);
        switch (J) {
            case 0: {
                int n4 = L;
                while (n4 < I && n4 - L < P) {
                    int n5 = h[n4] * 4;
                    short s = a[1332 + n5];
                    short s2 = a[1332 + (n5 + 1)];
                    short s3 = a[1332 + (n5 + 2)];
                    short s4 = a[1332 + (n5 + 3)];
                    String string = a[s4];
                    int n6 = 0xFFFFFF;
                    if (s == 4) {
                        string = string + " : ";
                        string = j[s2] == 1 ? string + a[10] : string + a[11];
                    }
                    int n7 = 120;
                    if (s == 5) {
                        string = string + " : ";
                        int n8 = d.a(J == 1 || u ? 3 : 8195, string);
                        int n9 = n8 + d.e();
                        int n10 = 120 - (n9 >> 1) + n8;
                        n7 = n10 - (n8 >> 1);
                        if (s2 == 0) {
                            d.f(n10, n, d.e(), 22, j[2]);
                        } else {
                            d.f(n10, n, d.e(), 22, j[s2]);
                        }
                    }
                    if (s3 == 1) {
                        n6 = 0xFFFFFF;
                    } else if (n4 == H) {
                        n6 = 0xFFFFFF;
                        if (s == 4) {
                            n2 = 7;
                        } else if (s == 5) {
                            if (s2 == 0) {
                                int n11 = 0;
                                if (j[2] != 0) {
                                    n11 = 1;
                                }
                                n2 = n11 + 10;
                            } else {
                                int n12 = 0;
                                if (j[s2] != 0) {
                                    n12 = 1;
                                }
                                n2 = n12 + 10;
                            }
                        } else {
                            n2 = 5;
                        }
                    }
                    l = 0;
                    b.setColor(n6);
                    d.a(J == 1 || u ? 3 : 8195, string, n7, n, 17);
                    if ((s == 5 || s == 6) && n4 == H) {
                        int n13 = d.a(J == 1 || u ? 3 : 8195, a[a[1332 + ((n5 >> 2) * 4 + 3)]] + (s == 5 ? " : " : ""));
                        int n14 = 1;
                        if (s == 5) {
                            n14 = d.e();
                        }
                        d.a(true, true, n13 + n14 + 2 + 3, n);
                    }
                    n += 22;
                    ++n4;
                }
                if (L > 0) {
                    bl = true;
                }
                if (L < I - P) {
                    bl4 = true;
                }
                d.c(bl, bl4);
                d.e(n2, n3);
                return;
            }
            case 5: {
                int n15 = L;
                while (n15 < I && n15 - L < P) {
                    int n16 = h[n15] * 4;
                    short s = a[1332 + n16];
                    short s5 = a[1332 + (n16 + 2)];
                    if (s5 != 1 && n15 == H) {
                        n2 = s == 4 || s == 5 ? 7 : 5;
                    }
                    d.q(n, n16 >> 2);
                    n += d.d();
                    ++n15;
                }
                if (L > 0) {
                    bl = true;
                }
                if (L < I - P) {
                    bl4 = true;
                }
                d.c(bl, bl4);
                d.e(n2, n3);
                return;
            }
            case 1: {
                b.setColor(0xFFFFFF);
                d.d(J == 1 || u ? 3 : 8195, 7, 56, O);
                if (O > 0) {
                    bl = true;
                }
                if (O < ad) {
                    bl4 = true;
                }
                d.c(bl, bl4);
                d.e(-1, n3);
                return;
            }
            case 2: {
                d.X(f[G]);
            }
        }
    }

    private static void f(int n, int n2) {
        if (n == 0) {
            if (n2 == 100) {
                d.j[2] = j[2] > 0 ? 0 : 64;
            } else if (n2 > 0) {
                j[2] = (byte)(j[2] + 8);
                if (j[2] > 64) {
                    d.j[2] = 0;
                }
            } else {
                j[2] = (byte)(j[2] - 8);
                if (j[2] < 0) {
                    d.j[2] = 64;
                }
            }
            if (j[2] == 0) {
                d.j[0] = 0;
            } else {
                d.j[0] = 1;
                d.e();
            }
            d.d(j[2]);
            d.a(30, 128, 1);
            return;
        }
    }

    private static int d(int n, int n2) {
        if (R != n || i == null) {
            d.b(n);
        }
        int n3 = 0;
        while (n3 < 10) {
            if (n2 > i[n3]) {
                return n3;
            }
            ++n3;
        }
        return -1;
    }

    private static int a(int n, String string, int n2, int n3, int n4, String string2) {
        int n5;
        int n6;
        if (R != n || i == null) {
            d.b(n);
        }
        int n7 = -1;
        int n8 = 0;
        while (n8 < 10) {
            if (n2 > i[n8]) {
                if (n4 == 0) {
                    n6 = 8;
                    while (n6 >= n8) {
                        n5 = 0;
                        while (n5 < 5) {
                            d.i[n6 + 1 + 10 * n5] = i[n6 + 10 * n5];
                            ++n5;
                        }
                        n5 = 0;
                        while (n5 < 2) {
                            d.b[n6 + 1 + 10 * n5] = b[n6 + 10 * n5];
                            ++n5;
                        }
                        --n6;
                    }
                } else {
                    int n9;
                    n6 = -1;
                    n5 = 0;
                    while (n5 < 10) {
                        if (i[n5 + 40] == n4) {
                            n6 = n5;
                        }
                        ++n5;
                    }
                    if (n6 == -1) {
                        n5 = 8;
                        while (n5 >= n8) {
                            n9 = 0;
                            while (n9 < 5) {
                                d.i[n5 + 1 + 10 * n9] = i[n5 + 10 * n9];
                                ++n9;
                            }
                            n9 = 0;
                            while (n9 < 2) {
                                d.b[n5 + 1 + 10 * n9] = b[n5 + 10 * n9];
                                ++n9;
                            }
                            --n5;
                        }
                    } else {
                        if (n6 < n8) break;
                        if (n6 > n8) {
                            n5 = n6 - 1;
                            while (n5 >= n8) {
                                n9 = 0;
                                while (n9 < 5) {
                                    d.i[n5 + 1 + 10 * n9] = i[n5 + 10 * n9];
                                    ++n9;
                                }
                                n9 = 0;
                                while (n9 < 2) {
                                    d.b[n5 + 1 + 10 * n9] = b[n5 + 10 * n9];
                                    ++n9;
                                }
                                --n5;
                            }
                        }
                    }
                }
                d.i[n8] = n2;
                d.i[n8 + 10] = n8;
                d.i[n8 + 20] = n3;
                d.i[n8 + 30] = (int)(System.currentTimeMillis() / 1000L);
                d.i[n8 + 40] = n4;
                d.b[n8] = string;
                d.b[n8 + 10] = string2 == null ? "" : string2;
                n7 = n8;
                break;
            }
            ++n8;
        }
        if (n7 < 0) {
            return -1;
        }
        n8 = 1;
        n6 = -1;
        n5 = 0;
        while (n5 < 10) {
            if (i[n5] == n6 && i[n5 + 40] != 0) {
                d.i[n5 + 10] = n8;
            } else {
                d.i[n5 + 10] = n5 + 1;
                n8 = n5 + 1;
                n6 = i[n5];
            }
            ++n5;
        }
        d.b();
        return n7;
    }

    private static boolean b(int n) {
        byte[] byArray;
        if (i == null) {
            i = new int[50];
            b = new String[20];
        }
        if ((byArray = d.a("420" + (R = n))) != null) {
            try {
                a = new ByteArrayInputStream(byArray);
                b = new DataInputStream(a);
                int n2 = 0;
                while (n2 < i.length) {
                    d.i[n2] = b.readInt();
                    ++n2;
                }
                n2 = 0;
                while (n2 < b.length) {
                    d.b[n2] = b.readUTF();
                    ++n2;
                }
                a = null;
                b = null;
            }
            catch (Exception exception) {}
        } else {
            d.m(n);
            return false;
        }
        return true;
    }

    private static boolean b() {
        if (R < 0) {
            return false;
        }
        boolean bl = false;
        try {
            a = new ByteArrayOutputStream();
            a = new DataOutputStream(a);
            int n = 0;
            while (n < i.length) {
                a.writeInt(i[n]);
                ++n;
            }
            n = 0;
            while (n < b.length) {
                a.writeUTF(b[n]);
                ++n;
            }
            byte[] byArray = a.toByteArray();
            a = null;
            a = null;
            bl = d.a("420" + R, byArray);
        }
        catch (Exception exception) {}
        return bl;
    }

    private static void F() {
        int n = R;
        int n2 = 0;
        while (n2 < 7) {
            d.m(n2);
            ++n2;
        }
        R = n;
    }

    private static void m(int n) {
        if (i == null) {
            i = new int[50];
            b = new String[20];
        }
        R = n;
        int n2 = 0;
        while (n2 < 10) {
            d.i[n2 + 10] = n2 + 1;
            d.i[n2] = d.j(n, n2);
            d.i[n2 + 20] = d.k(n, n2);
            d.b[n2] = d.b(n, n2);
            d.i[n2 + 40] = 0;
            d.i[n2 + 30] = (int)(System.currentTimeMillis() / 1000L);
            d.b[n2 + 10] = "";
            ++n2;
        }
        d.b();
    }

    private static String b(int n) {
        if (R != n) {
            d.b(n);
        }
        StringBuffer stringBuffer = new StringBuffer();
        int n2 = 0;
        while (n2 < 10) {
            stringBuffer.append('\u0014');
            stringBuffer.append(i[n2 + 10]);
            stringBuffer.append(' ');
            stringBuffer.append(b[n2]);
            stringBuffer.append('\u0015');
            stringBuffer.append(i[n2]);
            ++n2;
        }
        return stringBuffer.toString();
    }

    private static void G() {
        String string = d.a();
        if (string != null) {
            d.a(string);
        }
        d.v();
    }

    private static String a() {
        String string = d.d();
        if (string != null) {
            String string2 = "i" + d.b() + "_o6_";
            return "http://sg3.mrg.fi/?request=" + d.a(string2 + string);
        }
        return null;
    }

    private static String a(String string) {
        return d.a(string, 27);
    }

    private static String a(String string, int n) {
        StringBuffer stringBuffer = new StringBuffer();
        String string2 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_.!~*'()";
        int n2 = 0;
        while (n2 < string.length()) {
            stringBuffer.append(d.a(string.charAt(n2), n, string2));
            ++n2;
        }
        string = stringBuffer.toString();
        return string;
    }

    private static char a(char c2, int n, String string) {
        int n2 = string.indexOf(c2);
        if (n2 == -1) {
            return c2;
        }
        int n3 = string.length();
        if ((n2 += n % (n3 - 1) + 1) >= n3) {
            n2 -= n3;
        }
        return string.charAt(n2);
    }

    private static String b() {
        if (b == null) {
            d.d();
        }
        if (b == null) {
            b = "" + t + System.currentTimeMillis() % 100000000L;
            if (b.length() > 15) {
                b = b.substring(0, 14);
            }
            d.c();
        }
        return b;
    }

    private static boolean c() {
        if (b == null) {
            return false;
        }
        try {
            a = new ByteArrayOutputStream();
            a = new DataOutputStream(a);
            a.writeUTF(b);
            byte[] byArray = a.toByteArray();
            a = null;
            a = null;
            d.a("hs_browserlaunch", byArray);
            return true;
        }
        catch (Exception exception) {
            return false;
        }
    }

    private static boolean d() {
        byte[] byArray = d.a("hs_browserlaunch");
        if (byArray != null) {
            try {
                a = new ByteArrayInputStream(byArray);
                b = new DataInputStream(a);
                b = b.readUTF();
                a = null;
                b = null;
                return true;
            }
            catch (Exception exception) {}
        }
        return false;
    }

    private static void H() {
        d.a("http://sg3.mrg.fi/?cid=6");
        d.v();
    }

    private static void a(int n, String string, int n2, int n3, boolean bl) {
        d.a(-1, n, string, n2 - 2, n3, bl, true);
        ab = n3 / d.e(n);
        if (ab < 1) {
            ab = 1;
        }
        ac = 1;
        int n4 = 1;
        while (n4 < W) {
            if (c[n4] - c[ai] >= X) {
                ai = n4 - 1;
                ++ac;
            }
            ++n4;
        }
        ad = U - 1;
    }

    private static void a(int n, int n2, String string, int n3, int n4, boolean bl, boolean bl2) {
        ai = 0;
        W = 0;
        int n5 = (int)System.currentTimeMillis();
        y = false;
        if (bl) {
            aa = n;
            ae = n2;
            af = n3;
            ag = n4;
            d = string;
            x = bl2;
        }
        Z = n;
        c = null;
        c = string;
        X = n4;
        S = 0;
        U = 0;
        ah = 0;
        int n6 = 0;
        int n7 = 0;
        int n8 = 0;
        int n9 = 0;
        int n10 = d.e(n2);
        int n11 = d.a(n2, " ");
        int n12 = 0;
        int n13 = bl2 ? 2 : 0;
        d.c[0] = 0;
        int n14 = 1;
        int n15 = 0;
        int n16 = n3;
        int n17 = 0;
        int n18 = 0;
        int n19 = 0;
        int n20 = 0;
        int n21 = n16;
        int n22 = -1;
        int n23 = 0;
        int n24 = 0;
        int n25 = 0;
        int n26 = 0;
        boolean bl3 = false;
        int n27 = 0;
        int n28 = 0;
        int n29 = 0;
        int n30 = n13;
        int n31 = string.length();
        V = 0;
        boolean bl4 = false;
        short s = 0;
        short s2 = 0;
        while (n7 < n31) {
            int n32;
            if ((!m || !d.n && m) && (int)(System.currentTimeMillis() - (long)n5) > 250) {
                d.o();
                n5 = (int)System.currentTimeMillis();
            }
            int n33 = string.charAt(n7);
            ++n7;
            boolean bl5 = false;
            boolean bl6 = false;
            switch (n33) {
                case 19: 
                case 25: 
                case 27: 
                case 28: {
                    int n34;
                    n32 = n7 - 1;
                    int n35 = 0;
                    while (string.charAt(n7) != ':') {
                        n35 *= 10;
                        n35 += string.charAt(n7) - 48;
                        ++n7;
                    }
                    ++n7;
                    if (n33 == 25 || n33 == 28) {
                        n34 = d.l(n35);
                        n19 = n34 >> 16 & (char)-1;
                        n18 = n34 & (char)-1;
                        if (n33 == 25) {
                            n8 = n15;
                            if ((n15 += n19) <= n16 || n3 < n19) {
                                d.d[n17++] = (short)n35;
                                d.d[n17++] = (short)n8;
                                d.d[n17++] = (short)n9;
                                d.d[n17++] = (short)n18;
                                if (n12 < n18) {
                                    n12 = n18;
                                }
                                n29 = n15;
                                if (n7 != n31) break;
                                bl5 = true;
                                break;
                            }
                            n7 = n32;
                            bl5 = true;
                            break;
                        }
                        if (n15 == 0) {
                            switch (n13) {
                                case 0: {
                                    n8 = 0;
                                    n15 = n20 = n19;
                                    break;
                                }
                                case 1: {
                                    n21 = n8 = n3 - n19;
                                    n16 = n8;
                                    break;
                                }
                                case 2: {
                                    n8 = (n3 - n19) / 2;
                                    n12 = n18;
                                    bl5 = true;
                                    bl6 = true;
                                }
                            }
                            n28 = n15;
                            n22 = n9 + n18 + 2;
                            d.d[n17++] = (short)n35;
                            d.d[n17++] = (short)n8;
                            d.d[n17++] = (short)n9;
                            d.d[n17++] = (short)n18;
                            n23 = n17;
                            n24 = n17;
                            break;
                        }
                        bl5 = true;
                        bl6 = true;
                        n7 = n32;
                        break;
                    }
                    if (n33 == 27) {
                        d.b[n6++] = (short)(-(n35 + 1 + 16383));
                        break;
                    }
                    d.b[n6++] = (short)(-(n35 + 1));
                    n2 = d.m(n35);
                    n11 = d.a(n2, " ");
                    n10 = d.e(n2);
                    if (n15 != n28) break;
                    n12 = n10;
                    break;
                }
                case 10: {
                    if (n9 < n22 - n12) {
                        n9 = n22 - n12;
                    }
                    n22 = -1;
                    bl5 = true;
                    bl6 = true;
                    break;
                }
                case 20: 
                case 21: 
                case 22: {
                    bl3 = true;
                    n30 = n33 - 20;
                    if (bl2 && n13 == 1 && n30 == 0 && n15 > n28) {
                        bl5 = true;
                    }
                    if (n13 != 2 || n30 == 2 || n15 <= n28) break;
                    bl5 = true;
                    break;
                }
                case 24: {
                    if (bl4) {
                        d.e[d.V++] = s;
                        d.e[d.V++] = s2;
                        d.e[d.V++] = (short)(n9 - s2 + n12 + 2);
                        bl4 = false;
                        break;
                    }
                    s = (short)(n14 - 1);
                    s2 = (short)n9;
                    bl4 = true;
                    break;
                }
                case 32: {
                    if (n15 > n28) {
                        n15 += n11;
                    }
                    while (n7 < n31 && string.charAt(n7) == ' ') {
                        ++n7;
                    }
                    break;
                }
                default: {
                    n32 = --n7;
                    int n35 = -1;
                    int n34 = n16 - n15;
                    boolean bl7 = false;
                    int n36 = n21 - n20;
                    while (n7 <= n31) {
                        n33 = n7 == n31 ? -1 : (int)string.charAt(n7);
                        if (n33 == 32 || n33 == 10 || n33 == 9 || n33 >= 19 && n33 <= 28 || n33 == -1) {
                            int n37 = d.a(n2, string.substring(n32, n7));
                            if (n37 > n34) {
                                if (n35 >= 0) {
                                    n7 = n35;
                                    if (n33 != 32 || n37 - n25 >= n36) break;
                                    bl7 = true;
                                    break;
                                }
                                if (n34 < n36 && n37 < n36) {
                                    n7 = n32;
                                    bl5 = true;
                                    break;
                                }
                                n7 = n32 + 1;
                                int n38 = n37 = d.a(n2, string.substring(n32, n7));
                                while (n37 < n34) {
                                    n38 = n37;
                                    n37 = d.a(n2, string.substring(n32, ++n7));
                                }
                                n25 = n38;
                                if (n32 != --n7) break;
                                if (n37 < n36) {
                                    bl5 = true;
                                    break;
                                }
                                n25 = d.a(n2, string.substring(n32, ++n7));
                                break;
                            }
                            n35 = n7;
                            n25 = n37;
                            if (n33 != 32) {
                                while (string.charAt(n7 - 1) == ' ') {
                                    --n7;
                                }
                                int n39 = 0;
                                if (n7 > n32) {
                                    n39 = d.a(n2, string.substring(n32, n7));
                                }
                                n25 = n39;
                                break;
                            }
                        }
                        ++n7;
                    }
                    if (bl5) break;
                    n8 = n15;
                    n29 = n15 += n25;
                    d.b[n6++] = (short)n32;
                    d.b[n6++] = (short)(n7 - n32);
                    d.b[n6++] = (short)n8;
                    d.b[n6++] = (short)n9;
                    d.b[n6++] = (short)n25;
                    d.b[n6++] = (short)n10;
                    if (bl7) {
                        bl5 = true;
                    }
                    if (n12 >= n10) break;
                    n12 = n10;
                }
            }
            if (n7 == n31) {
                bl5 = true;
            }
            if (bl3 || bl5) {
                if (ah < n15) {
                    ah = n15;
                }
                if (n13 != 0) {
                    n32 = n16 - n29;
                    if (n13 == 2) {
                        n32 = (n32 + 1) / 2;
                    }
                    while (n27 < n6) {
                        if (b[n27] >= 0) {
                            int n40 = n27 + 2;
                            b[n40] = (short)(b[n40] + n32);
                            n27 += 6;
                            continue;
                        }
                        ++n27;
                    }
                    while (n24 < n17) {
                        int n41 = n24 + 1;
                        d[n41] = (short)(d[n41] + n32);
                        n24 += 4;
                    }
                    if (n13 == 1) {
                        ah = n16;
                        n15 = n28;
                        n16 = n15 + n32;
                    } else if (n30 == 1) {
                        n15 = n29;
                    } else {
                        n15 = n28;
                        n16 -= n29;
                    }
                } else {
                    n27 = n6;
                    n24 = n17;
                }
                n28 = n15;
                bl3 = false;
                n13 = n30;
            }
            if (!bl5) continue;
            while (n26 < n6) {
                if (b[n26] >= 0) {
                    int n42 = n26 + 3;
                    b[n42] = (short)(b[n42] + (n12 - b[n26 + 5]) / 2);
                    n26 += 6;
                    continue;
                }
                ++n26;
            }
            while (n23 < n17) {
                int n43 = n23 + 2;
                d[n43] = (short)(d[n43] + (n12 - d[n23 + 3]) / 2);
                n23 += 4;
            }
            n9 += n12;
            if (bl6) {
                n9 += 0;
            }
            n12 = n10;
            if (n9 > n22) {
                n15 = 0;
                n16 = n3;
                n20 = 0;
                n21 = n3;
            } else {
                n15 = n20;
                n16 = n21;
            }
            n28 = n15;
            n29 = n15;
            d.c[n14] = (short)n9;
            ++n14;
        }
        while (n9 <= n22) {
            d.c[n14] = (short)(n9 += n10);
            ++n14;
        }
        Y = n9;
        S = n6;
        T = n17;
        W = n14;
        while (n14 > 1) {
            if (c[W - 1] - c[--n14 - 1] <= n4) continue;
            ++n14;
            break;
        }
        U = n14;
        y = true;
        i = false;
    }

    private static void d(int n, int n2, int n3, int n4) {
        int n5;
        int n6;
        if (!y) {
            return;
        }
        int n7 = b.getColor();
        int n8 = n;
        int n9 = 0;
        int n10 = 0;
        short s = c[n4];
        while (n9 < S) {
            if (b[n9] >= 0) {
                n6 = b[n9 + 3] - s;
                if (n6 >= 0 && n6 <= X - b[n9 + 5]) {
                    n5 = n2 + b[n9 + 2];
                    short s2 = b[n9];
                    d.a(n, c.substring(s2, s2 + b[n9 + 1]), n5, n6 += n3, 20);
                } else if (n6 >= 0) break;
                n9 += 6;
                continue;
            }
            n10 = -b[n9] - 1;
            if (n10 < 16383) {
                n = n10 == 0 ? n8 : d.m(n10);
            } else {
                b.setColor((n10 -= 16383) == 0 ? n7 : d.c());
            }
            ++n9;
        }
        n9 = 0;
        while (n9 < T) {
            n6 = d[n9 + 2] - s;
            if (n6 >= -d[n9 + 3] && n6 < X) {
                n5 = n2 + d[n9 + 1];
                d.h(d[n9], n5, n6 + n3);
            } else if (n6 >= 0) break;
            n9 += 4;
        }
        b.setColor(n7);
    }

    /*
     * Unable to fully structure code
     */
    private static String a(String var0, String var1_1, int var2_2, int var3_3) {
        d.m = d.a(0, 0, 51, null);
        d.l = d.a(0, 51, 11, null);
        d.aj = var3_3;
        d.e = var1_1 == null ? "" : var1_1;
        d.am = 0;
        d.z = true;
        d.A = false;
        d.ak = -1;
        d.al = d.t;
        var4_4 = d.n;
        d.n = false;
        d.q();
        d.q = true;
        ** GOTO lbl34
        {
            try {
                Thread.sleep(100L);
            }
            catch (Exception v0) {}
            if (d.a == null) ** GOTO lbl22
            do {
                if (d.p) continue block4;
lbl22:
                // 2 sources

                d.j();
                d.r();
                if (d.a == null) break block4;
                d.a.s();
                do {
                    d.j = false;
                    try {
                        Thread.yield();
                    }
                    catch (Exception v1) {}
                } while (!d.l || d.j);
                d.q = false;
lbl34:
                // 2 sources

            } while (!d.A && d.a != null);
        }
        d.m = null;
        d.l = null;
        d.z = false;
        d.n = var4_4;
        return d.e;
    }

    private static void n(int n) {
        if (d.a(n, 46)) {
            ak = -1;
            return;
        }
        int n2 = d.f(n);
        if (n2 != -1) {
            if (ak == n2 && t - al < 1000) {
                if (++am == l[n2 + 1] - l[n2]) {
                    am = 0;
                }
                if (e.length() > 0) {
                    e = e.substring(0, e.length() - 1);
                }
                e = e + (char)m[l[n2] + am];
            } else if (ak != n2 || t - al >= 1000) {
                am = 0;
                if (e.length() == aj) {
                    e = e.substring(0, e.length() - 1);
                }
                e = e + (char)m[l[n2] + am];
            }
            al = t;
            ak = n2;
            return;
        }
        if (d.a(n, 196625)) {
            A = true;
            return;
        }
        if (d.a(n, 43)) {
            if (e.length() > 0) {
                e = e.substring(0, e.length() - 1);
                am = 0;
                e = true;
            }
            al = t;
            ak = -1;
        }
    }

    private static void g(int n, int n2) {
        try {
            if (b == null) {
                b = new short[3][];
            }
            if (b[n] != null) {
                return;
            }
            d.b[n] = new short[1 + n2];
            d.a("k" + n, 0L);
            int n3 = 0;
            while (n3 < n2) {
                d.b[n][n3] = a.readShort();
                ++n3;
            }
            d.p();
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static short a(int n) {
        int n2 = n >>> 13;
        int n3 = n & 0x1FFF;
        return b[n2][3 + b[n2][1 + n3]];
    }

    private static short b(int n) {
        int n2 = n >>> 13;
        int n3 = n & 0x1FFF;
        return b[n2][4 + b[n2][1 + n3]];
    }

    private static short c(int n) {
        int n2 = n >>> 13;
        int n3 = n & 0x1FFF;
        return b[n2][1 + b[n2][1 + n3]];
    }

    private static short d(int n) {
        int n2 = n >>> 13;
        int n3 = n & 0x1FFF;
        return b[n2][2 + b[n2][1 + n3]];
    }

    private static void a(int n, int n2, int n3, int n4, int n5, int n6) {
        d.a(n, 0, n2, 65536, n3, n4, n5, n6);
    }

    /*
     * Enabled aggressive block sorting
     */
    private static void a(int n, int n2, int n3, int n4, int n5, int n6, int n7, int n8) {
        int n9;
        int n10;
        int n11;
        int n12;
        int n13;
        int n14;
        int n15;
        int n16;
        int n17;
        int n18;
        int n19;
        int n20;
        if (n4 > Short.MAX_VALUE) {
            int n21 = n2;
            n2 = n3;
            n3 = n21;
            n4 = 65536 - n4;
        }
        short[] sArray = b[n >>> 13];
        int n22 = n & 0x1FFF;
        n22 = sArray[1 + n22];
        short s = sArray[n22++];
        if (n7 != 1024) {
            n5 -= d.b(n7, (int)d.a(n));
            n6 -= d.c(n7, (int)d.b(n));
        } else {
            n5 -= n8 != 4 ? sArray[n22] : sArray[n22 + 2] - sArray[n22];
            n6 -= sArray[n22 + 1];
        }
        if (n5 >= 240 || n6 >= 320 || n5 + sArray[n22 + 2] <= 0 || n6 + sArray[n22 + 3] <= 0) {
            return;
        }
        short[] sArray2 = null;
        int n23 = 0;
        if (n + 1 == an) {
            sArray2 = f;
            n20 = (sArray[n22 + 2] + 120 - 1) / 120;
            n19 = (sArray[n22 + 3] + 160 - 1) / 160;
            n18 = -n6 / 160;
            n17 = n18 + 3;
            while (n18 < n17) {
                n16 = -n5 / 120;
                n15 = n16 + 3;
                while (n16 < n15) {
                    if (n16 >= 0 && n18 >= 0 && n16 < n20 && n18 < n19) {
                        n14 = n18 * n20 + n16;
                        n13 = sArray2[n14];
                        while (n13 < sArray2[n14 + 1]) {
                            n12 = n23++;
                            n11 = sArray2[n13];
                            while (n12 > 0 && (n10 = g[n9 = n12 - 1 >> 1]) > n11) {
                                d.g[n12] = n10;
                                n12 = n9;
                            }
                            d.g[n12] = n11;
                            ++n13;
                        }
                    }
                    ++n16;
                }
                ++n18;
            }
            d.g[n23] = Short.MAX_VALUE;
        }
        n22 += 4;
        block16: while (true) {
            int n24;
            if (sArray2 != null) {
                n22 = g[0];
                if (n22 == Short.MAX_VALUE) {
                    return;
                }
                do {
                    n20 = 0;
                    while ((n19 = n20 * 2 + 1) < n23) {
                        n18 = g[n19];
                        n17 = g[n19 + 1];
                        if (n18 > n17) {
                            n18 = n17;
                        }
                        d.g[n20] = n18;
                        n20 = ++n19;
                    }
                    d.g[n20] = Short.MAX_VALUE;
                } while (g[0] == n22);
            }
            if ((n24 = sArray[n22++]) == 0) {
                return;
            }
            int n25 = 0;
            if (s > 1) {
                n25 = sArray[n22++];
            }
            n20 = n25;
            n19 = n24 & 0xFF;
            n18 = n20;
            n17 = 0;
            while (n17 < n19) {
                if ((n18 & 1) == 0) {
                    int n26 = sArray[n22++];
                    d.j[n17 + 6] = n26;
                    d.j[n17] = n26;
                } else {
                    d.j[n17] = sArray[n22 + n2];
                    d.j[n17 + 6] = sArray[n22 + n3];
                    n22 += s;
                }
                ++n17;
                n18 >>= 1;
            }
            n18 = j[2];
            n17 = j[8];
            b.setColor(((n18 & 0xF800) * (65536 - n4) + (n17 & 0xF800) * n4 >>> 13) * 33 & 0xFF0000 | ((n18 & 0x7E0) * (65536 - n4) + (n17 & 0x7E0) * n4 >>> 17) * 65 & 0xFF00 | ((n18 & 0x1F) * (65536 - n4) + (n17 & 0x1F) * n4) * 33 >>> 18 & 0xFF);
            if ((n24 & 0x4000) == 16384) continue;
            int n27 = 0;
            if ((n24 & 0x8000) == 32768) {
                n27 = 1;
            }
            n16 = n27;
            n24 &= 0x3FFF;
            n15 = 0;
            while (n15 < 6) {
                n14 = j[n15 + 6] - j[n15];
                d.j[n15 + 6] = j[n15];
                int n28 = n15++;
                j[n28] = j[n28] + (n14 * n4 + 32768 >> 16);
            }
            if (n16 != 0) {
                d.a(n, n24, j);
            }
            n15 = j[0];
            n14 = j[1];
            if (n8 == 4) {
                switch (n24 >> 8) {
                    case 0: {
                        if (j[8] == -1) continue block16;
                        n13 = d.c(j[8] >> 3);
                        if ((1 << (j[8] & 7) & 0xCA) != 0) {
                            n13 = d.b(j[8] >> 3);
                        }
                        n15 = -n15 - (n13 & 1);
                        j[8] = j[8] ^ ((j[8] - 1 & 2) == 0 ? 7 : 4);
                        break;
                    }
                    case 1: {
                        n15 = -n15 - j[3];
                        break;
                    }
                    case 4: {
                        n15 = -n15 - d.a(j[2]);
                    }
                }
                n15 += d.a(n);
            }
            n15 += n5;
            n14 += n6;
            switch (n24 >> 8) {
                case 0: {
                    if (j[8] == -1) continue block16;
                    d.a(j[8] >> 3, n15, n14, 3, j[8] & 7);
                    break;
                }
                case 3: {
                    d.j(n15, n14, j[n19 - 2], j[n19 - 1]);
                    break;
                }
                case 1: {
                    b.fillRect(n15, n14, j[3], j[4]);
                    break;
                }
                case 4: {
                    d.a(j[2], 0, n15, n14, 20, 0);
                    break;
                }
                case 2: {
                    n13 = d.b(j[2] >> 3);
                    n12 = d.c(j[2] >> 3);
                    if ((1 << (j[2] & 7) & 0xCA) != 0) {
                        n9 = n12;
                        n12 = n13;
                        n13 = n9;
                    }
                    d.b(n15, n14, j[3], j[4]);
                    n9 = n15 + j[3];
                    n10 = n14 + j[4];
                    if (n15 < 0) {
                        n15 -= n15 / n12 * n12;
                    }
                    if (n14 < 0) {
                        n14 -= n14 / n13 * n13;
                    }
                    if (n9 > 240) {
                        n9 = 240;
                    }
                    if (n10 > 320) {
                        n10 = 320;
                    }
                    n11 = n14;
                    block21: while (true) {
                        if (n11 >= n10) {
                            d.g();
                            continue block16;
                        }
                        int n29 = n15;
                        while (true) {
                            if (n29 >= n9) {
                                n11 += n13;
                                continue block21;
                            }
                            d.a(j[2] >> 3, n29, n11, 0, j[2] & 7);
                            n29 += n12;
                        }
                        break;
                    }
                }
            }
        }
    }

    private static void a(int n, int[] nArray) {
        short[] sArray = b[n >>> 13];
        int n2 = n & 0x1FFF;
        n2 = sArray[1 + n2];
        int n3 = sArray[n2++];
        n2 += 4;
        short s;
        block0: while ((s = sArray[n2++]) != 0) {
            int n4 = 0;
            if (n3 > 1) {
                n4 = sArray[n2++];
            }
            int n5 = n4;
            int n6 = s & 0xFF;
            int n7 = n5;
            int n8 = 0;
            while (true) {
                int n9;
                if (n8 >= n6) continue block0;
                boolean bl = false;
                if ((n7 & 1) == 0) {
                    bl = true;
                }
                boolean bl2 = bl;
                int n10 = 1;
                if (!bl2) {
                    n10 = n9 = n3;
                }
                if ((s & 0x3FFF) >> 8 == 0 && n8 == 2) {
                    int n11 = 0;
                    while (n11 < n9) {
                        short s2 = sArray[n2 + n11];
                        int n12 = s2 & 7;
                        int n13 = s2 >> 3;
                        int n14 = d.a(n13, nArray);
                        sArray[n2 + n11] = (short)(n12 | n14 << 3);
                        ++n11;
                    }
                }
                n2 += n9;
                ++n8;
                n7 >>= 1;
            }
            break;
        }
        return;
    }

    private static int a(int n, int[] nArray) {
        int n2 = 0;
        while (n2 < nArray.length) {
            if (nArray[n2] == n) {
                return nArray[n2 + 1];
            }
            n2 += 2;
        }
        return n;
    }

    private static int a(int n, char c2) {
        int n2 = 0;
        while (n2 < a[45 + (n + 1 - 3)] - a[45 + (n - 3)]) {
            if (a[50 + (a[45 + (n - 3)] + n2)] == c2) {
                return n2;
            }
            ++n2;
        }
        return 0;
    }

    private static int b(int n, String string) {
        int n2 = 0;
        int n3 = 0;
        while (n3 < string.length()) {
            int n4 = d.a(n, string.charAt(n3));
            n2 += k[a[30 + (n - 3)] + n4];
            ++n3;
        }
        return n2;
    }

    private static void a(int n, String string, int n2, int n3) {
        int n4 = 0;
        while (n4 < string.length()) {
            char c2 = string.charAt(n4);
            int n5 = d.a(n, c2);
            short s = a[1477 + (a[1472 + (n - 3)] + n5)];
            if (s > 0) {
                d.a(s - 1, n2 + k[301 + (a[35 + (n - 3)] + n5)], n3 + k[602 + (a[40 + (n - 3)] + n5)], 20, 0);
            }
            n2 += k[a[30 + (n - 3)] + n5];
            ++n4;
        }
    }

    private static int g(int n) {
        if (n < 0) {
            return -n;
        }
        return n;
    }

    private static int a() {
        if (a == null) {
            a = new Random();
        }
        return d.g(a.nextInt());
    }

    private static int h(int n) {
        n = n << 13 ^ n;
        return n * (n * n * 15731 + 789221) + 1376312589 & Integer.MAX_VALUE;
    }

    private static short[] a(int n, boolean bl) {
        short[] sArray = new short[512];
        n = 30 - n;
        int n2 = bl ? 128 : 0;
        long l = 0x40000000L;
        long l2 = 1073660973L;
        sArray[n2] = (short)(l >> n);
        sArray[n2 + 1] = (short)(l2 >> n);
        int n3 = 2;
        while (n3 < 128) {
            long l3 = (l2 * 2147321946L >> 30) - l;
            sArray[n2 + n3 & 0x1FF] = (short)(l3 >> n);
            l = l2;
            l2 = l3;
            ++n3;
        }
        n3 = 127;
        while (n3 >= 0) {
            sArray[n2 + 256 - n3 & 0x1FF] = -sArray[n2 + n3 & 0x1FF];
            --n3;
        }
        n3 = 255;
        while (n3 > 0) {
            sArray[n2 + 256 + n3 & 0x1FF] = -sArray[n2 + n3 & 0x1FF];
            --n3;
        }
        return sArray;
    }

    private static void I() {
        k = new int[100];
    }

    private static void J() {
        int n = k[0] >> 16;
        int n2 = n << 16;
        int n3 = 0;
        int n4 = 0;
        int n5 = 0;
        int n6 = n << 16;
        int n7 = -1;
        int n8 = n;
        int n9 = 0;
        block0: while (n9 < ao) {
            int n10 = n2;
            n2 = k[n9 + n9];
            int n11 = k[n9 + n9 + 1];
            int n12 = n11 >> 16 & 0xFF;
            int n13 = n11 >> 8 & 0xFF;
            int n14 = n11 & 0xFF;
            boolean bl = true;
            while (bl) {
                int n15 = 65536;
                if (n10 > n6) {
                    n15 -= n10 - n6;
                }
                if (n2 < n6 + 65536) {
                    n15 -= n6 + 65536 - n2;
                }
                if (n15 != 65536) {
                    n3 += n12 * n15;
                    n4 += n13 * n15;
                    n5 += n14 * n15;
                }
                if (n2 >= n6 + 65536) {
                    int n16 = n15 == 65536 ? n11 : (n5 >> 16) + (n4 >> 8 & 0xFF00) + (n3 & 0xFF0000);
                    if (n16 != n7) {
                        if (n7 != -1) {
                            b.setColor(n7);
                            b.fillRect(0, n8, 240, n - n8);
                        }
                        n8 = n;
                        n7 = n16;
                    }
                    if (n >= 320) break block0;
                    ++n;
                    n6 += 65536;
                    n5 = 0;
                    n4 = 0;
                    n3 = 0;
                    continue;
                }
                bl = false;
            }
            ++n9;
        }
        b.setColor(n7);
        b.fillRect(0, n8, 240, n - n8);
    }

    private static int a(int n, int n2, int n3) {
        return ((n & 0xFF00FF) * (256 - n3) + (n2 & 0xFF00FF) * n3 & 0xFF00FF00 | (n & 0xFF00) * (256 - n3) + (n2 & 0xFF00) * n3 & 0xFF0000) >> 8;
    }

    private static int e(int n, int n2) {
        if ((n += 0) < 900) {
            int n3 = 0;
            if (n2 > 56) {
                n3 = 600 * (n2 - 56) / (319 - d.e(0) - 1 - 22 - 56);
            }
            if (n < n3) {
                return 240;
            }
            if (n < n3 + 300) {
                return d.c(0, 240, 300 - (n - n3), 300);
            }
            return 0;
        }
        return 0;
    }

    private static void a(boolean bl) {
        d.a(12, bl);
        int n = d.l(aZ, (bc & 1) != 0 ? 3 : bd);
        int n2 = d.m(aM, n);
        if (n2 >= E[1]) {
            int n3;
            if (aI == 2) {
                n3 = 16;
            } else {
                int n4 = 0;
                if (aJ == 0) {
                    n4 = 1;
                }
                n3 = 15 - n4;
            }
            d.o(n3);
        }
        if (E || F) {
            d.o(8);
        }
    }

    private static void K() {
        l = new int[8];
        m = new int[8];
        ap = 0;
        ar = 0;
    }

    private static void b(boolean bl) {
        if (bl) {
            d.f();
            H = false;
        } else if (ap == 0 || l[0] < 1 || l[0] > 3) {
            H = false;
        }
        ap = 0;
    }

    private static void o(int n) {
        d.a(n, 1, false);
    }

    private static void h(int n, int n2) {
        d.a(n, n2, false);
    }

    private static void a(int n, boolean bl) {
        d.a(n, 1, bl);
    }

    private static void a(int n, int n2, boolean bl) {
        if (bl) {
            d.b(true);
        }
        if (ap >= 8) {
            return;
        }
        if (m && (d.n || !m) && aQ == 1 && ba == 3) {
            return;
        }
        d.l[d.ap] = n;
        d.m[d.ap] = n2;
        if (ap == 0) {
            aq = ar + d[34 + n];
            d.a(n, 128, n2);
            if (n2 == 0) {
                d.b(false);
                return;
            }
        }
        ++ap;
    }

    private static void L() {
        if (ap > 0 && (ar += v) >= aq) {
            int n = 1;
            while (n < ap) {
                d.l[n - 1] = l[n];
                d.m[n - 1] = m[n];
                ++n;
            }
            if (--ap > 0) {
                aq = ar + d[34 + l[0]];
                d.a(l[0], 128, m[0]);
                if (m[0] == 0) {
                    d.b(false);
                    return;
                }
            }
        }
    }

    private static void M() {
        n = new int[2400];
        o = new int[6];
        int n = 0;
        while (n < 6) {
            d.p(n);
            ++n;
        }
    }

    private static void N() {
        n = null;
        o = null;
    }

    private static void p(int n) {
        d.o[n] = 0;
        int n2 = n * 400;
        while (n2 < (n + 1) * 400) {
            d.n[n2] = -1;
            ++n2;
        }
    }

    private static void q(int n) {
        d.o[n] = 0;
    }

    private static void d(int n, int n2, int n3) {
        if (o[n] + 1 >= 400) {
            return;
        }
        int n4 = n * 400 + o[n];
        d.n[n4] = n2;
        d.n[n4 + 1] = n3;
        int n5 = n;
        o[n5] = o[n5] + 2;
    }

    private static int f(int n, int n2) {
        if (o[n] >= 400) {
            return -1;
        }
        int n3 = n * 400 + o[n];
        if (d.n[n3] < 0 || d.n[n3] > n2) {
            return -1;
        }
        int n4 = n;
        o[n4] = o[n4] + 2;
        return d.n[n3 + 1];
    }

    private static void O() {
        h = d.a(14, true);
        d.I();
    }

    private static int g(int n, int n2) {
        if (n < n2) {
            return n;
        }
        return n2;
    }

    private static int h(int n, int n2) {
        if (n > n2) {
            return n;
        }
        return n2;
    }

    private static int b(int n, int n2, int n3) {
        if (n < n2) {
            return n2;
        }
        if (n > n3) {
            return n3;
        }
        return n;
    }

    private static int i(int n) {
        if (n >= 0) {
            return n;
        }
        return -n;
    }

    private static int j(int n) {
        int n2 = n >> 8 & 0x1FF;
        return h[n2] + ((n & 0xFF) * (h[n2 + 1 & 0x1FF] - h[n2]) >> 8);
    }

    private static int k(int n) {
        int n2 = (n >> 8) + 128 & 0x1FF;
        return h[n2] + ((n & 0xFF) * (h[n2 + 1 & 0x1FF] - h[n2]) >> 8);
    }

    private static int a(int n, int n2, int n3, int n4) {
        return (n * (n4 - n3) + n2 * n3) / n4;
    }

    private static int b(int n, int n2, int n3, int n4) {
        return d.a(n, n2, d.j(n3 * 65536 / n4 - 32768) + 16384 >> 7, 256);
    }

    private static int c(int n, int n2, int n3, int n4) {
        n3 = (4 * n3 * n3 / n4 - n3) / 3;
        return d.a(n, n2, n3, n4);
    }

    private static int d(int n, int n2, int n3, int n4) {
        n3 = (n4 * 3 * n3 * n3 / n4 + (n4 - n4 * 3) * n3) / n4;
        return d.a(n, n2, n3, n4);
    }

    private static long a(long l) {
        long l2 = 0L;
        int n = 0;
        while (l >= 0x80000000L) {
            l >>= 2;
            ++n;
        }
        int n2 = 31;
        while (n2 >= 0) {
            long l3 = l2 | 1L << n2;
            if (l3 * l3 <= l) {
                l2 |= l3;
            }
            --n2;
        }
        return l2 << n;
    }

    private static String a(int n, int n2) {
        return d.a(n, n2, false);
    }

    private static String a(int n, int n2, boolean bl) {
        if (n == -1) {
            return a[47];
        }
        if ((n2 & 4) != 0) {
            int n3 = n / 100;
            int n4 = n % 100;
            return "" + n3 + '.' + (n4 < 10 ? "0" : "") + n4 + (bl ? "s" : "");
        }
        int n5 = n / 100;
        int n6 = n % 100;
        return "" + n5 + '.' + (n6 < 10 ? "0" : "") + n6 + (bl ? "m" : "");
    }

    private static int i(int n, int n2) {
        if (n == 0) {
            if (n2 > 0) {
                return 256;
            }
            return 0;
        }
        if (n < 0) {
            return 511 - (n2 < 0 ? (n << 7) / (n + n2) : (n - n2 - n2 << 7) / (n - n2));
        }
        if (n2 < 0) {
            return (n << 7) / (n - n2);
        }
        return (n + n2 + n2 << 7) / (n + n2);
    }

    private static void r(int n) {
        n = d.b(n, 0, 65536);
        int n2 = 109 + (n * 9 >> 16);
        int n3 = 0;
        while (n3 < 320) {
            int n4 = 0;
            while (n4 < 240) {
                d.a(n2, n4, n3, 20, 0);
                n4 += 24;
            }
            n3 += 24;
        }
    }

    private static void i(int n, int n2) {
        b.setColor(0);
        b.fillRect(0, 319 - d.e(0), 240, 320 - (319 - d.e(0)));
        d.e(n, n2);
    }

    private static void e(int n, int n2, int n3, int n4) {
        b.setColor(0xFFFFFF);
        b.fillRect(n + 8, n2, n3 - 16, 8);
        b.fillRect(n, n2 + 8, n3, n4 - 16);
        b.fillRect(n + 8, n2 + n4 - 8, n3 - 16, 8);
        b.setColor(0);
        b.drawLine(n + 8, n2, n + n3 - 8 - 1, n2);
        b.drawLine(n + n3 - 1, n2 + 8, n + n3 - 1, n2 + n4 - 8 - 1);
        b.drawLine(n, n2 + 8, n, n2 + n4 - 8 - 1);
        b.drawLine(n + 8, n2 + n4 - 1, n + n3 - 8 - 1, n2 + n4 - 1);
        b.drawLine(n + 1, n2 + 8, n + 1, n2 + n4 - 8 - 1);
        b.drawLine(n + 8, n2 + n4 - 2, n + n3 - 8 - 1, n2 + n4 - 2);
        d.a(37, n, n2, 20, 0);
        d.a(38, n + n3, n2, 24, 0);
        d.a(39, n + n3, n2 + n4, 40, 0);
        d.a(40, n, n2 + n4, 36, 0);
    }

    private static void a(int n, int n2, int n3, String string, String string2, int n4) {
        int n5 = 120;
        int n6 = 0;
        int n7 = 0;
        String string3 = "";
        String string4 = "";
        int n8 = 0;
        if (string2 != null) {
            n5 = d.a(3, string2 + ' ') + 24;
        }
        if (n5 < d.a(8196, string + ' ') + 24) {
            n5 = d.a(8196, string + ' ') + 24;
        }
        if (aI == 2) {
            n8 = bb - 500;
            if (n8 < 0) {
                n8 = 0;
            } else if (n8 > 1500) {
                n8 = 1500;
            }
            n7 = aU + (aT - aU) * n8 / 1500;
            string3 = "" + n7;
            string4 = " " + a[53] + ' ' + aS;
            if (n4 >= 0) {
                n6 = 0 + d.a(6, "" + aT + ' ');
                n6 += d.a(3, string4);
                n6 += 24;
            }
        }
        if (n5 < n6) {
            n5 = n6;
        }
        if (n5 > 240) {
            n5 = 240;
        }
        int n9 = 0;
        if (n3 >= 0) {
            n9 = d.b(n3) / 2 + 3;
        }
        int n10 = n9 + 9;
        if (string2 != null) {
            n10 += 22;
        }
        n10 = aI == 2 ? (n4 < 0 ? (n10 += 56) : (n10 += 56)) : (n4 != -1 ? (n10 += 25 + d.b(n4) / 2) : (n10 += 22));
        if (n2 > 319 - d.e(0) - n10) {
            n2 = 319 - d.e(0) - n10;
        }
        int n11 = n2 + 6;
        d.e(n - n5 / 2, n2, n5, n10);
        if (n3 >= 0) {
            if (aI == 2) {
                d.a(n3, n, n11 - d.b(n3) / 6, 3, 0);
            } else {
                d.a(n3, n, n11, 3, 0);
            }
            n11 += d.b(n3) / 2 + 3 + 22;
        }
        d.a(8196, string, n, n11, 33);
        n11 += 22;
        if (string2 != null) {
            d.a(3, string2, n, n11, 33);
        }
        if (aI == 2) {
            n11 += 31;
            if (n4 < 0) {
                d.a(6, "" + n7, n, n11, 33);
            } else {
                int n12 = d.a(6, string3);
                int n13 = n12 + d.a(3, string4);
                d.a(6, string3, n -= n13 >> 1, n11 + 3, 36);
                d.a(3, string4, n += n12, n11, 36);
            }
            return;
        }
        if (n4 != -1) {
            n11 = n2 + n10;
            d.a(n4, n, n11, 3, 0);
        }
    }

    private static void P() {
        if (ba != 3) {
            d.h(70);
        }
    }

    private static void e(int n, int n2, int n3) {
        int n4 = (10 * h[t & 0x1FF] >> 14) / 4;
        d.a(422, n - (n3 / 2 + 6) - n4, n2 += 11, 10, 0);
        d.a(422, n + (n3 / 2 + 6) + n4, n2, 6, 4);
    }

    private static void b(int n, int n2, int n3, int n4, int n5, int n6, int n7, int n8) {
        if (n4 <= n) {
            return;
        }
        int n9 = (n4 - n + 1 << 8) / 20;
        int n10 = (n9 >> 8) + 1;
        int n11 = n7 >> 16 & 0xFF;
        int n12 = n7 >> 8 & 0xFF;
        int n13 = n7 & 0xFF;
        int n14 = ((n8 >> 16 & 0xFF) - n11 << 8) / 20;
        int n15 = ((n8 >> 8 & 0xFF) - n12 << 8) / 20;
        int n16 = ((n8 & 0xFF) - n13 << 8) / 20;
        n11 <<= 8;
        n12 <<= 8;
        n13 <<= 8;
        int n17 = (n5 - n2 << 8) / 20;
        int n18 = (n6 - n3 << 8) / 20;
        n2 = (n2 << 8) + 128;
        n3 = (n3 << 8) + 128;
        int n19 = n << 8;
        while (n19 < n4 + 1 << 8) {
            if (n19 >> 8 >= 320) {
                return;
            }
            b.setColor(n11 << 8 & 0xFF0000 | n12 & 0xFF00 | n13 >> 8 & 0xFF);
            b.fillRect(n2 >> 8, n19 >> 8, (n3 >> 8) - (n2 >> 8) + 1, n10);
            n11 += n14;
            n12 += n15;
            n13 += n16;
            n2 += n17;
            n3 += n18;
            n19 += n9;
        }
    }

    private static void b(int n, int n2, int n3, int n4, int n5, int n6) {
        int n7;
        int n8;
        int n9;
        int n10;
        if (n2 < n5) {
            n10 = n2;
            n9 = n5;
        } else {
            n10 = n5;
            n9 = n2;
        }
        if (n3 < n6) {
            n8 = n3;
            n7 = n6;
        } else {
            n8 = n6;
            n7 = n3;
        }
        d.a(n10, n, n7 - n10 + 1, n4 - n + 1);
        b.fillRect(n9, n, n8 - n9 + 1, n4 - n + 1);
        if (n10 < 240 && n9 >= 0) {
            if (n2 < n5) {
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n5 + 1, n, n2, n, n5 + 1, n4 + 1, b.getColor() & 0xFFFFFF | 0xFF000000);
            } else {
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n2 + 1, n4 + 1, n5, n4 + 1, n2 + 1, n, b.getColor() & 0xFFFFFF | 0xFF000000);
            }
        }
        if (n8 < 240 && n7 >= 0) {
            if (n3 > n6) {
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n6, n, n3 + 1, n, n6, n4 + 1, b.getColor() & 0xFFFFFF | 0xFF000000);
            } else {
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n3, n4 + 1, n6 + 1, n4 + 1, n3, n, b.getColor() & 0xFFFFFF | 0xFF000000);
            }
        }
        d.a(0, 0, 240, 320);
    }

    private static void b(int n, int n2, int n3, int n4, int n5) {
        d.l(n, 0, n2);
        int n6 = bA;
        int n7 = bB;
        d.l(n + n3, 0, n2);
        int n8 = bA;
        d.l(n + n3, 0, n2 + n4);
        int n9 = bA;
        d.l(n, 0, n2 + n4);
        int n10 = bA;
        int n11 = bB;
        b.setColor(n5);
        d.b(n7, n6, n8, n11, n10, n9);
    }

    private static void f(int n, int n2, int n3) {
        d.f(n, n2, n3, 0);
    }

    private static void f(int n, int n2, int n3, int n4) {
        int n5 = d.a(n);
        if (n5 == 0) {
            return;
        }
        d.l(0, 0, n2);
        int n6 = bA;
        n6 = n6 % (n5 += n3) - n5;
        int n7 = d.b();
        if (n7 == 1) {
            n7 = 2;
            n4 *= 2;
        }
        int n8 = 0;
        int n9 = 0;
        int n10 = 0;
        if (n4 > 0 && n7 > 1) {
            if (n7 == 2) {
                n9 = n8 = ch / n4 % n7;
            } else {
                n8 = ch / n4 % n7;
                n9 = (n8 + 1) % n7;
                n10 = (ch % n4 << 16) / n4;
            }
        }
        while (n6 < 240) {
            d.a(n, n8, n9, n10, n6 + n3, bB + 1, 1024, 0);
            n6 += n5;
        }
    }

    private static void g(int n, int n2, int n3) {
        int n4 = d.c(n);
        if (n4 == 0) {
            return;
        }
        d.l(n2, 0, n2);
        int n5 = bA;
        n5 = n5 % (n4 += n3) - n4;
        while (n5 < 240) {
            d.a(n, n5, bB, 36, 0);
            n5 += n4;
        }
    }

    private static void g(int n, int n2, int n3, int n4) {
        d.c(n, n2, n3, n4, 0);
    }

    private static void c(int n, int n2, int n3, int n4, int n5) {
        int n6 = d.a(n);
        if (n6 == 0) {
            return;
        }
        d.l(0, 0, n2);
        int n7 = bA;
        n7 = n7 % (n6 += n3) - n6;
        int n8 = d.b();
        if (n8 == 1) {
            n8 = 2;
            n5 *= 2;
        }
        int n9 = 0;
        int n10 = 0;
        int n11 = 0;
        if (n5 > 0 && n8 > 1) {
            if (n8 == 2) {
                n10 = n9 = ch / n5 % n8;
            } else {
                n9 = ch / n5 % n8;
                n10 = (n9 + 1) % n8;
                n11 = (ch % n5 << 16) / n5;
            }
        }
        while (n7 < 240) {
            d.a(n, n9, n10, n11, n7 + n3, n4 + 1, 1024, 0);
            n7 += n6;
        }
    }

    private static void h(int n, int n2, int n3, int n4) {
        int n5 = d.a(n);
        if (n5 == 0) {
            return;
        }
        d.l(n4, 0, n2);
        int n6 = bA;
        if (n6 > n6 % (n5 += n3) + n5) {
            n6 = n6 % n5 + n5;
        }
        while (n6 > 0) {
            d.a(n, 0, (n6 -= n5) + n3, bB + 1, 1024, 0);
        }
    }

    private static void i(int n, int n2, int n3, int n4) {
        int n5 = d.a(n);
        if (n5 == 0) {
            return;
        }
        n5 += n3;
        d.l(0, 0, n4);
        if (bB < 0) {
            return;
        }
        d.a(0, 0, 240, bB);
        d.l(0, 0, n2);
        int n6 = bA;
        n6 = n6 % n5 - n5;
        while (n6 < 240) {
            d.a(n, 0, n6 + n3, bB + 1, 1024, 0);
            n6 += n5;
        }
        d.a(0, 0, 240, 320);
    }

    private static int b() {
        if (aQ != 1 || (bc & 1) != 0) {
            return 4;
        }
        if (ba == 10) {
            return 1;
        }
        if (ba == 11) {
            if (aM == 1) {
                if (d.c.c < -1048576) {
                    return 2;
                }
                return 4;
            }
            if (aM == 3) {
                if (d.e.c < -1310720) {
                    return 2;
                }
                return 4;
            }
            if (aM == 4) {
                if (d.d.c < -1966080) {
                    return 2;
                }
                return 4;
            }
            return 4;
        }
        if (ba == 12) {
            int n = d.l(aZ, (bc & 1) != 0 ? 3 : bd);
            int n2 = d.m(aM, n);
            if (n2 >= E[1]) {
                return 4;
            }
            return 1;
        }
        return 4;
    }

    private static void Q() {
        a = new a[13];
        r = new int[20];
        int n = 0;
        while (n < 13) {
            d.a[n] = new a();
            int[] nArray = d.a(d[96 + (12 + n) * 3], d[96 + ((12 + n) * 3 + 1)], d[96 + ((12 + n) * 3 + 2)], null);
            d.a[n].a = a[nArray[0]];
            d.a[n].a = nArray[1];
            d.a[n].b = nArray[2];
            d.a[n].a = new int[20];
            int n2 = 0;
            while (n2 < 20) {
                d.a[n].a[n2] = nArray[3 + n2];
                ++n2;
            }
            if (n == 12) {
                n2 = 0;
                while (n2 < 20) {
                    d.r[n2] = nArray[23 + n2];
                    ++n2;
                }
            }
            ++n;
        }
        p = d.a(0, 646, 240, null);
        q = d.a(0, 886, 168, null);
    }

    private static void a(e e2, a a2, boolean bl) {
        e2.a = a2.a;
        e2.a = a2.a;
        e2.b = a2.b;
        e2.c = p[e2.a * 5];
        e2.d = p[e2.a * 5 + 2];
        int n = p[e2.a * 5 + 1];
        int n2 = p[e2.a * 5 + 3];
        int n3 = q[e2.b * 2 + e2.c];
        e2.a = d.a(e2.c, n, n2, n3);
        if (bl) {
            e2.b = new int[20];
            int n4 = 0;
            while (n4 < 20) {
                e2.b[n4] = a2.a[n4];
                ++n4;
            }
        }
    }

    private static void R() {
        int n;
        a = new e[6];
        if (aI == 0) {
            d.a[0] = new e();
            d.a[d.aJ].a = null;
            d.a[d.aJ].a = d.b[0].a;
            d.a(a[0], a[aJ], false);
        } else {
            n = 0;
            while (n < aN) {
                d.a[n] = new e();
                d.a(a[n], b[n], false);
                ++n;
            }
        }
        if (aI == 0 && aK == 1) {
            d.a[1] = new e();
            n = 0;
            int n2 = 0;
            while (n2 < 5 && (aL & 1 << n) == 0) {
                ++n;
                ++n2;
            }
            d.a(a[1], a[2 + aJ * 5 + n], true);
            n2 = 1;
            int n3 = 0;
            while (n2 < 5) {
                d.a[1 + n2] = new e();
                if (n3 == n) {
                    ++n3;
                }
                d.a(a[1 + n2], a[2 + aJ * 5 + n3], true);
                ++n2;
                ++n3;
            }
            return;
        }
        if (aI == 2) {
            d.a[1] = new e();
            d.a(a[1], a[12], true);
            d.a[1].a = null;
            d.a[1].a = a[82 + (aS - 1) % 10];
            n = 0;
            while (n < 20) {
                d.a[1].b[n] = d.a[1].b[n] + r[n] * (aS - 1);
                if (d.a[1].b[n] > 100) {
                    d.a[1].b[n] = 100;
                }
                ++n;
            }
            return;
        }
        n = 0;
        while (n < 6 - aN) {
            d.a[d.aN + n] = new e();
            d.a(a[aN + n], a[2 + aJ * 5 + n], true);
            ++n;
        }
    }

    private static void S() {
        int n = aI == 2 ? 2 : 6;
        int n2 = 0;
        while (n2 < n) {
            d.a[n2].a = null;
            d.a[n2].a = null;
            if (n2 >= aN) {
                d.a[n2].b = null;
            }
            d.a[n2] = null;
            ++n2;
        }
        a = null;
    }

    private static void T() {
        b = new a[6];
        int n = 0;
        while (n < 6) {
            d.b[n] = new a();
            ++n;
        }
        d.U();
    }

    private static void U() {
        try {
            byte[] byArray = d.a("n");
            if (byArray == null) {
                d.W();
            } else {
                a = new ByteArrayInputStream(byArray);
                b = new DataInputStream(a);
                aw = b.readInt();
                int n = 0;
                while (n < 6) {
                    d.b[n].a = null;
                    d.b[n].a = b.readUTF();
                    d.b[n].a = b.readInt();
                    d.b[n].b = b.readInt();
                    ++n;
                }
                a = null;
                b = null;
            }
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void V() {
        try {
            a = new ByteArrayOutputStream();
            a = new DataOutputStream(a);
            a.writeInt(aw);
            int n = 0;
            while (n < 6) {
                a.writeUTF(d.b[n].a);
                a.writeInt(d.b[n].a);
                a.writeInt(d.b[n].b);
                ++n;
            }
            byte[] byArray = a.toByteArray();
            d.a("n", byArray);
            a = null;
            a = null;
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    /*
     * Unable to fully structure code
     */
    private static void W() {
        d.aw = 1;
        var0 = 0;
        var1_1 = 0;
        ** GOTO lbl14
        {
            var1_1 = (var1_1 + 1) % 12;
            do {
                if (d.G[var1_1] != var1_1) continue block0;
                d.b[var0].a = null;
                d.b[var0].a = d.a(d.a[30], "" + (var0 + 1), 1);
                d.b[var0].b = var0;
                d.b[var0].a = var1_1;
                var1_1 = (var1_1 + 1) % 12;
                ++var0;
lbl14:
                // 2 sources

            } while (var0 < 6);
        }
        d.V();
    }

    private static void X() {
        as = 0;
        au = -1;
        av = -1;
        at = as;
    }

    private static void Y() {
        int n;
        boolean bl = false;
        boolean bl2 = false;
        int n2 = 0;
        int n3 = 0;
        int n4 = t - cd;
        if (as == 0 && (at == 0 || at == 1 || at == aw + 1)) {
            if (n4 > 200) {
                n2 -= 12;
            } else {
                if (at != aw + 1) {
                    n3 = 0 - d.c(0, 12, 200 - n4, 200);
                }
                n2 = 0 - d.c(12, 0, 200 - n4, 200);
            }
        }
        if (as == 1 && (at == 0 || at == 1 || at == 2)) {
            if (n4 > 200) {
                n3 -= 12;
            } else {
                if (at != 2) {
                    n2 -= d.c(0, 12, 200 - n4, 200);
                }
                n3 -= d.c(12, 0, 200 - n4, 200);
            }
        }
        if (as == aw + 1 && at == 0 && n4 < 200) {
            n2 -= d.c(0, 12, 200 - n4, 200);
        }
        if (as == 2 && at == 1 && n4 < 200) {
            n3 -= d.c(0, 12, 200 - n4, 200);
        }
        int n5 = 0;
        int n6 = 0;
        n6 = 60 + d.e(t - cc, 56);
        if (as == 0) {
            d.a(true, n6, 56, false);
        }
        d.a(J == 1 || u ? 3 : 8195, a[133], n6 += n2, 59, 20);
        if (f[G] != 13) {
            n6 = 60 + d.e(t - cc, 84);
            String string = a[146] + ' ';
            if (as == 1) {
                d.a(true, n6, 84, false);
            }
            d.a(J == 1 || u ? 3 : 8195, string, n6 += n3, 87, 20);
            n = d.a(J == 1 || u ? 3 : 8195, "" + aw);
            d.a(J == 1 || u ? 3 : 8195, "" + aw, n6 += n + d.a(J == 1 || u ? 3 : 8195, string) + 10 + 5, 87, 17);
            if (as == 1) {
                d.e(n6, 87, n);
            }
        }
        n5 = 0;
        if (3 + (as + 1) * 28 > 319 - d.e(0) - 1 - 22 - 56) {
            n5 = (56 + as * 28 - (319 - d.e(0) - 1 - 22 - 56)) / 28;
        }
        if (n5 > 0) {
            bl = true;
        }
        b.setColor(0xBFBFBF);
        n6 = 319 - d.e(0) - 28;
        boolean bl3 = false;
        if (n6 > 283) {
            n6 = 283;
        }
        n = 143;
        while (n <= n6) {
            b.fillRect(10, n, 220, 1);
            if (f[G] == 13) break;
            n += 28;
        }
        n = n5;
        while (n < aw) {
            int n7 = 59 + (2 + n - n5) * 28;
            if (n7 + 28 > 56 + (319 - d.e(0) - 1 - 22 - 56)) {
                bl2 = true;
                break;
            }
            int n8 = 3 + d.e(t - cc, n7);
            if (as == 2 + n) {
                d.a(true, n8, n7, true);
            }
            if (n == au && av == 0) {
                d.a(d.b[n].a + 54 - 1, n8 += 10, n7 + 28 - 3 + 8, 36, 0);
                d.e((n8 += 50) - 25, n7 + 3, 38);
                n8 += 13;
            } else {
                d.a(d.b[n].a + 42 - 1, n8, n7 + 28 - 3, 36, 0);
                n8 += 28;
            }
            if (n == au && av == 1) {
                d.j(n8, n7);
            } else {
                d.a(J == 1 || u ? 3 : 8195, d.b[n].a, n8, n7 + 3, 20);
            }
            if (n == au && av == 2) {
                n8 = 187 + d.e(t - cc, n7);
                d.a(d.b[n].b + 65, n8, n7 + 28 - 3, 36, 0);
                d.e(n8 + 20 - 3, n7 + 3, 31);
            } else {
                n8 = 207 + d.e(t - cc, n7);
                d.a(d.b[n].b + 86, n8, n7 + 28 - 3, 36, 0);
            }
            if (f[G] == 13) break;
            ++n;
        }
        d.c(bl, bl2);
        if (au < 0) {
            d.e(as == 0 ? 5 : 7, 6);
            return;
        }
        d.e(25, av == 1 ? 27 : 6);
    }

    private static void s(int n) {
        boolean bl = d.a(n, 196616);
        boolean bl2 = d.a(n, 131076);
        boolean bl3 = d.a(n, 131078);
        boolean bl4 = d.a(n, 131072);
        boolean bl5 = d.a(n, 131074);
        boolean bl6 = d.a(n, 43);
        if (au < 0) {
            if (bl6) {
                d.a(29, 128, 1);
                d.C();
                d.q();
            }
            if (as == 0) {
                if (bl) {
                    d.a(28, 128, 1);
                    d.V();
                    aN = aw;
                    if (f[G] == 13) {
                        aI = 2;
                        s = true;
                    } else if (aw < 6) {
                        aO = 6 - aw;
                        d.l(9);
                        d.q();
                    } else {
                        aO = 0;
                        d.l(10);
                        d.q();
                    }
                }
            } else if (as == 1) {
                if (bl2) {
                    d.a(27, 128, 1);
                    if (--aw < 1) {
                        aw = 6;
                    }
                } else if (bl3 || bl) {
                    d.a(27, 128, 1);
                    if (++aw > 6) {
                        aw = 1;
                    }
                }
            } else if (bl) {
                d.a(28, 128, 1);
                au = as - 2;
                av = 0;
            }
            if (bl4) {
                d.a(27, 128, 1);
                cd = t;
                at = as--;
                if (f[G] == 13) {
                    if (as == 1) {
                        --as;
                    }
                    if (as < 0) {
                        as = 2;
                    }
                } else if (as < 0) {
                    as = 2 + aw - 1;
                }
            } else if (bl5) {
                d.a(27, 128, 1);
                cd = t;
                at = as++;
                if (f[G] == 13) {
                    if (as == 1) {
                        ++as;
                    }
                    if (as >= 3) {
                        as = 0;
                    }
                } else if (as >= 2 + aw) {
                    as = 0;
                }
            }
            return;
        }
        if (av == 0) {
            if (bl2) {
                d.a(27, 128, 1);
                do {
                    --d.b[d.au].a;
                    if (d.b[d.au].a >= 0) continue;
                    d.b[d.au].a = 11;
                } while (G[d.b[d.au].a] != d.b[d.au].a);
            } else if (bl3) {
                d.a(27, 128, 1);
                do {
                    ++d.b[d.au].a;
                    if (d.b[d.au].a < 12) continue;
                    d.b[d.au].a = 0;
                } while (G[d.b[d.au].a] != d.b[d.au].a);
            } else if (bl) {
                d.a(28, 128, 1);
                av = 1;
                String string = d.a(a[29], d.b[d.au].a, 22, 8).toUpperCase();
                d.a(28, 128, 1);
                if (string.length() > 0) {
                    d.b[d.au].a = null;
                    d.b[d.au].a = string;
                }
                av = 2;
            } else if (bl6) {
                d.a(29, 128, 1);
                au = -1;
            }
            return;
        }
        if (av != 1) {
            if (bl2) {
                d.a(27, 128, 1);
                --d.b[d.au].b;
                if (d.b[d.au].b < 0) {
                    d.b[d.au].b = 20;
                }
                return;
            }
            if (bl3) {
                d.a(27, 128, 1);
                ++d.b[d.au].b;
                if (d.b[d.au].b >= 21) {
                    d.b[d.au].b = 0;
                }
                return;
            }
            if (bl || bl6) {
                d.a(28, 128, 1);
                au = -1;
                cd = t;
                at = as++;
                if (f[G] == 13) {
                    if (as >= 3) {
                        as = 0;
                    }
                    return;
                }
                if (as >= 2 + aw) {
                    as = 0;
                }
            }
        }
    }

    private static void Z() {
        d.ch();
        d.D();
        if (f[G] == 6) {
            d.co();
            return;
        }
        d.Y();
    }

    private static void j(int n, int n2) {
        int n3 = d.a(J == 1 || u ? 3 : 8195, "WWWWWWWW");
        int n4 = n2 + 3 - 1;
        int n5 = n + 3;
        int n6 = n + n3 - 3;
        b.setColor(0);
        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n5, n4, n5 - 6, n4 + 24, n5, n4 + 24, b.getColor() & 0xFFFFFF | 0xFF000000);
        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n6 + 6, n4, n6, n4 + 24, n6, n4, b.getColor() & 0xFFFFFF | 0xFF000000);
        b.fillRect(n5, n4, n6 - n5, 24);
        String string = e.toUpperCase();
        if (e) {
            if (string.length() < 8) {
                string = string + '_';
            } else {
                int n7 = (d.a(J == 1 || u ? 3 : 8195, string.substring(7, 8)) - d.a(J == 1 || u ? 3 : 8195, "_")) / 2;
                d.a(J == 1 || u ? 3 : 8195, "_", n + d.a(J == 1 || u ? 3 : 8195, string) - n7, n2 + 3, 24);
            }
        }
        d.a(J == 1 || u ? 3 : 8195, string, n, n2 + 3, 20);
    }

    private static void aa() {
        ay = 0;
        ax = 0;
        az = 0;
        d.b(ax);
        aB = -1;
        aC = 0;
        B = true;
    }

    private static void ab() {
        int n = ax == 6 ? 128 : (ax == 5 ? 98 : 93 + ax);
        int n2 = 35;
        b.setColor(16715817);
        b.fillRect(0, 35, 240, 23);
        b.setColor(0xDBDBDB);
        b.fillRect(0, 59, 240, 319 - d.e(0) - 35 - 22 - 1 - 1);
        d.a(0, 4, 4, 20, 0);
        int n3 = d.a(8195, a[n]);
        int n4 = 0;
        d.a(8195, a[n], 147, 35, 17);
        n2 += 23;
        n2 += 28;
        boolean bl = false;
        if (ax == 6 || ay == 6) {
            bl = true;
        }
        int n5 = 0;
        if (t - aB < 240 && B) {
            n5 = t - aB;
            if (ay != 6) {
                bl = false;
            }
        } else if (B) {
            d.b(ax);
            B = false;
            aB = t;
            n5 = -240;
            ay = ax;
            if (ax != 6) {
                bl = false;
            }
        } else if (t - aB < 240) {
            n5 = -(240 - (t - aB));
        } else {
            aB = -1;
        }
        int n6 = az;
        while (n6 < 10) {
            int n7 = 0;
            String string = b[n6];
            int n8 = i[n6];
            int n9 = i[n6 + 20];
            int n10 = -1;
            int n11 = -1;
            if (n9 >= 0) {
                n10 = 41 + (n9 >> 8 & 0xFF);
                n11 = 86 + (n9 & 0xFF);
            }
            String string2 = ay == 5 || ay == 6 ? "" + n8 : (ay == 0 || ay == 2 ? d.a(-n8, 4) : d.a(n8, 8));
            if (bl) {
                b.setColor(0xAAAAAA);
                b.fillRect(aC * n5, n2 - 25, 240, 28);
                b.setColor(0xFFFFFF);
                b.fillRect(aC * n5, n2 - 25, 240, 1);
            }
            n4 = 0 + (1 + aC * n5);
            d.a(3, "" + (n6 + 1) + '.', n4 += 3 + d.a(3, "10."), n2, 40);
            n4 += d.a(3, " ");
            if (bl) {
                n7 = n4;
            }
            n4 += 9;
            if (n11 >= 0) {
                d.a(n11, n4, n2, 36, 0);
            }
            n4 -= 12;
            if (n10 >= 0) {
                d.a(n10, n4, n2, 36, 0);
            }
            d.a(3, string, (n4 += 46) - 3, n2 + 1, 36);
            if (bl) {
                d.a(3, d.a(a[49], "" + (n9 >> 17), 1), n7 - 3, (n2 += 25) + 1, 36);
            }
            d.a(5, string2, aC * n5 + 240 - 6, n2 + 1, 40);
            if (bl) {
                n2 += 2;
            }
            aA = n6;
            if (bl ? n2 > 319 - d.e(0) - 22 - 3 : (n2 += 28) > 319 - d.e(0)) break;
            ++n6;
        }
        d.e(147, 35, n3);
        d.e(-1, 6);
        boolean bl2 = false;
        if (az > 0) {
            bl2 = true;
        }
        boolean bl3 = false;
        if (aA < 9) {
            bl3 = true;
        }
        d.c(bl2, bl3);
    }

    private static void t(int n) {
        boolean bl = d.a(n, 131076);
        boolean bl2 = d.a(n, 131078);
        boolean bl3 = d.a(n, 131072);
        boolean bl4 = d.a(n, 131074);
        boolean bl5 = d.a(n, 43);
        if (bl) {
            d.a(27, 128, 1);
            ay = ax--;
            if (ax < 0) {
                ax = 6;
            }
            aB = aB == -1 ? t : t - 240;
            aC = 1;
            az = 0;
            B = true;
            return;
        }
        if (bl2) {
            d.a(27, 128, 1);
            ay = ax++;
            if (ax > 6) {
                ax = 0;
            }
            aB = aB == -1 ? t : t - 240;
            aC = -1;
            az = 0;
            B = true;
            return;
        }
        if (bl3) {
            if (az > 0) {
                d.a(27, 128, 1);
                --az;
            }
            return;
        }
        if (bl4) {
            if (aA < 9) {
                d.a(27, 128, 1);
                ++az;
            }
            return;
        }
        if (bl5) {
            d.a(29, 128, 1);
            d.C();
            d.q();
        }
    }

    private static void ac() {
        d.g(0, 34433);
        t = d.a(0, 262, 84, null);
        u = d.a(0, 346, 84, null);
        v = d.a(0, 430, 84, null);
        w = d.a(0, 514, 84, null);
        int[] nArray = new int[58];
        int n = 0;
        while (n < 21) {
            nArray[n * 2] = u[n];
            nArray[n * 2 + 1] = n;
            ++n;
        }
        n = 0;
        int n2 = 21;
        while (n < 4) {
            nArray[n2 * 2] = 794 + n;
            nArray[n2 * 2 + 1] = n2;
            ++n;
            ++n2;
        }
        n = 0;
        n2 = 25;
        while (n < 4) {
            nArray[n2 * 2] = 612 + n;
            nArray[n2 * 2 + 1] = n2;
            ++n;
            ++n2;
        }
        d.a(0, nArray);
        n = 0;
        while (n < 21) {
            nArray[n * 2] = w[n];
            nArray[n * 2 + 1] = n;
            ++n;
        }
        n = 0;
        n2 = 21;
        while (n < 4) {
            nArray[n2 * 2] = 790 + n;
            nArray[n2 * 2 + 1] = n2;
            ++n;
            ++n2;
        }
        n = 0;
        n2 = 25;
        while (n < 4) {
            nArray[n2 * 2] = 624 + n;
            nArray[n2 * 2 + 1] = n2;
            ++n;
            ++n2;
        }
        d.a(1, nArray);
        i = d.a(0, 2378, 10, null);
        j = d.a(0, 2388, 34, null);
        k = d.a(0, 2422, 4, null);
        l = d.a(0, 2426, 22, null);
        m = d.a(0, 2448, 22, null);
        d.n = d.a(0, 2470, 42, null);
        o = d.a(0, 2512, 22, null);
        p = d.a(0, 2534, 20, null);
        q = d.a(0, 2554, 20, null);
        r = d.a(0, 2574, 18, null);
        s = d.a(0, 2592, 4, null);
        t = d.a(0, 2596, 22, null);
        u = d.a(0, 2618, 22, null);
        d.a(0, 2640, 10, null);
        v = d.a(0, 2650, 22, null);
        w = d.a(0, 2672, 12, null);
        x = d.a(0, 2684, 22, null);
        y = d.a(0, 2706, 22, null);
        z = d.a(0, 2728, 16, null);
        A = d.a(0, 2744, 10, null);
        B = d.a(0, 2754, 20, null);
        C = d.a(0, 2774, 12, null);
        D = d.a(0, 2786, 18, null);
        E = d.a(0, 2804, 30, null);
        F = d.a(0, 2834, 32, null);
        G = d.a(0, 2866, 46, null);
        H = d.a(0, 2912, 46, null);
        I = d.a(0, 2958, 46, null);
        J = d.a(0, 3004, 46, null);
        K = d.a(0, 3050, 18, null);
        L = d.a(0, 3068, 12, null);
        M = d.a(0, 3080, 30, null);
        d.a(0, 3110, 6, null);
        N = d.a(0, 3116, 16, null);
        O = d.a(0, 3132, 14, null);
        P = d.a(0, 3146, 10, null);
        Q = d.a(0, 3156, 10, null);
        R = d.a(0, 3166, 10, null);
        S = d.a(0, 3176, 140, null);
        T = d.a(0, 3316, 50, null);
        U = d.a(0, 3366, 16, null);
        V = d.a(0, 3382, 10, null);
        W = d.a(0, 3392, 10, null);
        X = d.a(0, 3402, 28, null);
        Y = d.a(0, 3430, 10, null);
        Z = d.a(0, 3440, 6, null);
        aa = d.a(0, 3446, 26, null);
        ab = d.a(0, 3472, 64, null);
        ac = d.a(0, 3536, 10, null);
        ad = d.a(0, 3546, 22, null);
    }

    private static void a(int n, int[] nArray, int n2, int n3, int n4, int n5, int n6) {
        boolean bl;
        short[] sArray;
        int n7 = 120;
        switch (n3) {
            case 1: {
                int n8 = 80 * (Z.length - 1);
                int n9 = n8 + 160 * (aa.length - 1);
                int n10 = n9 + 120 * (U.length - 1);
                if (n4 < n8) {
                    sArray = Z;
                    bl = false;
                    n7 = 80;
                    break;
                }
                if (n4 < n9) {
                    sArray = aa;
                    bl = true;
                    n7 = 80;
                    n4 -= n8;
                    break;
                }
                if (n4 < n10) {
                    sArray = U;
                    bl = false;
                    n7 = 120;
                    n4 -= n9;
                    break;
                }
                sArray = V;
                bl = true;
                n7 = 120;
                n4 -= n10;
                break;
            }
            case 2: {
                if (n4 < 120 * (X.length - 1)) {
                    sArray = X;
                    bl = false;
                    break;
                }
                sArray = Y;
                bl = true;
                n4 -= 120 * (X.length - 1);
                break;
            }
            case 4: {
                n7 = 80;
                if (n4 < 80 * (Z.length - 1)) {
                    sArray = Z;
                    bl = false;
                    break;
                }
                sArray = aa;
                bl = true;
                n4 -= 80 * (Z.length - 1);
                break;
            }
            case 5: {
                n7 = 120;
                if (n4 < 120 * (ab.length - 1)) {
                    sArray = ab;
                    bl = false;
                    break;
                }
                sArray = ac;
                bl = true;
                n4 -= 120 * (ab.length - 1);
                break;
            }
            default: {
                sArray = W;
                bl = true;
            }
        }
        d.a(n, nArray, n2, sArray, n4, n7, n5, n6, bl);
    }

    private static void a(int n, int[] nArray, int n2, short[] sArray, int n3, int n4, int n5, int n6, boolean bl) {
        int n7;
        int n8;
        int n9;
        if (bl) {
            if (n3 < 0) {
                n3 %= (sArray.length - 1) * n4;
            }
            if (n3 < 0) {
                n3 += (sArray.length - 1) * n4;
            }
            n9 = n3 / n4 % (sArray.length - 1);
            n8 = n9 + 1;
            n7 = (n3 % n4 << 16) / n4;
        } else {
            if (n3 < 0) {
                n3 = 0;
            }
            if (n3 >= n4 * (sArray.length - 1)) {
                n9 = n8 = sArray.length - 1;
                n7 = 0;
            } else {
                n9 = n3 / n4;
                n8 = n9 + 1;
                n7 = (n3 % n4 << 16) / n4;
            }
        }
        s = nArray;
        aD = n2;
        d.a(n, sArray[n9], sArray[n8], n7, n5, n6, 1024, 0);
    }

    private static void a(int n, int[] nArray, int n2, short[] sArray, int n3, int n4, int n5, int n6, int n7) {
        int n8;
        int n9;
        int n10;
        if (n7 >= 0) {
            int n11 = sArray.length - n7 - 1;
            if (n3 < 0) {
                n3 = 0;
            }
            n10 = n3 / n4;
            n9 = (n3 % n4 << 16) / n4;
            if (n10 >= n7) {
                n10 -= n7;
                n10 %= n11;
                n10 += n7;
            }
            n8 = n10 + 1;
        } else {
            if (n3 < 0) {
                n3 = 0;
            }
            if (n3 >= n4 * (sArray.length - 1)) {
                n10 = n8 = sArray.length - 1;
                n9 = 0;
            } else {
                n10 = n3 / n4;
                n8 = n10 + 1;
                n9 = (n3 % n4 << 16) / n4;
            }
        }
        s = nArray;
        aD = n2;
        d.a(n, sArray[n10], sArray[n8], n9, n5, n6, 1024, 0);
    }

    private static int[] a(int n, int n2, int n3, int n4) {
        int[] nArray = new int[29];
        int[] nArray2 = n == 0 ? (n2 == 0 ? t : u) : (n2 == 0 ? v : w);
        int n5 = 0;
        while (n5 < 21) {
            nArray[n5] = nArray2[n5];
            ++n5;
        }
        n5 = 0;
        while (n5 < 4) {
            nArray[21 + n5] = n3 + n5;
            ++n5;
        }
        n5 = 0;
        while (n5 < 4) {
            nArray[25 + n5] = n4 + n5;
            ++n5;
        }
        return nArray;
    }

    private static void a(int n, int n2, int[] nArray) {
        if (n == 0 || n == 1) {
            if (n2 >> 8 == 0) {
                int n3 = nArray[8];
                int n4 = n3 >> 3;
                int n5 = n3 & 7;
                if (n4 != -1 && s != null) {
                    n4 = s[n4];
                }
                nArray[8] = n4 << 3 | n5;
                n3 = d.d(n);
                if (j[26] == 0) {
                    nArray[1] = n3 + (nArray[1] - n3) * aD / 100;
                } else {
                    short s = d.c(n);
                    nArray[0] = s + (nArray[0] - s) * 70 / 100;
                    nArray[1] = n3 + (nArray[1] - n3) * 70 / 100;
                }
                return;
            }
            short s = d.d(n);
            int n6 = nArray[3];
            if (j[26] == 0) {
                nArray[1] = s - n6 + (nArray[1] - (s - n6)) * aD / 100;
                return;
            }
            short s2 = d.c(n);
            nArray[0] = s2 + (nArray[0] - s2) * 70 / 100;
            nArray[1] = s - n6 + (nArray[1] - (s - n6)) * 70 / 100;
        }
    }

    private static void j(int n, int n2, int n3, int n4) {
        if (aM == 4 && D) {
            d.a(b, n, n2 + n4 - 1, n3, -n4, false);
            return;
        }
        if (aM == 3 && C) {
            d.q(n, n2 + n4, d.i(n3, -n4));
        }
    }

    private static int l(int n) {
        if (n >= 1 && n <= 8 || n >= 10 && n <= 26 || n == 118 || n == 123 || n == 124 || n >= 129) {
            return 28;
        }
        return 0;
    }

    private static void h(int n, int n2, int n3) {
        int n4 = -1;
        if ((n >= 1 && n <= 8 || n >= 10 && n <= 26 || n == 118 || n == 123 || n == 124 || n >= 129) && n3 >= 56 && n3 < 319 - d.e(0) - 1 - 22 - 22) {
            int n5 = 35;
            int n6 = 34;
            int n7 = 36;
            if (n == 118 || n == 123 || n == 124) {
                n5 = 32;
                n6 = 31;
                n7 = 33;
                n -= 100;
            }
            if (n >= 129) {
                n5 = 32;
                n6 = 31;
                n7 = 33;
            }
            int n8 = 0;
            if (n >= 1 && n <= 8 || n >= 129) {
                n4 = n >= 129 ? 170 + (n - 121) : 170 + (n - 1);
            } else if (n >= 10 && n <= 26) {
                n4 = 180 + (n - 10);
            }
            int n9 = d.a(n >= 10 && n <= 26 || n >= 129 ? 3 : 8195, a[n4]) + 9;
            if (n >= 10 && n <= 26 || n >= 129) {
                n8 = 0 - ((n9 += 3) + d.c(n5) + d.c(n7)) / 2;
            }
            d.a(n5, n2 += n8, n3, 20, 0);
            d.a(n2 + d.c(n5), 0, n9, 320);
            int n10 = 0;
            while (n10 <= n9) {
                d.a(n6, n2 + d.c(n5) + n10, n3, 20, 0);
                n10 += d.c(n6);
            }
            d.a(0, 0, 240, 320);
            d.a(n7, n2 + d.c(n5) + n9, n3, 20, 0);
            if (n >= 10 && n <= 26 || n >= 129) {
                d.a(3, a[n4], n2 - n8, n3 + 1, 17);
                return;
            }
            d.a(8195, a[n4], n2 + d.c(n5), n3 + 1, 20);
        }
    }

    private static int m(int n) {
        if (n == 2) {
            return 8196;
        }
        return 3;
    }

    private static int c() {
        return 0xFFFFFF;
    }

    private static void ad() {
        if (ae == null) {
            ae = d.a(0, 3568, 224, null);
        }
        a = new b[1000];
        int n = 0;
        while (n < 1000) {
            d.a[n] = new b();
            d.a[n].a = -1;
            ++n;
        }
        aG = 0;
        aH = 0;
    }

    private static void ae() {
        int n = 0;
        while (n < 1000) {
            d.a[n].a = -1;
            ++n;
        }
        aG = 0;
        aH = 0;
    }

    private static void d(int n, int n2, int n3, int n4, int n5) {
        d.a(n, n2, n3, n4, n5, 0, 0, 1024, -1, -1);
    }

    private static void a(int n, int n2, int n3, int n4, int n5, int n6, int n7, int n8, int n9, int n10) {
        short[] sArray = ae;
        b[] bArray = a;
        if (n4 > 6) {
            n4 = 6;
        }
        int n11 = n5 << 7;
        int n12 = 16 * n3;
        int n13 = sArray[n12 + 1] * n8 >> 10;
        short s = sArray[n12 + 2];
        short s2 = 0;
        int n14 = 0;
        while (n13 > 0) {
            int n15;
            int n16;
            int n17;
            b b2 = bArray[aG];
            bArray[aG].a = n3;
            b2.h = sArray[n12];
            if (s2 == 0) {
                n17 = n9 != -1 ? n9 : sArray[n12 + 10];
                n16 = n10 != -1 ? n10 : sArray[n12 + 11];
                n15 = n16 - n17;
                n14 = n15 == 0 ? n17 : n17 + d.a() % (n16 - n17);
            }
            if (++s2 >= sArray[n12 + 3]) {
                s2 = 0;
            }
            n17 = 50 + d.a() % 50;
            b2.d = h[n14 & 0x1FF] * n17 >> 10;
            b2.e = h[n14 + 128 & 0x1FF] * n17 >> 10;
            n16 = sArray[n12 + 7];
            if (n16 != 0) {
                n15 = d.a() % (d.g(n16) << 4);
                if (n16 < 0) {
                    n15 = -n15;
                }
                b2.b = n + n15;
                b2.c = n2;
            } else {
                b2.b = n;
                b2.c = n2;
            }
            n16 = sArray[n12 + 4];
            b2.d = b2.d * n16 >> 8;
            b2.e = b2.e * n16 >> 8;
            switch (sArray[n12 + 5]) {
                case 1: {
                    b2.e = -(d.g(b2.e) + 2000);
                    break;
                }
                case 2: {
                    b2.e = d.g(b2.e) + 2000;
                    break;
                }
                case 3: {
                    b2.e = -d.g(b2.e);
                    break;
                }
                case 4: {
                    b2.d = n6;
                    b2.e = n7;
                }
            }
            b2.f = n;
            b2.g = n2;
            b2.k = n4;
            n16 = sArray[n12 + 9];
            n15 = n16 > 0 ? d.a() % n16 : d.a() & 0xF;
            b2.j = sArray[n12 + 6] + n15;
            b2.i = ch + (n11 >> 4);
            if (++aG >= 1000) {
                aG = 0;
            }
            ++aH;
            --n13;
            n11 += s;
        }
    }

    /*
     * Unable to fully structure code
     */
    private static void af() {
        if (d.aH == 0) {
            return;
        }
        var0 = d.ae;
        var1_1 = d.a;
        var2_2 = d.ch;
        var3_3 = 0;
        while (var3_3 < 1000) {
            block7: {
                block9: {
                    block8: {
                        var4_4 = var1_1[var3_3];
                        if (var4_4.a < 0 || var4_4.i > var2_2) break block7;
                        var5_5 = 16 * var4_4.a;
                        if (var2_2 < var4_4.i + var4_4.j) break block8;
                        var4_4.a = -1;
                        --d.aH;
                        break block7;
                    }
                    var4_4.b += var4_4.d;
                    var4_4.c += var4_4.e;
                    var7_7 = var0[var5_5 + 15];
                    if (var7_7 <= 0) break block9;
                    if (var4_4.c < var4_4.g) {
                        var4_4.c = var4_4.g;
                        var4_4.d = var4_4.d * var7_7 >> 8;
                        var4_4.e = -var4_4.e * var7_7 >> 8;
                    }
                    ** GOTO lbl-1000
                }
                if (var7_7 == -1 && var4_4.c < var4_4.g) {
                    var4_4.a = -1;
                    --d.aH;
                } else lbl-1000:
                // 2 sources

                {
                    if (var4_4.a == 5 && var4_4.b < -32768) {
                        var4_4.b += 311296;
                    }
                    if ((var6_6 = var0[var5_5 + 8]) != 0) {
                        var4_4.e += 150 * var6_6 >> 8;
                    }
                }
            }
            ++var3_3;
        }
    }

    private static void ag() {
        if (aH == 0) {
            return;
        }
        b[] bArray = a;
        int n = 0;
        while (n < 1000) {
            b b2 = bArray[n];
            if (b2.a >= 0 && b2.i <= ch) {
                int n2 = (ch - b2.i << 8) / b2.j;
                if (n2 > 256) {
                    n2 = 256;
                }
                int n3 = b2.b;
                int n4 = b2.c;
                int n5 = 16 * b2.a;
                int n6 = ae[n5 + 12];
                short s = ae[n5 + 13];
                if (s > 0) {
                    n6 = n6 * n2 >> 8;
                } else if (s < 0) {
                    n6 = n6 * (256 - n2) >> 8;
                }
                if (n6 > 0) {
                    n3 += (aE - b2.f) * n6 >> 14;
                    n4 += (aF - b2.g) * n6 >> 14;
                }
                int n7 = n3 >> 10;
                int n8 = n4 >> 10;
                if (ae[n5 + 14] != 0) {
                    d.l(n7, n8, 0);
                    n7 = bA;
                    n8 = bB;
                }
                switch (b2.h) {
                    case 0: 
                    case 1: 
                    case 2: {
                        d.a(463 + (n2 * 2 >> 8), n7, n8, 3, 0);
                        break;
                    }
                    case 3: {
                        int n9 = n2 * 3 >> 7;
                        if (n9 > 3) {
                            n9 = 3;
                        }
                        int n10 = b2.d + b2.e >> 2;
                        int n11 = n * 31 & 3;
                        if (n10 > 0) {
                            n11 = (ch >> 6) + n * 31 & 3;
                        }
                        if (n11 == 1) {
                            n11 = 1;
                        } else if (n11 == 2) {
                            n11 = 2;
                        } else if (n11 == 3) {
                            n11 = 3;
                        }
                        d.a(458 + n9, n7, n8, 3, n11);
                        break;
                    }
                    case 4: {
                        int n9 = n2 * 10 >> 8;
                        if (n9 > 2) {
                            n9 = 2;
                        }
                        d.a(466 + n9, n7, n8, 3, 0);
                        break;
                    }
                    case 5: {
                        int n9 = b2.k;
                        int n10 = (n9 % 3 << 6) + (255 >> n9++ % 3);
                        int n11 = (n9 % 3 << 6) + (255 >> n9++ % 3);
                        int n12 = (n9 % 3 << 6) + (255 >> n9 % 3);
                        b.setColor(n10 << 16 | n11 << 8 | n12);
                        int n13 = n7 + (h[(ch >> 3) + n & 0x1FF] * 450 >> 20);
                        int n14 = n8 + (h[(ch >> 2) + n * 3 & 0x1FF] * 450 >> 20);
                        int n15 = n7 + (h[(ch >> 2) + n * 7 + 35 & 0x1FF] * 450 >> 20);
                        int n16 = n8 - (h[(ch >> 2) + n * 6 + 197 & 0x1FF] * 450 >> 20);
                        int n17 = n7 + (h[(ch >> 3) + n * 31 + 124 & 0x1FF] * 450 >> 20);
                        int n18 = n8 + (h[(ch >> 3) + n * 69 + 230 & 0x1FF] * 450 >> 20);
                        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n13, n14, n15, n16, n17, n18, b.getColor() & 0xFFFFFF | 0xFF000000);
                    }
                }
            }
            ++n;
        }
    }

    private static void ah() {
        x = new int[36];
        y = new int[6];
        n = new byte[5];
    }

    private static void ai() {
        x = null;
        y = null;
        n = null;
    }

    private static void aj() {
        byte by = n[4];
        int n = 0;
        do {
            int n2 = d.a() % 5;
            int n3 = (n2 + (1 + d.a() % 3)) % 5;
            byte by2 = d.n[n2];
            d.n[n2] = d.n[n3];
            d.n[n3] = by2;
        } while (++n < 5 || d.n[0] == by);
    }

    private static void ak() {
        int n = 0;
        while (n < x.length) {
            d.x[n] = 0;
            ++n;
        }
        n = 0;
        while (n < 6) {
            d.y[n] = n;
            ++n;
        }
        if (aI == 2) {
            b = 0;
            aM = d.n[b];
        } else {
            aM = -1;
            n = 0;
            while (n < 5 && aM < 0) {
                if ((aL & 1 << n) != 0) {
                    aM = n;
                }
                ++n;
            }
        }
        d.R();
        int n2 = 0;
        if (aI == 2) {
            n2 = 1;
        }
        d.u(1 - n2);
    }

    private static void al() {
        d.S();
    }

    private static void am() {
        d.a("s");
        d.a("e");
        d.d(false);
    }

    private static void an() {
        try {
            byte[] byArray = d.a("s");
            if (byArray == null) {
                d.ak();
                d.as();
                d.au();
                d.ao();
            } else {
                int n;
                a = new ByteArrayInputStream(byArray);
                b = new DataInputStream(a);
                int n2 = b.readInt();
                aI = b.readInt();
                cl = b.readInt();
                aJ = b.readInt();
                aK = b.readInt();
                aL = b.readInt();
                aN = b.readInt();
                aO = b.readInt();
                aP = aN + aO;
                aM = b.readInt();
                if (aI == 0) {
                    K = true;
                    n = 6 + (aJ == 0 ? 0 : 6) + cl;
                    cs = j[n];
                    cr = -1;
                } else if (aI == 1) {
                    L = true;
                }
                n = 0;
                while (n < x.length) {
                    d.x[n] = b.readInt();
                    ++n;
                }
                d.aq();
                aS = b.readInt();
                aT = b.readInt();
                b = b.readByte();
                n = 0;
                while (n < d.n.length) {
                    d.n[n] = b.readByte();
                    ++n;
                }
                a = null;
                b = null;
                d.u(n2);
                d.R();
            }
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void ao() {
        try {
            a = new ByteArrayOutputStream();
            a = new DataOutputStream(a);
            a.writeInt(aQ);
            a.writeInt(aI);
            a.writeInt(cl);
            a.writeInt(aJ);
            a.writeInt(aK);
            a.writeInt(aL);
            a.writeInt(aN);
            a.writeInt(aO);
            a.writeInt(aM);
            int n = 0;
            while (n < x.length) {
                a.writeInt(x[n]);
                ++n;
            }
            a.writeInt(aS);
            a.writeInt(aT);
            a.writeByte(b);
            n = 0;
            while (n < d.n.length) {
                a.writeByte(d.n[n]);
                ++n;
            }
            byte[] byArray = a.toByteArray();
            d.a("s", byArray);
            a = null;
            a = null;
            d.aB();
            d.d(true);
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void u(int n) {
        aQ = n;
        aR = 0;
        if (n == 2 || n == 4) {
            d.a(12, 128, 1);
            return;
        }
        if (n == 3) {
            d.bT();
            d.b(true);
            d.o(11);
            d.h(1 + ca++ % 3, 0);
            H = true;
        }
    }

    private static void ap() {
        switch (aQ) {
            case 0: {
                d.u(1);
                d.ao();
                return;
            }
            case 1: {
                int n;
                int n2 = 0;
                while (n2 < aN + aO) {
                    int n3;
                    n = d.l(n2, 3);
                    d.x[n2 * 6 + d.aM] = n3 = d.m(aM, n);
                    int n4 = n2 * 6 + 5;
                    x[n4] = x[n4] + n3;
                    ++n2;
                }
                d.aq();
                if (aK == 1) {
                    d.am();
                    d.cl();
                } else if (aI == 2 && b < 4 || aI != 2 && aL >= 1 << aM + 1) {
                    d.u(2);
                    if (aI == 2) {
                        d.ap();
                    } else {
                        d.ao();
                    }
                } else {
                    if (aI == 0 && y[0] == 0) {
                        if (aJ == 0) {
                            d.j[23] = 1;
                            d.j[11] = 1;
                            d.l();
                        } else {
                            d.j[24] = 1;
                            d.j[17] = 1;
                            d.l();
                            d.ce();
                        }
                        d.ar();
                    } else if (aI == 2) {
                        ++aS;
                        d.aj();
                        d.at();
                        d.al();
                        d.ak();
                        d.as();
                        d.au();
                        return;
                    }
                    n2 = 0;
                    while (n2 < aN) {
                        n = x[n2 * 6 + 5];
                        d.a(5, d.a[n2].a, n, 0x10000 | d.a[n2].a << 8 | d.a[n2].b, 0, null);
                        ++n2;
                    }
                    d.u(3);
                    d.ao();
                }
                return;
            }
            case 2: {
                d.at();
                if (aI == 2) {
                    b = (byte)(b + 1);
                    aM = n[b];
                } else {
                    ++aM;
                    while ((aL & 1 << aM) == 0) {
                        ++aM;
                    }
                }
                d.as();
                d.au();
                d.u(1);
                d.ao();
                return;
            }
            case 3: {
                d.u(4);
                return;
            }
            case 4: {
                d.am();
                d.cl();
            }
        }
    }

    private static void aq() {
        int n = 0;
        while (n < 6) {
            d.y[n] = n;
            ++n;
        }
        n = aN + aO;
        int n2 = 1;
        while (n2 < n) {
            int n3 = 0;
            while (n3 < n2) {
                int n4 = x[y[n2] * 6 + 5];
                int n5 = x[y[n3] * 6 + 5];
                if (n4 > n5) {
                    int n6 = y[n2];
                    int n7 = n2;
                    while (n7 > n3) {
                        d.y[n7] = y[n7 - 1];
                        --n7;
                    }
                    d.y[n3] = n6;
                    break;
                }
                ++n3;
            }
            ++n2;
        }
    }

    private static void a(int n, String string) {
        d.a(n, string, true, true, true);
    }

    private static void a(int n, String string, boolean bl, boolean bl2, boolean bl3) {
        int n2 = 65536;
        if (bb < 400) {
            n2 = (bb << 16) / 400;
        } else if (G) {
            n2 = (char)-1 - (bb - bm << 16) / 400;
        }
        d.a(n, string, n2, bl, bl2, bl3);
    }

    private static void a(int n, String string, int n2, boolean bl, boolean bl2, boolean bl3) {
        d.a(n, string, n2, bl, bl2, bl3, true);
    }

    private static void a(int n, String string, int n2, boolean bl, boolean bl2, boolean bl3, boolean bl4) {
        int n3 = 0;
        int n4 = 0;
        int n5 = 0;
        int n6 = 0;
        if (n2 < 65536) {
            int n7 = (char)-1 - n2 >> 8;
            int n8 = d.c(0, 240, n7, 256);
            if (bl) {
                n3 = n8;
            }
            if (bl2) {
                n4 = n8;
            }
            if (bl3) {
                n5 = n8;
                n6 = d.c(0, 39, n7, 256);
            }
        }
        b.setColor(0xFFFFFF);
        b.fillRect(0, -n6, 240, 39);
        b.setColor(16715817);
        b.fillRect(-n5, 35, 240, 3);
        if (n >= 0) {
            if (bl4) {
                d.a(1, 3 - n4, 3, 20, 0);
            }
            d.a(n, 36 - n4, 28, 3, 0);
        }
        if (string != null) {
            d.a(8196, string, 224 + n3, 3, 24);
        }
    }

    private static void k(int n, int n2) {
        b.setColor(0xFFFFFF);
        b.fillRect(n + 8, n2, 212, 8);
        b.fillRect(n, n2 + 8, 228, 54);
        b.fillRect(n + 8, n2 + 70 - 8, 212, 8);
        b.setColor(0);
        b.drawLine(n + 8, n2, n + 228 - 8 - 1, n2);
        b.drawLine(n + 228 - 1, n2 + 8, n + 228 - 1, n2 + 70 - 8 - 1);
        b.drawLine(n, n2 + 8, n, n2 + 70 - 8 - 1);
        b.drawLine(n + 8, n2 + 70 - 1, n + 228 - 8 - 1, n2 + 70 - 1);
        b.drawLine(n + 1, n2 + 8, n + 1, n2 + 70 - 8 - 1);
        b.drawLine(n + 8, n2 + 70 - 2, n + 228 - 8 - 1, n2 + 70 - 2);
        d.a(37, n, n2, 20, 0);
        d.a(38, n + 228, n2, 24, 0);
        d.a(39, n + 228, n2 + 70, 40, 0);
        d.a(40, n, n2 + 70, 36, 0);
    }

    private static void ar() {
        if (j[25] != 0) {
            return;
        }
        boolean bl = true;
        int n = 0;
        while (bl && n < 12) {
            if (j[6 + n] == 0) {
                bl = false;
            }
            ++n;
        }
        if (bl) {
            d.j[25] = 1;
            d.cf();
            d.l();
        }
    }

    private static int j(int n, int n2) {
        switch (n) {
            case 0: {
                return d.p(n2);
            }
            case 2: {
                return d.s(n2);
            }
            case 1: {
                return d.v(n2);
            }
            case 4: {
                return d.y(n2);
            }
            case 3: {
                return d.B(n2);
            }
            case 5: {
                return 5000 - n2 * 500;
            }
            case 6: {
                return 20000 - n2 * 2000;
            }
        }
        return 0;
    }

    private static String b(int n, int n2) {
        switch (n) {
            case 0: 
            case 1: 
            case 2: 
            case 3: 
            case 4: {
                if (n2 < 5) {
                    return d.a[7 + (n2 + n) % 5].a;
                }
                if (n2 >= 5) {
                    return d.a[2 + (n2 + n) % 5].a;
                }
            }
            case 5: {
                return d.a[11 - n2].a;
            }
            case 6: {
                return a[91 - n2];
            }
        }
        return null;
    }

    private static int k(int n, int n2) {
        int n3 = 0;
        int n4 = 0;
        int n5 = 0;
        switch (n) {
            case 0: 
            case 1: 
            case 2: 
            case 3: 
            case 4: {
                if (n2 < 5) {
                    n3 = d.a[7 + (n2 + n) % 5].a;
                    n4 = d.a[7 + (n2 + n) % 5].b;
                }
                if (n2 < 5) break;
                n3 = d.a[2 + (n2 + n) % 5].a;
                n4 = d.a[2 + (n2 + n) % 5].b;
                break;
            }
            case 5: {
                n3 = d.a[11 - n2].a;
                n4 = d.a[11 - n2].b;
                break;
            }
            case 6: {
                n3 = d.a[11].a;
                n4 = d.a[11].b;
                n5 = 1 + (10 - n2) / 3;
            }
        }
        return n5 << 17 | n3 << 8 | n4;
    }

    private static void v(int n) {
        aY = n + 62;
    }

    private static void a(int n, c c2, long l, int n2) {
        d.a(n, c2, l, n2, 1);
    }

    private static void a(int n, c c2, long l, int n2, int n3) {
        c2.a = n;
        c2.b = -1;
        c2.c = n2;
        c2.j = 0;
        c2.k = 0;
        c2.i = 0;
        c2.a = (l ^ 0x5DEECE66DL) & 0xFFFFFFFFFFFFL;
        c2.g = n3;
        c2.e = (int)(l >> 4 & 1L);
        c2.d = (int)(l >> 6 & 1L);
        c2.b[0] = (int)(l >> 4 & 1L);
        c2.b[1] = (int)(l >> 6 & 1L);
        c2.b[2] = (int)(l >> 7 & 1L);
        c2.l = 0;
        int n4 = 0;
        while (n4 < 10) {
            c2.a[n4] = 22;
            ++n4;
        }
        c2.f = -1;
        c2.h = 0;
        aX = -1;
        c2.o = 0;
        c2.p = 0;
        c2.a = false;
        c2.q = 0;
        c2.m = 0;
        c2.n = 0;
        c2.F = -1;
        c2.c = false;
        c2.G = 0;
    }

    private static void a(c c2) {
        int n;
        if (c2.g == 1) {
            if (c2.d == c2.e) {
                c2.d = 1 - c2.d;
            } else {
                c2.e = c2.d;
                c2.a = c2.a * 25214903917L + 11L & 0xFFFFFFFFFFFFL;
                c2.d = (int)(c2.a >> 5 & 1L);
            }
            return;
        }
        c2.a = c2.a * 25214903917L + 11L & 0xFFFFFFFFFFFFL;
        c2.b[0] = (int)(c2.a >> 4 & 1L);
        c2.b[1] = (int)(c2.a >> 6 & 1L);
        c2.b[2] = (int)(c2.a >> 7 & 1L);
        c2.l = 0;
        int n2 = 50 * c2.m / 3;
        int n3 = 0;
        while (n2 > 0 && n3 < 10) {
            n = 22 - c2.a[9 - n3];
            if (n > n2) {
                n = n2;
            }
            n2 -= n;
            int n4 = 9 - n3;
            c2.a[n4] = c2.a[n4] + n;
            ++n3;
        }
        n2 = 0;
        n = 0;
        while (n < 10) {
            int n5 = c2.a[n] - 0;
            if (n5 > 0) {
                n2 += n5;
            }
            ++n;
        }
        c2.b = 24000 - 8000 * (n2 - 80) / 140;
        c2.n = c2.m;
        c2.m = 0;
        aX = -1;
    }

    private static void a(c c2, int n, boolean bl) {
        if (c2.b < 0) {
            c2.b = 0;
        }
        if (c2.q - c2.h <= 4) {
            ++c2.G;
            if (c2.G >= 12) {
                c2.G = 12;
            }
        } else {
            --c2.G;
        }
        boolean bl2 = false;
        if (c2.G >= 6) {
            bl2 = true;
        }
        c2.c = bl2;
        c2.h = c2.q;
        if (c2.f < 0) {
            if (n == 10) {
                c2.f = c2.c;
                if (bl) {
                    aX = ch << 3;
                    d.P();
                    if (ba != 3) {
                        d.a(20, 128, 1);
                    }
                }
                return;
            }
            if (c2.g == 1) {
                if (n == c2.d) {
                    if (bl) {
                        aX = n | 4 | ch << 3;
                        d.v(c2.q);
                    }
                    d.a(c2);
                    int n2 = 9;
                    while (n2 > 0) {
                        c2.a[n2] = c2.a[n2 - 1];
                        --n2;
                    }
                    c2.a[0] = 0;
                } else {
                    int n3 = 50;
                    int n4 = 0;
                    while (n3 > 0 && n4 < 10) {
                        int n5 = 22 - c2.a[9 - n4];
                        if (n5 > n3) {
                            n5 = n3;
                        }
                        n3 -= n5;
                        int n6 = 9 - n4;
                        c2.a[n6] = c2.a[n6] + n5;
                        ++n4;
                    }
                    if (bl) {
                        aX = n | ch << 3;
                        if (ba != 3) {
                            d.a(19, 128, 1);
                        }
                    }
                }
                return;
            }
            if (c2.l < 3) {
                if (n == c2.b[c2.l]) {
                    if (bl) {
                        aX = n | 4 | ch << 3;
                        d.v(c2.q);
                    }
                    int n7 = 9;
                    while (n7 > 0) {
                        c2.a[n7] = c2.a[n7 - 1];
                        --n7;
                    }
                    c2.a[0] = 0;
                } else {
                    ++c2.m;
                    if (bl) {
                        aX = n | ch << 3;
                    }
                }
                ++c2.l;
            }
        }
    }

    private static void b(c c2) {
        d.c(c2);
    }

    private static void c(c c2) {
        if (c2.c && c2.q - c2.h > (c2.G - 6 + 1) * 4) {
            c2.G = 0;
            c2.c = false;
        }
        if (c2.b >= 0) {
            int n;
            int n2 = c2.a >= 0 && c2.a < aN && c2.c ? 30 : 10;
            int n3 = 0;
            while (n3 < n2) {
                if (c2.a[0] < 22) {
                    c2.a[0] = c2.a[0] + 1;
                    n3 = 10;
                } else {
                    int n4 = c2.a[9];
                    n = 9;
                    while (n > 0) {
                        c2.a[n] = c2.a[n - 1];
                        --n;
                    }
                    c2.a[0] = n4;
                }
                ++n3;
            }
            n3 = 0;
            n = 0;
            while (n < 10) {
                int n5 = c2.a[n] - 0;
                if (n5 > 0) {
                    n3 += n5;
                }
                ++n;
            }
            n = c2.g == 1 ? 8 : 4;
            c2.b = 24000 - 8000 * (n3 - 10 * n) / (10 * (22 - n));
            d.d(c2);
        }
    }

    private static void d(c c2) {
        d.a(c2, 0);
    }

    private static void a(c c2, int n) {
        if (c2.b >= 0) {
            if (c2.f >= 0) {
                c2.c = c2.c - c2.f < 409600 ? (c2.c += 3900) : (c2.c -= 3900);
            }
            c2.c += c2.b;
            c2.j -= n;
            c2.k += c2.j;
        }
    }

    private static void e(c c2) {
        int n = 0;
        if (c2.f >= 0) {
            n = c2.c - c2.f < 409600 ? 3900 : -3900;
        }
        if (c2.b >= 0) {
            c2.b -= 630;
            if (c2.b < 4000) {
                c2.b = -1;
                return;
            }
            c2.c += c2.b + n;
        }
    }

    private static void b(c c2, int n) {
        int n2;
        int n3;
        if (c2.b < 0) {
            return;
        }
        int n4 = 0;
        if (c2.f >= 0) {
            n4 = c2.c - c2.f < 409600 ? 3900 : -3900;
        }
        if (n > c2.c + (n3 = (n2 = (c2.b - 4000 + n4) / 630) * (c2.b + n4) - n2 * n2 * 630 / 2)) {
            c2.c += c2.b + n4;
        } else {
            d.e(c2);
        }
        if (c2.c > n) {
            c2.c = n;
            c2.b = -1;
        }
    }

    private static void a(c c2, int n, int n2) {
        n2 -= 38;
        int n3 = aX & 3;
        boolean bl = false;
        if ((aX & 4) != 0) {
            bl = true;
        }
        boolean bl2 = bl;
        int n4 = aX >> 3;
        int n5 = ch - n4;
        int n6 = 0;
        if (aX >= 0) {
            if (bl2) {
                if (n5 < 100) {
                    n6 = 70 * (100 - n5) / 100;
                }
                if (n5 < 200) {
                    if (c2.g == 1) {
                        d.i(n5, n3 == 0 ? n - 56 - 9 : n + 56 - 9, n2);
                    } else {
                        d.i(n5, n + 56 - 9 + (c2.l - 1) * 34, n2);
                    }
                }
            } else if (n5 < 300) {
                if (c2.g == 1) {
                    d.a(477, n3 == 0 ? n - 56 - 9 : n + 56 - 9, n2, 3, 0);
                } else {
                    d.a(477, n + 56 - 9 + (c2.l - 1) * 34, n2, 3, 0);
                }
            }
        }
        if (c2.g == 1) {
            if (c2.q < aY || e) {
                if (c2.d == 0) {
                    d.a(470, n - 56 - 9, n2 + n6, 3, 0);
                } else {
                    d.a(471, n + 56 - 9, n2 + n6, 3, 0);
                }
            }
            return;
        }
        if (c2.b >= 0 || c2.q < aY || e) {
            int n7 = c2.l;
            while (n7 < 3) {
                int n8 = 0;
                if (c2.b[n7] == 0) {
                    n8 = 1;
                }
                d.a(471 - n8, n + 56 - 9 + n7 * 34, n2, 3, 0);
                ++n7;
            }
        }
    }

    private static void i(int n, int n2, int n3) {
        if (n < 0 || n >= 200) {
            return;
        }
        d.a(474 + n * 3 / 200, n2, n3, 3, 0);
    }

    private static void l(int n, int n2) {
        d.b(n, n2, false);
    }

    private static void b(int n, int n2, boolean bl) {
        if (bl && ch % 200 < 100) {
            d.a(473, n, n2, 3, 0);
            return;
        }
        d.a(472, n, n2, 3, 0);
    }

    private static void w(int n) {
        bp = n;
        bq = 0;
    }

    private static void as() {
        z = new int[24];
        A = new int[6];
        B = new int[6];
        C = new int[12];
        D = d.a(0, 82, 180, null);
        d.b(aM);
        switch (aM) {
            case 0: {
                d.aK();
                return;
            }
            case 2: {
                d.aU();
                return;
            }
            case 1: {
                d.bd();
                return;
            }
            case 4: {
                d.bq();
                return;
            }
            case 3: {
                d.bE();
            }
        }
    }

    private static void at() {
        z = null;
        A = null;
        B = null;
        C = null;
        D = null;
        switch (aM) {
            case 0: {
                d.aL();
                return;
            }
            case 2: {
                d.aV();
                return;
            }
            case 1: {
                d.be();
                return;
            }
            case 4: {
                d.br();
                return;
            }
            case 3: {
                d.bF();
            }
        }
    }

    private static void au() {
        G = false;
        int n = 0;
        while (n < z.length) {
            d.z[n] = -1;
            ++n;
        }
        bh = -1;
        bg = -1;
        bi = d.a() % 974431;
        bo = 0;
        switch (aM) {
            case 0: {
                d.aM();
                break;
            }
            case 2: {
                d.aW();
                break;
            }
            case 1: {
                d.bf();
                break;
            }
            case 4: {
                d.bs();
                break;
            }
            case 3: {
                d.bG();
            }
        }
        d.av();
        d.z(1);
        bj = 0;
        bk = -1;
        bl = -200;
        aP = aN + aO;
        bn = 0;
    }

    private static void av() {
        d.b(aM);
        be = i[0];
        if ((bc & 1) != 0) {
            be = -be;
        }
        f = (i[20] & 0x10000) != 0 ? b[0] : null;
        bf = be;
    }

    private static void aw() {
        E = false;
        F = false;
        br = d.g(3, aP - 1);
        int n = 0;
        while (n < br) {
            int n2;
            boolean bl = false;
            do {
                if ((n2 = d.a() % aP) == aZ) continue;
                boolean bl2 = false;
                int n3 = 0;
                while (n3 < n && !bl2) {
                    if (n2 == C[n3 * 4]) {
                        bl2 = true;
                    }
                    ++n3;
                }
                if (bl2) continue;
                d.C[n * 4] = n2;
                bl = true;
            } while (!bl);
            n2 = 0;
            while (n2 < 3) {
                d.C[n * 4 + n2 + 1] = D[aM * 3 * 3 + n * 3 + n2];
                ++n2;
            }
            ++n;
        }
        bh = bg;
        bf = be;
        switch (aM) {
            case 0: {
                d.aO();
                break;
            }
            case 2: {
                d.aY();
                break;
            }
            case 1: {
                d.bg();
                break;
            }
            case 4: {
                d.bt();
                break;
            }
            case 3: {
                d.bH();
            }
        }
        d.ae();
    }

    private static void ax() {
        if (G && bb >= bm + 400) {
            d.aG();
        } else {
            bb += 32;
        }
        switch (aM) {
            case 0: {
                d.aP();
                break;
            }
            case 2: {
                d.aZ();
                break;
            }
            case 1: {
                d.bj();
                break;
            }
            case 4: {
                d.bw();
                break;
            }
            case 3: {
                d.bK();
            }
        }
        if (ba == 3) {
            bq += 32;
            if (bp == 0) {
                if (bq > 1600) {
                    d.w(1);
                }
            } else if (bp == 1) {
                if (bq > 750) {
                    d.w(8);
                }
            } else if ((bp & 8) != 0) {
                if (bq > 2400) {
                    d.w(bp & 7 | 0x20);
                    switch (aM) {
                        case 0: {
                            d.F(bp & 7);
                            break;
                        }
                        case 2: {
                            d.K(bp & 7);
                            break;
                        }
                        case 1: {
                            d.N(bp & 7);
                            break;
                        }
                        case 4: {
                            d.R(bp & 7);
                            break;
                        }
                        case 3: {
                            d.U(bp & 7);
                        }
                    }
                }
            } else if ((bp & 0x20) != 0) {
                switch (aM) {
                    case 0: {
                        d.aT();
                        break;
                    }
                    case 2: {
                        d.bc();
                        break;
                    }
                    case 1: {
                        d.bp();
                        break;
                    }
                    case 4: {
                        d.bD();
                        break;
                    }
                    case 3: {
                        d.bQ();
                    }
                }
            }
        }
        if (ba == 13) {
            d.d(d.a() % 245760, Short.MIN_VALUE, 5, d.a() % 3, 0);
        }
    }

    private static void ay() {
        if (ba != 1 && ba != 13) {
            switch (aM) {
                case 0: {
                    d.aQ();
                    break;
                }
                case 2: {
                    d.ba();
                    break;
                }
                case 1: {
                    d.bk();
                    break;
                }
                case 4: {
                    d.bx();
                    break;
                }
                case 3: {
                    d.bL();
                }
            }
            d.ag();
        }
        switch (ba) {
            case 1: {
                b.setColor(0xFFFFFF);
                b.fillRect(0, 0, 240, 320);
                int n = 69;
                if (aI != 2) {
                    int n2 = 0;
                    if (aJ == 0) {
                        n2 = 1;
                    }
                    d.a(22 - n2, 3, 53, 20, 0);
                }
                d.a(3, a[163], 218, 69, 24);
                n += 30;
                d.a(6, d.a(be, bc, true), 218, 99, 24);
                if (L && f != null) {
                    n += 30;
                    d.a(3, d.a(a[169], f, 1), 218, 129, 24);
                }
                if (j[18 + aM] != 0) {
                    boolean bl = false;
                    d.a(true, 60, (n += 38) + bj * 28, false);
                    int n3 = bb - d.bl;
                    if (n3 < 0) {
                        d.bl = bb - 200;
                        n3 = 200;
                    }
                    int n4 = 0;
                    int n5 = n + 3;
                    while (n4 < 2) {
                        int n6 = 60;
                        if (n3 < 200) {
                            if (n4 == bj) {
                                n6 = 60 - d.c(12, 0, 200 - n3, 200);
                            } else if (n4 == bk) {
                                n6 = 60 - d.c(0, 12, 200 - n3, 200);
                            }
                        } else if (n4 == bj) {
                            n6 -= 12;
                        }
                        int n7 = n4 == 0 ? 136 : 36;
                        d.a(8195, a[n7], n6, n5, 20);
                        ++n4;
                        n5 += 28;
                    }
                    n += 56;
                }
                d.a(25 + aM, a[93 + aM]);
                if (K && n < 319 - d.e(0) - 68) {
                    d.r(120, 319 - d.e(0) - 46);
                }
                d.i(23, 24);
                return;
            }
            case 3: {
                int n;
                if (bb >= 400 && G) {
                    d.r((char)-1 - (bb - bm << 16) / 400);
                }
                if (ba == 3 && bp == 0) {
                    b.setColor(0xFFFFFF);
                    b.fillRect(0, 0, 240, 320);
                    d.a(8196, a[36], 120, 160, 33);
                } else if (ba == 3 && ((bp & 8) != 0 || (bp & 0x20) != 0 && bq < 400)) {
                    n = 0;
                    if ((bp & 8) != 0) {
                        if (bq < 400) {
                            n = d.c(0, 240, 400 - bq, 400);
                        }
                    } else {
                        n = d.c(0, -240, bq, 400);
                    }
                    int n8 = bp & 7;
                    int n9 = 0;
                    switch (aM) {
                        case 0: {
                            n9 = d.q(n8);
                            break;
                        }
                        case 2: {
                            n9 = d.t(n8);
                            break;
                        }
                        case 1: {
                            n9 = d.w(n8);
                            break;
                        }
                        case 4: {
                            n9 = d.z(n8);
                            break;
                        }
                        case 3: {
                            n9 = d.C(n8);
                        }
                    }
                    switch (aM) {
                        case 0: {
                            d.m(n8, n, n9);
                            break;
                        }
                        case 2: {
                            d.n(n8, n, n9);
                            break;
                        }
                        case 1: {
                            d.o(n8, n, n9);
                            break;
                        }
                        case 4: {
                            d.p(n8, n, n9);
                            break;
                        }
                        case 3: {
                            d.r(n8, n, n9);
                        }
                    }
                }
                if (bp == 0) {
                    n = 65536;
                    if (bq < 400) {
                        n = (bq << 16) / 400;
                    } else if (bq > 1200) {
                        n = (1600 - bq << 16) / 400;
                    }
                    d.a(25 + aM, a[93 + aM], n, true, true, true);
                } else {
                    n = 65536;
                    if (bp == 1 && bq < 400) {
                        n = (bq << 16) / 400;
                    } else if (G) {
                        n = 65536 - (bb - bm << 16) / 400;
                    }
                    d.a(25 + aM, a[36], n, true, true, true);
                }
                return;
            }
            case 10: {
                if (bb < 400) {
                    d.r((bb << 16) / 400);
                } else if (!G && e) {
                    d.l(120, 131);
                    d.a(3, a[31], 120, 114, 33);
                }
                d.a(25 + aM, (bc & 1) != 0 ? a[48] : d.a(a[49], "" + (bd + 1), 1));
                d.a(a[aZ], true, 47, null, 0, 0);
                return;
            }
            case 11: {
                d.az();
                return;
            }
            case 12: {
                if (G && (E || (bc & 2) != 0 && aO > 0 || aZ < aN - 1)) {
                    d.r((char)-1 - (bb - bm << 16) / 400);
                }
                d.a(25 + aM, (bc & 1) != 0 ? a[50] : d.a(a[49], "" + (bd + 1), 1));
                d.a(a[aZ], true, 47, null, 0, 0);
                d.i(25, 24);
                d.aJ();
                return;
            }
            case 13: {
                int n = bb * 3 / 2;
                d.k(n, 128, -768);
                if (aI == 2) {
                    d.bR();
                    d.bS();
                } else {
                    d.bN();
                }
                d.l(n, 0, -1280);
                int n10 = n / 320 & 3;
                d.a(518 + n10, bA - 9, bB, 3, 0);
                int n11 = 0;
                if (d.a[d.aZ].c == 0) {
                    n11 = 1;
                }
                int n12 = 1 - n11;
                d.a(n12, d.a[d.aZ].a, d.a[d.aZ].d, ad, n, 160, bA, bB, true);
                d.ag();
                d.aJ();
                if (G) {
                    d.r((char)-1 - (bb - bm << 16) / 400);
                } else if (bb < 400) {
                    d.r((bb << 16) / 400);
                }
                d.a(25 + aM, a[93 + aM]);
                d.i(25, 24);
                return;
            }
            case 15: {
                if (e) {
                    d.a(8195, a[33], 120, 69, 17);
                }
                d.a(25 + aM, (bc & 1) != 0 ? a[50] : d.a(a[49], "" + (bd + 1), 1));
                d.i(26, 24);
                return;
            }
            case 16: 
            case 20: 
            case 21: 
            case 22: {
                int n;
                if (ba == 16 && G) {
                    d.r((char)-1 - (bb - bm << 16) / 400);
                }
                d.i(25, 24);
                if (ba == 22) {
                    d.a(513, a[128], true, true, false);
                    int n13 = 120;
                    if (bb < 400) {
                        n13 = 120 + d.c(0, 240, 256 - (bb << 8) / 400, 256);
                    } else if (G) {
                        n13 = 120 + d.c(0, -240, (bb - bm << 8) / 400, 256);
                    }
                    int n14 = 0;
                    boolean bl = false;
                    int n15 = bb - 500;
                    if (n15 < 0) {
                        n15 = 0;
                    } else if (n15 > 1500) {
                        n15 = 1500;
                    }
                    int n16 = aU + (aT - aU) * n15 / 1500;
                    String string = "" + n16;
                    n14 = d.a(6, string);
                    int n17 = d.a(4, a[56]) + d.a(4, "M") / 2;
                    if (n14 < n17) {
                        n14 = n17;
                    }
                    if (n14 < (n17 = d.a(3, a[57]))) {
                        n14 = n17;
                    }
                    if (n14 < (n17 = d.a(3, a[51]))) {
                        n14 = n17;
                    }
                    d.e(n13 - (n14 += 12) / 2, 90, n14, 141);
                    boolean bl2 = false;
                    d.a(4, a[56], n13, 93, 17);
                    d.a(3, a[57], n13, 125, 17);
                    b.setColor(16715817);
                    b.fillRect(n13 - n14 / 2 + 6, 160, n14 - 12, 2);
                    d.a(3, a[51], n13, 171, 17);
                    d.a(6, string, n13, 196, 17);
                    return;
                }
                d.a(25 + aM, ba == 21 ? a[35] : (ba == 20 ? a[32] : a[34]));
                d.a(0, 48, 240, (319 - d.e(0) - 56) / 32 * 32);
                int n18 = 0;
                int n19 = 0;
                int n20 = 56 - bn * 32;
                while (n19 < aP) {
                    int n21;
                    n = A[n19];
                    int n22 = 0;
                    int n23 = n19 * 60;
                    if (bb < n23 + 400) {
                        n22 = bb < n23 ? 240 : d.c(0, 240, 256 - (bb - n23 << 8) / 400, 256);
                    } else if (G) {
                        n21 = bm + n19 * 30;
                        if (bb >= n21 + 200) {
                            n22 = 240;
                        } else if (bb >= n21) {
                            n22 = d.c(0, 240, (bb - n21 << 8) * 2 / 400, 256);
                        }
                    }
                    n21 = 8 + d.a(3, "6.");
                    int n24 = 0;
                    if (n < aN) {
                        n24 = 1;
                    }
                    int n25 = n24 + 107;
                    int n26 = 240;
                    while (n26 > 0) {
                        d.a(n25, n26 - n22, n20, 24, 0);
                        n26 -= 64;
                    }
                    d.a(3, "" + (n19 + 1) + '.', n21 - n22, n20 + 21 - 20, 24);
                    d.a(86 + d.a[n].b, n21 - n22 + 3, n20 + 21, 36, 0);
                    d.a(3, d.a[n].a, n21 - n22 + 3 + 30, n20 + 21 - 20, 20);
                    n26 = d.l(n, 3);
                    if (ba == 21) {
                        d.a(5, "" + d.m(aM, n26), 240 + n22 - 8, n20 + 21 - 16, 24);
                    } else if (n26 == -1) {
                        d.a(3, a[47], 240 + n22 - 8, n20 + 21 - 20, 24);
                    } else {
                        d.a(5, d.a(n26, bc & 0xC), 240 + n22 - 8, n20 + 21 - 16, 24);
                    }
                    b.setColor(0xFFFFFF);
                    b.drawLine(-n22, n20 + 24, 239 - n22, n20 + 24);
                    n18 += n20;
                    ++n19;
                    n20 += 32;
                }
                d.a(0, 0, 240, 320);
                if (aI == 2 && ba == 21 || aI == 0 && aK == 1 && ba == 20) {
                    n19 = 120;
                    n20 = 53 + d.a[d.A[0]].a;
                    n = aJ == 0 ? 2 + aM : 8 + aM;
                    if (bb < 400) {
                        n19 = 120 + d.c(0, 240, 256 - (bb << 8) / 400, 256);
                    } else if (G) {
                        n19 = 120 + d.c(0, -240, (bb - bm << 8) / 400, 256);
                    }
                    if (A[0] < aN) {
                        if (aI == 2) {
                            d.a(n19, n18, n20, a[52], a[51], -1);
                        } else if (K && cs == 0 && j[6 + (aJ == 0 ? 0 : 6) + cl] != 0) {
                            d.a(n19, n18, n20, a[58], a[59], n);
                        } else {
                            d.a(n19, n18, n20, a[58], null, -1);
                        }
                    } else if (aI == 2) {
                        d.a(n19, n18, 513, a[54], a[55], 0);
                    } else {
                        d.a(n19, n18, n20, a[60], a[61], -1);
                    }
                }
                boolean bl = false;
                if (bn > 0) {
                    bl = true;
                }
                boolean bl3 = false;
                if (bn < aP - (319 - d.e(0) - 56) / 32) {
                    bl3 = true;
                }
                d.c(bl, bl3);
                return;
            }
            case 30: {
                if (G) {
                    d.r((char)-1 - (bb - bm << 16) / 400);
                } else {
                    int n;
                    boolean bl = false;
                    boolean bl4 = false;
                    boolean bl5 = false;
                    d.a(0, 72, 240, 90);
                    int n27 = 72;
                    while (n27 < 162) {
                        n = 0;
                        while (n < 240) {
                            d.a(111, n, n27, 20, 0);
                            n += 24;
                        }
                        n27 += 24;
                    }
                    d.a(true, 60, 75 + bj * 28, false);
                    n27 = bb - d.bl;
                    n = 0;
                    int n28 = 78;
                    while (n < 3) {
                        int n29 = 60;
                        if (n27 < 200) {
                            if (n == bj) {
                                n29 = 60 - d.c(12, 0, 200 - n27, 200);
                            } else if (n == bk) {
                                n29 = 60 - d.c(0, 12, 200 - n27, 200);
                            }
                        } else if (n == bj) {
                            n29 -= 12;
                        }
                        int n30 = n == 0 ? 134 : (n == 1 ? 36 : 135);
                        d.a(8195, a[n30], n29, n28, 20);
                        ++n;
                        n28 += 28;
                    }
                    d.a(0, 0, 240, 320);
                }
                d.a(25 + aM, a[93 + aM]);
                d.i(5, 24);
            }
        }
    }

    private static void az() {
        if ((bc & 1) != 0) {
            d.a(a[aZ], false, 0, d.a(aV * 100 / 1024, bc), 5, 16);
            return;
        }
        d.a(a[aZ], false, 0, null, 0, 0);
    }

    private static void x(int n) {
        if (G) {
            return;
        }
        if (ba == 11) {
            switch (aM) {
                case 0: {
                    d.A(n);
                    break;
                }
                case 2: {
                    d.G(n);
                    break;
                }
                case 1: {
                    d.L(n);
                    break;
                }
                case 4: {
                    d.O(n);
                    break;
                }
                case 3: {
                    d.S(n);
                }
            }
            return;
        }
        if (ba == 30 || ba == 1) {
            boolean bl = d.a(n, 131074);
            boolean bl2 = d.a(n, 131072);
            boolean bl3 = d.a(n, 196616);
            if (ba == 1 && j[18 + aM] == 0) {
                bl2 = false;
                bl = false;
            }
            if (bl || bl2) {
                d.a(27, 128, 1);
                int n2 = 0;
                if (ba == 30) {
                    n2 = 1;
                }
                int n3 = n2 + 2;
                bk = bj--;
                if (bl2) {
                    if (bj < 0) {
                        bj = n3 - 1;
                    }
                } else if (++bj >= n3) {
                    bj = 0;
                }
                d.bl = bb;
            } else if (bl3) {
                d.a(28, 128, 1);
                d.aF();
            }
            return;
        }
        if (ba != 3) {
            if (ba == 16 || ba == 20 || ba == 21) {
                boolean bl = d.a(n, 131074);
                boolean bl4 = d.a(n, 131072);
                boolean bl5 = d.a(n, 196616);
                if (bl || bl4) {
                    if (bl4) {
                        if (bn > 0) {
                            --bn;
                            d.a(27, 128, 1);
                        }
                    } else if (bn < aP - (319 - d.e(0) - 56) / 32) {
                        ++bn;
                        d.a(27, 128, 1);
                    }
                    d.bl = bb;
                } else if (bl5) {
                    d.a(28, 128, 1);
                    d.aF();
                }
                return;
            }
            if (d.a(n, 196616)) {
                if (ba == 10) {
                    d.a(18, 128, 1);
                }
                aU = aT;
                d.aF();
            }
            return;
        }
    }

    private static void y(int n) {
        if (ba == 11) {
            switch (aM) {
                case 0: {
                    return;
                }
                case 2: {
                    return;
                }
                case 1: {
                    return;
                }
                case 4: {
                    d.P(n);
                    return;
                }
                case 3: {
                    d.T(n);
                }
            }
        }
    }

    private static void aA() {
        try {
            byte[] byArray = d.a("e");
            if (byArray == null) {
                d.au();
            } else {
                G = false;
                switch (aM) {
                    case 0: {
                        d.aM();
                        break;
                    }
                    case 2: {
                        d.aW();
                        break;
                    }
                    case 1: {
                        d.bf();
                        break;
                    }
                    case 4: {
                        d.bs();
                        break;
                    }
                    case 3: {
                        d.bG();
                    }
                }
                a = new ByteArrayInputStream(byArray);
                b = new DataInputStream(a);
                int n = b.readInt();
                int n2 = 0;
                while (n2 < z.length) {
                    d.z[n2] = b.readInt();
                    ++n2;
                }
                bg = bh = b.readInt();
                bi = b.readInt();
                aZ = b.readInt();
                bd = b.readInt();
                bo = b.readInt();
                d.av();
                d.aD();
                d.z(n);
                if (ba == 11) {
                    bo |= 1 << aZ;
                    d.j(aZ, (bc & 1) != 0 ? 3 : bd, -1);
                }
                while (ba == 11 || ba == 12 || ba == 13) {
                    d.aG();
                }
                a = null;
                b = null;
            }
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void aB() {
        try {
            a = new ByteArrayOutputStream();
            a = new DataOutputStream(a);
            a.writeInt(ba);
            int n = 0;
            while (n < z.length) {
                a.writeInt(z[n]);
                ++n;
            }
            a.writeInt(bg);
            a.writeInt(bi);
            a.writeInt(aZ);
            a.writeInt(bd);
            a.writeInt(bo);
            byte[] byArray = a.toByteArray();
            d.a("e", byArray);
            a = null;
            a = null;
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void aC() {
        try {
            a = new ByteArrayOutputStream();
            a = new DataOutputStream(a);
            int n = 0;
            while (n < 6) {
                int n2 = 0;
                int n3 = n * 400;
                while (n2 < 400) {
                    int n4 = d.n[n3];
                    a.writeInt(n4);
                    if (n4 < 0) break;
                    ++n2;
                    ++n3;
                }
                ++n;
            }
            byte[] byArray = a.toByteArray();
            d.a("r", byArray);
            a = null;
            a = null;
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void aD() {
        int n = 0;
        while (n < 6) {
            d.p(n);
            ++n;
        }
        try {
            byte[] byArray = d.a("r");
            if (byArray != null) {
                a = new ByteArrayInputStream(byArray);
                b = new DataInputStream(a);
                int n2 = 0;
                while (n2 < 6) {
                    int n3 = 0;
                    int n4 = n2 * 400;
                    while (n3 < 400) {
                        int n5;
                        d.n[n4] = n5 = b.readInt();
                        if (n5 < 0) break;
                        ++n3;
                        ++n4;
                    }
                    ++n2;
                }
                n2 = 0;
                while (n2 < 6) {
                    d.q(n2);
                    ++n2;
                }
                a = null;
                b = null;
            }
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    private static void z(int n) {
        ba = n;
        bb = 0;
        switch (n) {
            case 1: {
                bj = 0;
                bk = -1;
                bl = -200;
                return;
            }
            case 20: {
                d.aH();
                int n2 = 0;
                while (n2 < aP) {
                    int n3 = A[n2];
                    switch (aM) {
                        case 1: {
                            d.c[n3].F = n2;
                            if (d.c[n3].o != 9) break;
                            d.c[n3].p = 0;
                            break;
                        }
                        case 3: {
                            d.e[n3].F = n2;
                            if (d.e[n3].o != 7 || d.e[n3].p <= 600) break;
                            d.e[n3].p = 600;
                            break;
                        }
                        case 4: {
                            d.d[n3].F = n2;
                        }
                    }
                    ++n2;
                }
                if (aI == 0 && aK == 1 && A[0] == 0) {
                    n2 = 6 + (aJ == 0 ? 0 : 6) + aM;
                    d.j[n2] = 1;
                    d.l();
                    d.ar();
                }
                if (aK == 1) {
                    if (A[0] < aN) {
                        d.o(9);
                    } else {
                        d.o(10);
                    }
                }
                if (aI != 2 && aK > 1) {
                    if (aL > (1 << aM + 1) - 1) {
                        int n4;
                        n2 = d.a() % 3;
                        if (n2 == 0) {
                            n4 = 6;
                        } else {
                            int n5 = 0;
                            if (n2 == 1) {
                                n5 = 1;
                            }
                            n4 = 5 - n5;
                        }
                        d.h(n4, 0);
                    }
                } else {
                    d.h(1 + ca++ % 3, 0);
                    H = true;
                }
                return;
            }
            case 16: {
                d.aH();
                return;
            }
            case 10: {
                d.aw();
                d.aB();
                d.b(true);
                d.a(13, 128, 1);
                return;
            }
            case 15: {
                d.a(B, bd);
                if ((bc & 2) != 0) {
                    if (aM == 1) {
                        d.bi();
                    } else if (aM == 4) {
                        d.bv();
                    } else if (aM == 3) {
                        d.bJ();
                    }
                }
                d.ae();
                return;
            }
            case 30: {
                bj = 0;
                bk = -1;
                bl = -200;
                return;
            }
            case 11: {
                d.aB();
                return;
            }
            case 12: {
                d.a(true);
            }
        }
    }

    private static void aE() {
        if ((bc & 2) != 0) {
            if (aM == 1) {
                d.bh();
                return;
            }
            if (aM == 4) {
                d.bu();
                return;
            }
            if (aM == 3) {
                d.bI();
            }
        }
    }

    private static void aF() {
        if (!G) {
            if (bb < 400) {
                d.aG();
                return;
            }
            G = true;
            bm = bb;
        }
    }

    private static void aG() {
        G = false;
        switch (ba) {
            case 1: {
                d.w(0);
                aZ = 0;
                switch (aM) {
                    case 0: {
                        d.aS();
                        break;
                    }
                    case 2: {
                        d.bb();
                        break;
                    }
                    case 1: {
                        d.bo();
                        break;
                    }
                    case 4: {
                        d.bC();
                        break;
                    }
                    case 3: {
                        d.bP();
                    }
                }
                if (j[18 + aM] == 0) {
                    d.j[18 + d.aM] = 1;
                    d.l();
                    d.z(3);
                    return;
                }
                if (bj == 1) {
                    d.z(3);
                    return;
                }
            }
            case 3: {
                int n = 0;
                while (n < 6) {
                    d.p(n);
                    ++n;
                }
                if ((bc & 1) != 0) {
                    if (aM == 0) {
                        d.aN();
                    } else if (aM == 2) {
                        d.aX();
                    }
                    d.aC();
                }
                aZ = 0;
                bd = 0;
                d.z(10);
                return;
            }
            case 10: {
                d.v(0);
                d.z(11);
                return;
            }
            case 11: {
                int n = d.l(aZ, (bc & 1) != 0 ? 3 : bd);
                if (n != -1) {
                    int n2 = d.d(aM, (bc & 1) != 0 ? -n : n);
                    boolean bl = false;
                    if (n2 >= 0 && n == d.l(aZ, 3)) {
                        bl = true;
                    }
                    F = bl;
                    boolean bl2 = false;
                    if ((bc & 1) != 0 && n < be || (bc & 2) != 0 && n > be) {
                        bl2 = true;
                    }
                    if (E = bl2) {
                        be = n;
                    }
                } else {
                    E = false;
                    F = false;
                }
                d.z(12);
                d.aB();
                d.aC();
                return;
            }
            case 12: {
                if (E) {
                    d.ae();
                    int n = 0;
                    while (n < 100) {
                        d.d(d.a() % 245760, d.a() % 327680, 5, d.a() % 3, 0);
                        ++n;
                    }
                    d.z(13);
                    return;
                }
            }
            case 13: {
                int n;
                if (ba == 13) {
                    d.ae();
                }
                if (((bc & 1) != 0 || bd == 2) && (n = d.l(aZ, 3)) != -1) {
                    if ((bc & 4) != 0) {
                        n = -n;
                    }
                    d.a(aM, d.a[d.aZ].a, n, 0x10000 | d.a[d.aZ].a << 8 | d.a[d.aZ].b, 0, null);
                }
                bh = bg;
                bf = be;
                if (aZ < aN - 1) {
                    ++aZ;
                    d.z(10);
                } else if ((bc & 1) != 0) {
                    d.z(20);
                } else {
                    d.aE();
                    d.z(15);
                    d.aB();
                    d.aC();
                }
                return;
            }
            case 15: {
                if (bd < 2) {
                    d.z(16);
                } else {
                    d.z(20);
                }
                return;
            }
            case 16: {
                int n = 0;
                while (n < 6) {
                    d.p(n);
                    ++n;
                }
                if (++bd < 3) {
                    aZ = 0;
                    bo = 0;
                    d.z(10);
                } else {
                    d.z(20);
                }
                return;
            }
            case 20: {
                if (aK > 1) {
                    if (aI == 2) {
                        int n = d.l(0, 3);
                        aU = aT;
                        aT += d.m(aM, n);
                    }
                    d.z(21);
                } else if (aI == 0 && A[0] < aN && cs == 0 && j[6 + (aJ == 0 ? 0 : 6) + cl] != 0) {
                    d.am();
                    d.cl();
                } else {
                    d.z(30);
                }
                return;
            }
            case 21: {
                if (aI == 2 && A[0] == 0 && b >= 4) {
                    aU = aT;
                    aT += 1000;
                    d.z(22);
                    return;
                }
            }
            case 22: {
                if (aI == 2) {
                    if (A[0] == 0) {
                        d.z(0);
                        d.ap();
                    } else {
                        d.a(6, d.a[0].a, aT, aS << 17 | 0x10000 | d.a[0].a << 8 | d.a[0].b, 0, null);
                        d.am();
                        d.cl();
                    }
                } else {
                    d.z(0);
                    d.ap();
                }
                return;
            }
            case 30: {
                if (bj == 0 || bj == 1) {
                    boolean bl = false;
                    if (bj == 1) {
                        bl = true;
                    }
                    boolean bl3 = bl;
                    d.al();
                    d.at();
                    d.ak();
                    d.as();
                    d.au();
                    d.ao();
                    if (bl3) {
                        d.w(0);
                        switch (aM) {
                            case 0: {
                                d.aS();
                                break;
                            }
                            case 2: {
                                d.bb();
                                break;
                            }
                            case 1: {
                                d.bo();
                                break;
                            }
                            case 4: {
                                d.bC();
                                break;
                            }
                            case 3: {
                                d.bP();
                            }
                        }
                        d.z(3);
                    } else {
                        d.aG();
                    }
                    return;
                }
                d.am();
                d.cl();
            }
        }
    }

    private static void a(e e2, boolean bl, int n, String string, int n2, int n3) {
        int n4 = 65536;
        if (bb < 400) {
            n4 = (bb << 16) / 400;
        } else if (G) {
            n4 = (char)-1 - (bb - bm << 16) / 400;
        }
        int n5 = 0;
        int n6 = 0;
        if (n4 < 65536) {
            n5 = d.c(0, 240, (char)-1 - n4 >> 8, 256);
            n6 = n4 > 32768 ? 0 : d.c(0, 240, Short.MAX_VALUE - n4 >> 7, 256);
        }
        int n7 = n + 8;
        int n8 = n7 + 20 + 1;
        int n9 = 240;
        while (n9 > 0) {
            d.a(107, n9 - n6, n7, 24, 0);
            n9 -= 64;
        }
        b.setColor(0xFFFFFF);
        b.drawLine(-n6, n7 + 24, 239 - n6, n7 + 24);
        n9 = 4;
        if (bl) {
            n9 = 200 - d.a(3, e2.a) >> 1;
        }
        d.a(65 + e2.b, n9 - n5, n8, 36, 0);
        d.a(3, e2.a, n9 + 40 - n5, n8 - 20, 20);
        if (string != null) {
            d.a(n2, n3, string, 236 + n5, n7 + 24 + 1, 8, false);
        }
    }

    private static void a(int n, int n2, String string, int n3, int n4, int n5, boolean bl) {
        int n6 = d.a(n, string);
        if (n5 == 1) {
            n3 -= n6 / 2;
        } else if (n5 == 8) {
            n3 -= n6 + 11;
        }
        int n7 = n4 - 16 - 6;
        d.a(bl ? 35 : 32, n3, n7, 24, 0);
        d.a(n3, n7, n6, 28);
        int n8 = n3;
        while (n8 < n3 + n6) {
            d.a(bl ? 34 : 31, n8, n7, 20, 0);
            n8 += 32;
        }
        d.a(0, 0, 240, 320);
        d.a(bl ? 36 : 33, n3 + n6, n7, 20, 0);
        d.a(n, string, n3, n4 - n2, 20);
    }

    private static void aH() {
        d.a(A, 3);
    }

    private static void a(int[] nArray, int n) {
        int n2 = 0;
        while (n2 < 6) {
            nArray[n2] = n2;
            ++n2;
        }
        int n3 = 0;
        if ((bc & 4) != 0) {
            n3 = 1;
        }
        n2 = n3;
        int n4 = 1;
        while (n4 < aP) {
            int n5 = 0;
            while (n5 < n4) {
                int n6 = d.l(nArray[n4], n);
                if (n6 == -1) break;
                int n7 = d.l(nArray[n5], n);
                if (n7 == -1 || n2 != 0 && n6 < n7 || n2 == 0 && n6 > n7) {
                    int n8 = nArray[n4];
                    int n9 = n4;
                    while (n9 > n5) {
                        nArray[n9] = nArray[n9 - 1];
                        --n9;
                    }
                    nArray[n5] = n8;
                    break;
                }
                ++n5;
            }
            ++n4;
        }
    }

    private static void j(int n, int n2, int n3) {
        boolean bl;
        d.z[n * 4 + n2] = n3;
        int n4 = d.l(n, 3);
        boolean bl2 = false;
        if ((bc & 4) != 0) {
            bl2 = bl = true;
        }
        if (n3 != -1) {
            if (n2 == 3) {
                if (bl && n3 < bg || !bl && n3 > bg) {
                    bg = n3;
                }
                return;
            }
            if (n4 == -1 || bl && n3 < n4 || !bl && n3 > n4) {
                d.j(n, 3, n3);
            }
        }
    }

    private static int l(int n, int n2) {
        return z[n * 4 + n2];
    }

    private static int m(int n, int n2) {
        if (n2 == -1) {
            return 0;
        }
        int n3 = 0;
        switch (n) {
            case 0: {
                n3 = d.o(n2);
                break;
            }
            case 2: {
                n3 = d.r(n2);
                break;
            }
            case 1: {
                n3 = d.u(n2);
                break;
            }
            case 4: {
                n3 = d.x(n2);
                break;
            }
            case 3: {
                n3 = d.A(n2);
                break;
            }
            default: {
                return 0;
            }
        }
        if (n3 < 0) {
            n3 = 0;
        }
        return n3;
    }

    private static int n(int n, int n2) {
        if (n2 == -1) {
            return 5;
        }
        if ((bc & 1) != 0 && n2 <= be || (bc & 2) != 0 && n2 >= be) {
            return 4;
        }
        if (aP > 3) {
            int n3 = 1;
            if (n != 0) {
                if (n < 3) {
                    return 3;
                }
                if (n == aP - 1) {
                    return 5;
                }
                n3 = 2;
            }
            return n3;
        }
        if (aP == 3) {
            int n4 = 1;
            if (n != 0) {
                if (n == 1) {
                    return 3;
                }
                n4 = 5;
            }
            return n4;
        }
        if (aP == 2) {
            if (n == 0) {
                return 1;
            }
            return 5;
        }
        return 1;
    }

    private static int n(int n) {
        if (n == -1) {
            return 5;
        }
        if ((bc & 1) != 0 && n <= be || (bc & 2) != 0 && n >= be) {
            return 4;
        }
        int n2 = d.m(aM, n);
        if (n2 >= E[1]) {
            return 1;
        }
        if (n2 >= E[3]) {
            return 3;
        }
        return 2;
    }

    private static int c(int n, int n2, int n3) {
        int n4 = d.m(n, n2);
        int n5 = -1;
        int n6 = 0;
        while (n6 < 5 && n5 < 0) {
            if (n4 >= E[n6]) {
                n5 = n6;
            }
            ++n6;
        }
        if (n5 < 0) {
            n5 = 5;
        }
        if (n3 == 0 && n5 > 2) {
            n5 = 2;
        } else if (n3 == 1 && n5 < 3) {
            n5 = 3;
        }
        return 154 + n5;
    }

    private static void aI() {
        if (ba == 3 || ba >= 15) {
            return;
        }
        int n = 0;
        while (n < br) {
            short[] sArray;
            int n2 = C[n * 4];
            int n3 = C[n * 4 + 1];
            int n4 = C[n * 4 + 2];
            int n5 = ch + 1251 * n;
            int n6 = 120;
            int n7 = C[n * 4 + 3];
            if (n7 == 0) {
                sArray = R;
            } else if (n7 == 1) {
                sArray = S;
            } else {
                sArray = T;
                n6 = 200;
            }
            d.l(n3, 0, n4);
            d.a(518, bA - 9, bB, 3, 0);
            int n8 = 0;
            if (d.a[n2].c == 0) {
                n8 = 1;
            }
            int n9 = 1 - n8;
            d.a(n9, d.a[n2].a, d.a[n2].d, sArray, n5, n6, bA, bB, true);
            ++n;
        }
    }

    private static void aJ() {
        int n;
        int n2 = d.l(aZ, (bc & 1) != 0 ? 3 : bd);
        int n3 = 6;
        int n4 = 25;
        String string = d.a(n2, bc & 0xC, true);
        if (n2 == -1) {
            n3 = 3;
            n4 = 20;
        }
        int n5 = 120;
        if (bb < 400) {
            n5 = 120 + d.c(0, 240, 256 - (bb << 8) / 400, 256);
        } else if (G) {
            n5 = 120 + d.c(0, 240, (bb - bm << 8) / 400, 256);
        }
        int n6 = 80;
        if (ba != 13) {
            n6 += 32;
        }
        d.a(n3, n4, string, n5, n6, 1, false);
        n6 += 10;
        if (ba != 13 && bb > 900 && !G) {
            n = -1;
            if (aN == 1 && aO == 1) {
                if (aM == 0) {
                    n = d.a[0].F;
                } else if (aM == 2) {
                    n = d.b[0].F;
                }
            }
            d.a(3, a[d.c(aM, n2, n)], 120, n6, 17);
        }
        int n7 = n = ba == 13 ? 400 : 1200;
        if (bb > n && (F || E)) {
            int n8 = 65536;
            if (G) {
                n8 = (char)-1 - (bb - bm << 16) / 400;
            } else if (bb < n + 400) {
                n8 = (bb - n << 16) / 400;
            }
            String string2 = E ? a[164] : a[162];
            int n9 = d.a(8196, string2) + 24;
            int n10 = d.c(0, 240, 65536 - n8 >> 8, 256);
            if (G) {
                n10 = -n10;
            }
            d.e(120 + n10 - (n9 >> 1), n6, n9, 37);
            d.a(8196, string2, 120 + n10, n6 + 6 + 25, 33);
        }
    }

    private static int o(int n) {
        if (n < 1120) {
            return d.a(600, 1000, 1120 - n, 146);
        }
        return d.a(600, 0, n - 1120, 380);
    }

    private static int p(int n) {
        return -(974 + n * 6 + n * n * 2);
    }

    private static void aK() {
        a = new c[6];
        int n = 0;
        while (n < 6) {
            d.a[n] = new c();
            d.a[n].a = new int[10];
            d.a[n].b = new int[3];
            ++n;
        }
        d.M();
    }

    private static void aL() {
        int n = 0;
        while (n < 6) {
            d.a[n].a = null;
            d.a[n].b = null;
            d.a[n] = null;
            ++n;
        }
        a = null;
        d.N();
    }

    private static void aM() {
        bc = 5;
        int n = 0;
        while (n < 6) {
            d.p(n);
            ++n;
        }
    }

    private static void aN() {
        int n = 0;
        while (n < aO) {
            int n2 = d.a(a[n + aN], n + aN, d.a[d.aN + n].b);
            d.j(aN + n, 3, n2 * 100 / 1024);
            ++n;
        }
        n = 0;
        while (n < 6) {
            d.a(n, a[n], (long)(bi * (n + 1)), 0);
            ++n;
        }
    }

    private static int a(c c2, int n, int[] nArray) {
        int n2;
        int n3;
        d.a(n, c2, (long)(bi * (n + 1)), 0);
        d.q(n);
        int n4 = d.b((24 * nArray[0] + d.a() % (2 * nArray[0])) / 25, 1, 100);
        int n5 = d.b((24 * nArray[1] + d.a() % (2 * nArray[1])) / 25, 1, 100);
        int n6 = d.b((24 * nArray[2] + d.a() % (2 * nArray[2])) / 25, 1, 100);
        int n7 = d.b((24 * nArray[3] + d.a() % (2 * nArray[3])) / 25, 1, 100);
        int n8 = 101 - n4;
        while (c2.c < 0x500000) {
            d.d(n, n8, c2.d == 0 ? 102 : 104);
            d.a(c2, c2.d, false);
            n3 = d.b(n5, n6, c2.c >> 8, 20480);
            n2 = n8 + 4 + (100 - n3) / 5;
            while (n8 < n2 && c2.c < 0x500000) {
                d.f(c2);
                ++n8;
            }
        }
        int n9 = 0;
        if (d.a() % 100 < n7) {
            n9 = 1;
        }
        n3 = n9;
        n2 = 0x640000;
        if (n3 != 0) {
            n2 -= 409600;
            n2 = (d.a() & 1) == 0 ? (n2 -= (100 - n7) * 524288 / 100) : (n2 += (100 - n7) * 393216 / 100);
        }
        while (c2.c < n2) {
            d.f(c2);
            ++n8;
        }
        if (n3 != 0) {
            d.d(n, n8, 103);
            d.a(c2, 10, false);
            while (c2.c < 0x640000) {
                d.f(c2);
                ++n8;
            }
        }
        return (n8 - 1) * 32;
    }

    private static void aO() {
        aV = 0;
        aW = 0;
        int n = 0;
        while (n < 6) {
            d.a(n, a[n], (long)(bi * (n + 1)), 0);
            d.q(n);
            ++n;
        }
        bs = 1;
        bt = 0;
        bu = 0;
        bv = 0;
    }

    private static void aP() {
        if (ba >= 11) {
            bt += 32;
            switch (bs) {
                case 1: {
                    if (bv == 0) {
                        d.b(false);
                        d.o(23);
                        bv = 1;
                    }
                    if (bt >= 3000) {
                        bs = 2;
                        bt = 0;
                    }
                    return;
                }
                case 2: {
                    if (bv <= 1) {
                        d.b(false);
                        d.o(24);
                        bv = 2;
                    }
                    if (bt >= 2000) {
                        bs = 3;
                        bt = 0;
                        d.v(0);
                    }
                    return;
                }
                case 3: {
                    if (bv <= 2) {
                        d.b(false);
                        d.o(25);
                        bv = 3;
                    }
                    aV += 32;
                }
                case 4: {
                    ++aW;
                    int n = 0;
                    while (n < aP) {
                        if ((n >= aN || n <= aZ) && (bo & 1 << n) == 0) {
                            if (n != aZ) {
                                int n2 = d.f(n, aW);
                                while (n2 >= 0) {
                                    d.b(a[n], n2, false);
                                    n2 = d.f(n, aW);
                                }
                            }
                            d.f(a[n]);
                        }
                        ++n;
                    }
                    if (ba != 11) break;
                    if (bs == 3) {
                        if (d.a[d.aZ].c >= 0x640000) {
                            d.a(13, 128, 1);
                            d.j(aZ, 3, aV * 100 / 1024);
                            bs = 4;
                            bt = 0;
                        }
                        return;
                    }
                    if (bs != 4 || d.a[d.aZ].b >= 0) break;
                    d.aF();
                }
            }
        }
    }

    private static void f(c c2) {
        ++c2.q;
        if (c2.c < 0x500000) {
            d.b(c2);
            if (bs == 3 && c2.b < 0 && bt > 5000) {
                c2.b = 0;
            }
            return;
        }
        if (c2.c < 0x640000) {
            d.d(c2);
            return;
        }
        if (c2.F < 0) {
            c2.F = bu++;
        }
        if (c2.b >= 0) {
            int n = c2.a == aZ ? 0x730000 : 28672 + d.h(bi + 813387 * c2.a) % 1536 << 8;
            d.b(c2, n);
            return;
        }
        c2.p += 32;
    }

    private static void A(int n) {
        if (bs != 3) {
            return;
        }
        d.d(aZ, aW + 1, n);
        d.b(a[aZ], n, true);
    }

    private static void b(c c2, int n, boolean bl) {
        if (bs == 3) {
            if (c2.c < 0x500000) {
                if (d.a(n, 131076)) {
                    d.a(c2, 0, bl);
                }
                if (d.a(n, 131078)) {
                    d.a(c2, 1, bl);
                }
                return;
            }
            if (c2.c < 0x640000 && c2.f < 0 && d.a(n, 131083)) {
                d.a(c2, 10, bl);
            }
        }
    }

    private static void aQ() {
        int n;
        int n2;
        int n3;
        int n4;
        int n5 = d.h(0, 4 - (aO + aZ + 1));
        int n6 = 0;
        int n7 = 0;
        int n8 = 0;
        switch (ba) {
            case 3: 
            case 11: {
                d.B(d.a[d.aZ].c >> 8);
                d.l(d.a[d.aZ].c >> 8, 0, (-6 + aZ + aO + n5) * 333 + 166);
                n6 = bA;
                n7 = bB;
                n3 = n4 = d.a[d.aZ].c >> 8;
                if (bs == 1) {
                    if (bt >= 100) {
                        n3 = bt < 500 ? (n3 += d.b(0, 128, bt - 100, 400)) : (n3 += 128);
                    }
                } else if (bs == 2) {
                    n3 = bt < 100 ? (n3 += 128) : (bt < 500 ? (n3 += d.b(128, 256, bt - 100, 400)) : (n3 += 256));
                } else if (bs == 3 && d.a[d.aZ].c < 655360) {
                    n3 += d.b(256, 0, d.a[d.aZ].c >> 8, 2560);
                }
                n2 = d.a[d.aZ].f;
                if (n2 >= 0) {
                    n3 = (n2 >>= 8) > 22528 ? (n3 += 256) : (n3 += d.b(0, 256, n2 - 19456, 3072));
                    n = n4 - n2;
                    if (n < 1536) {
                        n3 -= n / 3;
                    } else if (n < 3072) {
                        n3 -= 512 - (n - 1536) / 3;
                    }
                } else if (n4 > 22528) {
                    n3 += 256;
                } else if (n4 > 19456) {
                    n3 += d.b(0, 256, n4 - 19456, 3072);
                }
                if (n4 >= 25600) {
                    n3 = n4 < 28672 ? (n3 += d.b(0, -256, n4 - 25600, 3072)) : (n3 -= 256);
                }
                d.B(n3);
                break;
            }
            case 10: {
                d.B(0);
                break;
            }
            default: {
                d.B(29440);
            }
        }
        d.D(n5);
        n4 = 0;
        while (n4 < aO + aZ + 1) {
            n3 = n4 < aO ? aN + n4 : n4 - aO;
            if ((n3 >= aN || n3 <= aZ) && (bo & 1 << n3) == 0) {
                n2 = d.a[n3].c >> 8;
                d.l(n2, 0, (-6 + n4 + n5) * 333 + 166);
                n = d.a[n3].c / 81920 & 3;
                d.a(518 + n, bA - 9, bB, 3, 0);
            }
            ++n4;
        }
        n4 = 0;
        while (n4 < aO + aZ + 1) {
            n3 = n4 < aO ? aN + n4 : n4 - aO;
            if ((n3 >= aN || n3 <= aZ) && (bo & 1 << n3) == 0) {
                int n9;
                int n10;
                n2 = d.a[n3].c >> 8;
                d.l(n2, 0, (-6 + n4 + n5) * 333 + 166);
                n8 = bA;
                short[] sArray = i;
                int n11 = 120;
                boolean bl = true;
                int n12 = -1;
                if (ba != 10) {
                    if (bs == 1) {
                        sArray = j;
                        n10 = bt - 100 - d.h(n3 + 987) % 500;
                        n11 = 120;
                        bl = false;
                    } else if (bs == 2) {
                        sArray = k;
                        n10 = bt - 100 - d.h(n3 + 458) % 500;
                        n11 = 120;
                        bl = false;
                    } else if (d.a[n3].c > 0x640000 && d.a[n3].b < 0) {
                        n9 = d.l(n3, 3);
                        n12 = aP > 1 ? d.n(d.a[n3].F, n9) : d.n(n9);
                        n10 = d.a[n3].p;
                    } else if (d.a[n3].f < 0) {
                        n9 = (l.length - 1) * 160;
                        if (n2 < n9) {
                            sArray = l;
                            n10 = n2;
                            n11 = 160;
                            bl = false;
                        } else {
                            sArray = m;
                            n10 = n2 - n9;
                            n11 = 160;
                            bl = true;
                        }
                    } else {
                        n9 = d.a[n3].f >> 8;
                        int n13 = n9 + (d.n.length - 1) * 200;
                        if (n2 < n13) {
                            sArray = d.n;
                            n10 = n2 - n9;
                            n11 = 200;
                            bl = false;
                        } else {
                            sArray = m;
                            n10 = n2 - n13;
                            n11 = 160;
                            bl = true;
                        }
                    }
                } else {
                    sArray = i;
                    n10 = ch - d.h(n3 + 293) % 800;
                    n11 = 200;
                    bl = true;
                }
                int n14 = 0;
                if (d.a[n3].c == 0) {
                    n14 = 1;
                }
                n9 = 1 - n14;
                if (n12 < 0) {
                    d.a(n9, d.a[n3].a, d.a[n3].d, sArray, n10, n11, bA, bB, bl);
                } else {
                    d.a(n9, d.a[n3].a, d.a[n3].d, n12, n10, bA, bB);
                }
            }
            ++n4;
        }
        n4 = 0;
        n3 = 0;
        n2 = 0;
        if (ba == 11) {
            if (bs == 1) {
                n3 = 1;
                n4 = 1;
            } else if (bs == 2) {
                n3 = 2;
                n4 = 2;
            } else if (bs == 3) {
                int n15 = bt;
                if (n15 < 32) {
                    n3 = 3;
                    n4 = 3;
                } else if (n15 < 432) {
                    n4 = 4 + (n15 - 32) / 200;
                    n3 = n4 + 1;
                    n2 = ((n15 - 32) % 200 << 16) / 200;
                } else {
                    n3 = 7;
                    n4 = 7;
                }
            }
        }
        d.l(512, 0, aI == 2 ? 256 : 512);
        int n16 = 0;
        if (aI == 2) {
            n16 = 1;
        }
        d.a(n16 + 16386, n4, n3, n2, bA, bB, 1024, 0);
        if (ba == 11 || ba == 3) {
            if (d.a[d.aZ].f >= 0 && (n4 = ch - (aX >> 2)) < 200) {
                d.i(n4, n8 - 14, n7 - 124);
            }
            if (bs == 3 && d.a[d.aZ].f < 0 && d.a[d.aZ].c < 0x640000) {
                if (d.a[d.aZ].c >= 0x500000) {
                    if (e) {
                        d.l(n8 - 14, n7 - 124);
                    }
                } else {
                    d.a(a[aZ], n6, n7);
                }
            }
        }
        if (ba == 11 && aX < 0) {
            n4 = 0;
            if (bs == 1) {
                if (bt > 2200) {
                    n4 = d.c(0, -240, 2200 - bt, 400);
                }
                d.a(3, a[147], 120 + n4, 114, 17);
                return;
            }
            if (bs == 2) {
                if (bt > 1200) {
                    n4 = d.c(0, -240, 1200 - bt, 400);
                }
                d.a(3, a[148], 120 + n4, 114, 17);
                return;
            }
            if (bs == 3) {
                if (bt > 1200) {
                    n4 = d.c(0, -240, 1200 - bt, 400);
                }
                d.a(3, a[149], 120 + n4, 114, 17);
            }
        }
    }

    private static void B(int n) {
        bw = n;
        if (n <= 5120) {
            bx = 1792;
            by = 2048;
            bz = 8192;
            return;
        }
        if (n >= 20480) {
            bx = 1280;
            by = 2304;
            bz = 4608;
            return;
        }
        bx = d.b(1792, 1280, (n - 5120) / 60, 256);
        by = d.b(2048, 2304, (n - 5120) / 60, 256);
        bz = d.b(8192, 4608, (n - 5120) / 60, 256);
    }

    private static void m(int n, int n2) {
        d.k(n, n2, 0, 0);
    }

    private static void k(int n, int n2, int n3) {
        d.k(n, n2, n3, 0);
    }

    private static void k(int n, int n2, int n3, int n4) {
        bw = n;
        bx = 1408 + n2;
        by = 2304 + n3;
        bz = 6144 + n4;
    }

    private static void l(int n, int n2, int n3) {
        n -= bw + 128;
        int n4 = (int)((long)d.k(-bz) * (long)(n2 -= bx) + (long)d.j(-bz) * (long)(n3 -= by) >> 14);
        int n5 = (int)((long)(-d.j(-bz)) * (long)n2 + (long)d.k(-bz) * (long)n3 >> 14);
        n2 = n4;
        n3 = n5;
        if (n3 >= 0) {
            bC = n < 0 ? 0 : 0xEFFFFF;
            bA = bC >> 16;
            bD = 0x13FFFFF;
            bB = bD >> 16;
            return;
        }
        bC = 0x780000 + (int)(24641536L * (long)n / (long)(-n3));
        bD = (319 - d.e(0) + 22 >> 1 << 16) - (int)(24641536L * (long)n2 / (long)(-n3));
        bA = bC >> 16;
        bB = bD >> 16;
    }

    private static void C(int n) {
        int n2 = -bx;
        int n3 = (int)((long)d.k(-bz) * (long)n2 + (long)d.j(-bz) * (long)(n -= by) >> 14);
        int n4 = (int)((long)(-d.j(-bz)) * (long)n2 + (long)d.k(-bz) * (long)n >> 14);
        n2 = n3;
        n = n4;
        if (n >= 0) {
            bC = 0x780000;
            bA = bC >> 16;
            bD = 0x13FFFFF;
            bB = bD >> 16;
            return;
        }
        bD = (319 - d.e(0) + 22 >> 1 << 16) - (int)(24641536L * (long)n2 / (long)(-n));
        bB = bD >> 16;
    }

    private static void D(int n) {
        if (aI == 2) {
            d.bR();
            d.aR();
            d.bS();
            return;
        }
        d.E(n);
        d.aR();
        d.f(8200, 512, 0);
    }

    private static void E(int n) {
        int n2 = 0;
        ++n2;
        d.k[0] = 0;
        ++n2;
        d.k[1] = 0;
        d.l(bw, 0, -128000);
        if (bD > 0) {
            ++n2;
            d.k[2] = bD;
            ++n2;
            d.k[3] = 11725567;
        }
        d.l(bw, 0, -17920);
        if (bD > 0) {
            d.k[n2++] = bD;
            d.k[n2++] = 10143323;
        }
        d.l(bw, 0, -15360);
        if (bD > 0) {
            d.k[n2++] = bD;
            d.k[n2++] = ck;
        }
        int n3 = 0;
        while (n3 < 6) {
            int n4 = d.a(-13056, -2022, n3, 5);
            d.l(bw, 0, n4);
            if (bD > 0) {
                int[] nArray = new int[]{10143323, 12374893, 10143323, 11914837, 9814862, 11783245};
                d.k[n2++] = bD;
                d.k[n2++] = nArray[n3];
            }
            ++n3;
        }
        n3 = 0;
        while (n3 < 6) {
            d.l(bw, 0, (-6 + n3) * 333 + 12);
            d.k[n2++] = bD;
            d.k[n2++] = 0xFFFFFF;
            d.l(bw, 0, (-6 + n3 + 1) * 333 - 12);
            d.k[n2++] = bD;
            d.k[n2++] = ba == 10 && e && n3 == n + aO + aZ ? cj : ci;
            ++n3;
        }
        d.l(bw, 0, 24);
        d.k[n2++] = bD;
        d.k[n2++] = 0xFFFFFF;
        d.k[n2++] = 0x1400000;
        d.k[n2++] = 0x3F3F3F;
        ao = n2 >> 1;
        d.J();
        if (aJ == 1) {
            d.f(8203, -20480, 0, 120);
            d.g(8205, -18432, 0, bB);
        } else {
            d.i(8208, -29440, 0, -24320);
            d.i(8207, -24320, 0, -20480);
            d.f(8204, -20480, 0, 120);
            d.g(8206, -18432, 0, bB);
        }
        d.f(8202, -14848, (int)d.a(8202));
        d.f(8201, -2664, 64);
        d.l(25600, 0, -2126);
        d.a(711, bA, bB, 33, 0);
        d.a(712, bA + 32, bB, 33, 0);
    }

    private static void aR() {
        int n;
        int n2;
        int n3;
        b.setColor(0xFFFFFF);
        int n4 = bw - 1920;
        n4 = d.h(0, n4 - n4 % 2560);
        int n5 = 0;
        while (n5 << 8 < 6400) {
            if (n4 + (n5 << 8) != 23040) {
                d.l(n4 + (n5 << 8), 0, -1998);
                n3 = bA;
                n2 = bB;
                d.l(n4 + (n5 << 8), 0, 0);
                b.drawLine(bA, bB, n3, n2);
                if (n4 + (n5 << 8) == 25600 || n4 + (n5 << 8) == 0) {
                    n = 1;
                    while (n < 4) {
                        b.drawLine(bA + n, bB, n3 + n, n2);
                        ++n;
                    }
                }
            }
            n5 += 10;
        }
        if (bw > 21120) {
            b.setColor(0xFFFFFF);
            n5 = 0;
            while (n5 < 3) {
                n3 = 24000 + n5 * 416;
                n2 = 0;
                while (n2 < 6) {
                    d.l(n3, 0, (-6 + n2) * 333 + 82);
                    n = bA;
                    int n6 = bB;
                    d.l(n3, 0, (-6 + n2 + 1) * 333 - 82);
                    d.c(n, n6, bA, bB);
                    ++n2;
                }
                ++n5;
            }
            n5 = 0;
            while (n5 < 6) {
                d.l(25216, 0, (-6 + n5) * 333 + 166);
                d.a(486 + n5, bA, bB, 3, 0);
                ++n5;
            }
        }
        if (bw < 1920) {
            n5 = 0;
            while (n5 < 6) {
                d.l(384, 0, (-6 + n5) * 333 + 166);
                d.a(486 + n5, bA, bB, 3, 0);
                if (aI != 2) {
                    d.l(0, 0, (-6 + n5) * 333 + 166);
                    d.a(709, bA - 6, bB + 5, 40, 0);
                    d.a(710, bA - 6 - 56 - n5 * 28 / 6, bB + 5, 40, 0);
                }
                ++n5;
            }
        }
    }

    private static void aS() {
        aV = 0;
        aW = 0;
        bs = 3;
        bt = 0;
        d.a(-1, a[0], 0L, 0);
        while (d.a[0].c < 0x320000) {
            if (d.a[0].q - d.a[0].h > 10) {
                d.b(a[0], d.a[0].d == 0 ? 102 : 104, false);
            }
            d.f(a[0]);
        }
        aX = -1;
        d.v(d.a[0].q);
    }

    private static void F(int n) {
        switch (n) {
            case 0: {
                d.b(a[0], d.a[0].d == 0 ? 102 : 104, true);
                return;
            }
            case 1: {
                d.b(a[0], 103, true);
            }
        }
    }

    private static void aT() {
        switch (bp & 7) {
            case 0: {
                if (d.a[0].c < 0x500000) {
                    if (d.a[0].q - d.a[0].h > 10) {
                        d.b(a[0], d.a[0].d == 0 ? 102 : 104, true);
                    }
                    d.f(a[0]);
                } else {
                    d.f(a[0]);
                    if (d.a[0].c >= 6144000) {
                        d.w(9);
                    }
                }
                return;
            }
            case 1: {
                d.f(a[0]);
                if (d.a[0].c < 0x6E0000 || G) break;
                d.aF();
            }
        }
    }

    private static int q(int n) {
        if (n == 1) {
            return 187;
        }
        return 0;
    }

    private static void m(int n, int n2, int n3) {
        int n4 = 6 + n2;
        int n5 = 57 + n3;
        d.k(n4, n5);
        if (n == 0) {
            d.a(8196, "1. " + a[37], n4 + 6, n5 + 6, 20);
            int n6 = n5 + 6 + 29 + 14;
            d.a(471, n4 + 228 - 6, n6, 10, 0);
            d.a(470, n4 + 228 - 6 - 34, n6, 10, 0);
            d.a(3, a[43] + ' ', n4 + 228 - 6 - 68, n6 - 11, 24);
            return;
        }
        d.a(8196, "2. " + a[39], n4 + 6, n5 + 6, 20);
        int n7 = n5 + 6 + 29 + 14;
        d.a(472, n4 + 228 - 6, n7, 10, 0);
        d.a(3, a[44] + ' ', n4 + 228 - 6 - 34, n7 - 11, 24);
    }

    private static int r(int n) {
        if (n < 1450) {
            return d.a(600, 1000, 1450 - n, 162);
        }
        return d.a(600, 0, n - 1450, 270);
    }

    private static int s(int n) {
        return -(1288 + n * 7 + n * n * 3);
    }

    private static void aU() {
        b = new c[6];
        int n = 0;
        while (n < 6) {
            d.b[n] = new c();
            d.b[n].a = new int[10];
            d.b[n].b = new int[3];
            ++n;
        }
        d.M();
    }

    private static void aV() {
        int n = 0;
        while (n < 6) {
            d.b[n].a = null;
            d.b[n].b = null;
            d.b[n] = null;
            ++n;
        }
        b = null;
        d.N();
    }

    private static void aW() {
        bc = 5;
        int n = 0;
        while (n < 6) {
            d.p(n);
            ++n;
        }
    }

    private static void aX() {
        int n = 0;
        while (n < aO) {
            int n2 = d.b(b[n + aN], n + aN, d.a[d.aN + n].b);
            d.j(aN + n, 3, n2 * 100 / 1024);
            ++n;
        }
        n = 0;
        while (n < 6) {
            d.a(n, b[n], (long)(bi * (n + 1)), 2);
            ++n;
        }
    }

    private static int b(c c2, int n, int[] nArray) {
        int n2;
        d.a(n, c2, (long)(bi * (n + 1)), 0, 2);
        d.q(n);
        int n3 = d.b((24 * nArray[4] + d.a() % (2 * nArray[4])) / 25, 1, 100);
        int n4 = d.b((24 * nArray[5] + d.a() % (2 * nArray[5])) / 25, 1, 100);
        int n5 = d.b((24 * nArray[6] + d.a() % (2 * nArray[6])) / 25, 1, 100);
        int n6 = d.b((24 * nArray[7] + d.a() % (2 * nArray[7])) / 25, 1, 100);
        int n7 = 101 - n3;
        int n8 = 0;
        while (n8 < 7) {
            n2 = 0;
            while (n2 < 2) {
                ++n7;
                d.a(c2, false);
                ++n2;
            }
            n2 = (100 - n4) / 2;
            while (c2.l < 3) {
                int n9;
                int n10 = n7 + 6;
                n10 = c2.l == 0 ? (n10 += d.b(n2 - 12, 0, 11)) : (c2.l == 1 ? (n10 += d.b(n2 - 12 - 11, 0, 11)) : (n10 += d.h(n2 - 12 - 11 - 11, 0)));
                while (n7 < n10) {
                    d.a(c2, false);
                    ++n7;
                }
                boolean bl = false;
                if (d.a() % 100 > 10 || d.a() % 100 > n5) {
                    bl = true;
                }
                boolean bl2 = bl;
                boolean bl3 = false;
                if (c2.b[c2.l] == 0) {
                    bl3 = true;
                }
                d.d(n, n7, (n9 = 1 - (bl3 ^ bl2 ^ 1)) == 0 ? 102 : 104);
                d.a(c2, n9, false);
            }
            while (c2.l >= 3 && c2.c < 0x640000) {
                ++n7;
                d.a(c2, false);
            }
            ++n8;
        }
        int n11 = 0;
        if (d.a() % 100 < n6) {
            n11 = 1;
        }
        n8 = n11;
        n2 = 0x780000;
        if (n8 != 0) {
            n2 -= 409600;
            n2 = (d.a() & 1) == 0 ? (n2 -= (100 - n6) * 524288 / 100) : (n2 += (100 - n6) * 393216 / 100);
        }
        while (c2.c < n2) {
            d.a(c2, false);
            ++n7;
        }
        if (n8 != 0) {
            d.d(n, n7, 103);
            d.a(c2, 10, false);
            while (c2.c < 0x780000) {
                d.a(c2, false);
                ++n7;
            }
        }
        return (n7 - 1) * 32;
    }

    private static void aY() {
        aV = 0;
        aW = 0;
        int n = 0;
        while (n < 6) {
            d.a(n, b[n], (long)(bi * (n + 1)), 0, 2);
            d.q(n);
            ++n;
        }
        bE = 1;
        bF = 0;
        bG = 0;
        bH = 0;
    }

    private static void aZ() {
        if (ba >= 11) {
            bF += 32;
            switch (bE) {
                case 1: {
                    if (bH == 0) {
                        d.b(false);
                        d.o(23);
                        bH = 1;
                    }
                    if (bF >= 3000) {
                        bE = 2;
                        bF = 0;
                    }
                    return;
                }
                case 2: {
                    if (bH <= 1) {
                        d.b(false);
                        d.o(24);
                        bH = 2;
                    }
                    if (bF >= 2000) {
                        bE = 3;
                        bF = 0;
                        d.v(0);
                    }
                    return;
                }
                case 3: {
                    if (bH <= 2) {
                        d.b(false);
                        d.o(25);
                        bH = 3;
                    }
                    aV += 32;
                }
                case 4: {
                    ++aW;
                    int n = 0;
                    while (n < aP) {
                        if ((n >= aN || n <= aZ) && (bo & 1 << n) == 0) {
                            if (n != aZ) {
                                int n2 = d.f(n, aW);
                                while (n2 >= 0) {
                                    d.c(b[n], n2, false);
                                    n2 = d.f(n, aW);
                                }
                            }
                            c c2 = b[n];
                            boolean bl = false;
                            if (n == aZ) {
                                bl = true;
                            }
                            d.a(c2, bl);
                        }
                        ++n;
                    }
                    if (ba != 11) break;
                    if (bE == 3) {
                        if (d.b[d.aZ].c >= 0x780000) {
                            d.a(13, 128, 1);
                            d.j(aZ, 3, aV * 100 / 1024);
                            bE = 4;
                            bF = 0;
                        }
                        return;
                    }
                    if (bE != 4 || d.b[d.aZ].b >= 0) break;
                    d.aF();
                }
            }
        }
    }

    private static void a(c c2, boolean bl) {
        ++c2.q;
        if (c2.c >= 0x780000) {
            if (c2.F < 0) {
                c2.F = bG++;
            }
            if (c2.b >= 0) {
                int n = c2.a == aZ ? 0x870000 : 33792 + d.h(bi + 813387 * c2.a) % 1536 << 8;
                d.b(c2, n);
            } else {
                c2.p += 32;
            }
            return;
        }
        int n = c2.c;
        if (bE == 3 && c2.b < 0 && bF > 5000) {
            c2.b = 0;
        }
        if (c2.l >= 3) {
            d.d(c2);
        } else {
            d.b(c2);
        }
        int n2 = -117760;
        if ((n + 327680 - n2) / 983040 < (c2.c + 327680 - n2) / 983040) {
            int n3;
            if (c2.l < 3 && c2.m == 0) {
                c2.m = 1;
            }
            if ((n3 = (c2.c - 655360 - n2) / 983040) < 7) {
                if (c2.m > 0) {
                    c2.o |= 1 << n3;
                }
                if (bl && ba != 3) {
                    if (c2.m > 1) {
                        d.a(22, 128, 1);
                    } else {
                        d.a(18, 128, 1);
                    }
                }
                d.a(c2);
                if (n3 >= 6) {
                    c2.l = 3;
                }
            }
        }
    }

    private static void G(int n) {
        if (bE != 3) {
            return;
        }
        d.d(aZ, aW + 1, n);
        d.c(b[aZ], n, true);
    }

    private static void c(c c2, int n, boolean bl) {
        if (bE == 3) {
            if (d.a(n, 131076)) {
                int n2 = (c2.c + 327680) % 983040;
                if (n2 >= 0 && n2 < 865280) {
                    d.a(c2, 0, bl);
                }
                return;
            }
            if (d.a(n, 131078)) {
                int n3 = (c2.c + 327680) % 983040;
                if (n3 >= 0 && n3 < 865280) {
                    d.a(c2, 1, bl);
                }
                return;
            }
            if (d.a(n, 131083) && c2.c >= 0x640000) {
                d.a(c2, 10, bl);
            }
        }
    }

    private static void ba() {
        int n;
        int n2;
        int n3;
        int n4;
        int n5 = d.h(0, 4 - (aO + aZ + 1));
        int n6 = 0;
        int n7 = 0;
        int n8 = 0;
        switch (ba) {
            case 3: 
            case 11: {
                d.B(d.b[d.aZ].c >> 8);
                d.l(d.b[d.aZ].c >> 8, 0, (-6 + aO + aZ + n5) * 333 + 166);
                n6 = bA;
                n7 = bB;
                n3 = n4 = d.b[d.aZ].c >> 8;
                if (bE == 1) {
                    if (bF >= 100) {
                        n3 = bF < 500 ? (n3 += d.b(0, 128, bF - 100, 400)) : (n3 += 128);
                    }
                } else if (bE == 2) {
                    n3 = bF < 100 ? (n3 += 128) : (bF < 500 ? (n3 += d.b(128, 256, bF - 100, 400)) : (n3 += 256));
                } else if (bE == 3 && d.b[d.aZ].c < 655360) {
                    n3 += d.b(256, 0, d.b[d.aZ].c >> 8, 2560);
                }
                n2 = d.b[d.aZ].f;
                if (n2 >= 0) {
                    n3 = (n2 >>= 8) > 27648 ? (n3 += 256) : (n3 += d.b(0, 256, n2 - 24576, 3072));
                    n = n4 - n2;
                    if (n < 1536) {
                        n3 -= n / 3;
                    } else if (n < 3072) {
                        n3 -= 512 - (n - 1536) / 3;
                    }
                } else if (n4 > 27648) {
                    n3 += 256;
                } else if (n4 > 24576) {
                    n3 += d.b(0, 256, n4 - 24576, 3072);
                }
                if (n4 >= 30720) {
                    n3 = n4 < 33792 ? (n3 += d.b(0, -256, n4 - 30720, 3072)) : (n3 -= 256);
                }
                d.B(n3);
                break;
            }
            case 10: {
                d.B(0);
                break;
            }
            default: {
                d.B(34560);
            }
        }
        d.H(n5);
        n4 = 0;
        while (n4 < aO + aZ + 1) {
            n3 = n4 < aO ? aN + n4 : n4 - aO;
            if ((n3 >= aN || n3 <= aZ) && (bo & 1 << n3) == 0) {
                n2 = d.b[n3].c >> 8;
                d.l(n2, 0, (-6 + n4 + n5) * 333 + 166);
                n = d.b[n3].c / 81920 & 3;
                d.a(518 + n, bA - 9, bB, 3, 0);
            }
            ++n4;
        }
        n4 = 0;
        while (n4 < aO + aZ + 1) {
            n3 = n4 < aO ? aN + n4 : n4 - aO;
            if ((n3 >= aN || n3 <= aZ) && (bo & 1 << n3) == 0) {
                int n9;
                int n10;
                n2 = d.b[n3].c >> 8;
                d.l(n2, 0, (-6 + n4 + n5) * 333 + 166);
                n8 = bA;
                short[] sArray = i;
                int n11 = 120;
                boolean bl = true;
                int n12 = -1;
                if (ba != 10) {
                    int n13;
                    if (bE == 1) {
                        sArray = j;
                        n10 = bF - 100 - d.h(n3 + 987) % 500;
                        n11 = 120;
                        bl = false;
                    } else if (bE == 2) {
                        sArray = k;
                        n10 = bF - 100 - d.h(n3 + 458) % 500;
                        n11 = 120;
                        bl = false;
                    } else if (d.b[n3].c > 0x780000 && d.b[n3].b < 0) {
                        n9 = d.l(n3, 3);
                        n12 = aP > 1 ? d.n(d.b[n3].F, n9) : d.n(n9);
                        n10 = d.b[n3].p;
                    } else if (d.b[n3].f < 0) {
                        n9 = 3840 / (G.length - 1) + 1;
                        n13 = 25140 + n9 * (G.length - 1);
                        if (n2 < 2100) {
                            sArray = F;
                            n10 = n2;
                            n11 = 2100 / (F.length - 1);
                            bl = false;
                        } else if (n2 < n13) {
                            int n14 = (n2 - 2100) % 3840;
                            switch (d.b[n3].n) {
                                case 1: {
                                    sArray = H;
                                    n10 = n14;
                                    n11 = n9;
                                    bl = false;
                                    break;
                                }
                                case 2: {
                                    sArray = I;
                                    n10 = n14;
                                    n11 = n9;
                                    bl = false;
                                    break;
                                }
                                case 3: {
                                    sArray = J;
                                    n10 = n14;
                                    n11 = n9;
                                    bl = false;
                                    break;
                                }
                                default: {
                                    sArray = G;
                                    n10 = n14;
                                    n11 = n9;
                                    bl = false;
                                    break;
                                }
                            }
                        } else {
                            sArray = m;
                            n10 = n2 - n13;
                            n11 = n9;
                            bl = true;
                        }
                    } else {
                        n9 = d.b[n3].f >> 8;
                        n13 = n9 + (d.n.length - 1) * 200;
                        if (n2 < n13) {
                            sArray = d.n;
                            n10 = n2 - n9;
                            n11 = 200;
                            bl = false;
                        } else {
                            sArray = m;
                            n10 = n2 - n13;
                            n11 = 160;
                            bl = true;
                        }
                    }
                } else {
                    sArray = i;
                    n10 = ch - d.h(n3 + 293) % 800;
                    n11 = 200;
                    bl = true;
                }
                int n15 = 0;
                if (d.a[n3].c == 0) {
                    n15 = 1;
                }
                n9 = 1 - n15;
                if (n12 < 0) {
                    d.a(n9, d.a[n3].a, d.a[n3].d, sArray, n10, n11, bA, bB, bl);
                } else {
                    d.a(n9, d.a[n3].a, d.a[n3].d, n12, n10, bA, bB);
                }
            }
            ++n4;
        }
        n4 = 0;
        n3 = 0;
        n2 = 0;
        if (ba == 11) {
            if (bs == 1) {
                n3 = 1;
                n4 = 1;
            } else if (bE == 2) {
                n3 = 2;
                n4 = 2;
            } else if (bE == 3) {
                int n16 = bF;
                if (n16 < 32) {
                    n3 = 3;
                    n4 = 3;
                } else if (n16 < 432) {
                    n4 = 4 + (n16 - 32) / 200;
                    n3 = n4 + 1;
                    n2 = ((n16 - 32) % 200 << 16) / 200;
                } else {
                    n3 = 7;
                    n4 = 7;
                }
            }
        }
        d.l(512, 0, aI == 2 ? 256 : 512);
        int n17 = 0;
        if (aI == 2) {
            n17 = 1;
        }
        d.a(n17 + 16386, n4, n3, n2, bA, bB, 1024, 0);
        if (ba == 11 || ba == 3) {
            if (d.b[d.aZ].f >= 0 && (n4 = ch - (aX >> 2)) < 200) {
                d.i(n4, n8 - 14, n7 - 124);
            }
            if (bE == 3 && d.b[d.aZ].f < 0 && d.b[d.aZ].c < 0x780000) {
                if (d.b[d.aZ].c >= 0x640000) {
                    if (e) {
                        d.l(n8 - 14, n7 - 124);
                    }
                } else {
                    n4 = (d.b[d.aZ].c + 327680) % 983040;
                    if (n4 >= 0 && n4 < 865280) {
                        d.a(b[aZ], n6, n7);
                    }
                }
            }
        }
        if (ba == 11 && aX < 0) {
            n4 = 0;
            if (bE == 1) {
                if (bF > 2200) {
                    n4 = d.c(0, -240, 2200 - bF, 400);
                }
                d.a(3, a[147], 120 + n4, 114, 17);
                return;
            }
            if (bE == 2) {
                if (bF > 1200) {
                    n4 = d.c(0, -240, 1200 - bF, 400);
                }
                d.a(3, a[148], 120 + n4, 114, 17);
                return;
            }
            if (bE == 3) {
                if (bF > 1200) {
                    n4 = d.c(0, -240, 1200 - bF, 400);
                }
                d.a(3, a[149], 120 + n4, 114, 17);
            }
        }
    }

    private static void H(int n) {
        if (aI == 2) {
            d.bR();
            d.J(n);
            d.bS();
            return;
        }
        d.I(n);
        d.J(n);
        d.f(8200, 512, 0);
    }

    private static void I(int n) {
        int n2 = 0;
        ++n2;
        d.k[0] = 0;
        ++n2;
        d.k[1] = 0;
        d.l(bw, 0, -128000);
        if (bD > 0) {
            ++n2;
            d.k[2] = bD;
            ++n2;
            d.k[3] = 11725567;
        }
        d.l(bw, 0, -17920);
        if (bD > 0) {
            d.k[n2++] = bD;
            d.k[n2++] = 10143323;
        }
        d.l(bw, 0, -15360);
        if (bD > 0) {
            d.k[n2++] = bD;
            d.k[n2++] = ck;
        }
        int n3 = 0;
        while (n3 < 6) {
            int n4 = d.a(-13056, -2022, n3, 5);
            d.l(bw, 0, n4);
            if (bD > 0) {
                int[] nArray = new int[]{10143323, 12374893, 10143323, 11914837, 9814862, 11783245};
                d.k[n2++] = bD;
                d.k[n2++] = nArray[n3];
            }
            ++n3;
        }
        n3 = 0;
        while (n3 < 6) {
            d.l(bw, 0, (-6 + n3) * 333 + 12);
            d.k[n2++] = bD;
            d.k[n2++] = 0xFFFFFF;
            d.l(bw, 0, (-6 + n3 + 1) * 333 - 12);
            d.k[n2++] = bD;
            d.k[n2++] = ba == 10 && e && n3 == n + aO + aZ ? cj : ci;
            ++n3;
        }
        d.l(bw, 0, 24);
        d.k[n2++] = bD;
        d.k[n2++] = 0xFFFFFF;
        d.k[n2++] = 0x1400000;
        d.k[n2++] = 0x3F3F3F;
        ao = n2 >> 1;
        d.J();
        if (aJ == 1) {
            d.f(8203, -20480, 0, 120);
            d.g(8205, -18432, 0, bB);
        } else {
            d.i(8208, -29440, 0, -24320);
            d.i(8207, -24320, 0, -20480);
            d.f(8204, -20480, 0, 120);
            d.g(8206, -18432, 0, bB);
        }
        d.f(8202, -14848, (int)d.a(8202));
        d.f(8201, -2664, 64);
        d.l(30720, 0, -2126);
        d.a(711, bA, bB, 33, 0);
        d.a(712, bA + 32, bB, 33, 0);
    }

    private static void J(int n) {
        int n2;
        int n3;
        int n4;
        int n5;
        int n6;
        if (bw < 1920) {
            d.l(0, 0, -1998);
            n6 = bA;
            n5 = bB;
            d.l(0, 0, 0);
            b.setColor(0xFFFFFF);
            n4 = 0;
            while (n4 < 4) {
                b.drawLine(n6 + n4, n5, bA + n4, bB);
                ++n4;
            }
        }
        if (bw > 28800) {
            d.l(30720, 0, -1998);
            n6 = bA;
            n5 = bB;
            d.l(30720, 0, 0);
            b.setColor(0xFFFFFF);
            n4 = 0;
            while (n4 < 4) {
                b.drawLine(n6 + n4, n5, bA + n4, bB);
                ++n4;
            }
        }
        if (bw > 21120) {
            b.setColor(0xFFFFFF);
            n6 = 0;
            while (n6 < 3) {
                n5 = 29120 + n6 * 416;
                n4 = 0;
                while (n4 < 6) {
                    d.l(n5, 0, (-6 + n4) * 333 + 82);
                    n3 = bA;
                    n2 = bB;
                    d.l(n5, 0, (-6 + n4 + 1) * 333 - 82);
                    d.c(n3, n2, bA, bB);
                    ++n4;
                }
                ++n6;
            }
            n6 = 0;
            while (n6 < 6) {
                d.l(30336, 0, (-6 + n6) * 333 + 166);
                d.a(486 + n6, bA, bB, 3, 0);
                ++n6;
            }
        }
        if (bw < 1920) {
            n6 = 0;
            while (n6 < 6) {
                d.l(384, 0, (-6 + n6) * 333 + 166);
                d.a(486 + n6, bA, bB, 3, 0);
                if (aI != 2) {
                    d.l(0, 0, (-6 + n6) * 333 + 166);
                    d.a(709, bA - 6, bB + 5, 40, 0);
                    d.a(710, bA - 6 - 56 - n6 * 28 / 6, bB + 5, 40, 0);
                }
                ++n6;
            }
        }
        n6 = 0;
        while (n6 < 6) {
            n5 = 10;
            while (n5 <= 100) {
                if (n5 << 8 > bw - 1920 && n5 << 8 < bw + 1920) {
                    d.l(n5 << 8, 0, (-6 + n6) * 333 + 12);
                    n4 = bA;
                    n3 = bB;
                    d.l(n5 << 8, 0, (-5 + n6) * 333 - 12);
                    if (n6 < n || n6 >= n + aO + aZ + 1) {
                        d.a(n4, n3, bA, bB, false);
                    } else {
                        n2 = n6 - n < aO ? aN + (n6 - n) : n6 - n - aO;
                        boolean bl = false;
                        if (d.b[n2].c > (n5 << 16) + -25600 && (d.b[n2].o & 1 << (n5 - 10) / 15) != 0) {
                            bl = true;
                        }
                        boolean bl2 = bl;
                        d.a(n4, n3, bA, bB, bl2);
                    }
                }
                n5 += 15;
            }
            ++n6;
        }
    }

    private static void a(int n, int n2, int n3, int n4, boolean bl) {
        short s = d.b(16384);
        d.a(16384, bl ? 1 : 0, n, n2, 1024, 0);
        if (bl) {
            b.setColor(0);
            b.drawLine(n + s - 1, n2 - 2, n + s - 8, n2 - 2);
            b.setColor(0xFFFFFF);
            int n5 = 2;
            while (n5 <= 7) {
                b.drawLine(n + s - n5, n2 - 1, n3 + s - n5, n4 - 1);
                ++n5;
            }
            b.setColor(0);
            b.drawLine(n + s - 1, n2 - 1, n3 + s - 1, n4 - 1);
            b.drawLine(n + s - 8, n2 - 1, n3 + s - 8, n4 - 1);
        } else {
            b.setColor(0);
            b.drawLine(n - 2, n2 - s + 1, n - 2, n2 - s + 8);
            b.setColor(0xFFFFFF);
            int n6 = 2;
            while (n6 <= 7) {
                b.drawLine(n - 1, n2 - s + n6, n3 - 1, n4 - s + n6);
                ++n6;
            }
            b.setColor(0);
            b.drawLine(n - 1, n2 - s + 1, n3 - 1, n4 - s + 1);
            b.drawLine(n - 1, n2 - s + 8, n3 - 1, n4 - s + 8);
        }
        d.a(16384, bl ? 1 : 0, n3, n4, 1024, 0);
    }

    private static void bb() {
        aV = 0;
        aW = 0;
        bE = 3;
        bF = 0;
        d.a(-1, b[0], 0L, 0, 2);
        d.c(b[0], d.b[0].b[d.b[0].l] == 0 ? 102 : 104, false);
        while (d.b[0].c < 0x490000) {
            int n = (d.b[0].c - 655360) % 983040;
            if (d.b[0].l < 3 && n > 196608 + d.b[0].l * 196608) {
                d.c(b[0], d.b[0].b[d.b[0].l] == 0 ? 102 : 104, false);
            }
            ++d.b[0].q;
            d.a(b[0], false);
        }
        aX = -1;
        d.v(d.b[0].q);
    }

    private static void K(int n) {
        switch (n) {
            case 0: {
                d.c(b[0], d.b[0].d == 0 ? 102 : 104, true);
                return;
            }
            case 1: {
                d.c(b[0], 103, true);
            }
        }
    }

    private static void bc() {
        switch (bp & 7) {
            case 0: {
                int n = (d.b[0].c - 655360) % 983040;
                if (d.b[0].l < 3 && n > 196608 + d.b[0].l * 98304) {
                    d.c(b[0], d.b[0].b[d.b[0].l] == 0 ? 102 : 104, true);
                }
                ++d.b[0].q;
                d.a(b[0], true);
                if (d.b[0].c >= 7454720) {
                    d.w(9);
                }
                return;
            }
            case 1: {
                d.a(b[0], true);
                if (d.b[0].c < 0x820000 || G) break;
                d.aF();
            }
        }
    }

    private static int t(int n) {
        if (n == 1) {
            return 187;
        }
        return 0;
    }

    private static void n(int n, int n2, int n3) {
        d.m(n, n2, n3);
    }

    private static int u(int n) {
        if (n > 550) {
            return d.a(600, 1000, n - 550, 345);
        }
        return d.a(600, 0, 550 - n, 250);
    }

    private static int v(int n) {
        return 895 - 5 * n - 4 * n * n;
    }

    private static void bd() {
        c = new c();
        d.c.a = new int[10];
        d.c.b = new int[3];
        c = new c[6];
        int n = 0;
        while (n < 6) {
            d.c[n] = new c();
            d.c[n].a = new int[10];
            d.c[n].b = new int[3];
            ++n;
        }
        d.M();
    }

    private static void be() {
        d.c.a = null;
        d.c.b = null;
        c = null;
        int n = 0;
        while (n < 6) {
            d.c[n].a = null;
            d.c[n].b = null;
            d.c[n] = null;
            ++n;
        }
        c = null;
        d.N();
    }

    private static void bf() {
        bc = 10;
    }

    private static void bg() {
        d.a(aZ, c, (long)(bi * (aZ + 1) + bd * 479111), -3889152);
        d.c.o = 1;
        d.c.p = 0;
        d.p(aZ);
    }

    private static void bh() {
        int n = aN;
        while (n < aP) {
            int n2 = d.c(c[n], n, d.a[n].b);
            d.j(n, bd, n2);
            ++n;
        }
    }

    private static void bi() {
        bI = -131072;
        bJ = 0;
        int n = 0;
        while (n < aP) {
            int n2 = B[n];
            if ((bo & 1 << n2) == 0) {
                d.b(c[n2], n2, n);
                d.c[n2].i = -80 + 160 * (aP - 1 - n) / aP + d.a() % (160 / aP);
                if (n == 0) {
                    int n3 = d.l(n2, bd);
                    bJ = n3 == -1 ? 0 : d.h(0, (n3 * 416 << 8) / 100);
                }
            }
            ++n;
        }
        n = 0;
        while (n < aP) {
            d.c[d.B[n]].F = n;
            ++n;
        }
    }

    private static int c(c c2, int n, int[] nArray) {
        int n2;
        d.a(n, c2, (long)(bi * (n + 1) + bd * 479111), -3889152);
        c2.o = 1;
        c2.p = 0;
        d.p(n);
        int n3 = d.b((24 * nArray[8] + d.a() % (2 * nArray[8])) / 25, 1, 100);
        int n4 = d.b((24 * nArray[9] + d.a() % (2 * nArray[9])) / 25, 1, 100);
        int n5 = d.b((24 * nArray[10] + d.a() % (2 * nArray[10])) / 25, 1, 100);
        int n6 = d.b((24 * nArray[11] + d.a() % (2 * nArray[11])) / 25, 1, 100);
        while (c2.o == 2 || c2.o == 1) {
            d.d(n, c2.q, c2.d == 0 ? 102 : 104);
            d.d(c2, c2.d == 0 ? 102 : 104, false);
            n2 = c2.q + 4 + (100 - n3) / 5;
            while (c2.q < n2) {
                d.b(c2, false);
            }
        }
        n2 = (-100 + n4 + d.a() % n5) * 262144 / 100;
        while (c2.c + c2.b < n2) {
            d.b(c2, false);
        }
        d.d(n, c2.q, 103);
        d.d(c2, 103, false);
        int n7 = -d.h(0, 10000 - n6 * 100);
        while (c2.j > n7 && (c2.o == 4 || c2.o == 6 || c2.o == 5)) {
            d.b(c2, false);
        }
        d.d(n, c2.q, 103);
        d.d(c2, 103, false);
        while (c2.o == 4 || c2.o == 6 || c2.o == 5) {
            d.b(c2, false);
        }
        if (c2.a) {
            return -1;
        }
        return c2.c * 100 / 416 >> 8;
    }

    private static void b(c c2, int n, int n2) {
        d.a(n, c2, (long)(bi * (n + 1) + bd * 479111), -3889152);
        c2.o = 1;
        c2.p = 0;
        d.q(n);
        while (c2.c < -131072 - (4 * (aP - 1 - n2) << 16) && c2.o != 9) {
            int n3 = d.f(n, c2.q);
            while (n3 >= 0) {
                if ((n3 & 0x40000000) == 0) {
                    d.d(c2, n3, false);
                }
                n3 = d.f(n, c2.q);
            }
            d.b(c2, false);
        }
    }

    private static void bj() {
        if (ba == 11) {
            d.b(c, true);
            return;
        }
        if (ba == 12 || ba == 13) {
            d.b(c, true);
            return;
        }
        if (ba >= 15) {
            int n = 0;
            while (n < aP) {
                if ((bo & 1 << n) == 0) {
                    c c2 = c[n];
                    int n2 = d.f(n, c2.q);
                    while (n2 >= 0) {
                        if ((n2 & 0x40000000) == 0) {
                            d.d(c2, n2, false);
                        }
                        n2 = d.f(n, c2.q);
                    }
                    d.b(c2, false);
                }
                ++n;
            }
            if (ba == 15 && bb < 3000) {
                bI = d.b(0, bJ >> 8, bb, 3000) << 8;
                return;
            }
            bI = bJ;
        }
    }

    private static void b(c c2, boolean bl) {
        c2.p += 32;
        ++c2.q;
        if (c2.o == 1) {
            if (c2.p >= 120 * (u.length - 1)) {
                c2.o = 2;
                c2.p = 0;
                c2.b = 0;
            }
            return;
        }
        if (c2.o == 2) {
            d.b(c2);
            if (c2.c >= -1048576) {
                c2.o = 3;
                c2.p = 0;
            }
            return;
        }
        if (c2.o == 3) {
            d.d(c2);
            if (c2.c >= 131072) {
                c2.a = true;
                c2.o = 10;
                c2.p = 0;
                if (bl) {
                    d.j(aZ, bd, -1);
                    d.aF();
                }
            }
            return;
        }
        if (c2.o == 10 || c2.o == 11) {
            d.d(c2);
            return;
        }
        if (c2.o == 4 || c2.o == 6 || c2.o == 5) {
            if (c2.o == 4 && c2.j <= 0 || c2.o == 6) {
                c2.b -= 600;
            }
            if (c2.b < 0) {
                c2.b = 0;
            }
            d.a(c2, 650);
            if (c2.k < 0) {
                c2.k = 0;
                c2.j = 0;
                if (c2.c < 0) {
                    c2.a = true;
                }
                if (c2.c > 106496) {
                    d.a(c2.c << 2, c2.k << 2, 3, 0, 0, 0, 0, bl ? 1024 : 512, -1, -1);
                }
                if (c2.a) {
                    c2.o = 11;
                    c2.b = 16000;
                    if (bl) {
                        d.a(22, 128, 1);
                        d.j(aZ, bd, -1);
                        d.aF();
                    }
                } else {
                    if (c2.o == 6 || c2.o == 4) {
                        if (bl) {
                            d.a(22, 128, 1);
                        }
                        c2.o = 8;
                        if (c2.b < 16000) {
                            c2.b = 16000;
                        }
                    } else {
                        if (bl && ba != 3) {
                            d.a(26, 128, 1);
                        }
                        c2.o = 7;
                        if (c2.b < 20000) {
                            c2.b = 20000;
                        }
                    }
                    if (bl) {
                        d.j(aZ, bd, c2.c * 100 / 106496);
                    }
                }
                c2.p = 0;
            }
            return;
        }
        if (c2.o == 7 || c2.o == 8) {
            c2.b -= 1400;
            if (c2.b < 0) {
                c2.b = 0;
            }
            d.d(c2);
            if (c2.o == 7 && c2.p >= 1080 && c2.p < 1200 || c2.o == 8 && c2.p >= 60 * (v.length - 1) + 480 && c2.p < 60 * (v.length - 1) + 600) {
                c2.c += 4000;
            }
            if (c2.p > 2000) {
                c2.o = 9;
                c2.p = 0;
                if (bl && ba == 11 && !G) {
                    d.aF();
                }
            }
        }
    }

    private static void L(int n) {
        d.d(aZ, d.c.q, n);
        d.d(c, n, true);
    }

    private static void d(c c2, int n, boolean bl) {
        if (c2.o == 2 || c2.o == 1) {
            if (c2.o == 1) {
                c2.o = 2;
                c2.p = 0;
                c2.b = 0;
            }
            if (d.a(n, 131076)) {
                d.a(c2, 0, bl);
            } else if (d.a(n, 131078)) {
                d.a(c2, 1, bl);
            }
            return;
        }
        if (c2.o == 3) {
            if (d.a(n, 131083)) {
                c2.o = 4;
                c2.p = 0;
                c2.r = c2.c;
                boolean bl2 = false;
                if (c2.c > 0) {
                    bl2 = c2.a = true;
                }
                if (c2.a) {
                    c2.o = 11;
                    c2.p = 0;
                    c2.b = 16000 + (c2.b - 16000) / 2;
                    if (bl) {
                        d.a(22, 128, 1);
                        d.j(aZ, bd, -1);
                        d.aF();
                    }
                } else {
                    c2.j = 4000 + c2.b * 300 / 1000;
                    if (c2.j > 13000) {
                        c2.j = 13000;
                    }
                    c2.b = 16000 + (c2.b - 16000) / 2;
                    if (bl && ba != 3) {
                        d.P();
                        d.a(18, 128, 1);
                    }
                }
            }
            return;
        }
        if (c2.o == 4 && d.a(n, 131083)) {
            if (c2.j - 650 <= 0) {
                c2.o = 5;
                c2.p = 0;
                c2.j += d.h(0, 8000 - d.i(c2.j) / 4);
                if (bl && ba != 3) {
                    d.P();
                    d.a(20, 128, 1);
                }
                return;
            }
            c2.o = 6;
            c2.p = 0;
            if (bl) {
                d.a(22, 128, 1);
            }
        }
    }

    private static void M(int n) {
        d.l(n, 0, 0, 0);
    }

    private static void l(int n, int n2, int n3, int n4) {
        if (n < -4096) {
            n2 -= d.b(0, 512, n - -15192, 11096);
            n4 -= d.b(0, 3584, n - -15192, 11096);
        } else {
            n2 -= 512;
            n4 -= 3584;
        }
        d.k(n, n2, n3, n4);
    }

    private static void bk() {
        switch (ba) {
            case 3: 
            case 10: 
            case 11: 
            case 12: 
            case 13: {
                if (d.c.o == 2 || d.c.o == 1) {
                    d.M(d.c.c >> 8);
                    break;
                }
                if (d.c.o == 3) {
                    if (d.c.c < -524288) {
                        d.M((d.c.c >> 8) + d.a(0, 256, (d.c.c >> 8) + 4096, 2048));
                        break;
                    }
                    if (d.c.c > 0) {
                        d.M(256);
                        break;
                    }
                    d.M((d.c.c >> 8) + 256);
                    break;
                }
                if (d.c.o == 4) {
                    if (d.c.a) {
                        d.M(256);
                        break;
                    }
                    if (d.c.p < 400) {
                        d.M((d.c.c >> 8) + d.b(256, -128, d.c.p, 400));
                        break;
                    }
                    d.M((d.c.c >> 8) - 128);
                    break;
                }
                if (d.c.o == 5 || d.c.o == 6) {
                    if (d.c.a) {
                        d.M(256);
                        break;
                    }
                    d.M((d.c.c >> 8) - 128);
                    break;
                }
                if (d.c.o == 10 || d.c.o == 11) {
                    d.M(256);
                    break;
                }
                if (d.c.o == 7 || d.c.o == 8) {
                    if (d.c.p < 400) {
                        d.M((d.c.c >> 8) + d.b(-128, -256, d.c.p, 400));
                        break;
                    }
                    d.M((d.c.c >> 8) - 256);
                    break;
                }
                d.M((d.c.c >> 8) - 256);
                break;
            }
            default: {
                d.M(bI >> 8);
            }
        }
        d.bl();
        if (ba == 10 || ba == 11 || ba == 12 || ba == 13 || ba == 3) {
            d.l(0, 0, -640);
            int n = 0;
            if (d.c.a && (ba == 12 && bb > 500 || ba == 13)) {
                n = 1;
            }
            int n2 = n;
            d.a(aI == 2 ? 16388 : 16385, n2, bA, bB, 1024, 0);
            d.e(c, aZ, false);
            d.g(c);
            d.c(c, aZ, 0);
            d.e(c, aZ, true);
            if (ba == 11 || ba == 3) {
                if (d.c.o == 2 || d.c.o == 1) {
                    d.a(c, bA, bB);
                }
                if (d.c.o == 3) {
                    if (e) {
                        d.l(bA - 14, bB - 124);
                    }
                } else if (d.c.o == 4) {
                    if (!d.c.a && d.c.j - 650 <= 0) {
                        d.b(bA - 14, bB - 124 + 17, true);
                    }
                } else if (d.c.o == 5) {
                    if (d.c.p < 200) {
                        d.i(d.c.p, bA - 14, bB - 124 + 17);
                    }
                } else if (d.c.o == 6) {
                    d.a(477, bA - 14, bB - 124 + 17, 3, 0);
                }
            }
            return;
        }
        if (ba >= 15) {
            int n = 0;
            while (n < aP) {
                if ((bo & 1 << n) == 0) {
                    d.e(c[n], n, false);
                }
                ++n;
            }
            n = 0;
            while (n < aP) {
                if ((bo & 1 << n) == 0) {
                    d.g(c[n]);
                }
                ++n;
            }
            n = 0;
            while (n < aP) {
                int n3 = B[aP - 1 - n];
                if ((bo & 1 << n3) == 0) {
                    d.c(c[n3], n3, aP - 1 - n);
                    d.e(c[n], n, true);
                }
                ++n;
            }
        }
    }

    private static void e(c c2, int n, boolean bl) {
        int n2;
        if ((c2.o == 7 || c2.o == 8 || c2.o == 9) && (n2 = d.l(n, bd) * 416 / 100) > 416) {
            d.l(d.l(n, bd) * 416 / 100, 0, c2.i);
            if (!bl) {
                d.a(482, bA - 6, bB, 6, 0);
                return;
            }
            d.a(462, bA - 6, bB - 3, 20, 0);
        }
    }

    private static void g(c c2) {
        d.l(c2.c >> 8, 0, c2.i);
        int n = c2.c / 81920 & 3;
        d.a(518 + n, bA - 9, bB, 3, 0);
    }

    private static void c(c c2, int n, int n2) {
        int n3;
        if (c2.o != 2 && c2.o != 1 && c2.o != 3 && c2.o != 10) {
            d.l(c2.r >> 8, 0, c2.i);
            int n4 = 0;
            if (c2.a && e) {
                n4 = 1;
            }
            d.a(n4 + 478, bA + 1, bB, 10, 0);
        }
        int n5 = c2.c >> 8;
        d.l(n5, c2.k >> 8, c2.i);
        if (ba == 15 && c2.o == 9) {
            int n6 = d.l(n, bd);
            if (n6 != -1) {
                int n7 = 0;
                if (n2 == 0) {
                    n7 = 1;
                }
                int n8 = n7 + 5;
                int n9 = n2 == 0 ? 25 : 16;
                boolean bl = false;
                if (n2 == 0) {
                    bl = true;
                }
                String string = d.a(n6, bc, bl);
                int n10 = bB - (256 * d.a[n].d >> 8) - 4;
                boolean bl2 = false;
                if (n >= aN) {
                    bl2 = true;
                }
                d.a(n8, n9, string, bA, n10, 1, bl2);
            } else {
                int n11 = bB - (256 * d.a[n].d >> 8) - 4;
                boolean bl = false;
                if (n >= aN) {
                    bl = true;
                }
                d.a(3, 20, "FOUL", bA, n11, 1, bl);
            }
        }
        short[] sArray = o;
        int n12 = 160;
        int n13 = n5 + 40;
        boolean bl = true;
        int n14 = -1;
        if (c2.o == 1) {
            sArray = u;
            n13 = c2.p;
            n12 = 120;
            bl = false;
        } else if (c2.o != 2) {
            if (c2.o == 4) {
                sArray = c2.a ? r : p;
                n13 = c2.p;
                n12 = 80;
                bl = false;
            } else if (c2.o == 5) {
                sArray = c2.a ? s : q;
                n12 = 76;
                n13 = c2.p;
                bl = false;
            } else if (c2.o == 6) {
                sArray = p;
                n13 = 80 * (p.length - 1);
                n12 = 80;
                bl = false;
            } else if (c2.o == 7) {
                sArray = t;
                n13 = c2.p;
                n12 = 120;
                bl = false;
            } else if (c2.o == 8) {
                if (c2.p < 60 * (v.length - 1)) {
                    sArray = v;
                    n13 = c2.p;
                    n12 = 60;
                    bl = false;
                } else {
                    sArray = w;
                    n13 = c2.p - 60 * (v.length - 1);
                    n12 = 120;
                    bl = false;
                }
            } else if (c2.o == 9) {
                n13 = c2.p;
                n12 = 120;
                n3 = d.l(n, bd);
                n14 = ba >= 15 && aP > 1 ? d.n(c2.F, n3) : d.n(n3);
            } else if (c2.o == 11) {
                if ((n5 -= c2.r >> 8) < 280 * (r.length - 1)) {
                    sArray = r;
                    n13 = n5;
                    n12 = 280;
                    bl = false;
                } else {
                    sArray = o;
                    n13 = n5 - 280 * (r.length - 1);
                }
            }
        }
        if (c2.o == 5 || c2.o == 7 || c2.o == 8 || c2.o == 9) {
            d.a(0, 0, 240, bB);
        }
        int n15 = 0;
        if (d.a[n].c == 0) {
            n15 = 1;
        }
        n3 = 1 - n15;
        if (n14 < 0) {
            d.a(n3, d.a[n].a, d.a[n].d, sArray, n13, n12, bA, bB, bl);
        } else {
            d.a(n3, d.a[n].a, d.a[n].d, n14, n13, bA, bB);
        }
        d.a(0, 0, 240, 320);
    }

    private static void bl() {
        if (aI == 2) {
            d.bR();
            d.bn();
            d.bS();
            d.aI();
            return;
        }
        d.bm();
        d.bn();
        d.aI();
        d.f(8200, 896, (int)d.a(8200));
    }

    private static void bm() {
        int n = 0;
        ++n;
        d.k[0] = 0;
        ++n;
        d.k[1] = 0;
        d.l(bw, 0, -16640);
        if (bD > 0) {
            ++n;
            d.k[2] = bD;
            ++n;
            d.k[3] = 11725567;
        }
        d.l(bw, 0, -14080);
        if (bD > 0) {
            d.k[n++] = bD;
            d.k[n++] = ci;
        }
        d.l(bw, 0, -280);
        d.k[n++] = bD;
        d.k[n++] = ci;
        d.l(bw, 0, -244);
        d.k[n++] = bD;
        d.k[n++] = 0xFFFFFF;
        d.l(bw, 0, 244);
        d.k[n++] = bD;
        d.k[n++] = cj;
        d.l(bw, 0, 280);
        d.k[n++] = bD;
        d.k[n++] = 0xFFFFFF;
        d.k[n++] = 0x1400000;
        d.k[n++] = ci;
        ao = n >> 1;
        d.J();
        d.l(bw, 0, -1536);
        int n2 = bB;
        if (aJ == 1) {
            d.g(8213, -1536, 0, n2 - 20 - 88);
            d.c(8195, -1536, 0, n2 - 20 - 88, 120);
            d.c(8194, -1536, 0, n2 - 20 - 44, 120);
            d.c(8195, -1536, 0, n2 - 20, 120);
            d.g(8193, -1280, 0, n2);
        } else {
            d.f(8222, -128000, 0);
            d.i(8208, -10496, 0, -5376);
            d.i(8207, -5376, 0, -1536);
            d.g(8192, -1536, 0, n2 - 20);
            d.c(8195, -1536, 0, n2 - 20, 120);
            d.g(8197, -1280, 0, n2);
        }
        d.h(8196, -512, 256, -4608);
        d.l(2496, 0, -768);
        d.a(8212, 0, bA, bB, 1024, 0);
    }

    private static void bn() {
        int n;
        int n2;
        int n3;
        int n4;
        int n5;
        int n6;
        int n7;
        int n8;
        int n9;
        int n10;
        int n11;
        int n12;
        b.setColor(0xFFFFFF);
        d.l(-4096, 0, 0);
        int n13 = bA;
        int n14 = bB;
        d.l(-4156, 0, -60);
        int n15 = 0;
        while (n15 < 4) {
            b.drawLine(n13 + n15, n14, bA + n15, bB);
            ++n15;
        }
        d.l(-4156, 0, 60);
        n15 = 0;
        while (n15 < 4) {
            b.drawLine(n13 + n15, n14, bA + n15, bB);
            ++n15;
        }
        d.l(0, 0, -244);
        n13 = bA;
        n14 = bB;
        d.l(0, 0, 244);
        b.setColor(0xE03030);
        n15 = 1;
        while (n15 < 5) {
            b.drawLine(n13 + n15, n14, bA + n15, bB);
            ++n15;
        }
        b.setColor(0xFFFFFF);
        n15 = 0;
        while (n15 < 8) {
            b.drawLine(n13 - n15, n14, bA - n15, bB);
            ++n15;
        }
        d.b(-4112, -340, 32, 84, 0xFFFFFF);
        d.b(-4080, -340, 1334, 84, 16776502);
        d.b(-2746, -340, 32, 84, 0xFFFFFF);
        d.b(-2714, -340, 1333, 84, 15834373);
        d.b(-1381, -340, 32, 84, 0xFFFFFF);
        d.b(-1349, -340, 1333, 84, 13641520);
        d.b(-4112, 256, 32, 84, 0xFFFFFF);
        d.b(-4080, 256, 1334, 84, 16776502);
        d.b(-2746, 256, 32, 84, 0xFFFFFF);
        d.b(-2714, 256, 1333, 84, 15834373);
        d.b(-1381, 256, 32, 84, 0xFFFFFF);
        d.b(-1349, 256, 1333, 84, 13641520);
        d.l(0, 0, -320);
        d.a(493, bA, bB, 33, 0);
        d.l(0, 0, 320);
        d.a(492, bA, bB, 33, 0);
        if (bw > -1920) {
            d.l(416, 0, -400);
            n12 = bA;
            n11 = bB;
            d.l(5408, 0, -400);
            n10 = bA;
            n9 = bB;
            d.l(5408, 0, 400);
            n8 = bA;
            n7 = bB;
            d.l(416, 0, 400);
            n6 = bA;
            n5 = bB;
            d.b(n11, n12, n10, n5, n6, n8, 16049328, 13742479);
            n4 = 0;
            n3 = 0;
            n2 = d.a(n12, n6, -10, n5 - n11);
            n = d.a(n12, n6, n5 - n11 + 10, n5 - n11);
            int n16 = d.a(n10, n8, -10, n7 - n9);
            int n17 = d.a(n10, n8, n7 - n9 + 10, n7 - n9);
            int n18 = n2 < 120 ? n12 : n2;
            int n19 = n16 < 120 ? n16 : n10;
            int n20 = n17 < 120 ? n8 : n17;
            int n21 = n < 120 ? n : n6;
            b.setColor(0x484848);
            b.fillRect(n18, n11 - 10, n19 - n18, 10);
            b.fillRect(n21, n5, n20 - n21, 10);
            b.setColor(0x323232);
            int n22 = 0;
            while (n22 < 4) {
                int n23 = d.a(0, 10, n22 + 1, 5);
                b.drawLine(n18, n11 - n23, n19, n11 - n23);
                b.drawLine(n21, n5 + n23, n20, n5 + n23);
                ++n22;
            }
            b.setColor(0x484848);
            n22 = -1;
            while (n22 < 7) {
                b.drawLine(n2 - n22, n11 - 10, n - n22, n5 + 10 - 1);
                b.drawLine(n16 + n22, n9 - 10, n17 + n22, n7 + 10 - 1);
                ++n22;
            }
        }
        if (bw >= -1920) {
            d.l(416, 0, -528);
            n12 = bA;
            n11 = bB;
            d.l(4160, 0, -528);
            n10 = bA;
            b.setColor(0xFFFFFF);
            b.fillRect(n12, n11 - 29, n10 - n12, 29);
            n6 = n12;
            n5 = 1;
            while (n5 < 10) {
                n4 = d.a(n12, n10, n5, 9);
                if (n5 > 1) {
                    d.a(5, "" + n5, n6, n11 - 8 - 2, 33);
                }
                d.a(n6 + 1, n11 - 8 - 1, n4 - n6 - 3, 8);
                n3 = n6;
                n2 = 0;
                while (n2 < 10) {
                    b.setColor((n2 & 1) == 0 ? 11670053 : 16518171);
                    n = d.a(n6, n4, n2 + 1, 10);
                    b.fillRect(n3, n11 - 8 - 1, n - n3, 8);
                    n3 = n;
                    ++n2;
                }
                d.a(0, 0, 240, 320);
                n6 = n4;
                ++n5;
            }
        }
        if (ba != 3) {
            n12 = bg - bh;
            n11 = be - bf;
            n10 = ba == 11 ? bf * 416 / 100 : (ba == 12 && n11 != 0 && bb < n12 + n11 ? (bb < n12 ? bf * 416 / 100 : bf * 416 / 100 + d.b(0, n11 * 416 / 100, bb - n12, n11)) : be * 416 / 100);
            d.l(n10, 0, -400);
            n9 = bA;
            n8 = bB;
            d.a(483, n9, n8, 33, 0);
            d.l(n10, 0, 400);
            b.setColor(16098699);
            n7 = -2;
            while (n7 <= 2) {
                b.drawLine(n9 + n7, n8, bA + n7, bB - 1);
                ++n7;
            }
            if (bg != -1) {
                if (ba == 11) {
                    n10 = bh * 416 / 100;
                } else if (ba == 12 && n12 != 0 && bb < n12) {
                    n10 = bh * 416 / 100 + d.b(0, n12 * 416 / 100, bb, n12);
                } else {
                    if (ba == 12 && bh != bg && bb < n12 + 2000 && bb % 200 < 100) {
                        return;
                    }
                    n10 = bg * 416 / 100;
                }
                n9 = n10 > 416 ? 400 : 244;
                d.l(n10, 0, -n9);
                n8 = bA;
                n7 = bB;
                d.l(n10, 0, n9);
                b.setColor(9223887);
                n6 = -2;
                while (n6 <= 2) {
                    b.drawLine(n8 + n6, n7, bA + n6, bB - 1);
                    ++n6;
                }
            }
        }
    }

    private static void bo() {
        d.a(-1, c, 0L, -3889152);
        d.c.o = 1;
        d.c.p = 0;
        while (d.c.c < -2359296) {
            if (d.c.q - d.c.h > 10) {
                d.d(c, d.c.d == 0 ? 102 : 104, false);
            }
            d.b(c, false);
        }
        aX = -1;
        d.v(d.c.q);
    }

    private static void N(int n) {
        switch (n) {
            case 0: {
                d.d(c, d.c.d == 0 ? 102 : 104, true);
                return;
            }
            case 1: {
                d.d(c, 103, true);
                return;
            }
            case 2: {
                d.d(c, 103, true);
            }
        }
    }

    private static void bp() {
        switch (bp & 7) {
            case 0: {
                if (d.c.o == 2 || d.c.o == 1) {
                    if (d.c.q - d.c.h > 10) {
                        d.d(c, d.c.d == 0 ? 102 : 104, true);
                    }
                    d.b(c, false);
                } else if (d.c.o == 3) {
                    d.b(c, false);
                    if (d.c.c + d.c.b >= 0) {
                        d.w(9);
                    }
                }
                return;
            }
            case 1: {
                d.b(c, false);
                if (d.c.j <= 0) {
                    d.w(10);
                }
                return;
            }
            case 2: {
                d.b(c, false);
                if (d.c.o < 7 || G) break;
                d.aF();
            }
        }
    }

    private static int w(int n) {
        if (n == 0) {
            return 0;
        }
        return 187;
    }

    private static void o(int n, int n2, int n3) {
        int n4 = 6 + n2;
        int n5 = 57 + n3;
        d.k(n4, n5);
        if (n == 0) {
            d.a(8196, "1. " + a[37], n4 + 6, n5 + 6, 20);
            int n6 = n5 + 6 + 29 + 14;
            d.a(471, n4 + 228 - 6, n6, 10, 0);
            d.a(470, n4 + 228 - 6 - 34, n6, 10, 0);
            d.a(3, a[43] + ' ', n4 + 228 - 6 - 68, n6 - 11, 24);
            return;
        }
        if (n == 1) {
            d.a(8196, "2. " + a[38], n4 + 6, n5 + 6, 20);
            int n7 = n5 + 6 + 29 + 14;
            d.a(472, n4 + 228 - 6, n7, 10, 0);
            d.a(3, a[44] + ' ', n4 + 228 - 6 - 34, n7 - 11, 24);
            return;
        }
        d.a(8196, "3. " + a[40], n4 + 6, n5 + 6, 20);
        int n8 = n5 + 6 + 29 + 14;
        d.a(472, n4 + 228 - 6, n8, 10, 0);
        d.a(3, a[44] + ' ', n4 + 228 - 6 - 34, n8 - 11, 24);
    }

    private static int x(int n) {
        if (n > 6500) {
            return d.a(600, 1000, n - 6500, 3348);
        }
        return d.a(600, 0, 6500 - n, 4500);
    }

    private static int y(int n) {
        return 9848 - 100 * n - 35 * n * n;
    }

    private static void bq() {
        d = new c();
        d.d.a = new int[10];
        d.d.b = new int[3];
        d = new c[6];
        int n = 0;
        while (n < 6) {
            d.d[n] = new c();
            d.d[n].a = new int[10];
            d.d[n].b = new int[3];
            ++n;
        }
        d.M();
    }

    private static void br() {
        d.d.a = null;
        d.d.b = null;
        d = null;
        int n = 0;
        while (n < 6) {
            d.d[n].a = null;
            d.d[n].b = null;
            d.d[n] = null;
            ++n;
        }
        d = null;
        d.N();
    }

    private static void bs() {
        bc = 10;
        bL = 0;
        bK = 0;
    }

    private static void bt() {
        d.a(aZ, d, (long)(bi * (aZ + 1) + bd * 479111), -4587520);
        d.d.o = 1;
        d.d.p = 0;
        d.p(aZ);
        bM = 0;
        bN = 0;
        bO = 0;
    }

    private static void bu() {
        int n = aN;
        while (n < aP) {
            int n2 = d.d(d[n], n, d.a[n].b);
            d.j(n, bd, n2);
            ++n;
        }
    }

    private static void bv() {
        int n = 0;
        while (n < aP) {
            int n2 = B[n];
            if ((bo & 1 << n2) == 0) {
                d.d(d[n2], n2, n);
                d.d[n2].i = -80 + 160 * (aP - 1 - n) / aP + d.a() % (160 / aP);
            }
            ++n;
        }
        n = 0;
        while (n < aP) {
            d.d[d.B[n]].F = n;
            ++n;
        }
    }

    private static int d(c c2, int n, int[] nArray) {
        int n2;
        d.a(n, c2, (long)(bi * (n + 1) + bd * 479111), -4587520);
        c2.o = 1;
        c2.p = 0;
        d.p(n);
        int n3 = d.b((24 * nArray[12] + d.a() % (2 * nArray[12])) / 25, 1, 100);
        int n4 = d.b((24 * nArray[13] + d.a() % (2 * nArray[13])) / 25, 1, 100);
        int n5 = d.b((24 * nArray[14] + d.a() % (2 * nArray[14])) / 25, 1, 100);
        int n6 = d.b((24 * nArray[15] + d.a() % (2 * nArray[15])) / 25, 1, 100);
        while (c2.o == 2 || c2.o == 1) {
            d.d(n, c2.q, c2.d == 0 ? 102 : 104);
            d.f(c2, c2.d == 0 ? 102 : 104, false);
            n2 = c2.q + 4 + (100 - n3) / 5;
            while (c2.q < n2) {
                d.c(c2, false);
            }
        }
        n2 = -((C.length - 1) * 100 / 32) * c2.b;
        int n7 = (100 - n6) * 300;
        n7 = (d.a() & 1) == 0 ? 65536 - n7 : 65536 + n7;
        int n8 = (n2 += (-100 + n4 + d.a() % n5) * 655360 / 100) - n7 * c2.b / 1920;
        while (c2.c < n8) {
            d.c(c2, false);
        }
        d.d(n, c2.q, 103);
        d.f(c2, 103, false);
        while (c2.c < n2) {
            d.c(c2, false);
        }
        d.d(n, c2.q, 1073741927);
        d.g(c2, 103, false);
        while (c2.o != 7 && c2.o != 8) {
            d.c(c2, false);
        }
        if (c2.a) {
            return -1;
        }
        return c2.z * 100 / 346 >> 8;
    }

    private static void d(c c2, int n, int n2) {
        d.a(n, c2, (long)(bi * (n + 1) + bd * 479111), -4587520);
        c2.o = 1;
        c2.p = 0;
        d.q(n);
        while (c2.c < -524288 - (4 * (aP - 1 - n2) << 16) && c2.o != 5) {
            int n3 = d.f(n, c2.q);
            while (n3 >= 0) {
                if ((n3 & 0x40000000) == 0) {
                    d.f(c2, n3, false);
                } else {
                    d.g(c2, n3 & 0xBFFFFFFF, false);
                }
                n3 = d.f(n, c2.q);
            }
            d.c(c2, false);
        }
    }

    private static void bw() {
        if (ba == 11) {
            d.c(d, true);
            return;
        }
        if (ba == 12) {
            d.c(d, true);
            return;
        }
        if (ba >= 15) {
            int n = 0;
            while (n < aP) {
                if ((bo & 1 << n) == 0) {
                    c c2 = d[n];
                    int n2 = d.f(n, c2.q);
                    while (n2 >= 0) {
                        if ((n2 & 0x40000000) == 0) {
                            d.f(c2, n2, false);
                        } else {
                            d.g(c2, n2 & 0xBFFFFFFF, false);
                        }
                        n2 = d.f(n, c2.q);
                    }
                    d.c(c2, false);
                }
                ++n;
            }
        }
    }

    private static void c(c c2, boolean bl) {
        c2.p += 32;
        ++c2.q;
        if (bl && (c2.o == 4 || c2.o == 5)) {
            ++bM;
        }
        if (c2.o == 1) {
            if (c2.p >= 120 * (y.length - 1)) {
                c2.o = 2;
                c2.p = 0;
                c2.b = 0;
            }
            return;
        }
        if (c2.o == 2) {
            d.b(c2);
            if (c2.c >= -1966080) {
                c2.o = 3;
                c2.p = 0;
            }
            return;
        }
        if (c2.o == 3 || c2.o == 4) {
            d.d(c2);
            if (c2.c > 131072) {
                c2.a = true;
                c2.o = 8;
                c2.p = 0;
                if (bl && ba == 11 && !G) {
                    d.j(aZ, bd, -1);
                    d.aF();
                }
            }
            return;
        }
        if (c2.o == 8) {
            d.d(c2);
            return;
        }
        if (c2.o == 5) {
            if (c2.p < (C.length - 1) * 100) {
                d.d(c2);
            }
            if (c2.p >= (D.length - 1) * 100) {
                c2.o = 6;
                c2.p = 0;
                if (bl && ba == 11) {
                    int n = c2.v >> 11;
                    n = n * n >> 10;
                    d.a(aE, aF, 2, 0, 0, 0, 0, n, -1, -1);
                }
                if (bl && !c2.a && ba != 3) {
                    d.a(20, 128, 1);
                }
            }
            return;
        }
        if (c2.o == 6) {
            c2.r += c2.t;
            c2.s += c2.u;
            c2.v += c2.x;
            c2.x = c2.x * 247 >> 8;
            long l = d.k(c2.v >> 8) << 2;
            long l2 = d.j(c2.v >> 8) << 2;
            long l3 = -l2;
            long l4 = l;
            long l5 = (long)c2.r + l;
            long l6 = (long)c2.s + l2;
            long l7 = (long)c2.r - l;
            long l8 = (long)c2.s - l2;
            long l9 = 0L;
            long l10 = 0L;
            long l11 = 0L;
            long l12 = 0L;
            l10 = 0L - 428L;
            l12 = 0L - 256L;
            long l13 = -c2.t;
            long l14 = -c2.u;
            long l15 = l13 * l3 + l14 * l4 >> 16;
            l9 = 0L + ((l15 >>= 6) * 192L * l3 >> 24);
            l11 = 0L + (l15 * 320L * l3 >> 24);
            if ((l10 += l15 * 192L * l4 >> 24) < 0L && l6 < 0L || (l12 += l15 * 320L * l4 >> 24) < 0L && l8 < 0L) {
                boolean bl2 = false;
                if (l6 < 0L) {
                    bl2 = true;
                }
                boolean bl3 = false;
                if (l8 < 0L) {
                    bl3 = true;
                }
                if (bl2 == bl3) {
                    c2.s = 0;
                } else {
                    c2.r = (int)(l5 + (l7 - l5) * -l6 / (l8 - l6));
                    c2.s = 0;
                }
                int n = c2.z = c2.r;
                if (c2.z < 0) {
                    c2.a = true;
                }
                if (bl) {
                    d.j(aZ, bd, c2.a ? -1 : n * 100 / 88576);
                }
                if (n > 0) {
                    d.d(c2.r << 2, 0, 4, 0, 0);
                }
                c2.o = 7;
                if (ba < 15) {
                    c2.p = 0;
                }
                if (bl && !c2.a) {
                    d.a(26, 128, 1);
                }
            } else {
                long l16 = (l9 * l3 + l10 * l4) * 192L / 512L >> 16;
                long l17 = -((l11 * l3 + l12 * l4) * 320L / 512L) >> 16;
                long l18 = l16 + l17;
                c2.x = (int)((long)c2.x + (l18 << 6));
                c2.t = (int)((long)c2.t + ((l9 -= l18 * l3 * 192L / 512L >> 16) + (l11 += l18 * l3 * 320L / 512L >> 16)));
                c2.u = (int)((long)c2.u + ((l10 -= l18 * l4 * 192L / 512L >> 16) + (l12 += l18 * l4 * 320L / 512L >> 16)));
            }
            if (bl && c2.a && c2.p > 2000 && ba == 11 && !G) {
                d.aF();
            }
            return;
        }
        if (c2.o == 7 && bl && c2.p > 250 && ba == 11 && !G) {
            d.aF();
        }
    }

    private static void O(int n) {
        if (ba == 11 && d.d.o == 6 && d.a(n, 196616)) {
            while (d.d.o != 7) {
                d.c(d, true);
            }
            if (!d.d.a) {
                d.a(26, 128, 1);
            }
        }
        d.d(aZ, d.d.q, n);
        d.f(d, n, true);
    }

    private static void f(c c2, int n, boolean bl) {
        if (c2.o == 2 || c2.o == 1) {
            if (c2.o == 1) {
                c2.o = 2;
                c2.p = 0;
                c2.b = 0;
            }
            if (d.a(n, 131076)) {
                d.a(c2, 0, bl);
            } else if (d.a(n, 131078)) {
                d.a(c2, 1, bl);
            }
            return;
        }
        if (c2.o == 3 && d.a(n, 131083)) {
            c2.o = 4;
            c2.p = 0;
            if (bl && ba != 3) {
                d.P();
                d.a(18, 128, 1);
            }
        }
    }

    private static void P(int n) {
        d.d(aZ, d.d.q, n | 0x40000000);
        d.g(d, n, true);
    }

    private static void g(c c2, int n, boolean bl) {
        if (c2.o == 4 && d.a(n, 131083)) {
            d.d(c2, bl);
            c2.o = 5;
            c2.p = 0;
            if (bl && ba != 3) {
                if (c2.a) {
                    d.a(22, 128, 1);
                    return;
                }
                d.P();
                d.a(19, 128, 1);
            }
        }
    }

    private static void d(c c2, boolean bl) {
        c2.r = c2.c + (C.length - 1) * 100 / 32 * c2.b;
        c2.s = 131072;
        boolean bl2 = false;
        if (c2.r > 0) {
            bl2 = c2.a = true;
        }
        if (bl && c2.a) {
            d.j(aZ, bd, -1);
        }
        int n = c2.p * 60 & 0x1FFFF;
        int n2 = 0x400000 + (n - 65536 << 6);
        c2.A = n;
        c2.v = n2;
        c2.x = 0;
        int n3 = c2.b * 3;
        c2.t = d.k(16384) * n3 >> 14;
        c2.u = d.j(16384) * n3 >> 14;
        c2.B = c2.r;
        c2.C = c2.u;
    }

    private static void Q(int n) {
        d.n(n, 0);
    }

    private static void n(int n, int n2) {
        int n3 = n2 + -512;
        int n4 = -3584;
        if (n > -7680) {
            n = n < -3840 ? (n += d.a(0, 128, n - -7680, 3840)) : (n < 3460 ? (n += 128) : (n < 17300 ? (n += d.a(128, 0, n - 3460, 13840)) : (n < 34600 ? (n += d.a(0, -512, n - 17300, 17300)) : (n -= 512))));
        }
        if (d.d.o == 5) {
            n += d.a(0, 128, d.d.p, (D.length - 1) * 100);
        } else if (d.d.o > 5) {
            n += 128;
        }
        if (d.d.o == 7) {
            n = d.d.a || d.d.p > 300 ? (n += 128) : (n += 128 - d.c(0, 256, 300 - d.d.p, 300));
        }
        if (n > -11520) {
            if (n < -3840) {
                n4 = -3584 - d.b(0, 1280, n - -11520, 7680);
                n3 -= d.b(0, 128, n - -11520, 7680);
            } else {
                n4 -= 1280;
                n3 -= 128;
            }
        }
        d.k(n, n3, 512, n4);
    }

    private static void bx() {
        int n;
        int n2;
        switch (ba) {
            case 3: 
            case 10: 
            case 11: 
            case 12: 
            case 13: {
                if (d.d.o == 1 || d.d.o == 2 || d.d.o == 3 || d.d.o == 4 || d.d.o == 5) {
                    if (d.d.c < 0) {
                        d.Q(d.d.c >> 8);
                        break;
                    }
                    d.Q(0);
                    break;
                }
                if (d.d.o == 6) {
                    if (d.d.a) {
                        d.Q(0);
                        break;
                    }
                    n2 = d.d.r >> 8;
                    n = d.h(0, (d.d.s >> 8) - 512);
                    if (d.d.s > 0x2E0000) {
                        n -= 1024;
                    } else if (d.d.s > 393216) {
                        n -= d.b(0, 1024, (d.d.s >> 8) - 1536, 10240);
                    }
                    d.n(n2, n);
                    break;
                }
                if (d.d.o == 7) {
                    d.n(d.d.r >> 8, d.h(0, (d.d.s >> 8) - 512));
                    break;
                }
                if (d.d.o != 8) break;
                d.Q(0);
                break;
            }
            case 15: {
                if (bb < 0) {
                    d.Q(-2166);
                    break;
                }
                if (bb < 2000) {
                    d.Q(d.b(-2422, -1384, bb, 2000) + 256);
                    break;
                }
                d.Q(-1128);
                break;
            }
            default: {
                d.Q(-1128);
            }
        }
        d.by();
        if (ba == 10 || ba == 11 || ba == 12 || ba == 13 || ba == 3) {
            d.l(0, 0, -1280);
            int n3 = 0;
            if (d.d.a && (d.d.o == 7 || d.d.o == 8 || d.d.o == 6 && bb > 1000)) {
                n3 = 1;
            }
            n2 = n3;
            d.a(aI == 2 ? 16388 : 16385, n2, bA, bB, 1024, 0);
            d.a(483, bK, bL, 33, 0);
            d.e(d, aZ, 0);
            if (ba == 11 || ba == 3) {
                if (d.d.o == 2 || d.d.o == 1) {
                    d.a(d, bA, bB);
                } else if (d.d.o == 4) {
                    if (d.d.p < 200) {
                        d.i(d.d.p, bA - 14, bB - 124);
                    }
                } else if (d.d.o == 3 && e) {
                    d.l(bA - 14, bB - 124);
                }
            }
            return;
        }
        if (ba >= 15) {
            n2 = 0;
            while (n2 < aP) {
                n = B[aP - 1 - n2];
                if ((bo & 1 << n) == 0) {
                    d.e(d[n], n, aP - 1 - n2);
                }
                ++n2;
            }
        }
    }

    private static void e(c c2, int n, int n2) {
        int n3;
        int n4;
        int n5 = c2.c >> 8;
        if (ba == 15 && (c2.o == 6 && c2.p > 3000 || c2.o == 7)) {
            d.l(c2.c >> 8, 0, c2.i);
            n4 = d.l(n, bd);
            if (n4 != -1) {
                int n6 = 0;
                if (n2 == 0) {
                    n6 = 1;
                }
                int n7 = n6 + 5;
                int n8 = n2 == 0 ? 25 : 16;
                boolean bl = false;
                if (n2 == 0) {
                    bl = true;
                }
                String string = d.a(n4, bc, bl);
                int n9 = bB - (256 * d.a[n].d >> 8) - 4;
                boolean bl2 = false;
                if (n >= aN) {
                    bl2 = true;
                }
                d.a(n7, n8, string, bA, n9, 1, bl2);
            } else {
                int n10 = bB - (256 * d.a[n].d >> 8) - 4;
                boolean bl = false;
                if (n >= aN) {
                    bl = true;
                }
                d.a(3, 20, "FOUL", bA, n10, 1, bl);
            }
        }
        d.l(n5, 0, c2.i);
        n4 = c2.c / 81920 & 3;
        d.a(518 + n4, bA - 9, bB, 3, 0);
        d.l(n5, 0, c2.i);
        short[] sArray = x;
        int n11 = n5;
        int n12 = 160;
        boolean bl = true;
        int n13 = -1;
        if (c2.o == 1) {
            sArray = y;
            n11 = c2.p;
            n12 = 120;
            bl = false;
        } else if (c2.o == 2) {
            if (n5 - -17920 < 160 * (z.length - 1)) {
                sArray = z;
                n11 = n5 - -17920;
                bl = false;
            }
        } else if (c2.o == 4) {
            n11 = c2.b * c2.p / 32 >> 8;
            n12 = 200;
            if (n11 > 200 * (A.length - 1)) {
                n11 -= 200 * (A.length - 1);
                sArray = B;
            } else {
                sArray = A;
                bl = false;
            }
        } else if (c2.o == 5) {
            n11 = c2.p;
            n12 = 100;
            bl = false;
            sArray = D;
        } else if (c2.o == 6 || ba >= 15 && c2.o == 7) {
            n11 = c2.p;
            n12 = 100;
            bl = false;
            sArray = E;
            if (ba >= 15 && n11 > 100 * (sArray.length - 1) + 1000) {
                n11 -= 100 * (sArray.length - 1) + 1000;
                n12 = 120;
                n3 = d.l(n, bd);
                n13 = aP > 1 ? d.n(c2.F, n3) : d.n(n3);
            }
        } else if (c2.o == 7) {
            n11 = c2.p;
            n12 = 120;
            n3 = d.l(n, bd);
            n13 = ba >= 15 && aP > 1 ? d.n(c2.F, n3) : d.n(n3);
        }
        boolean bl3 = false;
        if (c2.o != 6 && c2.o != 7) {
            bl3 = true;
        }
        D = bl3;
        b = c2;
        int n14 = 0;
        if (d.a[n].c == 0) {
            n14 = 1;
        }
        n3 = 1 - n14;
        if (n13 < 0) {
            d.a(n3, d.a[n].a, d.a[n].d, sArray, n11, n12, bA, bB, bl);
        } else {
            d.a(n3, d.a[n].a, d.a[n].d, n13, n11, bA, bB);
        }
        if (c2.o == 6 || c2.o == 7) {
            int n15 = c2.v >> 8;
            d.l(c2.r >> 8, c2.s >> 8, 0);
            int n16 = bA;
            int n17 = bB;
            if (c2.o == 6) {
                d.l(c2.r >> 8, 0, 0);
                d.a(522, bA, bB, 33, 0);
                int n18 = c2.r + (d.k(n15) << 4) >> 8;
                int n19 = c2.s + (d.j(n15) << 4) >> 8;
                d.l(n18, n19, 0);
                d.a(c2, n16, n17, bA - n16, bB - n17, false);
            } else {
                int n20 = c2.r + (d.k(n15) << 2) >> 8;
                int n21 = c2.s + (d.j(n15) << 2) >> 8;
                d.l(n20, 0, 0);
                d.a(0, 0, 240, bB);
                d.l(n20, n21, 0);
                d.a(522, n16, n17, 40, 0);
                d.a(c2, n16, n17, bA - n16, bB - n17, true);
            }
            d.a(0, 0, 240, 320);
        }
    }

    private static void a(c c2, int n, int n2, int n3, int n4, boolean bl) {
        int n5;
        int n6;
        int n7;
        int n8;
        int n9;
        int n10;
        if (n < -64 || n > 304 || n2 < -64 || n2 > 384) {
            return;
        }
        int n11 = -64;
        int n12 = -5;
        int n13 = 5;
        int n14 = 40;
        if (bl) {
            boolean bl2 = false;
            n11 = -106;
            n12 = -47;
            n13 = -37;
            n14 = -2;
        }
        long l = (long)n << 8;
        long l2 = (long)n2 << 8;
        long l3 = d.a((long)(n3 * n3 + n4 * n4) << 8) << 4;
        long l4 = ((long)n3 << 16) / l3;
        long l5 = ((long)n4 << 16) / l3;
        int n15 = (int)((long)2 * l + (long)(2 * n11) * l4 + 256L >> 9);
        int n16 = (int)((long)2 * l2 + (long)(2 * n11) * l5 + 256L >> 9);
        int n17 = (int)((long)2 * l + (long)114 * l4 + 256L >> 9);
        int n18 = (int)((long)2 * l2 + (long)114 * l5 + 256L >> 9);
        int[] nArray = new int[9];
        nArray[0] = 15128068;
        nArray[1] = 5862914;
        nArray[2] = 0x2E0E00;
        nArray[3] = 2780928;
        nArray[4] = 3025408;
        nArray[6] = 0xFCEFEF;
        nArray[7] = 11499374;
        nArray[8] = 5451825;
        int[] nArray2 = nArray;
        int n19 = d.i(n17 - n15, n18 - n16);
        short s = h[n19 + 128 & 0x1FF];
        short s2 = h[n19 & 0x1FF];
        int n20 = d.a(n15, n17, n12 - n11, 57 - n11);
        int n21 = d.a(n16, n18, n12 - n11, 57 - n11);
        int n22 = d.a(n15, n17, n13 - n11, 57 - n11);
        int n23 = d.a(n16, n18, n13 - n11, 57 - n11);
        int n24 = d.a(n15, n17, n14 - n11, 57 - n11);
        int n25 = d.a(n16, n18, n14 - n11, 57 - n11);
        int n26 = -2 * s >> 14;
        int n27 = -2 * s2 >> 14;
        int n28 = 3 * s >> 14;
        int n29 = 3 * s2 >> 14;
        boolean bl3 = false;
        if (d.i(n27 - n29) > d.i(n26 - n28)) {
            bl3 = true;
        }
        boolean bl4 = bl3;
        boolean bl5 = false;
        if (bl4) {
            n10 = n26;
            n26 = n27;
            n27 = n10;
            n10 = n28;
            n28 = n29;
            n29 = n10;
        }
        if (n26 > n28) {
            n10 = n26;
            n26 = n28;
            n28 = n10;
            n10 = n27;
            n27 = n29;
            n29 = n10;
            bl5 = true;
        }
        n10 = n28 - n26;
        int n30 = d.i(n29 - n27);
        int n31 = -(n10 + 1) / 2;
        int n32 = n27;
        int n33 = n27 < n29 ? 1 : -1;
        int n34 = 1;
        if (!bl5) {
            n9 = n26;
            n26 = n28;
            n28 = n9;
            n34 = -1;
            n33 = -n33;
        }
        n9 = 0;
        int n35 = n26;
        while (n35 != n28) {
            n8 = bl4 ? n32 : n35;
            n7 = bl4 ? n35 : n32;
            n6 = n9 ^ 1;
            if (n6 != 0) {
                if ((n31 += n30) >= 0) {
                    n9 = 1;
                    n31 -= n10;
                }
            } else {
                n32 += n33;
                n9 = 0;
            }
            n5 = 1;
            if (n35 == n26) {
                n5 = 2;
            } else if (n35 + n34 == n28) {
                n5 = 0;
            }
            b.setColor(nArray2[n5]);
            b.drawLine(n15, n16, n20 + n8, n21 + n7);
            b.setColor(nArray2[3 + n5]);
            b.drawLine(n20 + n8, n21 + n7, n22 + n8, n23 + n7);
            b.setColor(nArray2[n5]);
            b.drawLine(n22 + n8, n23 + n7, n24 + n8, n25 + n7);
            b.setColor(nArray2[6 + n5]);
            b.drawLine(n24 + n8, n25 + n7, n17, n18);
            if (n6 == 0) continue;
            n35 += n34;
        }
        if (ba < 15 && (c2.o == 4 || c2.o == 5)) {
            int n36;
            n35 = c2.o == 4 ? c2.p * 60 & 0x1FFFF : c2.A;
            if (n35 > 65536) {
                n35 = 131072 - n35;
            }
            n35 = -64 + (121 * n35 >> 16);
            n8 = (int)((long)2 * l + (long)(2 * n35) * l4 + 256L);
            n7 = (int)((long)2 * l2 + (long)(2 * n35) * l5 + 256L);
            n6 = n8;
            n5 = n7;
            int n37 = bM - bN;
            if (n37 > 0) {
                n36 = (int)((long)n6 - ((long)2 * l + (long)(2 * bO) * l4 + 256L)) / n37;
                int n38 = (int)((long)n5 - ((long)2 * l2 + (long)(2 * bO) * l5 + 256L)) / n37;
                aE = n6 << 1;
                aF = n5 << 1;
                while (n37 > 0) {
                    d.d(n6 << 1, n5 << 1, 0, 0, 0);
                    n6 -= n36;
                    n5 -= n38;
                    --n37;
                }
                bN = bM;
                bO = n35;
            }
            if ((n36 = ch >> 7 & 3) == 1) {
                n36 = 1;
            } else if (n36 == 2) {
                n36 = 2;
            } else if (n36 == 3) {
                n36 = 3;
            }
            d.a(469, n8 >> 9, n7 >> 9, 3, n36);
        }
    }

    private static void by() {
        if (aI == 2) {
            d.bR();
            d.bA();
            d.bS();
            return;
        }
        d.bz();
        d.bA();
        d.bB();
    }

    private static void bz() {
        int n;
        int n2 = 0;
        ++n2;
        d.k[0] = 0;
        ++n2;
        d.k[1] = 0;
        d.l(bw, 0, -19712);
        if (bD > 0) {
            ++n2;
            d.k[2] = bD;
            ++n2;
            d.k[3] = 11725567;
        }
        d.l(bw, 0, -10752);
        if (bD > 0) {
            d.k[n2++] = bD;
            d.k[n2++] = 8104530;
        }
        d.l(bw, 0, -10740);
        if (bD > 0) {
            d.k[n2++] = bD;
            d.k[n2++] = 0xFFFFFF;
        }
        int n3 = 0;
        while (n3 < 6) {
            d.l(bw, 0, -10752 + (n3 + 1) * 512 - 12);
            if (bD > 0) {
                d.k[n2++] = bD;
                d.k[n2++] = ci;
            }
            d.l(bw, 0, -10752 + (n3 + 1) * 512 + 12);
            if (bD > 0) {
                d.k[n2++] = bD;
                d.k[n2++] = 0xFFFFFF;
            }
            ++n3;
        }
        n3 = 0;
        while (n3 < 6) {
            n = -5120 + n3 * 5120 * 2 / 5;
            d.l(bw, 0, n);
            if (bD > 0) {
                int[] nArray = new int[]{10143323, 12374893, 10143323, 11914837, 9814862, 11783245};
                d.k[n2++] = bD;
                d.k[n2++] = nArray[n3];
            }
            if (bB >= 320) break;
            ++n3;
        }
        d.k[n2++] = 0x1400000;
        d.k[n2++] = 0x3F3F3F;
        ao = n2 >> 1;
        d.J();
        d.l(173, 0, -7680);
        n3 = bA;
        int n4 = bB;
        d.l(173, 0, 0);
        n = bA;
        int n5 = bB;
        n = n3 + (n - n3) * (319 - n4) / (n5 - n4);
        d.l(173, 0, -512);
        int n6 = bA;
        int n7 = bB;
        d.l(173, 0, 512);
        int n8 = bA;
        int n9 = bB;
        d.e(n3, n4, n6, n7, ci);
        d.e(n6, n7, n8, n9, cj);
        d.e(n8, n9, n, 319, ci);
    }

    private static void e(int n, int n2, int n3, int n4, int n5) {
        b.setColor(n5);
        if (n < 0) {
            return;
        }
        int n6 = n4 - n2 + 1;
        if (n >= 239) {
            b.fillRect(0, n2, 240, n6);
            return;
        }
        int n7 = d.g(n, n3);
        if (n7 > 0) {
            b.fillRect(0, n2, n7, n6);
        }
        if (n > n3) {
            DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n3, n2, n, n2, n3, n4, b.getColor() & 0xFFFFFF | 0xFF000000);
            return;
        }
        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n, n4, n3, n4, n, n2, b.getColor() & 0xFFFFFF | 0xFF000000);
    }

    private static void bA() {
        int n;
        int n2;
        int n3;
        int n4;
        int n5;
        int n6;
        int n7;
        int n8;
        block21: {
            block23: {
                block24: {
                    block22: {
                        b.setColor(0xFFFFFF);
                        d.l(-7680, 0, 0);
                        if (bA < 360 && bA > -120) {
                            n8 = bA;
                            n7 = bB;
                            d.l(-7740, 0, -60);
                            n6 = 0;
                            while (n6 < 4) {
                                b.drawLine(n8 + n6, n7, bA + n6, bB);
                                ++n6;
                            }
                            d.l(-7740, 0, 60);
                            n6 = 0;
                            while (n6 < 4) {
                                b.drawLine(n8 + n6, n7, bA + n6, bB);
                                ++n6;
                            }
                        }
                        d.l(0, 0, -512);
                        n8 = bA;
                        n7 = bB;
                        if (bA >= 0) {
                            d.c(bA, bB, 0, bB);
                        }
                        d.l(0, 0, 512);
                        n6 = bA;
                        n5 = bB;
                        if (bA >= 0) {
                            d.c(bA, bB, 0, bB);
                        }
                        n4 = 0;
                        while (n4 < 6) {
                            b.drawLine(n8 - n4, n7, n6 - n4, n5);
                            ++n4;
                        }
                        n4 = -d.k(4096) * 2 * 256 / d.j(4096);
                        if (ba == 3) break block21;
                        n3 = bg - bh;
                        n2 = be - bf;
                        if (ba == 11) {
                            n = (bf << 8) / 100;
                        } else if (ba == 12 && n2 != 0 && bb < n3 + n2) {
                            if (bb < n3) {
                                n = (bf << 8) / 100;
                            } else {
                                if (bf < be - 1000) {
                                    bf = be - 1000;
                                }
                                n = (bf << 8) / 100 + d.b(0, (n2 << 8) / 100, bb - n3, n2);
                            }
                        } else {
                            n = (be << 8) / 100;
                        }
                        b.setColor(0xE15151);
                        d.c(n, n4, true);
                        if (bg == -1) break block21;
                        if (ba != 11) break block22;
                        n = (bh << 8) / 100;
                        break block23;
                    }
                    if (ba != 12 || n3 == 0 || bb >= n3) break block24;
                    if (bh < bg - 1000) {
                        bh = bg - 1000;
                    }
                    n = (bh << 8) / 100 + d.b(0, (n3 << 8) / 100, bb, n3);
                    break block23;
                }
                if (ba == 12 && bh != bg && bb < n3 + 2000 && bb % 200 < 100) break block21;
                n = (bg << 8) / 100;
            }
            b.setColor(5346785);
            d.o(n, n4);
        }
        b.setColor(0xFFFFFF);
        n3 = 10;
        while (n3 <= 120 && n8 < 240) {
            n2 = ((((n3 << 8) - n4) * d.k(-4096) >> 14) + n4) * 346 >> 8;
            n = ((n3 << 8) - n4) * d.j(-4096) >> 14;
            d.l(n2, 0, n);
            if (bA >= 0) {
                d.c(n8, n7, bA, bB);
            }
            n8 = bA;
            n7 = bB;
            if (n5 < 320) {
                d.l(n2, 0, -n);
                d.c(n6, n5, bA, bB);
                n6 = bA;
                n5 = bB;
            }
            if (n3 >= 30 && n8 > -60) {
                b.setColor(0xFFFFFF);
                d.o(n3 << 8, n4);
            }
            n3 += 10;
        }
        d.b(-7696, -640, 32, 128, 0xFFFFFF);
        d.b(-7664, -640, 2528, 128, 16776502);
        d.b(-5136, -640, 32, 128, 0xFFFFFF);
        d.b(-5104, -640, 2528, 128, 15834373);
        d.b(-2576, -640, 32, 128, 0xFFFFFF);
        d.b(-2544, -640, 2528, 128, 13641520);
        d.b(-7696, 544, 32, 128, 0xFFFFFF);
        d.b(-7664, 544, 2528, 128, 16776502);
        d.b(-5136, 544, 32, 128, 0xFFFFFF);
        d.b(-5104, 544, 2528, 128, 15834373);
        d.b(-2576, 544, 32, 128, 0xFFFFFF);
        d.b(-2544, 544, 2528, 128, 13641520);
        d.l(0, 0, -512);
        d.a(493, bA, bB, 33, 0);
        d.l(0, 0, 512);
        d.a(492, bA, bB, 33, 0);
    }

    private static void bB() {
        d.i(8209, -128000, 0, -19712);
        d.i(8208, -19712, 0, -14592);
        d.i(8207, -14592, 0, -10752);
        if (aJ == 1) {
            d.f(8203, -10752, 0, 120);
            d.g(8205, -9728, 0, bB);
        } else {
            d.f(8204, -10752, 0, 120);
            d.g(8206, -9728, 0, bB);
        }
        int n = -d.k(4096) * 2 * 256 / d.j(4096);
        int n2 = 50;
        while (n2 <= 100) {
            int n3 = ((((n2 << 8) - n) * d.k(-4096) >> 14) + n) * 346 >> 8;
            int n4 = ((n2 << 8) - n) * d.j(-4096) >> 14;
            d.l(n3, 0, n4);
            int n5 = 494 + (n2 - 50) / 10;
            int n6 = d.b(n5);
            d.a(n5, bA, bB + n6 / 4, 33, 0);
            n2 += 10;
        }
        d.l(-1536, 0, -2048);
        d.a(690, bA, bB, 33, 0);
        d.h(8198, -2048, 384, -7936);
        d.h(8196, -768, 256, -8448);
        d.aI();
        d.h(8200, 1152, 256, -4608);
    }

    private static void o(int n, int n2) {
        d.c(n, n2, false);
    }

    private static void c(int n, int n2, boolean bl) {
        int n3 = 3 + n / 2560;
        int n4 = 8192 / n3;
        int n5 = -4096;
        int n6 = (((n - n2) * d.k(-4096) >> 14) + n2) * 346 >> 8;
        int n7 = (n - n2) * d.j(-4096) >> 14;
        d.l(n6, 0, n7);
        int n8 = bA;
        int n9 = bB;
        if (bl) {
            bK = n8;
            bL = n9;
        }
        int n10 = 0;
        while (n10 < n3 && n9 < 320) {
            d.l((((n - n2) * d.k(n5 += n4) >> 14) + n2) * 346 >> 8, 0, (n - n2) * d.j(n5) >> 14);
            d.c(n8, n9, bA, bB);
            n8 = bA;
            n9 = bB;
            ++n10;
        }
    }

    private static void bC() {
        d.a(-1, d, 0L, -4587520);
        d.d.o = 1;
        d.d.p = 0;
        while (d.d.c < -3276800) {
            if (d.d.q - d.d.h > 10) {
                d.f(d, d.d.d == 0 ? 102 : 104, false);
            }
            d.c(d, false);
        }
        aX = -1;
        d.v(d.d.q);
    }

    private static void R(int n) {
        switch (n) {
            case 0: {
                d.f(d, d.d.d == 0 ? 102 : 104, true);
                return;
            }
            case 1: {
                d.f(d, 103, true);
                return;
            }
            case 2: {
                d.g(d, 103, true);
            }
        }
    }

    private static void bD() {
        switch (bp & 7) {
            case 0: {
                if (d.d.o == 2 || d.d.o == 1) {
                    if (d.d.q - d.d.h > 10) {
                        d.f(d, d.d.d == 0 ? 102 : 104, true);
                    }
                    d.c(d, false);
                } else if (d.d.o == 3) {
                    d.c(d, false);
                    if (d.d.c >= -1179648) {
                        d.w(9);
                    }
                }
                return;
            }
            case 1: {
                d.c(d, false);
                if (d.d.p * 60 >= 66000) {
                    d.w(10);
                }
                return;
            }
            case 2: {
                d.c(d, false);
                if (d.d.o != 7 && (d.d.o != 6 || d.d.p <= 200) || G) break;
                d.aF();
            }
        }
    }

    private static int z(int n) {
        if (n == 0) {
            return 0;
        }
        return 187;
    }

    private static void p(int n, int n2, int n3) {
        int n4 = 6 + n2;
        int n5 = 57 + n3;
        d.k(n4, n5);
        if (n == 0) {
            d.a(8196, "1. " + a[37], n4 + 6, n5 + 6, 20);
            int n6 = n5 + 6 + 29 + 14;
            d.a(471, n4 + 228 - 6, n6, 10, 0);
            d.a(470, n4 + 228 - 6 - 34, n6, 10, 0);
            d.a(3, a[43] + ' ', n4 + 228 - 6 - 68, n6 - 11, 24);
            return;
        }
        if (n == 1) {
            d.a(8196, "2. " + a[41], n4 + 6, n5 + 6, 20);
            int n7 = n5 + 6 + 29 + 14;
            d.a(472, n4 + 228 - 6, n7, 10, 0);
            d.a(485, n4 + 228 - 6 - 17, n7 - 17 + 6 - 3 + ch * 256 / 12000 % 9, 33, 0);
            d.a(3, a[45] + ' ', n4 + 228 - 6 - 34, n7 - 11, 24);
            return;
        }
        d.a(8196, "3. " + a[42], n4 + 6, n5 + 6, 20);
        int n8 = n5 + 6 + 29 + 14;
        d.a(472, n4 + 228 - 6, n8, 10, 0);
        d.a(484, n4 + 228 - 6 - 17, n8 - 17 + 6 + 3 - ch * 256 / 12000 % 9, 33, 0);
        d.a(3, a[46] + ' ', n4 + 228 - 6 - 34, n8 - 11, 24);
    }

    private static int A(int n) {
        if (n > 500) {
            return d.a(600, 1000, n - 500, 114);
        }
        return d.a(600, 0, 500 - n, 300);
    }

    private static int B(int n) {
        return 614 - 12 * n - n * n;
    }

    private static void bE() {
        e = new c();
        d.e.a = new int[10];
        d.e.b = new int[3];
        e = new c[6];
        int n = 0;
        while (n < 6) {
            d.e[n] = new c();
            d.e[n].a = new int[10];
            d.e[n].b = new int[3];
            ++n;
        }
        d.M();
    }

    private static void bF() {
        d.e.a = null;
        d.e.b = null;
        e = null;
        int n = 0;
        while (n < 6) {
            d.e[n].a = null;
            d.e[n].b = null;
            d.e[n] = null;
            ++n;
        }
        e = null;
        d.N();
    }

    private static void bG() {
        bc = 10;
    }

    private static void bH() {
        d.a(aZ, e, (long)(bi * (aZ + 1) + bd * 982365), -3932160);
        d.h(e);
        d.p(aZ);
        bP = 0;
        bQ = -1;
        bT = 0;
        bU = 0;
    }

    private static void h(c c2) {
        c2.o = 0;
        c2.p = 0;
        c2.a = false;
        c2.b = false;
        c2.k = 0;
        c2.j = 0;
        c2.r = c2.c;
        c2.s = c2.k + 51200;
        c2.v = c2.r + h[55] * 18;
        c2.C = c2.w = c2.s + h[183] * 18;
        c2.B = -5120 + (c2.v - c2.r >> 8);
        c2.t = 0;
        c2.u = 0;
        c2.x = 0;
        c2.y = 0;
        c2.z = 655360000;
    }

    private static void bI() {
        int n = aN;
        while (n < aP) {
            int n2 = d.e(e[n], n, d.a[n].b);
            d.j(n, bd, n2);
            ++n;
        }
    }

    private static void bJ() {
        int n = 0;
        while (n < aP) {
            int n2 = B[n];
            if ((bo & 1 << n2) == 0) {
                d.f(e[n2], n2, n);
                d.e[n2].i = -80 + 160 * (aP - 1 - n) / aP + d.a() % (160 / aP);
            }
            ++n;
        }
        n = 0;
        while (n < aP) {
            d.e[d.B[n]].F = n;
            ++n;
        }
    }

    private static int e(c c2, int n, int[] nArray) {
        int n2;
        d.a(n, c2, (long)(bi * (n + 1) + bd * 982365), -3932160);
        d.h(c2);
        d.p(n);
        int n3 = d.b((24 * nArray[16] + d.a() % (2 * nArray[16])) / 25, 1, 100);
        int n4 = d.b((24 * nArray[17] + d.a() % (2 * nArray[17])) / 25, 1, 100);
        int n5 = d.b((24 * nArray[18] + d.a() % (2 * nArray[18])) / 25, 1, 100);
        while (c2.o == 1 || c2.o == 0) {
            d.d(n, c2.q, c2.d == 0 ? 102 : 104);
            d.h(c2, c2.d == 0 ? 102 : 104, false);
            n2 = c2.q + 4 + (100 - n3) / 5;
            while (c2.q < n2) {
                d.e(c2, false);
            }
        }
        n2 = (-100 + n4) * 5000;
        while (c2.v + c2.x < n2) {
            d.e(c2, false);
        }
        d.d(n, c2.q, 103);
        d.h(c2, 103, false);
        int n6 = 4800 - (100 - n5) * 12;
        while (c2.o != 7 && (c2.o < 4 || c2.o == 4 && c2.D < n6)) {
            d.e(c2, false);
        }
        d.d(n, c2.q, 1073741927);
        d.i(c2, 103, false);
        while (c2.o != 7) {
            d.e(c2, false);
        }
        if (c2.z > 0) {
            return c2.z / 855;
        }
        return -1;
    }

    private static void f(c c2, int n, int n2) {
        d.a(n, c2, (long)(bi * (n + 1) + bd * 982365), -3932160);
        d.h(c2);
        d.q(n);
        while (c2.c < -524288 - (4 * (aP - 1 - n2) << 16) && c2.o != 7) {
            int n3 = d.f(n, c2.q);
            while (n3 >= 0) {
                if ((n3 & 0x40000000) == 0) {
                    d.h(c2, n3, false);
                } else {
                    d.i(c2, n3 & 0xBFFFFFFF, false);
                }
                n3 = d.f(n, c2.q);
            }
            d.e(c2, false);
        }
    }

    private static void bK() {
        if (ba == 11) {
            d.e(e, true);
            return;
        }
        if (ba == 12 || ba == 13) {
            d.e(e, true);
            return;
        }
        if (ba >= 15) {
            int n = 0;
            while (n < aP) {
                if ((bo & 1 << n) == 0) {
                    c c2 = e[n];
                    int n2 = d.f(n, c2.q);
                    while (n2 >= 0) {
                        if ((n2 & 0x40000000) == 0) {
                            d.h(c2, n2, false);
                        } else {
                            d.i(c2, n2 & 0xBFFFFFFF, false);
                        }
                        n2 = d.f(n, c2.q);
                    }
                    d.e(c2, false);
                }
                ++n;
            }
        }
    }

    private static void S(int n) {
        d.d(aZ, d.e.q, n);
        d.h(e, n, true);
    }

    private static void h(c c2, int n, boolean bl) {
        if (c2.o == 1) {
            if (d.a(n, 131076)) {
                d.a(c2, 0, bl);
            } else if (d.a(n, 131078)) {
                d.a(c2, 1, bl);
            }
            return;
        }
        if (c2.o == 2) {
            if (!c2.b && d.a(n, 131083)) {
                int n2 = (c2.v >> 4) + 1500;
                if (n2 > 0) {
                    n2 = 0;
                }
                c2.E += n2;
                if (c2.E < 0) {
                    c2.E = 0;
                }
                if (c2.b < 4000) {
                    c2.b = 4000;
                }
                c2.t = c2.b;
                c2.j += 4000;
                c2.u = c2.j;
                c2.b = true;
                if (bl) {
                    aX = ch << 2;
                    d.P();
                    if (ba != 3) {
                        d.a(18, 128, 1);
                    }
                }
            }
            return;
        }
        if ((c2.o == 3 || c2.o == 4) && !c2.b && d.a(n, 131083)) {
            c2.t += 3000;
            if (c2.t < 12500) {
                c2.t = 12500;
            }
            c2.u += 5000;
            c2.b = true;
            if (bl) {
                d.P();
                if (ba != 3) {
                    d.a(18, 128, 1);
                }
            }
        }
    }

    private static void T(int n) {
        d.d(aZ, d.e.q, n | 0x40000000);
        d.i(e, n, true);
    }

    private static void i(c c2, int n, boolean bl) {
        if ((c2.o == 3 || c2.o == 4 || c2.o == 2) && d.a(n, 131083)) {
            if (c2.o == 2) {
                c2.j = 4000;
                c2.b >>= 1;
                c2.t >>= 1;
                c2.x >>= 1;
            } else {
                int n2 = (128000 - c2.c) * 10 >> 8;
                if (c2.b < n2) {
                    c2.b = n2;
                }
                c2.j += (c2.E - 2700) * 5 / 4;
                if (c2.j < 0) {
                    c2.j = 0;
                }
                c2.t = -5000;
                if (bl) {
                    d.P();
                    if (ba != 3) {
                        d.a(20, 128, 1);
                    }
                    d.a(aE, aF, 6, 0, 0, 0, 0, 1000, -1, -1);
                }
            }
            c2.o = 5;
            c2.p = 0;
        }
    }

    private static void e(c c2, boolean bl) {
        int n;
        c2.p += 32;
        ++c2.q;
        if (c2.o >= 3 && c2.c >= 118000 && c2.c <= 138000 && c2.z > c2.k) {
            c2.z = c2.k;
        }
        if (c2.o == 2) {
            if (bP < 110592) {
                bP += (n = 110592 - bP >> 3) < 1200 ? 1200 : n;
            }
        } else if (bP > 0) {
            bP -= 6000;
        }
        switch (c2.o) {
            case 0: {
                if (c2.p >= 160 * (L.length - 1)) {
                    c2.o = 1;
                    c2.p = 0;
                    c2.b = 0;
                }
                return;
            }
            case 1: {
                d.b(c2);
                c2.r += c2.t;
                c2.s += c2.u;
                c2.x = c2.b;
                c2.y = c2.j;
                d.j(c2);
                c2.v += c2.x;
                c2.w += c2.y;
                c2.t = c2.b;
                c2.u = c2.j;
                if (c2.c > -1310720) {
                    c2.o = 2;
                    c2.E = c2.b - 16000;
                }
                return;
            }
            case 2: {
                if (c2.v >= 0) {
                    c2.t = c2.b;
                    c2.x = 0;
                    c2.v = 0;
                    c2.w = 0;
                    c2.k = c2.s;
                    c2.p = 0;
                    c2.o = 3;
                } else {
                    c2.c += c2.b;
                    c2.k += c2.j;
                    c2.r += c2.t;
                    c2.s += c2.u;
                    c2.j -= 700;
                    if (c2.b) {
                        if (c2.k < 0) {
                            c2.k = 0;
                        }
                    } else if (c2.k + c2.j < 0) {
                        c2.j -= c2.k + c2.j;
                    }
                    n = c2.b ? 50 : 50 - 50 * c2.r / -1310720;
                    c2.s = c2.k + (50 + n * 6 / 4 << 10);
                    c2.x = c2.b;
                    c2.y = c2.b ? (c2.y -= 2100) : c2.C * (c2.v >> 8) / c2.B - c2.w;
                    d.j(c2);
                    if (c2.w + c2.y < 0) {
                        c2.y -= c2.w + c2.y;
                    }
                    c2.v += c2.x;
                    c2.w += c2.y;
                    bQ = 16384 - (c2.w >> 4);
                    if (bQ < 0) {
                        bQ = 0;
                    }
                    ++bT;
                    c2.t = c2.b;
                    c2.u = c2.j;
                }
                return;
            }
            case 3: 
            case 4: {
                c2.r += c2.t;
                c2.s += c2.u;
                c2.u -= 700;
                if (c2.s + c2.u < 51200) {
                    if (c2.o == 4) {
                        c2.o = 5;
                        c2.p = 0;
                        return;
                    }
                    c2.u = 51200 - c2.s;
                }
                if (c2.b && c2.s < 51200 && c2.u < 0) {
                    c2.u += 51200 - c2.s;
                    c2.o = 6;
                    c2.j = c2.u;
                    c2.A = 51200;
                    c2.p = 0;
                    return;
                }
                bQ = (c2.v - c2.r >> 4) - 2000;
                if (bQ > 16384) {
                    bQ = 16384;
                }
                ++bT;
                if (c2.b) {
                    c2.t = c2.p < 1000 && c2.s > 120000 ? (c2.t += (-c2.t * 50 >> 8) + (c2.v - c2.r >> 6)) : (c2.t -= 300);
                } else if (c2.t > 0) {
                    c2.t -= c2.t * 50 >> 8;
                }
                n = c2.D;
                d.i(c2);
                if (c2.p > 3000 || c2.o == 3 && n - c2.D < 0) {
                    c2.o = 4;
                }
                if (c2.o == 4) {
                    c2.c = c2.r;
                    c2.k += c2.j;
                    c2.b = c2.t;
                    int n2 = c2.s - c2.k;
                    n2 = n2 < 0 ? n2 * 38 >> 8 : n2 * 230 >> 8;
                    c2.j = (c2.j * 150 >> 8) + n2;
                } else {
                    c2.c = c2.r;
                    c2.k = c2.s;
                    c2.b = c2.t;
                    c2.j = c2.u;
                }
                return;
            }
            case 5: 
            case 6: {
                c2.r += c2.t;
                c2.s += c2.u;
                c2.v += c2.x;
                c2.w += c2.y;
                c2.u -= 1400;
                c2.y -= 1400;
                d.a(c2);
                if (c2.s + c2.u < 0) {
                    c2.y += -c2.u * 200 >> 8;
                    c2.u = -c2.u * 120 >> 8;
                    c2.s = 0;
                    c2.t = c2.t * 80 >> 8;
                }
                if (c2.w + c2.y < 0) {
                    c2.u += -c2.y * 200 >> 8;
                    c2.y = -c2.y * 120 >> 8;
                    c2.w = 0;
                    c2.x = c2.x * 80 >> 8;
                }
                c2.c += c2.b;
                c2.k += c2.j;
                if (c2.c > 128000) {
                    c2.b = c2.c > 256000 ? c2.b * 240 >> 8 : c2.b * 250 >> 8;
                }
                c2.j -= 700;
                if (c2.c >= 47360) {
                    if (c2.k + c2.j < 62720) {
                        if (c2.c < 128000) {
                            c2.b = -6000;
                            c2.j = 12000;
                            c2.k = 62720;
                        } else {
                            c2.o = 7;
                            c2.p = 0;
                            if (c2.z > 1024000) {
                                c2.z = 0;
                            }
                            c2.k = 62720;
                            if (bl) {
                                d.a(26, 128, 1);
                                d.j(aZ, bd, c2.z > 0 ? c2.z / 855 : -1);
                            }
                        }
                    }
                } else if (c2.k + c2.j < 0) {
                    c2.j = (-c2.j >> 1) - c2.k;
                    c2.b >>= 4;
                }
                if (c2.o == 5 && c2.j < -2000) {
                    c2.o = 6;
                    c2.p = 0;
                    c2.A = c2.k;
                }
                if (c2.k < 65000 && d.g(c2.b) < 10 && d.g(c2.j) < 1000) {
                    c2.o = 7;
                    c2.p = 0;
                    if (c2.z > 1024000) {
                        c2.z = 0;
                    }
                    if (bl) {
                        if (c2.z <= 0) {
                            d.a(22, 128, 1);
                        }
                        d.j(aZ, bd, c2.z > 0 ? c2.z / 855 : -1);
                    }
                    return;
                }
                return;
            }
            case 7: {
                c2.r += c2.t;
                c2.s += c2.u;
                c2.v += c2.x;
                c2.w += c2.y;
                if (c2.s < 1000) {
                    c2.t = c2.t * 100 >> 8;
                    c2.u = c2.u * 100 >> 8;
                } else {
                    c2.t = c2.t * 230 >> 8;
                }
                if (c2.w < 1000) {
                    c2.x = c2.x * 100 >> 8;
                    c2.y = c2.y * 100 >> 8;
                } else {
                    c2.x = c2.x * 230 >> 8;
                }
                c2.u -= 1400;
                c2.y -= 1400;
                d.a(c2);
                if (c2.s + c2.u < 0) {
                    c2.y += -c2.u * 150 >> 8;
                    c2.u = -c2.u * 90 >> 8;
                    c2.s = 0;
                    c2.t = c2.t * 80 >> 8;
                }
                if (c2.w + c2.y < 0) {
                    c2.u += -c2.y * 150 >> 8;
                    c2.y = -c2.y * 90 >> 8;
                    c2.w = 0;
                    c2.x = c2.x * 80 >> 8;
                }
                if (!bl || c2.p <= 250 || ba != 11 || G) break;
                d.aF();
            }
        }
    }

    private static int o(int n, int n2) {
        if (n < 0) {
            n = -n;
        }
        if (n2 < 0) {
            n2 = -n2;
        }
        if (n < n2) {
            if (n2 < n << 4) {
                return n * 441 + n2 * 967 >> 10;
            }
            return n * 441 + n2 * 1007 >> 10;
        }
        if (n < n2 << 4) {
            return n2 * 441 + n * 967 >> 10;
        }
        return n2 * 441 + n * 1007 >> 10;
    }

    private static void i(c c2) {
        int n;
        int n2 = c2.r + c2.t - (c2.v + c2.x);
        int n3 = c2.s + c2.u - (c2.w + c2.y);
        c2.D = n = d.o(n2, n3) >> 6;
        if (n > 0) {
            long l = n - 5160;
            if (l > 0L) {
                c2.t = (int)((long)c2.t - l * (long)n2 / (long)n);
                c2.u = (int)((long)c2.u - l * (long)n3 / (long)n);
                return;
            }
            if (l < 0L) {
                c2.t = (int)((long)c2.t - (l * (long)n2 / (long)n >> 6));
                c2.u = (int)((long)c2.u - (l * (long)n3 / (long)n >> 6));
            }
        }
    }

    private static void j(c c2) {
        int n;
        int n2 = c2.v + c2.x - (c2.r + c2.t);
        int n3 = c2.w + c2.y - (c2.s + c2.u);
        c2.D = n = d.o(n2, n3) >> 6;
        if (n > 0) {
            long l = n - 5160;
            if (l > 0L) {
                c2.x = (int)((long)c2.x - l * (long)n2 / (long)n);
                c2.y = (int)((long)c2.y - l * (long)n3 / (long)n);
                return;
            }
            if (l < 0L) {
                c2.x = (int)((long)c2.x - l * (long)n2 / (long)n);
                c2.y = (int)((long)c2.y - l * (long)n3 / (long)n);
            }
        }
    }

    private static int a(c c2) {
        int n;
        int n2 = c2.r + c2.t - (c2.v + c2.x);
        int n3 = c2.s + c2.u - (c2.w + c2.y);
        c2.D = n = d.o(n2, n3) >> 6;
        if (n > 0) {
            long l = n - 5160;
            int n4 = (int)(l * (long)n2 / (long)n) >> 3;
            int n5 = (int)(l * (long)n3 / (long)n) >> 3;
            c2.t -= n4;
            c2.u -= n5;
            c2.x += n4;
            c2.y += n5;
        }
        return n;
    }

    private static void bL() {
        int n;
        int n2;
        int n3;
        int n4;
        int n5;
        switch (ba) {
            case 3: 
            case 10: 
            case 11: 
            case 12: 
            case 13: {
                n5 = d.e.c + ((bP >> 8) * (bP >> 8) >> 1);
                n4 = d.e.k >> 8;
                if (n4 < 0) {
                    n4 = 0;
                }
                if (d.e.o < 3) {
                    n4 = d.a(100, -1024, (d.e.c >> 8) - -15360, 15360);
                    n3 = d.a(0, -8800, (d.e.c >> 8) - -15360, 15360);
                    d.k(n5 + 36864 >> 8, n4, 0, n3);
                    break;
                }
                if (d.e.o > 7) break;
                if (n5 > 256000) {
                    n5 = 256000;
                }
                n3 = n4 * n4;
                n2 = -8800 - (n3 >> 10);
                n4 >>= 1;
                if ((n4 -= 300) < 0) {
                    n4 = 0;
                }
                if (d.e.o == 7) {
                    n = d.e.p;
                    if (n > 3000) {
                        n = 3000;
                    }
                    n4 += n >> 3;
                    n2 += n;
                }
                d.k(n5 >> 8, -1024 + n4, 0, n2);
                break;
            }
            case 15: {
                n5 = 0;
                n4 = 0;
                n3 = 0;
                if (bb < 1000) {
                    d.m(-512, 0);
                    break;
                }
                if (bb < 3000) {
                    d.m(d.b(-512, 768, bb - 1000, 4000), d.b(0, 937, bb - 1000, 2000));
                    break;
                }
                if (bb < 5000) {
                    d.m(d.b(-512, 768, bb - 1000, 4000), d.b(937, 0, bb - 1000 - 2000, 2000));
                    break;
                }
                d.m(768, 0);
                break;
            }
            default: {
                if (d.e.c >= 0) {
                    d.m(768, 0);
                    break;
                }
                d.m(d.e.c >> 8, 0);
            }
        }
        d.bM();
        d.l(500, 2500, -256);
        n4 = bB;
        d.l(500, 0, -256);
        n3 = bA;
        n2 = bB;
        d.a(n3, n2, n3, n4, 7, 16771584, 0x938338);
        d.l(500, be * 855 >> 8, -256);
        n5 = n3 + 22;
        b.setColor(16606208);
        d.b(n3, bB, n5);
        b.setColor(7284224);
        d.b(n3, bB + 1, n5);
        d.a(483, n5, bB, 3, 0);
        n5 = (d.e.c >= 118000 ? d.e.z : d.e.k) >> 8;
        d.l(500, n5, -256);
        n4 = bA;
        n3 = bB;
        d.l(500, n5, 256);
        n2 = bA;
        n = bB;
        d.l(500, 0, -256);
        int n6 = bA;
        d.l(500, 0, 256);
        int n7 = bA;
        d.a(481, n6, n3, 3, 0);
        d.l(200, 250, -256);
        int n8 = bA;
        int n9 = bB;
        d.l(1800, 250, -256);
        int n10 = bA;
        int n11 = bB;
        d.l(1800, 250, 256);
        int n12 = bA;
        int n13 = bB;
        d.l(200, 250, 256);
        int n14 = bA;
        int n15 = bB;
        d.l(200, 0, -256);
        int n16 = bB;
        d.l(200, 0, 256);
        int n17 = bB;
        if (aI != 2) {
            b.setColor(1721735);
            d.c(n10, n16, n16 - n11, n12, n17, n17 - n13);
        }
        b.setColor(16765490);
        d.c(n8, n9, n10 - n8, n14, n15, n12 - n14);
        if (n8 < n14) {
            b.setColor(12292608);
            d.d(n8, n9, 10, n14, n15, 10);
            b.setColor(2041415);
            d.d(n8, n9 + 10, n16 - n9 - 10, n14, n15 + 10, n17 - n15 - 10);
        }
        b.setColor(1517941);
        b.fillRect(n14, n15, n12 - n14, n17 - n15);
        d.a(n14, 0, n12 - n14, 320);
        int n18 = n14;
        while (n18 < n12) {
            d.a(514, n18, n15, 20, 0);
            n18 += 59;
        }
        n18 = n15 + n17 >> 1;
        d.a(515, n14 + 27, n18, 17, 0);
        d.a(515, n12 - 27, n18, 17, 0);
        d.a(515, n12 + n14 >> 1, n18, 17, 0);
        d.a(0, 0, 240, 320);
        if (d.e.c >= 128000 && n5 > 250) {
            n8 = n4 > n2 ? 2 : -2;
            b.setColor(0xCCCCEE);
            n9 = 0;
            while (n9 < 6) {
                b.drawLine(n6 + n8, n3 + n9, n7 + n8, n + n9);
                ++n9;
            }
            b.setColor(0xFFFFFF);
            if (n3 < n) {
                n9 = -2;
                while (n9 <= 2) {
                    b.drawLine(n6 + n9, n3, n7 + n9, n);
                    ++n9;
                }
            } else {
                n9 = -2;
                while (n9 <= 2) {
                    b.drawLine(n6 + n9, n3 + 6, n7 + n9, n + 6);
                    ++n9;
                }
            }
        }
        if (ba == 10 || ba == 11 || ba == 12 || ba == 13 || ba == 3) {
            d.k(e);
            d.g(e, aZ, 0);
        } else if (ba >= 15) {
            n8 = 0;
            while (n8 < aP) {
                if ((bo & 1 << n8) == 0) {
                    d.k(e[n8]);
                }
                ++n8;
            }
            n8 = 0;
            while (n8 < aP) {
                n9 = B[aP - 1 - n8];
                if ((bo & 1 << n9) == 0) {
                    d.g(e[n9], n9, aP - 1 - n8);
                }
                ++n8;
            }
        }
        d.l(500, 2500, 256);
        n9 = bB;
        d.l(500, 0, 256);
        n10 = bA;
        n11 = bB;
        d.a(n10, n11, n10, n9, 7, 16771584, 0x938338);
        d.a(480, n7, n, 3, 0);
        if (ba == 11 || ba == 3) {
            d.l(d.e.c >> 8, d.e.k >> 8, 0);
            if (d.e.o == 1) {
                d.a(e, bA + 15, bB);
            } else if (d.e.b) {
                if (aX >= 0 && ch - (aX >> 2) < 200) {
                    d.l(bR, bS, 0);
                    d.i(ch - (aX >> 2), bR, bS);
                }
            } else if (d.e.o == 2) {
                bR = bA + 61;
                bS = bB - 124;
                if (e) {
                    d.l(bR, bS);
                }
            }
        }
        if (aI != 2) {
            d.l(500, 0, 256);
            d.a(691, bA, bB, 3, 0);
            d.a(693, bA + 60, bB + 10, 3, 0);
            d.a(693, bA + 100, bB, 3, 0);
        }
    }

    private static void k(c c2) {
        int n = c2.c >> 8;
        if (n < 200) {
            d.l(n, 0, c2.i);
            int n2 = c2.c / 81920 & 3;
            d.a(518 + n2, bA - 9, bB, 3, 0);
            return;
        }
        d.l(n, 250, c2.i);
        if (c2.k > 76800) {
            int n3 = c2.c / 81920 & 3;
            d.a(518 + n3, bA - 9, bB, 3, 0);
        }
    }

    private static void g(c c2, int n, int n2) {
        int n3;
        a = c2;
        C = true;
        int n4 = c2.c >> 8;
        if (ba == 15 && c2.o == 7) {
            d.l(n4, c2.k >> 8, c2.i);
            int n5 = d.l(n, bd);
            if (n5 != -1) {
                int n6 = 0;
                if (n2 == 0) {
                    n6 = 1;
                }
                int n7 = n6 + 5;
                int n8 = n2 == 0 ? 25 : 16;
                boolean bl = false;
                if (n2 == 0) {
                    bl = true;
                }
                String string = d.a(n5, bc, bl);
                int n9 = bB - (256 * d.a[n].d >> 8) - 4;
                boolean bl2 = false;
                if (n >= aN) {
                    bl2 = true;
                }
                d.a(n7, n8, string, bA, n9, 1, bl2);
            } else {
                int n10 = bB - (256 * d.a[n].d >> 8) - 4;
                boolean bl = false;
                if (n >= aN) {
                    bl = true;
                }
                d.a(3, 20, "FOUL", bA, n10, 1, bl);
            }
        }
        if (c2.c > 51200) {
            d.l(n4, 250, c2.i);
            d.a(0, 0, 240, bB);
        }
        d.l(n4, c2.k >> 8, c2.i);
        short[] sArray = M;
        int n11 = -1;
        int n12 = 160;
        int n13 = n4 - -15360;
        int n14 = -1;
        if (ba == 10) {
            sArray = K;
            n13 = ch;
            n11 = 0;
        } else if (c2.o <= 0) {
            sArray = L;
            n13 = c2.p;
        } else if (c2.o <= 2) {
            if (!c2.b) {
                sArray = M;
                n11 = 4;
            } else {
                sArray = N;
                n12 = 1000;
                n13 = 0;
                bA -= 14;
                bB -= 84;
            }
        } else if (c2.o < 5) {
            sArray = N;
            n12 = 90;
            n13 = c2.k >> 9;
            if (n13 < 220) {
                n13 = 220 - (n13 - 220) * 3 / 2;
            }
        } else if (c2.o == 5) {
            sArray = O;
            n12 = 100;
            if (c2.b) {
                n13 = c2.p;
            } else {
                sArray = s;
                n12 = 120;
                n13 = c2.p;
            }
        } else if (c2.o == 6) {
            if (c2.c > 0) {
                sArray = P;
                n12 = (c2.A >> 8) / 6;
                n13 = c2.A - c2.k >> 8;
            } else {
                sArray = v;
                n12 = 120;
                n13 = c2.p;
            }
        } else if (c2.o == 7) {
            n13 = c2.p;
            n12 = 120;
            if (n13 < 600) {
                sArray = Q;
            } else {
                n3 = d.l(n, bd);
                n14 = ba >= 15 && aP > 1 ? d.n(c2.F, n3) : d.n(n3);
            }
        }
        int n15 = 0;
        if (d.a[n].c == 0) {
            n15 = 1;
        }
        n3 = 1 - n15;
        if (n14 < 0) {
            d.a(n3, d.a[n].a, d.a[n].d, sArray, n13, n12, bA, bB, n11);
        } else {
            d.a(n3, d.a[n].a, d.a[n].d, n14, n13, bA, bB);
        }
        d.a(0, 0, 240, 320);
        if (c2.o >= 2) {
            d.l(c2);
        }
        if (n4 >= 200) {
            d.l(200, 250, -256);
            int n16 = bA + 9 + 23;
            d.l(1800, 250, -256);
            int n17 = bA - 9 - 23;
            d.l(n4, 250, c2.i);
            if (c2.k <= 76800 && bA >= n16 && bA <= n17) {
                if (c2.o == 7 && c2.p > 600) {
                    d.a(517, bA - 10, bB, 3, 0);
                    return;
                }
                d.a(516, bA - (c2.o == 7 ? 10 : 0), bB, 3, 0);
            }
        }
    }

    private static void q(int n, int n2, int n3) {
        if (d.a.o < 2) {
            d.a(a, true, n, n2, n3);
        }
    }

    private static void l(c c2) {
        d.a(c2, false, 0, 0, 0);
    }

    private static void a(c c2, boolean bl, int n, int n2, int n3) {
        int n4;
        d.l((c2.r >> 8) + 0, c2.s >> 8, c2.i);
        int n5 = bA;
        int n6 = bB;
        int n7 = n5;
        int n8 = n6;
        d.l((c2.v >> 8) + 0, c2.w >> 8, c2.i);
        int n9 = bA;
        int n10 = bB;
        if (bl) {
            n5 = n;
            n6 = n2;
            n9 = n5 + (h[n3] * 184 >> 14);
            n10 = n6 - (h[128 + n3 & 0x1FF] * 184 >> 14);
        }
        b.setColor(0xFFFFFF);
        int n11 = (n9 <<= 10) - (n5 <<= 10);
        int n12 = (n10 <<= 10) - (n6 <<= 10);
        int n13 = c2.o < 3 ? 184 : d.o(n11, n12) >> 10;
        if (n13 > 0) {
            n11 /= n13;
            n12 /= n13;
            n4 = 184 - n13 >> 1;
            if (n4 < 0) {
                n4 = 0;
            }
            int n14 = (n9 - n5) / 16;
            int n15 = (n10 - n6) / 16;
            n9 = n5 >> 10;
            n10 = n6 >> 10;
            boolean bl2 = false;
            int n16 = 0;
            while (n16 <= 16) {
                int n17 = 256 * n16 / 16;
                n17 = h[n17];
                n17 = n4 * n17 >> 14;
                int n18 = n5 + n17 * n12 >> 10;
                int n19 = n6 - n17 * n11 >> 10;
                if (n16 < 15) {
                    if (bl2 ^= true) {
                        d.a(n9, n10, n18, n19, 3, 8045824, 22272);
                    } else {
                        d.a(n9, n10, n18, n19, 3, 6067456, 18176);
                    }
                } else {
                    d.a(n9, n10, n18, n19, 3, 6441289, 4602176);
                }
                if (n16 == bQ >> 10) {
                    n7 = n18;
                    n8 = n19;
                }
                if ((ba == 11 || ba == 3) && n16 == bQ >> 10) {
                    int n20 = 256 * (n16 + 1) / 16;
                    n20 = h[n20];
                    n20 = n4 * n20 >> 14;
                    int n21 = n5 + n14 + n20 * n12 >> 10;
                    int n22 = n6 + n15 + n20 * n11 >> 10;
                    int n23 = bQ & 0x3FF;
                    int n24 = d.i(n18 - n9 << 10, n19 - n10 << 10);
                    n24 = 512 - n24;
                    if (c2.o == 3 || c2.o == 4) {
                        n24 += 256;
                        n21 = n9;
                        n22 = n10;
                        n23 = 1023 - n23;
                    }
                    n24 += 512;
                    b.setColor((int)((char)-1));
                    n7 = n18 + ((n21 - n18) * n23 >> 10);
                    n8 = n19 + ((n22 - n19) * n23 >> 10);
                    int n25 = bT - bU;
                    if (n25 > 0) {
                        while (n25 > 0) {
                            aE = n7 << 10;
                            aF = n8 << 10;
                            d.a(aE, aF, 1, 0, 0, 0, 0, 1024, n24 - 64, n24 + 64);
                            --n25;
                        }
                        bU = bT;
                    }
                }
                n9 = n18;
                n10 = n19;
                n5 += n14;
                n6 += n15;
                ++n16;
            }
        }
        if ((ba == 11 || ba == 3) && c2.o >= 2 && c2.o < 5) {
            n4 = ch >> 7 & 3;
            if (n4 == 1) {
                n4 = 1;
            } else if (n4 == 2) {
                n4 = 2;
            } else if (n4 == 3) {
                n4 = 3;
            }
            d.a(469, n7, n8, 3, n4);
        }
    }

    private static void bM() {
        if (aI == 2) {
            d.bR();
            d.bS();
        } else {
            d.bN();
        }
        C = false;
        d.aI();
        d.bO();
    }

    private static void bN() {
        int n = 0;
        ++n;
        d.k[0] = 0;
        ++n;
        d.k[1] = 0;
        d.l(bw, 0, -8960);
        if (bD > 0) {
            ++n;
            d.k[2] = bD;
            ++n;
            d.k[3] = 11725567;
        }
        d.l(bw, 0, -1536);
        if (bD > 0) {
            d.k[n++] = bD;
            d.k[n++] = 8104530;
        }
        int n2 = -1536;
        while (n2 < -768) {
            d.l(bw, 0, n2);
            d.k[n++] = bD;
            d.k[n++] = ci;
            d.l(bw, 0, n2 + 32);
            d.k[n++] = bD;
            d.k[n++] = aJ == 0 ? 13801852 : 9284301;
            n2 += 200;
        }
        d.l(bw, 0, -768);
        d.k[n++] = bD;
        d.k[n++] = ci;
        n2 = -743;
        while (n2 < -544) {
            d.l(bw, 0, n2);
            d.k[n++] = bD;
            d.k[n++] = 0x484848;
            d.k[n++] = bD + 16384;
            d.k[n++] = 0;
            n2 += 50;
        }
        d.l(bw, 0, -544);
        d.k[n++] = bD;
        d.k[n++] = 0x484848;
        d.l(bw, 0, -512);
        d.k[n++] = bD;
        d.k[n++] = 0xFFFFFF;
        d.l(bw, 0, -256);
        d.k[n++] = bD;
        d.k[n++] = ci;
        d.l(bw, 0, 256);
        d.k[n++] = bD;
        d.k[n++] = cj;
        d.l(bw, 0, 512);
        d.k[n++] = bD;
        d.k[n++] = ci;
        d.l(bw, 0, 544);
        d.k[n++] = bD;
        d.k[n++] = 0xFFFFFF;
        d.k[n++] = bD + 65536;
        d.k[n++] = 0;
        d.k[n++] = 0x1400000;
        d.k[n++] = 0x30A030;
        ao = n >> 1;
        d.J();
        d.l(bw, 0, -1792);
        n2 = bB;
        if (aJ == 1) {
            d.g(8213, -1536, 0, n2 - 20 - 88);
            d.c(8195, -1536, 0, n2 - 20 - 88, 120);
            d.c(8194, -1536, 0, n2 - 20 - 44, 120);
            d.c(8195, -1536, 0, n2 - 20, 120);
            d.g(8193, -1280, 0, n2);
        } else {
            d.f(8222, -128000, 0);
            d.i(8208, -10496, 0, -5376);
            d.i(8207, -5376, 0, -1536);
            d.g(8192, -1536, 0, n2 - 20);
            d.c(8195, -1536, 0, n2 - 20, 120);
            d.g(8197, -1280, 0, n2);
        }
        short s = d.a(8210);
        d.l(-5120, 0, -304);
        int n3 = bA;
        while (n3 > -s) {
            d.a(8210, 0, n3, bB, 40, 0);
            n3 -= s + 256;
        }
    }

    private static void bO() {
        b.setColor(0xFFFFFF);
        d.l(-5120, 0, 0);
        int n = bA;
        int n2 = bB;
        d.l(-5180, 0, -60);
        int n3 = 0;
        while (n3 < 4) {
            b.drawLine(n + n3, n2, bA + n3, bB);
            ++n3;
        }
        d.l(-5180, 0, 60);
        n3 = 0;
        while (n3 < 4) {
            b.drawLine(n + n3, n2, bA + n3, bB);
            ++n3;
        }
        d.l(-280, 0, -64);
        n = bA;
        n2 = bB;
        d.l(-20, 0, -64);
        n3 = bA;
        d.l(-20, 0, 64);
        int n4 = bA;
        d.l(-280, 0, 64);
        int n5 = bA;
        int n6 = bB;
        b.setColor(0x484848);
        d.c(n, n2, n3 - n, n5, n6, n4 - n5);
        d.l(-20, 0, -256);
        n = bA;
        n2 = bB;
        d.l(20, 0, -256);
        n3 = bA;
        d.l(20, 0, 256);
        n4 = bA;
        d.l(-20, 0, 256);
        n5 = bA;
        n6 = bB;
        b.setColor(0xFFFFFF);
        d.c(n, n2, n3 - n, n5, n6, n4 - n5);
        d.b(-5136, -340, 32, 84, 0xFFFFFF);
        d.b(-5104, -340, 1675, 84, 16776502);
        d.b(-3429, -340, 32, 84, 0xFFFFFF);
        d.b(-3397, -340, 1675, 84, 15834373);
        d.b(-1722, -340, 32, 84, 0xFFFFFF);
        d.b(-1690, -340, 1674, 84, 13641520);
        d.b(-5136, 256, 32, 84, 0xFFFFFF);
        d.b(-5104, 256, 1675, 84, 16776502);
        d.b(-3429, 256, 32, 84, 0xFFFFFF);
        d.b(-3397, 256, 1675, 84, 15834373);
        d.b(-1722, 256, 32, 84, 0xFFFFFF);
        d.b(-1690, 256, 1674, 84, 13641520);
        d.b(-30720, -256, 30720, 45, 0xFFFFFF);
        d.b(-30720, 211, 30720, 45, 0xFFFFFF);
        d.l(0, 0, -320);
        d.a(493, bA, bB, 33, 0);
        d.l(0, 0, 320);
        d.a(492, bA, bB, 33, 0);
    }

    private static void a(int n, int n2, int n3, int n4, int n5, int n6, int n7) {
        int n8;
        int n9;
        int n10;
        int n11;
        int n12;
        int n13;
        int n14;
        int n15 = n4 - n2;
        int n16 = n - n3;
        int n17 = d.g(n15);
        int n18 = d.g(n16);
        int n19 = n15 < 0 ? -1 : 1;
        int n20 = n14 = n16 < 0 ? -1 : 1;
        if (n17 > n18) {
            n13 = n19;
            n12 = 0;
            n11 = n19;
            n10 = n14;
            n9 = n18;
            n8 = n17;
        } else {
            n13 = 0;
            n12 = n14;
            n11 = n19;
            n10 = n14;
            n9 = n17;
            n8 = n18;
        }
        int n21 = 0;
        int n22 = 0;
        int n23 = n3 - n;
        int n24 = n4 - n2;
        int n25 = d.o(n8, n9);
        if (n25 < 1) {
            n25 = 1;
        }
        n5 = n5 * n8 >> 1;
        int n26 = n8 >> 1;
        if (n > n3) {
            int n27 = n7;
            n7 = n6;
            n6 = n27;
        }
        n23 += n;
        n24 += n2;
        ++n5;
        while (true) {
            if ((n26 -= n9) < 0) {
                n26 += n8;
                b.setColor(n6);
                b.drawLine(n + (n21 += n11), n2 + n22, n23 + n21, n24 + n22);
                b.setColor(n7);
                b.drawLine(n - n21, n2 - n22, n23 - n21, n24 - n22);
                n22 += n10;
            } else {
                n21 += n13;
                n22 += n12;
            }
            if ((n5 -= n25) < 0) {
                b.setColor(0);
                b.drawLine(n - n21, n2 - n22, n23 - n21, n24 - n22);
                b.setColor(n6);
                b.drawLine(n, n2, n23, n24);
                return;
            }
            b.setColor(n6);
            b.drawLine(n + n21, n2 + n22, n23 + n21, n24 + n22);
            b.setColor(n7);
            b.drawLine(n - n21, n2 - n22, n23 - n21, n24 - n22);
        }
    }

    private static void c(int n, int n2, int n3, int n4, int n5, int n6) {
        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n, n2, n + n3, n2, n4 + n6, n5, b.getColor() & 0xFFFFFF | 0xFF000000);
        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n, n2, n4 + n6, n5, n4, n5, b.getColor() & 0xFFFFFF | 0xFF000000);
    }

    private static void d(int n, int n2, int n3, int n4, int n5, int n6) {
        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n, n2, n4, n5, n4, n5 + n6, b.getColor() & 0xFFFFFF | 0xFF000000);
        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n, n2, n4, n5 + n6, n, n2 + n3, b.getColor() & 0xFFFFFF | 0xFF000000);
    }

    private static void bP() {
        d.a(-1, e, 0L, -3932160);
        d.h(e);
        while (d.e.c < -2621440) {
            if (d.e.q - d.e.h > 10) {
                d.h(e, d.e.d == 0 ? 102 : 104, false);
            }
            d.e(e, false);
        }
        aX = -1;
        d.v(d.e.q);
        bP = 0;
    }

    private static void U(int n) {
        switch (n) {
            case 0: {
                d.h(e, d.e.d == 0 ? 102 : 104, true);
                return;
            }
            case 1: {
                d.h(e, 103, true);
                return;
            }
            case 2: {
                d.i(e, 103, true);
            }
        }
    }

    private static void bQ() {
        switch (bp & 7) {
            case 0: {
                if (d.e.o == 1) {
                    if (d.e.q - d.e.h > 10) {
                        d.h(e, d.e.d == 0 ? 102 : 104, true);
                    }
                    d.e(e, false);
                } else if (d.e.o == 2) {
                    d.e(e, false);
                    if (d.e.v + d.e.x >= 0) {
                        d.w(9);
                    }
                }
                return;
            }
            case 1: {
                d.e(e, false);
                if (d.e.u - 700 <= 0) {
                    d.w(10);
                }
                return;
            }
            case 2: {
                d.e(e, false);
                if (d.e.o != 6 || d.e.k >= 131072 || G) break;
                d.aF();
            }
        }
    }

    private static int C(int n) {
        if (n == 1) {
            return 187;
        }
        return 0;
    }

    private static void r(int n, int n2, int n3) {
        int n4 = 6 + n2;
        int n5 = 57 + n3;
        d.k(n4, n5);
        if (n == 0) {
            d.a(8196, "1. " + a[37], n4 + 6, n5 + 6, 20);
            int n6 = n5 + 6 + 29 + 14;
            d.a(471, n4 + 228 - 6, n6, 10, 0);
            d.a(470, n4 + 228 - 6 - 34, n6, 10, 0);
            d.a(3, a[43] + ' ', n4 + 228 - 6 - 68, n6 - 11, 24);
            return;
        }
        if (n == 1) {
            d.a(8196, "2. " + a[38], n4 + 6, n5 + 6, 20);
            int n7 = n5 + 6 + 29 + 14;
            d.a(472, n4 + 228 - 6, n7, 10, 0);
            d.a(485, n4 + 228 - 6 - 17, n7 - 17 + 6 - 3 + ch * 256 / 12000 % 9, 33, 0);
            d.a(3, a[45] + ' ', n4 + 228 - 6 - 34, n7 - 11, 24);
            return;
        }
        d.a(8196, "3. " + a[46], n4 + 6, n5 + 6, 20);
        int n8 = n5 + 6 + 29 + 14;
        d.a(472, n4 + 228 - 6, n8, 10, 0);
        d.a(484, n4 + 228 - 6 - 17, n8 - 17 + 6 + 3 - ch * 256 / 12000 % 9, 33, 0);
        d.a(3, a[46] + ' ', n4 + 228 - 6 - 34, n8 - 11, 24);
    }

    private static void bR() {
        int n = 0;
        ++n;
        d.k[0] = 0;
        ++n;
        d.k[1] = 0;
        d.l(0, 0, -23040);
        int n2 = bB;
        int n3 = n2 - 64 - 1;
        int n4 = 0;
        while (n4 <= 16) {
            if (n3 > 0) {
                d.k[n++] = n3 << 16;
                d.k[n++] = (0 + 91 * (n4 * 256 / 16) & 0xFF00FF00 | 0 + 18944 * (n4 * 256 / 16) & 0xFF0000) >> 8;
            }
            ++n4;
            n3 += 4;
        }
        d.l(0, 0, -17920);
        d.k[n++] = bD - 65536;
        d.k[n++] = 3693153;
        d.l(0, 0, -12800);
        d.k[n++] = bD - 65536;
        d.k[n++] = 6194829;
        d.l(0, 0, -7680);
        d.k[n++] = bD - 65536;
        d.k[n++] = 10071735;
        d.l(0, 0, -7168);
        d.k[n++] = bD - 65536;
        d.k[n++] = 10926773;
        d.l(0, 0, -6656);
        d.k[n++] = bD - 65536;
        d.k[n++] = 7382183;
        n3 = 0;
        while (n3 < 12) {
            n4 = d.a(-6656, -2022, n3, 11);
            d.l(0, 0, n4);
            if (bD > 0) {
                d.k[n++] = bD;
                d.k[n++] = (10879157 * (256 - n3 * 256 / 11) + 14221536 * (n3 * 256 / 11) & 0xFF00FF00 | 47616 * (256 - n3 * 256 / 11) + 59648 * (n3 * 256 / 11) & 0xFF0000) >> 8;
            }
            ++n3;
        }
        if ((bc & 1) != 0) {
            d.l(0, 0, -2022);
            d.k[n++] = bD;
            d.k[n++] = 14281184;
            n3 = 0;
            while (n3 < 6) {
                d.l(0, 0, (-6 + n3) * 333 + 12);
                d.k[n++] = bD;
                d.k[n++] = 0xFFFFFF;
                d.l(0, 0, (-6 + n3 + 1) * 333 - 12);
                d.k[n++] = bD;
                d.k[n++] = 6077096;
                ++n3;
            }
            d.l(0, 0, 24);
            d.k[n++] = bD;
            d.k[n++] = 0xFFFFFF;
        } else if (aM == 1) {
            d.l(bw, 0, -268);
            d.k[n++] = bD;
            d.k[n++] = 14281184;
            d.l(bw, 0, -244);
            d.k[n++] = bD;
            d.k[n++] = 0xFFFFFF;
            d.l(bw, 0, 244);
            d.k[n++] = bD;
            d.k[n++] = 6077096;
            d.l(bw, 0, 268);
            d.k[n++] = bD;
            d.k[n++] = 0xFFFFFF;
        }
        d.k[n++] = 0x1400000;
        d.k[n++] = 14281184;
        ao = n >> 1;
        d.J();
        d.a(0, 0, 240, n2);
        d.a(8219, 0, 0, (int)d.b(8219), 1024, 0);
        d.a(0, 0, 240, 320);
        d.f(8221, -23040, (int)d.a(8221));
        d.g(785, -17920, 0);
        d.g(784, -12800, 0);
        d.f(8217, -6400, (int)d.a(8217));
        if (aM == 4) {
            d.l(-7680, 0, -5120);
            d.a(8220, 0, bA, bB, 1024, 0);
        }
        d.h(8216, aM == 4 ? -3328 : -2816, 512, -7168);
        d.h(8218, aM == 4 ? -2560 : -2048, 384, -2048);
        switch (aM) {
            case 0: 
            case 2: {
                n3 = -2048;
                break;
            }
            case 4: {
                n3 = -768;
                break;
            }
            default: {
                n3 = -512;
            }
        }
        d.f(8215, n3, 0);
    }

    private static void bS() {
        int n;
        switch (aM) {
            case 0: 
            case 2: {
                n = 512;
                break;
            }
            case 4: {
                n = 1024;
                break;
            }
            default: {
                n = 768;
            }
        }
        d.f(8214, n, 0);
    }

    private static void bT() {
        bV = -1;
        bX = 0;
        bW = 0;
    }

    private static void bU() {
        bX += 32;
        if (bV == 0) {
            bb += 32;
        } else if (bV >= 3) {
            d.af();
        }
        if (bX >= bW) {
            d.bV();
        }
    }

    private static void bV() {
        if (++bV == 0) {
            int n = aP;
            int n2 = 0;
            while (n2 < n) {
                d.a[n2].g = -400 + (h[123 + n2 * 19357 & 0x1FF] * 300 >> 14);
                d.a[n2].h = -400 - 100 * n + n2 * 110;
                d.a[n2].e = 0;
                d.a[n2].f = 0;
                ++n2;
            }
            bW = 2000;
        } else if (bV <= 3) {
            switch (bV) {
                case 1: {
                    if (aP >= 3) {
                        ++d.a[d.y[2]].e;
                        d.a[d.y[2]].f = aR + 500;
                        break;
                    }
                    d.bV();
                    break;
                }
                case 2: {
                    if (aP >= 2) {
                        ++d.a[d.y[1]].e;
                        d.a[d.y[1]].f = aR + 500;
                        break;
                    }
                    d.bV();
                    break;
                }
                case 3: {
                    ++d.a[d.y[0]].e;
                    d.a[d.y[0]].f = aR + 500;
                    if (aP >= 3 && y[3] >= aN && (t & 0xFF) < 64) {
                        e e2 = a[y[3]];
                        int n = 0;
                        if ((t & 0xFF) < 16) {
                            n = 1;
                        }
                        e2.e = 3 - n;
                        d.a[d.y[3]].f = aR + 500;
                    }
                    d.ae();
                    int n = 0;
                    while (n < 200) {
                        d.d(d.a() % 245760, -(d.a() % 819200), 5, d.a() % 3, 0);
                        ++n;
                    }
                    break;
                }
            }
            bW = 4000;
        } else if (bV == 4) {
            bW = 8000;
        } else {
            d.ap();
        }
        bX = 0;
    }

    private static void bW() {
        int n;
        int n2;
        int n3;
        int n4;
        int n5;
        int n6;
        int n7;
        int n8;
        int n9;
        if (bV == 0) {
            n9 = d.a(600, 0, bX, 1000);
            if (n9 < 0) {
                n9 = 0;
            }
        } else {
            n9 = 0;
        }
        d.k(0, n9, 0, 0);
        int n10 = 0;
        ++n10;
        d.k[0] = 0;
        ++n10;
        d.k[1] = 0;
        d.l(bw, 0, -1281);
        if (bD > 0) {
            ++n10;
            d.k[2] = bD;
            ++n10;
            d.k[3] = 11725567;
        }
        int n11 = 0;
        while (n11 < 6) {
            int n12 = d.a(-1280, 1024, n11, 5);
            d.l(bw, 0, n12);
            if (bD > 0) {
                int[] nArray = new int[]{10143323, 12374893, 10143323, 11914837, 9814862, 11783245};
                d.k[n10++] = bD;
                d.k[n10++] = nArray[n11];
            }
            ++n11;
        }
        d.k[n10++] = 0x1400000;
        d.k[n10++] = 0x30A030;
        ao = n10 >> 1;
        d.J();
        d.l(0, 0, -1792);
        n11 = bB;
        boolean bl = false;
        if (aJ == 1) {
            d.g(8213, -1536, -50, n11 - 20 - 88);
            d.c(8195, -1536, -50, n11 - 20 - 88, 120);
            d.c(8194, -1536, -50, n11 - 20 - 44, 120);
            d.c(8195, -1536, -50, n11 - 20, 120);
            d.g(8193, -1280, -50, n11);
        } else {
            d.f(8222, -128000, 0);
            d.i(8208, -10496, 0, -5376);
            d.i(8207, -5376, 0, -1536);
            d.g(8192, -1536, 0, n11 - 20);
            d.c(8195, -1536, 0, n11 - 20, 120);
            d.g(8197, -1280, 0, n11);
        }
        d.C(-1400);
        int n13 = -32;
        while (n13 < 240) {
            d.a(689, n13, bB, 36, 0);
            n13 += 86;
        }
        d.C(-1000);
        d.a(16385, 0, 240 - d.a(16385), bB, 1024, 0);
        int n14 = aP;
        int n15 = 0;
        while (n15 < n14) {
            if (d.a[n15].e == 0 || d.a[n15].f > aR) {
                n8 = d.a[n15].g;
                n7 = d.a[n15].h;
                d.l(n8, 0, n7);
                n6 = n8 / 81920 & 3;
                d.a(518 + n6, bA - 18, bB + 1, 3, 0);
                int n16 = 0;
                if (d.a[n15].c == 0) {
                    n16 = 1;
                }
                n5 = 1 - n16;
                d.a(n5, d.a[n15].a, d.a[n15].d, i, aR + 12 * (n15 ^ 0xC), 190 + 21 * (n15 ^ 6), bA, bB, true);
            } else if (d.a[n15].e >= 2) {
                n8 = d.a[n15].g;
                n7 = d.a[n15].h;
                d.l(n8, 0, n7);
                n6 = n8 / 81920 & 3;
                d.a(518 + n6, bA - 18, bB + 1, 3, 0);
                int n17 = 0;
                if (d.a[n15].c == 0) {
                    n17 = 1;
                }
                n5 = 1 - n17;
                d.a(n5, d.a[n15].a, d.a[n15].d, d.a[n15].e == 2 ? 5 : 2, aR - d.a[n15].f + 12 * (n15 ^ 0xC), bA, bB);
            }
            ++n15;
        }
        d.C(0);
        bA = 0;
        n13 = bA + 50 + 28 + 120;
        n15 = bB;
        d.a(500, n13, n15, 40, 0);
        d.a(502, n13, n15 - 16, 40, 0);
        n8 = n13 - 28;
        n7 = n15 - 16 - 9;
        d.a(500, n13 -= 50, n15 += 5, 40, 0);
        d.a(501, n13, n15 - 16, 40, 0);
        d.a(501, n13, n15 - 16 - 8, 40, 0);
        d.a(502, n13, n15 - 16 - 8 - 8, 40, 0);
        n6 = n13 - 28;
        n5 = n15 - 16 - 8 - 8 - 9;
        d.a(500, n13 -= 50, n15 += 7, 40, 0);
        d.a(501, n13, n15 - 16, 40, 0);
        d.a(502, n13, n15 - 16 - 8, 40, 0);
        int n18 = n13 - 28;
        int n19 = n15 - 16 - 8 - 9;
        int n20 = 0;
        while (n20 < 3) {
            n4 = y[n20];
            if (d.a[n4].e == 1 && d.a[n4].f <= aR) {
                int n21;
                int n22;
                n3 = aR - d.a[n4].f;
                switch (n20) {
                    case 0: {
                        n2 = n6 + 14;
                        n22 = n5;
                        break;
                    }
                    case 1: {
                        n2 = n18 + 14;
                        n22 = n19;
                        break;
                    }
                    default: {
                        n2 = n8 + 14;
                        n22 = n7;
                    }
                }
                int n23 = d.a[n4].g;
                int n24 = d.a[n4].h;
                d.l(n23, 0, n24);
                n = n3 < 800 ? n3 : 800;
                int n25 = d.a(bA, n2, n, 800);
                int n26 = d.a(bB, n22, n, 800);
                if (n == 800) {
                    n21 = n23 / 81920 & 3;
                    d.a(518 + n21, n25 - 18, n26 + 1, 3, 0);
                }
                int n27 = 0;
                if (d.a[n4].c == 0) {
                    n27 = 1;
                }
                n21 = 1 - n27;
                d.a(n21, d.a[n4].a, d.a[n4].d, 1, n3 + 12 * (n4 ^ 0xC), n25, n26);
            }
            ++n20;
        }
        d.ag();
        d.C(1000);
        if (bB < 280) {
            bB = 280;
        }
        n20 = 0;
        n13 = 28;
        while (n13 < 240) {
            d.a(16386, 0, 120 + n13, bB - n20 % 15, 1024, 0);
            d.a(16386, 0, 120 - n13, bB - (n20 += 1834356) % 15, 1024, 0);
            n20 += 28361233;
            n13 += 63;
        }
        if (aR < 400) {
            d.r((aR << 16) / 400);
        }
        d.a(30, a[98]);
        n4 = bV - 1;
        if (n4 >= 0 && n4 <= 2) {
            n3 = d.D(50);
            d.a(503 + n4, 50 + n3, 47, 20, 0);
            n2 = bX - 460;
            if (n2 > 0 && (n2 >>= 6) < 4) {
                d.a(506 + n2, 50, 47, 20, 0);
            }
            d.a(3, a[165 + n4], 76 + n3, 54, 20);
            n3 = d.D(80);
            int n28 = 0;
            if (aN > 0) {
                n28 = 1;
            }
            n = n28 + 107;
            n13 = 240;
            while (n13 > 0) {
                d.a(n, n13 - n3, 92, 24, 0);
                n13 -= 64;
            }
            n4 = y[2 - n4];
            d.a(65 + d.a[n4].b, 0 - n3 + 3, 113, 36, 0);
            d.a(3, d.a[n4].a, 0 - n3 + 3 + 40, 93, 20);
            d.a(6, "" + x[n4 * 6 + 5], 240 + n3 - 8, 88, 24);
        }
        if (bV == 4 && bX > bW - 300) {
            n3 = bX - (bW - 300);
            if (n3 > 300) {
                n3 = 300;
            }
            d.r((char)-1 - (n3 << 16) / 300);
        }
    }

    private static int D(int n) {
        if (bX < n + 400) {
            if (bX < n) {
                return 240;
            }
            if (bX < n + 400) {
                return d.c(0, 240, 256 - (bX - n << 8) / 400, 256);
            }
            return 240;
        }
        return 0;
    }

    private static void bX() {
        if (t < bY || t > bY + 500) {
            d.t();
            bY = t;
        }
    }

    private static void bY() {
        int n = t / 2 % 480;
        if (h == null) {
            if (n < 240) {
                int n2 = 2;
                while (n2 < n) {
                    b.fillRect(n2, 159, 2, 2);
                    n2 += 4;
                }
            } else {
                int n3 = 2 + n - 240;
                while (n3 < 240) {
                    b.fillRect(n3, 159, 2, 2);
                    n3 += 4;
                }
            }
        } else if (n < 240) {
            int n4 = 2;
            while (n4 < n) {
                b.setColor(d.a(0xFF0000, 0xFFFFFF, d.g(n4 * 2 - 240) % 255));
                int n5 = h[128 + 128 * d.g(n4 - 120) / 120 & 0x1FF];
                n5 = n5 * n5 >> 14;
                n5 = n5 * n5 >> 14;
                n5 = n5 * (h[n4 * 8 & 0x1FF] * 53 >> 14) >> 14;
                b.fillRect(n4 / 2 + 60, 160 + n5 / 3, 2, 2);
                n4 += 7 - (h[128 + 128 * d.g(n4 - 120) / 120 & 0x1FF] >> 12);
            }
        } else {
            n -= 240;
            int n6 = 2;
            while (n6 < 240) {
                if (n6 > n) {
                    int n7 = h[128 + 128 * d.g(n6 - 120) / 120 & 0x1FF];
                    n7 = n7 * n7 >> 14;
                    n7 = n7 * n7 >> 14;
                    n7 = n7 * (h[n6 * 8 & 0x1FF] * 53 >> 14) >> 14;
                    b.fillRect(n6 / 2 + 60, 160 + n7 / 2, 2, 2);
                }
                n6 += 4;
            }
        }
        b.setColor(0xFF0000);
    }

    private static void bZ() {
        b.setColor(0xFFFFFF);
        b.fillRect(0, 0, 240, 320);
        if (J) {
            b.setColor(16715817);
            b.fillRect(0, 35, 240, 3);
        }
        d.bY();
        if (!h) {
            b.setColor(0xFFFFFF);
            l = 0;
            d.a(1024, a[19], 120, 120, 33);
            return;
        }
        d.a(3, a[19], 120, 120, 33);
    }

    private static void ca() {
        J = true;
        H = false;
        d.b(true);
        d.o(0);
        d.h(1, 0);
        bZ = t;
    }

    private static void cb() {
        int n;
        int n2;
        int n3 = (319 - d.e(0)) / 2;
        int n4 = t - bZ;
        b.setColor(0xFFFFFF);
        b.fillRect(0, 0, 240, 320);
        int n5 = 0;
        int n6 = n4 % 3000 >> 6;
        if (n4 < 6000) {
            boolean bl = false;
            if (n4 % 3000 >= 0) {
                bl = true;
            }
            boolean bl2 = false;
            if (n4 % 3000 < 500) {
                bl2 = true;
            }
            if (bl & bl2) {
                n5 = d.a(240, 0, n4 % 3000, 500);
            } else {
                boolean bl3 = false;
                if (n4 % 3000 >= 2500) {
                    bl3 = true;
                }
                boolean bl4 = false;
                if (n4 % 3000 < 3000) {
                    bl4 = true;
                }
                if (bl3 & bl4) {
                    n5 = d.a(0, -240, n4 % 3000 - 2500, 500);
                }
            }
        } else {
            if (n4 - 6000 < 500) {
                n5 = 240;
            } else {
                boolean bl = false;
                if (n4 - 6000 >= 500) {
                    bl = true;
                }
                boolean bl5 = false;
                if (n4 - 6000 < 1000) {
                    bl5 = true;
                }
                if (bl & bl5) {
                    n5 = d.a(240, 0, n4 - 6000 - 500, 500);
                } else {
                    boolean bl6 = false;
                    if (n4 - 6000 >= 5000) {
                        bl6 = true;
                    }
                    boolean bl7 = false;
                    if (n4 - 6000 < 5500) {
                        bl7 = true;
                    }
                    if (bl6 & bl7) {
                        n5 = d.a(0, -240, n4 - 6000 - 5000, 500);
                    } else if (n4 - 6000 >= 5500) {
                        n5 = -240;
                    }
                }
            }
            n6 = n4 - 6000 >> 6;
        }
        if (n4 > 5000) {
            n2 = 0;
            n = 0;
            while (n < 6) {
                int n7 = n++;
                int n8 = n4 - 5000;
                int n9 = d.a(240, -480, n8, 6800);
                d.a(14 + n7, 120 + n9 + n2, n3, 36, 0);
                n2 += d.c(14 + n7);
            }
        }
        if (n4 > 5000) {
            n2 = n3;
            n = (n4 - 5000) / 2;
            if (n > 255) {
                n = 255;
            }
            b.setColor(d.a(0xFFFFFF, 16715817, n));
            if (n4 >= 12000) {
                n2 = d.c(n3, 35, n4 - 12000, 750);
            }
            b.fillRect(0, n2, 240, 3);
        }
        if (n4 < 3000) {
            d.a(419, 150 + n5 - n6, n3 + 12, 33, 0);
            return;
        }
        if (n4 < 6000) {
            d.a(420, 150 + n5 - n6, n3, 33, 0);
            return;
        }
        if (n4 < 12000) {
            d.a(417, 150 + n5 - n6, n3 + 2 + 2, 33, 0);
            d.a(418, 90 - n5 + n6, n3 - 2 - 2, 17, 0);
        }
    }

    private static void cc() {
        if (t > bZ + 12000 + 750) {
            H = true;
            a.keyPressed(a[17] - 50);
        }
    }

    private static void cd() {
        J = false;
        I = false;
        byte[] byArray = d.a("s");
        if (byArray == null) {
            d.d(false);
        } else {
            d.d(true);
        }
        d.a[1406] = 1;
        d.ce();
        d.cf();
        d.cn();
        L = false;
        aJ = 0;
    }

    private static void ce() {
        int n = 0;
        if (j[24] != 0) {
            n = 1;
        }
        d.a[1406] = (short)(1 - n);
    }

    private static void cf() {
        int n = 0;
        if (j[25] != 0) {
            n = 1;
        }
        d.a[1458] = (short)(1 - n);
    }

    private static void c(boolean bl) {
        d.a[1362] = (short)(bl ? 0 : 2);
        d.a[1454] = (short)(bl ? 0 : 2);
    }

    private static void d(boolean bl) {
        d.a[1334] = (short)(bl ? 0 : 2);
        d.c(16, 3, bl ? 1 : 0);
        d.c(17, 3, bl ? 1 : 0);
        d.c(18, 3, bl ? 1 : 0);
    }

    private static void V(int n) {
        int n2 = f[G];
        if (n2 == 2 && n == 12) {
            I = true;
            s = true;
            d.cl();
            return;
        }
        if (n2 == 0) {
            if (n == 0) {
                N = true;
                s = true;
            } else if (n == 16) {
                d.am();
                d.d(false);
                d.l(6);
            } else if (n == 17) {
                d.am();
                d.d(false);
                d.l(8);
            } else if (n == 18) {
                d.am();
                d.d(false);
                d.l(13);
            }
        }
        if (n2 == 9) {
            if (n == 28) {
                aO = 6 - aN;
                aP = 6;
                d.l(10);
            } else if (n == 29) {
                aO = 0;
                aP = aN;
                d.l(10);
            }
        }
        if (n2 == 11 && n == 26) {
            aI = 1;
            aK = 5;
            aL = 31;
            L = true;
            s = true;
            return;
        }
        if (n2 == 12) {
            if (n >= 19 && n < 24) {
                int n3 = n - 19;
                if ((aL & 1 << n3) != 0) {
                    aL &= ~(1 << n3);
                    return;
                }
                aL |= 1 << n3;
                return;
            }
            if (n == 25) {
                aI = 1;
                aK = 0;
                int n4 = 0;
                while (n4 < 5) {
                    if ((aL & 1 << n4) != 0) {
                        ++aK;
                    }
                    ++n4;
                }
                if (aK > 0) {
                    L = true;
                    s = true;
                }
                return;
            }
        }
        if (n2 == 1 && n == 30) {
            d.j[18] = 0;
            d.j[19] = 0;
            d.j[20] = 0;
            d.j[21] = 0;
            d.j[22] = 0;
            int n5 = 0;
            while (n5 < 12) {
                d.j[6 + n5] = 0;
                ++n5;
            }
            d.j[23] = 0;
            aJ = 0;
            d.W();
            d.am();
            cl = 0;
            d.j[24] = 0;
            d.j[4] = 0;
            d.j[5] = 0;
            d.j[25] = 0;
            d.j[26] = 0;
            d.cf();
            d.ce();
            d.l();
        }
        if (n2 == 15) {
            if (n == 33) {
                d.cv();
                return;
            }
            if (n == 34) {
                d.cw();
            }
        }
    }

    private static void cg() {
        if (n || !m) {
            d.L();
            if (cb >= 0) {
                d.a(cb, 128, 1);
                cb = -1;
            }
        }
    }

    private static void ch() {
        int n;
        int n2;
        b.setColor(0xFFFFFF);
        b.fillRect(0, 0, 240, 319 - d.e(0));
        b.setColor(0);
        b.fillRect(0, 319 - d.e(0), 240, 320 - (319 - d.e(0)));
        boolean bl = false;
        if (f[G] == 15) {
            bl = true;
        }
        if (u) {
            return;
        }
        if (h[H] != ce) {
            cf = ce;
            cd = t;
            ce = h[H];
        }
        if (f[G] != 0) {
            String string = a[K];
            int n3 = 0;
            n2 = 0;
            n3 = 60 + d.e(t - cc, 3);
            b.setColor(16715817);
            if (f[G] == 14 || f[G] == 5) {
                n3 = 147 + d.e(t - cc, 3);
                d.a(8196, string, n3, 3, 17);
            } else if (f[G] == 3 || f[G] == 6 || f[G] == 7 || f[G] == 10 || bl) {
                n3 = 224 + d.e(t - cc, 3);
                d.a(8196, string, n3, 3, 24);
            } else {
                d.a(8196, string, n3, 3, 20);
            }
        }
        boolean bl2 = false;
        b.setColor(16715817);
        b.fillRect(0, 35, 240, 3);
        if (f[G] == 3) {
            b.setColor(0x9B9B9B);
            b.fillRect(0, 39, 240, 319 - d.e(0) - 39);
            d.a(421, 120, 160, 3, 0);
            d.a(0, 39, 240, 319 - d.e(0) - 39);
            n2 = 0;
            while (n2 <= 240) {
                n = 0;
                while (n <= 320) {
                    d.a(111, n2, n, 20, 0);
                    n += 24;
                }
                n2 += 24;
            }
            d.a(0, 0, 240, 320);
        }
        if (f[G] == 7) {
            if (aJ == 1) {
                d.a(54, 4, 4, 20, 0);
            } else if (aJ == 0) {
                d.a(53, 4, 4, 20, 0);
            }
        } else if (f[G] == 0) {
            d.a(0, 4, 4, 20, 0);
        } else if (f[G] != 5 || !bl) {
            d.a(0, 4, 4, 20, 0);
        }
        if (!u) {
            String string = "";
            n2 = 1;
            if (f[G] == 0) {
                switch (h[H]) {
                    case 0: {
                        string = a[99];
                        break;
                    }
                    case 16: {
                        string = a[100];
                        break;
                    }
                    case 17: {
                        string = a[101];
                        break;
                    }
                    case 18: {
                        string = a[102];
                        break;
                    }
                    case 6: {
                        string = a[103];
                        break;
                    }
                    case 2: {
                        string = a[104];
                        break;
                    }
                    case 4: {
                        string = a[105];
                        break;
                    }
                    case 32: {
                        string = a[111];
                        break;
                    }
                    case 3: {
                        string = a[106];
                    }
                }
                if (a[1332 + (h[H] * 4 + 2)] == 1) {
                    string = a[114];
                }
                if (j[25] == 1) {
                    string = string + ' ' + a[28];
                }
            } else if (f[G] == 12) {
                string = a[107];
            } else if (f[G] == 8) {
                string = a[108];
            } else if (f[G] == 9) {
                if (h[H] != ce) {
                    cd = t;
                    ce = h[H];
                }
                switch (h[H]) {
                    case 28: {
                        string = a[112];
                        break;
                    }
                    case 29: {
                        string = a[113];
                    }
                }
            } else if (f[G] == 6 && aJ == 1 && j[23] == 0) {
                string = a[109];
            } else {
                n2 = 0;
            }
            if (n2 != 0) {
                n = d.a(3, string);
                int n4 = 80 + (t - cd) * 3520 / 40000;
                b.setColor(0xBFBFBF);
                b.fillRect(0, 319 - d.e(0) - 23, 240, 23);
                d.a(3, string, 240 - (n4 %= n + 240), 319 - d.e(0) - 24, 20);
            }
        }
        r = false;
    }

    private static void W(int n) {
        if (n == 8 || n == 13) {
            d.X();
            return;
        }
        if (n == 5) {
            d.aa();
            return;
        }
        if (n == 14) {
            d.cq();
            return;
        }
        if (n == 6 || n == 7) {
            d.cn();
            return;
        }
        if (n == 8) {
            L = false;
            return;
        }
        if (n == 10) {
            aJ = 0;
            return;
        }
        if (n == 15) {
            d.ct();
        }
    }

    private static void X(int n) {
        if (n == 8 || n == 13) {
            d.Y();
            return;
        }
        if (n == 5) {
            d.ab();
            return;
        }
        if (n == 14) {
            d.cr();
            return;
        }
        if (n == 6 || n == 7) {
            d.co();
            return;
        }
        if (n == 10) {
            d.a(aJ == 0 ? 21 : 23, 60, 108, 3, 0);
            d.a(aJ == 1 ? 22 : 24, 180, 108, 3, 0);
            d.a(true, 0, 160, true);
            String string = aJ == 0 ? a[142] : a[143];
            d.a(J == 1 || u ? 3 : 8195, string, 120, 163, 17);
            d.e(120, 163, d.a(J == 1 || u ? 3 : 8195, string));
            d.e(5, 6);
            return;
        }
        if (n == 15) {
            d.cu();
        }
    }

    private static void p(int n, int n2) {
        if (n2 == 8 || n2 == 13) {
            d.s(n);
            return;
        }
        if (n2 == 5) {
            d.t(n);
            return;
        }
        if (n2 == 14) {
            d.ac(n);
            return;
        }
        if (n2 == 6 || n2 == 7) {
            d.ab(n);
            return;
        }
        if (n2 == 10) {
            boolean bl = d.a(n, 131076);
            boolean bl2 = d.a(n, 131078);
            boolean bl3 = d.a(n, 196616);
            boolean bl4 = d.a(n, 43);
            if (bl || bl2) {
                aJ = 1 - aJ;
                d.a(27, 128, 1);
            } else if (bl3) {
                d.l(11);
                d.a(28, 128, 1);
            } else if (bl4) {
                d.C();
                d.a(29, 128, 1);
            }
            return;
        }
        if (n2 == 15) {
            d.ad(n);
        }
    }

    private static void Y(int n) {
        cd = cc = t;
        ce = -1;
        cf = -1;
        cb = -1;
        if (n == 0) {
            N = false;
            if (!H) {
                d.b(true);
                d.h(1 + ca++ % 3, 0);
                H = true;
            }
            if (K) {
                if (!I) {
                    d.l(6);
                    d.l(7);
                    if (cs == 0 && j[6 + (aJ == 0 ? 0 : 6) + cl] != 0) {
                        cr = t;
                    }
                    if (!H) {
                        d.b(true);
                        d.h(1 + ca++ % 3, 0);
                    }
                }
                K = false;
            }
            if (L) {
                L = false;
            }
            I = false;
            d.c(true);
            return;
        }
        if (n == 2) {
            d.c(false);
        }
    }

    private static int d() {
        return 28;
    }

    private static void a(boolean bl, int n, int n2, boolean bl2) {
        b.setColor(bl ? 16715817 : 0x7F7F7F);
        if (bl2) {
            b.fillRect(0, n2, 240, 28);
            return;
        }
        d.a(0, n2, 240, 28);
        DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n - 42, n2, n - 42 - 8, n2 + 28, n - 42, n2 + 28, b.getColor() & 0xFFFFFF | 0xFF000000);
        b.fillRect(n - 42, n2, 240 - (n - 42), 28);
        d.a(0, 0, 240, 320);
    }

    private static void q(int n, int n2) {
        int n3;
        int n4;
        int n5;
        int n6;
        boolean bl = false;
        boolean bl2 = false;
        if (a[1332 + (n2 * 4 + 2)] == 1) {
            bl2 = true;
        }
        if (h[H] == n2) {
            bl = true;
        }
        int n7 = 0;
        if (bl) {
            d.a(bl2 ^ true, 60, n, false);
        }
        n7 = 60 + d.e(t - cc, n);
        if (!bl2) {
            int n8;
            if (bl) {
                n8 = t - cd;
                n7 = n8 > 200 ? (n7 -= 12) : (n7 -= d.c(12, 0, 200 - n8, 200));
            } else if (cf == n2 && (n8 = t - cd) < 200) {
                n7 -= d.c(0, 12, 200 - n8, 200);
            }
        }
        String string = a[a[1332 + (n2 * 4 + 3)]];
        if (a[1332 + n2 * 4] == 4) {
            string = string + " : ";
            string = j[a[1332 + (n2 * 4 + 1)]] == 1 ? string + a[10] : string + a[11];
        } else if (a[1332 + n2 * 4] == 5) {
            string = string + " : ";
            n6 = a[1332 + (n2 * 4 + 1)];
            n5 = n7 + d.a(J == 1 || u ? 3 : 8195, string);
            if (n6 == 0) {
                n6 = 2;
            }
            d.f(n5, n + 3, d.e(), 22, j[n6]);
        }
        if (f[G] == 12 && n2 >= 19 && n2 < 24) {
            n7 -= 2;
            n7 += 20;
            n6 = n2 - 19;
            int n9 = 0;
            if ((aL & 1 << n6) != 0) {
                n9 = n5 = 1;
            }
            if (bl) {
                d.a(425, n7, n + 3 + 20, 40, 0);
                if (n5 != 0) {
                    d.a(426, n7 - 13, n + 3 + 20, 33, 0);
                }
            } else {
                d.a(423, n7, n + 3 + 20, 40, 0);
                if (n5 != 0) {
                    d.a(424, n7 - 10, n + 3 + 20, 33, 0);
                }
            }
            n7 += 4;
        }
        n6 = -1;
        n5 = -1;
        if (f[G] == 0) {
            switch (n2) {
                case 16: {
                    n6 = 14;
                    n5 = 427;
                    break;
                }
                case 17: {
                    n6 = 15;
                    n5 = 429;
                    break;
                }
                case 18: {
                    n6 = 16;
                    n5 = 431;
                    break;
                }
                case 6: {
                    n6 = 17;
                    n5 = 433;
                    break;
                }
                case 2: {
                    n6 = 18;
                    n5 = 435;
                    break;
                }
                case 4: {
                    n6 = 19;
                    n5 = 437;
                    break;
                }
                case 3: {
                    n6 = 20;
                    n5 = 439;
                    break;
                }
                case 32: {
                    n5 = 441;
                    break;
                }
                case 0: {
                    n5 = 443;
                }
            }
            if (n6 >= 0 || n5 >= 0) {
                if (bl) {
                    if (!bl2) {
                        n4 = t - cd;
                        n3 = 0;
                        if (n6 >= 0) {
                            if (n4 < 200) {
                                n3 = d.c(0, d.b(n6), 200 - n4, 200);
                            }
                            d.a(0, 0, 240, n + 28);
                            d.a(n6, 236, n + 28 + n3, 40, 0);
                            d.a(0, 0, 240, 320);
                        }
                        d.a(n5, n7 - 2, n + 3 + 20, 40, 0);
                    } else {
                        d.a(445, n7 - 2, n + 3 + 20, 40, 0);
                    }
                } else if (!bl2) {
                    d.a(n5 + 1, n7 - 2, n + 3 + 20, 40, 0);
                } else {
                    d.a(446, n7 - 2, n + 3 + 20, 40, 0);
                }
            }
        }
        b.setColor(0xFFFFFF);
        l = 0;
        d.a(J == 1 || u ? 3 : 8195, string, n7, n + 3, 20);
        if (bl2) {
            if (bl) {
                d.a(0, n, 240, 28);
                n4 = 0;
                while (n4 <= 240) {
                    d.a(113, n4, n, 20, 0);
                    d.a(113, n4, n + 24, 20, 0);
                    n4 += 24;
                }
            } else {
                int n10 = 0;
                if (n6 >= 0) {
                    n10 = d.c(n6 + 2);
                }
                n3 = n7 - 2 - n10;
                d.a(n3, n, 240, 28);
                n4 = n3;
                while (n4 <= n7 - 2 + d.a(J == 1 || u ? 3 : 8195, string + ' ')) {
                    d.a(113, n4, n, 20, 0);
                    d.a(113, n4, n + 24, 20, 0);
                    n4 += 24;
                }
            }
            d.a(0, 0, 240, 320);
        }
    }

    private static String c(int n) {
        if (n == 30) {
            return a[150];
        }
        if (n == 7) {
            return a[151];
        }
        if (n == 16 || n == 17 || n == 18) {
            return a[152];
        }
        if (n == 33) {
            return d.e();
        }
        if (n == 34) {
            return d.f();
        }
        return null;
    }

    private static int e() {
        return 60;
    }

    private static void f(int n, int n2, int n3, int n4, int n5) {
        int n6 = (n3 - 7) / 8;
        int n7 = n4 * 287 / 1000;
        int n8 = 0;
        while (n8 < 8) {
            if (n5 > n8 * 8) {
                b.setColor(0xFFFFFF);
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n + 1, n2 + n4, n + n7 + 1, n2, n + n7 + n6 - 1, n2, b.getColor() & 0xFFFFFF | 0xFF000000);
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n + 1, n2 + n4, n + n7 + n6 - 1, n2, n + n6 - 1, n2 + n4, b.getColor() & 0xFFFFFF | 0xFF000000);
                b.setColor(0);
                b.drawLine(n, n2 + n4 - 1, n + n6 - 1, n2 + n4 - 1);
                b.drawLine(n, n2 + n4 - 2, n + n6 - 1, n2 + n4 - 2);
                b.drawLine(n + n6 - 1, n2 + n4 - 1, n + n7 + n6 - 1, n2);
                b.drawLine(n + n7 + n6 - 1, n2, n + n7, n2);
                b.drawLine(n + n7, n2, n, n2 + n4 - 1);
            } else {
                b.setColor(0xBFBFBF);
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n, n2 + n4 - 1, n + n7, n2 + 1, n + n7 + n6 - 1, n2 + 1, b.getColor() & 0xFFFFFF | 0xFF000000);
                DirectUtils.getDirectGraphics((Graphics)b).fillTriangle(n, n2 + n4 - 1, n + n7 + n6 - 1, n2 + 1, n + n6 - 1, n2 + n4 - 1, b.getColor() & 0xFFFFFF | 0xFF000000);
            }
            n += n6 + 1;
            ++n8;
        }
    }

    private static void ci() {
        G = d.a(0, 598, 48, null);
        d.O();
        d.i(0);
        d.ac();
        d.Q();
        d.T();
        E = d.a(0, 62, 20, null);
        d.g(1, 2990);
        d.g(2, 240);
        aL = 0;
        N = false;
        d.K();
        H = false;
        ca = 0;
        cl = 0;
        cs = 1;
        F = d.a(0, 1054, 48, null);
        d.ad();
    }

    private static void cj() {
        int n;
        ch = 0;
        M = false;
        cg = 0;
        d.bT();
        if (aI == 2) {
            aK = 5;
            aL = 31;
            aN = 1;
            aO = 1;
            aP = 2;
            aS = 1;
            aT = 0;
            aU = 0;
            b = (byte)-1;
            d.ah();
            n = 0;
            while (n < 5) {
                d.n[n] = n;
                n = (byte)(n + 1);
            }
            d.aj();
        } else {
            d.ah();
        }
        if (N) {
            d.an();
            d.as();
            d.aA();
        } else {
            d.ak();
            d.as();
            d.au();
            d.ao();
        }
        H = false;
        if (aI != 2 && aJ == 0) {
            ci = 12934952;
            cj = 14715962;
            ck = 13666640;
        } else {
            ci = 3565735;
            cj = 6591180;
            ck = 5278160;
        }
        d.c(3);
        if (aI == 2) {
            d.c(5);
        } else {
            d.c(4);
        }
        n = 0;
        while (n < aP) {
            d.c(F[d.a[n].a]);
            ++n;
        }
    }

    private static void ck() {
        d.al();
        d.ai();
        d.at();
    }

    private static void cl() {
        M = true;
    }

    private static void Z(int n) {
        if (d.a(n, 43)) {
            d.w();
            return;
        }
        if (aQ == 1) {
            d.x(n);
            return;
        }
        if (aQ == 2 || aQ == 4) {
            boolean bl = d.a(n, 131074);
            boolean bl2 = d.a(n, 131072);
            boolean bl3 = d.a(n, 196616);
            if (bl || bl2) {
                if (bl2) {
                    if (bn > 0) {
                        --bn;
                        d.a(27, 128, 1);
                    }
                } else if (bn < aP - (319 - d.e(0) - 56) / 32) {
                    ++bn;
                    d.a(27, 128, 1);
                }
            } else if (bl3) {
                d.ap();
            }
            return;
        }
        if ((aQ != 3 || aR > 4000) && d.a(n, 196616)) {
            d.ap();
        }
    }

    private static void aa(int n) {
        if (aQ == 1) {
            d.y(n);
        }
    }

    private static boolean e() {
        if (M) {
            return false;
        }
        d.L();
        int n = v;
        if (n > 125) {
            n = 125;
        }
        ch += n;
        cg += n;
        while (cg >= 32) {
            cg -= 32;
            aR += 32;
            if (aQ == 1) {
                d.ax();
                if (ba == 1) continue;
                d.af();
                continue;
            }
            if (aQ != 3) continue;
            d.bU();
        }
        q = true;
        return true;
    }

    private static void cm() {
        if (aQ == 1) {
            d.ay();
        } else if (aQ == 2 || aQ == 4) {
            b.setColor(0xFFFFFF);
            b.fillRect(0, 0, 240, 320);
            int n = 65536;
            if (aR < 400) {
                n = (aR << 16) / 400;
            }
            d.a(30, aQ == 2 ? a[34] : a[32], n, true, true, true);
            d.i(25, 24);
            int n2 = 4 * d.a(5, "0");
            d.a(0, 56, 240, (319 - d.e(0) - 56) / 32 * 32);
            int n3 = 0;
            int n4 = 56 - bn * 32;
            while (n3 < aN + aO) {
                int n5 = y[n3];
                int n6 = 0;
                int n7 = n3 * 60;
                if (aR < n7 + 400) {
                    n6 = aR < n7 ? 240 : d.c(0, 240, 256 - (aR - n7 << 8) / 400, 256);
                }
                int n8 = 8 + d.a(3, "6.");
                int n9 = 0;
                if (n5 < aN) {
                    n9 = 1;
                }
                int n10 = n9 + 107;
                int n11 = 240;
                while (n11 > 0) {
                    d.a(n10, n11 - n6, n4, 24, 0);
                    n11 -= 64;
                }
                if (aQ == 4 && n3 < 3) {
                    d.a(512 - n3, 240 + n6 - 16 - n2, n4 + 12, 10, 0);
                }
                d.a(3, "" + (n3 + 1) + '.', n8 - n6, n4 + 21 - 20, 24);
                d.a(86 + d.a[n5].b, n8 - n6 + 3, n4 + 21, 36, 0);
                d.a(3, d.a[n5].a, n8 - n6 + 3 + 30, n4 + 21 - 20, 20);
                d.a(5, "" + x[n5 * 6 + 5], 240 + n6 - 8, n4 + 21 - 16, 24);
                ++n3;
                n4 += 32;
            }
            d.a(0, 0, 240, 320);
            boolean bl = false;
            if (bn > 0) {
                bl = true;
            }
            boolean bl2 = false;
            if (bn < aP - (319 - d.e(0) - 56) / 32) {
                bl2 = true;
            }
            d.c(bl, bl2);
        } else if (aQ == 0) {
            b.setColor(0xFFFFFF);
            b.fillRect(0, 0, 240, 320);
            int n = 65536;
            if (aR < 400) {
                n = (aR << 16) / 400;
            }
            d.a(513, a[128], n, true, true, true, false);
            int n12 = 200;
            boolean bl = false;
            boolean bl3 = false;
            d.a(4, d.a(a[49], "" + aS, 1), 120, 107, 17);
            boolean bl4 = false;
            String string = d.a[d.aN].a;
            int n13 = d.c(64) + d.a(8196, string + ' ');
            int n14 = n13 * 3 / 2;
            if (200 > n14) {
                n12 = n14;
            }
            if (n12 < n13) {
                n12 = n13;
            }
            int n15 = n12;
            d.e(120 - n15 / 2 - 6, 218, n15 + 12, 46);
            d.a(3, a[68], 120 - n15 / 2, 225, 36);
            d.a(8196, string, 120 - n15 / 2, 252, 36);
            d.a(64, 120 + n15 / 2, 252, 40, 0);
            d.i(25, 24);
        }
        if (aQ == 3) {
            d.bW();
        }
    }

    private static void cn() {
        cm = 0;
        cn = 1;
        if (!K) {
            cr = -1;
        }
        K = false;
        O = false;
        Q = false;
        P = false;
        co = 0;
        cp = 319 - d.e(0) - 160 + 14;
        cq = 220;
        cu = 3;
        int n = 0;
        while (n < 2) {
            int n2 = 0;
            while (n2 < 6) {
                int n3 = n2 + (n == 0 ? 6 : 12);
                if (j[n3] == 0) {
                    cu &= ~(1 << n);
                    break;
                }
                ++n2;
            }
            ++n;
        }
        ct = 0;
        au = -1;
        R = false;
    }

    private static void co() {
        int n;
        int n2;
        int n3;
        int n4 = 120;
        int n5 = 120;
        int n6 = 400 - (t - cm);
        if (n6 > 0) {
            n4 = 120 + d.c(0, 360 * cn, n6, 400);
        }
        if ((n3 = 400 - (t - (cm + 200))) > 0) {
            n5 = 120 + d.c(0, 360 * cn, n3, 400);
        }
        if (f[G] == 6) {
            int n7;
            n2 = 0;
            n = 0;
            String string = aJ == 0 ? a[142] : a[143];
            d.a(aJ == 1 ? 41 : 53, 80, 96, 3, 0);
            d.a(aJ == 0 ? 42 : 54, 160, 96, 3, 0);
            d.a(0, 56, 240, 70);
            int n8 = 1;
            if (aJ == 1) {
                n8 = -1;
            }
            if (aJ == 1 && j[23] == 0) {
                n8 = 1;
            }
            int n9 = n8 < 0 ? 0 : 120;
            while (n9 < (n8 < 0 ? 120 : 240)) {
                int n10 = 126;
                while (n10 >= 0) {
                    d.a(113, n9, n10, 20, 0);
                    n10 -= 24;
                }
                n9 += 24;
            }
            d.a(0, 0, 240, 320);
            int n11 = 0;
            if (aJ == 1 && j[23] == 0) {
                n11 = 1;
            }
            d.a(1 - n11, 0, 126 + ct * 28, true);
            d.a(J == 1 || u ? 3 : 8195, string, 120, 129, 17);
            if (ct == 0) {
                d.e(120, 129, d.a(J == 1 || u ? 3 : 8195, string));
            }
            int n12 = 0;
            if (aJ == 0) {
                n12 = 1;
            }
            d.a(42 - n12, 3, 179, 36, 0);
            if (au >= 0) {
                d.j(31, 154);
            } else {
                d.a(3, d.b[0].a, 31, 157, 20);
            }
            d.a(86, 207, 179, 36, 0);
            b.setColor(0xBFBFBF);
            b.fillRect(10, 182, 220, 1);
            n8 = d.c(453) - d.c(453) / 7;
            n2 = aJ == 0 ? 0 : 6;
            n9 = 0;
            while (n9 < 6) {
                d.a(453, n4 - n8 * 2 - n8 / 2 + n8 * n9, 217, 3, 0);
                if (j[6 + n9 + n2] == 1) {
                    d.a(454, n4 - n8 * 2 - n8 / 2 + n8 * n9, 217 + d.b(454), 33, 0);
                }
                ++n9;
            }
            int n13 = 0;
            n9 = 0;
            while (n9 < 6) {
                int n14 = 2 + n9 + n2;
                n7 = 217 + d.b(454) / 2;
                if ((cu & 1 << aJ) != 0 && ct == 0) {
                    int n15 = 256 * (20 + (d.h(n9 * 917243) >> 1) % 10) >> 8;
                    int n16 = 400 + (d.h(n9 * 82123) >> 1) % 100;
                    int n17 = t % (n16 + 0);
                    if (n17 < n16) {
                        n7 -= 4 * n15 * n17 * (n16 - n17) / (n16 * n16);
                    }
                }
                if (j[6 + n9 + n2] == 1) {
                    d.a(n14, n4 - n8 * 2 - n8 / 2 + n8 * n9, n7, 33, 0);
                    ++n13;
                }
                ++n9;
            }
            if (aJ == 1 && j[23] == 0) {
                n7 = 24 + d.a(8196, a[161] + ' ');
                d.e(n5 - n7 / 2, 176, n7, 41);
                d.a(8196, a[161], n5 - 3, 208, 33);
            }
            if (n13 == 6) {
                n7 = 24 + d.a(8196, a[168] + ' ');
                d.e(n5 - n7 / 2, 224, n7, 41);
                d.a(8196, a[168], n5 - 3, 256, 33);
            }
        } else if (f[G] == 7) {
            int n18;
            n2 = aJ == 0 ? 0 : 6;
            n = 0;
            String string = a[93 + cl];
            int n19 = d.c(455) - d.c(455) / 8;
            int n20 = 120 - n19 * 2 - n19 / 2;
            int n21 = 0;
            while (n21 < 6) {
                n18 = 447 + n21;
                if (n21 != cl) {
                    if (j[6 + n21 + n2] == 1) {
                        d.a(456, n20, 96, 3, 0);
                    } else {
                        d.a(455, n20, 96, 3, 0);
                    }
                    d.a(n18, n20, 96, 3, 0);
                }
                n20 += n19;
                ++n21;
            }
            n20 = 120 - n19 * 2 - n19 / 2 + n19 * cl;
            if (!(j[6 + cl + n2] != 1 || e && 1500 - (t - cr) > 0)) {
                d.a(457, n20, 96, 3, 0);
            } else {
                d.a(1, n20, 96, 3, 0);
            }
            d.a(25 + cl, n20, 96, 3, 0);
            d.r(n4, 224);
            d.a(true, 0, 126, true);
            d.a(J == 1 || u ? 3 : 8195, string, 120, 129, 17);
            if (cr < 0 && !O && !P) {
                d.e(120, 129, d.a(J == 1 || u ? 3 : 8195, string));
            }
            int n22 = n2 = aJ == 0 ? 0 : 6;
            if (j[6 + cl + n2] == 1) {
                n18 = 1000 - (t - (cr + 1500));
                int n23 = 224;
                boolean bl = false;
                if (n18 > 0 && cr >= 0) {
                    n23 = 224 + d.c(0, 96, n18, 1000);
                } else {
                    n18 = 0;
                    cr = -1;
                }
                int n24 = 2 + cl + n2;
                int n25 = d.c(n24);
                boolean bl2 = false;
                int n26 = 12 + n25 + d.a(8196, a[137]);
                d.a(0, 154, 240, 319 - d.e(0) - 154);
                n2 = 113 + (n18 < 500 ? n18 / 100 : 5);
                if (n2 <= 117) {
                    n21 = 0;
                    while (n21 <= 240) {
                        int n27 = 126;
                        while (n27 <= 320) {
                            d.a(n2, n21, n27, 20, 0);
                            n27 += 24;
                        }
                        n21 += 24;
                    }
                }
                int n28 = n5 + 3 + n26 / 2 - 3;
                int n29 = 214;
                if (cr >= 0) {
                    int n30 = t - (cr + 1500);
                    if (n30 > 0 && n30 < 600) {
                        if (!R) {
                            cb = 17;
                            R = true;
                        }
                        n19 = d.c(455) - d.c(455) / 8;
                        int n31 = 120 - n19 * 2 - n19 / 2 + n19 * cl;
                        n28 = d.a(n31, n28, n30, 600);
                        n29 = d.d(96, 214, n30, 600);
                    }
                    if (n30 < 0) {
                        n28 = 120;
                        n29 = 640;
                    }
                    if (!O) {
                        O = true;
                    }
                }
                d.a(0, 224 - (41 + d.c(n24)), 240, 319 - d.e(0) - (224 - (41 + d.c(n24))));
                d.e(n5 - n26 / 2 - 6, (n23 += 13) - 41, n26 + 12, 41);
                d.a(8196, a[137], n5 - n26 / 2 + 3, n23 - 20 + 14 - 3, 36);
                d.a(0, 0, 240, 320);
                d.a(n24, n28, n29 += 13, 40, 0);
            }
        }
        if (P) {
            n2 = 319 - d.e(0) - cp - 12;
            n = 120 - cq / 2;
            d.e(n, n2, cq, cp);
            d.d(3, n + 6, n2 + 6, co);
        }
        if (f[G] == 6 && ct == 1) {
            if (au >= 0) {
                d.e(25, 27);
            } else {
                d.e(7, 6);
            }
        } else if (aJ == 1 && j[23] == 0) {
            d.e(-1, 6);
        } else if (cr >= 0) {
            d.e(-1, -1);
        } else if (O || P) {
            d.e(25, -1);
        } else {
            d.e(5, 6);
        }
        if (P) {
            boolean bl = false;
            if (co > 0) {
                bl = true;
            }
            boolean bl3 = false;
            if (Y - c[co] > cp) {
                bl3 = true;
            }
            d.c(bl, bl3);
        }
    }

    private static void r(int n, int n2) {
        String string = null;
        int n3 = aJ == 0 ? 0 : 5;
        int n4 = 55 + n3 + cl;
        int n5 = 65;
        if (cl < 5) {
            n5 = 65 + d.a[d.cl + n3 + 2].b;
        }
        if (cl < 5) {
            string = d.a[d.cl + n3 + 2].a;
        }
        if (n2 + d.b(n5) + d.b(n5) / 8 > 319 - d.e(0) - 12) {
            n2 = 319 - d.e(0) - (d.b(n5) + d.b(n5) / 8) - 12;
        }
        int n6 = 200;
        if (cl < 5) {
            int n7 = d.c(n4) + d.c(n5) + d.a(8196, string + ' ');
            int n8 = d.c(n4) + d.c(n5) + d.a(8196, string + ' ') * 3 / 2;
            if (200 > n8) {
                n6 = n8;
            }
            if (n6 < n7) {
                n6 = n7;
            }
            int n9 = n6;
            d.e(n - n9 / 2 - 6, n2 - 6, n9 + 12, 46);
            d.a(3, a[68], n - n9 / 2, n2 + 1, 36);
            d.a(n5, n - n9 / 2, n2 += 27, 36, 0);
            d.a(8196, string, n - n9 / 2 + d.c(n5), n2 += 3, 36);
            d.a(n4, n + n9 / 2, n2, 40, 0);
            return;
        }
        boolean bl = false;
        boolean bl2 = false;
        d.e(n - 110 - 6, n2 - 6, 232, 46);
        d.a(3, a[69], n, n2 + 34 + 12, 33);
        int n10 = 0;
        while (n10 < 5) {
            n4 = 55 + n3 + n10;
            d.a(n4, n - 110 + 22 + 44 * n10, n2, 3, 0);
            ++n10;
        }
    }

    private static void ab(int n) {
        boolean bl = d.a(n, 131076);
        boolean bl2 = d.a(n, 131078);
        boolean bl3 = d.a(n, 131072);
        boolean bl4 = d.a(n, 131074);
        boolean bl5 = d.a(n, 43);
        boolean bl6 = d.a(n, 196616);
        if (!(!bl5 || cr != -1 && P && O)) {
            if (!P && !O) {
                d.a(29, 128, 1);
            }
            if (f[G] == 6 && au >= 0 && ct == 1) {
                au = -1;
            } else if (!P && !O) {
                d.q();
                d.C();
            }
        }
        if (P) {
            if (bl4 && Y - c[co] > cp) {
                d.a(27, 128, 1);
                ++co;
            }
            if (bl3 && co > 0) {
                d.a(27, 128, 1);
                --co;
            }
        }
        if (cr == -1 || !O || f[G] == 6) {
            if (f[G] == 6) {
                if (ct == 0 && (bl || bl2)) {
                    d.a(27, 128, 1);
                    if (aJ == 1) {
                        aJ = 0;
                    } else if (aJ == 0) {
                        aJ = 1;
                    }
                    cm = cd = t;
                    if (bl) {
                        cn = -1;
                    }
                    if (bl2) {
                        cn = 1;
                    }
                }
                if ((bl3 || bl4) && (aJ == 0 || j[23] == 1)) {
                    d.a(27, 128, 1);
                    ct = 1 - ct;
                }
            } else if (!O && !P && cr < 0) {
                if (bl) {
                    d.a(27, 128, 1);
                    if (--cl < 0) {
                        cl = 5;
                    }
                    cm = t;
                    cn = -1;
                } else if (bl2) {
                    d.a(27, 128, 1);
                    ++cl;
                    cl %= 6;
                    cm = t;
                    cn = 1;
                }
            }
            if (bl6) {
                if (f[G] == 6) {
                    if (aJ != 1 || j[23] != 0) {
                        d.a(28, 128, 1);
                        if (ct == 0) {
                            d.V();
                            d.l(7);
                            d.q();
                            cl = 0;
                            cs = 1;
                            Q = true;
                        } else {
                            au = 0;
                            String string = d.a(a[29], d.b[0].a, 22, 8).toUpperCase();
                            if (string.length() > 0) {
                                d.b[0].a = null;
                                d.b[0].a = string;
                            }
                            d.a(28, 128, 1);
                            au = -1;
                            ct = 0;
                        }
                    }
                    return;
                }
                d.a(28, 128, 1);
                boolean bl7 = false;
                if (P) {
                    if ((cu & 1 << aJ) != 0) {
                        P = false;
                        Q = true;
                        O = false;
                        bl7 = true;
                    } else {
                        O = true;
                        P = false;
                        Q = true;
                    }
                }
                int n2 = 0;
                int n3 = 0;
                while (n3 < 6) {
                    if (j[6 + n3] == 1) {
                        ++n2;
                    }
                    ++n3;
                }
                if (!(j[25] == 1 && j[11] == 1 && j[17] == 1 && j[5] == 1 && j[4] == 1 || Q || cs != 0 || j[6 + (aJ == 0 ? 0 : 6) + cl] == 0)) {
                    if (cl == 5 && aJ == 0 && j[11] == 1) {
                        P = true;
                        if (n2 == 6) {
                            d.j[4] = 1;
                            d.a(-1, 3, a[67], cq - 12, cp - 12, true, false);
                        } else {
                            d.a(-1, 3, a[62], cq - 12, cp - 12, true, false);
                        }
                        O = false;
                    }
                    if (cl == 5 && aJ == 1 && j[17] == 1) {
                        P = true;
                        if (j[25] == 1) {
                            d.j[5] = 1;
                            d.a(-1, 3, a[66], cq - 12, cp - 12, true, false);
                        } else {
                            d.a(-1, 3, a[63], cq - 12, cp - 12, true, false);
                        }
                        O = false;
                    }
                    if (!P && j[5] == 0 && j[25] == 1) {
                        d.j[5] = 1;
                        P = true;
                        d.a(-1, 3, a[65], cq - 12, cp - 12, true, false);
                        O = false;
                    }
                    if (!P && j[4] == 0 && aJ == 0 && n2 == 6) {
                        d.j[4] = 1;
                        P = true;
                        d.a(-1, 3, a[64], cq - 12, cp - 12, true, false);
                        O = false;
                    }
                }
                if (O) {
                    n3 = aJ == 0 ? 0 : 6;
                    int n4 = 1;
                    while (n4 < 6) {
                        int n5 = (cl + n4) % 6;
                        if (j[6 + n3 + n5] == 0) {
                            cm = t;
                            cn = cl < n5 ? 1 : -1;
                            cl = n5;
                        }
                        ++n4;
                    }
                    O = false;
                    return;
                }
                if (!P && !bl7) {
                    P = false;
                    Q = false;
                    d.cp();
                    return;
                }
                if (bl7) {
                    d.C();
                }
            }
        }
    }

    private static void cp() {
        int n = 6 + (aJ == 0 ? 0 : 6) + cl;
        cs = j[n];
        if (cl < 5) {
            aI = 0;
            aK = 1;
            aL = 1 << cl;
            aN = 1;
            aO = 1;
            aP = aN + aO;
            K = true;
            cr = -1;
            s = true;
            return;
        }
        if (cl == 5) {
            aI = 0;
            aK = 5;
            aL = 31;
            aN = 1;
            aO = 5;
            aP = aN + aO;
            K = true;
            cr = -1;
            s = true;
            return;
        }
    }

    private static void cq() {
        cv = 0;
        cw = 0;
        if (j[25] == 1 && cv == 0) {
            d.a(-1, 3, a[28] + "\n\n" + a[115 + cv], 226, 319 - d.e(0) - 1 - 22 - 56, true, false);
        } else {
            d.a(-1, 3, a[115 + cv], 226, 319 - d.e(0) - 1 - 22 - 56, true, false);
        }
        cx = 0;
        cy = 0;
        S = true;
    }

    private static void cr() {
        int n = 92 + cv;
        b.setColor(16715817);
        b.fillRect(0, 35, 240, 23);
        b.setColor(0xDBDBDB);
        b.fillRect(0, 59, 240, 319 - d.e(0) - 35 - 22 - 1 - 1);
        d.a(0, 4, 4, 20, 0);
        int n2 = d.a(8195, a[n]);
        int n3 = 0;
        d.a(8195, a[n], 147, 35, 17);
        d.e(147, 35, n2);
        n3 = 0;
        if (t - cx < 240 && S) {
            n3 = t - cx;
        } else if (S) {
            if (j[25] == 1 && cv == 0) {
                d.a(-1, 3, a[28] + "\n\n" + a[115 + cv], 226, 319 - d.e(0) - 1 - 22 - 56, true, false);
            } else {
                d.a(-1, 3, a[115 + cv], 226, 319 - d.e(0) - 1 - 22 - 56, true, false);
            }
            S = false;
            cx = t;
            n3 = -240;
        } else if (t - cx < 240) {
            n3 = -(240 - (t - cx));
        } else {
            cx = -1;
        }
        d.d(3, cy * n3 + 7, 64, cw);
        d.e(-1, 6);
        boolean bl = false;
        if (cw > 0) {
            bl = true;
        }
        boolean bl2 = false;
        if (Y - c[cw] > 319 - d.e(0) - 1 - 22 - 56) {
            bl2 = true;
        }
        d.c(bl, bl2);
    }

    private static void ac(int n) {
        boolean bl = d.a(n, 131076);
        boolean bl2 = d.a(n, 131078);
        boolean bl3 = d.a(n, 131072);
        boolean bl4 = d.a(n, 131074);
        boolean bl5 = d.a(n, 43);
        if (bl) {
            d.a(27, 128, 1);
            if (--cv < 0) {
                cv = 5;
            }
            cx = cx == -1 ? t : t - 240;
            cw = 0;
            cy = 1;
            S = true;
            return;
        }
        if (bl2) {
            d.a(27, 128, 1);
            if (++cv > 5) {
                cv = 0;
            }
            cx = cx == -1 ? t : t - 240;
            cw = 0;
            cy = -1;
            S = true;
            return;
        }
        if (bl3) {
            if (cw > 0) {
                d.a(27, 128, 1);
                --cw;
            }
            return;
        }
        if (bl4) {
            if (Y - c[cw] > 319 - d.e(0) - 1 - 22 - 56) {
                d.a(27, 128, 1);
                ++cw;
            }
            return;
        }
        if (bl5) {
            d.a(29, 128, 1);
            d.C();
            d.q();
        }
    }

    private static boolean f() {
        d.b(5);
        int n = 0;
        while (n < 10) {
            if ((i[n + 20] & 0x10000) != 0) {
                return true;
            }
            ++n;
        }
        return false;
    }

    private static String c() {
        int n = H[5];
        return "" + n;
    }

    private static void cs() {
        if (cB == 0) {
            if (U) {
                d.a(2, 3, a[205], 226, 319 - d.e(0) - 107, true, false);
            } else {
                d.a(2, 3, a[205] + "\n\n" + a[203], 226, 319 - d.e(0) - 107, true, false);
            }
            return;
        }
        if (cB == 1) {
            d.a(2, 3, a[204], 226, 319 - d.e(0) - 107, true, false);
            return;
        }
        if (cB == 2) {
            d.a(2, 3, d.c(), 226, 319 - d.e(0) - 107, true, false);
        }
    }

    private static void ct() {
        U = d.f();
        H = new int[7];
        c = new String[7];
        int n = 0;
        while (n < 7) {
            d.H[n] = d.F(n);
            d.c[n] = d.d(n);
            ++n;
        }
        cz = -1;
        cA = -1;
        cC = 0;
        cB = 0;
        cD = 0;
        d.cs();
        T = false;
    }

    private static void cu() {
        d.a(true, 0, 62, true);
        b.setColor(0xDBDBDB);
        b.fillRect(0, 91, 240, 319 - d.e(0) - 91);
        String string = a[199];
        if (cB == 1) {
            string = a[200];
        }
        if (cB == 2) {
            string = a[206];
        }
        d.a(J == 1 || u ? 3 : 8195, string, 120, 65, 17);
        d.e(120, 65, d.a(J == 1 || u ? 3 : 8195, string));
        boolean bl = false;
        if (cB == 2 || cC == 2) {
            bl = true;
        }
        int n = 0;
        if (t - cz < 240 && T) {
            n = t - cz;
            if (cC != 2) {
                bl = false;
            }
        } else if (T) {
            d.cs();
            T = false;
            cz = t;
            n = -240;
            cC = cB;
            if (cC != 2) {
                bl = false;
            }
        } else if (t - cz < 240) {
            n = -(240 - (t - cz));
        } else {
            cz = -1;
        }
        if (bl) {
            int n2 = 0;
            while (n2 < (319 - d.e(0) - 107) / 22) {
                if (cD % 2 == 0) {
                    if (n2 % 2 == 0) {
                        b.setColor(0xFFFFFF);
                        b.fillRect(cA * n, 107 + n2 * 22, 240, 1);
                        b.setColor(0xAAAAAA);
                        b.fillRect(cA * n, 107 + n2 * 22 + 1, 240, 22);
                    }
                } else if (n2 % 2 == 1) {
                    b.setColor(0xFFFFFF);
                    b.fillRect(cA * n, 107 + n2 * 22, 240, 1);
                    b.setColor(0xAAAAAA);
                    b.fillRect(cA * n, 107 + n2 * 22 + 1, 240, 22);
                }
                ++n2;
            }
        }
        d.d(3, cA * n + 7, 107, cD);
        d.e(U || cB == 1 ? 5 : -1, 6);
        boolean bl2 = false;
        if (cD > 0) {
            bl2 = true;
        }
        boolean bl3 = false;
        if (Y - c[cD] > 319 - d.e(0) - 107) {
            bl3 = true;
        }
        d.c(bl2, bl3);
    }

    private static void ad(int n) {
        boolean bl = d.a(n, 131076);
        boolean bl2 = d.a(n, 131078);
        boolean bl3 = d.a(n, 131072);
        boolean bl4 = d.a(n, 131074);
        boolean bl5 = d.a(n, 43);
        boolean bl6 = d.a(n, 196616);
        if (bl5) {
            d.a(29, 128, 1);
            H = null;
            c = null;
            d.C();
        }
        if (bl || bl2) {
            d.a(27, 128, 1);
            cz = t;
            if (bl) {
                cC = cB--;
                if (cB < 0) {
                    cB = 1;
                }
                cA = 1;
            }
            if (bl2) {
                cC = cB++;
                if (cB > 1) {
                    cB = 0;
                }
                cA = -1;
            }
            cD = 0;
            T = true;
        }
        if (bl6 && (U || cB == 1)) {
            T = true;
            d.a(28, 128, 1);
            if (cB == 1) {
                d.d(34, 1);
            } else {
                d.d(33, 1);
            }
            T = true;
        }
        if (bl3 && cD > 0) {
            d.a(27, 128, 1);
            --cD;
        }
        if (bl4 && Y - c[cD] > 319 - d.e(0) - 107) {
            d.a(27, 128, 1);
            ++cD;
        }
    }

    private static String d() {
        d.b(5);
        int n = d.E(5);
        String string = "";
        if (n != -1) {
            string = string + 's' + i[n] + "_h" + (i[n + 20] >> 8 & 0xFF) + "_c" + (i[n + 20] & 0xFF) + "_x" + i[n] / 17 * (1 + (i[n + 20] >> 8 & 0xFF)) / (1 + (i[n + 20] & 0xFF));
        }
        return string;
    }

    private static int E(int n) {
        d.b(n);
        int n2 = 0;
        while (n2 < 10) {
            if ((i[n2 + 20] & 0x10000) != 0) {
                return n2;
            }
            ++n2;
        }
        return -1;
    }

    private static int F(int n) {
        int n2 = d.E(n);
        if (n2 != -1) {
            return i[n2];
        }
        return -1;
    }

    private static String d(int n) {
        int n2 = d.E(n);
        if (n2 != -1) {
            return b[n2];
        }
        return "";
    }

    private static String e() {
        return a[201] + ' ' + d.c();
    }

    private static String f() {
        return a[202];
    }

    private static void cv() {
        d.G();
    }

    private static void cw() {
        d.H();
    }

    public d() {
        super(false);
    }

    public d(S s) {
        super(false);
        a = s;
        a = this;
        this.setFullScreenMode(true);
        a = new Thread(this);
    }

    static {
        a = "IHDR".getBytes();
        b = "PLTE".getBytes();
        c = "tRNS".getBytes();
        d = "IEND".getBytes();
        e = "IDAT".getBytes();
        b = false;
        f = new byte[4096];
        g = new byte[89159];
        h = new byte[89159];
        a = new Image[1735];
        b = new int[1735];
        i = 0;
        j = 0;
        k = 100;
        d = false;
        a = Font.getFont((int)64, (int)1, (int)8);
        b = Font.getFont((int)64, (int)1, (int)0);
        c = Font.getFont((int)64, (int)1, (int)16);
        d = Font.getFont((int)0, (int)0, (int)8);
        m = -1;
        n = -1;
        a = null;
        c = new int[4];
        a = new String[219];
        e = new int[20];
        a = new boolean[111];
        b = new boolean[20];
        D = 0;
        k = false;
        a = new Object();
        f = new int[8];
        g = new int[8];
        h = new int[12];
        b = new short[1200];
        c = new short[200];
        d = new short[120];
        e = new short[200];
        j = new int[12];
    }
}

