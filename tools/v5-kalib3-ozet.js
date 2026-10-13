// v5-kalib3.js özeti: seçimdeki tahmin, çok denemeli model, gerçek. secim−model = seçme yanılgısı, model−gercek = aklın hatası. Ağırlık: her pasın gerçek sonucu gn denemeden.
const fs = require('fs'); const R = process.argv.slice(2).flatMap(f => fs.readFileSync(f, 'utf8').split('\n').filter(Boolean).map(JSON.parse)).filter(r => r.model != null && r.gercek != null);
const mean = (a, k) => a.reduce((s, r) => s + r[k], 0) / a.length, p = x => (100 * x).toFixed(0).padStart(3), d = x => (x >= 0 ? '+' : '') + (100 * x).toFixed(0);
const se = a => { const g = mean(a, 'gercek'), v = a.reduce((s, r) => s + (r.gercek - g) ** 2, 0) / Math.max(1, a.length - 1); return Math.sqrt(v / a.length); };
const sat = (ad, a) => { if (a.length < 5) return; const S = mean(a, 'secim'), Mo = mean(a, 'model'), G = mean(a, 'gercek'); console.log(`| ${ad} | ${a.length} | %${p(S)} | %${p(Mo)} | %${p(G)} ±${(100 * se(a)).toFixed(0)} | ${d(S - Mo)} | ${d(Mo - G)} |`); };
console.log('| Grup | n | Seçimde | Model | Gerçek | Seçme yanılgısı | Aklın hatası |\n|---|---|---|---|---|---|---|');
sat('hepsi', R); for (const t of ['pas', 'önüne', 'aşırt', 'kenardan', 'kendine']) sat('tür: ' + t, R.filter(r => r.tur === t));
for (const [a, b] of [[0, 10], [10, 20], [20, 30], [30, 99]]) sat(`uzaklık ${a}–${b}`, R.filter(r => r.uz >= a && r.uz < b));
for (const [a, b] of [[0, 2], [2, 4], [4, 7], [7, 99]]) sat(`en yakın rakip ${a}–${b}`, R.filter(r => r.rak >= a && r.rak < b));
for (const [a, b, n] of [[0, 33, 'kendi üçte biri'], [33, 67, 'orta'], [67, 101, 'son üçte bir']]) sat(`hedef: ${n}`, R.filter(r => r.hileri >= a && r.hileri < b));
for (const e of [...new Set(R.map(r => r.evre))].sort()) sat('evre: ' + e, R.filter(r => r.evre === e));
for (const [a, b] of [[0, .8], [.8, .9], [.9, .97], [.97, 1.01]]) sat(`seçimde P ${a}–${b}`, R.filter(r => r.secim >= a && r.secim < b));
