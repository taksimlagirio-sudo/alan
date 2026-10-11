# ALAN v5 · Claude Code için

**Temel:** v3d motoru (kararlar ve yerleşim kuralları olduğu gibi) + v4'teki fizik ve kural mekanikleri. Beyin, hafıza ve tablo turları yok.
Neden: v3d'nin sahne sahne inceltilmiş yerel kuralları, v4'ün tek beyninin gürültülü ileri oynatma değerlendirmesinden daha iyi karar veriyordu (kullanıcı gözüyle). Beyin ileride ancak ölçülerek kuraldan iyi olduğu bir karara girer.

## Dosyalar
- `core.js`: fizik (v4h'den, olduğu gibi): Çekirdek, alanlar, şarj sönümü, temas/karşılama, herkes için tepki süresi (hazırlık), Bekçi elleri (uzanma, kapasite, yana çelme), kontrol edilemeyen top arkadaşta geliş yönünde kaçar (`ALAN_KACIR`).
- `decide.js`: v3d top sahibi kararı (pas, önüne, kenardan, sürme, tut, gönder). v5 değişikliği: şarj payı (`chgMin .1`: duran tam, tam hızla koşan %10); Bekçi serbestse yarış hesabında görülür.
- `shape.js`: v3d yerleşim (topsuz oyuncular, savunma, pres, kovalama). v5 değişiklikleri:
  - Bekçi açıortayda (topla iki direk arası açının ortası), derinlik Okuma'ya bağlı.
  - Bekçi serbest (`S.bekSerbest`): boştaki topa ve rakip pasına ancak takımında ilk o varacaksa çıkar; kendi takımının pasına koşmaz.
  - Kendi pasımız havadayken tek kişi gider (alıcının önceliği +10 tik).
  - Her rakibi tek kişi tutar: arkada kalanlar derin rakipleri sırayla paylaşır; fazlası alan kapatır.
- `match.js`: v3d maç adımı. v5 değişiklikleri: dizilişler (`dizSlots`, `dizUygula`; savunma `slot`, hücum `rslot`), katı gövdeler (oyuncular 3 geçişle ayrışır; Çekirdek tutmayan gövdeden seker, `BODY_R 1.0`, `BODY_E .35`), topa yığılma yok (alıcı dışındakiler buluşma noktasından 7 br uzakta), topla hız Sürme'ye bağlı (`topHiz`), gönderme isabeti (`GON`: baskı, hareket, güç dağıtır; Aktarım küçültür), şarjda hareket payı oyuncunun kendi en yüksek hızına göre.
- `mac-isci.js`: maçı izlemenin önünde oynar. v3d'de kopya yok: taktik değişince maç aynı tohum + değişiklik geçmişiyle baştan o ana kadar yeniden oynanır (belirlenimci). Menajer kurulumu (`oyuncular`, `roller`) `kurUygula` ile uygulanır; diziliş rollerden sonra yeniden uygulanır (yer dizilişten, davranış rolden).
- `ALAN v5 Maç.html`: maç ekranı (yayın görünümü, gece/gündüz, alanlar, olaylar, karar paneli = v3d'nin son kararı). `?menajer=1` ile `localStorage['alan-v5-menajer']`dan kadro/roller/diziliş/taktik okur.
- `ALAN v5 Menajer.dc.html` (+ `support.js`): maç öncesi ekran. Takım Liman (12 oyuncu), 3 sabit rakip (Demir Blok, Fırtına, Saat Kulesi), mevki ataması, oyuncu başına hücum/savunma rolü, ayrı savunma/hücum dizilişi, takım talimatları.
- Diğerleri (`scenes.js`, `*-worker.js`, `calib.js` …): v3d'den kalan sahne/ölçüm araçları.

## Açık konular
1. **Karar kapıları duruyor:** gönderme menzili, kenardan pas / tek dokunuş / fake için Okuma eşikleri (v3d'deki gibi). v4'te beyin tarttığı için kaldırılmıştı; v3d kararıyla kaldırmak uzaktan gönderme yağmurunu geri getirebilir. Kaldırılacaksa önce ölç.
2. **Perde:** gövdeler katı, ama perde yapmaya karar veren bir yerleşim mantığı yok.
3. **Sayı/maç düşük olabilir** (kısa denemelerde 90 sn başına 0–3). Bekçi'nin eli + açıortay v3d kapılı kararıyla göndermeyi fazla zorlaştırıyor olabilir. Ölç: uzaklık dilimine göre gönderme ve sayı oranı.
4. **Rol ve diziliş:** yer her zaman dizilişten; v3d'de Kanat/Pivot/Kurucu/Koşucu rolü yeri de seçiyordu (`RSLOT`), şimdi diziliş onu eziyor.
5. **Hız:** v3d motoru tek iş parçacığında ~70–90 tik/sn; 90 sn maç ~1 dk. Profil çıkar.

## Kurallar (CLAUDE.md)
Katman katman; kullanıcı gözle onaylamadan sonrakine geçme. Yama üstüne yama yok, kural yazıp katsayıyla gizleme yok. Denge ölçümü hücre başına ≥30 maç.
