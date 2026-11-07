# dalarka.sh özeti: kol başına rakip topta tik, en geri savunmacının arkasında rakip oranı (hat.py tanımı), gol; geçersiz çatal sayısı.
import json,sys,glob,math,os
from collections import defaultdict
kl=sys.argv[1]
def arka(r):
    h=r['h']
    if h is None or h<0: return None
    tm=0 if h<7 else 1; dt=1-tm; gx=0.0 if dt==0 else 120.0
    dd=min(math.hypot(r['p'][i][0]-gx,r['p'][i][1]-30) for i in range(dt*7+1,dt*7+7))
    da=min(math.hypot(r['p'][i][0]-gx,r['p'][i][1]-30) for i in range(tm*7+1,tm*7+7) if i!=h)
    return da<dd
T=defaultdict(lambda:[0,0,0]); gec=0; top=0; tab={}
for d in sorted(glob.glob(kl+'/d-*')):
    s,t=d.split('/d-')[-1].split('-'); t=int(t)
    if s not in tab: tab[s]=[l for l in open(f"{kl}/taban-{s}.jsonl")]
    top+=1
    O=open(d+'/once.jsonl').read().splitlines()
    if O!=[x.rstrip('\n') for x in tab[s][t:t+len(O)]]: gec+=1; continue
    for f in glob.glob(d+'/*.jsonl'):
        k=os.path.basename(f)[:-6]; R=[json.loads(l) for l in open(f)]
        for r in R:
            a=arka(r)
            if a is None: continue
            T[k][0]+=1; T[k][1]+=a
        T[k][2]+=sum(R[-1]['s'])-sum(R[0]['s']) if R else 0
print(f"{top} çatal, geçersiz (once taban maçtan ayrıştı) {gec}")
for k,(n,a,g) in sorted(T.items()): print(f"  {k:10s} rakip topta {n} tik, en geri savunmacının arkasında %{100*a/max(1,n):.2f}, gol {g}")
