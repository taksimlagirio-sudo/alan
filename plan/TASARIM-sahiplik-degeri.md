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

**Sahiplik değeri S(takım, yer, durum):** Takım topu bu yerde, bu durumda kontrol ediyorken bu sahipliğin gol ile bitme olasılığı.

- **Rakibin sahiplik değeri S_r:** Rakip topu bu yerde kazanırsa, onun bu sahipliğinin gol ile bitme olasılığı. Kontra burada: top kaybından başlayan sahiplik, karşı takım dağınık başlar.

Kafanın kullanacağı iki ölçü:
- **Tutulursa değer** = max(imkân, S). İmkân "buradan şimdi açık olan"; S "bu sahipliğin ortalama geleceği". İyi bir an ortalamanın üstündedir, ortalama ise tabandır. Bugünkü `zemin` tabanının yerine S geçer.
- **Kaybın bedeli** = max(rakibin imkânı ve zinciri (bugünkü), S_r). Kaybı rakibin geçiş sahipliği olarak sayar.

Birim aynı kalır, gol olasılığı. Şut, pas, sürme, tutma aynı ölçüyle tartılmaya devam eder.

## 3 · S nereden gelir: ölçülmüş tablo (önerim)

**Önerim:** S'yi motorun kendi maçlarından ölçmek. Şut tablosunda (`shot-table.js`) yaptığımızın aynısı.

- **Nasıl:** Her sahiplik kaydedilir: hangi takım, hangi tikten hangi tike, her tikte top nerede. Sahiplik gol ile bittiyse o sahipliğin her anı "1", bitmediyse "0" yazar. Bir bölgedeki ortalama, o bölgede topa sahip olmanın gol olasılığıdır.
- **S_r:** Yalnız top kaybıyla başlayan sahipliklerin başlangıç noktasından ölçülür (kontra dahil).
- **Kafa = maç:** Tablo maçın kendisinden geliyor. Kafa kendi motorunun gerçeğini bilir, uydurma bir eğri değil.
- **Döngü:** Tablo kafanın kararlarını değiştirir, kararlar da tabloyu. Ölç → yükle → yeniden ölç, değişim küçülene kadar (2–3 tur). Tablonun her sürümü ölçüldüğü motor sürümüyle etiketlenir.

**Alternatif: formülle zincir.** S(x) = en iyi devam seçeneğinin değeri, adım adım geriye doğru (değer yinelemesi). Daha "temiz" görünür ama her kararda pahalı. Ayrıca kafanın tahminiyle beslendiği için iyimserliğini kendine yansıtır. Ölçülen tablo bunu düzeltir. Önerim tablo.

## 4 · Tablonun boyutları

Az boyut, bol veri. Öneri:
- **Boy:** Kaleye uzaklık, 10 dilim (12 br).
- **En:** Orta, iç kanat, kenar (3 dilim).
- **Durum:** Rakip yerleşik mi, geçişte mi (`m.gc`, bugünkü geçiş tanımı), 2 dilim.

Toplam 60 hücre. 5 dakikalık bir maçta birkaç gol var (son üç maçta 2–6); hücreleri ayırt edecek kadar gol görmek için **60–100 maç** gerekir. Komşu hücreler yumuşatılır (az veri olan hücre komşusuna yaslanır).

Bilerek koymadıklarım:
- **Baskı ve oyuncu yeteneği:** Bunlar seçeneğin tutma ihtimalinde (P) zaten var. Tabloya eklersek iki kez sayılır.
- **Skor ve kalan süre:** 5 dakikalık maçta önemli olabilir ama ayrı bir konu. Talimat olarak "skoru koru / risk al" (`risk` talimatı kaybın bedelini zaten çarpıyor).

## 5 · Kafada nereye girer

| Yer | Bugün | Sonra |
|---|---|---|
| Tutulursa değer (pas, önüne, kenardan, sürme, tut) | max(imkân, zemin) | max(imkân, S) |
| Kaybın bedeli (`lossAt`, A2) | rakip imkânı, kontraKur | max(o, S_r) |
| Şut | gol olasılığı − kaybın bedeli | aynı, kaybın bedeli yeni ölçüyle |
| Topsuz oyuncunun yer tartısı (A3) | yerDeger | değişmez (bu tasarımın dışında) |

## 6 · Beklenen etki ve riskler

**Beklenen:**
- Orta sahada %10–40'lık paslar azalır. Kayıp artık en az ~0,01–0,03 (S_r), tutmak en az S.
- Rakip yarıda risk hâlâ alınır, çünkü imkân S'nin üstüne çıkar; ama tamamen bedava olmaz.
- Kaybın kontraya dönüştüğü yerlerde (rakip dağınık değil, biz dağınıkken) kafa daha temkinli olur.

**Riskler:**
- **Fazla temkin:** Top yan yana dolaşır, kimse ileri pas vermez. Ölçüt: ileri pas oranı, şut sayısı, gol. Döngüde S küçülürse oyun kendini dengeler (temkinli oyun gol üretmezse S düşer).
- **Beklemenin bedava üstünlüğü:** Tutmanın tabanı S. Aynı bölgedeki arkadaşa pas da S taşır, yani ikisi eşit. Bugünkü `combo` indirimi devam seçeneklerine uygulanmaya devam eder, tutmaya üstünlük doğmaz.
- **Tablo bayatlaması:** Motor değiştikçe tablo eskir. Her önemli motor değişikliğinden sonra yeniden ölçüm gerekir (şut tablosunda olduğu gibi).
- **Tarzlar:** Direkt oyun (uzun top) S_r'yi daha çok yer, çünkü uzun top daha çok kaybedilir. Bu doğru ama tarz matrisinde ölçülmeli. Dikine'nin baskınlığı değişebilir.

## 7 · Karar bekleyenler

1. **Kaynak:** Ölçülmüş tablo (önerim) mi, formülle zincir mi?
2. **`zemin` tabanı:** Tamamen S ile değişsin mi, yoksa ikisinin büyüğü mü kalsın? Öneri: değişsin. zemin el yazımı bir eğri, S ölçülmüş.
3. **Tablonun boyutu:** 10 × 3 × 2 yeterli mi? Geçiş ayrımı dışında bir şey (örneğin topun gerisindeki rakip sayısı) gerekli mi?
4. **Ölçüm:** 60–100 maçlık arka plan koşusu kabul mü? Bir maç bu makinede ~10 dakika sürüyor; 4 işlemciyle 60 maç ~2,5 saat, 100 maç ~4 saat.

## 8 · Ölçüm (uygulama sonrası)

- Pas kaybı bölgeye göre (özellikle orta saha), %50'nin altında seçilen pas sayısı.
- İleri pas oranı, şut sayısı, gol (fazla temkin kontrolü).
- Kafa ↔ maç sapma dedektörü: kayıp sapmaları.
- Tarz matrisi: Dikine, pres, derin blok.
- Aynı tohumlar: 462566, 1101, 1102 eski/yeni.
