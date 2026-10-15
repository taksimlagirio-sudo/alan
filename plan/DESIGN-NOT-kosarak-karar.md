# ALAN · Koşarak topu alan neden durur: fake ve tut, sürmeyle neredeyse eşit değerde

Claude Code'dan Design'a. Kaynak: zip67 ve zip69 kayıtları (16'şar maç), karar kaydı. Topu koşarak (hız > 0,15) alan oyuncunun ilk kararı (önüne / ayağa / kenardan pas). "Koşmaya devam": 10 tik sonra hız > 0,15 ve 30 tik sonra > 0,12. Araç: `tools/kosu-karar.js`. (Bu ölçüde ilk kararı kayıtlı olan alışlar var; tablodaki %53'ten farklı alt küme, oran %36–37.)

## 1 · İlk karar ve sonrası (zip69; zip67 benzer)
| İlk karar | Hepsi | Önü açık* | Önü kapalı | Sonra koşmaya devam |
|---|---|---|---|---|
| Sür | %35 | %39 | %28 | **%82** |
| Fake | **%45** | **%37** | %58 | %14 |
| Tut | %21 | **%24** | %14 | %3 |

*Önü açık: alıcının 8 birim içinde, önünde (rakip Kuyu yönünde) Bekçi dışı rakip yok. 875 alışın 557'si.

- Koşarak alıp duranların neredeyse hepsi **fake ya da tut** seçiyor. Sürmeyi seçen %82 koşmaya devam ediyor; sorun sürme değil, sürmenin seçilmemesi.
- **Önü açıkken de** fake %37, tut %24. Önünde 8 birim kimse yokken aldatacak kimse yok; beklemek de koşunun hızını sıfırlıyor.

## 2 · Değerler birbirine çok yakın
"Sür" seçilmeyen 571 kararın 338'inde "sür" ilk 4 seçenekte:
| Seçilen − "sür" değeri | Hepsi | Önü açık | Önü kapalı |
|---|---|---|---|
| Ortanca | 0,015 | 0,019 | 0,009 |
| %25 | 0,002 | 0,003 | 0,001 |
| %75 | 0,062 | 0,066 | 0,041 |
| 0,01'den az fark | **%44** | %39 | %53 |
Kararların neredeyse yarısında fake / tut sürmeyi 0,01'den az farkla geçiyor: seçim fiilen yazı-tura. Kalan 233 kararda "sür" ilk 4'e hiç girmiyor.

## 3 · Muhtemel sebepler
1. **Fake'in değeri aldatacak kimse yokken de yüksek.** Fake'in değeri, gösterilen hamle ile asıl hamlenin değerinden geliyor; aldatma ihtimali (inanılırlık) düşük olsa da değer asıl hamleye çok yakın kalıyor, gecikmenin bedeli küçük görünüyor. Önü açıkken fake yalnız gecikme (ve koşu hızının kaybı) olmalı.
2. **"Tut"un değeri koşu hızının kaybını görmüyor.** Koşarken tutmak, taşıyıcıyı durdurup sonra yeniden hızlandırıyor; rakip bu sürede yerleşiyor. Kopya beklemeyi "yerinde" oynatıyor ama taşıyıcının başlangıç hızını (koşarak alması) değer farkına yansıtıyor mu, belirsiz.
3. **Sürmenin ilerleme değeri düşük.** Önü açıkken bile "sür" ilk 4'e girmeyen kararlar var (132 / 557 açık alışta).

## 4 · Öneri
- **Fake yalnız aldatılacak biri varken:** fake seçeneği, fake'in yöneldiği rakip fake'e tepki verebilecek uzaklıktaysa üretilsin (ya da değeri, aldatma ihtimali × kazanç − gecikmenin bedeli olsun; aldatma ihtimali ~0 ise fake asıl hamleden açıkça düşük). Gizli katsayı değil: "aldatacak kimse yoksa çalım atılmaz".
- **Tut'ta koşunun bedeli:** taşıyıcı koşarken "tut" seçeneğinin kopyası, durmanın ve yeniden hızlanmanın süresini içersin (kopyada taşıyıcı hızını koruyarak başlıyor mu, kontrol edilmeli).
- Kabul (kontrol tablosu): koşarak alıp koşmaya devam eden ≥ %60; önü açıkken ilk karar fake / tut ≤ %30; pas tutma ve top kaybı kötüleşmesin.

Not: zip69'daki "top gelmeden karar" bu sorunu çözmedi çünkü karar erken verilse de içeriği aynı (fake / tut). Bu not, içeriği hedefliyor.
