// v5-kalib.js çıktısının özeti: tahmin (ortalama P) ile gerçek (tutma oranı) dilim dilim. fark = tahmin − gerçek (+ iyimser, − kötümser)
const fs = require('fs'); for (const f of process.argv.slice(2)) { const R = fs.readFileSync(f, 'utf8').split('\n').filter(Boolean).map(JSON.parse);
  const sat = (ad, a) => { if (!a.length) return; const P = a.reduce((s, r) => s + r.P, 0) / a.length, G = a.reduce((s, r) => s + r.sonuc, 0) / a.length, se = Math.sqrt(G * (1 - G) / a.length); console.log(`  ${ad.padEnd(22)} n=${String(a.length).padStart(4)} · tahmin %${(100 * P).toFixed(0).padStart(3)} · gerçek %${(100 * G).toFixed(0).padStart(3)} ±${(100 * se).toFixed(0)} · fark ${(P - G >= 0 ? '+' : '') + (100 * (P - G)).toFixed(0)}${Math.abs(P - G) > 2 * se ? ' *' : ''}`); };
  console.log(`== ${f} (${R.length} pas)`); sat('hepsi', R);
  console.log(' tür'); for (const t of ['pas', 'önüne', 'kenardan', 'kendine', 'aşırt']) sat(t, R.filter(r => r.tur === t));
  console.log(' uzaklık'); for (const [a, b] of [[0, 10], [10, 20], [20, 30], [30, 99]]) sat(`${a}–${b}`, R.filter(r => r.uz >= a && r.uz < b));
  console.log(' en yakın rakip (pas atana)'); for (const [a, b] of [[0, 3], [3, 6], [6, 10], [10, 99]]) sat(`${a}–${b}`, R.filter(r => r.rak >= a && r.rak < b));
  console.log(' tahmin dilimi'); for (const [a, b] of [[0, .5], [.5, .8], [.8, .95], [.95, 1.01]]) sat(`P ${a}–${b}`, R.filter(r => r.P >= a && r.P < b));
  console.log(' tür × uzaklık'); for (const t of ['pas', 'önüne', 'kenardan', 'kendine']) for (const [a, b] of [[0, 10], [10, 20], [20, 99]]) sat(`${t} ${a}–${b}`, R.filter(r => r.tur === t && r.uz >= a && r.uz < b));
  console.log(' tür × rakip'); for (const t of ['pas', 'önüne', 'kenardan', 'kendine']) for (const [a, b] of [[0, 3], [3, 6], [6, 99]]) sat(`${t} rakip ${a}–${b}`, R.filter(r => r.tur === t && r.rak >= a && r.rak < b)); }
