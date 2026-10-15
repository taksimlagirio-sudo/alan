# Ölçüm 1 · Donuk dünya (K1): sonuç

**Soru:** Akıl, koşan taşıyıcının hızının değerini görmüyor mu?

**Yöntem (doğal deney, yeni maç yok):** zip65–69 kayıtları (64 maç). Taşıyıcının ilk 4 seçeneğinde "sür" ile "fake / tut"un değeri birbirine çok yakın (fark < 0,02) olan 5220 karar. Akıl bu iki hamleyi eşit görüyor, seçim fiilen rastgele. Seçilen hamleye göre gerçekte ne olduğu ölçüldü ve taşıyıcının karar anındaki hızına göre ayrıldı. Araç: `tools/k1-donuk.js`.

| Taşıyıcının hızı | Seçilen | n | 2 sn sonra top bizde | İlerleme (br) | 3 sn'de gönderme | 5 sn'de sayı |
|---|---|---|---|---|---|---|
| Yavaş (< 0,08) | sür | 383 | %73,6 | 7,1 | %43,6 | %13,8 |
| | fake / tut | 353 | %71,7 | 6,3 | %49,3 | %15,3 |
| Orta | sür | 266 | %75,9 | 12,6 | %40,6 | %15,8 |
| | fake / tut | 192 | %78,6 | 10,8 | %35,4 | %14,1 |
| **Hızlı (> 0,18)** | **sür** | 2570 | %77,2 | **15,8** | **%43,7** | **%14,3** |
| | **fake / tut** | 1456 | %77,2 | 14,0 | %33,4 | %10,4 |

Hızlı taşıyıcı, önündeki rakibe göre:
| Rakip | Seçilen | n | Top bizde | İlerleme | 3 sn'de gönderme |
|---|---|---|---|---|---|
| < 4 br | sür | 1201 | %73,0 | 14,2 | %52,4 |
| | fake / tut | 626 | %77,3 | 13,0 | %35,8 |
| 4–8 | sür | 643 | %78,8 | 16,1 | %38,6 |
| | fake / tut | 391 | %76,7 | 13,6 | %32,2 |
| 8+ | sür | 726 | %82,8 | 18,3 | %33,9 |
| | fake / tut | 439 | %77,4 | 15,7 | %31,0 |

## Ne öğrendik
- **K1 doğrulandı.** Akıl ikisini eşit görüyor, ama **hızlı** taşıyıcıda "sür" gerçekte belirgin daha iyi: 3 sn'de gönderme %44'e karşı %33, 5 sn'de sayı %14,3'e karşı %10,4 (göreli +%38). Top kaybı aynı.
- **Yavaş** taşıyıcıda fark yok (hatta fake / tut biraz iyi). Yani akıl duran taşıyıcı için doğru tartıyor; yanılgı **yalnız hız varken**. Bu, "değer hareketsiz bir resimle ölçülüyor" hipoteziyle tam uyumlu: duran oyuncu için resim zaten hareketsiz.
- Büyüklük: akıl iki hamle arasında ~0,004 fark görüyor; gerçekleşen sayı farkı ~0,04. Hızlı taşıyıcıda sürmenin değeri yaklaşık **10 kat** küçük görülüyor. Fake / tut'un %36 seçilmesinin sebebi bu.
- Önü kapalıyken (rakip < 4) bile hızlı taşıyıcının sürmesi daha çok göndermeye gidiyor (%52 / %36), top kaybı biraz artıyor (%73 / %77): hızın getirisi riskinden büyük.

## Sınır
Seçim tam rastgele değil: "fark < 0,02" içinde de akıl birini seçiyor; seçimi etkileyen başka bir şey sonucu da etkileyebilir. Ama fark hızla düzenli büyüyor ve yavaşta sıfır; başka bir açıklama bu deseni vermiyor. Kesin kanıt için kopyalı ölçüm (aynı an, iki seçenek) Ölçüm 3'le birlikte yapılabilir.

## Bundan çıkan (motor için, doğrulanmış kök)
Taşıyıcının hamle değerlerinde, hamleden sonraki dünya hızları sıfırlanmış bir resimle değil, hızları koruyan bir tahminle (ya da kısa bir kopya) ölçülmeli. Ayrı bir "koşu bonusu" yazılmamalı: bu, yeni bir istisna olurdu; sebep değeri ölçen resim.
