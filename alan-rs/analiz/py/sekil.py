from yukle import *
from collections import Counter,defaultdict
# sahiplik sırasında takım şekli
for takim in (0,1):
  out=[i for i in range(14) if TK[i]==takim and i not in (0,7)]
  L=[];kopuk=Counter();n=0;bagsiz=0;yakin_destek=[]
  for k in K:
    h=k['h']
    if h<0 or TK[h]!=takim: continue
    n+=1
    hx,hy=k['p'][h][0],k['p'][h][1]
    xs=[k['p'][i][0] for i in out]
    L.append(max(xs)-min(xs))
    far=[i for i in out if i!=h and d(k['p'][i][0],k['p'][i][1],hx,hy)>50]
    for i in far: kopuk[AD[i]]+=1
    near=sum(1 for i in out if i!=h and d(k['p'][i][0],k['p'][i][1],hx,hy)<15)
    yakin_destek.append(near)
    if near==0: bagsiz+=1
  print(['Liman','Kale Duvarı'][takim],'topta',n,'tik; takım boyu ort %.0f maks %.0f'%(sum(L)/n,max(L)),'| taşıyıcıya 15 br içinde arkadaş yok: %%%d'%(100*bagsiz/n), '| ort yakın destek %.2f'%(sum(yakin_destek)/n))
  print('   taşıyıcıdan 50+ br uzakta kalma (tik):',dict(kopuk.most_common()))
