// 8. madde · 10'ar gerçek maç (Herkes 10, karışık taktik; S maçlarıyla aynı tohumlar ve taktikler). Yöntem yalnız top sahibinin kararını değiştirir; savunmanın beyni hepsinde açık.
//  C: top sahibinin araması kapalı (ilk bakış: decide'ın en iyisi; tek dokunuş da refleks)
//  A0 / A: şimdiki arama, hafızanın öğrendiği tabloyla (tablo maçın her yerinde: arama, ilk bakış, uç değer)
//  B: top sahibinin her kararında aramanın aday listesi hücumun sonuna kadar oynatılır (ilk bakışla, NB deneme, ortak zar), en yüksek ortalama seçilir. Tek dokunuş eski aramayla.
// Kullanım: node tools/ab/mac.js <motor> <maç no> <yöntem> [tablo.json] [NB=6]
const fs = require('fs'), O = require('./ortak'), E = require('./ek');
const [dir, iS, yon, tabloF, nbS] = process.argv.slice(2), i = +iS, NB = +nbS || 6, seed = 880000 + i * 7919;
const g = O.yukle(dir, seed), M = g.AlanMatch, B = g.AlanBeyin, D = g.AlanDecide.D, T = O.taktik(seed), m = M.createMatch(seed, { tac: T.tac, len: +process.env.LEN || 5400 }), ol = O.olcuKur(), t0 = Date.now();
if (yon === 'C') { globalThis.__NOKARAR = true; B.A.otBeyin = false; }
if (yon === 'A' || yon === 'A0') { const TT = JSON.parse(fs.readFileSync(tabloF, 'utf8'))[yon]; g.ALAN_VS.v = Float64Array.from(TT.v); if (TT.ek) globalThis.__VSEK = E.okuyucu(g, TT.ek); }
let nB = 0, msB = 0;
while (!m.over) { let p = B.planla(m);
  if (yon === 'B' && p && p.tip === 'karar') { const tb = Date.now(), h = m.holder, combos = []; let last = null; for (const j of p.jobs) { if (last && last.d === j.d && last.run === j.run) continue; last = j; combos.push(j); }
    const v = combos.map(x => { let s = 0; for (let r = 0; r < NB; r++) s += B.rollout(m, { tip: 'karar', hid: h.id, d: x.d, o: x.o, run: x.run, team: h.team, sigma: x.sigma, sona: true, H: 480, HMAX: 480, salt: ((m.tick * 40503) ^ (r * 2654435761 + 99)) >>> 0 }); return s / NB; });
    let k = 0; v.forEach((x, q) => { if (x > v[k]) k = q; }); const x = combos[k];
    m._zorla = { hid: h.id, d: x.d }; m.kosu = m.kosu ? m.kosu.slice() : [null, null]; m.kosu[h.team] = x.run ? { ...x.run, until: m.tick + B.A.KT } : (m.kosu[h.team] && m.kosu[h.team].until > m.tick ? m.kosu[h.team] : null);
    nB++; msB += Date.now() - tb; p = null; }
  while (p) p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; M.step(m); O.olcuAdim(m, ol); }
console.log(JSON.stringify({ i, yon, tac: T.ad, sn: Math.round((Date.now() - t0) / 1000), bKarar: nB, bMs: nB ? Math.round(msB / nB) : undefined, ...O.olcuSon(m, ol) }));
