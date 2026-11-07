# En geri savunmacının arkasına geçen rakip anları (hat.py tanımı, en az 10 tik): kim geçti, onu tutan (işinde adı geçen)
# savunmacı kimdi, geçişten 15 tik önce ve geçiş anında ne işteydi, gol 90 tik içinde mi.
#   python3 analiz/py/arka_an.py <analiz.jsonl> [en çok an]
import json,sys,math
AD=['Aras','Deniz','Mert','Ece','Cansu','Kaan','Tuna','Sur','Burç','Kale','Siper','Hendek','Mancınık','Ok']
R=[json.loads(l) for l in open(sys.argv[1])]; N=int(sys.argv[2]) if len(sys.argv)>2 else 99
def arkada(r):
    h=r['h']
    if h is None or h<0: return []
    tm=0 if h<7 else 1; dt=1-tm; gx=0.0 if dt==0 else 120.0
    D=range(dt*7+1,dt*7+7)
    dd=min(math.hypot(r['p'][i][0]-gx,r['p'][i][1]-30) for i in D)
    return [i for i in range(tm*7+1,tm*7+7) if i!=h and math.hypot(r['p'][i][0]-gx,r['p'][i][1]-30)<dd]
ac={}; an=[]
for i,r in enumerate(R):
    A=set(arkada(r))
    for a in list(ac):
        if a not in A:
            if i-ac[a]>=10: an.append((ac[a],i,a))
            del ac[a]
    for a in A:
        ac.setdefault(a,i)
an.sort()
print(f"{sys.argv[1]}: {len(an)} an (10 tik+)")
for (i0,i1,a) in an[:N]:
    r=R[i0]; tm=0 if a<7 else 1; dt=1-tm; gx=0.0 if dt==0 else 120.0
    s0=r['s'][tm]; gol=any(R[j]['s'][tm]>s0 for j in range(i0,min(len(R),i0+90)))
    tut=[d for d in range(dt*7+1,dt*7+7) if AD[a] in R[max(0,i0-15)]['p'][d][6] or AD[a] in r['p'][d][6]]
    D=range(dt*7+1,dt*7+7); en=min(D,key=lambda d:math.hypot(r['p'][d][0]-gx,r['p'][d][1]-30))
    pa=r['p'][a]
    print(f"\n{r['t']}–{R[i1-1]['t']} {AD[a]} kaleye {math.hypot(pa[0]-gx,pa[1]-30):.0f} br, iş '{pa[6]}', taşıyıcı {AD[r['h']]} {'GOL' if gol else ''}")
    for d in sorted(set(tut+[en])):
        q=R[max(0,i0-15)]['p'][d]; p=r['p'][d]
        print(f"   {AD[d]:8s} {'(en geri)' if d==en else '':9s} -15: '{q[6]}' ({q[0]:.0f},{q[1]:.0f}) v={math.hypot(q[2],q[3]):.2f} | an: '{p[6]}' ({p[0]:.0f},{p[1]:.0f}) hedef ({p[4]:.0f},{p[5]:.0f}) v={math.hypot(p[2],p[3]):.2f} adama {math.hypot(p[0]-pa[0],p[1]-pa[1]):.1f}")
