// Gerçek maç (Herkes 10, karışık taktik, tam beyin) oynanır; top sahibinin her ADIM'ıncı karar anı, beyin düşünmeden hemen önce kaydedilir (paketle).
// Aynı koşu, mevcut aramanın 10 maçlık ölçülerini de verir. Kullanım: node tools/ab/topla.js <motor> <maç no> <çıktı klasörü> [ADIM=3]
const path = require('path'), fs = require('fs'), O = require('./ortak');
const [dir, iS, out, adS] = process.argv.slice(2), i = +iS, ADIM = +adS || 3, seed = 880000 + i * 7919; fs.mkdirSync(out, { recursive: true });
const g = O.yukle(dir, seed), M = g.AlanMatch, B = g.AlanBeyin, T = O.taktik(seed), m = M.createMatch(seed, { tac: T.tac, len: 5400 }), ol = O.olcuKur();
let nk = 0, ns = 0; const t0 = Date.now();
while (!m.over) { const S = B.paketle(m); let p = B.planla(m), ilk = true;
  while (p) { if (ilk && p.tip === 'karar') { nk++; if (nk % ADIM === 0) { O.kaydet(path.join(out, `a${i}_${m.tick}.bin`), { mac: i, seed, tac: T.ad, S }); ns++; } } ilk = false; p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; }
  M.step(m); O.olcuAdim(m, ol); }
console.log(JSON.stringify({ i, tac: T.ad, sn: Math.round((Date.now() - t0) / 1000), karar: nk, an: ns, ...O.olcuSon(m, ol) }));
