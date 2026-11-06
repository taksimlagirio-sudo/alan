# Davranış kalitesi ölçüleri (skordan bağımsız), takım başına. Kullanım: ANALIZ_OUT=<olc çıktısı> python3 davranis.py
# Ölçüler:
#  bekleme_bosa_%   : "tut" seçen kararlardan, aynı taşıyıcının sonraki kararında en iyi değer beklemenin değerinin %60'ının altına düşen
#  kale25_karar     : rakip kaleye 25 br içindeki taşıyıcı kararı; sut_orani_% bunların kaçında şut seçildi
#  kutuya_giris     : sahiplik parçalarından taşıyıcının rakip kaleye 20 br içine girdiği sayısı
#  destek_yok_%     : takım topta iken taşıyıcıya 15 br içinde arkadaş olmayan tik
#  baskici_kale_tarafi_% : rakip kendi yarımızda topta iken en yakın baskıcının (pres/kovala/kapatma işi) taşıyıcı ile kalemiz arasında olduğu tik
#  sut_hatta_bos_%  : şut anında şutçu–kale hattında (2 br) savunmacı olmayan şut
#  rakibe_pas_%     : pas+önüne+kenardan uçuşlarının rakipte biten oranı
#  donma            : değeri ≈0 (|v|<1e-4) karar
#  uzun_tutma       : 10 sn'den uzun tek elde tutma (oyuncu@tik/sn)
import os,json,math,sys
sys.path.insert(0,os.path.dirname(__file__))
from yukle import K,AD,TK,d,S
KR=[json.loads(l) for l in open(os.path.join(S,'kararlar.txt'))]
def seg_d(px,py,ax,ay,bx,by):
  vx,vy=bx-ax,by-ay; L=vx*vx+vy*vy or 1e-9; u=max(0,min(1,((px-ax)*vx+(py-ay)*vy)/L)); return d(px,py,ax+u*vx,ay+u*vy)
o={}
for T,n in ((0,'Liman'),(1,'KaleDuvari')):
  gx=120 if T==0 else 0; ox=0 if T==0 else 120
  R=[r for r in KR if r[2]==T]
  # bekleme
  tut=0;bosa=0
  for i,r in enumerate(R):
    if not r[5][0][0].startswith('tut'): continue
    v=r[5][0][2]
    if v is None or v<0.05: continue
    for r2 in R[i+1:i+3]:
      if r2[1]==r[1] and r2[0]-r[0]<=45:
        tut+=1
        if r2[5][0][2] is not None and r2[5][0][2]<0.6*v: bosa+=1
        break
  k25=[r for r in R if d(r[3],r[4],gx,30)<=25]
  sut=sum(1 for r in k25 if r[5][0][0].startswith('gönder'))
  donma=sum(1 for r in R if r[5][0][2] is not None and abs(r[5][0][2])<1e-4)
  # tik ölçüleri
  giris=0;cur=None;destek_yok=0;topta=0;bas_n=0;bas_ok=0
  for k in K:
    h=k['h']
    if h>=0 and TK[h]==T and h not in (0,7):
      topta+=1; hp=k['p'][h]
      if not any(d(k['p'][i][0],k['p'][i][1],hp[0],hp[1])<15 for i in range(14) if TK[i]==T and i!=h and i not in (0,7)): destek_yok+=1
      if d(hp[0],hp[1],gx,30)<20:
        if cur!=('in',h): giris+=1
        cur=('in',h)
      elif cur and cur[1]!=h: cur=None
    else:
      if h<0 or TK[h]!=T: cur=None
    # baskıcı kale tarafında (biz T savunurken)
    if h>=0 and TK[h]!=T and h not in (0,7):
      hp=k['p'][h]
      if d(hp[0],hp[1],ox,30)<50:
        pr=[i for i in range(14) if TK[i]==T and i not in (0,7) and k['p'][i][6].startswith(('pres','kovala','kapatma'))]
        if pr:
          j=min(pr,key=lambda i:d(k['p'][i][0],k['p'][i][1],hp[0],hp[1])); p=k['p'][j]
          bas_n+=1
          # kale tarafında: kaleye taşıyıcıdan yakın ve taşıyıcı–kale doğrusuna 3 br'den yakın
          if d(p[0],p[1],ox,30)<d(hp[0],hp[1],ox,30) and seg_d(p[0],p[1],hp[0],hp[1],ox,30)<3: bas_ok+=1
  # şut hattı ve pas
  prev=None;sut_n=0;sut_bos=0;pas_n=0;pas_rak=0;lasth=-1
  for i,k in enumerate(K):
    if k['h']>=0: lasth=k['h']
    fl=k['fl']
    if fl and (prev is None or not prev['fl'] or prev['fl'][1]!=fl[1]) and lasth>=0 and TK[lasth]==T:
      sp=prev['p'][lasth] if prev else k['p'][lasth]
      if fl[0]=='gönder':
        sut_n+=1
        if not any(seg_d(prev['p'][j][0],prev['p'][j][1],sp[0],sp[1],gx,30)<2 for j in range(14) if TK[j]!=T and j not in (0,7)): sut_bos+=1
      elif fl[0] in ('pas','önüne','kenardan'):
        pas_n+=1; j=i
        while j<len(K) and K[j]['h']<0: j+=1
        if j<len(K) and TK[K[j]['h']]!=T: pas_rak+=1
    prev=k
  uzun=[];cur=None
  for k in K:
    h=k['h']
    if cur and h==cur[0]: cur[2]=k['t']; continue
    if cur and cur[0]>=0 and TK[cur[0]]==T and cur[2]-cur[1]>=300: uzun.append(f'{AD[cur[0]]}@{cur[1]}/{round((cur[2]-cur[1])/30)}sn')
    cur=[h,k['t'],k['t']]
  pc=lambda a,b: round(100*a/b) if b else None
  o[n]={'donma':donma,'bekleme_bosa_%':pc(bosa,tut),'tut_karar':tut,'kale25_karar':len(k25),'sut_orani_%':pc(sut,len(k25)),'kutuya_giris':giris,
        'destek_yok_%':pc(destek_yok,topta),'baskici_kale_tarafi_%':pc(bas_ok,bas_n),'sut':sut_n,'sut_hatta_bos_%':pc(sut_bos,sut_n),'rakibe_pas_%':pc(pas_rak,pas_n),'uzun_tutma':uzun}
print(json.dumps(o,ensure_ascii=False))
