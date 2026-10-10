# v4g test raporu (CLAUDE-CODE-TEST.md görevleri)

## Önce: kritik hata, top sahibinin "sürüyor" işareti hiç sıfırlanmıyor

`match.js`'te taşıyıcı dalı `return` ile erken çıkıyor. `for (const p of ps) p.drive = false;` ve `if (m.tick >= m.len) m.over = true;` satırları yalnızca top boştayken çalışıyor. Bunun iki sonucu var:

1. Taşıyıcı bir kez "sür" kararı verince `h.drive = true` oluyor ve top onda kaldıkça öyle kalıyor. Bu yüzden "doğal sürüş" bloğu (`if (h === m.holder && !h.drive)`) neredeyse hiç çalışmıyor. Bu blok rakipten sıyrılma, ara noktayı geçince asıl yöne kırma ve 3 birim ileriye bakma işlerini yapıyor. Ölçüm (3 maç, refleks): taşıyıcı tiklerinin **%86–91'inde** bu blok kapalı. Taşıyıcı, kararın ilk verdiği noktaya düz koşuyor.
2. Kilitlenme: Sürme planının ara noktası `W` taşıyıcının kendi konumuna eşit çıkarsa (`tx = W = kendisi`), taşıyıcı duruyor. Etrafta rakip yoksa `surDevam` her seferinde "plan sürüyor" diyor ve karar hiç alınmıyor. Aynı tik içinde maç bitiş kontrolü de atlandığı için **maç bitmiyor** (öğrenme testinde 45.000. tike kadar gitti). Hafif hâli de her maçta görülüyor: maçlar 5400 yerine 5430–5540 tikte bitiyor.

Test için kendi kopyamda iki satırlık yama kullandım; motor dosyasına dokunmadım:
```
      h.drive = false;                                   // taşıyıcı dalının başında, karar satırından önce
      if (m.tick >= m.len) m.over = true;                // taşıyıcı dalındaki return'den önce
```
Yamayla maçlar tam 5400'de bitiyor ve kilitlenen maç sorunsuz oynanıyor. Taşıyıcı artık sıyrılıyor. Beynin ileri oynatması da `step`'i kullandığı için hata hayalde de vardı; hayal ile gerçek tutarlıydı ama ikisi de yanlıştı. Asıl düzeltme sizin kararınız: işaret mi sıfırlanmalı, yoksa sürüş bloğunun koşulu mu değişmeli?

## Görev 1: uzaktan göndermede hayal iyimser mi? (100 gönderme)
| uzaklık | n | beynin seçtiği değer | hayal (20 tekrar) | gerçek (20 tekrar) |
|---|---|---|---|---|
| ≤20 | 88 | +0,95 | +0,88 | +0,88 |
| 20–35 | 5 | +0,76 | +0,69 | +0,54 |
| 50+ | 6 | +0,23 | +0,20 | +0,08 |

- Yeniden tartma işe yarıyor. Eski v4g'de 50+ birimde değer +0,56'ya karşı +0,05'ti, şimdi +0,23'e karşı +0,08. Uzak gönderme oranı da %17'den %6'ya düştü.
- Hayal ile gerçek arasında kalan fark algıdan ya da hafif kipten gelmiyor; bunları kapatmak sonucu değiştirmiyor. Az sayıda uzak gönderme kaldığı için kalan farkın tamamı yanılgı sayılamaz.

## Görev 2: gerçek maçta gönderme ve Bekçi
- 20–35 birimden göndermelerde Bekçi, gönderme anında **%80** oranında hattın 1 birimden uzağında. ≤20 birimde bu oran %39. Bekçi ortaortayda duruyor ama gönderme hattı, direk açısının ortaortayı değil; taşıyıcı açının bir kenarına gönderiyor. Mekanik sebep bu: Bekçi açıyı kapatıyor, gönderilen çizgiyi değil.
- Yeni sürümde bu dilimde yalnızca 5 gönderme var; sonuç kesin değil.

## Görev 3: hareket (4 maç × 1800 tik, oyuncu-dakika başına)
| | eski v4g | yeni v4g | yeni + yama |
|---|---|---|---|
| hedef sıçraması | 253 | 76 | 77 |
| 45°+ dönüş | 91 | 84 | 75 |
| ivme sıçraması | 180 | 152 | 131 |
| dur-kalk | – | 32 | 32 |

