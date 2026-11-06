from yukle import *
import os
from collections import Counter
KR=[json.loads(l) for l in open(os.path.join(S,'kararlar.txt'))]
def seg_d(px,py,ax,ay,bx,by):
  vx,vy=bx-ax,by-ay; L=vx*vx+vy*vy or 1e-9; u=max(0,min(1,((px-ax)*vx+(py-ay)*vy)/L)); return d(px,py,ax+u*vx,ay+u*vy)
print("=== KALEYE YAKIN KARARLAR (<=26 br) ===")
print("UYARI: 'en iyi şut' kararlar.txt'deki seçenek listesinden; liste motorda ilk 14 seçenekle kesik (dongu.rs take(14)): None, şut seçeneği üretilmedi demek DEĞİL")
for r in KR:
  t,ad,team,x,y,o4,hep=r
  gx=120 if team==0 else 0
  dk=d(x,y,gx,30)
  if dk>26: continue
  k=tk(t); i=AD.index(ad)
  bek=k['p'][7 if team==0 else 0]
  lane=[AD[j] for j in range(14) if TK[j]!=team and j not in (0,7) and seg_d(k['p'][j][0],k['p'][j][1],x,y,gx,30)<2.0]
  nr=min(d(x,y,k['p'][j][0],k['p'][j][1]) for j in range(14) if TK[j]!=team)
  g=[e for e in hep if e[0].startswith('gönder')]
  bestg=max([e[3] for e in g if e[3] is not None],default=None)
  # açık arkadaşlar
  ac=[]
  for j in range(14):
    if TK[j]!=team or j==i or j in (0,7): continue
    q=k['p'][j]; dq=d(q[0],q[1],gx,30)
    nq=min(d(q[0],q[1],k['p'][m][0],k['p'][m][1]) for m in range(14) if TK[m]!=team)
    ln=min(seg_d(k['p'][m][0],k['p'][m][1],x,y,q[0],q[1]) for m in range(14) if TK[m]!=team)
    if dq<dk-3 and nq>5 and ln>2.5: ac.append(f"{AD[j]}(kaleye {dq:.0f}, boş {nq:.0f})")
  ch=o4[0]
  print(f"{t:5d} {sn(t)} {ad:8s} kaleye {dk:4.1f} açı-y {y:4.1f} rakip {nr:4.1f} hatta {lane} Bekçi ({bek[0]:.0f},{bek[1]:.0f}) | seçim {ch[0][:22]:22s} {ch[1]:8s} v={ch[2]:.4f} | en iyi şut v={bestg if bestg is None else round(bestg,4)} | açık önde: {ac}")
