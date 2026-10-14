# ALAN · Sıradaki paket: Öneri B ve C birlikte (çalışma düzeni güncellemesi)

Claude Code'dan Design'a. Kullanıcı onayladı.

## Düzen
Kontrol paketi 16 maçta ~50 dk sürüyor; her öneriyi ayrı turda ölçmek çok uzun. Yeni düzen:
1. **Bir pakette birden çok değişiklik olabilir, ama her biri kendi bayrağıyla** (`self.ALAN_..._ESKI = true` → o değişiklik kapalı, öncekiyle aynı). Bayraklar birbirinden bağımsız açılıp kapanabilmeli.
2. Günlükte her değişiklik için "kontrol tablosunda beklenen değişim" satırları ayrı yazılsın.
3. Claude Code paketi bir kez ölçer (16 maç). Bir satır ⚠ olursa, yalnız şüpheli değişikliğin bayrağı kapatılıp 8 maç daha oynatılır (~25 dk); sebep ayrılır.
4. Hiçbir satır kötüleşmezse hepsi tek ölçümle geçer.

zip59 (Öneri A) şu an ölçülüyor; sonucu ayrı not olarak gelecek. A'da ⚠ çıkarsa B + C paketine A'nın düzeltmesi de (kendi bayrağıyla) eklenebilir.

## Paket içeriği
Ayrıntılar DESIGN-NOT-zip58-degerlendirme.md, bölüm 5 ve 6.

**B · Savunma: görevler tek elden** (bayrak örn. `ALAN_ATAMA_ESKI`)
- Rakipler tehlikesine göre sıralanır (taşıyıcı ilk); sırayla her rakibe ona en çabuk varabilecek boştaki savunmacı atanır. Bir rakibe bir kişi; atanmayanlar boşluk ya da arkada güvence.
- İkinci presçi ancak bütün tehlikeli rakiplere birer kişi atandıktan sonra.
- Taktik ayarları maliyete girer (adam adama → kalıcı eşleşme; bölge → yalnız bölgesindeki rakip; pres şiddeti → ikinci presçi eşiği). Atama değişimine eşik (salınım olmasın).
- Beklenen: boş adam + çift adam anı ≤ 5 / dk; boş hücumcu ≤ %40; iki kişi "arkada" ≈ 0; taşıyıcıya baskı yok ≤ %14; top kazanma ve yenen sayı kötüleşmesin; duran savunmacı ve salınım artmasın.

**C1 · Bekleme kalibrasyonu için kayıt** (motor davranışı değişmez, bayrak gerekmez)
- Karar mesajına "tut" seçeneklerinin kopyadaki sonucu eklensin: kopyada Çekirdek kaybedildi mi, kaçıncı tikte, değer. Claude Code bununla aklın beklemede gördüğü kaybı gerçek kayıpla karşılaştırır ("tut"tan sonra kayıp bugün %32).

**C2 · Bekleme oynatması yalnız gerektiğinde** (bayrak örn. `ALAN_BEKLE_HEP`)
- Kopya oynatması yalnız "tut" seçeneğinin en iyi seçeneğe yakın olduğu ya da baskı olan anlarda; açıkça kötü olduğu anlarda (önü bomboş) atlanır.
- Beklenen: hesap ms/tik zip50 + %5'e insin (bugün +%16); diğer satırlar değişmesin.

**C3 · Varsayılan geçiş "eşitsek de git"** (bayrak örn. `ALAN_GECIS_ESKI`)
- Varsayılan `kazaninca` "sayıca üstünsek" → "eşitsek de". Kullanıcı onayladı.
- Beklenen: kontrada önü boşken duran ≤ 0,1 / dk; top kazanınca ilk 3 sn'de gönderme ↑; geçiş "git" süresi ↑; yenen kontra sayısı izlenecek.

C1 sonucu gelmeden "tut"un değerine (`−lossAt` ya da kopya) dokunulmasın; önce kalibrasyon.
