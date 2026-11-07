# dal kipinin iki kolunu karşılaştır: python3 dal.py <klasör>  (once.jsonl, sonra.jsonl)
import json,sys,math
AD=['Aras','Deniz','Mert','Ece','Cansu','Kaan','Tuna','Sur','Burç','Kale','Siper','Hendek','Mancınık','Ok']
def d(a,b,c,e): return math.hypot(a-c,b-e)
def seg_d(px,py,ax,ay,bx,by):
  vx,vy=bx-ax,by-ay; L=vx*vx+vy*vy or 1e-9; u=max(0,min(1,((px-ax)*vx+(py-ay)*vy)/L)); return d(px,py,ax+u*vx,ay+u*vy)
for kol in ('once','sonra'):
  K=[json.loads(l) for l in open(f'{sys.argv[1]}/{kol}.jsonl')]
  s0=K[0]['s']; out=[]
  prev=None;lasth=-1
  for i,k in enumerate(K):
    p0=prev;prev=k
    if k['h']>=0: lasth=k['h']
    fl=k['fl']
    if fl and fl[0]=='gönder' and p0 and (not p0['fl'] or p0['fl'][1]!=fl[1]) and lasth>=0:
      sp=p0['p'][lasth]; T=0 if lasth<7 else 1; gx=120 if T==0 else 0
      hat=[AD[j] for j in range(14) if (j<7)!=(T==0) and j not in (0,7) and seg_d(p0['p'][j][0],p0['p'][j][1],sp[0],sp[1],gx,30)<2]
      nd=min(d(sp[0],sp[1],p0['p'][j][0],p0['p'][j][1]) for j in range(14) if (j<7)!=(T==0) and j not in (0,7))
      out.append(f"{k['t']} şut {AD[lasth]} ({sp[0]:.0f},{sp[1]:.0f}) kaleye {d(sp[0],sp[1],gx,30):.0f} br, en yakın savunmacı {nd:.1f} br, hatta {hat}")
    if k['s']!=s0: out.append(f"{k['t']} GOL skor {k['s']}"); s0=k['s']
  # S7h işi ve kutu ağzı: Liman savunurken KAL taşıyıcısı Liman kalesine <22 br iken en yakın Liman ve "ağız" işindeki oyuncu
  nd=[];agiz=0;n=0
  for k in K:
    h=k['h']
    if h>=7:
      hp=k['p'][h]
      if d(hp[0],hp[1],0,30)<22:
        n+=1; nd.append(min(d(hp[0],hp[1],k['p'][j][0],k['p'][j][1]) for j in range(1,7)))
        if any('ağız' in k['p'][j][6] or 'şut hattı' in k['p'][j][6] for j in range(1,7)): agiz+=1
  isler=set(k['p'][j][6] for k in K for j in range(1,7) if 'ağız' in k['p'][j][6])
  print(f"== {kol}: {len(K)} tik | KAL taşıyıcı Liman kalesine <22 br: {n} tik, en yakın Liman ort {sum(nd)/max(len(nd),1):.1f} br | yeni işler: {sorted(isler)[:4]}")
  for o in out: print('   ',o)
