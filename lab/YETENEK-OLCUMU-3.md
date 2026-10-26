# ALAN · Yetenek ölçümü 3: mekaniklerin sahadaki izleri (26 Ekim 2026)

> Motor: main b121ca4 (26.10 14:33; kopyası `lab/yt3/motor`). Araçlar: `tools/yet3-olc.js`, `tools/yet3-ozet.js`. Veriler: `lab/yt3/sonuc` (432 maç).
> İstek: skorlara değil, oyunda gerçekten ne olduğuna bakan, neyin nasıl çalışıp çalışmadığını anlayan bir ölçüm.

## Yöntem
- Herkes 10 (34 özellik), taktik varsayılan (2-2-2 / 2-1-3). A'da tek özellik **4** ya da **16**; saha özellikleri A'nın 6 saha oyuncusuna, Bekçi özellikleri A'nın Bekçi'sine. Markaj ayrıca **adam adama** düzende (iki takım `adam = 1`).
- 72 kol × 6 tohum × 90 sn = 432 maç. Her özellik için ana sayı: **aynı tohumda 16'lı maç − 4'lü maç** (etki iki kat, gürültü aynı).
- Maç başına ölçülenler (A ve rakip): pas (isabet, kesilen, ilerleme, boy, geçilen rakip, alıcının boşluğu, alıştan sonra 1 sn içinde kayıp); topu tutma (süre, topla gidilen yol); top kazanma (pas arası, boş top / sekme, taşıyıcıdan); şut (sayı, uzaklık, şarj, kafanın gol olasılığı, Bekçi kurtarışı); ikili; enerji (bar, dip, sprint, harcamanın türleri); alan kullanımı; denge kaybı; karar türleri; kafa ↔ maç sapması.
- Her özelliğe yetenek tablosundaki mekaniklerinden **beklenen izler** yazıldı (ör. Pas yükselince isabet artmalı, kesilen azalmalı); araç bunların anlamlı ve doğru yönde değişip değişmediğine bakıyor.

## Önce: motorun bugünkü taban hâli (uyarı)
Bütün kollar ortalaması, herkes 10, 90 sn:

| | 22.10 (yetenek ölçümü 1, taban) | bugün (b121ca4) |
|---|---|---|
| maç başına şut | 2,4 | **1,1** |
| pas isabeti | %90 | **%74** |
| kafa ↔ maç sapması | %7–9 (önceki ölçümler) | **%18** |

