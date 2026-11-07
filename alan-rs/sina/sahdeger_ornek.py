#!/usr/bin/env python3
# ÖRNEK üretici (okuyucuyu sınamak için; asıl üretici analiz oturumunun). Biçim: kafa2_sah_deger.json (bicim 1).
# Hücre: kaleye uzaklık × topun önündeki rakip saha oyuncusu sayısı. Değer: bu durumda topu tutan takım N sn içinde gol atar mı.
#   python3 sina/sahdeger_ornek.py <N_sn> <analiz.jsonl>... > motor/src/kafa/kafa2_sah_deger.json
import json,sys,math
W,CY,TPS=120.0,30.0,30
N=float(sys.argv[1]); H=int(N*TPS)
U=list(range(0,66,6)); O=list(range(0,7))
n=[[0.0]*len(O) for _ in U]; g=[[0.0]*len(O) for _ in U]
mac=0
for f in sys.argv[2:]:
    R=[json.loads(l) for l in open(f)]
    if not R: continue
    mac+=1
    gol=[]; s0=list(R[0]['s'])
    for r in R:
        for tm in (0,1):
            if r['s'][tm]>s0[tm]: gol.append((r['t'],tm))
        s0=list(r['s'])
    for r in R:
        h=r['h']
        if h is None or h<0: continue
        tm=0 if h<7 else 1; gx=W if tm==0 else 0.0
        x,y=r['p'][h][0],r['p'][h][1]
        d=math.hypot(gx-x,CY-y)
        rak=range(8,14) if tm==0 else range(1,7)       # saha oyuncuları (Bekçi 0 ve 7 hariç)
        onde=sum(1 for q in rak if abs(gx-r['p'][q][0])<abs(gx-x))
        i=min(len(U)-1,int(round(d/6))); j=min(len(O)-1,onde)
        n[i][j]+=1; g[i][j]+=any(r['t']<t<=r['t']+H and t2==tm for t,t2 in gol)
V=[]
for i in range(len(U)):
    for j in range(len(O)):
        rr=0
        while True:
            NN=GG=0.0
            for a in range(max(0,i-rr),min(len(U),i+rr+1)):
                for b in range(max(0,j-rr),min(len(O),j+rr+1)):
                    NN+=n[a][b]; GG+=g[a][b]
            if NN>=300 or rr>4: break
            rr+=1
        V.append(round(GG/NN,4) if NN else 0.0)
print(json.dumps({"bicim":1,"motor":"ornek","N_sn":N,"mac":mac,
  "eksen":[{"ad":"uzaklik","birim":"br","dugum":U},{"ad":"onde","dugum":O}],
  "deger":V,"ornek":[int(n[i][j]) for i in range(len(U)) for j in range(len(O))]},ensure_ascii=False))
