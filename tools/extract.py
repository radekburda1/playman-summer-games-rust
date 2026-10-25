#!/usr/bin/env python3
"""Decode the supplied JAR's raw sprite banks; standard library only."""
import argparse, json, struct, zipfile, zlib
from pathlib import Path

def chunk(tag, data):
    return struct.pack('>I', len(data)) + tag + data + struct.pack('>I', zlib.crc32(tag + data))

def png(w, h, rgba):
    rows = b''.join(b'\0' + rgba[y*w*4:(y+1)*w*4] for y in range(h))
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', w,h,8,6,0,0,0)) + chunk(b'IDAT', zlib.compress(rows)) + chunk(b'IEND', b'')

def extract(jar, out):
    out.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(jar) as z:
        p=z.read('p'); pos=1; palettes=[]
        for _ in range(p[0]):
            count=p[pos]; pos+=1; pal=[(0,0,0,0)]
            for _ in range(1,count):
                c=struct.unpack_from('>H',p,pos)[0]; pos+=2
                r=(c>>11)&31; g=(c>>5)&63; b=c&31
                pal.append(((r<<3)+(r>>2),(g<<2)+(g>>4),(b<<3)+(b>>2),255))
            palettes.append(pal)
        info=z.read('i'); offsets=struct.unpack_from('>1778h',info,938)
        sprites=[]; metadata=[]
        for bank in range(18):
            data=z.read(f'g{bank}'); pos=0; idx=offsets[355+bank]
            while pos<len(data):
                w,h,pal=struct.unpack_from('>HHB',data,pos); pos+=5
                size=w*h*(4 if pal==255 else 1); raw=data[pos:pos+size]; pos+=size
                assert len(raw)==size and w*h>0
                if pal==255:
                    rgba=bytes(v for i in range(0,size,4) for v in (raw[i+2],raw[i+1],raw[i],raw[i+3]))
                else:
                    rgba=bytes(v for c in raw for v in palettes[pal][c])
                transforms=data[pos]&15; pos+=1
                variants=data[pos:pos+transforms]; pos+=transforms
                sprites.append((idx,w,h,rgba)); metadata.append({'id':idx,'bank':bank,'w':w,'h':h,'variants':list(variants)})
                for vi, transform in enumerate(variants):
                    tw,th=(h,w) if transform in (1,3,6,7) else (w,h)
                    pixels=bytearray(tw*th*4)
                    for sy in range(h):
                        for sx in range(w):
                            tx,ty={0:(sx,sy),4:(w-1-sx,sy),5:(sx,h-1-sy),2:(w-1-sx,h-1-sy),6:(sy,sx),1:(h-1-sy,sx),3:(sy,w-1-sx),7:(h-1-sy,w-1-sx)}[transform]
                            pixels[(ty*tw+tx)*4:(ty*tw+tx+1)*4]=rgba[(sy*w+sx)*4:(sy*w+sx+1)*4]
                    sprites.append((idx+vi+1,tw,th,bytes(pixels)))
                idx+=1+transforms
            assert idx==offsets[356+bank], (bank,idx,offsets[356+bank])
        width=2048; x=y=row_h=0; rects={}
        for idx,w,h,rgba in sprites:
            if x+w>width: x=0; y+=row_h+2; row_h=0
            rects[idx]=(x,y,w,h); x+=w+2; row_h=max(row_h,h)
        height=y+row_h; atlas=bytearray(width*height*4)
        for idx,w,h,rgba in sprites:
            x,y,_,_=rects[idx]
            for r in range(h): atlas[((y+r)*width+x)*4:((y+r)*width+x+w)*4]=rgba[r*w*4:(r+1)*w*4]
        (out/'atlas.png').write_bytes(png(width,height,atlas))
        (out/'sprites.json').write_text(json.dumps(metadata,indent=2))
        (out/'rects.rs').write_text('pub const SPRITES: &[(usize, [f32; 4])] = &[\n'+''.join(f'    ({i}, [{x}., {y}., {w}., {h}.]),\n' for i,(x,y,w,h) in sorted(rects.items()))+'];\n' + 'pub const TRANSFORMS: &[(usize, u8, usize)] = &[\n' + ''.join(f"    ({m['id']}, {t}, {m['id']+j+1}),\n" for m in metadata for j,t in enumerate(m['variants'])) + '];\n')
        (out/'logical.rs').write_text('pub const LOGICAL: &[usize] = &'+str(list(offsets[374:374+len(metadata)])).replace('[','[').replace(']',']')+';\n')
        sounds=z.read('s'); integers=struct.unpack_from('>324i',info,938+1778*2)
        (out/'sounds').mkdir(exist_ok=True)
        for sound in range(27):
            offset=integers[3+sound]; length=integers[65+sound]
            extension='mid' if info[907+sound]==1 else 'wav'
            (out/'sounds'/f'{sound}.{extension}').write_bytes(sounds[offset:offset+length])
        (out/'animation.bin').write_bytes(z.read('k0'))
        for bank in (1,2): (out/f'scene{bank}.bin').write_bytes(z.read(f'k{bank}'))
        (out/'tables.bin').write_bytes(z.read('b0'))
        strings=[]; data=z.read('l'); pos=0
        while pos<len(data):
            n=struct.unpack_from('>H',data,pos)[0]; pos+=2; strings.append(data[pos:pos+n].decode('utf8','replace')); pos+=n
        (out/'strings.json').write_text(json.dumps(strings,indent=2,ensure_ascii=False))
        from extract_fonts import extract as extract_fonts
        extract_fonts(info,out/'fonts.rs')
        print(f'Decoded {len(metadata)} base sprites ({len(sprites)} including transforms), {width}x{height} atlas; all 18 bank boundaries validated.')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('jar',type=Path);p.add_argument('--out',type=Path,default=Path(__file__).resolve().parent.parent/'assets'); a=p.parse_args();extract(a.jar,a.out)
