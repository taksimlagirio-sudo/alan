from yukle import *
import os
from collections import defaultdict
KR=[json.loads(l) for l in open(os.path.join(S,'kararlar.txt'))]
kr={r[0]:r for r in KR}
prev=None
for i,k in enumerate(K):
  fl=k['fl']
  if fl and fl[0]=='gönder' and (prev is None or prev['fl'] is None or prev['fl'][1]!=fl[1]):
    t=k['t']; t0=int(fl[1])
    # shooter = holder at t0-1
    kk=tk(max(1,t0-1)) ; sh=kk['h'] if kk['h']>=0 else prev['h']
    sp=kk['p'][sh]; team=TK[sh]; gx=120 if team==0 else 0
    dist=d(sp[0],sp[1],gx,30)
    bek=kk['p'][7 if team==0 else 0]
    # defenders in lane (within 2.5 of segment shooter->goal center)
    def seg_d(px,py,ax,ay,bx,by):
      vx,vy=bx-ax,by-ay; L=vx*vx+vy*vy; u=max(0,min(1,((px-ax)*vx+(py-ay)*vy)/L)); return d(px,py,ax+u*vx,ay+u*vy)
    lane=[AD[j] for j in range(14) if TK[j]!=team and j not in (0,7) and seg_d(kk['p'][j][0],kk['p'][j][1],sp[0],sp[1],gx,30)<2.5]
    # decision
    dec=None
    for tt in range(t0-40,t0+1):
      if tt in kr and kr[tt][1]==AD[sh]: dec=kr[tt]
    # follow trajectory
    path=[];j=i
    while j<len(K) and K[j]['fl'] and K[j]['fl'][1]==fl[1]:
      path.append(K[j]); j+=1
    end=K[min(j,len(K)-1)]
    evs=[e for x in K[i-2:j+3] for e in x['ev']]
    nb=[(x['t'],x['b'][0],x['b'][1]) for x in path[::4]]
    print(f"\n== {t0} {sn(t0)} {AD[sh]} ({['LMN','KAL'][team]}) yer ({sp[0]:.1f},{sp[1]:.1f}) kaleye {dist:.1f} | Bekçi ({bek[0]:.1f},{bek[1]:.1f}) | hatta: {lane} | to={fl[3]} shot={fl[4]}")
    if dec: print('  karar ilk 4:', [(o[0],o[1],round(o[2],4) if o[2] else o[2],o[3]) for o in dec[5][:4]])
    print('  yol:', nb[:14])
    print('  son:', end['t'], 'sahip', AD[end['h']] if end['h']>=0 else '-', 'top', (end['b'] or [None,None])[:2], 'skor', end['s'])
    print('  olay:', [e[1] for e in evs][:12])
  prev=k
