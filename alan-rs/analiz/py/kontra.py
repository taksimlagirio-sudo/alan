import json,math,glob,sys
from collections import defaultdict
fs=sorted(set(glob.glob('analiz/out/*/t*/analiz.jsonl')+glob.glob('analiz/out/izle-hepsi/*/t*/analiz.jsonl')+glob.glob('analiz/out/izle1/analiz.jsonl')))
TK=lambda i:0 if i<7 else 1
kova=defaultdict(lambda:[0,0,0]); geri=defaultdict(lambda:[0,0]); n_k=0
bar_min=[];bar_son=[];hiz_kont=[]
for f in fs:
  K=[json.loads(l) for l in open(f)]
  son=None
  for i,k in enumerate(K):
    h=k['h']
    if h<0 or h in (0,7): 
      continue
    T=TK(h)
    if son is not None and son!=T:
      gx=120 if T==0 else 0; hp=k['p'][h]
      if abs(hp[0]-gx)>50:
        # rakip saha oyuncularından topun kale tarafında olan
        D=sum(1 for j in range(14) if TK(j)!=T and j not in (0,7) and abs(k['p'][j][0]-gx)<abs(hp[0]-gx))
        if D<=3:
          n_k+=1; s0=k['s'][T]; t25=None; gol=False; Dv=None
          for k2 in K[i:i+360]:
            if k2['s'][T]>s0: gol=True; break
            h2=k2['h']
            if h2>=0 and TK(h2)!=T and h2 not in (0,7): break
            if t25 is None and h2>=0 and TK(h2)==T and math.hypot(k2['p'][h2][0]-gx,k2['p'][h2][1]-30)<25:
              t25=(k2['t']-k['t'])/30; hp2=k2['p'][h2]
              Dv=sum(1 for j in range(14) if TK(j)!=T and j not in (0,7) and math.hypot(k2['p'][j][0]-gx,k2['p'][j][1]-30)<math.hypot(hp2[0]-gx,hp2[1]-30))
          b='ulaşmadı' if t25 is None else '<5 sn' if t25<5 else '5–8 sn' if t25<8 else '8+ sn'
          kova[b][0]+=1; kova[b][1]+=gol
          if Dv is not None: geri[Dv][0]+=1; geri[Dv][1]+=gol
    son=T
  for j in range(14):
    if j in (0,7): continue
    for a,b in ((0,4499),(4500,8999)):
      e=[K[t]['p'][j][7] for t in range(a,b,15)]
      bar_min.append(min(e)); bar_son.append(K[b]['p'][j][7])
print(f"{len(fs)} maç, {n_k} kontra (top rakip kaleye 50+ br'de kazanıldı, topun önünde en çok 3 rakip saha oyuncusu)")
for b in ('<5 sn','5–8 sn','8+ sn','ulaşmadı'):
  n,g,_=kova[b]; print(f"  kaleye 25 br'ye {b:9s}: {n:4d} kontra, 12 sn içinde gol %{100*g/max(n,1):.0f}")
print("  25 br'ye girişte topun önündeki savunmacı sayısı → gol:")
for D in sorted(geri): n,g=geri[D]; print(f"     {D} savunmacı: {n:3d} kez, gol %{100*g/max(n,1):.0f}")
import statistics as st
print(f"oyuncu-devre ({len(bar_min)}): en düşük bar medyan {st.median(bar_min):.2f}, <0.3'e inen %{100*sum(x<0.3 for x in bar_min)/len(bar_min):.0f}, <0.5 %{100*sum(x<0.5 for x in bar_min)/len(bar_min):.0f}; devre sonu bar medyan {st.median(bar_son):.2f}")
