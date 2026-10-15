# ALAN · Bekçi teması: sayılar nerede kaçıyor

Claude Code'dan Design'a. Design'ın düzeltmesi doğru: A2 önerisinde tepkiyi Okuma'dan hesaplamıştım, motor Refleks'ten okuyor; Refleks 20'de tepki 0 tik. O öneri geçersiz.

**Yöntem:** zip60, senaryo 0–7 (8 maç, 177 gönderme, 32 sayı), `contact` içine yalnız okuyan bir kanca: gerçek maçın Çekirdeğine (kafadaki kopyalar değil) rakip Bekçi her değdiğinde göreli hız, kapasite (cap, gürültülü capE), tutma ihtimali, hazırlık (`haz`), sonuç ve sekme sonrası yön. Yeniden oynatılan 8 maçın skoru kayıtlarla birebir aynı (kanca akışı değiştirmiyor). Araç: `tools/bekci-temas.js`, özet `tools/bekci-temas-ozet.js`.

## 1 · Sayıların %59'unda Bekçi topa hiç değmiyor
32 sayının 19'u Bekçi'ye değmeden giriyor; 13'ünde değiyor.

**Değmeden girenlerde Bekçi nerede (gönderme anında, Kuyu ortasına uzaklık):** 2,0 · 2,2 · 2,4 · 3,0 · 3,3 · 5,0 · 5,0 · 6,5 · 6,6 · 6,7 · 7,1 · 8,8 · 9,4 · 10,5 · 11,1 · 12,5 · 13,0 · 13,1 · 18,5 (ortanca 6,7). Yarısında Bekçi kalesinden 7 birimden uzakta; çoğunda **hedefinde** (hedefine 0,5–1,5): yani kuralın koyduğu yerde, öne çıkmış.

**Çelişki (kodda):**
- `shape.js bekci`: öne çıkış `rOut` Bekçi'yi Kuyu'dan **KR + 4 = 13** birime kadar çıkarıyor (taşıyıcı yaklaşınca, karşı karşıya).
- `core.js contact`: Bekçi uzanması (`bekR`, 1,74 → 2,90) yalnız Kuyu'ya **12 birimden yakınken** (`ALAN_BEK_ALAN = 12`); dışında saha oyuncusu uzanması (gövde 0,9, tepki `reachReact` 6 tik, büyüme 0,12 / tik).
- Yani öne çıkan Bekçi, tam şutun geleceği anda ellerini kaybediyor. 12–13 ve ötesinde (4 sayı) bu doğrudan; 7–12 arasında ise Bekçi şutçunun dibinde, top 1–2 tikte yanından geçiyor (uzanma 1,74–2,16'da).

**Öneri (tek yer):** Öne çıkış Bekçi'yi kendi uzanma bölgesinin dışına koymasın: `rOut` üst sınırı `ALAN_BEK_ALAN − 1` (ya da `ALAN_BEK_ALAN` öne çıkış sınırına eşitlensin; ikisi aynı sabitten okunsun). Ayrıca karşı karşıyada şutçuya ne kadar yaklaşılacağı (bugün "taşıyıcının 3 birim önü") gözden geçirilmeli: şutçunun dibindeki Bekçi'nin yanından top 1–2 tikte geçiyor, uzanma büyüyemiyor.

## 2 · Değdiği sayılarda: sekme yönü baskın
| Sonuç | Temas | Ardından 1 sn'de sayı |
|---|---|---|
| Tuttu | 27 (%16) | 0 |
| Sekti | 122 (%73) | 15 |
| Deldi (`thru`) | 19 (%11) | 5 |

Sekenlerin yönü: **dışarı 101**, Kuyu yönüne ama ağız dışına 9, **Kuyu'ya doğru (çizgiyi ağızda kesen) 12**. Kuyu'ya doğru sekenlerin **10'u sayı** (12'de 10). Dışarı sekip sonradan sayı olan 5.

**Kapasite:** göreli hız 2,6 üstü 88 temasın hiçbiri tutulmuyor, 2–2,6 arası %2; 2'nin altı %79. Kapasite ort. 1,4–1,5. Güçlü şut tutulmuyor ama çoğu (%83) dışarı çeliniyor; bu kendi başına sorun değil (futbolda da güçlü şut çelinir).

**Hazırlık:** sayıya giden temaslarda `haz` 0,42, Çekirdek yola çıkalı 1,65 tik (yakından şut); tutulanlarda `haz` 0,94, 11 tik.
| Hazırlık | Temas | Tuttu | Ardından sayı |
|---|---|---|---|
| 0–0,5 | 25 | %4 | %32 |
| 0,5–0,99 | 27 | %4 | %26 |
| 1 | 116 | %22 | %4 |

**Sebep (kodda):** Çelme payı `st = q.steer × bekSteer × min(1, capE / vr) × (…Hakimiyet)`. Güçlü ve yakın şutta `capE / vr` küçük (hazırlıksız Bekçi'de cap ~1,0, vr ~2,9 → 0,34): Bekçi en çok çelmesi gerektiği anda en az çeliyor; kalan hız gelişi yönünde ve Kuyu'ya doğru gidiyor.

**Öneri (tek yer):** Bekçi'nin çelme payı kapasite oranından bağımsız olsun, yalnız Alan hakimiyeti'ne bağlı (`st = steer × bekSteer × (…Hakimiyet)`). Kapasite "tutar mı" sorusunu belirlesin; "nereye çeler" sorusunu Hakimiyet. Hazırlık da tutmayı etkilemeye devam eder (yakın şut yine zor), ama tutamadığı topu Kuyu'ya itmez.

## 3 · Beklenen (kontrol tablosu)
- Sayılarda Bekçi'nin değdiği pay ↑; değmeden giren ↓.
- Kuyu'ya doğru seken ↓ (bugün temasların %7'si, bunların %83'ü sayı).
- Gönderme başına sayı ↓, sayı / dk ↓.
- Gönderme tablosu yeniden üretilmeli (`bekR` değişmiyor ama çelme yönü ve öne çıkış akıl tarafından da görülüyor mu, kontrol edilmeli).
- Bekçi'yi değiştirmeden ölçmek için iki bayrak ayrı olsun (öne çıkış sınırı / çelme).

Not: Bekçi özelliği farklı iki takımla ölçüm yapılacaksa Refleks ve Alan hakimiyeti değiştirilecek (Okuma değil).
