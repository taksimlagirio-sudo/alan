# Kontrol tablosu

Her yeni zip aynı 16 maçta (senaryo 0–15, 120×60, herkes 20, ayarsız takımlar) oynatılır ve bugüne kadar bulunan bütün sorunlar aynı ölçülerle bir önceki sürümle yan yana konur. Bir kural başka bir şeyi bozarsa aynı gün görünür.

    tools/kontrol-kos.sh ../siteNN/alan-v5 zNN lab/kayit/<önceki>     # ~50 dk (16 maç × 120 sn, 4 paralel)
    node tools/kontrol-tablosu.js lab/kayit/A=ad1 lab/kayit/B=ad2 ...  # yalnız tablo, kayıtlardan (1 sn)

Maç süreleri farklı olabilir; sayımlar dakika başına. ↑ / ↓ iyi yön; tabana göre %15'ten çok kötüleşen ⚠, iyileşen ✓.

| Satır | Neden var (hangi sorun) |
|---|---|
| Sayı, gönderme, pas, kesme, pas tutma | Genel denge |
| Hesap ms/tik | Hız (4 maç birlikte oynarken; oran önemli) |
| Saha dışında Çekirdek | zip52 hatası |
| Bekçi pas kovalaması, kazanma, sayılarda yerinde | zip49b–50: Bekçi yetişemeyeceği topa çıkıyordu |
| Yakın direkten sayı, dar açı sayı oranı, Bekçi–yakın direk | Köşeden, Bekçi ile direk arasından sayı |
| Uzak geniş / yakın geniş (60–75°) | zip48–49: geniş açıdan fazla sayı |
| Kontra: önü boşken topla duran (santra hariç) | Kontrada bekleme |
| Pastan sonra geri koşan pasçı | zip56: ver-koş |
| Baskı altında "tut" ve sonrası kayıp | zip54/57: bekleme |
| Boş hücumcu, çift adam, iki kişi "arkada", taşıyıcıya baskı yok | zip55: yanlış adama koşma |
| Her an / uzun tutuşta duran hücumcu | zip34–38: durma |
