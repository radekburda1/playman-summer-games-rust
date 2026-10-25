#!/usr/bin/env python3
"""Extract original bitmap-font metrics from the JAR's i resource."""
import struct,zipfile,sys
from pathlib import Path
def extract(data,out):
 a=struct.unpack_from('>1778h',data,938)
 signed=lambda n:n-256 if n>127 else n
 lines=['// Generated from the original bitmap-font tables.','pub static FONTS: &[BitmapFont] = &[']
 for n in range(4):
  start,end=a[45+n:47+n]
  lines.append(f'    BitmapFont {{ height: {data[903+n]}, glyphs: &[')
  for i,c in enumerate(a[50+start:50+end]):
   sprite=a[1477+a[1472+n]+i]-1
   advance=signed(data[a[30+n]+i]);x=signed(data[301+a[35+n]+i]);y=signed(data[602+a[40+n]+i])
   lines.append(f'        Glyph {{ code: {c}, sprite: {sprite}, advance: {advance}, x: {x}, y: {y} }},')
  lines.append('    ] },')
 lines.append('];');out.write_text('\n'.join(lines)+'\n')
if __name__=='__main__':
 with zipfile.ZipFile(sys.argv[1])as z:extract(z.read('i'),Path(__file__).resolve().parent.parent/'assets/fonts.rs')
