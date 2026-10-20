# ALAN · Enerji ölçümü 1: enerjili motorun genel hâli (20 Ekim 2026)

> Motor: main e0ea04a ("Çekirdeğin tek enerjisi"). Ölçüm sürerken f675ddb geldi ("Pas ve şut gücü Güç'ten; rakip alanının yavaşlatması azaldı"); gol sorunu onda da ayrıca denendi, aşağıda.
> Araçlar: `tools/enerji-olc.js`, `tools/enerji-ozet.js`, `tools/sut-iz.js`. Veriler: `lab/en1/`.

## Düzen

- Herkes 10. Tarzlar `tools/taktik-ayar.js`'ten. Maç 2700 tik (90 sn). Taraflar tohumla dengeli.
- Özellik kolları: A'nın saha oyuncularında tek özellik 16 ya da 4, rakip 10. 16 tohum × 5 kol = 80 maç.
- Tarz eşleşmeleri: 15 eşleşme × 12 tohum = 180 maç.
- Toplam 260 maç.

## 1 · Asıl bulgu: gol yok

260 maçta **hiç gol yok**. Maç başına 1,1 şut, 0,22 xG.

Ölçüm aracından mı, motordan mı diye ayrıca denendi. Varsayılan taktik, aynı araç, 5 dakikalık tam maç (9000 tik), 8 maç:

| motor | maç başına şut | maç başına gol |
|---|---|---|
| 6a73024 (enerjiden önce) | 9,0 | 1,25 |
| f675ddb (bugünkü main) | 4,3 | **0** |

Yani sorun motorda. Şutlar tek tek izlendi (`tools/sut-iz.js`, 90 sn, 12 maç, f675ddb):

| | şut | vuruştaki şarj | çıkış hızı | xG | gol |
|---|---|---|---|---|---|
| bugünkü (tek enerji) | 8 | **0,20** (taban) | 1,43 | 0,23 | 0 |
| `ALAN_TEK_ENERJI_ESKI` | 28 | 0,49 | 2,39 | 0,40 | 1 |
| `ALAN_ENERJI_ESKI` (8 maç) | 22 | 0,46 | 2,43 | 0,35 | 4 |
| `ALAN_SARJ_DEGER_ESKI` | 14 | 0,20 | 1,26 | 0,18 | 0 |

**Mekanizma:**
- Tek enerjide pas uçuşta şarj kaybeder. Alıcı Çekirdeği **0,05–0,13 şarjla** alıyor (`match.js` `take`: `m.ch = ball.ch`).
- Şutçu topu alıp 4–16 tikte vuruyor, arada şarj etmiyor. 66 tik tutan bir oyuncuda bile şarj yalnız 0,13'ten 0,19'a çıktı.
- Şut tabandan (`tekTaban` 0,2) çıkıyor: yavaş. Bekçi tutuyor ya da top sönüyor.
- Kafa bunu biliyor: düşük şarjla şutun değeri düşük, o yüzden şutu az seçiyor (şut sayısı yarıya iniyor).
- Şarj kararını eski kurala döndürmek (`ALAN_SARJ_DEGER_ESKI`) düzeltmiyor. Sorun karar değil, **alınan şarjın azlığı ve şarjın yavaşlığı**.

f675ddb'deki rakip alanı düzeltmesi yetmemiş. Sebep uçuştaki direnç değil, şutun başlangıç şarjı.

