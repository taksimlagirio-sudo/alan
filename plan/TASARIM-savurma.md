# Tasarım · Savurma: pas ve şut tek kural (sapan gibi teğet bırakma)

> Durum: tasarım, kod yok. 16 Ekim 2026. Menajerin kararı: a (teğet bırakma). Günlükte kayıtlı eski "fırlatma (savurma)" mekaniği bugünkü motorda yok; bu not onu genel kural olarak yeniden kurar.

## Güncelleme · 17 Ekim 2026: karma bırakma (a + itme)

Sitedeki saf teğet sürümde (main 4ab0811) dönüş çok fazlaydı: maçta pas başına ortanca ~195°, şutta ~2 tur. Çekirdek öndeyken ileri pas iki yönden de 270° istiyordu (teğet bırakma noktası gövdenin yanında). Menajerin kararı: karma yol.

- **Bırakma hızı iki parçadan oluşur:** dönüşün teğeti (ω × 1,15) ve gövdeden dışarı itiş. Topun yönü, Çekirdeğin yönü + s × β, β = atan2(ω × 1,15, itiş).
- **İtişin üst sınırı Güç'e bağlı:** Güç 10'da 1,0, Güç 20'de 1,3. Bu hıza kadar pas yalnız itişle çıkabilir; daha sert vuruş, itişin yetmediği kadar dönüş ister.
- **Dönüş ivmesi:** Güç 10'da 0,45, Güç 20'de 0,70 rad/tik².
- **Geri çekme:** Çekirdek önce ters yöne çekilir, yavaşlayarak durur, sonra savrulur. Açısı en çok 135°; savurma gereken hıza tam bırakma noktasında ulaşacak kadar seçilir.
- **Plan:** iki yön × bırakma anındaki üç dönüş hızı adayı × geri çekmeli ya da geri çekmesiz; en kısa süren seçilir.
- **İsabet:** açı hatası = (dönüşün zamanlama payı ω × τ + itişin payı × eski nişan hatası) × (20 − Pas ya da Gönderme) / 10. Yalnız itişle çıkan pas eski hatayla birebir aynı.
- **Ölçüm** (maçın ilk 900 tiki, ortanca dönüş):

  | | Saf teğet | Karma |
  |---|---|---|
  | Kısa pas | 198° | 10° |
  | Orta pas | 196° | 44° |
  | Uzun pas | 194° | 128° |
  | Şut | 642° | ~300° |
  | Çekirdek öndeyken pas | 271° | 16° |

- **Bayrak:** `ALAN_SAVUR_ITME_ESKI` (saf teğet). Şut tablosu sürümü `v5-20 · karma bırakma` (`ALAN_SHT_KARMA`).

## Bugün

