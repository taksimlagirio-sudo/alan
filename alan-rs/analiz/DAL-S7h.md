# Dal oynatma: S7h (savunma, bit 128) gol anlarında — tohum 217468, izle1 ayarları
Yöntem: `alan-kos analiz/out/dal/kayit.json <T> dal <klasör> 400 128` — maç T'ye kadar aynı oynar, sonra iki kol: S7h kapalı (özgün maçla aynı, doğrulandı) / açık.

| An | Özgün | S7h açık | Neden |
|---|---|---|---|
| m1 T=1150 (0:46 Mancınık golü) | gol | gol (7 sn sonra) | Kaan Ok'u köşede 4 sn 1–2 br'den tuttu (iyi); Ok kenardan çıktı, yeniden kurulan atakta Cansu Mancınık'ın 1–3 br yanında ama kale hattında değil → 12 br'den boş hat gol |
| m2 T=2050 (1:16 Hendek golü) | gol | gol yok | Akış ayrılması (ilk fark 2079, Deniz 1 br). Yeni akışta Cansu "pres · şut hattı" Mancınık'ı 5 sn kale önünde tuttu, tek şut Ok 20 br |
| m3 T=3010 (1:48 Ok golü) | gol | şut yok | **Nedensel**: Deniz kale tarafından ~1 br yakın çıktı, 3146'da müdahalesi topu aldı (özgünde "uzanıyor → geçildi", Ok 15 br'den gol) |
| m4 T=5900 (3:24 Ok golü) | gol | gol yok | **S7h'den değil**: Deniz "pres · şut hattı" Ok'un 11 br *arkasında* (x 23'e 12) kaldı, hiç varamadı; Ok 17 br'den baskısız şut attı, kaçtı. Özgünde 12 br'ye sürüp atmıştı |

Kalan sorun (savunmaya): "pres · şut hattı" işindeki oyuncu taşıyıcının arkasındaysa ya da taşıyıcı kaleye hızlı sürüyorsa (S7h koşulu `v·u < 0.6·vp` tutmuyor) şut hattına hiç varamıyor ve 10+ br uzakta kalıyor; taşıyıcı ceza alanına kadar baskısız sürüyor (m4 özgün: Ok (29,16)→(6,19), 16 br, Deniz hep 10–15 br). Bu durumda o oyuncu yerine kale tarafında olan en yakın oyuncu çıkmalı.
