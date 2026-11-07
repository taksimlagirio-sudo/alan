# Bir tikte hızı 90°+ dönen (hız değişimi |Δv|>=0.1, iki tikte de |v|>=0.02) topsuz oyuncular ve kaynakları.
#   python3 analiz/py/hiz_don.py <analiz.jsonl> [ayrıntı sayısı]
# Kaynak (ilk tutan): olay (oyuncunun adı geçen olay, bu ya da önceki tik), iş (iş adı değişti), hedef (hedef 3 br+ sıçradı),
# denge (den düştü), hedef-yön (hedef aynı ama oyuncu hedefini geçti / hedefe göre ters), bilinmiyor.
import json,sys,math
from collections import Counter
AD=['Aras','Deniz','Mert','Ece','Cansu','Kaan','Tuna','Sur','Burç','Kale','Siper','Hendek','Mancınık','Ok']
R=[json.loads(l) for l in open(sys.argv[1])]; N=int(sys.argv[2]) if len(sys.argv)>2 else 0
C=Counter(); ORN={}; isd=Counter()
for i in range(2,len(R)):
    r,q=R[i],R[i-1]
    ev=' '.join(e[1] for e in r['ev']+q['ev'])
    for k in range(14):
        if r['h']==k or q['h']==k: continue
        a,b=q['p'][k],r['p'][k]
        va=math.hypot(a[2],a[3]); vb=math.hypot(b[2],b[3])
        if va<0.02 or vb<0.02 or math.hypot(a[2]-b[2],a[3]-b[3])<0.1: continue
        if a[2]*b[2]+a[3]*b[3]>=0: continue
        j0,j1=a[6].split(' · ')[0],b[6].split(' · ')[0]
        if AD[k] in ev: s='olay'
        elif a[6]!=b[6]: s='iş'
        elif math.hypot(a[4]-b[4],a[5]-b[5])>3: s='hedef'
        elif a[9] is not None and b[9] is not None and b[9]<a[9]-0.02: s='denge'
        else:
            # hedefe göre: önceki hız hedefe doğru muydu
            dx,dy=b[4]-b[0],b[5]-b[1]; dl=math.hypot(dx,dy)
            s='hedef dibinde' if dl<1.0 else ('hedefe dönüş' if a[2]*dx+a[3]*dy<0 else 'bilinmiyor')
        C[s]+=1; isd[(s,j1)]+=1
        ORN.setdefault(s,[]).append((r['t'],AD[k],a[6],b[6],round(va,2),round(vb,2),round(math.hypot(a[4]-b[4],a[5]-b[5]),1),ev[:90]))
print(f"{sys.argv[1]}: {len(R)} tik, 90°+ dönüş {sum(C.values())}")
for s,n in C.most_common(): print(f"  {s:14s} {n}")
print("  kaynak × iş (en sık 15):")
for (s,j),n in isd.most_common(15): print(f"    {n:4d} {s:14s} {j}")
if N:
    for s in C:
        print(f"-- {s}")
        for o in ORN[s][:N]: print('   ',o)
