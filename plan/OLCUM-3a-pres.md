# Ölçüm 3a · Pres kararının sonucu (kopyalı): sonuç

**Soru:** İlk presçinin "her zaman bas" kuralı ne zaman doğru, ne zaman yanlış? Kayıttaki ilişki (DESIGN-NOT-pres-ani: "arkasına pas %44, bundan %49 gönderme") gerçekten presten mi geliyor?

**Yöntem:** zip69 motoru, 8 maç. Bir savunmacı ilk presçi olarak basmaya başladığı anda (en çok 5 sn'de bir) maç kopyalandı; aynı an üç kolla 3 sn oynatıldı, her kol 2 zarla: **A** kural (basar) · **B** bu presçi basmaz (görevini sürdürür; başka biri basabilir) · **C** kimse basmaz (savunma yerinde). Motor değişmedi; kopyada yalnız "bu oyuncu pres adayı değil" listesi. 156 an. Araç: `tools/k3-pres.js`, özet `tools/k3-ozet.js`.

## Genel
| 3 sn içinde | A basar | B bu basmaz | C kimse basmaz |
|---|---|---|---|
| Rakip gönderme | 0,64 | 0,56 | 0,55 |
| Rakip sayı | 0,109 | 0,083 | 0,099 |
| Top kazanıldı | **%54** | %47 | %45 |
| En iyi kol olduğu an | **%54** | %26 | %19 |
Basmak topu daha sık kazandırıyor (+7–9 puan), bedeli biraz daha fazla gönderme.

## Nerede doğru, nerede yanlış
| Durum | n | A gönderme / sayı / kazanma | B | C | Kural en iyi |
|---|---|---|---|---|---|
| Taşıyıcı kendi üçte birinde | 19 | 0,05 / 0 / %29 | 0 / 0 / %24 | 0,05 / 0,03 / %18 | **%84** |
| Orta üçte bir | 76 | 0,39 / 0,05 / %41 | 0,43 / 0,06 / %31 | 0,36 / 0,05 / %31 | **%61** |
| **Bizim son üçte birimiz** | 61 | **1,14 / 0,21** / %78 | **0,90 / 0,14** / %75 | 0,94 / 0,19 / %71 | %38 |
| Presçi hattından < 15 br önde | 75 | 0,97 / 0,15 / %67 | 0,77 / 0,12 / %62 | 0,80 / 0,16 / %61 | %39 |
| Presçi hattından 15–30 br önde | 42 | 0,35 / 0,08 / %46 | 0,38 / 0,05 / %29 | 0,25 / 0,04 / %30 | **%69** |
| Presçi hattından 30+ br önde | 39 | 0,32 / 0,05 / %38 | 0,36 / 0,05 / %40 | 0,39 / 0,05 / %31 | **%69** |

## Ne öğrendik
1. **Önceki notumdaki teşhis yanlıştı.** "Orta sahada ve hattın çok önünde basmak arkada boşluk bırakıyor" kayıttan çıkan bir ilişkiydi. Kopyalı ölçüm: orta sahada ve hattın önünde **basmak doğru** (kural %61–84 en iyi; basmamak gönderme / sayıyı azaltmıyor, sadece top kazanmayı düşürüyor). Presçinin arkasına giden pas basılmasa da gidiyordu.
2. **Asıl zarar kendi kalemizin önünde.** Taşıyıcı bizim son üçte birimizdeyken basmak: 3 sn'de rakip göndermesi 1,14'e karşı 0,90; sayı **0,21'e karşı 0,14** (her 3 sn'de yarım sayının üçte biri kadar fark). Top kazanma neredeyse aynı (%78 / %75). Yani kalemizin önünde presçi topu daha sık kazanmıyor, ama yerini bırakınca şut alanı açılıyor.
3. Kullanıcının "her zaman basmak mantıklı değil" gözlemi doğru; yalnız nerede yanlış olduğu sanıldığı yer değil: **ceza alanı önü**.
4. Bu, K3'ü (sonuca bakmayan topsuz kararlar) destekliyor: aynı "bas" kararı bir bölgede doğru, başka bölgede zararlı; sabit kural ve donuk puan bunu ayırt etmiyor (`prV`'nin maliyeti presçinin evinin yakınındaki rakip, kalenin önündeki şut alanı değil).
5. Yöntem dersi: kayıttan ilişki, kural önermek için yetmiyor; kopyalı ölçüm şart.

## Bundan çıkan
- Kural yazmak yerine (ör. "son üçte birde basma") karar kopyada tartılırsa doğru ayrım kendiliğinden çıkar: A / B / C kollarının farkı tam olarak kopyanın göreceği şey. Bu ölçümde en iyi kolu seçmek, maçta ne kazandırırdı? Kabaca: son üçte birde en iyi kol seçilseydi 3 sn'de sayı ~0,21 → ~0,12.
- Hız: bu ölçüm her pres anında 3 kol × 180 tik oynattı. Motorda tek kol, 60–90 tik, yalnız topa yakın bölgede ve "bas / basma" arasında yeterli olabilir; bedeli ölçülmeli.