- Dönüşlerin çoğu artık "hedef sabitken" (46/75). Sebep: niyet, oyuncunun koşusundan hızlı kayıyor. `niyetHiz` 0,6 birim/tik, oyuncu hızı ise yaklaşık 0,3. Hedef oyuncunun önünde yön değiştirince oyuncu dönüyor; görünürde hedef sıçramadığı hâlde dönüş oluşuyor.
- Kalan sıçramaların iki kaynağı var: refleks görev değişimi (29) ve aynı görevde hedefin zıplaması (32). En sık geçişler "bölge → pres", "bölge → kovala" ve "geçildi → pres".

## Görev 4: hız ve belirlenimcilik
Eski v4g'de maç 4 çekirdekte 43 sn, tek çekirdekte 64 sn sürüyordu; iz aynıydı, yani belirlenimciydi. Yeni v4g'de tek çekirdekte bir maç yaklaşık 70 sn (öğrenme koşusu, yeniden tartma ile).

## Görev 5: hafıza (50 maç)
Hâlâ çalışıyor (yamalı kopyada iki ayrı öğrenme). Yamasız sürümde ikinci öğrenme 2. maçta yukarıdaki hata yüzünden kilitlendi. Sonuç ayrıca gelecek.

## Görev 6: top saklama ve savunma zekâsı (6 maç; parantez içi yamalı)
- İkili fırsatlarının %60'ına giriliyor (%61). Savunmacı %45 kazanıyor (%39), beklenen P ortalaması %43 (%40). Kazanma, P ile uyumlu.
- Çekirdek gövdeyle korunuyorsa savunmacı %20 (%20) kazanıyor, korunmuyorsa %60 (%54). Gövde tarafından girişte %20, top tarafından girişte %63 (%55).
- İkinci presçi az etkili: varken %48, yokken %42 (%42 / %38).
- Baskı altında topun kaybına kadar geçen süre medyan 7 tik.
- Yerleşim (rakip topa sahipken):
  - Çekirdek ile Kuyu arasında savunmacı olan tik oranı %41 (%42).
  - En tehlikeli rakibin 4 birim içinde savunmacı olmadığı tik oranı **%71 (%74)**.
  - İki savunmacının 3 birim içinde üst üste durduğu tik oranı **%38 (%40)**.
  - Beynin yerleşim kararlarının %90'ı refleks yerinin dışında.

**En zayıf halka: savunma yerleşimi.** Mekanik sebepler:
1. **Gövde yönünden girilen ikili neredeyse hiç kazanılmıyor (%20), ama ikililerin yaklaşık %40'ı bu yönden.** Beynin ikilide iki seçeneği var: savunmacı menzildeyken "gir" ya da "girme". Savunmacının taşıyıcıya hangi taraftan, yani Çekirdeğin bulunduğu taraftan mı geleceği bu seçenekler arasında yok. Yaklaşmayı pres veya kovalama yapıyor ve bunlar taşıyıcıya doğru gidiyor. Taşıyıcı da Çekirdeği rakipten uzak tarafa çeviriyor; bu yüzden savunmacı çoğu zaman gövde tarafına varıyor. Yani zayıflık giriş kararında değil, yaklaşma yolunda.
2. **Yerleşim seçimi büyük olasılıkla gürültüye dayanıyor.** Her 10 tikte bir oyuncu için 9–11 aday yer var: refleks yeri, onun 7 birim çevresindeki 8 nokta, "yerinde dur" ve yakın savunmacı için 1–2 ek aday. Her aday 2 kez ve yalnızca 60 tik (1 sn) ileri oynatılıyor. 1 saniyede maç sonucu neredeyse hiç değişmiyor, bu yüzden adayların değerleri arasındaki fark denemenin kendi gürültüsünden küçük kalıyor. Bunun işareti şu: refleks yeri dışı seçim oranı %90. Seçim rastgele olsaydı oran yaklaşık 10/11 = %91 olurdu. Yani beyin, yer seçerken neredeyse yazı-tura atıyor ve savunmacıyı refleks yerinin 7 birim çevresinde rastgele bir yere 2 saniyeliğine (YT = 120 tik) kaydırıyor. Tehlikeli rakibin boş kalması (%71) ve kümelenme (%38) bu rastgele kaymaların sonucu. Göndermeye eklenen yeniden tartma, yerleşimde yok.

Ölçüm araçları: `tools/sut-test.js`, `sut-ozet.js`, `hareket.js`, `savunma.js`, `hs-ozet.js`, `ogren-test.js`. Veriler `lab/sut2`, `lab/hareket2` (yamasız), `lab/hareket3` (yamalı), `lab/savunma` (yamasız), `lab/savunma2` (yamalı) ve `lab/ogren` klasörlerinde.
