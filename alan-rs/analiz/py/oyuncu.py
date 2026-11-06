from yukle import *
from collections import Counter,defaultdict
def faz(k,i):
  h=k['h']
  if h<0:
    fl=k['fl']
    if fl and fl[2]>=0: return 'hücum' if TK[fl[2]]==TK[i] else 'savunma'
    return 'boş'
  return 'hücum' if TK[h]==TK[i] else 'savunma'
def kok(j):
  j=j.split(' · ')[0]
  return j.split(':')[0]
for i in range(14):
  jobs=defaultdict(Counter); dist=0; dur=0; dur_uzak=0; ters=0; duvar=0; sonuk=0
  durrun=0; durlist=[]
  for t in range(2,9001):
    k=tk(t); p=k['p'][i]; q=tk(t-1)['p'][i]
    f=faz(k,i); jobs[f][kok(p[6])]+=1
    dist+=d(p[0],p[1],q[0],q[1])
    sp=math.hypot(p[2],p[3])
    b=k['b'] if k['b'] else None
    bx,by=(k['p'][k['h']][0],k['p'][k['h']][1]) if k['h']>=0 else ((b[0],b[1]) if b else (p[0],p[1]))
    if sp<0.03 and i not in (0,7) and k['h']!=i:
      durrun+=1
    else:
      if durrun>=60: durlist.append((t-durrun,durrun,f,round(p[0]),round(p[1]),p[6],round(d(p[0],p[1],bx,by))))
      durrun=0
    if t>4:
      r=tk(t-4)['p'][i]
      if math.hypot(r[2],r[3])>0.12 and sp>0.12 and (r[2]*p[2]+r[3]*p[3])/(math.hypot(r[2],r[3])*sp)<-0.7: ters+=1
    if (p[1]<1.6 or p[1]>58.4) and i not in (0,7): duvar+=1
    if 'sönük' in p[6]: sonuk+=1
  print(f"\n##### {AD[i]} ({['LMN','KAL'][TK[i]]} {ROL[i]}) koşu {dist:.0f} br, ters dönüş {ters}, duvar dibi {duvar}t, sönük {sonuk}t")
  for f in ('hücum','savunma','boş'):
    tot=sum(jobs[f].values())
    print(f"  {f:7s} {tot:5d}t: "+', '.join(f"{a or '(boş)'} %{100*b/tot:.0f}" for a,b in jobs[f].most_common(9)))
  if durlist:
    print('  uzun duruşlar (≥2sn, top onda değilken):', [(sn(x[0]),f"{x[1]/30:.1f}sn",x[2],(x[3],x[4]),x[5],'topa',x[6]) for x in durlist][:12], f'(toplam {len(durlist)})')
