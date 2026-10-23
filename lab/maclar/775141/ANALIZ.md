# Maç 775141 · Liman – Kale Duvarı: "oyuncular salaklaşmış mı?" (23 Ekim 2026, 01:07)

> Kayıt `kayit.json`. Araç `zeka.js` (pas, kayıp, şut, kafa ↔ maç sapması), özet `ozet.js`. Aynı kurulum (Liman'ın taktiği ve dizilişi, Kale Duvarı, herkes 20), 6 tohum × 90 sn, beş motor. Veriler `z/`.

## Maçın kendisi (91ceeb8, tam süre: `tam-91ceeb8.json`)
**Liman 0–1.** Liman 76 pas (%93), 10 şut, xG 2,7; Kale Duvarı 26 pas, 12 şut, **xG 6,6** (şut başına 0,55, ortalama 23,5 br). Aynı kurulum 21 Ekim motorunda 12 tohumda ortalama +1,2 golle kazanıyordu.

## Motor motor (6 maç × 90 sn, Liman)

| motor | skorlar | pas / 90 sn | pas isabeti | şut | xG | kafa ↔ maç sapması |
|---|---|---|---|---|---|---|
| 1ab1167 (21.10, 5–1 maçı) | 1-1 3-0 0-0 2-1 0-1 2-0 | 31,3 | %95,8 | 5,0 | 3,30 | %8,5 |
| 64d5e04 (22.10 18:15, ince yapı 1) | 0-0 0-1 1-1 1-0 1-1 1-0 | 27,5 | %94,6 | 5,2 | 2,08 | %6,9 |
| 752074d (+ görme) | 0-0 0-0 0-0 0-0 1-0 0-1 | 28,3 | %91,8 | 5,3 | 2,44 | %8,6 |
| 6e976f3 (+ şutun değeri) | 0-0 0-0 2-1 0-0 0-1 0-1 | 26,5 | %88,1 | 3,5 | 1,82 | %9,6 |
| 91ceeb8 (+ sürme-ikili 4a–4c) | 0-0 0-0 0-0 1-0 2-0 0-0 | **20,0** | %90,8 | 3,7 | 2,05 | %6,8 |

(Sapma iki takımın toplamı.)

## Okuma
- **Evet, oyun belirgin şekilde durgunlaştı:** Liman'ın pası 31 → 20 (−%36), isabeti %95,8 → %90,8, şutu 5,0 → 3,7; 6 maçın 4'ü 0–0. Gol de azaldı (eski motorda 6 maçta 10 gol, bugün 3).
- Adım adım (her motor 6 maç; gürültülü, yön güvenilir):
  - **Görme (752074d):** pas sayısı aynı, isabet −3 puan, kafa ↔ maç sapması %6,9 → %8,6. Oyuncu algıladığı (eski, gürültülü) dünyayla karar verince bazı paslar kapanmış yollara gidiyor.
  - **Şutun değeri (6e976f3):** Liman'ın şutu 5,3 → 3,5. Kafa artık şutun Kuyu'ya girme olasılığını direk açılarından tartıyor; daha seçici. Uzak şutun fazla değerli sayılması sorunu düzelirken şut sayısı düştü.
  - **Sürme ve ikili (91ceeb8):** pas 26,5 → 20,0. Kafanın topu elde tutma ihtimali maçın ikili kurallarına yaklaştı; oyuncular topu daha çok tutuyor ve sürüyor, daha az pas veriyor. "Bekliyor, düşünüyor, sonra kaybediyor" görüntüsü buradan olabilir.
- Kale Duvarı'nın tam maçta 6,6 xG üretmesi, Liman'ın yüksek bloğunun arkasının bu motorda daha kolay açıldığını gösteriyor; 6 kısa maçta görülmedi (KD orada az şut attı), tek maça özgü olabilir.

## Sonraki adım (önerim)
- Topu tutma ve sürme kararlarının değeri izlenmeli (kafa ↔ maç kaydında "tut" ve "sür"den sonra ne oluyor), çünkü pas düşüşünün en büyüğü sürme-ikili adımlarında.
- Görme katmanında pas isabetinin düşmesi beklenen bir bedel; ama oyuncunun kendi görme payını bilerek daha güvenli seçmesi gerekirdi (kafa kendi yanılgısını bilmeli). Sapma oranı burada arttı.

## Ek · Karar kalitesi: tutma, pas, şut (23 Ekim)

> Araç `kalite.js` (kararın tam seçenek listesi match.js'e yükleme sırasında eklenen bir kancayla okunur; maç değişmez), özet `kq-ozet.js`. Aynı kurulum, tohum 11–16 × 90 sn, beş motor. Veriler `kq/`.

### Liman (maç başına, 90 sn)

| motor | maç | skorlar | karar: tut / sür / pas / önüne / kenardan / şut | tutma sayısı | ort. tutma (sn) | pas | kesilen | ilerleme (br) | boy | alıcıya en yakın rakip | geçilen rakip | pastan sonra 2 sn içinde kayıp | pastan sonra 5 sn içinde şut |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 21.10 (5–1) | 6 | 0-0 1-0 1-0 0-0 1-0 3-1 | 50 / 12 / 14 / 8 / 8 / 4 | 41.0 | 0.54 | 35.8 | 7% | 10.2 | 22.9 | 9.5 | 0.45 | 21% | 50% |
| ince yapı 1 | 6 | 1-0 0-0 2-0 1-1 0-0 0-0 | 42 / 24 / 12 / 5 / 8 / 4 | 35.5 | 0.69 | 30.0 | 6% | 9.1 | 22.0 | 10.5 | 0.37 | 19% | 40% |
| + görme | 6 | 2-0 0-0 2-0 1-0 1-1 0-0 | 39 / 25 / 13 / 6 / 6 / 7 | 35.7 | 0.75 | 29.5 | 11% | 12.1 | 23.9 | 11.8 | 0.44 | 31% | 41% |
| + şut değeri | 6 | 2-0 1-0 0-0 0-0 0-0 0-0 | 34 / 26 / 13 / 8 / 8 / 6 | 33.3 | 0.70 | 28.2 | 10% | 11.0 | 24.4 | 11.7 | 0.42 | 22% | 37% |
| + sürme-ikili (bugün) | 6 | 0-0 0-0 0-0 0-0 1-0 0-1 | 28 / 41 / 10 / 6 / 6 / 3 | 31.8 | 0.95 | 26.5 | 9% | 8.8 | 23.3 | 11.9 | 0.19 | 15% | 27% |

**Tutmanın süresine göre** (tutmaların payı · sonraki 5 sn içinde şut · savunmanın yerleşmesi: tutma boyunca topun kale tarafına geçen rakip sayısı)

| motor | < 0,5 sn | 0,5–1,5 sn | 1,5–3 sn | > 3 sn |
|---|---|---|---|---|
| 21.10 (5–1) | %69 · şut %47 · 0.09 | %22 · şut %43 · 0.18 | %6 · şut %29 · 0.36 | %4 · şut %33 · 0.44 |
| ince yapı 1 | %63 · şut %38 · 0.09 | %23 · şut %35 · 0.15 | %10 · şut %48 · 0.05 | %5 · şut %10 · 0.20 |
| + görme | %59 · şut %44 · 0.06 | %29 · şut %29 · 0.08 | %7 · şut %43 · 0.07 | %5 · şut %18 · 0.09 |
| + şut değeri | %60 · şut %39 · 0.06 | %25 · şut %26 · 0.06 | %13 · şut %32 · 0.40 | %3 · şut %0 · 0.00 |
| + sürme-ikili (bugün) | %60 · şut %31 · 0.02 | %19 · şut %16 · 0.14 | %12 · şut %18 · 0.23 | %9 · şut %12 · -0.12 |

**Şut** (iki takımın toplamı: kafanın şut anındaki gol olasılığı toplamı ile gol; bu takımın kararlarında gol olasılığı ≥ 0,3 şut seçeneği varken başka hamle)

| motor | şut | beklenen gol | gol | yüksek olasılıklı şut seçeneği (karar) | bırakılan | bırakılınca seçilen | bırakılan şutun olasılığı | vurulanın olasılığı |
|---|---|---|---|---|---|---|---|---|
| 21.10 (5–1) | 47 | 24.5 | 7 | 80 | 63 (%79) | tut 55, pas 4, fake 2, kenardan 1, önüne 1 | 0.44 | 0.79 |
| ince yapı 1 | 42 | 17.5 | 5 | 21 | 2 (%10) | kenardan 1, tut 1 | 0.47 | 0.50 |
| + görme | 52 | 25.1 | 7 | 31 | 3 (%10) | tut 2, pas 1 | 0.42 | 0.70 |
| + şut değeri | 50 | 21.1 | 3 | 22 | 4 (%18) | sür 2, kenardan 1, pas 1 | 0.41 | 0.60 |
| + sürme-ikili (bugün) | 38 | 15.2 | 2 | 19 | 7 (%37) | tut 2, fake 2, kenardan 1, önüne 1, sür 1 | 0.41 | 0.51 |

### Kale Duvarı (maç başına, 90 sn)

| motor | maç | skorlar | karar: tut / sür / pas / önüne / kenardan / şut | tutma sayısı | ort. tutma (sn) | pas | kesilen | ilerleme (br) | boy | alıcıya en yakın rakip | geçilen rakip | pastan sonra 2 sn içinde kayıp | pastan sonra 5 sn içinde şut |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 21.10 (5–1) | 6 | 0-0 1-0 1-0 0-0 1-0 3-1 | 46 / 20 / 6 / 9 / 11 / 5 | 18.8 | 0.64 | 14.5 | 14% | 23.8 | 34.7 | 6.5 | 1.53 | 28% | 51% |
| ince yapı 1 | 6 | 1-0 0-0 2-0 1-1 0-0 0-0 | 52 / 28 / 3 / 5 / 5 / 3 | 17.7 | 0.82 | 15.0 | 20% | 18.9 | 33.1 | 7.2 | 1.13 | 25% | 42% |
| + görme | 6 | 2-0 0-0 2-0 1-0 1-1 0-0 | 36 / 39 / 4 / 4 / 6 / 7 | 15.8 | 0.93 | 11.2 | 22% | 25.5 | 36.1 | 7.6 | 1.90 | 31% | 58% |
| + şut değeri | 6 | 2-0 1-0 0-0 0-0 0-0 0-0 | 38 / 30 / 5 / 4 / 7 / 11 | 19.3 | 0.72 | 13.3 | 15% | 22.3 | 35.5 | 7.1 | 1.23 | 22% | 56% |
| + sürme-ikili (bugün) | 6 | 0-0 0-0 0-0 0-0 1-0 0-1 | 26 / 55 / 3 / 3 / 4 / 6 | 13.8 | 1.13 | 8.2 | 14% | 28.5 | 38.4 | 6.0 | 1.83 | 24% | 43% |

### Okuma
1. **Kafanın şut tahmini her motorda 3–7 kat şişkin.** Şut anında kafanın verdiği gol olasılıklarının toplamı: 24,5 beklenen → 7 gol (21.10), bugün 15,2 → 2. "Şutun değeri" adımı (6e976f3) düzeltmemiş, en kötü oran ondan sonra (21,1 → 3). Yani şut daha iyi tartılmıyor; çok iyimser tartılıyor. Bütün seçeneklerin değeri dolaylı olarak şut değerinden geldiği için (değer tablosu, kafadaki şut ihtimali) şişkinlik her yere yayılıyor; sıralama bozuluyor.
2. **Sürme kararları patlamış:** Liman'ın kararlarının %12'si sürme iken bugün %41; Kale Duvarı'nda %20 → %55. Ortalama tutma 0,54 → 0,95 sn; 3 sn'den uzun tutma %4 → %9.
3. **Uzun tutma pozisyonu öldürüyor:** bugün 0,5 sn'den kısa tutmaların %31'i 5 sn içinde şuta varıyor, 0,5–1,5 sn tutmaların %16'sı, 3 sn'den uzunların %12'si. Kafa tutmanın ve sürmenin değerine "zaman geçiyor, savunma yerleşiyor" bedelini katmıyor gibi. (Savunmanın yerleşmesi ölçüsü burada zayıf, sayılar küçük.)
4. **Pasın kalitesi düşmüş, isabeti değil:** Liman'ın pası 35,8 → 26,5; pas başına geçilen rakip 0,45 → **0,19**; ilerleme 10,2 → 8,8 br; pastan sonra 5 sn içinde şut %50 → **%27**. Paslar daha güvenli (alıcıya en yakın rakip 9,5 → 11,9 br) ama ileri gitmiyor: yana, kısa, boşa.
5. **Bırakılan şut:** 21.10'da yüksek olasılıklı şut seçeneklerinin %79'u "tut" için bırakılıyordu (eski bir sorun); sonra %10'a indi, bugün yine %37.

### Sonuç
Kullanıcının sezgisi doğru: oyuncular topu tutarak ve sürerek pozisyona girme şanslarını düşürüyor, paslar ileri gitmiyor. Kökte bir yanlış hesap var: kafanın şut değeri çok şişkin (her motorda), sürme-ikili adımıyla tutma ve sürme güvenli göründüğü için seçimin ağırlığı oraya kaydı; tutmanın zaman bedeli tartıda yok.

### Motor oturumu için
- Önce **şut tahmininin ayarı**: kafanın şut olasılığı gerçekleşen golle tutmalı (bugün 7 kat). `m.st.cal` bunu zaten sayıyor; her değişiklikten sonra beklenen / gol oranı 1'e yakın olmalı.
- **Tutmanın ve sürmenin zaman bedeli**: beklerken savunmanın yerleşmesi ve açık pas yollarının kapanması değerde görünmeli (kafa zaten ileri bakıyor; bekledikten sonraki dünyanın değeri bunu içermeli).
- Pas değeri: savunmayı geçen pas (geçilen rakip) bugün değer kaybetmiş; ilerlemenin değeri izlenmeli.
