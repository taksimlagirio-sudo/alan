# ALAN · zip40 ölçümü: özellikler ve taktik kabulü

Yöntem: birleşik kayıt (maç bir kez oynatılır, her şey kayıttan ölçülür).
- Özellik: iki takımda bütün özellikler 10; Liman'ın saha oyuncularında (Bekçi özelliğinde Bekçi'sinde) tek özellik 6 ve 14. 18 özellik × 2 × 6 maç + 6 taban = 222 maç.
- Taktik: Liman'ın 6 maçında geçiş kuralı, karşı pres kuralı ve duruş ayarı; varsayılanla karşılaştırma (30 + 6 maç).
- Her hücre 6 maç: gol sayıları gürültülü, mekaniğe bağlı göstergeler (koşu hızı, baskı altında pas) daha güvenilir.

## 1 · Genel (zip39 → zip40, 16 maç, ayarsız takımlar)
Oyun bozulmadı: pas (96 → 93), pas tutma (%90,9 → %91,4), durma, salınım, aklın isabeti aynı. Gol 4,1 → 3,6 (gönderme aynı, 16,4 → 16,6): muhtemelen "herkes 20"de Soğukkanlılık'ın 10'da kalması (baskıda gönderme daha çok dağılıyor) ve Cesaret çarpanının kalkması.

## 2 · Özellikler (6 / 10 / 14)

**Çalışıyor:**
| Özellik | Kanıt |
|---|---|
| Hız | koşu hızı 0,26 / 0,29 / 0,32; yenen gol 3,8 / 3,3 / 1,5 |
| Kontrol | baskı altında pas tutma %66 / %82 / %91 |
| Pas | atılan 2,3 / 2,5 / 3,7; baskı altında pas %82 / %82 / %91 |
| Yükleme | gol farkı −1,2 / −0,8 / +0,5; pas tutma %88 / %90 / %93 |
| Alan gücü | yenen 3,8 / 3,3 / 2,5; top kapma 4,8 / 5,7 / 6,8 |
| Soğukkanlılık | 14'te gol farkı +2,0 (atılan 4,3); 6'da belirgin değil |
| Alan hakimiyeti (B) | yenen 3,7 / 3,3 / 2,8 |

