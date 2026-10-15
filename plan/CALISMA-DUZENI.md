# ALAN · Çalışma düzeni (15 Ekim 2026'dan itibaren)

Kullanıcının kararı: **Design yalnız tasarım verir; oyun motoruna karışmaz.**

## Kim neyden sorumlu
- **Claude Code — oyun motoru:** `alan-v5/*.js` (core, decide, match, shape, shot-table, value-table, mac-isci, lab-isci, analiz-*, kadro). Kurallar, akıl, fizik, tablolar, maç worker'ı, kadro verisi.
  - Her değişiklik kendi bayrağıyla, ölçümle ve tarayıcı testiyle siteye girer.
  - Değişiklik günlüğü: arsiv dalında `plan/DEGISIKLIK-GUNLUGU.md` (zip71'e kadarki günlük Design'ın; sonrası Claude Code'un).
- **Design — sayfalar ve tasarım:** `ALAN Oyun.dc.html`, `ALAN v5 Taktik.dc.html`, `ALAN v5 Canlı.dc.html`, `ALAN Lab.dc.html`, `support.js`. Görünüm, ekranlar, etkileşim.

## Design paketi gelince
1. Yalnız sayfa dosyaları alınır. Paketteki `alan-v5/*.js` yok sayılır; sitedeki motorun üstüne yazılmaz.
2. Sayfanın motordan beklediği yeni bir şey varsa (yeni bir alan, yeni bir mesaj), motora Claude Code ekler.
3. Oynanışa dokunan sayfa ayarları her pakette kontrol edilip korunur. Şu an: **Canlı sayfada başlama payı %30** (`kareler.length - t >= .3 * (hedef - t)`; Design'ın dosyasında %20 yazıyordu).

## Sitedeki motor
- Son sürüm: zip78 + canlı sayfa %30 (main `ec4dc5c`).
- Motorun zip71'e göre farkı: `plan/zip78-motor.diff`, `plan/DESIGN-NOT-zip78-sitede.md`.
