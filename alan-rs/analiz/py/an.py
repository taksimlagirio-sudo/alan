# Anları kare kare yaz: ANALIZ_OUT=<out> python3 an.py <baş> <son> <adım> [oyuncu sıraları virgüllü]
import os,sys,math,json
sys.path.insert(0,os.path.dirname(__file__))
from yukle import *
a,b,st=int(sys.argv[1]),int(sys.argv[2]),int(sys.argv[3])
sel=[int(x) for x in sys.argv[4].split(',')] if len(sys.argv)>4 else range(14)
KR={}
for l in open(os.path.join(S,'kararlar.txt')):
  r=json.loads(l); KR[r[0]]=r
for t in range(a,b+1,st):
  k=tk(t); h=k['h']; bb=k['b']
  print(f"-- {t} {sn(t)} sahip={AD[h] if h>=0 else '-'} top={'(%.0f,%.0f)'%(bb[0],bb[1]) if bb else '-'} uçuş={k['fl'][0] if k['fl'] else '-'}")
  for i in sel:
    p=k['p'][i]; print(f"   {AD[i]:9s} ({p[0]:5.1f},{p[1]:4.1f}) hız {math.hypot(p[2],p[3]):.2f} bar {p[7]} {p[6][:34]}")
  for tt in range(t,t+st):
    if tt in KR: r=KR[tt]; print(f"   KARAR {tt} {r[1]} → {r[5][0][0][:14]} {r[5][0][1]} v={r[5][0][2]:.3f} | {[(x[0][:8],x[1],round(x[2],3)) for x in r[5][1:3]]}")
