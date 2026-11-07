import json,sys,glob
# gidip.py <desen>: savunmacının işi 30 tik içinde markaj ↔ geri koşu arasında gidip geliyor (A → B → A) sayısı
for pat in sys.argv[1:]:
    n=0; mac=0
    for f in sorted(glob.glob(pat)):
        mac+=1; son={}
        for l in open(f):
            r=json.loads(l); t=r['t']
            for i,p in enumerate(r['p']):
                j=p[6]; k='m' if j.startswith(('markaj','arkada','kapatma')) else 'g' if j.startswith('geri koşu') else None
                if not k: continue
                h=son.setdefault(i,[])
                if not h or h[-1][0]!=k:
                    h.append((k,t))
                    if len(h)>=3 and h[-3][0]==k and t-h[-3][1]<=30: n+=1
                    del h[:-3]
    print(f"{pat.split('/')[-1]}: {mac} maç, markaj ↔ geri koşu gidip gelme (30 tikte A→B→A) {n} ({n/max(1,mac):.1f}/maç)")
