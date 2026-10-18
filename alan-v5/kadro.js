// ALAN v5 · ortak kadro verisi (Liman ve rakipler). Menajer ve Taktik ekranları kullanır.
(function () {
/* v5 · özellik standardı (15 saha + 3 Bekçi + Taraf). Her özellik tek iş; bağlama tablosu alan-v5/OZELLIKLER.md. Veri aşağıda eski 8'li biçimde yazılı, dosyanın sonunda yeni anahtarlara çevrilir */
const OZG = [
  { ad: 'Fiziksel', k: [['hiz', 'Hız'], ['ceviklik', 'Çeviklik'], ['guc', 'Güç'], ['uzanma', 'Uzanma'], ['alan', 'Alan gücü']] },
  { ad: 'Topla', k: [['kontrol', 'Kontrol'], ['surme', 'Sürme'], ['pas', 'Pas'], ['gonderme', 'Gönderme'], ['yukleme', 'Yükleme']] },
  { ad: 'Zihinsel', k: [['tepki', 'Tepki'], ['gorus', 'Görüş'], ['yeralma', 'Yer alma'], ['sogukkanlilik', 'Soğukkanlılık']] },
  { ad: 'Savunma', k: [['kesme', 'Kesme']] }];
const OZB = { ad: 'Bekçi', k: [['refleks', 'Refleks'], ['yerlesim', 'Yerleşim'], ['hakimiyet', 'Alan hakimiyeti'], ['pas', 'Pas'], ['kontrol', 'Kontrol'], ['tepki', 'Tepki']] };
const OZ = OZG.flatMap(g => g.k), OZ_BEKCI = [['refleks', 'Refleks'], ['yerlesim', 'Yerleşim'], ['hakimiyet', 'Alan hakimiyeti']];
const TARAF = { aras: 'sag', deniz: 'sag', mert: 'sol', ece: 'iki', cansu: 'sol', kaan: 'sag', selin: 'sol', tuna: 'sag', ilgin: 'iki', bora: 'sag', yaz: 'sol' };
const a = (okuma, aktarim, tutus, kesme, yogunluk, hiz, cesaret, surme) => ({ okuma, aktarim, tutus, kesme, yogunluk, hiz, cesaret, surme });
const KADRO = [
  { id: 'aras', ad: 'Aras', tip: 'Bekçi', a: a(13, 11, 15, 10, 10, 10, 11, 8) },
  { id: 'levent', ad: 'Levent', tip: 'Bekçi', a: a(10, 9, 13, 9, 11, 12, 13, 8) },
  { id: 'deniz', ad: 'Deniz', tip: 'Kesici', a: a(13, 10, 11, 15, 13, 9, 11, 8) },
  { id: 'mert', ad: 'Mert', tip: 'Duvar', a: a(11, 9, 10, 13, 16, 8, 12, 7) },
  { id: 'ece', ad: 'Ece', tip: 'Kurucu', a: a(14, 15, 12, 10, 9, 10, 10, 11) },
  { id: 'cansu', ad: 'Cansu', tip: 'Okuyucu', a: a(15, 13, 11, 12, 10, 10, 9, 10) },
  { id: 'kaan', ad: 'Kaan', tip: 'Koşucu', a: a(10, 9, 10, 8, 9, 16, 12, 13) },
  { id: 'selin', ad: 'Selin', tip: 'Sürücü', a: a(11, 11, 11, 8, 9, 14, 13, 15) },
  { id: 'tuna', ad: 'Tuna', tip: 'Pivot', a: a(12, 13, 14, 9, 13, 9, 11, 10) },
  { id: 'ilgin', ad: 'Ilgın', tip: 'Çok yönlü', a: a(11, 11, 11, 12, 11, 13, 10, 11) },
  { id: 'bora', ad: 'Bora', tip: 'Cesur', a: a(10, 11, 10, 10, 11, 12, 15, 12) },
  { id: 'yaz', ad: 'Yaz', tip: 'Genç', a: a(9, 10, 10, 10, 10, 14, 11, 12) }
];
const RAKIP = [
  { ad: 'Demir Blok', tarz: 'Savunmacı · kontra', tac: { sistem: 'Alan', pres: 1, blok: 'Düşük', genislik: 'Dar', tempo: .4, risk: .3, kazaninca: 'Kontra' }, diz: { sav: '3-2-1', huc: '2-2-2' },
    roller: [{ h: 'Kurucu', s: 'Süpürücü' }, { h: 'Serbest', s: 'Süpürücü' }, { h: 'Serbest', s: 'Markajcı' }, { h: 'Serbest', s: 'Bölge' }, { h: 'Koşucu', s: 'Presçi' }, { h: 'Koşucu', s: 'Önde' }],
    notlar: ['Düşük blok, iki kişi hep arkada: Kuyu önü kalabalık', 'Topu kazanınca iki hızlı uç kontraya çıkar', 'Kesme ve Yoğunluk yüksek, Sürme zayıf: sabırlı pasla açılır'],
    oyuncular: [['Kaya', a(12, 9, 14, 10, 11, 9, 11, 7)], ['Bulut', a(11, 9, 10, 15, 14, 8, 11, 7)], ['Taş', a(10, 8, 10, 14, 16, 8, 12, 7)], ['Ayaz', a(13, 11, 10, 13, 12, 9, 10, 8)], ['Gökay', a(10, 10, 10, 12, 13, 10, 11, 8)], ['Rüzgâr', a(9, 9, 10, 8, 9, 16, 12, 12)], ['Şimşek', a(9, 10, 10, 8, 9, 15, 13, 12)]] },
  { ad: 'Fırtına', tarz: 'Hücumcu · ön alan', tac: { sistem: 'Adam adama', pres: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .8, risk: .8, kazaninca: 'Dengeli' }, diz: { sav: '2-2-2', huc: '2-1-3' },
    roller: [{ h: 'Serbest', s: 'Bölge' }, { h: 'Kurucu', s: 'Süpürücü' }, { h: 'Koşucu', s: 'Presçi' }, { h: 'Kanat', s: 'Presçi' }, { h: 'Kanat', s: 'Bölge' }, { h: 'Pivot', s: 'Presçi' }],
    notlar: ['Yüksek blok, iki presçi: arkası açık kalır', 'Adam adama: oyuncular yer değiştirdikçe dağılır', 'Hızlı ve cesur, Okuma düşük: uzun pasa ve boşluğa zayıf'],
    oyuncular: [['Tufan', a(9, 9, 12, 9, 10, 12, 14, 8)], ['Alp', a(9, 10, 10, 12, 11, 14, 13, 9)], ['Baran', a(9, 11, 10, 11, 10, 13, 14, 11)], ['Efe', a(10, 10, 10, 12, 12, 15, 13, 10)], ['Kartal', a(9, 12, 11, 9, 9, 15, 14, 14)], ['Yıldırım', a(9, 11, 10, 9, 9, 16, 13, 14)], ['Volkan', a(10, 12, 13, 9, 13, 13, 15, 11)]] },
  { ad: 'Saat Kulesi', tarz: 'Sabırlı · topa sahip', tac: { sistem: 'Alan', pres: 1, blok: 'Orta', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Yerleş' }, diz: { sav: '2-3-1', huc: '2-2-2' },
    roller: [{ h: 'Kurucu', s: 'Bölge' }, { h: 'Kurucu', s: 'Süpürücü' }, { h: 'Serbest', s: 'Bölge' }, { h: 'Serbest', s: 'Bölge' }, { h: 'Kanat', s: 'Presçi' }, { h: 'Pivot', s: 'Bölge' }],
    notlar: ['Düşük tempo, az risk: topu uzun tutar, rakibi üstüne çeker', 'Okuma ve Aktarım yüksek, Hız düşük: arkaya koşulara geç döner', 'Kazanınca yerleşir: kontraya az çıkar, yüksek pres onu zorlar'],
    oyuncular: [['Zaman', a(14, 12, 13, 10, 10, 9, 10, 8)], ['Akrep', a(14, 14, 11, 12, 10, 9, 9, 10)], ['Yelkovan', a(15, 15, 11, 11, 10, 9, 9, 11)], ['Sarkaç', a(14, 15, 12, 10, 10, 10, 10, 12)], ['Çark', a(13, 13, 12, 12, 11, 10, 9, 10)], ['Kadran', a(13, 14, 11, 9, 9, 11, 10, 13)], ['Zemberek', a(13, 13, 14, 9, 12, 9, 10, 11)]] }
];

// Çeviklik (fren ve dönüş): verilmemişse Sürme'den ve gövdeden türetilir (iyi süren çevik, yoğun gövde hantal)
const cev = o => { if (o.ceviklik == null) o.ceviklik = Math.max(6, Math.min(16, Math.round(((o.surme ?? 10) + 20 - (o.yogunluk ?? 10)) / 2 + ((o.hiz ?? 10) > 12 ? -1 : 0)))); };
for (const p of KADRO) cev(p.a || p); for (const r of RAKIP) for (const o of r.oyuncular) cev(o[1]);
// eski 8'li değerlerden yeni anahtarlara (planın 3. bölümü): Okuma → Tepki, Görüş, Yer alma; Aktarım → Pas, Gönderme; Tutuş → Kontrol, Yükleme; Yoğunluk → Güç, Uzanma; Soğukkanlılık ve Alan gücü 10; Bekçi: Refleks (Okuma+Tutuş)/2, Yerleşim Okuma, Alan hakimiyeti Tutuş. Cesaret kalktı
const yeni = (o, taraf) => { const ok = o.okuma ?? 10, ak = o.aktarim ?? 10, tu = o.tutus ?? 10, yo = o.yogunluk ?? 10; Object.assign(o, { tepki: ok, gorus: ok, yeralma: ok, pas: ak, gonderme: ak, kontrol: tu, yukleme: tu, guc: yo, uzanma: yo, sogukkanlilik: 10, alan: 10, refleks: Math.round((ok + tu) / 2), yerlesim: ok, hakimiyet: tu, taraf: o.taraf || taraf || 'sag' }); for (const x of ['okuma', 'aktarim', 'tutus', 'yogunluk', 'cesaret']) delete o[x]; };
for (const p of KADRO) yeni(p.a, TARAF[p.id]); for (const r of RAKIP) for (const o of r.oyuncular) yeni(o[1]);
// GEÇİCİ DENEME: bütün takımların (Liman ve bütün rakipler) bütün özellikleri 20 (Alan gücü ve Soğukkanlılık 10). Eski hâle dönmek için HEPSI_20 = false (herkes asıl değerlerine); RAKIP_10 = true rakipleri 10 yapar. Asıl değerler silinmedi.
const HEPSI_20 = true, RAKIP_10 = false;
if (HEPSI_20) { const yirmi = o => { for (const [key] of OZ.concat(OZ_BEKCI)) if (key !== 'alan' && key !== 'sogukkanlilik') o[key] = 20; }; /* yeni iki özellik (Alan gücü, Soğukkanlılık) 10'da kalır: oyun bugünkü gibi oynasın */ for (const p of KADRO) yirmi(p.a || p); if (!RAKIP_10) for (const r of RAKIP) for (const o of r.oyuncular) yirmi(o[1]); }
if (RAKIP_10) { const on = o => { for (const [key] of OZ.concat(OZ_BEKCI)) o[key] = 10; }; for (const r of RAKIP) for (const o of r.oyuncular) on(o[1]); }
window.ALAN_KADRO = { OZ, OZG, OZB, OZ_BEKCI, KADRO, RAKIP };
})();
