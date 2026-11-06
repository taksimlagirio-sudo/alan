from yukle import *
import os
from collections import Counter,defaultdict
KR=[json.loads(l) for l in open(os.path.join(S,'kararlar.txt'))]
kr=defaultdict(list)
for r in KR: kr[r[0]].append(r)
# segments by holder (incl -1)
seg=[];cur=None
for k in K:
  h=k['h']
  if cur is None or h!=cur[0]:
    if cur: seg.append(cur)
    cur=[h,k['t'],k['t']]
  else: cur[2]=k['t']
seg.append(cur)
def yakin_rakip(k,i):
  p=k['p'][i]; best=(99,None)
  for j in range(14):
    if TK[j]!=TK[i]:
      q=k['p'][j]; dd=d(p[0],p[1],q[0],q[1])
      if dd<best[0]: best=(dd,j)
  return best
for s in seg:
  h,a,b=s
  k0=tk(a);k1=tk(b)
  if h<0:
    fl=k0['fl']
    print(f"{a:5d} {sn(a)} [boş {b-a+1:3d}t] top ({k0['b'][0]},{k0['b'][1]})→({k1['b'][0]},{k1['b'][1]}) fl={fl[0] if fl else None} q={AD[fl[2]] if fl and fl[2]>=0 else '-'}")
  else:
    p0=k0['p'][h];p1=k1['p'][h]
    rk=[yakin_rakip(tk(t),h)[0] for t in range(a,b+1)]
    ks=[ (t,r[5][0][0], r[5][0][1]) for t in range(a,b+1) for r in kr[t]]
    print(f"{a:5d} {sn(a)} {'LMN' if TK[h]==0 else 'KAL'} {AD[h]:9s} {b-a+1:4d}t ({p0[0]:.0f},{p0[1]:.0f})→({p1[0]:.0f},{p1[1]:.0f}) rakip min {min(rk):.1f} ort {sum(rk)/len(rk):.1f}  kararlar {len(ks)}: {Counter(x[1].split(' ')[0] for x in ks).most_common()}")
