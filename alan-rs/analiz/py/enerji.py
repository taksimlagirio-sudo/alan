# Oyuncunun topsuz koşusu ve enerjisi: rakip topta iken (takımı savunurken) koşulan yol, hız dağılımı, bar; kontra başında bar.
# Kullanım: ANALIZ_OUT=<out> python3 enerji.py [oyuncu adı...]
import os,sys,math
sys.path.insert(0,os.path.dirname(__file__))
from yukle import *
from collections import Counter
adlar=sys.argv[1:] or AD[1:7]+AD[8:14]
for ad in adlar:
  i=AD.index(ad); T=TK[i]
  yol=0;n=0;hiz=Counter();bar=[];yon_deg=0;son_v=None;kontra_bar=[];is_=Counter()
  prevT=None
  for t in range(1,len(K)):
    k=K[t]; q=K[t-1]; h=k['h']
    p=k['p'][i]; pq=q['p'][i]
    if h>=0 and TK[h]!=T:            # rakip topta
      n+=1; s=math.hypot(p[0]-pq[0],p[1]-pq[1]); yol+=s
      hiz['dur (<0.03)' if s<0.03 else 'yürü (<0.15)' if s<0.15 else 'koş (<0.30)' if s<0.30 else 'sprint (0.30+)']+=1
      if p[7] is not None: bar.append(p[7])
      v=(p[2],p[3])
      if son_v and math.hypot(*v)>0.1 and math.hypot(*son_v)>0.1 and (v[0]*son_v[0]+v[1]*son_v[1])/(math.hypot(*v)*math.hypot(*son_v))<0: yon_deg+=1
      if t%6==0: son_v=v
      is_[p[6].split(' · ')[0]]+=1
    if h>=0:
      if TK[h]==T and prevT is not None and prevT!=T and p[7] is not None: kontra_bar.append(p[7])
      prevT=TK[h]
  if not n: continue
  tot=sum(hiz.values())
  print(f"{ad:9s} rakip topta {n/30:5.0f} sn: yol {yol:5.0f} br ({yol/(n/30):.1f} br/sn) | "+' '.join(f"{a} %{100*b/tot:.0f}" for a,b in hiz.most_common())+f" | yön değiştirme {yon_deg} | ort bar {sum(bar)/max(len(bar),1):.2f} | top kazanılınca bar ort {sum(kontra_bar)/max(len(kontra_bar),1):.2f} (n={len(kontra_bar)}) | iş: {is_.most_common(2)}")
