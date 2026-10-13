// v5-yer.js özeti: aynı anda "doğal" (motorun yerleşimi) ile "açık nokta" (oyuncu yakındaki açık noktaya koşar) karşılaştırması; fark = açık nokta − doğal, anlar üzerinden ortalama ± standart hata.
// Kullanım: node tools/v5-yer-ozet.js lab/yer32/*.jsonl
const fs = require('fs'); const R = process.argv.slice(2).flatMap(f => fs.readFileSync(f, 'utf8').split('\n').filter(Boolean).map(l => JSON.parse(l)));
const mean = a => a.reduce((s, x) => s + x, 0) / a.length, se = a => { const m = mean(a); return Math.sqrt(a.reduce((s, x) => s + (x - m) ** 2, 0) / Math.max(1, a.length - 1) / a.length); };
function blok(ad, rs) { if (!rs.length) return; const d = k => rs.map(r => r.acikNokta[k] - r.dogal[k]), f = (k, p) => p ? (v => (v * 100).toFixed(0) + '%') : (v => v.toFixed(1));
  console.log(`\n## ${ad} (${rs.length} an)\n\n| | doğal | açık nokta | fark |\n|---|---|---|---|`);
  for (const [k, n, p] of [['kayip', 'top kaybı (2,5 sn)', 1], ['aldi', 'o oyuncu pas aldı', 1], ['gon', 'gönderme', 1], ['sayi', 'sayı', 1], ['il', 'ilerleme (birim)', 0], ['yak', 'oyuncunun açık noktaya en yakın mesafesi', 0]]) { const a = mean(rs.map(r => r.dogal[k])), b = mean(rs.map(r => r.acikNokta[k])), x = d(k); console.log(`| ${n} | ${f(k, p)(a)} | ${f(k, p)(b)} | ${p ? (mean(x) * 100).toFixed(0) + ' ± ' + (se(x) * 100).toFixed(0) + ' puan' : mean(x).toFixed(1) + ' ± ' + se(x).toFixed(1)} |`); } }
blok('Hepsi', R);
blok('Açık nokta ileride (2+ birim)', R.filter(r => r.ileri >= 2)); blok('Açık nokta yanda ya da geride', R.filter(r => r.ileri < 2));
blok('Kendi yarısında', R.filter(r => r.x < 50)); blok('Rakip yarısında', R.filter(r => r.x >= 50));
blok('Pas hattı kapalıydı (< 0,5)', R.filter(r => r.hat < .5)); blok('Hat açıktı ama savunmacı yakındı (< 5)', R.filter(r => r.hat >= .5));
console.log('\nRoller: ' + Object.entries(R.reduce((o, r) => (o[r.rol] = (o[r.rol] || 0) + 1, o), {})).map(([k, v]) => k + ' ' + v).join(', ') + ' · görevler: ' + Object.entries(R.reduce((o, r) => (o[r.job] = (o[r.job] || 0) + 1, o), {})).map(([k, v]) => k + ' ' + v).join(', '));
