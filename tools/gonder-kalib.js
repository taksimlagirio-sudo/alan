// Gönderme kalibrasyonu: aklın girme tahmini (Pk) → gerçek, mesafeye göre
const { oku } = require('./kayit-oku.js'), fs = require('fs'), d = process.argv[2]; const R = [];
for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) { const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H;
  for (const p of k.pas) { if (p.tur !== 'gönder' || p.Pk == null || p.kim == null) continue; const tm = k.oyuncular[p.kim].takim, gx = tm === 0 ? W : 0, m = Math.hypot(gx - k.T.bx[p.t] / 100, k.T.by[p.t] / 100 - H / 2);
    const s = k.olay.find(o => o.t > p.t && o.t <= p.t + 120 && (o.tur === 'sayi' || o.tur === 'kesme' || o.tur === 'pas' || o.tur === 'gonder')); R.push({ m, Pk: p.Pk, ok: s && s.tur === 'sayi' && s.takim === tm ? 1 : 0 }); } }
for (const [a, b] of [[0, 15], [15, 25], [25, 99], [0, 99]]) { const A = R.filter(r => r.m >= a && r.m < b); console.log(`${a}–${b === 99 ? '+' : b}: n ${A.length} · akıl %${(100 * A.reduce((s, r) => s + r.Pk, 0) / A.length).toFixed(1)} · gerçek %${(100 * A.reduce((s, r) => s + r.ok, 0) / A.length).toFixed(1)}`); }