Pas ve şut kararı verildiği tik Çekirdek gövdenin kenarından istenen yöne ve hızla anında çıkar. Çekirdeğin gövdenin hangi yanında olduğu, döndürmenin süresi ve açısı fırlatmayı etkilemez (909623 00:11: Çekirdek arka yandaydı, top öne Kaan'ın içinden geçerek çıktı). Şutta 4 sabit hedef (güçlü 2,6 / plase 1,7, kale ortasına göre sabit noktalar), Bekçi'nin yerine bakılmaz.

## Kural

1. **Dönüş:** Çekirdek gövde yarıçapında (Q.body = 1,15 br) döner. Taşıyıcı savurmaya başlayınca dönüş hızı ω her tik α kadar artar (en çok ω_max).
2. **Bırakma:** Top, bırakıldığı andaki dönüşün teğetiyle çıkar: hız = ω × 1,15 (+ taşıyıcının hızı). Hedefe atmak için Çekirdek hedef yönüne göre gövdenin yanındayken (90°) bırakılır. Plan, dönüşün tam o açıda ve gereken hızda bitmesi için kurulur (gerekirse yarım tura kadar fazladan dönüş).
3. **Süre:** Gereken hız ω = v / 1,15. Hızlanma süresi ω / α, üstüne açıyı tutturma payı. Sert şut uzun, kısa pas kısa hazırlık ister.
4. **Oyuncu:** α Güç'e bağlı (güçlü oyuncu aynı hıza daha az dönüşle ulaşır); ω_max Güç'e bağlı (en sert şut); bırakma zamanlaması Pas / Gönderme'ye bağlı.
5. **İsabet:** Hata, bırakma anının zamanlamasından gelir: açı hatası ≈ ω × zamanlama hatası. Hızlı dönüş (sert şut) doğal olarak daha az isabetli; yetenekli oyuncunun zamanlama hatası küçük. Bugünkü rastgele açı sapması bunun yerine geçer; hız hatası kalır.
6. **Risk:** Savurma süresince Çekirdek gövdenin etrafında döner. Yetişen rakip bugünkü kuralla (gövdeler katı, ikili) Çekirdeğe çarpabilir ya da uzanıp alabilir. Taşıyıcı savururken yavaşlar (hız ×0,5). Savurma başlayınca karar kilitlenir.
7. **Şarj:** Hedef hız bugünkü gibi `o.v × chgMul(şarj)`.
8. **Tek dokunuş · kanca:** Gelen top durdurulmaz: gelişinin hızıyla (ω₀ = u / 1,15) gövdenin etrafında kıvrılır ve yeni yöne teğet çıkar. Dönüş açısı büyüdükçe hız kaybeder (çıkış ≈ u × (1 − 0,35 × Δ/π) + küçük ek itiş). Süre Δ / ω₀; görüntüde top gövdenin etrafında eğriyle dönüp gider. İsabet aynı kural (ω × zamanlama).
9. **Şut hedefi:** Sabit noktalar yerine, şutu çekenin gözünden iki direk arasındaki açıklık taranır, Bekçi'nin topa yetişebileceği aralık çıkarılır, en geniş iki boşluğun ortası hedef olur; her biri için iki şiddet. Seçenek sayısı 4'te kalır (hesap süresi).

## Önerilen sayılar (ayarlanacak)

| | Güç 10 | Güç 20 |
|---|---|---|
| α (rad/tik²) | 0,18 | 0,30 |
| Sert şut (v 2,6, ω 2,26) hazırlık | ~13 tik (0,42 sn) + açı payı | ~8 tik (0,25 sn) + açı payı |
| Normal pas (v 1,0, ω 0,87) | ~5 tik (0,16 sn) | ~3 tik (0,1 sn) |
| Kısa pas (v 0,6) | ~3 tik | ~2 tik |

Zamanlama hatası (tik): 0,08 × (20 − Pas ya da Gönderme) / 10. Pas 10'da sert şutta ~10°, normal pasta ~4° açı hatası; 20'de sıfıra yakın.

## Akıl (kafadaki tartı)

- **Şut:** Kafadaki maç (ucusMac) savurmayı da oynar: kopya maçta taşıyıcı savurma durumunda başlar, rakipler tepki verir, bırakma maçtaki kuralla olur. Kural ayrıca yazılmaz.
- **Pas:** Basit deneme (simOne) için: savurma süresi T kadar ilerletilmiş dünyadan fırlatılır; savurma süresince tutma ihtimali (keepP, T) ile çarpılır, kayıpta lossAt.
- Böylece hazırlığı uzun sert şut ya da pas, dibinde rakip varken kendiliğinden az değerli çıkar.

## Ölçülecek sahneler (siteye almadan)

1. **Karşı karşıya:** Yalnız hücumcu ve Bekçi; 10, 15, 20 br, üç açı. Gol oranı ve şutun nereye gittiği, bugünkü motorla.
2. **Baskı altında pas:** Dibinde presçi (2–4 br); savurma sırasında kayıp oranı, kısa pas ve uzun pas.
3. **Ters yan:** Çekirdek arka yandayken öne pas; hazırlık süresi ve çıkış yönü (Kaan 00:11 durumu).
4. **Kanca:** Gelen pasa tek dokunuş, 30°, 90°, 150° dönüş; çıkış hızı ve isabet.
5. **Genel:** İki pencerede pas isabeti, şut sayısı ve gol, hesap süresi.

## Bayraklar

`ALAN_SAVURMA_ESKI` (anında fırlatma), `ALAN_KANCA_ESKI` (eski tek dokunuş), `ALAN_SUT_HEDEF_ESKI` (sabit 4 nokta).
