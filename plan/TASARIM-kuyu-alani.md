# Tasarım planı · Kuyu alanı (süre sınırlı bölge) ve Bekçi'nin koruma süresi

> Durum: fikir aşaması, kod yok. 27 Ekim 2026. Fikir kullanıcının. Bu iki kural oyunun kuralıdır (basketboldaki 3 saniye kuralı gibi), yapay zekâ açığını kapatan yama değil.

## 1 · Kuyu alanı
- Her Kuyu'nun önünde bugünkü Bekçi alanından (yarıçap 9 br) **daha büyük** bir alan.
- Alan **girilemez değil**. Ama **Bekçi dışındaki her oyuncu** için (hücum ya da savunma, fark etmez) alanda kesintisiz kalma süresinin bir **sınırı** var.
- Bekçi alanın sınırsız sahibi.

### Ne kazandırır
- **Kuyu önüne yığılmanın önü kesilir** (enerji ve alan planındaki "Kuyu önü bloğu" riski). Derin blok oyuncularını alana girip çıkarak döndürmek zorunda kalır; döndürme anlarında boşluk açılır.
- **Hücumda zamanlama değer kazanır:** forvet alanda bekleyemez, doğru anda girer. Hücumda yer alma, koşu zamanlaması, Hız, Okuma'ya iş çıkar.
- **Kuyu önü pozisyonu artabilir:** şut anatomisinde asıl gollerin Kuyu önünden geldiği, ama oraya pek girilemediği görüldü; bugün şut sayısı da çok düşük.
- Bekçi'nin kimliği netleşir: alanın sahibi.

### Karar bekleyenler
- **Boyut ve biçim:** Bekçi alanının 1,5–2 katı mı; daire mi, dikdörtgen mi? 120 × 60 sahada çok büyük olursa oyunun yarısı bu kurala döner.
- **Süre:** kaç saniye; saat ne zaman sıfırlanır (alandan tamamen çıkınca mı, belli bir süre dışarıda kalınca mı); saat top kimdeyken işler?
- **Taşıyıcı:** muaf mı, yoksa saati şuta ya da pasa kadar mı donar? (Öneri: topla Kuyu'ya giden oyuncunun durması tuhaf olur; taşıyıcının saati topu bırakana kadar dursun.)
- **İhlalin cezası:** (a) top rakibe geçer (basit, sert); (b) **oyuncu söner**: süre dolunca topa dokunamaz ve alanı terk edene kadar alanı kapanır (ALAN'a özgü, ceza oyunun içinden). Öneri: (b).

## 2 · Bekçi'nin koruma süresi
- Top Bekçi'nin kontrolüne geçtiğinde belli bir süre (öneri 2–3 sn) rakipler Bekçi'ye **baskı yapamaz**.
- **"Baskı yapamaz" ne demek:** öneri, **topa giremez** (ikiliye girmez, topu çalamaz) ama **pas yollarını kapatabilir, adam tutabilir**. Bu, pres yapan takıma bir ikilem verir: Bekçi'ye basamıyorsan çıkışını kapat. (Diğer seçenek, belli bir mesafeye yaklaşamamak; daha basit, ama savunmanın yerleşmesini de değiştirir.)
- Bekçi'den başlayan oyun kurma güvenilir olur; Bekçi'nin topu kaptıktan hemen sonra kaybetmesi azalır.

### Bekçi'nin topu tutma süresi: yok (kullanıcı kararı)
- Koruma süresi bitince Bekçi pres yer; topu uzun tutmak zaten bedelli.
- Oyunun kilitlenmesi (maç 380418: Bekçi topla bekliyor, dibindeki rakip girmiyor, her tik yeniden karar) **ayrı bir bug'dır** ve kökünden düzeltilmelidir: aynı kilit dibinde rakip olan herhangi bir taşıyıcıda da olabilir. Kuralla örtülmez. Bkz. `lab/maclar/380418/ANALIZ.md`.

## Motorda (motor oturumu için not)
- Alanın sayacı oyuncu başına; kafa sayacı bilmeli (kafa = maç): sayacı dolmak üzere olan oyuncu planını ona göre yapar, yoksa sürekli ihlal olur.
- Bugünkü Bekçi alanı kuralları (Bekçi dışında kimsenin Kuyu alanına girememesi; Bekçi'nin Çekirdeğine menzil yetişmemesi gibi istisnalar) yeni alanla birlikte gözden geçirilmeli.
- Koruma süresi: savunmanın pres ve ikili kararları (`shape.js` pres seçimi, `match.js` ikili) Bekçi'nin topu alma anını ve süreyi bilmeli; kafa da.

## Ölçüm (test istenince)
- Alana giriş sıklığı, alanda geçen süre, ihlal sayısı; alandan atılan şut ve gol; Kuyu önündeki savunmacı sayısı.
- Bekçi: topu aldıktan sonraki kayıplar, oyun kurmanın süresi, Bekçi'den başlayan ataklar.
- Tarzlar: derin blok Dikine'ye karşı yeniden (alan derin bloğu zayıflatırsa Dikine daha da güçlenebilir); pres tarzının Bekçi'ye karşı ikilemi.
