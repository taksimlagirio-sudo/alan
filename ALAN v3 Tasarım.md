# ALAN v3 · Nereye varmak istiyoruz

## Oyun
Futbolun "boşluk oyunu" özünü taklit etmeden yakalayan bir menajerlik simülasyonu. 7v7, alan kontrolü, şarj toplayan Çekirdek, iki uçta Kuyu. Menajer sadece izler; etkisi taktik, kadro ve maç içi talimattır.

## Vazgeçilmez ilkeler
1. **Kararlar yereldir.** Bir anı o noktadaki kişi sayısı, yönler ve hızlar belirler; takımların toplam gücü değil.
2. **Özellik = bir hamlenin başarı ihtimali. Taktik = o hamlenin ne sıklıkla denendiği.**
3. **Zaaflar elle yazılmaz, fizikten doğar.** Yüksek bloğun arkası açılır, fazla presin bıraktığı alan vurulur, geniş dizilişin ortası açılır.
4. **Oyuncular kusurlu okur.** Herkes her an okumaya çalışır; ne kadar doğru okuduğunu Okuma belirler. Bariz durumlarda herkes emin olur.
5. **Hiçbir tarz bir tarza hep kaybetmez.** Avantajlı taraf 10 maçın ~6'sını alır, 8'den fazlasını değil. Zayıf takım doğru taktik + doğru oyuncuyla güçlüyü zorlayabilir.
6. **Pres bedavaya değil:** kondisyon değil, alan öder. Fazla basan takımın arkası açılır; hücum bunu görmelidir.
7. **Çekirdeği tutmak gerçek bir seçimdir:** sabırlı oynayan güç toplar ve rakibi üstüne çeker; hızlı oynayan Çekirdeği az şarjla taşır.
8. **Savunma akıllıdır:** Kuyu tarafına yerleşir, geri koşar, arkada adam bırakmak doğal olarak değerlidir.
9. **Fizik tek kaynaktır:** araya girme, sekme, kenardan sektirme, Bekçi kurtarışı, boşta Çekirdek ayrı kurallar değil, aynı fiziğin sonuçlarıdır.

## Çekirdek fiziği (katman 1, kullanıcı kararları)
- Pas veren bir başlangıç hızı seçer; Çekirdek sürtünmeyle yavaşlar ve kendiliğinden durur. Hız seçiminin doğruluğu Aktarım'a bağlıdır.
- Rakip alanı yavaşlatır ve enerji tüketir; şarj bu direnci azaltır. Kendi alanı akıntı gibi sürtünmeyi azaltır.
- Enerjisi biten Çekirdek **yumuşak** bir şekilde sönüp durur, boşta kalır; ilk kontrol eden alır.
- Bütün kenarlar sektirir ve enerji kaybettirir (bilardo).
- Karşılama: Çekirdeğin hızı ve şarjı + oyuncunun Tutuş'u ve baskı. Kaçarsa seker.
- Gönderme de aynı fizikle gider: güçlü ve plase türleri, isabet Aktarım'a bağlı.

## Katmanlar ve kabul ölçütleri
| # | Katman | Kabul |
|---|---|---|
| 1 | Çekirdek fiziği, oyuncusuz deneme sahası | Kullanıcı gözle onaylar: yavaşlama doğal, pas kısa kalmıyor, sönme yumuşak, kenar sekmesi inandırıcı |
| 2 | Karşılama (durağan oyuncular) | Tutma/kaçırma/sekme oranları Tutuş ve hıza göre makul; Bekçi kurtarışı aynı kuralla |
| 3 | Taşıyıcı kararı (pas / sür / gönder), kafada simülasyon | Tahmin ile gerçek tutma oranı birbirine yakın; iyi pas %85+, baskı altında riskli pas %50- |
| 4 | Yerleşim (hücum: boşluk, hat, denge · savunma: bölge, Kuyu tarafı, pres) | Çekirdeğe göre sürü yok; presin bıraktığı alan görülür; arkadaşın hareketi öngörülür |
| 5 | Taktik + özellikler, laboratuvar | Hücre başına ≥30 maç; ilke 5 ve 6 sayıyla doğrulanır; maç başına 3–6 sayı |

## Birimler
Saha 100×50 birim, 60 tik = 1 sn. Kuyu yarıçapı 5, ağız yarı genişliği 2,2.
