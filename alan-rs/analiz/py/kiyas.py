# İki maçın oyun biçimi karşılaştırması (aynı tohum, iki motor). Kullanım: python3 kiyas.py <A_out> <B_out>
import json,math,os,sys
from collections import Counter,defaultdict
AD=['Aras','Deniz','Mert','Ece','Cansu','Kaan','Tuna','Sur','Burç','Kale','Siper','Hendek','Mancınık','Ok']
TK=[0]*7+[1]*7
def d(a,b,c,e): return math.hypot(a-c,b-e)
def yukle(o):
  K=[json.loads(l) for l in open(o+'/analiz.jsonl')]; R=[json.loads(l) for l in open(o+'/kararlar.txt')]; return K,R
def olc(o):
  K,R=yukle(o); r={}
  for T,n in ((0,'L'),(1,'K')):
    g=1 if T==0 else -1; gx=120 if T==0 else 0; ox=120-gx
    # kararlar
    kt=Counter(x[5][0][0].split(' ')[0] for x in R if x[2]==T)
    tot=sum(kt.values()) or 1
    r[n+' karar dağılımı']=' '.join(f'{k} %{100*v/tot:.0f}' for k,v in kt.most_common(5))+f' (n={tot})'
    # pas
    pl=[];ileri=0;geri=0;yan=0;rak=0;ok=0;uzun=0;kenar=0
    sur_l=[];sur_ileri=0
    prev=None;lasth=-1
    for i,k in enumerate(K):
      if k['h']>=0: lasth=k['h']
      fl=k['fl']
      if fl and prev and (not prev['fl'] or prev['fl'][1]!=fl[1]) and lasth>=0 and TK[lasth]==T and fl[0] in ('pas','önüne','kenardan'):
        a=prev['p'][lasth]; j=i
        while j<len(K) and K[j]['h']<0: j+=1
        if j<len(K):
          b=K[j]['p'][K[j]['h']]; L=d(a[0],a[1],b[0],b[1]); pl.append(L)
          dx=(b[0]-a[0])*g
          if dx>5: ileri+=1
          elif dx<-5: geri+=1
          else: yan+=1
          if L>35: uzun+=1
          if fl[0]=='kenardan': kenar+=1
          if TK[K[j]['h']]!=T: rak+=1
      prev=k
    n_p=len(pl) or 1
    r[n+' pas']=f'{len(pl)} pas, ort {sum(pl)/n_p:.0f} br, ileri %{100*ileri/n_p:.0f} yan %{100*yan/n_p:.0f} geri %{100*geri/n_p:.0f}, 35+ br %{100*uzun/n_p:.0f}, kenardan %{100*kenar/n_p:.0f}, rakibe %{100*rak/n_p:.0f}'
    # sürüş: aynı taşıyıcının kesintisiz tuttuğu parçalarda yol ve ilerleme
    seg=[];cur=None
    for k in K:
      h=k['h']
      if h>=0 and TK[h]==T and h not in (0,7):
        p=k['p'][h]
        if cur and cur[0]==h: cur[2]=(p[0],p[1]); cur[3]+=1
        else:
          if cur: seg.append(cur)
          cur=[h,(p[0],p[1]),(p[0],p[1]),1]
      else:
        if cur: seg.append(cur); cur=None
    yl=[d(*s[1],*s[2]) for s in seg]; il=[(s[2][0]-s[1][0])*g for s in seg]; sure=[s[3]/30 for s in seg]
    ns=len(seg) or 1
    r[n+' topla parça']=f'{len(seg)} parça, ort süre {sum(sure)/ns:.1f} sn, ort yol {sum(yl)/ns:.0f} br, ort ilerleme {sum(il)/ns:+.0f} br, 5 sn+ parça {sum(1 for x in sure if x>=5)}'
    # topa sahiplik ve bölge
    top=[k for k in K if k['h']>=0 and TK[k['h']]==T]
    if top:
      ucs=Counter('kendi üçte' if abs(k['p'][k['h']][0]-ox)<40 else 'orta' if abs(k['p'][k['h']][0]-ox)<80 else 'rakip üçte' for k in top)
      r[n+' topla bölge']=f"{len(top)} tik: "+' '.join(f'{a} %{100*b/len(top):.0f}' for a,b in ucs.most_common())
    # şekil topta: genişlik, derinlik, hatlar arası (taşıyıcı önünde/arkasında)
    W=[];L=[];onde=[];geri_k=[]
    for k in top[::5]:
      xs=[k['p'][i][0] for i in range(14) if TK[i]==T and i not in (0,7)]; ys=[k['p'][i][1] for i in range(14) if TK[i]==T and i not in (0,7)]
      W.append(max(ys)-min(ys)); L.append(max(xs)-min(xs)); hp=k['p'][k['h']]
      onde.append(sum(1 for i in range(14) if TK[i]==T and i not in (0,7) and (k['p'][i][0]-hp[0])*g>5))
    if W: r[n+' şekil topta']=f'genişlik {sum(W)/len(W):.0f}, boy {sum(L)/len(L):.0f}, topun önünde ort {sum(onde)/len(onde):.1f} oyuncu'
    # savunma: rakip topta iken blok yüksekliği (en geri saha oyuncusunun kaleye uzaklığı), hat boyu, topun önündeki sayı, baskı
    sv=[k for k in K if k['h']>=0 and TK[k['h']]!=T]
    BH=[];HL=[];ON=[];PR=Counter();ND=[]
    for k in sv[::5]:
      xs=[abs(k['p'][i][0]-ox) for i in range(14) if TK[i]==T and i not in (0,7)]
      BH.append(min(xs)); HL.append(max(xs)-min(xs)); hp=k['p'][k['h']]
      ON.append(sum(1 for i in range(14) if TK[i]==T and i not in (0,7) and abs(k['p'][i][0]-ox)<abs(hp[0]-ox)))
      PR[sum(1 for i in range(14) if TK[i]==T and k['p'][i][6].startswith(('pres','kovala','kapatma')))]+=1
      ND.append(min(d(hp[0],hp[1],k['p'][i][0],k['p'][i][1]) for i in range(14) if TK[i]==T and i not in (0,7)))
    if BH:
      r[n+' savunma']=f'en geri çizgi kaleye ort {sum(BH)/len(BH):.0f} br, takım boyu {sum(HL)/len(HL):.0f}, topun önünde ort {sum(ON)/len(ON):.1f}/6, taşıyıcıya en yakın ort {sum(ND)/len(ND):.1f} br, baskıcı sayısı '+' '.join(f'{a}:%{100*b/len(BH):.0f}' for a,b in sorted(PR.items()))
    # topsuz iş dağılımı (topta iken)
    jb=Counter()
    for k in top[::3]:
      for i in range(14):
        if TK[i]==T and i not in (0,7) and i!=k['h']: jb[k['p'][i][6].split(' · ')[0]]+=1
    tj=sum(jb.values()) or 1
    r[n+' topsuz işler']=' '.join(f'{a} %{100*b/tj:.0f}' for a,b in jb.most_common(6))
    # şut
    st=[];prev=None;lasth=-1
    for i,k in enumerate(K):
      if k['h']>=0: lasth=k['h']
      fl=k['fl']
      if fl and fl[0]=='gönder' and prev and (not prev['fl'] or prev['fl'][1]!=fl[1]) and lasth>=0 and TK[lasth]==T:
        sp=prev['p'][lasth]; s0=prev['s'][T]; gol=any(K[j]['s'][T]>s0 for j in range(i,min(len(K),i+60)))
        st.append(f"{d(sp[0],sp[1],gx,30):.0f}{'G' if gol else ''}")
      prev=k
    r[n+' şutlar (br, G=gol)']=' '.join(st) or '-'
  r['skor']='%d-%d'%tuple(K[-1]['s'])
  return r
a=olc(sys.argv[1]); b=olc(sys.argv[2])
for k in a:
  print(f'■ {k}\n   A: {a[k]}\n   B: {b.get(k)}')
