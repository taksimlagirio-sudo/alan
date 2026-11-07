# Dal oynatma: S7i (bit 256) ve S7j (bit 512) — 217468, "hepsi açık" maçın (15fc7dd-8cd3796-68a35c2, S7h dahil) 6 golü
Her gol için golden 200 tik önce dal: once (değişmeden; asıl maçla birebir aynı, doğrulandı) / S7i / S7j / S7i+S7j, 400 tik.
`alan-kos <kayıt> <T> dal <klasör> 400 256,512,768`, `python3 analiz/py/dal.py <klasör>`.

| Gol | S7i | S7j | İkisi | Not |
|---|---|---|---|---|
| 0:57 Mancınık | gol, 3.5 sn ERKEN (Hendek 14 br) | gol erken (Ok 14 br) | gol erken (Hendek) | S7i Deniz'i 19 br'den "öne çık"a çağırdı, Mancınık bölgesini bıraktı, varamadı |
| 1:38 Mancınık | **şut yok, top kazanıldı** | gol erken (Ok 17 br) | gol erken (Hendek) | S7i: Cansu öne çıkınca Mert Mancınık'ı 2 br'den kale tarafında aldı, top Liman'a geçti. S7j: Ok kutu yanında 5 sn boş (markajcısı Kaan 29 br geride) |
| 1:55 Ok | değişmedi | değişmedi | değişmedi | Kontrada tek savunmacı: Ok çizgi boyu 22 br sürdü, Deniz "pres · şut hattı" 7–12 br yanında, mesafeyi kapatmadı; diğerleri 30–40 br geride, öne çıkacak kimse yok |
| 2:43 Mancınık | gol erken (Hendek 16 br) | gol erken (Hendek 14 br) | gol (Hendek 16 br) | golcü değişti, gol kalmadı |
| 3:55 Ok | değişmedi | değişmedi | değişmedi | S7i ancak Ok 16 br'deyken (7052) devreye girdi, Cansu 12 br uzakta. Asıl hata önce: 6932'de Mert Ok'u 3 br'den ama kalenin ters tarafından markajlıyor, top boşta kalınca topa doğru gidiyor |
| 4:28 Ok | **KAL şutu yok** (top erken kazanıldı) | Mancınık 17 br, gol yok | gol erken (Mancınık 13 br) | |

Sonuç: S7i tek başına 2 golü önledi (1:38, 4:28), 2 golü erkene aldı; S7j tek başına 1 önledi, 3 erkene aldı; ikisi birlikte hiçbirini önlemedi, 3'ünü erkene aldı.
Savunmaya: (1) S7i öne çıkacak oyuncuyu uzaktan (14–19 br) seçiyor, varamıyor ve kendi adamını/bölgesini bırakıyor: yalnız şut hattındaki noktaya taşıyıcıdan önce varabilecek ve tehlike bölgesinde adam bırakmayacak oyuncu çıksın. (2) Kontrada tek savunmacı mesafeyi kapatmıyor (1:55, 3:55): 7–12 br'den yanında koşuyor. (3) Markajcı kale tarafında durma kuralı yalnız tehlike bölgesinde (S7j); 3:55 golü tehlike bölgesinin dışında, kontranın başında ters taraftan markajla başlıyor. (4) S7j'de boş kalan adamı (Ok, 2944) kimse almıyor.

## Neden işe yaramadı (kök neden)
**yuru, savunmadaki her oyuncuyu yürütüyor.** S7h/S7i/S7j hedef koyup `sprint = true` yazıyor, ama aynı tik `mac/enerji.rs sprint_karar3` hızı
yeniden seçiyor: değer = `Sy·wf(T, tD)` − enerji bedeli; `tD` = rakibin hedefe varışı. Hedef taşıyıcının önünde (onunla birlikte ilerleyen
nokta) olunca taşıyıcı hep önce varır (tD≈10 tik, savunmacı T≈35+ tik → wf≈0); yetişme değeri sıfır, yuru açıkken yürümenin bar kazancı
(c<0) kazanır → f=−1 (tam yürüyüş). `yuru_a = yuru() && !bizde_sahip(m, pi)`: topu olmayan takımın HERKESİ, presçi/markajcı dahil.
- İz (ALAN_K3_IZ, 1:55 golü, Deniz "pres · şut hattı"): 3370 `tD=10.3 f=-1`, 3390 `f=-0.85`. Deniz 0.11–0.13 br/tik yürüdü, Ok 0.30 koştu, Deniz'in barı 0.88→1.00 doldu.
- Tehlike bölgesinde (taşıyıcı kaleye <32 br), hedefine ≥2 br uzak savunmacılar, 6 golün dal kollarında "yürüyen (<0.18 br/tik)" oranı:

| iş | yuru açık, S7i/j yok | yuru açık, S7i+j | yuru kapalı, S7i/j yok | yuru kapalı, S7i+j |
|---|---|---|---|---|
| pres · öne çık (S7i) | – | %61 | – | %0 |
| pres · şut hattı | %27 | %38 | %0 | %0 |
| markaj | %31 | %54 | %6 | %31 |
| geri koşu | %41 | %41 | %1 | %3 |

Öneri: yuru yalnız savunma işi olmayanlara (önde bekle, boşluk, genişlik…; topa uzak) uygulansın; pres/öne çık/şut hattı/markaj/geri koşu/kovala/kapatma işlerinde yürüme seçeneği açılmasın.
Ek: (a) S7i aday seçiminde uzaklık/varış sınırı yok (yalnız "kaleye taşıyıcıdan yakın" süzgeci, en yakın nokta) → 19 br'den Deniz seçildi, Mancınık bölgesini bıraktı. (b) S7j'de yuru kapalıyken de markajcı %31 yürüyor (S7j'siz %6): hedef adamla birlikte kayan nokta olduğundan yine "varılamaz" sayılıyor olabilir.
