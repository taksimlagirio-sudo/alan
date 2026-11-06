from yukle import *
from collections import Counter,defaultdict
# Rakip topa sahipken savunan takımın baskısı
for takim in (0,1):
  rows=[]
  for k in K:
    h=k['h']
    if h<0 or TK[h]==takim: continue
    hp=k['p'][h]
    ds=sorted((d(hp[0],hp[1],k['p'][j][0],k['p'][j][1]),j) for j in range(14) if TK[j]==takim and j!=takim*7)
    jobs=[k['p'][j][6] for j in range(14) if TK[j]==takim]
    npres=sum(1 for j in jobs if j.startswith('pres') or j.startswith('kovala') or j.startswith('kapatma'))
    rows.append((k['t'],h,hp[0],hp[1],ds[0][0],ds[1][0],npres,AD[ds[0][1]]))
  print('SAVUNAN', ['Liman','Kale Duvarı'][takim], 'rakip topta tik:',len(rows))
  b=Counter()
  for r in rows:
    b['<3' if r[4]<3 else '<6' if r[4]<6 else '<10' if r[4]<10 else '<15' if r[4]<15 else '15+']+=1
  print(' en yakın savunmacı uzaklığı dağılımı', dict(b))
  print(' pres işi olan oyuncu sayısı dağılımı', Counter(r[6] for r in rows))
  # uzun baskısız dönemler
  run=None
  for r in rows+[(10**9,-1,0,0,0,0,0,'')]:
    if r[4]>10 and (run is None or r[0]==run[1]+1):
      run=[r[0],r[0],r] if run is None else [run[0],r[0],run[2]]
    else:
      if run and run[1]-run[0]>=60: print(f'  baskısız {run[0]}-{run[1]} ({(run[1]-run[0])/30:.1f}sn) taşıyıcı {AD[run[2][1]]} @({run[2][2]:.0f},{run[2][3]:.0f})')
      run=[r[0],r[0],r] if r[4]>10 else None
