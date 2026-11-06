from yukle import *
from collections import Counter,defaultdict
def seg_d(px,py,ax,ay,bx,by):
  vx,vy=bx-ax,by-ay; L=vx*vx+vy*vy or 1e-9; u=max(0,min(1,((px-ax)*vx+(py-ay)*vy)/L)); return d(px,py,ax+u*vx,ay+u*vy)
st=defaultdict(Counter); hic_acik=Counter(); n_t=Counter()
for k in K:
  h=k['h']
  if h<0 or h in (0,7): continue
  T=TK[h]; hp=k['p'][h]; g=1 if T==0 else -1
  n_t[T]+=1; acik_say=0
  for i in range(14):
    if TK[i]!=T or i==h or i in (0,7): continue
    p=k['p'][i]; s=st[i]; s['n']+=1
    dh=d(p[0],p[1],hp[0],hp[1]); s['dh']+=dh
    sp=math.hypot(p[2],p[3])
    if sp<0.03: s['duruyor']+=1
    if p[2]*g>0.15: s['ileri koşu']+=1
    if p[2]*g<-0.15: s['geri koşu']+=1
    nr=min(d(p[0],p[1],k['p'][j][0],k['p'][j][1]) for j in range(14) if TK[j]!=T)
    ln=min(seg_d(k['p'][j][0],k['p'][j][1],hp[0],hp[1],p[0],p[1]) for j in range(14) if TK[j]!=T and j not in (0,7))
    if nr<3: s['sıkı markajda(<3)']+=1
    if 6<=dh<=40 and nr>5 and ln>2.5:
      s['açık pas seçeneği']+=1; acik_say+=1
      if (p[0]-hp[0])*g>5: s['açık ve önde']+=1
    if dh>60: s['60+ uzak']+=1
    if p[1]<3 or p[1]>57: s['çizgide']+=1
    gx=120 if T==0 else 0
    if d(p[0],p[1],gx,30)<20: s['rakip kaleye<20']+=1
  if acik_say==0: hic_acik[T]+=1
for T in (0,1):
  print(['LİMAN','KALE DUVARI'][T],'topta',n_t[T],'tik; hiç açık pas seçeneği olmayan tik: %%%d'%(100*hic_acik[T]/n_t[T]))
  for i in range(14):
    if TK[i]!=T or i in (0,7): continue
    s=st[i]; n=s['n']
    print(f"  {AD[i]:9s} {ROL[i]:5s} taşıyıcıya ort {s['dh']/n:5.1f} | duruyor %{100*s['duruyor']/n:3.0f} | ileri %{100*s['ileri koşu']/n:3.0f} geri %{100*s['geri koşu']/n:3.0f} | açık %{100*s['açık pas seçeneği']/n:3.0f} (önde %{100*s['açık ve önde']/n:3.0f}) | sıkı markaj %{100*s['sıkı markajda(<3)']/n:3.0f} | 60+uzak %{100*s['60+ uzak']/n:3.0f} | kaleye<20 %{100*s['rakip kaleye<20']/n:3.0f} | çizgi %{100*s['çizgide']/n:3.0f}")
