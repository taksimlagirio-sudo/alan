# Kafanın savunma öngörüsü maçın görev dağılımını kullanmalı (davranış oturumundan mimari oturumuna)

## Sorun

Kafa bir pası ya da sürmeyi tartarken savunmacıların T tik sonraki yerini kendi basit kurallarıyla öngörüyor:

- `kafa1_tehlike.rs` `respond`: en hızlı iki savunmacı doğrudan alıcının kale tarafına koşar.
- `kafa2_dunya.rs` `predictWorld`: öteki savunmacılar o anki hedeflerine (`tx, ty`) yürür, tuttuğu adamı (`tutQ`) izler ya da yerleşime (`slotBase`) gider; Bekçi topun gideceği yerin idealine dosdoğru (`bekKay`).

Maçta savunmacıların hareketi bambaşka bir kuraldan geliyor: `sekil/konum/esles.rs` (eşleşme işleri: kişi, pres, bölge, buluşma) ve `sekil/konum/savunma.rs`. Bu işler topun durumu değişince (pas atılınca) **hemen yeniden dağıtılıyor**. Kafa ise karar anındaki işleri ileriye taşıyor.

Davranış oturumunda anları tek tek açarken (sina/dene.sh, ALAN_UCUS_IZ, ALAN_ENIYI_IZ, ALAN_BUL_IZ dökümleri) üç ayrı anda aynı kökten hata çıktı:

| an | kafanın öngörüsü | maçta |
|---|---|---|
| k107-292 (Kaan'dan Ece'ye ikinci pas) | Efe 31 tikte (109.1, 33.8), pas yolunun 1.1 br yanında: yol kapalı | Karar anında Efe'nin işi "bölge · geri", hedefi (114.8, 35.4). Pas atılınca bir tik sonra işi "arkada · Cansu": (98.6, 31) → (100, 31), 9 br uzakta. Pas geçti, gol |
| k41-370 (Tuna'dan Ece'ye) | Efe buluşmaya dönüp Ece'nin dibinde | Efe'nin işi "pres · buluşma", hedefi doğru; ama kaleye doğru koşarken pas geldi, frenleyip dönmesi ~12 tik sürdü, yetişemedi (bu kısmı alış yeri ve süre düzeltmeleri çözdü) |
| k41-370 (Bekçi) | Bekçi topun gideceği yerin idealine 27 tikte vardı | Bekçi topu izleyerek kaydı, geride kaldı (ALAN_TEK_SUT_IZ'de kafa içinde benzetildi) |

## İstek

Kafa savunmacıların pas / sürme sırasındaki hareketini maçın kendi kuralıyla öngörsün: "Rust tek doğru kaynak". Yani hayali dünyada (topun alıcıya gittiği, taşıyıcının sürdüğü) savunmanın işleri maçtaki gibi yeniden dağıtılsın (`esles`), her savunmacı o işin hedefine maçın koşusuyla (`kural/hareket.rs`) gitsin, Bekçi maçtaki gibi topun o anki yerine göre yerleşsin.

Bunun yapısal tarafı (kafanın yerleşim kodunu hayali bir dünyada çalıştırabilmesi: `Mc` / yerleşim girdilerinin hayali dünyadan kurulabilmesi, maliyetin taşınabilir olması) sizin alanınız. Davranış tarafı (hangi anlarda nasıl ölçüleceği, eski öngörüyle karşılaştırma) bizde: kurulunca anlar üzerinden (sina/dene.sh, tarama) ölçeriz.

## Ara yama (davranış oturumu, deney)

`ALAN_TUTAN_KALIR_IZ` (varsayılan kapalı): `respond`'da maçta birini tutan savunmacı (`tutQ`) da `markRef` gibi görevinde kalır. k107-292'de Tuna'nın kararını değiştiriyor (Ece'ye pas), ama Kaan'dan Ece'ye ikinci pası açmıyor: oradaki hata görev değişiminin kendisi.
