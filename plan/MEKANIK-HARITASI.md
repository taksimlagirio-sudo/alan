# ALAN · Mekanik haritası: ne neden çalışıyor, ne neden çalışmıyor

Claude Code. Kaynak: motor kodunun karar dökümü (zip69: `decide.js`, `match.js`, `shape.js`, `core.js`) ve şimdiye kadarki bütün ölçümler (zip46–69). Amaç yama listesi değil: sorunların ortak kökünü bulmak ve bunu ölçerek doğrulamak.

---

## 1 · Oyunda kararlar üç farklı yolla veriliyor

| Yol | Ne demek | Kim kullanıyor |
|---|---|---|
| **A · Kafada oynatma** | Seçenek, maçın kurallarıyla (Çekirdek fiziği, rakibin tepkisi ve ivmesi) ileri oynatılıp sonucuna bakılarak tartılır | Taşıyıcı: pas, gönderme, kenardan, kendine pas (`imagine`); bekleme ve sürme (`holdWorld`, maçın kopyası) |
| **B · Donuk resim puanı** | Konumların o anki resmine bakan bir puan (tehlike tablosu, boşluk, pas yolu); hızlar ve "sonra ne olur" çoğunlukla yok | Pasın **alındıktan sonraki** değeri (`recvVal`, `respond`, `predictWorld` — bu sonuncusu hızları sıfırlıyor); alıcının buluşma noktası (`alici`); topsuz hücumcunun koşusu; presin değeri (`prV`); bölge, kapatma, `kapatHat` |
| **C · Sabit kural** | Eşik ve istisna | İlk presçi her zaman basar; en çok 2 presçi; Bekçi'nin konumu (geometri) ve çıkışı; karşı pres (en yakın 3); geçiş (sayı karşılaştırması); kovalama eşikleri; ikiliye girme eşiği |

Önemli ayrıntı: kafada **tam** maç oynatma (`lookAhead`) motorda var ama **kapalı**. Açık olan tek tam kopya bekleme ve sürmenin kopyası.

---

## 2 · Ölçümlerle eşleştirme

### Çalışanlar (hepsi A yolunda)
| Karar | Kanıt |
|---|---|
| Pas, orta ihtimalli (P < 0,95) | Aklın tahmini = gerçek (%45 / %45, %89 / %90) |
| Kenardan pas | %94 tahmin, %95 gerçek |
| Sürmenin riski | Doğrudan "sür" seçilince tahmin %1,5 kayıp, gerçek %1,5 |
| Bekleme (ortalama) | Beklenen %22, gerçek %16–20 |
| Gönderme | Tablo ve maç uyumlu (dar açı, direk, Bekçi düzeltmelerinden sonra) |

**Ortak nokta:** Çekirdeğin yolu ve rakibin ona tepkisi maçın kendi kuralıyla, ivmesiyle hesaplanıyor; sonuç kısa vadede (birkaç saniye) belli oluyor.

