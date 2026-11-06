# Savunmada oyuncu paylaşımı: rakip topta iken (rakip yarımızda, taşıyıcı kalemize <60 br) her tik
#  - her rakip hücumcunun (taşıyıcı dışı) "tutanı": işi onun adını taşıyan savunmacılar (markaj/arkada/kapatma/yolu kesiyor · AD)
#  - çift tutulan (2+), tutulmayan ama tehlikeli (kalemize <30 br ve en yakın savunmacı >6 br)
#  - taşıyıcıya giden (pres/kovala) sayısı, işsiz/bölge savunmacı sayısı
import os,sys,re
sys.path.insert(0,os.path.dirname(__file__))
from yukle import *
from collections import Counter
for T,n in ((0,'Liman'),(1,'KaleDuvari')):
  ox=0 if T==0 else 120
  sav=[i for i in range(14) if TK[i]==T and i not in (0,7)]
  rak=[i for i in range(14) if TK[i]!=T and i not in (0,7)]
  N=0;cift=0;bos_teh=0;pres_n=Counter();bolge=0;ornek=[];adam_tutulan=Counter()
  for k in K:
    h=k['h']
    if h<0 or TK[h]==T: continue
    hp=k['p'][h]
    if d(hp[0],hp[1],ox,30)>60: continue
    N+=1
    tut=Counter()
    for i in sav:
      j=k['p'][i][6]
      for q in rak:
        if q!=h and (' · '+AD[q]) in j and not j.startswith('pres'): tut[q]+=1
    cift+=sum(1 for q in tut if tut[q]>=2)
    for q in rak:
      if q==h: continue
      p=k['p'][q]
      if d(p[0],p[1],ox,30)<30:
        nd=min(d(p[0],p[1],k['p'][i][0],k['p'][i][1]) for i in sav)
        if tut[q]==0 and nd>6:
          bos_teh+=1
          if len(ornek)<6 and (not ornek or k['t']-ornek[-1][0]>150): ornek.append((k['t'],sn(k['t']),AD[q],(round(p[0]),round(p[1])),round(nd,1)))
    pres_n[sum(1 for i in sav if k['p'][i][6].startswith(('pres','kovala')))]+=1
    bolge+=sum(1 for i in sav if k['p'][i][6].startswith(('bölge','arkada · alan','güvence')))
  print(f"{n} savunurken (taşıyıcı kalemize <60 br) {N} tik:")
  print(f"   çift tutulan adam (tik·adam): {cift}  (%{100*cift/max(N,1):.0f} tik başına)")
  print(f"   kalemize <30 br'de tutulmayan ve 6 br'den uzak rakip (tik·adam): {bos_teh}  (tik başına {bos_teh/max(N,1):.2f})  örnek: {ornek}")
  print(f"   taşıyıcıya giden savunmacı sayısı dağılımı: {dict(sorted(pres_n.items()))}")
  print(f"   bölge/alan/güvence işindeki savunmacı ort: {bolge/max(N,1):.2f}")
