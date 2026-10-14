// Birleşik ölçüm, adım 3: iki kayıt klasörünü (önce / sonra) aynı senaryolar üzerinden karşılaştırır. Saniyeler sürer.
// Kullanım: node tools/kayit-ozet.js <eski klasör> <yeni klasör>   (dosyalar <sc>.json.gz)
const fs = require('fs'), path = require('path'); const { oku, ol } = require('./kayit-oku');
const [A, B] = process.argv.slice(2), ad = d => path.basename(d);
const sc = d => fs.readdirSync(d).filter(f => /^\d+\.json\.gz$/.test(f)).map(f => +f.split('.')[0]);
const ort = sc(A).filter(s => sc(B).includes(s)).sort((a, b) => a - b);
const hesap = d => ort.map(s => { const k = oku(path.join(d, s + '.json.gz')); return { g: ol.genel(k), d: ol.durma(k), s: ol.salinim(k), k: ol.kosu(k), p: ol.pasTahmin(k), e: ol.evre(k) }; });
const RA = hesap(A), RB = hesap(B), mean = (R, f) => { const a = R.map(f).filter(x => x != null && !Number.isNaN(x)); return a.length ? a.reduce((s, x) => s + x, 0) / a.length : null; };
const topla = (R, f) => R.reduce((s, r) => s + f(r), 0);
const sat = (ad, f, d = 2, kind = 'ort') => { const g = R => kind === 'ort' ? mean(R, f) : f(R); const a = g(RA), b = g(RB); console.log(`| ${ad} | ${a == null ? '-' : a.toFixed(d)} | ${b == null ? '-' : b.toFixed(d)} |`); };
const K = (R, key) => topla(R, r => r.k[0][key] + r.k[1][key]);
const PT = (R, f) => { const a = R.flatMap(r => r.p).filter(f); return a.length ? [a.reduce((s, x) => s + x.P, 0) / a.length, a.reduce((s, x) => s + x.ok, 0) / a.length, a.length] : null; };
console.log(`Senaryolar (${ort.length}): ${ort.join(',')}\n\n| | ${ad(A)} | ${ad(B)} |\n|---|---|---|`);
console.log('| **Oyun** | | |'); sat('Sayı / maç', r => r.g.sayi); sat('Pas / maç', r => r.g.pas, 0); sat('Pas tutma %', r => r.g.pasTut * 100, 1); sat('Gönderme / maç', r => r.g.gon, 1); sat('Top kapma / maç', r => r.g.kapma, 1); sat('İkili / maç', r => r.g.ikili, 1); sat('Top el değiştirme / maç', r => r.g.elDegis, 1); sat('Hesap süresi (sn / maç)', r => r.g.hesapSn, 0);
console.log('| **Durma** | | |'); sat('Her an duran hücumcu (/5)', r => r.d.hucDuran); sat('Her an duran savunmacı (/6)', r => r.d.savDuran); sat('Açık pas yolu (laneOk ≥ 0,7)', r => r.d.acikYol);
sat('Uzun tutuş (≥ 2 sn) payı %', r => r.d.uzunPay * 100, 1); sat('  bunun Bekçi\'de olanı %', R => { const a = topla(R, r => r.d.uzunBekci.pay || 0), b = topla(R, r => (r.d.uzunBekci.pay || 0) + (r.d.uzun.pay || 0)); return b ? 100 * a / b : null; }, 0, 'top');
sat('  saha oyuncusu tutarken: duran hücumcu', r => r.d.uzun.hucDuran); sat('  saha oyuncusu tutarken: 1 sn+ duran hücumcu', r => r.d.uzun.uzunDuran); sat('  saha oyuncusu tutarken: açık pas yolu', r => r.d.uzun.acikYol); sat('  Bekçi tutarken: duran hücumcu', r => r.d.uzunBekci.hucDuran);
console.log('| **Salınım** | | |'); sat('Hücumcu hedefe varma %', r => r.s.huc.varma * 100, 0); sat('Hücumcu 3 sn içinde eski hedefe dönüş %', r => r.s.huc.geriDonus * 100, 0); sat('Savunmacı 3 sn içinde eski hedefe dönüş %', r => r.s.sav.geriDonus * 100, 0);
console.log('| **Koşu** | | |'); const n = R => R.length * 2; sat('Koşu / takım-maç', R => K(R, 'kosu') / n(R), 1, 'top'); sat('Çizgiyi geçen %', R => 100 * K(R, 'gecti') / K(R, 'kosu'), 0, 'top'); sat('Koşucuya pas / takım-maç', R => K(R, 'pas') / n(R), 1, 'top'); sat('Koşucuya, çizgi arkasına pas', R => K(R, 'kosuArka') / n(R), 2, 'top'); sat('Çizgi arkasına pas', R => K(R, 'pasArka') / n(R), 2, 'top'); sat('  tutulan %', R => 100 * K(R, 'pasArkaTut') / K(R, 'pasArka'), 0, 'top'); sat('Son çizginin arkasında oyuncu (hücum anı %)', R => 100 * K(R, 'arkadaAn') / K(R, 'hucAn'), 1, 'top');
console.log('| **Aklın pas tahmini (kopyasız)** | | |'); for (const [t, f] of [['hepsi', x => true], ['pas', x => x.tur === 'pas'], ['önüne', x => x.tur === 'önüne'], ['kenardan', x => x.tur === 'kenardan'], ['kendine', x => x.tur === 'kendine']]) { const a = PT(RA, f), b = PT(RB, f); const fm = v => v ? `%${(100 * v[0]).toFixed(0)} → %${(100 * v[1]).toFixed(0)} (${v[2]})` : '-'; console.log(`| tahmin → gerçek: ${t} | ${fm(a)} | ${fm(b)} |`); }
