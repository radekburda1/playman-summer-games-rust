#!/usr/bin/env python3
"""Compare isolated Java world captures with the shared Rust software renderer."""
import argparse,json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('reference',type=Path)
a=p.parse_args()
root=Path(__file__).resolve().parent.parent
out=root/'analysis/hurdles'; rows=[]
for rust in sorted((out/'rust').glob('*.png')):
    java=a.reference/rust.name
    if not java.exists() or rust.name.startswith('asset-'):continue
    original=Image.open(java).convert('RGB');actual=Image.open(rust).convert('RGB')
    difference=ImageChops.difference(original,actual)
    count=sum(max(v)>4 for v in difference.get_flattened_data())
    rows.append({'frame':rust.name,'different_pixels_rgb_tolerance_4':count,'total_pixels':76800})
(out/'visual-comparison.json').write_text(json.dumps({'scope':'World only; original GO/prompt overlays are present in Java captures but omitted by the Rust world-only harness. No masks applied. GPU output and audio playback are not validated by this test.','frames':rows},indent=2)+'\n')
for name in ['clean-0','clean-80','wrong-160','boundary-160','dive-320']:
    java=a.reference/(name+'.png');rust=out/'rust'/(name+'.png')
    if not java.exists() or not rust.exists():continue
    original=Image.open(java).convert('RGB');actual=Image.open(rust).convert('RGB');diff=ImageChops.difference(original,actual)
    mask=Image.new('RGB',original.size);mask.putdata([(255,0,0) if max(v)>4 else (0,0,0) for v in diff.get_flattened_data()])
    sheet=Image.new('RGB',(720,340),'white');d=ImageDraw.Draw(sheet)
    for i,(im,title) in enumerate([(original,'Original Java'),(actual,'Rust shared world renderer'),(mask,'Difference >4 RGB')]):sheet.paste(im,(i*240,20));d.text((i*240+4,3),title,fill='black')
    sheet.resize((1440,680),Image.Resampling.NEAREST).save(out/('comparison-'+name+'.png'))
print('Compared',len(rows),'frames; median difference',sorted(r['different_pixels_rgb_tolerance_4'] for r in rows)[len(rows)//2])
print(sorted(rows,key=lambda r:r['different_pixels_rgb_tolerance_4'])[-8:])