Şut yarıya inmiş, pasların dörtte biri tutmuyor, kafa maçı önceki günlerin iki katı sıklıkta yanlış tahmin ediyor. Bugünkü pas ve alıcı değişiklikleri (3c18eaa "alıcı pası bırakmaz… mücadeleye girer", b121ca4 "yedek buluşma") ilk aday; ayrıca ölçülmeli. **Şut çok az olduğu için şutla ilgili özellikler (Bitiricilik, Uzaktan şut, Yükleme'nin şarjı, bütün Bekçi özellikleri) bu ölçümde değerlendirilemedi.**

## Sonuç: beklenen izler


| özellik | beklenen izler (16 − 4) | karar | sonuç (xG farkı, gol farkı) |
|---|---|---|---|
| Hız | boş top / sekme kazanma +0.33; pas arası top kazanma +1.17; xG farkı +1.37 * ✓ | çalışıyor | +1.37 *, +0.67 |
| Patlayıcılık | boş top / sekme kazanma +0.50; ikili kazanma payı +0.18 * ✓ | çalışıyor | +0.48, +0.33 |
| Çeviklik | alıştan sonra 1 sn içinde kayıp −1.00; denge kaybı +0.001 | iz yok | +0.69, −0.17 |
| Güç | ikili kazanma payı +0.34; pas boyu +2.1 | iz yok | −0.12, −0.17 |
| Denge | denge kaybı −0.002 * ✓; ikili kazanma payı +0.19 | çalışıyor | +0.28, −0.17 |
| Gövde | ikili kazanma payı +0.27 | iz yok | +0.69, +0.17 |
| Uzanma | pas arası top kazanma +1.00; rakibin pas isabeti −0.009 | iz yok | +0.29, +0.00 |
| Dayanıklılık | bar +0.203 * ✓; dipte −0.088 * ✓ | çalışıyor | +0.10, +0.50 * |
| Toparlanma | bar +0.153 * ✓; dipte −0.062 * ✓ | çalışıyor | −0.22, −0.17 |
| İlk kontrol | alıştan sonra 1 sn içinde kayıp −0.67 | iz yok | −0.02, +0.00 |
| Teknik | alıştan sonra 1 sn içinde kayıp +0.33; topla gidilen yol +0.2 | iz yok | +0.59, +0.50 * |
| Sürme | topla gidilen yol +0.3 | iz yok | +0.05, +0.17 |
| Pas | pas isabeti A +0.008; pası kesilen A −0.018 | iz yok | +0.76 *, +0.17 |
| Bitiricilik | gol farkı −0.50 | iz yok | −0.36, −0.50 |
| Uzaktan şut | şut uzaklığı: –; şut A +0.2 | iz yok | +0.21, +0.33 |
| Aldatma |  | iz yok | +0.38, +0.17 |
| Alan gücü | alan: hızlandıran +0.015 * ✓; alan: yavaşlatan +0.020 * ✓; rakibin pas isabeti −0.031 | çalışıyor | +0.18, +0.00 |
| Alan kontrolü | harcamada alan payı −0.036 * ✓ | çalışıyor | +0.07, +0.17 |
| Yükleme | şut şarjı +0.01 | iz yok | +0.86 *, +0.17 |
| Tepki | pas arası top kazanma +1.33; boş top / sekme kazanma +0.50; kafa ↔ maç sapması −0.007 | iz yok | −0.28, −0.83 * |
| Okuma | pas arası top kazanma +0.83; kafa ↔ maç sapması −0.079 * ✓ | çalışıyor | +0.01, +0.17 |
| Vizyon | pas geçilen rakip +0.17; pas ilerlemesi −0.3 | iz yok | +0.10, +0.00 |
| Yaratıcılık | karar: kenardan pas payı +0.025 | iz yok | −0.13, −0.33 |
| Karar | kafa ↔ maç sapması +0.032; xG farkı +0.52 * ✓ | çalışıyor | +0.52 *, +0.33 |
| Soğukkanlılık | pas isabeti A +0.109 | iz yok | +0.48 *, +0.00 |
| Çalışkanlık | sprint payı −0.015 | iz yok | −0.49, −0.33 |
| Hücumda yer alma | alıcının boşluğu (br) +3.4 * ✓ | çalışıyor | −0.57, −0.17 |
| Savunmada yer alma | rakip alıcının boşluğu −3.9 * ✓; rakibin pas isabeti −0.022 | çalışıyor | −0.12, +0.17 |
| Kesme | pas arası top kazanma +1.33; rakibin pas isabeti −0.034 | iz yok | +0.21, +0.00 |
| Markaj | rakip alıcının boşluğu +0.2 | iz yok | +0.36, +0.17 |
| Refleks | rakip şutlarından Bekçi tuttu +0.33; yenen gol +0.00 | iz yok | −0.36, +0.00 |
| Yerleşim | rakip şutlarından Bekçi tuttu −0.25; yenen gol +0.00 | iz yok | −0.19, +0.00 |
| Alan hakimiyeti | rakip şutlarından Bekçi tuttu +1.00; yenen gol +0.00 | iz yok | −0.34, +0.00 |
| Birebir | rakip şutlarından Bekçi tuttu +0.50; yenen gol −0.17 | iz yok | +0.21, +0.17 |
| Markaj (adam adama) | rakip alıcının boşluğu +0.4 | iz yok | −0.12, +0.00 |

✓: beklenen yönde ve anlamlı; ✗: ters yönde ve anlamlı. "iz yok": beklenen göstergelerin hiçbiri anlamlı değişmiyor.

## Her özelliğin anlamlı etkileri (16 − 4; yalnız standart hatanın iki katını geçenler)

| özellik | anlamlı etkiler |
|---|---|
| Hız | xG farkı +1.4; rakip şut −2.0; alıcının boşluğu −6.8; rakip alıcının boşluğu +6.0; ikili +2.0; yenen gol −0.500; bar +0.053; sprint payı −0.065; harcamada alan payı +0.040; kafa ↔ maç sapması +0.140 |
| Patlayıcılık | pas ilerlemesi +5.2; ikili kazanma payı +0.181; bar −0.130; dipte +0.026; sprint payı −0.083; harcamada şarj payı −0.009 |
| Çeviklik | şut +1.0; sprint payı −0.093; alan: hızlandıran +0.006; sürme kararı payı +0.082; şut kararı payı +0.055 |
| Güç | alıştan sonra kayıp −1.2; şut uzaklığı +9.2; harcamada şarj payı −0.014; kenardan pas kararı payı +0.046 |
| Denge | rakip şut −0.500; denge kaybı −0.002 |
| Gövde | şut +0.500; boş top kazanma −1.2; rakip alıcının boşluğu +3.6 |
| Uzanma | alıcının boşluğu −4.2; boş top kazanma −1.3; sprint payı −0.028; sürme kararı payı −0.111 |
| Dayanıklılık | gol farkı +0.500; pas başına geçilen rakip −0.354; topla gidilen yol +0.529; taşıyıcıdan kazanma +0.667; bar +0.203; dipte −0.088; sprint payı +0.085; alan: yavaşlatan +0.012; sürme kararı payı +0.134 |
| Toparlanma | alıcının boşluğu −3.5; rakip pas isabeti −0.170; bar +0.153; dipte −0.062; sprint payı +0.066; harcamada şarj payı −0.009; alan: hızlandıran +0.009 |
| İlk kontrol | alıcının boşluğu −3.8; pas boyu −4.1; bar +0.037; harcamada alan payı −0.037; sürme kararı payı −0.078; pas kararı payı +0.077 |
| Teknik | gol farkı +0.500; pas isabeti +0.072; alıcının boşluğu −4.4; rakip pas isabeti −0.087; alan: hızlandıran +0.011 |
| Sürme | pas başına geçilen rakip +0.222; topu tutma (sn) +0.078; pas arası kazanma +1.2; rakip pas isabeti +0.042; rakip alıcının boşluğu +3.4; ikili kazanma payı +0.542 |
| Pas | xG farkı +0.758; şut +1.7; pas ilerlemesi +1.5; pas başına geçilen rakip +0.217; ikili kazanma payı −0.389; harcamada alan payı −0.031; alan: yavaşlatan −0.018 |
| Bitiricilik | harcamada şarj payı −0.010 |
| Uzaktan şut | pas isabeti +0.082; kesilen pas payı −0.123; pas başına geçilen rakip +0.240 |
| Aldatma | kesilen pas payı −0.062 |
| Alan gücü | taşıyıcıdan kazanma −0.500; ikili −1.7; şut uzaklığı +6.9; sprint payı −0.049; harcamada alan payı +0.086; alan: hızlandıran +0.015; alan: yavaşlatan +0.020 |
| Alan kontrolü | taşıyıcıdan kazanma +0.500; bar +0.052; harcamada alan payı −0.036 |
| Yükleme | xG farkı +0.865; ikili −1.2; ikili kazanma payı −0.467 |
| Tepki | gol farkı −0.833; kesilen pas payı +0.099 |
| Okuma | kafa ↔ maç sapması −0.079 |
| Vizyon | pas sayısı +7.8; topu tutma (sn) −0.103; topla gidilen yol −0.315 |
| Yaratıcılık | şut −0.500; şut kararı payı −0.012 |
| Karar | xG farkı +0.519; şut +0.500; bar +0.032; harcamada alan payı −0.040; harcamada şarj payı +0.009; şut kararı payı +0.010 |
| Soğukkanlılık | xG farkı +0.484; şut +0.500; rakip şut −0.833; kesilen pas payı −0.078 |
| Çalışkanlık | pas ilerlemesi +5.6; alıcının boşluğu −5.4; dipte +0.019 |
| Hücumda yer alma | pas sayısı −6.2; pas isabeti −0.074; alıcının boşluğu +3.4 |
| Savunmada yer alma | pas isabeti −0.088; rakip alıcının boşluğu −3.9 |
| Kesme | pas sayısı +4.0; taşıyıcıdan kazanma +0.833; harcamada alan payı −0.022; harcamada şarj payı +0.009; kafa ↔ maç sapması −0.082 |
| Markaj | boş top kazanma −0.500; kenardan pas kararı payı −0.063 |
| Refleks | pas boyu −3.8; ikili kazanma payı −0.700; sürme kararı payı −0.133 |
| Yerleşim | topu tutma (sn) −0.154; rakip alıcının boşluğu +3.0; harcamada şarj payı −0.012; pas kararı payı +0.130; kafa ↔ maç sapması +0.089 |
| Alan hakimiyeti | pas başına geçilen rakip +0.262 |
| Birebir | — |
| Markaj (adam adama) | rakip pas isabeti −0.067; xG/şut −0.356; alan: yavaşlatan −0.017 |

## Okuma

### Mekanizması çalışan (beklenen izler anlamlı ve doğru yönde)
- **Hız:** xG farkı +1,4 \*, rakip şut −2,0 \*, yenen gol −0,5 \*. Hâlâ en etkili özellik. Yan izler: hızlı takımın alıcıları daha az boş (−6,8 br) ama hızlıları kovalayan rakibin alıcıları daha boş (+6,0); **kafa ↔ maç sapması +0,14 \***: hızlı oyuncuda kafa maçı belirgin biçimde daha sık yanlış tahmin ediyor (kafa hızın sonucunu iyi kestiremiyor).
- **Patlayıcılık:** ikili kazanma payı +0,18 \*, pas ilerlemesi +5,2 \*. Bedeli: bar −0,13 \*, dipte +%2,6 \* (patlayıcı oyuncu daha çok yoruluyor). Hızlanma ile yorgunluk arasında beklenen artı-eksi kurulmuş.
- **Denge:** denge kaybı azalıyor \*, rakip şut −0,5 \*.
- **Dayanıklılık:** bar +0,20 \*, dipte −%8,8 \*, gol farkı +0,5 \*. Büyük barla daha çok sprint (+%8,5 \*), sürme (+%13 \*) ve taşıyıcıdan top kazanma (+0,67 \*).
- **Toparlanma:** bar +0,15 \*, dipte −%6,2 \*. Rakibin pas isabeti −%17 \* (daha çok basabildiği için).
- **Alan gücü:** alan hızlandıran ve yavaşlatan payı artıyor \*, alana harcama +%8,6 \*; şut uzaklığı +6,9 \* (alanla uzaktan şut). Alan kullanım ölçümündeki menzil sorunuyla tutarlı: menzil büyüyünce alan devreye giriyor.
- **Alan kontrolü:** alanın bedeli düşüyor (harcamada alan payı −%3,6 \*), bar +0,05 \*. Önceki ölçümdeki ters çalışma görünmüyor.
- **Okuma:** kafa ↔ maç sapması −%7,9 \*. İyi okuyan oyuncunun kafası maça daha yakın.
- **Karar:** xG farkı +0,52 \*, şut +0,5 \*. Önceki ölçümde eksiydi; artık artı.
- **Hücumda yer alma:** alıcının boşluğu +3,4 br \*. Yan etki: pas −6,2 \*, isabet −%7,4 \* (oyuncular koşuda, pas veren daha az seçenek buluyor ya da koşan alıcıya pas daha zor).
- **Savunmada yer alma:** rakip alıcının boşluğu −3,9 br \*. Yan etki: kendi pas isabeti −%8,8 \*.

### Etkili ama beklenen izden değil
- **Pas:** isabet ve kesilen değişmiyor, ama xG farkı +0,76 \*, şut +1,7 \*, pas ilerlemesi +1,5 \*, geçilen rakip +0,22 \*. Pas artık **koridordan** değer kazanıyor: iyi pasçı daha ileri ve savunmayı geçen pas atıyor (önceki ölçümde önerilen değişiklik gelmiş görünüyor).
- **Teknik:** pas isabeti +%7,2 \*, rakip pas isabeti −%8,7 \*, gol farkı +0,5 \*.
- **Sürme:** pas geçilen rakip +0,22 \*, ikili kazanma payı +0,54 \*, pas arası kazanma +1,2 \*. Önceki ölçümde ters çalışıyordu; artık artı.
- **Uzaktan şut:** pas isabeti +%8,2 \*, kesilen −%12 \*, geçilen rakip +0,24 \*. Şut özelliği pası iyileştiriyor: hangi mekanikten geldiği belirsiz, incelenmeli (şut tehdidi savunmayı mı çekiyor, yoksa bir pas mekaniği yanlışlıkla Uzaktan şut'u mu okuyor).
- **Soğukkanlılık:** xG farkı +0,48 \*, rakip şut −0,83 \*, kesilen pas −%7,8 \*.
- **Kesme:** taşıyıcıdan kazanma +0,83 \*, kafa sapması −%8,2 \*. Beklenen "pas arası kazanma" anlamlı değil (+1,3).
- **Yükleme:** xG farkı +0,86 \*; şut şarjı izi ölçülemedi (şut az). Yan etki: ikili −1,2 \*, ikili kazanma −0,47 \*.

### Şüpheli ya da ters
- **Tepki:** gol farkı −0,83 \*, kesilen pas payı +%9,9 \*. Beklenen kazanma ve sapma izleri anlamsız. Tepkili oyuncu daha erken karar verip kapanan yola mı pas atıyor? İncelenmeli.
- **Çalışkanlık:** alıcının boşluğu −5,4 \*, dipte +%1,9 \*, pas ilerlemesi +5,6 \*. Çok çalışan takım ilerliyor ama alıcıları baskı altında; sonuç eksi (xG −0,49, anlamsız).
- **Uzanma:** alıcının boşluğu −4,2 \*, boş top kazanma −1,3 \*, sürme kararı −%11 \*. Beklenen kesme izi yok, boş top kazanma ters.
- **Gövde:** boş top kazanma −1,2 \*, rakip alıcının boşluğu +3,6 \*: ters yönde.
- **İlk kontrol:** alıcının boşluğu −3,8 \*, pas boyu −4,1 \*: daha kısa pas, alıcı daha baskıda. Beklenen "alıştan sonra kayıp" izi anlamsız.
- **Bekçi özellikleri** (Refleks, Yerleşim) sahadaki oyunu değiştiriyor: Refleks 16'da ikili kazanma −0,70 \*, sürme kararı −%13 \*; Yerleşim 16'da pas kararı +%13 \*, kafa sapması +%8,9 \*. Bekçi'nin oyun kurma kararları bu özelliklerle değişiyor; şut az olduğu için asıl işleri (kurtarış) ölçülemedi.

### İz bırakmayanlar
- **Bitiricilik, Birebir, Alan hakimiyeti:** şut az olduğu için ölçülemedi (anlamlı tek etki yok ya da tek önemsiz iz).
- **Aldatma:** yalnız kesilen pas −%6,2 \*; fake sayısı motorda artık sayılmıyor (`m.st.fake` yok), fake'in kendisi ölçülemedi.
- **Yaratıcılık:** kenardan pas kararı +%2,5 (anlamsız), şut −0,5 \*.
- **Vizyon:** pas sayısı +7,8 \*, tutma −0,10 sn \*: iyi gören oyuncu topu daha çabuk bırakıyor; ilerleme ve geçilen rakip izi yok.
- **Markaj:** varsayılan düzende de adam adama düzende de beklenen iz yok. Adam adama düzende rakip pas isabeti −%6,7 \*, rakibin şut başına xG'si −0,36 \*: işe yarıyor olabilir ama "rakip alıcının boşluğu" değişmiyor.

## Sonuç
1. **Mekanizma düzeyinde çalışan özellik sayısı arttı:** fizik ve enerji (Hız, Patlayıcılık, Denge, Dayanıklılık, Toparlanma), ALAN (Alan gücü, Alan kontrolü), algı ve karar (Okuma, Karar), yer alma (iki yönde) beklenen izleri bırakıyor. Pas, Teknik, Sürme ve Soğukkanlılık başka yollardan sonuca etki ediyor; önceki ölçümlerdeki Sürme, Alan kontrolü ve Karar ters çalışmaları görünmüyor.
2. **Hız hâlâ en etkili** ve kafanın en çok yanıldığı özellik.
3. **Şüpheliler:** Tepki (gol −0,83), Uzanma ve Gövde (boş top kazanma ters), Çalışkanlık, İlk kontrol; Uzaktan şut'un pası iyileştirmesi.
4. **Önce motorun taban hâli:** şut yarıya, pas isabeti %74'e indi, kafa sapması %18. Bunun kaynağı (büyük olasılıkla bugünkü pas ve alıcı değişiklikleri) bulunmadan şut ve Bekçi özellikleri ölçülemez.

## Sınırlar
Kol başına 6 maç × 90 sn; anlamlılık eşiği iki standart hata, 72 kol × 40 gösterge içinde birkaç "anlamlı" sonuç rastlantı olabilir. Tek taktik, herkes 10.

## Ek · Taban düşüşünün kaynağı (aynı akşam)

> Herkes 10, varsayılan taktik, 3 tohum × 90 sn, motor başına 3 maç (az; yön güvenilir, sayılar kaba). Veriler `lab/yt3/taban-kok`. `tools/yet3-olc.js`'e deney bayrağı desteği eklendi (`BAYRAK=ALAN_PRENSIP=false`).

| motor | şut / maç | pas isabeti | kesilen pas | kafa ↔ maç sapması |
|---|---|---|---|---|
| 614496b (23.10 16:30) | 1,7 | %85 | %9 | %14 |
| 7062915 (24.10 22:21, prensip varsayılandan hemen önce) | 1,3 | %88 | %7 | %13 |
| **9fa249b (24.10 23:12, "Prensip varsayılan kafa")** | 1,0 | **%53** | **%27** | %15 |
| 452ee4f (25.10 17:29) | 1,7 | %70 | %22 | %17 |
| 7c6a3e1 (26.10 11:21) | 1,0 | %70 | %23 | %16 |
| b121ca4 (bugün) | 0,3 | %71 | %16 | %20 |
| b121ca4 + eski kafa (`ALAN_PRENSIP = false`) | 0,3 | %81 | %13 | %22 |

- **Pas isabetindeki düşüş prensipli kafayla geldi** (9fa249b, kullanıcı kararıyla varsayılan yapıldı): isabet %88 → %53, kesilen %7 → %27. Sonraki "Prensip:" düzeltmeleri isabeti %70'e çıkardı; bugünkü motorda eski kafa hâlâ %81 veriyor. Prensipli kafa pası daha riskli seçiyor ya da araya girmeyi kötü tartıyor; bugünkü farkın kaynağı ayrıca izlenmeli (kesilen pasların durumu, kafanın pasın kesilme ihtimali tahmini).
- **Şuttaki düşüş prensipten bağımsız:** eski kafayla da bugün maç başına 0,3. 26.10'daki son değişiklikler (imkân haritası, alıcı ve pas, şut tablosu: "dibinde savunmacı olan şutçuya şut yok") aday; 3 maçla kesin değil.
- Kafa ↔ maç sapması iki kafada da %20 civarı; günler içinde %13'ten yükselmiş.
