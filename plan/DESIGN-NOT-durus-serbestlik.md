# ALAN · Design için kısa not: elle verilen duruş ve serbestlik

**Bugün (zip42):** Tahtada yeri elle verilen oyuncu, serbestliği ne olursa olsun en az "serbestlik 0" kadar evine bağlanıyor, üstüne el payı ekleniyor (`shape.js` `elK`). Yani elle yer vermek oyuncunun serbestliğini fiilen siliyor.

**Kullanıcının istediği:** Elle yer vermek serbestliği ezmesin; oyuncunun **nereden** oynayacağını söylesin. Ne kadar uzaklaşabileceğini serbestlik belirlesin. İki ayar birlikte bir düğme gibi çalışsın:

| Menajer ne yaptı | Oyuncu ne yapmalı |
|---|---|
| Çizgiye koydu, serbestlik **düşük** | Orada durur. Kendi bölgesinde gerçek konumu evine **en az yarı yarıya** uyar (kabul ölçütü bu durum için). |
| Çizgiye koydu, serbestlik **orta** | Çoğunlukla orada; açık bir boşluk ya da pas çağırırsa ayrılır, sonra döner. |
| Çizgiye koydu, serbestlik **yüksek** | Oradan başlar, boşluğa göre içeri girer (Kuyu önü dahil). Menajerin bilerek verdiği karar. |

**Kurallar:**
- El payı serbestliğin **yerine** geçmesin; serbestlik bağını kendi kademesinde bıraksın, el payı yalnız evi (başlangıç yerini) belirlesin. İstenirse el payı bağa küçük bir ek yapabilir, ama serbestlik yüksek bir oyuncuyu serbestlik 0'a çevirmemeli.
- Kuyu önü halka adayları ve koşu adayları da serbestliğe uysun: düşük serbestlikte ellenen oyuncu Kuyu önüne ancak bir pas onu oraya çağırırsa gitsin; yüksek serbestlikte kendisi de gidebilsin.
- Ekranda bu ilişki görünür olsun: tahtadaki kesik serbestlik halkası, elle konan yerin çevresinde çizilsin.

**Claude Code ölçecek:** Aynı duruş (iki kanat çizgide), kanatların serbestliği 0 / 0,5 / 1. Beklenen: gerçek konumun evden uzaklığı serbestlikle düzenli artsın; serbestlik 0'da yarı yarıya ölçütü tutsun.

(Bugünkü ölçüm, zip42: kanatlar en sıkı bağdayken bile ev 19,4 → seçilen hedef 15,5 → gerçek konum 9,6 (ortadan uzaklık; varsayılan ev 13,5). Yani serbestlik 0'da bile şu an tutmuyor: önceki notun önerileri (evden uzaklığın bedeli; Kuyu önü adaylarının sınırlanması) bu notla birlikte geçerli.)
