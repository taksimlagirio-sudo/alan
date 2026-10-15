// Pas kalibrasyonu: aklın pası tutma tahmini (P) → gerçek (pası bizden biri aldı mı), pas türüne ve P dilimine göre
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const R = [];
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) { const k = oku(d + '/' + f), L = k.LEN, tm = i => k.oyuncular[i].takim;
  for (const p of k.pas) { if (p.tur === 'gönder' || p.P == null || p.kim == null) continue; let u = p.t + 1; while (u < L && k.T.hold[u] === 255) u++; if (u >= L) continue; R.push({ tur: p.tur, P: p.P, ok: tm(k.T.hold[u]) === tm(p.kim) ? 1 : 0, kendi: k.T.hold[u] === p.alici }); } }
const s = (ad, a) => console.log(`${ad.padEnd(22)} n ${String(a.length).padStart(5)} · aklın tahmini %${(100 * a.reduce((x, r) => x + r.P, 0) / a.length).toFixed(1)} · gerçek %${(100 * a.reduce((x, r) => x + r.ok, 0) / a.length).toFixed(1)}`);
for (const t of [...new Set(R.map(r => r.tur))]) s(t, R.filter(r => r.tur === t));
for (const [a, b] of [[0, .8], [.8, .95], [.95, .99], [.99, 1.01]]) s(`  P ${a}–${b > 1 ? 1 : b}`, R.filter(r => r.P >= a && r.P < b));
