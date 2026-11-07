import json,sys,glob,math
# tehlike_yuru.py <desen>: rakip topta ve taşıyıcı kaleye ≤25 br iken, savunan takımın saha oyuncularından hedefine 8 br+ uzak olup
# 0,2 br/tik altında hareket edenlerin payı (tehlikede yürüyen savunmacı)
for pat in sys.argv[1:]:
    n=y=0; an=0
    for f in sorted(glob.glob(pat)):
        for l in open(f):
            r=json.loads(l); h=r['h']
            if h is None or h<0 or h in (0,7): continue
            tm=0 if h<7 else 1; gx=120.0 if tm==0 else 0.0
            if math.hypot(gx-r['p'][h][0],30-r['p'][h][1])>25: continue
            an+=1
            for i in range((1-tm)*7+1,(1-tm)*7+7):
                p=r['p'][i]
                if math.hypot(p[4]-p[0],p[5]-p[1])<=8: continue
                n+=1; y+= math.hypot(p[2],p[3])<0.2
    print(f"{pat}: tehlikeli an {an} tik; hedefine 8 br+ uzak savunmacı-tik {n}, yürüyen (<0,2) %{100*y/max(1,n):.1f}")
