# Baskısız sürüş: aynı taşıyıcı (Bekçi dışı) en az 90 tik, en yakın rakip hep 12 br'den uzak. Dönem başına: taşıyıcının
# kendi kalesinden uzaklığı (x0 → x1), en yakın rakip (ort), taşıyıcıyla rakip kalesi arasında (kale tarafında) en yakın savunmacı ve işi.
#   python3 analiz/py/baskisiz.py "<desen>" [ayrıntı]
import json,sys,glob,math
AD=['Aras','Deniz','Mert','Ece','Cansu','Kaan','Tuna','Sur','Burç','Kale','Siper','Hendek','Mancınık','Ok']
ay=len(sys.argv)>2; top=0; nm=0; bol={'kendi yarı':0,'rakip yarı':0}
for f in sorted(glob.glob(sys.argv[1])):
    R=[json.loads(l) for l in open(f)]; nm+=1; c=None; ep=[]
    def bit():
        if c and c['n']>=90: ep.append(dict(c))
    for i,r in enumerate(R):
        h=r['h']
        ok=h is not None and h>=0 and h not in (0,7)
        if ok:
            hp=r['p'][h]; rk=range(8,14) if h<7 else range(1,7)
            dm=min(math.hypot(r['p'][q][0]-hp[0],r['p'][q][1]-hp[1]) for q in rk)
            ok=dm>12
        if ok and c and c['h']==h: c['n']+=1; c['dm']+=dm; c['i1']=i
        else:
            bit(); c={'h':h,'i0':i,'i1':i,'n':1,'dm':dm} if ok else None
    bit()
    for e in ep:
        top+=1; r0,r1=R[e['i0']],R[e['i1']]; h=e['h']; gx=120.0 if h<7 else 0.0
        u0=abs(gx-r0['p'][h][0]); u1=abs(gx-r1['p'][h][0]); bol['rakip yarı' if u0<60 else 'kendi yarı']+=1
        if ay:
            rk=range(8,14) if h<7 else range(1,7); hp=r0['p'][h]
            ks=[q for q in rk if abs(gx-r0['p'][q][0])<abs(gx-hp[0])]
            q=min(ks,key=lambda q:math.hypot(r0['p'][q][0]-hp[0],r0['p'][q][1]-hp[1])) if ks else None
            print(f"{f.split('/')[-1]} {r0['t']}–{r1['t']} {AD[h]} rakip kaleye {u0:.0f}→{u1:.0f} br, en yakın rakip ort {e['dm']/e['n']:.0f}; kale tarafında en yakın: "+(f"{AD[q]} {math.hypot(r0['p'][q][0]-hp[0],r0['p'][q][1]-hp[1]):.0f} br '{r0['p'][q][6]}'" if q is not None else 'yok'))
print(sys.argv[1].split('/')[-1], f"{nm} maç: baskısız sürüş (90 tik+, en yakın rakip >12 br) {top} ({top/max(1,nm):.1f}/maç)", bol)
