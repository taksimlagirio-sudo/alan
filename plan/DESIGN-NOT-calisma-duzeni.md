# ALAN · Design'a: yeni çalışma düzeni

Kullanıcının kararı: **Bundan sonra Design yalnız tasarım verir; oyun motoruna karışmaz.** Oyun motorunu (`alan-v5/*.js`) Claude Code yönetir.

**Senden beklenen:**
- Paketlerde sayfalar ve `support.js` (görünüm, ekranlar, etkileşim).
- Motor dosyalarını göndermene gerek yok; gönderirsen kullanılmaz.
- Sayfanın motordan yeni bir şeye ihtiyacı varsa (yeni bir veri alanı, yeni bir worker mesajı), notta yaz; motora Claude Code ekler.

**Sitedeki motor bugün:** zip78. Zip71'den bu yana neler değiştiği `DESIGN-NOT-zip78-sitede.md`'de; sayfaların göstereceği yeni şeyler için fikir verebilir. Örnekler:
- Kaleci tutma ve çelme Hakimiyet'e bağlandı.
- Kafadaki gönderme denemesi artık maçın kendisi.

**Canlı sayfada bir değişiklik var, korunmalı.** Maç, oynanacak devrenin kalanının **%30**'u hesaplanınca başlıyor (önce %20). Yeni sayfa sürümünde de bu oranı kullan:

`this.kareler.length - this.t >= .3 * (hedef - this.t)`

Bu oran oynanışı etkilediği için Claude Code her pakette kontrol edecek.

**Neden:** Motorun hesabı tarayıcıda izlemeden yavaş kalabiliyor. Daha büyük başlangıç payı, maçın ortasında "Hesap yetişiyor" beklemesini azaltıyor.
