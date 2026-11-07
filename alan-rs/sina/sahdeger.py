#!/usr/bin/env python3
# Sahiplik değeri tablosu: motorun kendi maçlarından (alan-kos ... analiz çıktıları), topa sahip takımın her tikte
# bulunduğu bölgede "bu sahiplik golle biter mi" oranı. Bölge: rakip kaleye boyuna uzaklık (6 br) × kale ekseninden
# yana uzaklık (6 br, simetrik). Seyrek hücreler komşularıyla düzlenir (yalnız bu tablonun kendi verisi).
#   python3 sina/sahdeger.py <analiz.jsonl>... > motor/src/kafa/kafa2_sah_deger.rs
import json,sys,math
W,CY,KG=120.0,30.0,6.0
NX,NY=int(W/KG)+1,int(CY/KG)+1
n=[[0.0]*NY for _ in range(NX)]; g=[[0.0]*NY for _ in range(NX)]
mac=0
for f in sys.argv[1:]:
    R=[json.loads(l) for l in open(f)]
    if not R: continue
    mac+=1
    sah=None; parca=[]   # bu sahipliğin (takım, i, j) tikleri
    s0=list(R[0]['s'])
    def kapat(gol):
        for (t,i,j) in parca:
            n[i][j]+=1; g[i][j]+=gol
        parca.clear()
    for r in R:
        s=r['s']
        if sah is not None and s[sah]>s0[sah]:
            kapat(1.0); sah=None
        elif s!=s0:
            kapat(0.0); sah=None
        s0=list(s)
        h=r['h']
        if h is not None and h>=0:
            tm=0 if h<7 else 1
            if sah is not None and tm!=sah: kapat(0.0)
            sah=tm
            x,y=r['p'][h][0],r['p'][h][1]
        elif r['b'] and sah is not None:
            x,y=r['b'][0],r['b'][1]
        else:
            continue
        dx=(W-x) if sah==0 else x
        i=min(NX-1,max(0,int(round(dx/KG)))); j=min(NY-1,max(0,int(round(abs(y-CY)/KG))))
        parca.append((sah,i,j))
    kapat(0.0)
# seyrek hücre: komşularla düzle (en az 300 tik olana dek halka genişler)
V=[[0.0]*NY for _ in range(NX)]
for i in range(NX):
    for j in range(NY):
        r=0
        while True:
            N=G=0.0
            for a in range(max(0,i-r),min(NX,i+r+1)):
                for b in range(max(0,j-r),min(NY,j+r+1)):
                    N+=n[a][b]; G+=g[a][b]
            if N>=300 or r>4: break
            r+=1
        V[i][j]=G/N if N>0 else 0.0
print(f"// Üretildi: sina/sahdeger.py, {mac} maç. Satır: rakip kaleye boyuna uzaklık {KG:.0f} br adımla; sütun: kale ekseninden yana uzaklık.")
print(f"pub const SAH_DEGER_KG: f64 = {KG};")
print(f"pub const SAH_DEGER: [[f64; {NY}]; {NX}] = [")
for i in range(NX): print("    ["+", ".join(f"{v:.4f}" for v in V[i])+"],")
print("];")
