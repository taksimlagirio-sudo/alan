// v5-kalib2.js özeti: seçimdeki tahmin, çok denemeli model, gerçek (kopyalarla). secim−model = seçme yanılgısı, model−gercek = aklın hatası
const fs = require('fs'); for (const f of process.argv.slice(2)) { const R = fs.readFileSync(f, 'utf8').split('\n').filter(Boolean).map(JSON.parse).filter(r => r.model != null && r.gercek != null);
  const mean = (a, k) => a.reduce((s, r) => s + r[k], 0) / a.length, p = x => (100 * x).toFixed(0).padStart(3), d = x => (x >= 0 ? '+' : '') + (100 * x).toFixed(0);
  const sat = (ad, a) => { if (!a.length) return; const S = mean(a, 'secim'), Mo = mean(a, 'model'), G = mean(a, 'gercek'); console.log(`  ${ad.padEnd(20)} n=${String(a.length).padStart(4)} · seçim %${p(S)} · model %${p(Mo)} · gerçek %${p(G)} · seçme yanılgısı ${d(S - Mo)} · aklın hatası ${d(Mo - G)}`); };
  console.log(`== ${f}`); sat('hepsi', R); for (const t of ['pas', 'önüne', 'kenardan', 'kendine']) sat(t, R.filter(r => r.tur === t));
  for (const [a, b] of [[0, 10], [10, 20], [20, 30], [30, 99]]) sat(`uzaklık ${a}–${b}`, R.filter(r => r.uz >= a && r.uz < b));
  for (const [a, b] of [[0, 3], [3, 6], [6, 99]]) sat(`rakip ${a}–${b}`, R.filter(r => r.rak >= a && r.rak < b)); }
