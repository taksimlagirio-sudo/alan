# Birebir aynı hızlandırma (zip 28 üzerine) · siteye konmadı, onay bekliyor

`base` = Design zip 28 motoru, `opt` = hızlandırılmış. Kural, sayı ve işlem sırası değişmedi; yalnız sonucu etkilemeyen hesap atlandı ya da aynı değer bir kez okunur oldu.

- `core.js` `pack`/`Kpk`: yarıçap çarpımları pakette bir kez; menzil dışındaki oyuncunun 0 katkısı eklenmiyor.
- `core.js` `hyp`: büyük olan kendine bölününce tam 1 olduğu için `sqrt(1 + r²)·m` (aynı toplam).
- `decide.js` `launchFor`/`solveTo`: Çekirdek denemelerinde alan paketlenmiş diziden (Kpk, slow K ile bit bit aynı).
- `decide.js` `chasePlan`: tepki süresinden önceki anlar ve x/y farkı tek başına yetişmeye engel olan noktalar atlanıyor (güvenli pay ile); oyuncu konumu bir kez okunuyor.
- `decide.js` `simOne`: alan paketi bir kez kuruluyor, her kullanımdan önce yalnız konumlar tazeleniyor; alıcının yolu yalnız bakacağı ana kadar hesaplanıyor (bulamazsa tam yol).
- `decide.js` `space`/`front`: oyuncuların konum, hız, Okuma ve takım bilgisi döngü dışında bir kez (`bosOy`).
- `decide.js` `reachSpeed`, `shape.js` `alici`: konumlar döngü dışında.
- `match.js` `govdeAyir`: konumlar düz diziden; kımıldayan oyuncunun yeri hemen geri yazılıyor.

Doğrulama: `tools/v5-iz2.js`, 16 kurulum (Liman–3 rakip ×2, rakip–rakip ×3, Liman'ın 3 farklı taktiği, 2 rastgele özellik, 2 maç içi taktik değişikliği), tam maç, her tik bütün oyuncuların konum/hız, Çekirdek, yük, skor, top sahibi, olay sayısı: 16/16 birebir aynı.
Süre: iki motor aynı anda aynı yükte: 116,9 → 91,0 sn, 110,6 → 89,2 sn (%19–22). 16 maç toplam (benzer yük): 3134 → 2353 sn (%25).
