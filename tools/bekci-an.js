// Gönderme anında Bekçi: yerleşmiş mi (hız < 0,05 ve hedefine < 1), hızı, hedefine uzaklığı, top yolunun Bekçi'ye uzaklığı, yol süresi; sayı / kaçan, mesafeye göre
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const G = {};
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length, L = k.LEN;
  k.olay.forEach((o, oi) => { if (o.tur !== 'gonder') return;
    let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
    const t = o.t; if (t + 3 >= L || k.T.hold[t + 3] !== 255) return;
    const bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'), q = t * N + bi, sx = k.T.bx[t] / 100, sy = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0, m = Math.hypot(gx - sx, sy - H / 2);
    const ex = k.P.x[q] / 100, ey = k.P.y[q] / 100, v = Math.hypot(k.P.vx[q], k.P.vy[q]) / 1000, hed = k.P.tx[q] === -32768 ? 0 : Math.hypot(ex - k.P.tx[q] / 100, ey - k.P.ty[q] / 100);
    const vx = k.T.bx[t + 3] / 100 - sx, vy = k.T.by[t + 3] / 100 - sy, vl = Math.hypot(vx, vy) || 1, yol = Math.abs(-(ex - sx) * vy + (ey - sy) * vx) / vl;
    let sure = 0; for (let u = t + 1; u < t + 120 && u < L && k.T.hold[u] === 255; u++) sure = u - t;
    // Çekirdek Bekçi'nin hizasına (kaleye uzaklığı Bekçi'ninkine eşit) kaç tikte geliyor
    let hiza = null; for (let u = t + 1; u < t + 60 && u < L; u++) { if (Math.abs(k.T.bx[u] / 100 - gx) <= Math.abs(ex - gx)) { hiza = u - t; break; } }
    const yer = v < .05 && hed < 1, md = m < 15 ? 'yakın <15' : m < 25 ? 'orta 15–25' : 'uzak 25+', key = md + (s ? ' · SAYI' : ' · kaçan');
    const r = (G[key] = G[key] || { n: 0, yer: 0, v: 0, hed: 0, yol: 0, hiza: 0, hizaN: 0, yolKucuk: 0 }); r.n++; if (yer) r.yer++; r.v += v; r.hed += hed; r.yol += yol; if (hiza != null) { r.hiza += hiza; r.hizaN++; } if (yol < 2.9) r.yolKucuk++;
  });
}
console.log('grup'.padEnd(20), 'n'.padStart(4), ' yerleşmiş', ' Bekçi hızı', ' hedefine', ' yol–Bekçi', ' yol 2,9 içinde', ' Bekçi hizasına (tik)');
for (const key of Object.keys(G).sort()) { const r = G[key]; console.log(key.padEnd(20), String(r.n).padStart(4), `%${(100 * r.yer / r.n).toFixed(0)}`.padStart(10), (r.v / r.n).toFixed(3).padStart(11), (r.hed / r.n).toFixed(1).padStart(9), (r.yol / r.n).toFixed(1).padStart(10), `%${(100 * r.yolKucuk / r.n).toFixed(0)}`.padStart(15), (r.hiza / r.hizaN).toFixed(1).padStart(20)); }
