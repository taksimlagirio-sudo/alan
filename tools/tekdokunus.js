// Tek dokunuş sayımı: kaç alışta tek dokunuş düşünüldü, kaçında seçildi (Çekirdek alınır alınmaz aynı tikte yeniden yola çıktıysa).
// Kullanım: node tools/tekdokunus.js <motor> <tohum> [tik]
const { load } = require('./load'); const [dir, sS, lS] = process.argv.slice(2), seed = +sS, len = +lS || 5400;
const g = load(dir, seed), M = g.AlanMatch, D = g.AlanDecide; let tried = 0; const od = D.decideOT; D.decideOT = (...a) => { tried++; return od(...a); };
const tac = [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Düşük', genislik: 'Dar', tempo: .8, risk: .8, kazaninca: 'Kontra' }, { sistem: 'Adam adama', pres: 2, arkada: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Dengeli' }];
const m = M.createMatch(seed, { tac, len }); let last = null, prevH = null, launches = 0, ot = 0;
while (!m.over && m.tick < len) { M.step(m); if (m.fl && m.fl !== last && m.fl.t0 === m.tick) { last = m.fl; launches++; if (m.ball && m.ball.from && m.ball.from !== prevH) ot++; } if (m.holder) prevH = m.holder; }
console.log(JSON.stringify({ dir, seed, launches, tried, ot, score: m.score }));
