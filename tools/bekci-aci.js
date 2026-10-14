// Uzak göndermelerde (ağız ortasına 25+): gönderme anında Bekçi'nin şut çizgisine uzaklığı ve Çekirdeğin Bekçi'nin önünden geçtiği andaki uzaklık
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const G = {};
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length;
  k.olay.forEach((o, i) => { if (o.tur !== 'gonder') return;
    const t = o.t, bx = k.T.bx[t] / 100, by = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0, dx = Math.abs(gx - bx), dy = Math.abs(by - H / 2), m = Math.hypot(dx, dy);
    if (m < 25) return; const a = Math.atan2(dy, dx) * 180 / Math.PI;
    let s = false; for (let j = i + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
    const bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi');
    // Çekirdeğin Bekçi'ye en yakın geçtiği uzaklık (gönderme sonrası 90 tik, top serbestken)
    let en = 1e9; for (let u = t + 1; u < t + 90 && u < k.LEN; u++) { if (k.T.hold[u] !== 255) break; const ex = k.P.x[u * N + bi] / 100, ey = k.P.y[u * N + bi] / 100; en = Math.min(en, Math.hypot(k.T.bx[u] / 100 - ex, k.T.by[u] / 100 - ey)); }
    const key = (a < 60 ? '0–60°' : a < 75 ? '60–75°' : '75°+') + (s ? ' sayı' : ' kaçan');
    (G[key] = G[key] || []).push(en);
  });
}
for (const key of Object.keys(G).sort()) { const v = G[key]; console.log(key.padEnd(14), 'n', v.length, '· Çekirdeğin Bekçi\'ye en yakın geçişi ort.', (v.reduce((a, b) => a + b, 0) / v.length).toFixed(1)); }
