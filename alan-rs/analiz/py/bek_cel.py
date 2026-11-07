# Bekçi'nin çeldiği toplar (BEK_CD doğrulaması): çelmeden sonra 1 sn (30 tik) içinde gol oranı; yan etki: Bekçi'nin 3 sn+ (90 tik)
# topun 3 br yakınında tutmadan kaldığı (sekme döngüsü) ve topu 3 sn+ elde tuttuğu dönemler.
#   python3 analiz/py/bek_cel.py "<klasör>/<kol>-*.jsonl" …
# Çelme: top serbest (h=-1, önceki tik de), Bekçi topa 2.5 br'den yakın, top kendi kalesine gidiyordu ve yönü 25°+ döndü ya da hızı %30+ düştü.
import json,sys,math,glob
BK=(0,7)
for pat in sys.argv[1:]:
    cel=gol=0; dongu=tut=0; nm=0
    for f in sorted(glob.glob(pat)):
        R=[json.loads(l) for l in open(f)]; nm+=1
        son={0:-99,7:-99}
        for i in range(1,len(R)):
            r,q=R[i],R[i-1]
            if r['b'] is None or q['b'] is None or r['h']!=-1 or q['h']!=-1: continue
            bx,by,vx,vy=r['b'][:4]; ux,uy=q['b'][2],q['b'][3]
            for k in BK:
                if r['t']-son[k]<15: continue
                p=r['p'][k]; gx=0.0 if k==0 else 120.0
                if math.hypot(bx-p[0],by-p[1])>=2.5: continue
                if (gx-q['b'][0])*ux<=0: continue  # kaleye gitmiyordu
                a=math.hypot(ux,uy); b=math.hypot(vx,vy)
                if a<0.3: continue
                cs=(ux*vx+uy*vy)/(a*max(b,1e-9))
                if not (cs<math.cos(math.radians(25)) or b<0.7*a): continue
                son[k]=r['t']; cel+=1
                rk=1 if k==0 else 0; s0=r['s'][rk]
                for j in range(i,min(len(R),i+31)):
                    if R[j]['s'][rk]>s0: gol+=1; break
        for k in BK:
            nd=nt=0
            for r in R:
                b=r['b'] if r['b'] is not None else None
                p=r['p'][k]
                yak=b is not None and math.hypot(b[0]-p[0],b[1]-p[1])<3.0
                if r['h']==k: nt+=1
                else:
                    if nt>=90: tut+=1
                    nt=0
                if yak and r['h']!=k: nd+=1
                else:
                    if nd>=90: dongu+=1
                    nd=0
    print(pat.split('/')[-1], f"{nm} maç: Bekçi çelmesi {cel}, 1 sn içinde gol {gol} (%{100*gol/max(1,cel):.1f}); 3 sn+ tutmadan yakında {dongu}, 3 sn+ elde {tut}")
