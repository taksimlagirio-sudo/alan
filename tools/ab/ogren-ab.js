// A · hafızanın kendi maçları (kusursuz okuma, karışık taktik, tam beyin). Tablo maç boyunca sabit (verilen tur tablosu); her 6 tikte top sahibinin durumu kaydedilir:
// eski hücre (2700), uzaklık, açıklık, boş arkadaş, takım. Sayı olunca o ana kadarki kayıtlar +1/−1 alır; maç sonunda sonucu belli olmayanlar atılır (ogrenme.js ile aynı kural).
// Kullanım: node tools/ab/ogren-ab.js <motor> <maç no> <çıktı.json> [tablo.json içindeki ad (A) ve dosya]
const fs = require('fs'), O = require('./ortak'), E = require('./ek');
const [dir, iS, out, tabloF, ad] = process.argv.slice(2), i = +iS, seed = 550000 + i * 104729;
const g = O.yukle(dir, seed), M = g.AlanMatch, B = g.AlanBeyin, ST = g.AlanState; self.ALAN_OKX = { eq: ['karar', 'kararHizi', 'rakipModel', 'gonder', 'kontrol', 'kenar', 'algi', 'yerlesim'], mid: 20 };
if (tabloF) { const T = JSON.parse(fs.readFileSync(tabloF, 'utf8'))[ad || 'A']; g.ALAN_VS.v = Float64Array.from(T.v); if (T.ek) globalThis.__VSEK = E.okuyucu(g, T.ek); }
const T = O.taktik(seed), m = M.createMatch(seed, { tac: T.tac, len: +process.env.LEN || 5400 }), t0 = Date.now(), ol = O.olcuKur(); let bek = [], s0 = m.score.slice(); const ornek = [];
while (!m.over) { let p = B.planla(m); while (p) p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; M.step(m); O.olcuAdim(m, ol);
  if (m.score[0] !== s0[0] || m.score[1] !== s0[1]) { const sc = m.score[0] > s0[0] ? 0 : 1; for (const e of bek) { e.push(e[5] === sc ? 1 : -1); ornek.push(e); } bek = []; s0 = m.score.slice(); }
  const h = m.holder; if (h && m.tick % 6 === 0) { const tr = m.winTeam === h.team && (m.tick - (m.winT ?? -1e9)) < 180, f = E.olc(g, m.ps, h.team, h.x, h.y); bek.push([ST.idx(m.ps, h.team, h.x, h.y, tr), +f.uz.toFixed(1), +f.acik.toFixed(1), f.bos, m.tick, h.team]); } }
fs.writeFileSync(out, JSON.stringify({ i, tac: T.ad, sn: Math.round((Date.now() - t0) / 1000), ...O.olcuSon(m, ol), ornek }));
console.log(i, T.ad, m.score, ornek.length, Math.round((Date.now() - t0) / 1000) + 's');
