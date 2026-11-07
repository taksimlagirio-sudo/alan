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

## hiz2 (savunma 8e7c297) dal sınaması — aynı 6 gol anı, golden 200 tik önce, 400 tik (once = asıl maç, birebir)
| Gol | hiz2 | hiz2 + S7i/j |
|---|---|---|
| 0:57 Mancınık | gol (Ok 11 br, Deniz hatta 2.4 br'de) | gol erken (Hendek 14 br) |
| 1:38 Mancınık | **gol yok** (Mancınık 18 br, Mert hatta) | gol (daha geç) |
| 1:55 Ok | **şut yok**: Deniz 0.10→0.28–0.33 koşuyor, Ok'u içeride kale tarafında karşılıyor | şut yok |
| 2:43 Mancınık | gol erken: Siper geç koşu, Ece 4812'de "markaj · Siper"ı bırakıp bölgeye geçti, arkadan kovaladı | gol |
| 3:55 Ok | **gol yok**: Mert Ok'la birlikte kale tarafına geçti (eskide 0.18 yürüyordu, Deniz 0.10) | gol yok |
| 4:28 Ok | **gol yok** (Hendek 31 br) | gol yok |

Hız dağılımı (6 an, hedefe >8 br; yürü / sprint): pres · şut hattı %36/%1 → %10/%46; arkada %39/%22 → %10/%5; güvence %4/%48 → %8/%0;
boşluk %8/%30 → %5/%3; geri koşu %42/%0 → %26/%1; markaj %39/%25 → %30/%5.
Kalan: uzak geri koşu hâlâ yürüyor (1543 Kaan/Tuna hedefe 55 br, taşıyıcı kaleye 33 br, v 0.10): sprintte bile T≈134 tik, tehlike anı
tD2≈50 → `1/(1+(T−tD2)/15)` geç kalanı yine ~0'a indiriyor, yarış modelinin kalıntısı. Hücumda kontra koşusu hiç depar atmıyor (%0, eskide %1).

## Düzeltme (02:45): "enerji hiç harcanmıyor" yanlıştı
Önceki turda 2:30'daki barlara bakmıştım: 4500. tik devre arası, barlar doluyor. Gerçek (bar ortalaması 1:15 / 3:45 / 4:58):
hiz2 yok 0.93 / 0.78 / 0.76; hiz2 ilk 0.76 / 0.87 / 0.92; S7k/S7l + dışbükey bedel 0.86 / 0.89 / 0.83. hiz2 biraz tutumlu, "hiç harcamıyor" değil.
## Kontra koşusu hâlâ depar atmıyor (0f15c40 sonrası %2)
796 Ok kontra: hedef (10,9) 36 br, bar 1.00, f=0 (tempo). Son tarih = en yakın savunmacının hedefe varışı (~50 tik); sprint T≈88, tempo T≈124:
g sprint≈0.115, tempo≈0.055 → fark ×Sy(0.26)≈0.016 < 88 tik sprintin bedeli (≈0.03). Geç kalan (15 tik ölçekli düşüş) yine iki seçeneği de
küçültüyor; kontrada asıl kazanç "her tik erken = geri dönemeyen bir savunmacı daha" (savunmacıların hepsinin geri dönüş süreleri) modelde yok.
(Not: ALAN_K3_IZ satırındaki tD eski değişken; hiz2'de kullanılan tD2 yazılmıyor.)
