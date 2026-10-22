# Maç 380418 · Liman – Kale Duvarı: maç takılıyor (22 Ekim 2026, 19:07)

> Kullanıcının maçı; sitede 02:06'da (kare 3780) takıldı. Motor: o saatte yayındaki main 64d5e04. Kayıt `kayit.json`; yeniden oynatma `an.js` (profil), `an2.js` (300 tik izleme), çıktılar `an-64d5e04.txt`, `an2-64d5e04.txt`.

## Ne oluyor
- Maç ~02:19'da (4171. tik, skor 3–0) **kilitleniyor**. Site maçı izlenenden birkaç saniye önde hesapladığı için görüntü 02:06'da duruyor.
- Liman'ın Bekçi'si **Aras** topla sahanın ortasına yakın (x 47, y 26) duruyor. Kale Duvarı'nın **Ok**'u 1,7 br dibinde ("geri geri" işinde), **Hendek** 2,5 br'de ("pres" işinde). Kimse topa girmiyor.
- Aras her tik **"tut, 1 sn bekle"** seçiyor. 300 tik (10 maç saniyesi) boyunca yeri değişmiyor (47,4 → 47,2); bütün oyuncuların toplam hareketi tik başına 0,1–0,7 br. Maç kendiliğinden çözülmüyor.
- **Her tik ~1 sn hesap** (normalde on milisaniyeler): 30 tik = 1 maç saniyesi ≈ 30 sn hesap. Ekranda takılma bu.

## Neden
1. **Her tik yeniden karar (`match.js:430`):** taşıyıcının bekleme planı (`holdD`) varken bir rakip 3,5 br'den yakına girerse plan bozulur, `decT = 1`, bir sonraki tikte yeni karar. Burada rakip zaten 1,7 br'de: kafa yine "bekle" seçiyor, plan bir sonraki tikte yine bozuluyor. Her tik tam karar hesabı.
2. **Kararın kendisi pahalı:** profilde zamanın %87'si kararda; %56 Bekçi'ye özel tartı (`bekTart`), %38 kenardan pas (`bankTart`, `kenarIm`), uçuş ve sekme yarışları (`ucusFormul`, `sekYaris`, `yaris`).
3. **Karşılıklı bekleme:** Aras rakip girmediği için tutmayı değerli buluyor; rakipler (pres yapmayan Kale Duvarı; Ok geri çekiliyor, Hendek basmıyor) Aras kıpırdamadığı için girmiyor. İki taraf da yerel olarak "doğru" bekliyor; kimse bozmuyor.

## Öneriler (motor oturumu için)
- **Döngüyü kır (hız):** bekleme planı, rakip *yeni* 3,5 br'ye girince bozulsun; karar anında rakip zaten oradaysa bozulmasın. Ya da plan bozulduktan sonra kafa aynı tikte "bekle"yi yeniden seçerse bir sonraki karar birkaç tik ertelensin.
- **Beklemeyi bitir (oyun):** topu tutmanın süresi bir bedel taşımalı (Bekçi için "topu elde tutma süresi" mekaniği Aşama 8'de, bağlanmadı) ya da taşıyıcı uzun süre kıpırdamazsa dibindeki rakip girsin. Sahada Bekçi'nin orta sahada topla beklemesi de ayrıca sorgulanmalı.
- **Güvenlik:** bir tikin hesabı belli bir süreyi aşarsa (örneğin 200 ms) kafa ucuz yola geçsin; site donmasın.