**Sorunlu:**
1. **Güç: etkisiz.** Yalnız omuz ikilisini besliyor ve omuz ikilisi maç başına 1'den az (6 maçta 4–5). Güç 14'te maçlar 10'dakilerle birebir aynı. Planda Güç'ün işi: omuz ikilisi + gövdeyle koruma (`shieldK`, topu koruma) + çarpışmada ayakta kalma (`govdeAyir`'da kim kimi iter). Son ikisi bağlanmalı; yoksa Güç gerçek bir özellik değil.
2. **Uzanma: yükseldikçe Liman daha az atıyor.** Atılan 3,8 / 2,5 / 2,0; gönderme 40 → 30 (6 maçta); pas tutma %91 / %90 / %85. İki gösterge aynı yönde: gürültü sayılmamalı. Sebep henüz bilinmiyor (adaylar: uzun menzil kendi pasına/arkadaşının topuna dokunuyor; akıl rakip menzilini kendi menziliyle karıştırıyor). Claude Code kopyalı ölçümle (aynı an, Uzanma 10 ve 14) ayırabilir.
3. **Sürme: yükseldikçe Liman daha az atıyor.** Atılan 3,3 / 2,5 / 2,2; gönderme 43 → 27; kendine pas 9 → 7. İyi süren takım topu daha uzun tutuyor, daha az şut atıyor. Aday sebep: tutma/sürme değeri (`keepP`, `topK`) Sürme'yle hızlı büyüyor ve akıl sürmeyi göndermeye ve pasa tercih ediyor (değer abartılı ya da sürmenin riski küçük görülüyor). Kural 3'e ("yükseldikçe hiç kötüleşmez") aykırı.
4. **Refleks (B):** 6'da yenen artıyor (5,2 / 3,3), 14'te azalmıyor (4,0). Üst tarafta etki yok gibi; tavan ya da formül kontrol edilmeli.

**Belirsiz (6 maçta görünmüyor):** Çeviklik, Tepki, Görüş, Yer alma, Gönderme, Kesme, Yerleşim. Etkileri muhtemelen var ama küçük; ayırmak için mekaniğe bağlı göstergeler (Tepki: pasa tepki süresi; Görüş: aklın tahmin hatası; Kesme: ikili ve pas arası kazanma) ya da daha çok maç gerekir. Claude Code bir sonraki turda bunları ekleyebilir.

## 3 · Taktik kabulü

**Geçiş kuralı mekanik olarak çalışıyor ama oyunda neredeyse yok.**
| | Geçiş süresi (Liman, tik/maç) | "Git" | Ortalama koşan | Geçişte şut / maç |
|---|---|---|---|---|
| varsayılan (sayıca üstünsek) | 126 | 13 | 2,5 | 0,0 |
| hiçbir zaman | 96 | 0 | – | 0,0 |
| her zaman | 97 | 97 | 1,2 | 0,5 |

Bir maçta geçiş toplam ~2 saniye; varsayılan kuralda "git" maç başına 0,2 saniye. Kural doğru işliyor (hiç → 0, her → bütün geçiş) ama menajerin seçimi maçı neredeyse hiç değiştirmiyor. Aday sebep: "kaybeden takım dağınık" eşiği (topun gerisinde Bekçi dışında 4'ten az oyuncu) 7v7'de (6 saha oyuncusu) çok sıkı; geçiş çok seyrek başlıyor ve hemen bitiyor. Eşiği 3'e indirip ölçmeyi öneriyorum.

**Karşı pres mekanik olarak çalışıyor, etkisi görünmüyor.** "Her yerde, 3 sn": 139 tik/maç karşı pres (varsayılan 31, "hiç" 0). Ama ortalama basan 1,0 kişi ve topu 3 saniyede geri kazanma %34 (varsayılan %39, "hiç" %40). Tek kişinin basması bir karşı pres sayılmaz; 8 birim yarıçapı içinde nadiren birden çok oyuncu oluyor. Basanları seçerken mesafeyi 8 yerine "kayıp anında topa en yakın 2–3 oyuncu (makul mesafede)" yapmayı öneriyorum.

**Duruş ayarı evi değiştiriyor ama oyuncular az uyuyor.** Son bölge duruşunda iki kanadı çizgiye çekmek (y 3 / 47) takımın ortadan ortalama açıklığını son bölgede 8,1 → 8,7, ortada 8,8 → 8,5 değiştirdi. Yön doğru ama küçük (iki oyuncunun 10 birim kayması takım ortalamasında ~2–3 birim beklenirdi). Design'ın kendi tek maçlık ölçümüyle aynı: ev açılıyor, oyuncunun gerçek yeri az kayıyor (aday tartımı evi ezip geçiyor). **Menajerin tahtada çizdiği şey sahada görünmezse ekranın vaadi tutmaz.** Evin çekimi (serbestlik bağının en yakın kademesi) duruşta açıkça ellenen oyuncu için güçlenmeli; ya da aday tartımında evden uzaklığın bedeli artmalı. Ölçüt: ellenen oyuncunun kendi bölgesinde gerçek yeri evine en az yarı yarıya uysun.

## 4 · Öneri sırası
1. Duruş ayarının sahada görünmesi (menajerin en doğrudan aracı).
2. Uzanma ve Sürme'nin ters etkisi (kural 3); Claude Code kopyalı ölçümle sebebi ayırabilir.
3. Güç'ün bağlanması.
4. Geçiş eşiği ve karşı pres basan seçimi.
5. Belirsiz özellikler için mekanik göstergeler (Claude Code).
