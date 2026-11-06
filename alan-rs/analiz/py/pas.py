from yukle import *
from collections import Counter,defaultdict
# uçuşlar: başlangıç (fl t0 değişimi), atan = önceki sahip, sonuç = sonraki sahip
ucus=[];prev=None;lasth=-1
for i,k in enumerate(K):
  if k['h']>=0: lasth=k['h']
  fl=k['fl']
  if fl and (prev is None or not prev['fl'] or prev['fl'][1]!=fl[1]):
    ucus.append([i,fl,lasth])
  prev=k
st=defaultdict(Counter)
for i,fl,at in ucus:
  # sonraki sahip
  j=i; 
  while j<len(K) and K[j]['h']<0: j+=1
  alan=K[j]['h'] if j<len(K) else -1
  sure=j-i
  tip=fl[0]; hedef=fl[2]
  if alan<0: sonuc='yok'
  elif alan==hedef: sonuc='hedefe'
  elif TK[alan]==TK[at]: sonuc='başka arkadaş' if alan!=at else 'kendine'
  else: sonuc='RAKİBE'
  if tip=='gönder' and K[min(j,len(K)-1)]['s']!=K[i-1]['s']: sonuc='GOL'
  st[(TK[at],tip)][sonuc]+=1
  a=K[i-1]['p'][at]; 
  print(f"{K[i]['t']:5d} {sn(K[i]['t'])} {['LMN','KAL'][TK[at]]} {tip:9s} {AD[at]:8s}({a[0]:.0f},{a[1]:.0f}) → hedef {AD[hedef] if hedef>=0 else '-':8s} to={fl[3]} | alan {AD[alan] if alan>=0 else '-':8s} {sure:3d}t {sonuc}")
print()
for k,v in sorted(st.items()): print(['Liman','Kale Duvarı'][k[0]],k[1],dict(v))
