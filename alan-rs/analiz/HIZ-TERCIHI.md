# Hız tercihi neden yanlış (mac/enerji.rs sprint_karar3) — 217468 "hepsi açık" maçı
Tüm maç, hedefine >8 br uzak oyuncular (taban koşu 0.29, tavan 0.42 br/tik):

| | iş | yürü | sprint |
|---|---|---|---|
| savunma | geri koşu | %51 | %1 |
| savunma | yolu kesiyor | %73 | %1 |
| savunma | pres | %48 | %2 |
| savunma | pres · şut hattı | %43 | %1 |
| savunma | markaj | %39 | %12 |
| hücum | güvence | %10 | **%44** |
| hücum | açma | %7 | %36 |
| hücum | boşluk | %7 | %21 |

İz (ALAN_K3_IZ):
- 1147 Hendek (KAL) geri koşu, Kaan kaleye 20 br: hedef 14 br, `tD=25`, sprint T≈34, tempo T≈48, Sy=0.46 → **f=−1 yürü**
- 1371 Deniz "yolu kesiyor · Ok": hedef 13 br ama hedef Ok'un 1.4 br yanında, `tD=5.6`, sprint T≈31 → **f=−1 yürü**
- 1415 Siper (KAL) güvence, top Ok'ta 40 br ileride: kendi kalesine doğru 20 br, `bizde`, `tD=54` (pas varışı), sprint T≈47, Sy=0.012, bar 0.996 → **f=1 sprint**

## Hata 1: koşunun değeri "hedef noktaya rakipten önce varır mıyım" yarışı
`DEG = Sy·wf(T,tD) − bedel`, `wf = 1/(1+exp((T−tD)/5))`. Savunma hedeflerinin çoğu rakibe bağlı (yolu kes = taşıyıcının yanı, şut hattı =
taşıyıcının 3 br önü, markaj = adamın yanı, kale önü = taşıyıcının gideceği yer): rakip oraya tanım gereği önce varır, tD küçük, 5 tiklik
sigmoid gecikeni sıfırlar. "Geç kaldım ama yaklaşırsam açısı daralır / şutu geç atar / pas yolu kapanır" değeri modelde yok: ya yarışı kazanırsın
ya hiç. Bu yüzden önemli (Sy=0.46) ama "kaybedilmiş" koşuda yürüyor; hedef ne kadar önemli olursa olsun.
## Hata 2: enerjinin bedeli o anki hedefin değeriyle ölçülüyor
`bedel = Σ Sy·(en_iyi(e_a)−en_iyi(e_b)) + ku(e_a)−ku(e_b)`: harcanan barın değeri, aynı hedefe ileride yetişebilme × aynı hedefin Sy'si.
Önemsiz hedefte (Sy=0.012) bar ucuz, bar doluyken (0.996) e_a 1'de doyduğu için neredeyse bedava → güvence/boşluk/açma için depar.
Barın asıl değeri sonraki işler (kontra, geri dönüş, pres) ve hedefin önemiyle ilgisiz. Ok'un ilk maçlardaki gereksiz koşuları da bu.
## Sonuç
Yuru bu iki hatayı görünür yaptı: yarış "kaybedilince" değer 0, yürümek enerji kazandırıyor → yürü. Yuru yokken de aynı koşuda yalnız tempo
(0.29), sprint hiç yok. Yasaklamak değil, değer düzeltilmeli: (1) koşunun değeri sürekli olmalı (varış ne kadar erkense tehlike o kadar azalır;
savunmada yarış rakibin hedefe değil, tehlikenin gerçekleşeceği ana — şut/pas/kaleye giriş — karşı), (2) barın değeri hedeften bağımsız,
oyuncunun ve takımın yakın gelecekteki ihtiyacıyla ölçülmeli.
