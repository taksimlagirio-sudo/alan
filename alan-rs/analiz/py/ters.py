import json,sys,glob,math
# ters.py <desen>: savunan takımın saha oyuncuları; hedef bir tikte 10 br+ sıçrar ve oyuncudan hedefe yön 90°+ döner (ters dönüş);
# ve 30 tik içinde geri ters dönüş (A→B→A). Maç başına.
for pat in sys.argv[1:]:
    tek=cift=0; mac=0
    for f in sorted(glob.glob(pat)):
        mac+=1; on={}; son_tm=None; ev={}
        for l in open(f):
            r=json.loads(l); h=r['h']; t=r['t']
            if h is not None and h>=0: son_tm=0 if h<7 else 1
            if son_tm is None: continue
            dt=1-son_tm
            for i in range(dt*7+1,dt*7+7):
                p=r['p'][i]; T=(p[4],p[5])
                if i in on and on[i][0]==dt:
                    T0=on[i][1]; ax,ay=T0[0]-p[0],T0[1]-p[1]; bx,by=T[0]-p[0],T[1]-p[1]
                    la,lb=math.hypot(ax,ay),math.hypot(bx,by)
                    if math.hypot(T[0]-T0[0],T[1]-T0[1])>10 and la>1 and lb>1 and (ax*bx+ay*by)/(la*lb)<0:
                        tek+=1
                        if i in ev and t-ev[i]<=30: cift+=1
                        ev[i]=t
                on[i]=(dt,T)
    print(f"{pat.split('/')[-1]}: {mac} maç, hedef ters dönüşü {tek} ({tek/max(1,mac):.0f}/maç), 30 tikte geri dönen {cift} ({cift/max(1,mac):.0f}/maç)")
