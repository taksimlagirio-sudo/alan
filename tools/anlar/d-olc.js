// D ölçümü: Rüzgâr 6903, koşusunun önüne pas (31.9,46): kafadaki deneme; hız ve rakip çıkarma deneyleri
const fs = require('fs'), [dir, anF] = process.argv.slice(2); const { M, kurMac } = require('./yukle')(dir); const { oku } = require('./graf');
const A = JSON.parse(fs.readFileSync(anF)); const m0 = kurMac(A.kurulum.yeni), m = oku(A.kopya, () => M.kopya(m0.r)); const AD = self.AlanDecide, D0 = AD.decide, f3 = v => (+v).toFixed(3);
AD.decide = function (h, src, ch, ...r) { const res = D0.call(this, h, src, ch, ...r); if (h.name !== 'Rüzgâr' || m.tick !== 6903 || AD.D._inLook) return res;
  const S = src.find(p => p.name === 'Şimşek'), o = res.opts.find(o => o.kind === 'önüne' && o.q && o.q.name === 'Şimşek' && Math.abs(o.to.x - 31.9) < .3), L = o.launch, sp0 = Math.hypot(L.vx, L.vy);
  let s = 99; const rnd = () => (s = (Math.imul(s, 1664525) + 1013904223) >>> 0) / 4294967296;
  const dene = (ad, src2, k) => { const im = AD.imagine(h, src2, { x: h.x, y: h.y, vx: L.vx * k, vy: L.vy * k, ch }, rnd, 40, S, o.to); console.log(`  ${ad.padEnd(34)} hız ${(sp0 * k).toFixed(2)} · Şimşek alır %${(im.P * 100).toFixed(0)} · kayıp %${(im.Pl * 100).toFixed(0)} · bitiş (${im.end.x.toFixed(1)},${im.end.y.toFixed(1)})`); };
  for (const k of [1, 1.5, 2, 3]) dene('olduğu gibi', src, k);
  for (const ad of ['Deniz', 'Mert', 'Ece', 'Aras']) dene(ad + ' yokmuş gibi', src.filter(p => p.name !== ad), 1);
  dene('Deniz ve Mert yokmuş gibi', src.filter(p => p.name !== 'Deniz' && p.name !== 'Mert'), 1);
  return res; };
while (m.tick < 6904) M.step(m); process.exit(0);
