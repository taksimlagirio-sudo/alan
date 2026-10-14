// Göndermelerde Bekçi: gönderme anında yerinde mi (hedefine uzaklık), Çekirdeğin yolu Bekçi'den ne kadar uzaktan geçiyor (gönderme anındaki konuma göre), yol süresi, en yakın geçiş
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const G = {}; const ornek = [];
const ort = a => a.length ? (a.reduce((x, y) => x + y, 0) / a.length).toFixed(1) : '–';
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length, X = (a, t, i) => a[t * N + i] / 100;
  k.olay.forEach((o, oi) => { if (o.tur !== 'gonder') return;
    const t = o.t, bx = k.T.bx[t] / 100, by = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0, dx = Math.abs(gx - bx), dy = Math.abs(by - H / 2), a = Math.atan2(dy, dx) * 180 / Math.PI, m = Math.hypot(dx, dy);
    let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
    const bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'); if (bi < 0) return;
    const ex = X(k.P.x, t, bi), ey = X(k.P.y, t, bi), txx = k.P.tx[t * N + bi], hedef = txx === -32768 ? null : Math.hypot(ex - txx / 100, ey - k.P.ty[t * N + bi] / 100);
    // Çekirdeğin yönü: gönderme sonrası 3 tik
    const u = t + 3; if (k.T.hold[u] !== 255) return; const vx = k.T.bx[u] / 100 - bx, vy = k.T.by[u] / 100 - by, vl = Math.hypot(vx, vy) || 1;
    const yolUz = Math.abs(-(ex - bx) * vy + (ey - by) * vx) / vl; // Bekçi'nin gönderme anındaki yerinin top yoluna dik uzaklığı
    let en = 1e9, sure = 0; for (let w = t + 1; w < t + 120 && w < k.LEN; w++) { if (k.T.hold[w] !== 255) break; sure = w - t; en = Math.min(en, Math.hypot(k.T.bx[w] / 100 - X(k.P.x, w, bi), k.T.by[w] / 100 - X(k.P.y, w, bi))); }
    const ad = a < 30 ? '0–30°' : a < 60 ? '30–60°' : a < 75 ? '60–75°' : '75°+', md = m < 15 ? 'yakın' : m < 25 ? 'orta' : 'uzak';
    const key = ad + ' ' + md + (s ? ' SAYI' : ' kaçan'); const r = (G[key] = G[key] || { n: 0, h: [], y: [], s: [], e: [], dep: [], v: [] }); r.n++;
    if (hedef != null) r.h.push(hedef); r.y.push(yolUz); r.s.push(sure); r.e.push(en); r.dep.push(Math.abs(ex - gx)); r.v.push(Math.hypot(vx, vy) / 3);
    if (s && a >= 60 && a < 75) ornek.push(`${d.split('/').pop()}/${f} t${t} şut (${bx.toFixed(1)},${by.toFixed(1)}) ${a.toFixed(0)}° ${m.toFixed(0)}br · Bekçi (${ex.toFixed(1)},${ey.toFixed(1)}) hedefine ${hedef == null ? '-' : hedef.toFixed(1)} · yola ${yolUz.toFixed(1)} · süre ${sure} tik · en yakın ${en.toFixed(1)} · hız ${(vl / 3).toFixed(2)}`);
  });
}
console.log('grup'.padEnd(20), 'n'.padStart(3), ' hedefine', ' yola', ' derinlik', ' süre', ' en yakın', ' hız');
for (const key of Object.keys(G).sort()) { const r = G[key]; console.log(key.padEnd(20), String(r.n).padStart(3), ort(r.h).padStart(9), ort(r.y).padStart(5), ort(r.dep).padStart(9), ort(r.s).padStart(5), ort(r.e).padStart(9), (r.v.reduce((a, b) => a + b, 0) / r.v.length).toFixed(2).padStart(5)); }
console.log('\n60–75° sayılar:\n' + ornek.join('\n'));
