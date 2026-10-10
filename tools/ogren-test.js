// v4g · hafıza testi (Design'ın görevi 5). Öğrenci gibi: kusursuz okuma, karışık taktikler; her maçta AlanOgren tabloyu günceller.
// 0, 25, 50. maçlarda tablonun kopyası lab/ogren/<k>/tablo-<n>.json olarak yazılır.
// Kullanım: node tools/ogren-test.js <motor> <k (tohum dizisi)> [maç=50]
const fs = require('fs'), path = require('path'), { load } = require('./load');
const [dir, kS, nS] = process.argv.slice(2), K = +kS || 1, NM = +nS || 50, out = path.join('lab/ogren', 'k' + K); fs.mkdirSync(out, { recursive: true });
const g = load(dir, 1000 + K); (0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/ogrenme.js', 'utf8'));
globalThis.ALAN_OKX = { eq: ['karar', 'kararHizi', 'rakipModel', 'gonder', 'kontrol', 'kenar', 'algi', 'yerlesim'], mid: 20 };
const M = g.AlanMatch, B = g.AlanBeyin, O = g.AlanOgren, T = g.ALAN_VS; if (!T.n) T.n = new Array(T.v.length).fill(0);
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacs = Object.values(STY).map(a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] }));
let r0 = (K * 2654435761) >>> 0; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296;
const yaz = n => fs.writeFileSync(path.join(out, `tablo-${n}.json`), JSON.stringify({ v: Array.from(T.v), n: Array.from(T.n) }));
yaz(0); const t0 = Date.now();
for (let k = 1; k <= NM; k++) { const m = M.createMatch(1 + Math.floor(R() * 1e9), { len: 5400, tac: [tacs[Math.floor(R() * 6)], tacs[Math.floor(R() * 6)]] }); O.basla(m);
  while (!m.over) { let p = B.planla(m); while (p) p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; M.step(m); O.adim(m); }
  console.error(`k${K} maç ${k} skor ${m.score} ${((Date.now() - t0) / 60000).toFixed(1)} dk`); if (k === 25 || k === NM) yaz(k); }
