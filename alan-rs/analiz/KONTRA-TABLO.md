# Kontra değer tablosu: biçim (okuyan taraf: savunma dalı, `mac/kontra_tablo.rs`)

Hız kararında (`mac/enerji.rs sprint_karar3`, `hiz2` + `ALAN_KTAB_IZ` / sitede `ktab=1`) kontra koşusunun değeri bu tablodan
okunur. `HIZ2_BAR` ve `Sy` değişmez; tablo değeri hedefin `Sy·g` değerine eklenir.

## Dosya
- Gömülü: `alan-rs/motor/src/mac/kontra_tablo.tsv` (derlemeye girer, wasm'da da). Üretici bu dosyanın üstüne yazar.
- Yerelde denemek için derlemeden: `ALAN_KONTRA_TABLO=<yol>` (yalnız `alan-kos`, iz açıkken).

## Biçim
Sekmeyle ayrılmış metin. `#` ile başlayan satırlar ve `uzak` ile başlayan başlık satırı atlanır. Her satır bir hücre:

| sütun | anlamı |
|---|---|
| `uzak` | kaleye uzaklık (br): koşucunun hedefinin rakip kale ortasına uzaklığı |
| `savunmaci` | topun önündeki rakip saha oyuncusu (tamsayı); en büyük değer "o ve üstü" |
| `sure` | top kazanılalı geçen süre (sn) |
| `gol` | o durumdan N sn içinde gol olasılığı (0–1) |
| `n` | hücrenin örnek sayısı (yoksa 3 sayılır) |

Eksenler satırlardaki değerlerden çıkar (artan sırada, eşit aralıklı olmaları gerekmez); tablo seyrek olabilir.

## Nasıl okunur
- **uzak:** hedefin rakip kaleye uzaklığı.
- **sure:** şimdiye kadar geçen süre + varış süresi. Varış süresi en az pasın varışı; pas gelmeden varmak sayılmaz.
- **savunmaci:** koşucunun varışında topun önüne dönebilmiş savunmacı sayısı. Her rakip saha oyuncusunun hedefe varış süresinden
  yumuşak sayımla bulunur: `Σ 1/(1+exp((t_q − T)/20))`. Yani tamsayı değil.
- Üç eksende de doğrusal ara değer alınır; eksenin dışında sınır değer kullanılır.
- **Az örnekli ya da eksik hücre:** `n/(n+3)` ağırlıkla yedek değerle harmanlanır. Yedek değer, `savunmaci` için
  0: %73, 1: %50, 2: %33, 3+: %0 (analiz/ZAMAN-DEGERI.md).

Başlangıç tablosu yalnız bu yedek değerleri içerir (uzak 25, süre 0); bayrak açıkken davranış bugünkü `hiz2` ile bire bir aynı.
