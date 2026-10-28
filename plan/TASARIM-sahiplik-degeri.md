# Tasarım planı · Sahiplik değeri (topun bizde olmasının değeri)

> Durum: tasarım, kod yok. 28 Ekim 2026. Pas hatalarının incelemesinden çıktı (tohumlar 462566, 1101, 1102). Bu bir yama katsayısı değil; kafanın değer ölçüsünde eksik olan bir kavram.

## 1 · Sorun

Kafa her seçeneği aynı birimle tartıyor: **gol olasılığı**. Bugün iki şeyi ölçüyor:
- **Tutulursa değer** (`durum`): oradan ufuk içinde doğacak şut imkânı (imkân haritası). İmkân görünmeyen yerde taban değer (`zemin`: taşıma, ilerleme) devreye giriyor.
- **Kaybın bedeli** (`lossAt`): topu kazanan rakibin o noktadaki imkânı ve yapabileceği pas (kontraKur).

İkisi de **"buradan hemen ne doğar"** sorusunu soruyor, **"bu topa sahip olmak ne kazandırır"** sorusunu değil. Kafanın kendi dökümünden ölçüm (1102, 4500 tik):

| Bölge (bizim yönümüzde) | Topu tutmanın değeri | Riskli seçenekte tutulursa | Kaybın bedeli |
|---|---|---|---|
| Kendi kale önü | 0,004 | 0,004 | 0,127 |
| Kendi yarı | 0,009 | 0,005 | 0,117 |
| Orta saha | 0,007 | 0,007 | 0,017 |
| Rakip yarı | 0,038 | 0,024 | 0,005 |
| Rakip kale önü | 0,050 | 0,035 | 0,001 |

**Sonuç:** Orta sahada ve ileride topu kaybetmek neredeyse bedava. Kafa %13 tutulacak bir ara pası %100 güvenli tutmaya tercih ediyor:
- 1102 03:07 Mert: pas 0,003, tutmak 0,001.
- 1102 03:23 Hendek: %2,5'lik pas 0,000, sürme 0,001.

Seçim hatası düzeltildikten sonra (sürüm 202610281120) bu tablo kafanın gerçek tercihi oldu:
- %85'in altında kaybedilen pas: 35 → 47 (3 maç).
- Toplam pas kaybı düşmedi: 57 → 59.

Kendi yarımızda kafa doğru davranıyor, çünkü orada kaybın bedeli büyük. Sorun orta saha ve ileride.

**Futbolda olan:** Top bizdeyken bu atağın ve topu tuttukça gelecek atakların değeri var. Rakipteyken onların atağının değeri var. Orta sahada topu tutmak, "buradan şut yok" diye sıfır değildir.

## 2 · Kavram

**Sahiplik değeri N(takım, yer):** Takım topu bu yerde kontrol ediyorken, bu sahiplikten **bizim gol atmamız ile rakibin gol atması arasındaki olasılık farkı**. Kendi kale önünde, kaybedersek rakip hemen vurabileceği için eksiye de düşebilir.

Kafanın kullanacağı iki ölçü:
- **Tutulursa değer** = N(biz, seçeneğin bitiş yeri). Bugünkü max(imkân, zemin) yerine geçer.
- **Kaybın bedeli** = N(rakip, topun kaybedildiği yer). Topu orada kazanan rakibin sahipliğinin değeri: hemen şut, kontra pası (kontraKur), oradan kuracağı oyun.

Birim aynı kalır, gol olasılığı. Şut, pas, sürme, tutma aynı ölçüyle tartılmaya devam eder.

## 3 · N nasıl hesaplanır: formülle zincir

Sahiplik bir zincir: top sahibi ya vurur, ya topu başka bir yere götürür (pas ya da sürme) ve orada yeni bir top sahibi olur, ya da topu kaybeder ve rakip orada top sahibi olur. N bu zincirin değeri; geriye doğru, birkaç adımda hesaplanır (değer yinelemesi).

**Izgara:** Saha, imkân haritasının bölge ızgarasıyla (6 br) hücrelere bölünür. Her takım için bir alan: N_biz(hücre), N_rak(hücre).

**Başlangıç (0. adım):** Her hücrenin değeri oradan hemen şutun değeri, yani imkân haritası (`imkan`, kafa ve maçın şut kuralıyla). Haritanın erimi dışında 0.

**Bir adım (her hücre için, iki takımda birlikte):**

  N(c) = max( şut(c),  max over c' [ k(c→c') · γ · N(c')  −  (1 − k(c→c')) · N_karşı(kayıp yeri) ] )

