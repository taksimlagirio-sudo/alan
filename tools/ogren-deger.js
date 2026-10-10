// Hafıza değerlendirmesi: verilen tablo kopyasıyla (normal okuma, Herkes 10, karışık taktikler) maç oynatır; uzaklığa göre gönderme sayısı ve sonucu.
// Kullanım: node tools/ogren-deger.js <motor> <tablo.json> <i>
const fs = require('fs'), { load } = require('./load');
const [dir, tp, iS] = process.argv.slice(2), i = +iS, seed = 190000 + i * 7919;
const g = load(dir, seed); (0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8'));
const tb = JSON.parse(fs.readFileSync(tp)); for (let k = 0; k < tb.v.length; k++) g.ALAN_VS.v[k] = tb.v[k];
const M = g.AlanMatch, B = g.AlanBeyin, ks = ['Dengeli', 'Sabırlı', 'Dikine', 'Kontra', 'Ön alan', 'Kuyu önü'];
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
const m = M.createMatch(seed, { len: 5400, tac: [tacOf(STY[ks[i % 6]]), tacOf(STY[ks[(i * 5 + 2) % 6]])] }); const sh = [];
while (!m.over) { let p = B.planla(m); while (p) p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; const sc = m.score.slice(); M.step(m);
  if (m.fl && m.fl.t0 === m.tick && m.fl.kind === 'gönder' && m.ball) { const gx = m.ball.team === 0 ? 100 : 0; sh.push({ uz: Math.hypot(gx - m.ball.x, 25 - m.ball.y), team: m.ball.team, sayi: 0 }); }
  if ((m.score[0] !== sc[0] || m.score[1] !== sc[1]) && sh.length) { const s = sh[sh.length - 1]; if (m.tick - 0 >= 0 && m.score[s.team] !== sc[s.team]) s.sayi = 1; } }
console.log(JSON.stringify({ i, tablo: tp, skor: m.score, sh }));