Motor oturumu için olası yollar (karar onların):
- Alıcının en az bir taban şarjla alması (vuruşun tabanı gibi, alışın da tabanı).
- Şarj hızının tek enerjide artması (072ab6f'deki ×3 geri alınmıştı).
- Şutun gücünün yalnız şarjdan değil, vuranın kendisinden de gelmesi (Güç ve Gönderme zaten var; tabanı 0,2 yerine bunlara bağlamak).

**Bu sorun düzelmeden aşağıdaki sonuçlar temkinle okunmalı:** şut az, xG farkları küçük sayılara dayanıyor.

## 2 · Enerjinin kendisi

Bu kısım şut sorunundan bağımsız; enerji sistemi beklendiği gibi çalışıyor.

- Ortalama bar 0,60–0,73. Barın dipte (<0,15) geçtiği zaman %4–10.
- Oyuncular zamanın %38–43'ünde sprintte (enerjiden önce ~%70).
- Harcamanın dağılımı bütün tarzlarda: sprint ~%62, ivme ~%19, alan ~%14, ikili ~%2, sürme ~%1,3, fake ~%0,6, şarj ~%0,7.
- **Şarj enerjinin %1'ini bile almıyor.** Bölüm 1'le aynı sorun: Çekirdeğin enerjisi oyuncunun enerjisinden beslenmiyor gibi.

### Tarzların enerji profili

| tarz | ort. bar | dipte | top bizde |
|---|---|---|---|
| Kale önü | 0,72 | %5 | %53 |
| Topa sahip | 0,73 | %4 | %55 |
| Kanat tuzağı | 0,65 | %7 | %58 |
| Ön alan pres | 0,63 | %8 | %50 |
| Dikine | 0,63 | %9 | %41 |
| Kontra | 0,60 | %10 | %44 |

- Topu tutan ve geride bekleyen tarzlar dinç kalıyor. Kontra, Dikine ve Pres daha çok yoruluyor. Futbola uygun yön.
- Fark küçük: 0,60 ile 0,73 arası. Tarzın enerji bedeli henüz maçı belirleyecek kadar değil.
- Harcamanın türlere dağılımı tarzdan tarza neredeyse aynı. Pres takımında ikili ve alan payı beklenen kadar ayrışmıyor (Kanat tuzağında ikili %2,6, en yüksek).

## 3 · Özellikler

| A | şut farkı | xG farkı | kapma farkı | bar A / rakip | dipte A / rakip |
|---|---|---|---|---|---|
| taban | +0,1 | −0,06 | +0,1 | 0,66 / 0,67 | %7 / %6 |
| Hız 16 | **+1,1 \*** | **+0,27 \*** | +0,1 | 0,69 / 0,63 | %6 / %9 |
| Hız 4 | −0,3 | **−0,29 \*** | −0,3 | 0,65 / 0,72 | %9 / %5 |
| Enerji 16 | −0,2 | −0,04 | **−0,3 \*** | 0,81 / 0,66 | %2 / %8 |
| Enerji 4 | −0,3 | +0,03 | −0,2 | 0,50 / 0,68 | %21 / %6 |

- **Hız hâlâ en etkili özellik**, ama artık enerjiyle bir bedeli var gibi görünmüyor. Tersine: hızlı takım daha az sprint atıyor (%36), rakibi daha çok (%44) ve rakip daha çok yoruluyor. Hızlı oyuncu aynı işi daha kısa sprintle yapıyor. Bu doğal, ama Hız'ın baskınlığını enerji dengelemiyor.
- **Enerji özelliği barı belirgin değiştiriyor** (16: bar 0,81, dipte %2; 4: bar 0,50, dipte %21), ama **maça yansımıyor**. 90 sn kısa olabilir; ama dipte %21 geçiren takımın xG'si düşmüyor. Dipteki oyuncunun cezası hissedilmiyor.
- Enerji 16'nın kapma farkı eksi çıktı (beklenmeyen). Küçük örnek; tekrar ölçülmeli.

## 4 · Tarz döngüsü

xG farkları (satır sütuna karşı, 12 maç):
- **Dikine yine hiçbir tarza kaybetmiyor** (ortalama +0,11). Önceki ölçümle aynı.
- Kanat tuzağı Ön alan presi yeniyor (+0,38).
- Pres topa sahip takımı yeniyor (+0,18), Kontra ve Dikine'ye kaybediyor (−0,24, −0,17). Pres → topa sahip, direkt → pres ilişkisi hâlâ görülüyor.
- Kale önünün xG'si birçok eşleşmede ±0,00: bu eşleşmelerde **iki taraf da hiç şut atmamış**. Şut sorunu döngüyü okumayı zorlaştırıyor.

## Sonuç

1. **Önce gol sorunu.** Tek enerjiyle alıcı boş Çekirdekle başlıyor, şut tabandan çıkıyor; şut yarıya, gol sıfıra indi. Motor oturumuna iletilmeli.
2. Enerji sistemi sayısal olarak sağlıklı: bar, dip ve sprint payı makul; tarzlar yön olarak doğru ayrışıyor ama farkı küçük.
3. Enerji özelliği barı değiştiriyor ama sonucu değiştirmiyor: dipte olmanın bedeli zayıf.
4. Hız hâlâ baskın.
5. Şut sorunu düzelince ölçüm tekrarlanmalı, tercihen 5 dakikalık tam maçla.
