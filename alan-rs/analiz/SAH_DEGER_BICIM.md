# Sahiplik değeri tablosu: dosya biçimi (bicim 1)

Okuyan taraf: `motor/src/kafa/kafa2_tarti.rs` (`sah_deger_oku`), derlemeye gömülü `motor/src/kafa/kafa2_sah_deger.json`.
Bayraklar: `ALAN_SAH_DEGER2_IZ` (EK `sahdeger2`) kayıp bedelinin tabanına girer; `ALAN_SAH_DEGER_SUT_IZ`
(EK `sahdegersut`) taban yalnız şutun bedelinde. Biçim tutmazsa okuyucu bir kez uyarır, taban kapanır.

```json
{
  "bicim": 1,
  "motor": "<üreticinin yazdığı motor izi>",
  "N_sn": 10,
  "mac": 109,
  "eksen": [
    {"ad": "uzaklik", "birim": "br", "dugum": [0, 6, 12, ...]},
    {"ad": "onde", "dugum": [0, 1, 2, 3, 4, 5, 6]},
    {"ad": "sure", "birim": "tik", "dugum": [...]}
  ],
  "deger": [...],
  "ornek": [...]
}
```

- `deger`: düz dizi, ilk eksen en dışta (row-major); boyu eksen düğüm sayılarının çarpımı. `ornek` bilgi için (okunmaz).
- `uzaklik`: taşıyıcının rakip kale merkezine (W ya da 0, CY) öklid uzaklığı.
- `onde`: rakip saha oyuncularından (Bekçi hariç, dokunamayanlar hariç) kale çizgisine toptan yakın olanların sayısı (`|gx − q.x| < |gx − x|`).
- `sure` isteğe bağlı; kafa sahipliğin süresini bilmediğinden 0 alınır (eksen varsa ilk düğüme denk düşer).
- Okuma: eksenler arasında çoklu doğrusal ara değer; düğümlerin dışında uç değer. Eksen adları yalnız bu üçü.
- Örnek üretici (sınama için): `sina/sahdeger_ornek.py <N_sn> <analiz.jsonl>...`.
