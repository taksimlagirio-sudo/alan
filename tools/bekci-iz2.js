// Göndermelerde Bekçi yerinde mi (hedefine ≤ 3): açı dilimine ve sonuca göre pay. Yerinde değilse görevi ve topun gönderenin ayağında kaç tik kaldığı. Yerindeyse yoldan uzaklık ve yol süresi.
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const G = {}, J = {}, IN = [];
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length;
  k.olay.forEach((o, oi) => { if (o.tur !== 'gonder') return;
    const t = o.t, bx = k.T.bx[t] / 100, by = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0, a = Math.atan2(Math.abs(by - H / 2), Math.abs(gx - bx)) * 180 / Math.PI;
    let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
    const bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'), q = t * N + bi; if (k.T.hold[t + 3] !== 255) return;
    const ex = k.P.x[q] / 100, ey = k.P.y[q] / 100, hed = k.P.tx[q] === -32768 ? 0 : Math.hypot(ex - k.P.tx[q] / 100, ey - k.P.ty[q] / 100), yer = hed <= 3;
    let el = 0; const h = k.T.hold[t - 1]; for (let w = t - 1; w > 0 && k.T.hold[w] === h; w--) el++;
    const ad = a < 30 ? '0–30°' : a < 60 ? '30–60°' : a < 75 ? '60–75°' : '75°+'; const r = (G[ad] = G[ad] || { gs: 0, gy: 0, ks: 0, ky: 0 });
    if (s) { r.gs++; if (yer) r.gy++; } else { r.ks++; if (yer) r.ky++; }
    if (s && !yer) { const jb = k.gorevler[k.P.job[q]] || '?'; const jj = jb.split(' · ')[0]; (J[jj] = J[jj] || []).push(el); }
    if (s && yer) { const u = t + 3, vx = k.T.bx[u] / 100 - bx, vy = k.T.by[u] / 100 - by, vl = Math.hypot(vx, vy) || 1; let sure = 0; for (let w = t + 1; w < t + 120 && k.T.hold[w] === 255; w++) sure = w - t; IN.push({ ad, yol: Math.abs(-(ex - bx) * vy + (ey - by) * vx) / vl, sure, el }); }
  });
}
console.log('açı      sayılarda Bekçi yerinde   kaçanlarda Bekçi yerinde');
for (const ad of ['0–30°', '30–60°', '60–75°', '75°+']) { const r = G[ad]; console.log(ad.padEnd(8), `${r.gy}/${r.gs} (%${(100 * r.gy / r.gs).toFixed(0)})`.padEnd(25), `${r.ky}/${r.ks} (%${(100 * r.ky / r.ks).toFixed(0)})`); }
console.log('\nYerinde olmayan Bekçi sayılarda: görevi · gönderenin topu tuttuğu süre (tik, ort.)');
for (const [j, v] of Object.entries(J).sort((a, b) => b[1].length - a[1].length)) console.log(' ', j, v.length, '· tutma ort.', (v.reduce((a, b) => a + b, 0) / v.length).toFixed(0));
console.log('\nYerindeyken giren sayılar: açı · topun yolunun Bekçi\'ye uzaklığı · yol süresi (tik)');
for (const ad of ['0–30°', '30–60°', '60–75°', '75°+']) { const a = IN.filter(x => x.ad === ad); if (a.length) console.log(' ', ad, a.length, '· yola', (a.reduce((s, x) => s + x.yol, 0) / a.length).toFixed(1), '· süre', (a.reduce((s, x) => s + x.sure, 0) / a.length).toFixed(1)); }
