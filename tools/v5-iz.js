// Birebir aynılık kontrolü: bir maç oynanır, her 60 tikte bütün oyuncuların konum ve hızları, Çekirdek, skor tam hassasiyetle (sayının bitleri) bir ize yazılır.
// Kullanım: node tools/v5-iz.js <motor> <tohum> [tik=5400]  → satır başına "tik özet" ve sonda süre
const { load } = require('./load'); const [dir, sS, lS] = process.argv.slice(2), seed = +sS, LEN = +lS || 5400; const g = load(dir, seed), M = g.AlanMatch;
const STY = [['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'], ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra']];
const tOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7], pasSurme: .5, sabir: .5, sut: .5, tetik: ['Geri pas', 'Kanatta'], kontraN: 3, kayip: .6, kpSure: 3 });
const m = M.createMatch(seed, { len: LEN, tac: [tOf(STY[seed % 4]), tOf(STY[(seed + 1) % 4])], diz: [{ sav: '2-2-2', huc: '2-2-2' }, { sav: '3-2-1', huc: '2-3-1' }] });
const f64 = new Float64Array(1), u32 = new Uint32Array(f64.buffer); let h = 0; const add = x => { f64[0] = x ?? -1; h = (Math.imul(h, 31) + u32[0]) | 0; h = (Math.imul(h, 31) + u32[1]) | 0; };
const t0 = Date.now(), out = [];
while (!m.over && m.tick < LEN) { M.step(m); for (const p of m.ps) { add(p.x); add(p.y); add(p.vx); add(p.vy); } if (m.ball) { add(m.ball.x); add(m.ball.y); add(m.ball.vx); add(m.ball.vy); } add(m.ch); add(m.score[0]); add(m.score[1]); if (m.tick % 60 === 0) out.push(m.tick + ' ' + (h >>> 0).toString(16)); }
console.log(out.join('\n')); console.error(`süre ${((Date.now() - t0) / 1000).toFixed(1)} sn · skor ${m.score}`);
