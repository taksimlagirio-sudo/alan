# Tasarım planı · Satış modeli: ücretsiz başla, hesapla kilit aç

> Durum: karar aşaması, kod yok. 20 Ekim 2026. Fikir kullanıcının. Mağaza kuralları sık değişir; buradaki bilgiler bugünkü aramalara dayanır, başvurudan önce güncel hâlleri kontrol edilmeli.

## Model

- Oyun **her platformda ücretsiz** indirilir (mobil, masaüstü).
- Oyunun tamamı **tek seferlik bir kilitle** açılır. Kilit **hesaba bağlıdır**: hangi cihazdan, hangi mağazadan girilirse girilsin hesap açıksa oyun açıktır.
- Kariyer kaydı da aynı hesaba bağlıdır (plandaki "kariyer cihazlar arasında taşınır").
- Planın ilkeleriyle uyumlu: reklam yok, güç satılmaz. Satılan şey güç değil, oyunun tamamına erişim.
- İleride online sezon aboneliği gelirse kilitten ayrı ve açıkça anlatılmış bir şey olur.

## Fiyat

- Kilit için **7–10 €** makul (derin menajerlik oyunları: FM masaüstü ~50 €, FM mobil ~10 €, bağımsızlar 10–20 €). 3 € düşük: KDV ve mağaza payından sonra satış başına ~1,7–2,1 € kalır; sunucu maliyetini karşılamaz.
- Mağazaların bölgesel fiyatlandırması: Türkiye'de çok daha düşük tutulabilir.
- Mağaza payları: Apple ve Google küçük geliştiriciden %15, Steam %30, web (kendi siten) yalnız ödeme sağlayıcısı payı.

## Ücretsiz kısmın sınırı (karar bekliyor)

İlk sezon mu, tek lig mi, belli sayıda maç mı? Oyuncu oyunu sevecek kadar oynayabilmeli; kilidi açmak için de bir sebebi kalmalı. İlk 10 dakika oyunun kendine özgü yanını göstermeli (mobilde ücretsiz oyun rekabeti çok yüksek).

## Mağaza kuralları

### iOS (Apple)
- **3.1.3(b) "çok platformlu hizmet":** başka yerde satın alınmış içeriğe uygulamada erişime izin verir. **Şart:** aynı kilit iOS uygulamasında da uygulama içi satın alma (IAP) olarak satılmalı.
- iOS kullanıcısı uygulama içinde başka bir ödeme yoluna yönlendirilemez.
- Sonuç: iPhone'da "kilidi aç" Apple ödemesiyle; Android'de, webde ya da başka yerde açılmış kilit iPhone'da hesapla girince açık.

### Android (Google)
- Benzer: uygulama içinde Google ödemesi; dışarıda alınmış hakka erişim genelde mümkün.

### Steam (belirsiz, Valve'a sorulmalı)
- Steam sürümünde oyun içinde satılan her şey **Steam Cüzdanı'ndan** geçmek zorunda (Steamworks: oyun içi satışlar mikro-işlem API'siyle).
- Steamworks'ün uygulama rehberi, oyunun başka satış yollarını da yasaklamadığını söylüyor; ama **başka mağazada alınmış bir kilidin Steam sürümünde tanınmasına** dair açık bir kural bulunamadı.
- Bilinen örnekler kısıtlayıcı:
  - **Killer Instinct (2016):** Xbox ve Windows 10'da yapılan satın almalar Steam'e taşınmadı.
  - **eFootball:** ilerleme KONAMI ID ile taşınabiliyor, ama **satın alınan eFootball Coin'ler platformlar arasında taşınmıyor** (iOS ile Android arasında bile). Platformlar arası paylaşılan tek şey eFootball Points (ayrı bir ödül puanı).
- Seçenekler:
  1. **Valve'a sormak** (Steam partner desteği): mobilde ya da webde açılmış hesap kilidi Steam sürümünde tanınabilir mi?
  2. İzin yoksa **Steam ayrı satış:** Steam sürümü ya ücretli oyun ya da ücretsiz + Steam Cüzdanı'yla kilit. Kariyer kaydı hesapla yine taşınır; yalnız kilit Steam'de ayrıca alınır.
  3. Steam anahtarı dağıtımı (mobilde alana Steam anahtarı): Valve'ın anahtar kuralları Steam müşterisine daha kötü koşul verilmemesini istiyor ve anahtar taleplerini sınırlayabiliyor. Ayrıca sorulmalı.

## Kurulacaklar

1. **Hesap sistemi ve lisans sunucusu** (kariyer kaydı için zaten gerekli; Supabase bağlantısı mevcut).
2. **Çevrimdışı oynama:** kilit cihazda saklanır, ara sıra çevrimiçi doğrulanır.
3. Her mağazada kendi ödeme entegrasyonu (Apple IAP, Google Play Billing, Steam mikro-işlem ya da ücretli oyun) ve hepsinin aynı hesap lisansına yazması.

## Kaynaklar

- Apple Developer Forums: [3.1.3(b) Multiplatform Services](https://developer.apple.com/forums/thread/117077), [çok platformlu ücretsiz uygulama](https://developer.apple.com/forums/thread/825551), [3.1.1 ile 3.1.3(b)](https://developer.apple.com/forums/thread/740710)
- Steamworks: [Free to Play](https://partner.steamgames.com/doc/store/freetoplay), [Microtransactions Implementation Guide](https://partner.steamgames.com/doc/features/microtransactions/implementation)
- [TweakTown: Killer Instinct Steam'de](https://www.tweaktown.com/news/59309/killer-instinct-now-live-steam/index.html)
- Konami: [eFootball FAQ](https://www.konami.com/efootball/faq/0/en-us/), [eFootball Points](https://www.konami.com/wepes/efootball_point/en/)
