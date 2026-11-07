import json,sys,glob,math
# kuyu_sut.py <desen>: şut anında (fl 'gönder' başlangıcı, ±15 tik) savunan takımdan 'Kuyu alanından çıkıyor' / 'sönük' işinde olan oyuncu
for pat in sys.argv[1:]:
    sut=kotu=0
    for f in sorted(glob.glob(pat)):
        R=[json.loads(l) for l in open(f)]
        for i,r in enumerate(R):
            fl=r.get('fl'); pr=R[i-1] if i else None
            if fl and fl[0]=='gönder' and pr and (not pr.get('fl') or pr['fl'][1]!=fl[1]) and pr['h'] is not None and pr['h']>=0:
                tm=0 if pr['h']<7 else 1; sut+=1
                if any('Kuyu alanından' in q['p'][j][6] or 'sönük' in q['p'][j][6] for q in R[max(0,i-15):i+3] for j in range((1-tm)*7+1,(1-tm)*7+7)): kotu+=1
    print(f"{pat}: şut {sut}, şut anında savunmada Kuyu'dan çıkan {kotu} (%{100*kotu/max(1,sut):.0f})")
