// Pasiflik ölçüsü: denenen takım (A) belirli tempo/risk ile varsayılan rakibe karşı oynar.
// Çıktı: A Çekirdekteyken kendi yarısında geçen süre payı, ileri/geri/yana pas sayıları, kararlarda tut/sür/pas payı, gönderiş, sayı.
// Kullanım: node tools/pasiflik.js <motor> <tempo> <risk> <i> [tik]
const { load } = require('./load'); const [dir, tS, rS, iS, lS] = process.argv.slice(2), i = +iS, len = +lS || 2700, seed = 3000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, sw = i % 2 === 1, A = sw ? 1 : 0, tacA = { tempo: +tS, risk: +rS };
const m = M.createMatch(seed, { tac: sw ? [{}, tacA] : [tacA, {}], len }); const dir0 = A === 0 ? 1 : -1, adv = x => (x - (A === 0 ? 0 : 100)) * dir0;
let own = 0, have = 0, fwd = 0, back = 0, side = 0, lastFl = null, lastDec = null; const dec = {};
while (!m.over && m.tick < len) { M.step(m); const h = m.holder;
  if (h && h.team === A) { have++; if (adv(h.x) < 50) own++; }
  if (m.fl && m.fl !== lastFl && m.fl.t0 === m.tick && m.fl.kind !== 'gönder') { lastFl = m.fl; const p = m.ball && m.ball.from; if (p && p.team === A) { const to = m.fl.to || m.fl.end, d = adv(to.x) - adv(p.x); if (d > 3) fwd++; else if (d < -3) back++; else side++; } }
  if (m.lastDec && m.lastDec !== lastDec) { lastDec = m.lastDec; if (lastDec.team === A) { const k = lastDec.opts[0] ? lastDec.opts[0].kind.split(' ')[0] : '?'; dec[k] = (dec[k] || 0) + 1; } } }
console.log(JSON.stringify({ dir: dir.split('/').slice(-1)[0], tempo: +tS, risk: +rS, i, own: have ? own / have : null, have, fwd, back, side, dec, shot: m.st.shot[A], shotB: m.st.shot[1 - A], g: m.score[A], gB: m.score[1 - A] }));
