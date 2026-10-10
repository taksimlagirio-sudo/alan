// v5 genel ölçüm: bir maç oynar; sayı, gönderme (uzaklık, sayı), pas tutma, top el değişimi, ikili, süre, belirlenimcilik izi.
// Kullanım: node tools/v5-olc.js <motor> <i>
const { load } = require('./load'); const [dir, iS] = process.argv.slice(2), i = +iS, seed = 260000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch;
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] }), ks = Object.keys(STY), tA = ks[i % 6], tB = ks[(i * 5 + 2) % 6];
const m = M.createMatch(seed, { len: 5400, tac: [tacOf(STY[tA]), tacOf(STY[tB])], diz: [{ sav: '2-2-2', huc: '2-2-2' }, { sav: '3-2-1', huc: '2-3-1' }] });
const sh = [], ps = []; let fl = null, swaps = 0, hT = null, iz = 0; const t0 = Date.now();
while (!m.over && m.tick < 5400) { const sc = m.score.slice(); M.step(m);
  if (m.fl && m.fl !== fl && m.ball) { fl = m.fl; const b = m.ball, gx = b.team === 0 ? 100 : 0; if (fl.kind === 'gönder') sh.push({ uz: +Math.hypot(gx - b.x, 25 - b.y).toFixed(1), t: b.team, s: 0 }); else ps.push({ t: b.team, r: null }); }
  if (m.score[0] !== sc[0] || m.score[1] !== sc[1]) { const tm = m.score[0] !== sc[0] ? 0 : 1, s = sh[sh.length - 1]; if (s && s.t === tm) s.s = 1; }
  if (m.holder) { for (const p of ps) if (p.r == null) p.r = m.holder.team === p.t ? 1 : 0; if (hT != null && m.holder.team !== hT) swaps++; hT = m.holder.team; }
  if (m.tick % 300 === 0) iz = (Math.imul(iz, 31) + Math.round(m.ps.reduce((s, p) => s + p.x * 7 + p.y, 0) * 1000)) | 0; }
const P = ps.filter(p => p.r != null);
console.log(JSON.stringify({ i, tac: [tA, tB], skor: m.score, sn: +((Date.now() - t0) / 1000).toFixed(1), gonder: sh, pas: P.length, tutma: +(P.reduce((s, p) => s + p.r, 0) / P.length).toFixed(2), elDegisti: swaps, ikili: m.st.duel, iz }));
