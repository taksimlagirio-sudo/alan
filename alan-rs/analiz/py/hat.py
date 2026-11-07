import json,sys,math,glob
for pat in sys.argv[1:]:
    n=arka=cik=0; gol=[0,0]
    for f in sorted(glob.glob(pat)):
        s=None
        for l in open(f):
            r=json.loads(l); h=r['h']; s=r['s']
            if h is None or h<0: continue
            tm=0 if h<7 else 1; dt=1-tm; gx=0.0 if dt==0 else 120.0
            D=range(dt*7+1,dt*7+7); A=[i for i in range(tm*7+1,tm*7+7) if i!=h]
            dd=min(math.hypot(r['p'][i][0]-gx,r['p'][i][1]-30) for i in D)
            da=min(math.hypot(r['p'][i][0]-gx,r['p'][i][1]-30) for i in A)
            n+=1; arka+= da<dd; cik+= sum(1 for i in D if 'öne çık' in r['p'][i][6])>0
        gol[0]+=s[0]; gol[1]+=s[1]
    print(pat.split('/')[-1], f"rakip topta {n} tik: en geri savunmacının arkasında rakip %{100*arka/n:.1f}; 'öne çık' olan tik %{100*cik/n:.1f}; goller {gol}")
