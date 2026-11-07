import json,math,sys
AD=['Aras','Deniz','Mert','Ece','Cansu','Kaan','Tuna','Sur','Burç','Kale','Siper','Hendek','Mancınık','Ok']
D=sys.argv[1]; t0,t1,st=int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]); kim=sys.argv[5].split(',') if len(sys.argv)>5 else None
K={json.loads(l)['t']:json.loads(l) for l in open(D+'/analiz.jsonl')}
R={}
for l in open(D+'/kararlar.txt'):
  r=json.loads(l); R.setdefault(r[0],[]).append(r)
for t in range(t0,t1+1,st):
  k=K.get(t)
  if not k: continue
  h=k['h']; s=f"{t} ({t//30//60}:{t//30%60:02d}) top {AD[h] if h>=0 else '—'}"
  if h>=0: s+=f"({k['p'][h][0]:.0f},{k['p'][h][1]:.0f})"
  print(s)
  for j in range(14):
    if kim and AD[j] not in kim: continue
    p=k['p'][j]; print(f"     {AD[j]:8s}({p[0]:5.1f},{p[1]:4.1f}) v{math.hypot(p[2],p[3]):.2f} hedef({p[4]:.0f},{p[5]:.0f}) bar{p[7]:.2f} {p[6][:30]}")
  for tt in range(t-st+1,t+1):
    for r in R.get(tt,[]): print(f"     KARAR {tt} {r[1]}: "+' | '.join(f"{str(o[0])[:14]} {o[2]:.3f}" for o in r[5][:4] if o[2] is not None))