- **c':** Topun c'den gidebileceği yerler. Pas için kendi oyuncumuzun rakipten önce varabildiği hücreler (pas menzili içinde); sürme için komşu hücreler.
- **k(c→c'):** Topun oraya bizde kalarak varma ihtimali. Maçın kuralıyla, imkân haritasının bugün kullandığı yarışla hesaplanır: kim nereye önce varır (`gecTab`, `vSp`, tepki), pas yolu açık mı (`laneOk`, `bekYol`). Kafa = maç: yeni bir olasılık kuralı yok.
- **Kayıp yeri:** Yolun üstünde rakibin önce varabildiği nokta; topu orada rakip kazanır.
- **γ:** Bir adımın indirimi, zincirde bugün kullanılan `combo` payı. Her pas ve sürme bir adım; adım arttıkça hem süre geçer hem dünya değişir, uzak gelecek daha az kesin.

**Kaç adım:** 3–4. Her adımda bilgi bir pas ötesine yayılır. Böylece orta sahadaki hücre "iki pas sonra kale önüne varabilmenin" değerini alır. `combo` indirimi zinciri doğal olarak kısar.

**İki alan birlikte:** Bizim değerimiz kaybın bedelinde rakibin alanını, rakibinki bizimkini kullanır. Aynı adımda ikisi birlikte güncellenir.

**Dünya:** Kararın tahmin dünyası (bugünkü imkân haritasıyla aynı dünya). Zincirin ilerleyen adımlarında oyuncuların yerleri sabit sayılır, imkân haritasında olduğu gibi kimin ne karar vereceği tahmin edilmez. `combo` indirimi bu belirsizliğin de payı.

**Neden bu, `zemin`'in yerini alır:** zemin (taşıma, ilerleme) "ileri gitmek değerlidir" diyen el yazımı bir eğri. Zincir aynı şeyi kendiliğinden söylüyor: ileri giden yolun değeri, varılan yerden açılan imkândır. İkisi birlikte kalırsa aynı şey iki kez sayılır.

## 4 · Kafada nereye girer

| Yer | Bugün | Sonra |
|---|---|---|
| Tutulursa değer (pas, önüne, kenardan, sürme, tut) | max(imkân, zemin) | N_biz(bitiş yeri), en az o anki imkân |
| Kaybın bedeli (`lossAt`, A2) | rakip imkânı, kontraKur | N_rak(kayıp yeri) |
| Şut | gol olasılığı − kaybın bedeli | aynı, kaybın bedeli N_rak ile |
| Topsuz oyuncunun yer tartısı (A3) | yerDeger | değişmez (bu tasarımın dışında) |

`risk` talimatı kaybın bedelini bugünkü gibi çarpar.

## 5 · Hız

Izgara yaklaşık 20 × 10 = 200 hücre, takım başına. Her hücreden 20–30 hedef, 3–4 adım, iki takım: kararda birkaç on bin yarış hesabı.
- Alan **karar başına bir kez** kurulur, kararın bütün seçenekleri onu okur (imkân haritasının önbelleği gibi: `cx.IM`).
- **İleri bakış kopyalarında** (kafanın içindeki öteki oyuncuların kararları) alan yeniden kurulmaz, ana kararın alanı kullanılır.
- Telefonda karar süresi ölçülür; maçın akışına yetişmezse adım sayısı ya da hedef sayısı düşer (davranış ölçülerek).

## 6 · Beklenen etki ve riskler

**Beklenen:**
- Orta sahada %10–40'lık paslar azalır. Kayıp artık rakibin sahipliğinin değeri kadar bedelli, tutmak da iki pas sonraki imkân kadar değerli.
- Rakip yarıda risk hâlâ alınır, çünkü şut imkânı yüksek; ama tamamen bedava olmaz.
- Kendi yarımızda bugünkü temkin korunur.

**Riskler:**
- **Fazla temkin:** Top yan yana dolaşır, kimse ileri pas vermez. Ölçüt: ileri pas oranı, şut sayısı, gol.
- **İyimserlik:** Zincir her adımda en iyi devamı seçer; oyuncu her adımda en iyiyi seçmez (seçim hatası). Uzun zincirde bu fark birikir. `combo` indirimi ve kısa zincir (3–4 adım) bunu sınırlar; kafa ↔ maç dedektörüyle ölçülür.
- **Sabit dünya:** Zincirin ilerleyen adımlarında savunma kaymıyor sayılıyor. Uzak pasların değeri biraz şişebilir.
- **Tarzlar:** Direkt oyun (uzun top) kaybın bedelini daha çok yer. Tarz matrisinde ölçülmeli.

## 7 · Karar bekleyenler

1. **Adım sayısı:** 3 mü, 4 mü?
2. **Pas hedefleri:** Yalnız arkadaşın önce vardığı hücreler mi (önerim), yoksa menzildeki bütün hücreler (koşu yaratma dahil) mi?
3. **İki takım birlikte (fark değeri) mi**, yoksa kaybın bedeli için yalnız rakibin alanı mı? Önerim birlikte: kendi kale önünde tutmanın eksi değeri ancak böyle ortaya çıkar.
4. **`zemin` tamamen kalksın mı?** Önerim kalksın, aksi halde ilerleme iki kez sayılır.

## 8 · Ölçüm (uygulama sonrası)

- Pas kaybı bölgeye göre (özellikle orta saha), %50'nin altında seçilen pas sayısı.
- İleri pas oranı, şut sayısı, gol (fazla temkin kontrolü).
- Kafa ↔ maç sapma dedektörü: kayıp sapmaları.
- Karar süresi (telefonda maçın akışına yetişiyor mu).
- Aynı tohumlar: 462566, 1101, 1102 eski/yeni.
