// v4 ölçümü: aynı maç (tohum, taktik, özellikler) beyin kapalı (refleks) ya da açık oynanır.
// Kullanım: node tools/v4-olc.js <motor> <i> <kip: refleks|beyin> [ayar JSON, örn. '{"K":3,"savunma":false}']   (AYNA=1: iki takım Dengeli, herkes 10 · TERS=1: takımlar yer değiştirir)
const fs = require('fs'), { load } = require('./load');
const [dir, iS, kip, ayar] = process.argv.slice(2), i = +iS, seed = 130000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, D = g.AlanDecide.D, B = g.AlanBeyin;
(0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8')); D.useVS = true; D._usePT = true;
B.A.bak = kip === 'beyin';
// hazır ayarlar: kosaz = sadece top sahibi, gürültüsüz · hepsiz = hepsi, gürültüsüz · tam = hepsi + Okuma (varsayılan) · ya da JSON
const HAZIR = { kosaz: { savunma: false, kovala: false, algi: 0, t0: .001, tOk: 0 }, hepsiz: { algi: 0, t0: .001, tOk: 0 }, tam: {}, orta: {}, hafif: { hafif: true }, hafifS: { hafif: true, SP: 60 }, hizli: { H: 20, HMAX: 60, SP: 45 }, ichafif: { icHafif: true }, eski: { algi: .12, t0: .02, tOk: .006 } };
if (ayar) Object.assign(B.A, HAZIR[ayar] || JSON.parse(ayar));
let r0 = seed ^ 0x9e3779b9; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296, pick = a => a[Math.floor(R() * a.length)], ri = (a, b) => a + Math.floor(R() * (b - a + 1));
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
const AYNA = !!process.env.AYNA; let A = AYNA ? 'Dengeli' : pick(Object.keys(STY)), Bn = AYNA ? 'Dengeli' : pick(Object.keys(STY)); const KEYS = ['okuma', 'aktarim', 'tutus', 'kesme', 'yogunluk', 'hiz', 'cesaret', 'surme'];
const at = [[], []]; for (let t = 0; t < 2; t++) for (let k = 0; k < 7; k++) at[t].push(KEYS.map(() => AYNA ? 10 : ri(6, 16)));
// OKU=a,b: takımların bütün oyuncularının Okuma'sı (diğer özellikler 10)
if (process.env.OKU) { const o = process.env.OKU.split(',').map(Number); for (let t = 0; t < 2; t++) for (const r of at[t]) r[0] = o[t]; }
const TERS = !!process.env.TERS; if (TERS) { [A, Bn] = [Bn, A]; at.reverse(); }
const m = M.createMatch(seed, { tac: [tacOf(STY[A]), tacOf(STY[Bn])], len: 5400 }); m.ps.forEach(p => KEYS.forEach((k, j) => p.a[k] = at[p.team][p.i ?? (p.id % 7)][j]));
const t00 = Date.now(), pas = []; let ball = null, own = 0, pos = 0, plans = { karar: 0, kovala: 0, savunma: 0 }, rolls = 0;
while (!m.over && m.tick < 5400) { const p = B.planla(m); if (p) { plans[p.tip]++; rolls += p.jobs.length; p.bitir(p.jobs.map(j => B.rollout(m, j))); } M.step(m);
  if (m.ball && m.ball !== ball && m.ball.from && !m.ball.defl && m.ball.recv) pas.push({ team: m.ball.team, vx: m.ball.vx, first: null }); ball = m.ball;
  if (m.holder) { for (const q of pas) if (q.first == null) q.first = m.holder.team === q.team ? 1 : 0; pos++; if ((m.holder.team === 0 ? m.holder.x : 100 - m.holder.x) < 50) own++; } }
const done = pas.filter(p => p.first != null), mean = f => done.length ? done.reduce((a, p) => a + f(p), 0) / done.length : 0;
console.log(JSON.stringify({ i, kip, oku: process.env.OKU || null, ters: TERS, tac: [A, Bn], score: m.score, sn: +((Date.now() - t00) / 1000).toFixed(1), pas: done.length, tutma: +mean(p => p.first).toFixed(3), ileri: +mean(p => (p.team === 0 ? p.vx : -p.vx) > 0 ? 1 : 0).toFixed(3), kendiYari: +(own / pos).toFixed(3), plans, rolls }));
