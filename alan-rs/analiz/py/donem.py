import json,math,sys
from collections import defaultdict
AD=['Aras','Deniz','Mert','Ece','Cansu','Kaan','Tuna','Sur','Burç','Kale','Siper','Hendek','Mancınık','Ok']
D=sys.argv[1]
K=[json.loads(l) for l in open(D+'/analiz.jsonl')]
R=[json.loads(l) for l in open(D+'/kararlar.txt')]
Rt=defaultdict(list)
for r in R: Rt[r[0]].append(r)
TK=lambda i:0 if i<7 else 1
def d(a,b,c,e): return math.hypot(a-c,b-e)
# dönemler: aynı takımın kesintisiz sahipliği (top havadayken son sahip takım)
don=[];cur=None;last=None
for i,k in enumerate(K):
  h=k['h']; T=TK(h) if h>=0 else last
  if h>=0: last=TK(h)
  if T is None: continue
  if cur is None or cur['T']!=T:
    if cur: cur['b']=i-1; don.append(cur)
    cur={'T':T,'a':i}
cur['b']=len(K)-1; don.append(cur)
bul=[]
for dn in don:
  T=dn['T']; gx=120 if T==0 else 0; ox=120-gx
  seg=K[dn['a']:dn['b']+1]
  if len(seg)<20: continue
  for k in seg[::5]:
    h=k['h']
    if h<0 or TK(h)!=T: continue
    hp=k['p'][h]; lg=d(hp[0],hp[1],gx,30)
    savs=[j for j in range(14) if TK(j)!=T and j not in (0,7)]
    # savunma: taşıyıcı kaleye 30 br içinde, savunmacılar ve işleri
    if lg<30:
      on=[j for j in savs if d(k['p'][j][0],k['p'][j][1],gx,30)<lg-1]
      yuru=[AD[j]+':'+k['p'][j][6][:14] for j in savs if not k['p'][j][6].startswith('önde') and math.hypot(k['p'][j][2],k['p'][j][3])<0.12 and d(k['p'][j][4],k['p'][j][5],k['p'][j][0],k['p'][j][1])>6]
      if not on: bul.append((k['t'],'SAV','kale tarafında savunmacı yok',AD[h],round(lg)))
      if yuru: bul.append((k['t'],'SAV','tehlikede yürüyen savunmacı',','.join(yuru),round(lg)))
    # topsuz hücumcular: takım topta, rakip yarıda, taşıyıcıya 25 br içinde açık arkadaş var mı
    if abs(hp[0]-ox)>60:
      ark=[j for j in range(14) if TK(j)==T and j not in (0,7,h)]
      acik=0
      for j in ark:
        p=k['p'][j]; dh=d(p[0],p[1],hp[0],hp[1])
        if 6<dh<25 and min(d(p[0],p[1],k['p'][q][0],k['p'][q][1]) for q in savs)>5: acik+=1
      if acik==0: bul.append((k['t'],'HÜC','rakip yarıda taşıyıcıya açık arkadaş yok',AD[h],round(lg)))
  # dönem sonu: nasıl bitti
  son=K[dn['b']]; nx=K[min(dn['b']+1,len(K)-1)]
  if nx['s'][T]>K[dn['a']]['s'][T]: sonuc='GOL'
  else:
    fl=None
    for k in reversed(seg):
      if k['fl']: fl=k['fl'][0]; break
    sonuc=fl or 'kayıp'
  dn['sonuc']=sonuc
# kararlar: değer 0'a yakın ve 'tut' dışı, uzak şut, rakibe giden pas
for r in R:
  T=r[2]; gx=120 if T==0 else 0; lg=d(r[3],r[4],gx,30); o=r[5][0]
  if str(o[0]).startswith('gönder') and lg>30: bul.append((r[0],'HÜC',f'uzak şut {lg:.0f} br (v {o[2]:.3f})',r[1],round(lg)))
  if o[2] is not None and abs(o[2])<1e-4: bul.append((r[0],'KARAR','bütün seçenekler ~0 (donma)',r[1],round(lg)))
# özet
from collections import Counter
c=Counter((b[1],b[2].split(' (')[0].split(' ')[0:4].__str__()) for b in bul)
print('dönem sayısı',len(don),'| sonuçlar',Counter(dn.get('sonuc') for dn in don if 'sonuc' in dn).most_common(6))
kats=defaultdict(list)
for b in bul: kats[b[2].split(' (')[0] if not b[2].startswith('uzak') else 'uzak şut'].append(b)
for k,v in kats.items():
  # tik kümeleri (aynı an 60 tik içinde birleşir)
  an=[];
  for b in sorted(v):
    if not an or b[0]-an[-1][-1][0]>60: an.append([b])
    else: an[-1].append(b)
  print(f'\n## {k}: {len(an)} an')
  for a in an[:12]: print('   ',a[0][0],f'({a[0][0]//30//60}:{a[0][0]//30%60:02d})',a[0][3],f'kaleye {a[0][4]} br', f'süre {(a[-1][0]-a[0][0])/30:.1f} sn')
