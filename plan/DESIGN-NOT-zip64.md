# ALAN · zip64 (buluşma pası): etkisi görünmüyor, hesap %41 arttı

Claude Code'dan Design'a. 16 maç, taban zip63 (zip63 motoru zip62 ile aynı).

## 1 · Beklenenler tutmadı
| Satır | Beklenen | zip63 → zip64 |
|---|---|---|
| Önüne pas: hedefin ±3 içinde alınan | ↑ | %18,7 → %18,8 |
| Önüne pas: alış–hedef uzaklık | ↓ | 8,12 → 8,27 |
| Koşarak alıp koşmaya devam eden | | %54 → %53 |
| Koşarak alıp ilk 20 tikte duran | | %29 → %32 |
| Pas tutma | kötüleşmesin | %91,5 → %91,0 |
| **Hesap ms/tik** | kötüleşmesin | **25,1 → 35,5 (+%41)** ⚠ |
| Sayı / maç | | 3,06 → 2,50 |
| Gönderme başına sayı | | %14,3 → %10,6 |

Önüne pasın alınışında hiçbir değişim yok; hesap ise %41 pahalandı. Maçın akışı da değişmiş (sayı −%18, Bekçi satırları oynadı: "sayılarda Bekçi değmiş" %41 → %23, yakın direkten sayı 0,34 → 0,53).

**Bilinmeyen:** Buluşma seçeneği maçta ne sıklıkla seçiliyor? Kayıtta ayırt edilemiyor: pas türü yine "önüne", seçeneğin `bulus: 1` işareti karar ve pas kaydına geçmiyor. Kısa sınamanızda 2 kez seçilmişti. Seçilmiyorsa etki olmaması beklenir, ama her kararda her arkadaş için yol ve aday hesabı (ve akılda tartılması) maliyeti getiriyor.

**İstek:** `lastDec.opts` ve `m.fl` (pas kaydı) buluşma işaretini taşısın (ör. `kind: 'önüne'` + `bulus: 1`, ya da `kind: 'buluşma'`). Ondan sonra: buluşma seçilme oranı, seçildiğinde hedefin ±3 içinde alınma, alıcının koşmaya devam etmesi ayrı ölçülür.

**Öneri:** Seçilme oranı düşükse (ör. < %5) sebebi tartımda: buluşma noktası akılda düz önüne pastan değersiz görünüyor olabilir (alıcı koşarken alınca hızını kaybetmiyor; bu değer akılda var mı?). Hesap için: buluşma adayı yalnız koşan (hız > 0,15) ve önü açık arkadaş için üretilsin.

## 2 · Pas kalibrasyonu (aklın P'si → gerçekte bizden biri aldı)
| Pas türü | zip63 akıl / gerçek | zip64 akıl / gerçek |
|---|---|---|
| Önüne | %96,3 / %91,3 | %95,7 / %90,9 |
| Ayağa (pas) | %98,1 / %95,2 | %97,1 / %95,8 |
| Kenardan | %91,6 / %85,2 | %92,0 / %84,4 |
| Kendine | %99,4 / %88,6 | %99,4 / %86,4 |

| Aklın P'si | zip64 n | Akıl | Gerçek |
|---|---|---|---|
| < 0,8 | 122 | %45,5 | %45,1 ✓ |
| 0,8–0,95 | 49 | %89,4 | %89,8 ✓ |
| 0,95–0,99 | 126 | %97,7 | %90,5 |
| ≥ 0,99 | 1502 | %99,4 | **%93,9** |

Akıl düşük ihtimalli paslarda doğru; **"kesin" dediği paslarda (P ≥ 0,99, pasların %83'ü) %6 iyimser**. Her 17 "kesin" pastan biri gidiyor. Kendine pas en iyimser (%99 → %87). Öneri: "kesin" sınıfındaki kayıpların nereden geldiği (alıcının kontrolü mi, araya giren mi, sekme mi) bir sonraki izde ayrılabilir (Claude Code yapar).

## 3 · Ölçüm aracında düzeltme
Bir önceki tabloda "koşarak alınca tek dokunuş kararı %" satırı yanlıştı (karşılama kancası her alışta çalışıyor; tek dokunuş kararı değil). Satır kaldırıldı; ilk karar süresi artık gerçek karar anlarından: zip63 3,65 · zip64 3,76 tik.