### Çalışmayanlar
| Sorun | Hangi yol | Neyi görmüyor |
|---|---|---|
| Kontrada sürmemek, koşarak alıp durmak (fake / tut, sürmeyle 0,01 farkla) | B: alıştan sonraki değer donuk resim (`predictWorld` hızları sıfırlıyor) | Taşıyıcının **koşu hızının** değerini; rakibin yerleşme süresini |
| Alıcının topa gelmemesi, pasın yolda kesilmesi ("kesin" pasların %60'ı) | B: `alici` konum + tepki süresi; rakip hızı yok | Rakibin **koşmakta olduğu yönü**; kafadaki rakip (sürüklenir) ile maçtaki rakip (görev kurallarıyla koşar) farklı |
| Pres: geçilen presçi hattından 25 birim önde, arkasına pastan %49 gönderme | B + C: `prV` statik maliyet, ilk presçi her zaman basar | **Geçilince ne olur** (arkada kalan boşluk); taşıyıcının bir sonraki pası |
| Yanlış adama koşma, çift adam, boş adam | B + C: görevler ayrı ayrı seçiliyor, sonra düzeltme | Takım arkadaşının **bir sonraki anda** nereye gideceği |
| Bekçi'nin çıkışı ve yakın direk | C: geometri ve eşikler | Çıkınca geride ne kalır; şutun gelme anı |
| Pastan sonra geri koşan pasçı | B: koşu adayı puanı | Pasın sonucu ve ileride açılacak boşluk |

**Ortak nokta:** Bu kararlar ya bir **donuk resme** bakıyor (hızlar ve zaman yok) ya da **sabit bir kurala**. Hiçbiri "bunu yaparsam birkaç saniye sonra ne olur" sorusunu sormuyor.

---

## 3 · Hipotez: üç kök

**K1 · Donuk dünya.** Bir hamlenin değeri, hamleden sonraki anın **hareketsiz** bir resmiyle ölçülüyor. Koşan oyuncu ile duran oyuncu, hızlanan savunmacı ile yerleşmiş savunmacı bu resimde aynı. Sonuç: hız, ivme, tempo değersiz → koşarak alan durur, kontra yavaşlar, fake ve tut sürmeyle eşit görünür.

**K2 · İki ayrı dünya.** Akıl kafasında rakipleri bir şekilde oynatıyor (kovalayan `chasePlan`, diğerleri hedefine sürüklenir, iki savunmacı kale tarafına kayar), maçta ise rakipler görev kurallarıyla (pres, bölge, kapatma) oynuyor. İkisi farklı olduğu yerde tahmin şaşıyor: "kesin" pasların %6'sı gidiyor, çoğu yolda kesiliyor.

**K3 · Sonuca bakmayan topsuz kararlar.** Pres, markaj, bölge, Bekçi, alıcı, koşu adayı: sonuçları hiç oynatılmıyor; puan ya da kural. Her sorun yeni kural istiyor, kurallar birbirine çarpıyor (Bekçi kovalaması iki kez geri geldi; atama sayıyı artırdı).

Üçü aynı şeyin yüzleri olabilir: **zaman boyutu eksik.** Taşıyıcının pas kararında zaman var (Çekirdeğin yolu, rakibin tepkisi), geri kalan her yerde yok.

---

## 4 · Doğrulama planı (yeni kural yazmadan önce)

Her hipotez ayrı ölçülecek; sonucu "doğru / yanlış / kısmen" diye yazılacak.

**Ölçüm 1 · K1, donuk dünya (kayıttan, yeni maç yok).**
Taşıyıcının kararlarında aklın değeri ile **gerçekleşen değer** (2 sn sonra top bizde mi, ne kadar ilerledi, tehlike tablosunda nereye vardı) karşılaştırılır; taşıyıcının hızına göre ayrılır.
- K1 doğruysa: hızlıyken "sür"ün gerçekleşen değeri tahmininden belirgin yüksek, "fake / tut"un gerçekleşen değeri tahmininden düşük çıkar; yavaşken fark yok.
- Yanlışsa: hızla ilişkisiz, ya da tersi.

**Ölçüm 2 · K2, iki ayrı dünya (yeniden oynatma, ~8 maç).**
Kesilen "kesin" paslarda aklın kafasında rakiplerin nerede olduğu (ikiz iz: `v5-ikiz.js` aracı var) ile gerçekte nerede oldukları, tik tik. Fark hangi rakipte (kesen mi, başkası mı) ve hangi yönde (rakip daha hızlı mı, farklı göreve mi koşuyor).
- K2 doğruysa: kesen rakip kafada daha geride / başka yerde; sebep görev kuralı.

**Ölçüm 3 · K3, topsuz kararların sonucu (kopyalı ölçüm, ~8 maç).**
Maçın belli anlarında maç kopyalanır, kopyalarda tek bir şey değiştirilir, sonuç karşılaştırılır (aynı an, aynı zar):
- pres anında: presçi bassın / oyalasın (yerinde, kale tarafında) → 3 sn sonra rakibin tehlikesi;
- Bekçi çıkış anında: çıksın / kalsın;
- alıcı: kendi noktasına koşsun / topa gelsin.
- K3 doğruysa: kuralın seçtiği, öbür seçenekten sık sık belirgin kötü; ve hangi durumlarda kötü olduğu ortaya çıkar (bu, motora girecek tek kuralı gösterir).
Bu ölçüm aynı zamanda "topsuz kararlar da kafada oynatılsın" fikrinin **işe yarayıp yaramayacağını, motoru değiştirmeden** gösterir: kopyalarda iyi seçeneği seçmek maçta ne kazandırırdı.

**Sıra ve süre:** 1 (kayıttan, kısa) → 3 (en çok şey söyleyen) → 2. Her birinin sonunda kısa bir "ne öğrendik" notu.

---

## 5 · Sonra (ölçümler doğrularsa)
- K1 için: alış sonrası değer donuk resimden değil kısa bir kopyadan (zaten var olan `holdWorld` yolu) ya da en azından hızları koruyan bir tahminden.
- K2 için: akıldaki rakipler maçtaki görev kurallarını kullansın (ayrı taklit yerine maçın kendi `shape.position`'ı; `holdWorld` bunu zaten yapıyor).
- K3 için: topa yakın 1–2 savunmacı için karar kopyada tartılsın; diğerleri bugünkü gibi. Hız bedeli ölçülerek.
- Biriken bayraklar ve istisnalar temizlenir.
