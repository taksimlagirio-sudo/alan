// Beyin prototipi. Tek beyin, oyun onun dünyasında (motorun kendi fiziği) oynanır. Top sahibinin her kararında beyin,
// en umutlu K aday hamleyi kendi dünyasını ileri oynatarak tartar ve en iyisini seçer. İleri oynatmada maçın gerçek zarları
// kullanılmaz (ayrı zar), içerideki herkes motorun kendi kurallarıyla hareket eder. Motor dosyalarına dokunulmaz.
// Ufuk: en az H tik, sonra top birinin eline geçene kadar (en çok HMAX). Sonda: sayı ±1, top bizde +V, rakipte −V (durum değeri tablosu), boşta 0.
// Kip: temel = motor olduğu gibi · beyin = ileri oynatarak karar
// Kullanım: node tools/beyin.js <motor> <i> <kip> [K=3] [R=1] [H=30] [HMAX=90]   (AYNA=1: iki takım Dengeli, herkes 10)
const fs = require('fs'), { load } = require('./load');
const [dir, iS, kip, KS, RS, HS, HMS] = process.argv.slice(2), i = +iS, K = +KS || 3, RR = +RS || 1, H = +HS || 30, HMAX = +HMS || 90, seed = 110000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, AD = g.AlanDecide, D = AD.D, S = g.AlanState;
(0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8')); D.useVS = true; D._usePT = true;
let r0 = seed ^ 0x9e3779b9; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296, pick = a => a[Math.floor(R() * a.length)], ri = (a, b) => a + Math.floor(R() * (b - a + 1));
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
const AYNA = !!process.env.AYNA, A = AYNA ? 'Dengeli' : pick(Object.keys(STY)), B = AYNA ? 'Dengeli' : pick(Object.keys(STY)), KEYS = ['okuma', 'aktarim', 'tutus', 'kesme', 'yogunluk', 'hiz', 'cesaret', 'surme'];
const m = M.createMatch(seed, { tac: [tacOf(STY[A]), tacOf(STY[B])], len: 5400 }); for (const p of m.ps) for (const k of KEYS) p.a[k] = AYNA ? 10 : ri(6, 16);
// ── karar kancası: yakalama (adayları öğren) ve zorlama (seçilen hamleyi oynat)
const desc = o => ({ kind: o.kind, q: o.q ? o.q.id : null, to: o.to ? { x: o.to.x, y: o.to.y } : null, shot: o.shot || null, v: o.v });
const same = (o, d) => o.kind === d.kind && (o.q ? o.q.id : null) === d.q && (o.shot || null) === d.shot && (!d.to || (o.to && Math.hypot(o.to.x - d.to.x, o.to.y - d.to.y) < .6));
let CAP = null, FORCE = null; const dec0 = AD.decide;
AD.decide = function (h, src, ...rest) { const r = dec0.call(this, h, src, ...rest);
  if (CAP && CAP.hid === h.id && !CAP.opts) CAP.opts = r.opts.filter(o => o.v != null).map(desc);
  if (FORCE && FORCE.hid === h.id && !FORCE.used) { FORCE.used = true; const f = r.opts.find(o => same(o, FORCE.d)); FORCE.hit = !!f; if (f) return { best: f, opts: r.opts }; }
  return r; };
// ── uç değer
function endVal(c, team, s0) { const ds = (c.score[team] - s0[team]) - (c.score[1 - team] - s0[1 - team]); if (ds) return Math.sign(ds);
  const h = c.holder; if (!h) return 0; const tr = c.winTeam === h.team && (c.tick - (c.winT ?? -1e9)) < 180, v = window.ALAN_VS.v[S.idx(c.ps, h.team, h.x, h.y, tr)] || 0; return h.team === team ? v : -v; }
function rollout(snap, hid, d, team, salt) { const c = M.cloneMatch(snap), s0 = c.score.slice(); c.r.s = (salt * 2654435761 + c.tick * 40503) | 0; const h = c.ps.find(p => p.id === hid); if (h) h._plan = null;
  FORCE = { hid, d, used: false }; let t = 0; for (; t < HMAX; t++) { M.step(c); if (c.over) break; if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) break; if (t >= H && c.holder) break; } FORCE = null; return endVal(c, team, s0); }
const stat = { plans: 0, rolls: 0, changed: 0, miss: 0, ms: 0 };
function plan(m) { const h = m.holder, t0 = Date.now(); const c0 = M.cloneMatch(m); CAP = { hid: h.id, opts: null }; M.step(c0); const opts = CAP.opts; CAP = null; if (!opts || !opts.length) return null;
  const cand = []; for (const d of opts.sort((a, b) => b.v - a.v)) { if (!cand.some(x => same({ kind: x.kind, q: x.q != null ? { id: x.q } : null, to: x.to, shot: x.shot }, d))) cand.push(d); if (cand.length >= K) break; }
  let best = null, bv = -1e9; for (let k = 0; k < cand.length; k++) { let s = 0; for (let r = 0; r < RR; r++) { s += rollout(m, h.id, cand[k], h.team, 1 + k * 31 + r * 977); stat.rolls++; } s /= RR; cand[k].rv = s; if (s > bv) { bv = s; best = cand[k]; } }
  stat.plans++; if (best !== cand[0]) stat.changed++; stat.ms += Date.now() - t0; return best; }
// ── maç + ölçüler (pas: ilk sahip; kendi yarıda; ileri pas)
const t00 = Date.now(), pas = []; let ball = null, own = [0, 0], posT = [0, 0];
while (!m.over && m.tick < 5400) {
  if (kip === 'beyin' && m.holder && m.decT <= 1 && !m.holder.stun) { const d = plan(m); if (d) { m.holder._plan = null; FORCE = { hid: m.holder.id, d, used: false }; } }
  M.step(m); if (FORCE) { if (!FORCE.hit) stat.miss++; FORCE = null; }
  if (m.ball && m.ball !== ball && m.ball.from && !m.ball.defl && m.ball.recv) pas.push({ t: m.tick, team: m.ball.team, fx: m.ball.from.x, vx: m.ball.vx, first: null });
  ball = m.ball;
  for (const p of pas) if (p.first == null && m.holder) p.first = m.holder.team === p.team ? 1 : 0;
  if (m.holder) { const tm = m.holder.team; posT[tm]++; if ((tm === 0 ? m.holder.x : 100 - m.holder.x) < 50) own[tm]++; } }
const done = pas.filter(p => p.first != null), fwd = done.filter(p => (p.team === 0 ? p.vx : -p.vx) > 0);
console.log(JSON.stringify({ i, kip, tac: [A, B], score: m.score, sn: +((Date.now() - t00) / 1000).toFixed(1), pas: done.length, tutma: +(done.reduce((a, p) => a + p.first, 0) / done.length).toFixed(3), ileri: +(fwd.length / done.length).toFixed(3), kendiYari: +((own[0] + own[1]) / (posT[0] + posT[1])).toFixed(3), ...(kip === 'beyin' ? { plan: stat.plans, degisti: +(stat.changed / stat.plans).toFixed(3), eslesmedi: stat.miss, kararMs: +(stat.ms / stat.plans).toFixed(0) } : {}) }));
