// Göndermelerin açısına göre sayı oranı. Açı: gönderme noktasından rakip ağzın ortasına giden çizginin arka çizgiye dikle yaptığı açı.
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2);
const dil = [[0, 30], [30, 60], [60, 75], [75, 91]], S = dil.map(() => ({ g: 0, s: 0 })); let top = { g: 0, s: 0, mac: 0 }; const orn = [];
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = (k.saha && k.saha.W) || 100, H = (k.saha && k.saha.H) || 50; top.mac++;
  const ol = k.olay;
  ol.forEach((o, i) => {
    if (o.tur !== 'gonder') return;
    const t = o.t, bx = k.T.bx[t] / 100, by = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0;
    const a = Math.atan2(Math.abs(by - H / 2), Math.abs(gx - bx)) * 180 / Math.PI;
    let sayi = false; for (let j = i + 1; j < ol.length; j++) { const p = ol[j]; if (p.tur === 'fake') continue; if (p.tur === 'sayi' && p.takim === o.takim) sayi = true; break; }
    const b = S[dil.findIndex(([x, y]) => a >= x && a < y)]; b.g++; top.g++; if (sayi) { b.s++; top.s++; }
    if (a >= 75) orn.push(`${f} t${t} (${bx.toFixed(0)},${by.toFixed(0)}) ${a.toFixed(0)}° ${sayi ? 'SAYI' : '-'} · ${o.metin}`);
  });
}
console.log(`${top.mac} maç · gönderme ${top.g} · sayı ${top.s}`);
dil.forEach(([x, y], i) => console.log(`${x}–${y === 91 ? 90 : y}°: gönderme ${S[i].g}, sayı ${S[i].s} (%${S[i].g ? (100 * S[i].s / S[i].g).toFixed(0) : '-'}), sayıların %${(100 * S[i].s / top.s).toFixed(0)}'i`));
console.log(orn.join('\n'));
