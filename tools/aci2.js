// Açı × mesafe: gönderme sayısı ve sayı oranı. Ayrıca Bekçi'nin gönderme anında ağız ortasına uzaklığı.
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2);
const AC = [[0, 30], [30, 60], [60, 75], [75, 91]], MS = [[0, 15], [15, 25], [25, 999]]; const S = {}, R = [];
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, n = k.P.x.length / k.LEN;
  k.olay.forEach((o, i) => { if (o.tur !== 'gonder') return;
    const t = o.t, bx = k.T.bx[t] / 100, by = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0, dx = Math.abs(gx - bx), dy = Math.abs(by - H / 2);
    const a = Math.atan2(dy, dx) * 180 / Math.PI, m = Math.hypot(dx, dy);
    let s = false; for (let j = i + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
    const ai = AC.findIndex(([x, y]) => a >= x && a < y), mi = MS.findIndex(([x, y]) => m >= x && m < y), key = ai + ',' + mi;
    (S[key] = S[key] || { g: 0, s: 0 }).g++; if (s) S[key].s++;
    R.push({ a, m, s });
  });
}
console.log('açı \\ mesafe (ağız ortasına): ' + MS.map(([x, y]) => y === 999 ? x + '+' : x + '–' + y).join(' | '));
AC.forEach(([x, y], ai) => console.log(`${x}–${y === 91 ? 90 : y}°: ` + MS.map((_, mi) => { const c = S[ai + ',' + mi]; return c ? `${c.s}/${c.g} (%${(100 * c.s / c.g).toFixed(0)})` : '–'; }).join(' | ')));
const ort = (arr) => (arr.reduce((a, r) => a + r.m, 0) / arr.length).toFixed(1);
AC.forEach(([x, y]) => { const a = R.filter(r => r.a >= x && r.a < y); console.log(`${x}–${y}° ortalama mesafe ${ort(a)}`); });
