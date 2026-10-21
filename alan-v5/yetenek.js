// ALAN v5 · yetenek sistemi (Yetenek Planı, 21 Ekim 2026). core.js'ten önce yüklenir; ALAN_OZ'u (core.js) yalnız çağrıldığında kullanır.
/* v5 · Aşama 1, altyapı: 34 özellik, eski kayıttan yeni özelliğe eşleme, mekanik tablosu ve ALAN_Y(oyuncu, mekanik). Bir mekaniğin değeri bileşimindeki özelliklerin ağırlıklı ortalaması (ağırlıkların toplamı 100): bütün bileşenler 10 iken tam 10, motorun bugünkü katsayıları aynen çalışır. Bu aşamada hiçbir mekanik ALAN_Y'yi okumaz, motor bugünkü özelliklerini okur; davranış değişmez. Sonraki her aşama kendi mekaniklerini kendi bayrağıyla ALAN_Y'ye bağlar: ALAN_YET.acik(n) aşama n açık mı (self.ALAN_YETENEK_ESKI hepsini, self.ALAN_YET_<AŞAMA>_ESKI yalnız o aşamayı kapatır; kapalıyken eski yol aynen çalışır). Aşamalar: 2 durum, 3 teknik ve vuruş, 4 fizik ve ikili, 5 zihin ve kafa, 6 topsuz oyun, 7 ALAN dinamikleri, 8 Bekçi, 9 kadrolar ve ekranlar */
(function () {
  const GRUP = { fiz: 'Fiziksel', tek: 'Teknik', alan: 'ALAN', zih: 'Zihinsel', yer: 'Yer alma', sav: 'Savunma', bek: 'Bekçi' };
  /* özellik anahtarı → [ad, grup]. Bugün de var olanlar (hiz, ceviklik, guc, uzanma, surme, pas, alan, yukleme, tepki, sogukkanlilik, kesme, refleks, yerlesim, hakimiyet) aynı anahtarla okunur */
  const OZ = { hiz: ['Hız', 'fiz'], patlayicilik: ['Patlayıcılık', 'fiz'], ceviklik: ['Çeviklik', 'fiz'], guc: ['Güç', 'fiz'], denge: ['Denge', 'fiz'], govde: ['Gövde', 'fiz'], uzanma: ['Uzanma', 'fiz'], dayaniklilik: ['Dayanıklılık', 'fiz'], toparlanma: ['Toparlanma', 'fiz'], ilkKontrol: ['İlk kontrol', 'tek'], teknik: ['Teknik', 'tek'], surme: ['Sürme', 'tek'], pas: ['Pas', 'tek'], bitiricilik: ['Bitiricilik', 'tek'], uzakSut: ['Uzaktan şut', 'tek'], aldatma: ['Aldatma', 'tek'], alan: ['Alan gücü', 'alan'], alanKontrol: ['Alan kontrolü', 'alan'], yukleme: ['Yükleme', 'alan'], tepki: ['Tepki', 'zih'], okuma: ['Okuma', 'zih'], vizyon: ['Vizyon', 'zih'], yaraticilik: ['Yaratıcılık', 'zih'], karar: ['Karar', 'zih'], sogukkanlilik: ['Soğukkanlılık', 'zih'], caliskanlik: ['Çalışkanlık', 'zih'], hucumYer: ['Hücumda yer alma', 'yer'], savunmaYer: ['Savunmada yer alma', 'yer'], kesme: ['Kesme', 'sav'], markaj: ['Markaj', 'sav'], refleks: ['Refleks', 'bek'], yerlesim: ['Yerleşim', 'bek'], hakimiyet: ['Alan hakimiyeti', 'bek'], birebir: ['Birebir', 'bek'] };
  /* eski kayıttan yeni özellik: kayıtta yoksa bugün o işi gören özellikten (bugünkü 19'lu kayıt; daha eski 8'li kayıt ALAN_OZ_ESKI ile). Gövde ve Çalışkanlık bugün yok: nötr 10. Eşlemesi yazılmayan aynı anahtardan (kayıtta yoksa 10). Kayıt yeni biçimde yazılınca (Aşama 9) motorun eski anahtar okumaları için ters eşleme gerekir */
  const o = (a, k) => self.ALAN_OZ({ a }, k), ESLE = { patlayicilik: a => o(a, 'hiz'), denge: a => (o(a, 'guc') + o(a, 'ceviklik')) / 2, govde: () => 10, dayaniklilik: a => o(a, 'enerji'), toparlanma: a => o(a, 'enerji'), ilkKontrol: a => o(a, 'kontrol'), teknik: a => (o(a, 'surme') + o(a, 'kontrol')) / 2, bitiricilik: a => o(a, 'gonderme'), uzakSut: a => o(a, 'gonderme'), aldatma: a => (o(a, 'surme') + (o(a, 'pas') + o(a, 'gorus')) / 2) / 2, alanKontrol: a => o(a, 'alan'), okuma: a => (o(a, 'tepki') + o(a, 'gorus')) / 2, vizyon: a => o(a, 'gorus'), yaraticilik: a => o(a, 'gorus'), karar: a => o(a, 'gorus'), caliskanlik: () => 10, hucumYer: a => o(a, 'yeralma'), savunmaYer: a => o(a, 'yeralma'), markaj: a => o(a, 'yeralma'), birebir: a => (o(a, 'refleks') + o(a, 'yerlesim')) / 2 };
  const yeni = (a, k) => { const v = a[k]; return v != null ? v : ESLE[k] ? ESLE[k](a) : o(a, k); };
  /* mekanik → [ad, aile, aşama, bugün okuduğu özellik(ler) (null: sabit, kural ya da motorda yok), bileşim [[özellik, ağırlık]], durumlar (E enerji, B baskı, D denge, H hareket, Hz hazırlık; Aşama 2)]. Aileler: A koşu ve gövde, B enerji, C alan, D Çekirdek, E top tekniği, F top alma ve kesme, G vuruş, H algı ve karar, I fake, J ikili, K topsuz oyun, L Bekçi */
  const MEK = {
    kosuHizi: ['Temel koşu hızı', 'A', 4, ['hiz'], [['hiz', 100]], ['E', 'D']],
    sprintKati: ['Sprint katı', 'A', 4, ['hiz'], [['hiz', 100]], ['E']],
    hizlanma: ['Hızlanma', 'A', 4, ['ceviklik'], [['patlayicilik', 60], ['ceviklik', 25], ['guc', 15]], []],
    donus: ['Fren ve yön değiştirme', 'A', 4, ['ceviklik'], [['ceviklik', 60], ['denge', 25], ['patlayicilik', 15]], ['D']],
    topluKosu: ['Topla koşu hızı', 'A', 4, ['surme'], [['surme', 60], ['teknik', 25], ['ceviklik', 15]], []],
    itilmePayi: ['Gövde ayırma (itilme payı)', 'A', 4, ['guc'], [['govde', 60], ['guc', 40]], []],
    barBoy: ['Bar büyüklüğü', 'B', 7, ['enerji'], [['dayaniklilik', 100]], []],
    barDolum: ['Barın dolması', 'B', 7, ['enerji'], [['toparlanma', 70], ['dayaniklilik', 30]], []],
    sprintBedel: ['Sprintin bedeli', 'B', 4, ['enerji'], [['dayaniklilik', 100]], []],
    ivmeBedel: ['İvmelenmenin bedeli', 'B', 4, ['ceviklik'], [['ceviklik', 60], ['patlayicilik', 40]], []],
    surmeBedel: ['Topla sürmenin bedeli', 'B', 4, ['surme'], [['surme', 60], ['teknik', 40]], []],
    dondurmeBedel: ['Topu döndürmenin bedeli', 'B', 4, ['kontrol', 'surme'], [['teknik', 70], ['ilkKontrol', 30]], []],
    fakeBedel: ['Fake ve fiskenin bedeli', 'B', 4, ['kontrol', 'ceviklik', 'surme'], [['teknik', 50], ['aldatma', 50]], []],
    temasBedel: ['Gövde temasının bedeli', 'B', 4, ['guc', 'uzanma', 'kesme'], [['guc', 50], ['denge', 30], ['govde', 20]], []],
    harcamaIstegi: ['Enerji harcama isteği', 'B', 6, null, [['caliskanlik', 100]], []],
    alanMenzil: ['Alanın menzili', 'C', 7, ['alan'], [['alan', 100]], ['E']],
    alanKuvvet: ['Alanın kuvveti', 'C', 7, ['alan'], [['alan', 70], ['alanKontrol', 30]], []],
    alanAcma: ['Alanı açma zamanı', 'C', 7, ['tepki'], [['okuma', 60], ['tepki', 40]], []],
    alanYon: ['Alanın yönü (hızlandır / yavaşlat / bırak)', 'C', 7, null, [['okuma', 60], ['karar', 40]], ['B']],
    alanBedel: ['Alanın enerji bedeli', 'C', 7, null, [['alanKontrol', 70], ['dayaniklilik', 30]], []],
    alanBicim: ['Alanın biçimi', 'C', 7, null, [['alanKontrol', 100]], []],
    bekciAlan: ['Bekçi alanı', 'C', 7, ['hakimiyet'], [['hakimiyet', 60], ['alan', 40]], []],
    sarjHiz: ['Şarj aktarım hızı', 'D', 7, ['yukleme'], [['yukleme', 80], ['teknik', 20]], ['B']],
    sarjKosuPay: ['Koşarken aktarılan pay', 'D', 7, ['yukleme'], [['yukleme', 60], ['surme', 20], ['denge', 20]], []],
    sarjBedel: ['Şarjın bardan bedeli', 'D', 7, ['yukleme'], [['yukleme', 70], ['dayaniklilik', 30]], []],
    sarjKoru: ['Alışta şarjı korumak', 'D', 7, null, [['ilkKontrol', 60], ['yukleme', 40]], ['B', 'Hz']],
    vurusTaban: ['Vuruşun taban enerjisi', 'D', 7, null, [['teknik', 50], ['guc', 50]], []],
    sutGucu: ['Şutun gücü', 'D', 3, ['gonderme', 'guc'], [['guc', 45], ['uzakSut', 35], ['teknik', 20]], ['D']],
    pasSert: ['Pasın en sert hali', 'D', 3, ['guc'], [['guc', 70], ['teknik', 30]], []],
    savurmaHiz: ['Topu döndürme hızı (savurma)', 'E', 3, ['surme'], [['teknik', 50], ['ceviklik', 30], ['surme', 20]], ['D']],
    birakma: ['Bırakma zamanlaması', 'E', 3, ['pas', 'gonderme'], [['teknik', 60], ['pas', 40]], ['B', 'D']],
    fiskeHiz: ['Fiske hızı', 'E', 3, null, [['teknik', 60], ['ceviklik', 40]], []],
    fiskeKacir: ['Fiskede topu kaçırmak', 'E', 3, ['surme'], [['teknik', 50], ['surme', 30], ['sogukkanlilik', 20]], ['B']],
    dolas: ['Dolaş (topu rakipten saklayarak çevirmek)', 'E', 3, ['kontrol', 'surme'], [['teknik', 50], ['denge', 30], ['guc', 20]], ['B']],
    kalkan: ['Gövdeyle koruma (kalkan)', 'E', 3, ['tepki'], [['okuma', 50], ['guc', 25], ['denge', 25]], []],
    kenarCalim: ['Kenardan çalım', 'E', 4, ['surme'], [['teknik', 40], ['patlayicilik', 30], ['surme', 30]], []],
    yenidenDokun: ['Topa yeniden dokunma beklemesi', 'E', 3, null, [['ilkKontrol', 50], ['teknik', 50]], []],
    kontrolKap: ['İlk kontrol kapasitesi', 'F', 3, ['kontrol'], [['ilkKontrol', 65], ['teknik', 20], ['denge', 15]], ['B', 'Hz', 'H', 'D']],
    kesmeKap: ['Pas arası kesme kapasitesi', 'F', 3, ['kesme'], [['kesme', 60], ['okuma', 25], ['uzanma', 15]], ['Hz', 'D']],
    uzanmaMenzil: ['Uzanma menzili', 'F', 3, ['uzanma'], [['uzanma', 70], ['ceviklik', 30]], ['D']],
    uzanmaBasla: ['Uzanmanın büyümeye başlaması', 'F', 3, ['tepki'], [['okuma', 50], ['tepki', 50]], []],
    hazirlik: ['Topa hazırlanma (alıştaki hazırlık süresi)', 'F', 3, ['tepki'], [['okuma', 50], ['tepki', 50]], []],
    sizma: ['Sert topun bacaktan sızması', 'F', 3, ['kesme'], [['kesme', 50], ['denge', 30], ['govde', 20]], []],
    tekDokunusDusun: ['Tek dokunuşu düşünmek', 'F', 5, ['gorus'], [['vizyon', 60], ['karar', 40]], []],
    tekDokunusIsabet: ['Tek dokunuşun isabeti', 'F', 3, ['pas'], [['pas', 40], ['teknik', 40], ['ilkKontrol', 20]], ['B']],
    onHareket: ['Alıcının ön hareketi', 'F', 5, ['tepki', 'gorus'], [['vizyon', 50], ['okuma', 30], ['tepki', 20]], []],
    pasYon: ['Pasın yön isabeti', 'G', 3, ['pas'], [['pas', 70], ['teknik', 15], ['vizyon', 15]], ['B', 'D', 'H']],
    pasHiz: ['Pasın hız isabeti', 'G', 3, ['guc'], [['pas', 50], ['guc', 30], ['teknik', 20]], ['B']],
    sutYon: ['Şutun yön isabeti', 'G', 3, ['gonderme'], [['bitiricilik', 40], ['uzakSut', 35], ['teknik', 15], ['sogukkanlilik', 10]], ['B', 'D', 'H']],
    plase: ['Plase', 'G', 3, ['gonderme'], [['bitiricilik', 60], ['teknik', 40]], ['B', 'D', 'H']],
    sutHedefSec: ['Şut hedefi ve dönüş yönü seçimi', 'G', 5, null, [['bitiricilik', 50], ['vizyon', 30], ['karar', 20]], ['B']],
    kenarPasDusun: ['Kenardan pası düşünmek', 'G', 5, ['gorus'], [['yaraticilik', 60], ['vizyon', 40]], []],
    kenarPasIsabet: ['Kenardan pasın isabeti', 'G', 3, ['pas'], [['pas', 60], ['vizyon', 25], ['teknik', 15]], ['B', 'D', 'H']],
    tehlike: ['Tehlike tahmini (kimi tutmalı)', 'G', 3, ['gonderme', 'hiz'], [['bitiricilik', 40], ['uzakSut', 30], ['hiz', 30]], []],
    algiHata: ['Algı hatası (rakip ve top nerede)', 'H', 5, ['gorus'], [['vizyon', 60], ['okuma', 40]], []],
    algiGecikme: ['Algı gecikmesi', 'H', 5, ['tepki'], [['tepki', 60], ['okuma', 40]], []],
    kararSure: ['Topu alınca karar süresi', 'H', 5, ['tepki'], [['karar', 50], ['tepki', 30], ['sogukkanlilik', 20]], ['B']],
    gorulenArkadas: ['Görülen arkadaşlar ve pas menzili', 'H', 5, null, [['vizyon', 100]], []],
    secenekTur: ['Düşünülen seçenek türleri', 'H', 5, null, [['yaraticilik', 80], ['vizyon', 20]], []],
    tartiIsabet: ['Seçenekleri tartmanın isabeti', 'H', 5, null, [['karar', 80], ['sogukkanlilik', 20]], ['B']],
    ileriBakis: ['İleri bakış', 'H', 5, ['gorus'], [['karar', 60], ['vizyon', 40]], []],
    niyetSez: ['Rakibin hareketinden niyet sezmek', 'H', 6, null, [['okuma', 100]], []],
    ikiliKarar: ['İkiliye girme kararı ve okunması', 'H', 5, ['tepki', 'gorus'], [['okuma', 50], ['tepki', 30], ['karar', 20]], []],
    enerjiTahmin: ['Rakibin enerjisini tahmin', 'H', 5, ['gorus'], [['okuma', 100]], []],
    fakeInan: ['Fake\'in inandırıcılığı', 'I', 5, ['surme', 'pas', 'gorus'], [['aldatma', 60], ['teknik', 20], ['pas', 20]], []],
    fakeGovde: ['Gövde fake\'inin inandırıcılığı', 'I', 5, ['surme'], [['aldatma', 60], ['teknik', 20], ['surme', 20]], []],
    fakeKan: ['Fake\'e kanmak', 'I', 5, ['tepki'], [['okuma', 70], ['tepki', 30]], []],
    fakeSure: ['Fake süresi', 'I', 5, ['ceviklik'], [['ceviklik', 50], ['aldatma', 30], ['teknik', 20]], []],
    presFake: ['Presçinin fake\'i', 'I', 5, ['gorus'], [['aldatma', 60], ['savunmaYer', 40]], []],
    kutle: ['Kütle', 'J', 4, ['guc'], [['govde', 60], ['guc', 40]], []],
    dengeDirenc: ['Dengeye direnç', 'J', 4, ['guc', 'ceviklik'], [['denge', 50], ['guc', 30], ['ceviklik', 20]], ['E']],
    dengeDolum: ['Dengenin geri gelmesi', 'J', 4, null, [['denge', 60], ['toparlanma', 20], ['ceviklik', 20]], []],
    sendeleme: ['Sendeleme süresi', 'J', 4, null, [['denge', 100]], []],
    durtme: ['Dürtme (top açıktayken)', 'J', 4, ['kesme', 'surme'], [['kesme', 50], ['okuma', 25], ['uzanma', 25]], ['D']],
    durtmeKoru: ['Dürtmeye karşı korumak (taşıyıcı)', 'J', 4, ['surme'], [['teknik', 40], ['surme', 30], ['denge', 30]], []],
    omuz: ['Omuz itişi', 'J', 4, ['guc'], [['guc', 50], ['govde', 30], ['denge', 20]], []],
    hamle: ['Hamle süresi ve adanma', 'J', 4, ['ceviklik'], [['ceviklik', 50], ['denge', 30], ['karar', 20]], []],
    sersem: ['İkili sonrası sersemleme', 'J', 4, null, [['denge', 70], ['ceviklik', 30]], []],
    bolge: ['Bölge, kapatma, güvence', 'K', 6, ['yeralma'], [['savunmaYer', 65], ['okuma', 35]], []],
    adamDevri: ['Adam devri', 'K', 6, ['yeralma'], [['savunmaYer', 40], ['okuma', 30], ['markaj', 15], ['karar', 15]], []],
    adamYetis: ['Adamının koşusuna yetişmek', 'K', 6, ['yeralma'], [['markaj', 50], ['okuma', 30], ['patlayicilik', 20]], []],
    markajSadakat: ['Markajın sadakati', 'K', 6, ['yeralma'], [['markaj', 70], ['caliskanlik', 30]], []],
    markajKurtul: ['Markajdan kurtulma (gel-dön, kör taraf)', 'K', 6, ['yeralma'], [['hucumYer', 50], ['okuma', 30], ['patlayicilik', 20]], []],
    bosluk: ['Boşluk bulma, hatlar arası, genişlik, bindirme', 'K', 6, ['yeralma'], [['hucumYer', 60], ['vizyon', 25], ['yaraticilik', 15]], []],
    kosuZaman: ['Koşu zamanlaması (arkaya, üçüncü adam, ver-kaç)', 'K', 6, ['yeralma'], [['hucumYer', 50], ['okuma', 30], ['karar', 20]], []],
    bosFark: ['Açılan boşluğu fark etmek', 'K', 6, ['yeralma'], [['okuma', 60], ['vizyon', 40]], []],
    presBasla: ['Prese koşmaya başlamak', 'K', 6, ['tepki', 'hiz'], [['tepki', 40], ['okuma', 30], ['caliskanlik', 30]], []],
    presYaklas: ['Presin yaklaşımı (açı kapatma, oyala)', 'K', 6, null, [['savunmaYer', 50], ['okuma', 30], ['karar', 20]], []],
    geriKosu: ['Geri koşuda ısrar', 'K', 6, null, [['caliskanlik', 70], ['savunmaYer', 30]], []],
    bekRefleks: ['Refleks', 'L', 8, ['refleks'], [['refleks', 75], ['okuma', 25]], []],
    dalis: ['Dalışın hızı ve boyu', 'L', 8, ['hiz'], [['patlayicilik', 40], ['uzanma', 40], ['refleks', 20]], ['E']],
    yanaKayma: ['Yana kayma', 'L', 8, ['ceviklik'], [['ceviklik', 50], ['patlayicilik', 30], ['yerlesim', 20]], []],
    aciDaralt: ['Açı daraltma ve süpürücü yeri', 'L', 8, ['yerlesim'], [['yerlesim', 60], ['okuma', 40]], []],
    cikis: ['Kalesinden çıkış', 'L', 8, ['yerlesim'], [['birebir', 50], ['yerlesim', 30], ['karar', 20]], []],
    karsiKarsiya: ['Karşı karşıya', 'L', 8, null, [['birebir', 60], ['sogukkanlilik', 20], ['okuma', 20]], ['B']],
    tutCel: ['Tutmak ya da çelmek', 'L', 8, ['hakimiyet'], [['hakimiyet', 60], ['ilkKontrol', 20], ['denge', 20]], []],
    eldeTut: ['Topu elde tutma süresi', 'L', 8, null, [['karar', 100]], []],
  };
  const BAYRAK = { 2: 'DURUM', 3: 'TEKNIK', 4: 'FIZIK', 5: 'ZIHIN', 6: 'TOPSUZ', 7: 'ALAN', 8: 'BEKCI', 9: 'KADRO' }, BAD = {}, OZK = Object.keys(OZ); for (const n in BAYRAK) BAD[n] = 'ALAN_YET_' + BAYRAK[n] + '_ESKI'; /* bayrak adları bir kez: sıcak döngüde dize birleştirmeden okunur */
  /* önbellek: kaydın bütün özellikleri (oz) ve bileşikleri ilk okumada bir kez hesaplanır, kayıt başına sayılamaz __Y alanında (maç dosyasına, kopyalara ve işçiye geçmez; kafanın kopyaları aynı kaydı paylaşır, önbellek de ortak). Hep aynı biçimde nesne: okuma hızlı. Özellikler maç içinde değişmez; kaydı yerinde değiştiren (rastgele özellik) ALAN_YET.yenile(a) çağırır */
  const IDS = Object.keys(MEK), BOS = () => ({ kosuHizi: 0, sprintKati: 0, hizlanma: 0, donus: 0, topluKosu: 0, itilmePayi: 0, barBoy: 0, barDolum: 0, sprintBedel: 0, ivmeBedel: 0, surmeBedel: 0, dondurmeBedel: 0, fakeBedel: 0, temasBedel: 0, harcamaIstegi: 0, alanMenzil: 0, alanKuvvet: 0, alanAcma: 0, alanYon: 0, alanBedel: 0, alanBicim: 0, bekciAlan: 0, sarjHiz: 0, sarjKosuPay: 0, sarjBedel: 0, sarjKoru: 0, vurusTaban: 0, sutGucu: 0, pasSert: 0, savurmaHiz: 0, birakma: 0, fiskeHiz: 0, fiskeKacir: 0, dolas: 0, kalkan: 0, kenarCalim: 0, yenidenDokun: 0, kontrolKap: 0, kesmeKap: 0, uzanmaMenzil: 0, uzanmaBasla: 0, hazirlik: 0, sizma: 0, tekDokunusDusun: 0, tekDokunusIsabet: 0, onHareket: 0, pasYon: 0, pasHiz: 0, sutYon: 0, plase: 0, sutHedefSec: 0, kenarPasDusun: 0, kenarPasIsabet: 0, tehlike: 0, algiHata: 0, algiGecikme: 0, kararSure: 0, gorulenArkadas: 0, secenekTur: 0, tartiIsabet: 0, ileriBakis: 0, niyetSez: 0, ikiliKarar: 0, enerjiTahmin: 0, fakeInan: 0, fakeGovde: 0, fakeKan: 0, fakeSure: 0, presFake: 0, kutle: 0, dengeDirenc: 0, dengeDolum: 0, sendeleme: 0, durtme: 0, durtmeKoru: 0, omuz: 0, hamle: 0, sersem: 0, bolge: 0, adamDevri: 0, adamYetis: 0, markajSadakat: 0, markajKurtul: 0, bosluk: 0, kosuZaman: 0, bosFark: 0, presBasla: 0, presYaklas: 0, geriKosu: 0, bekRefleks: 0, dalis: 0, yanaKayma: 0, aciDaralt: 0, cikis: 0, karsiKarsiya: 0, tutCel: 0, eldeTut: 0, oz: null }), yap = a => { const c = BOS(), z = {}; for (const k of OZK) z[k] = yeni(a, k); c.oz = z; for (const k of IDS) { let s = 0; for (const [o, w] of MEK[k][4]) s += w * z[o]; c[k] = s / 100; } if (Object.isExtensible(a)) Object.defineProperty(a, '__Y', { value: c, configurable: true }); return c; };
  self.ALAN_Y = (p, k) => { const a = p && p.a; if (!a) return 10; const v = (a.__Y || yap(a))[k]; if (v === undefined) throw new Error('ALAN_Y: tabloda olmayan mekanik ' + k); return v; };
  const oz = (p, k) => { const a = p && p.a; return a ? (a.__Y || yap(a)).oz[k] : 10; };
  /* şutun isabeti (sutYon): Bitiricilik ile Uzaktan şut'un payı mesafeyle kayar, yakında (12 br'ye kadar) hepsi Bitiricilik, uzakta (30 br'den) hepsi Uzaktan şut; tablodaki 40 / 35 orta mesafenin payı. Teknik ve Soğukkanlılık payı sabit */
  const SUT = { yakin: 12, uzak: 30 }, sutY = (p, d) => { const a = p && p.a; if (!a) return 10; const z = (a.__Y || yap(a)).oz, b = Math.max(0, Math.min(1, (SUT.uzak - d) / (SUT.uzak - SUT.yakin))); let s = 0, bu = 0; for (const [o, w] of MEK.sutYon[4]) { if (o === 'bitiricilik' || o === 'uzakSut') bu += w; else s += w * z[o]; } return (s + bu * (b * z.bitiricilik + (1 - b) * z.uzakSut)) / 100; };
  /* zar: belirlenimli zar (0–1), oyuncu ve olayla; maçın zar dizisini değiştirmez, aynı olayda hep aynı (seçim hatası birkaç tik kalıcı, aklına gelme alış başına bir kez). kademe: bugünkü aç/kapa eşiğinin çevresinde 4 puanlık rampa (eşik − 2'de 0, eşik + 2'de 1) */
  const zar = (a, b, c) => { const s = Math.sin(a * 12.9898 + b * 78.233 + c * 37.719) * 43758.5453; return s - Math.floor(s); }, kademe = (v, esik) => Math.max(0, Math.min(1, (v - esik + 2) / 4));
  const agirlik = (k, o) => { const r = MEK[k][4].find(x => x[0] === o); return r ? r[1] / 100 : 0; }; /* bir özelliğin mekanikteki payı (0–1) */
  self.ALAN_YET = { GRUP, OZ, ESLE, MEK, BAYRAK, SUT, oz, sutY, agirlik, zar, kademe, acik: n => !self.ALAN_YETENEK_ESKI && !self[BAD[n]], canli: n => !self.ALAN_YETENEK_ESKI && !self[BAD[n]] && !self.ALAN_YET_CANLI_ESKI, verim: (p, k) => self.ALAN_EN_VERIM_ESKI ? 1 : Math.max(.5, Math.min(1.4, 1 - (self.ALAN_Y(p, k) - 10) * .04)), yenile: a => { if (a) delete a.__Y; } }; /* canli(n): aşama n'de sabitten özelliğe geçen mekanikler (3: fiske hızı, yeniden dokunma beklemesi; 4: hızlanma, dengenin geri gelmesi, sendeleme süresi) ayrıca kapatılabilir: self.ALAN_YET_CANLI_ESKI. verim: işin enerji bedelini bileşimi ucuzlatır (10 → 1, 20 → 0,6, 0 → 1,4; maçın verim kuralı) */
  /* v5 · Aşama 4, hareket: fren ve dönüş çarpanı (AG) ile gidilen yönde hızlanma çarpanı (AH). Maçın ivme kuralı, kafanın varış modelleri ve yerleşimin geçiş tabloları aynı fonksiyondan okur. Eski yol: AG Çeviklik'ten (6 → ×0,8, 10 → ×1, 16 → ×1,3; 0,6–1,35), AH 1 (hızlanma sabit). Aşama 4: AG fren ve dönüş bileşiminden (Çeviklik 60, Denge 25, Patlayıcılık 15), dengesizken düşer (maçın dengesizken yavaşlama ölçüsü; Bekçi'de yok; Deney: self.ALAN_YET_DURUM_ESKI); AH hızlanma bileşiminden aynı ölçekle (Patlayıcılık 60, Çeviklik 25, Güç 15; Deney: self.ALAN_YET_CANLI_ESKI). Deney hepsi: self.ALAN_YET_FIZIK_ESKI */
  self.ALAN_AG = p => { const F = !self.ALAN_YETENEK_ESKI && !self.ALAN_YET_FIZIK_ESKI, g = Math.max(.6, Math.min(1.35, 1 + ((F ? self.ALAN_Y(p, 'donus') : ((p.a && p.a.ceviklik) ?? 10)) - 10) * .05)); return F && !self.ALAN_YET_DURUM_ESKI && p.den != null && p.den < 1 && p.role !== 'Bekçi' ? g * self.ALAN_DURUM.dengeK(p.den) : g; };
  self.ALAN_AH = p => self.ALAN_YETENEK_ESKI || self.ALAN_YET_FIZIK_ESKI || self.ALAN_YET_CANLI_ESKI ? 1 : Math.max(.6, Math.min(1.35, 1 + (self.ALAN_Y(p, 'hizlanma') - 10) * .05));
  /* v5 · Aşama 2, durum katmanı: anlık durumun (B baskı, D denge, H hareket) vuruşa ve alışa etkisi tek yerden; maç da kafa da bunu okur. Hata toplamalı (kullanıcı kararı, 21 Ekim 2026): hata puanı = beceri payı (20 − bileşik) + 10 × Σ katsayı × durum × yumuşatma. Beceri 10'da bugünkü çarpanla (1 + Σ) aynı; 20'lik oyuncu da baskıda, dengesizken, koşarken dağıtır. Katsayılar bugünkü kurallardan: pasta baskı 1,5; şutta baskı 0,6 ve hareket 0,4; denge baskıyla aynı (tam dengesiz = tam baskı), pasta hareket şuttaki gibi. Yumuşatan: baskıyı Soğukkanlılık, hareketi Teknik (bugünkü Soğukkanlılık kuralı: 10 → 1, 20 → 0,4, 0 → 1,6). Denge etkiyi değil durumun kendisini küçültür (dengeye direnç ve dolum, Aşama 4). Durumlar: B = min(1, üstündeki baskı), D = 1 − denge, H = hız / en yüksek hız */
  const DK = { pas: { B: 1.5, D: 1.5, H: .4 }, sut: { B: .6, D: .6, H: .4 }, alis: { B: 1, D: 1, H: 2 / 3 } }, YUM = { B: 'sogukkanlilik', H: 'teknik' }, yum = (p, d) => YUM[d] ? Math.max(.2, 1 - (oz(p, YUM[d]) - 10) * .06) : 1, har = p => Math.min(1, Math.hypot(p.vx || 0, p.vy || 0) / ((.17 + self.ALAN_HZ(p) * .006) * self.ALAN_SPR(p))); /* har: hareket durumu, hızın en yüksek hıza oranı (bugünkü şut kuralı) */
  self.ALAN_DURUM = { DK, YUM, yum,
    pay: (p, mek, tur, s) => { const k = DK[tur], du = MEK[mek][5]; let t = 0; for (const d of du) { const c = k[d], v = s[d]; if (c && v) t += c * v * yum(p, d); } return t; }, /* mekaniğin tablodaki durumları üzerinden toplam (Hz ve E burada yok: Hz alışın hazırlığında, E hızda) */
    alis: (p, mate, prK) => DK.alis.B * yum(p, 'B') * prK + DK.alis.D * (1 - (p.den ?? 1)) + (mate ? DK.alis.H * har(p) * yum(p, 'H') : 0), /* ilk kontrolde ve pas arası kesmede kapasiteden düşen pay (× capPr; prK: bugünkü baskı payı 0–1): baskı bugünkü gibi, Soğukkanlılık'la yumuşar; denge kaybı baskı kadar; koşarken alış (yalnız arkadaş, ilk kontrol) şuttaki hareket / baskı oranıyla (0,4 / 0,6) */
    har: p => har(p),
    dengeK: den => { const Y = (window.AlanMatch && window.AlanMatch.IK && window.AlanMatch.IK.yavas) || .35; return den == null || den >= Y ? 1 : .45 + .55 * Math.max(0, den) / Y; } /* dengesizken hareketin payı: maçın dengeHiz kuralı (denge yavaşlama eşiğinin altında hız en çok %55 düşer); uzanma, topu döndürme ve şutun gücü aynı ölçüde */
  };
})();
