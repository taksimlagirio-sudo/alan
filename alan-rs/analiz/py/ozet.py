# Bir koşunun ana ölçüleri (tek satır JSON): olc.sh çıktısından. Karşılaştırma: ozet.py <out> [başvuru_out]
import json,math,os,sys
from collections import Counter
out=sys.argv[1]
os.environ['ANALIZ_OUT']=out
sys.path.insert(0,os.path.dirname(__file__))
from yukle import K,AD,TK,d
KR=[json.loads(l) for l in open(os.path.join(out,'kararlar.txt'))]
def seg_d(px,py,ax,ay,bx,by):
  vx,vy=bx-ax,by-ay; L=vx*vx+vy*vy or 1e-9; u=max(0,min(1,((px-ax)*vx+(py-ay)*vy)/L)); return d(px,py,ax+u*vx,ay+u*vy)
o={}
o['skor']='%d-%d'%tuple(K[-1]['s'])
o['donma_karar']=sum(1 for r in KR if r[5][0][2] is not None and abs(r[5][0][2])<1e-6)
o['karar']=len(KR)
# uçuşlar
uc=Counter();prev=None
for k in K:
  fl=k['fl']
  if fl and (prev is None or not prev['fl'] or prev['fl'][1]!=fl[1]):
    h=prev['h'] if prev and prev['h']>=0 else -1
    uc[(TK[h] if h>=0 else -1,fl[0])]+=1
  prev=k
for T,n in ((0,'L'),(1,'K')):
  o[n+'_pas']=uc[(T,'pas')]+uc[(T,'önüne')]; o[n+'_kenardan']=uc[(T,'kenardan')]; o[n+'_sut']=uc[(T,'gönder')]
# baskı ve donma dönemleri
for T,n in ((0,'L'),(1,'K')):  # T = savunan
  rows=0;uzak=0;pressiz=0;run=0;uzun=[];
  for k in K:
    h=k['h']
    if h<0 or TK[h]==T:
      if run>=150: uzun.append(run)
      run=0; continue
    rows+=1; hp=k['p'][h]
    nd=min(d(hp[0],hp[1],k['p'][j][0],k['p'][j][1]) for j in range(14) if TK[j]==T and j not in (0,7))
    if nd>15: uzak+=1
    if not any(k['p'][j][6].startswith(('pres','kovala','kapatma')) for j in range(14) if TK[j]==T): pressiz+=1
    if nd>10: run+=1
    else:
      if run>=150: uzun.append(run)
      run=0
  o[n+'_sav_uzak15_%']=round(100*uzak/max(rows,1)); o[n+'_sav_pressiz_%']=round(100*pressiz/max(rows,1))
  o[n+'_sav_5sn+_baskisiz']='%d dönem/%.0f sn'%(len(uzun),sum(uzun)/30)
# hücum şekli ve topsuz
for T,n in ((0,'L'),(1,'K')):
  g=1 if T==0 else -1; gx=120 if T==0 else 0
  nt=0;L=[];acikyok=0;nb=0;bos_kutu=0
  for k in K:
    h=k['h']
    if h<0 or TK[h]!=T or h in (0,7): continue
    nt+=1; hp=k['p'][h]
    out_=[i for i in range(14) if TK[i]==T and i not in (0,7)]
    xs=[k['p'][i][0] for i in out_]; L.append(max(xs)-min(xs))
    ac=0
    for i in out_:
      if i==h: continue
      p=k['p'][i]; dh=d(p[0],p[1],hp[0],hp[1])
      nr=min(d(p[0],p[1],k['p'][j][0],k['p'][j][1]) for j in range(14) if TK[j]!=T)
      ln=min(seg_d(k['p'][j][0],k['p'][j][1],hp[0],hp[1],p[0],p[1]) for j in range(14) if TK[j]!=T and j not in (0,7))
      if 6<=dh<=40 and nr>5 and ln>2.5: ac+=1
    if ac==0: acikyok+=1
    if d(hp[0],hp[1],gx,30)<35:
      nb+=1
      if not any(d(k['p'][i][0],k['p'][i][1],gx,30)<20 for i in out_ if i!=h): bos_kutu+=1
  o[n+'_boy_ort/maks']='%.0f/%.0f'%(sum(L)/max(nt,1),max(L) if L else 0)
  o[n+'_acik_pas_yok_%']=round(100*acikyok/max(nt,1))
  o[n+'_kutu_bos_%']=round(100*bos_kutu/max(nb,1))
# markaj git-gel: Liman ve KAL dış oyuncularının >12 br hedef sıçraması
for T,n in ((0,'L'),(1,'K')):
  s=0
  for i in range(14):
    if TK[i]!=T or i in (0,7): continue
    for t in range(1,len(K)):
      a=K[t-1]['p'][i];b=K[t]['p'][i]
      if None in (a[4],b[4]): continue
      if d(a[4],a[5],b[4],b[5])>12: s+=1
  o[n+'_hedef_sicrama']=s
print(json.dumps(o,ensure_ascii=False))
if len(sys.argv)>2:
  b=json.loads(os.popen(f'{sys.executable} {__file__} {sys.argv[2]}').read())
  for k_ in o:
    if o[k_]!=b.get(k_): print(f'  {k_}: {b.get(k_)} → {o[k_]}')
