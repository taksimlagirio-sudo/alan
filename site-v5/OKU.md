# ALAN v5 · site

Düz statik site: derleme adımı yok, sunucu kodu yok. Klasörü olduğu gibi yayınla.

Sayfalar:
- index.html: Taktik ekranı (giriş). "Maça çık" → canli.html
- canli.html: canlı maç
- menajer.html → mac.html: eski menajer ve maç ekranı (deneme için duruyor)

Netlify: app.netlify.com/drop adresine bu klasörü (ya da zip'i) sürükle bırak.
Cloudflare Pages: Workers & Pages → Create → Pages → Upload assets → klasörü yükle. Build komutu yok.

Not: Sayfalar React'ı unpkg.com'dan, yazı tiplerini Google Fonts'tan yükler; internet bağlantısı gerekir.
