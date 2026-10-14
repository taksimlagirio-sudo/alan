// Bekçi yerinde değilken giren sayılar: gönderenin topu nasıl aldığı (pas mı), pas başladığında Bekçi nerede ve kime çıkmıştı, pas süresince ne kadar yol aldı
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const R = [];
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length;
  k.olay.forEach((o, oi) => { if (o.tur !== 'gonder') return;
    let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; } if (!s) return;
    const t = o.t, bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'), q = t * N + bi, gx = o.takim === 0 ? W : 0;
    const ex = k.P.x[q] / 100, ey = k.P.y[q] / 100, txx = k.P.tx[q] / 100, tyy = k.P.ty[q] / 100, hed = Math.hypot(ex - txx, ey - tyy); if (k.T.hold[t + 3] !== 255 || hed <= 3) return;
    const bx = k.T.bx[t] / 100, by = k.T.by[t] / 100, a = Math.atan2(Math.abs(by - H / 2), Math.abs(gx - bx)) * 180 / Math.PI;
    const h = k.T.hold[t - 1]; let w = t - 1; while (w > 0 && k.T.hold[w] === h) w--; const al = w + 1; // alış anı
    let w2 = w; while (w2 > 0 && k.T.hold[w2] === 255) w2--; const pb = w2 + 1, pasci = k.T.hold[w2]; // pas başı, pasçı
    const pasMi = pasci !== 255 && k.oyuncular[pasci] && k.oyuncular[pasci].takim === o.takim;
    const q0 = pb * N + bi, bx0 = k.P.x[q0] / 100, by0 = k.P.y[q0] / 100;
    const pcx = k.T.bx[pb] / 100, pcy = k.T.by[pb] / 100; // pasın başladığı yer
    R.push({ a, el: t - al, uc: al - pb, pasMi, derin0: Math.abs(bx0 - gx), pascUz: Math.hypot(bx0 - pcx, by0 - pcy), yol: Math.hypot(ex - bx0, ey - by0), sonUz: hed, derin: Math.abs(ex - gx), kaleUz: Math.hypot(ex - gx, ey - H / 2) });
  });
}
const o = (a, k) => (a.reduce((s, x) => s + x[k], 0) / a.length).toFixed(1);
console.log('Bekçi yerinde değilken giren sayı:', R.length, '· gönderen topu pasla almış:', R.filter(r => r.pasMi).length);
const P = R.filter(r => r.pasMi);
console.log('pas uçuşu', o(P, 'uc'), 'tik · alıştan göndermeye', o(P, 'el'), 'tik');
console.log('pas başladığında Bekçi: kalesinden derinlik', o(P, 'derin0'), '· pasçıya uzaklık', o(P, 'pascUz'));
console.log('gönderme anında Bekçi: derinlik', o(P, 'derin'), '· ağız ortasına uzaklık', o(P, 'kaleUz'), '· hedefine', o(P, 'sonUz'), '· pas boyunca aldığı yol', o(P, 'yol'));
console.log('pas başında pasçıya 6 birimden yakın (ona çıkmış):', P.filter(r => r.pascUz < 6).length, '/', P.length, '· Kuyu alanı dışında (derinlik > 9):', P.filter(r => r.derin0 > 9).length);
for (const g of [[0, 60], [60, 91]]) { const a = P.filter(r => r.a >= g[0] && r.a < g[1]); console.log(`${g[0]}–${g[1] === 91 ? 90 : g[1]}°: n ${a.length} · pasçıya ${o(a, 'pascUz')} · derinlik ${o(a, 'derin0')} → ${o(a, 'derin')} · ağıza ${o(a, 'kaleUz')}`); }
