// Kural ölçümü · ölü pas ayrıştırması. Gerçek maçta atılan paslar vuruştan hemen sonra cloneMatch ile kopyalanır.
// Önce "temel" kolda (motor olduğu gibi) N kez oynatılır; kesin ölü (tutma ≤ %10) olanlar, savunmanın tek bir davranışı kapatılarak yeniden oynatılır.
// Hangi kapatma pası canlandırıyorsa, pası atanın hesabının göremediği şey odur. Motor dosyalarına dokunulmaz; shape.js yalnızca bu süreçte, bayrakla açılıp kapanan küçük yamalarla yüklenir.
// Kollar:  tazele = kovalamacı hedefini pas boyunca güncellemez (ilk okuma sabit)
//          ikinci = "ben de yetişirim" payı kapalı: sadece en erken varacağını düşünen kovalar
//          kayma  = savunma dizilişi topun varacağı yere değil, topun o anki yerine göre kayar
//          ongoru = rakip hareketini öngörme kapalı (S.look = 0)
//          hepsi  = dördü birden
//          donuk  = rakipler pas anındaki yerlerinde donar · kovalayan = sadece o tik topu kovalayan rakipler hareket eder, diğerleri donar · yerlesen = kovalayanlar donar, diğerleri (yerleşim, pres, geri koşu) hareket eder
//          Her kolda bayrağın gerçekten davranışı değiştirdiği, temel kolla oyuncu konumlarının ayrışma tikiyle (ayr) gösterilir.
// Kullanım: node tools/ayir-pas.js <motor> <i> <vs tablo.js> <pas tablo.js> [N=10] [her kaçıncı pas=2] [en çok pas=25]
const fs = require('fs'), path = require('path');
const [dir, iS, vsPath, ptPath, nS, evS, mxS] = process.argv.slice(2), i = +iS, N = +nS || 10, EV = +evS || 2, MX = +mxS || 25, seed = 90000 + i * 7919;
// yükleyici (tools/load.js ile aynı, shape.js yamalı)
function rngF(s) { let a = s >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rngF(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
const PATCH = [['sh.frT > m.tick - 3', 'sh.frT > m.tick - (globalThis.__AY_FR ? 1e9 : 3)'],
  ['const f = fr.arr.get(fr.first); E = { x: f.x, y: f.y };', 'const f = fr.arr.get(fr.first); E = globalThis.__AY_E ? { x: m.ball.x, y: m.ball.y } : { x: f.x, y: f.y };'],
  ['r.est <= bestE + (20 - ok(p)) * S.marginT + 1', 'r.est <= bestE + (globalThis.__AY_M ? 0 : (20 - ok(p)) * S.marginT + 1)']];
for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) { let src = fs.readFileSync(path.join(dir, f), 'utf8');
  if (f === 'shape.js') for (const [a, b] of PATCH) { if (!src.includes(a)) throw new Error('yama bulunamadı: ' + a); src = src.replace(a, b); }
  (0, eval)(src + '\n//# sourceURL=' + f); }
const M = globalThis.AlanMatch, AD = globalThis.AlanDecide, D = AD.D, SS = globalThis.AlanShape.S, LOOK0 = SS.look;
if (vsPath) { (0, eval)(fs.readFileSync(vsPath, 'utf8')); D.useVS = true; } if (ptPath) { (0, eval)(fs.readFileSync(ptPath, 'utf8')); D._usePT = true; }
let r0 = seed ^ 0x9e3779b9; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296, pick = a => a[Math.floor(R() * a.length)], ri = (a, b) => a + Math.floor(R() * (b - a + 1));
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
const AYNA = !!process.env.AYNA, A = AYNA ? 'Dengeli' : pick(Object.keys(STY)), B = AYNA ? 'Dengeli' : pick(Object.keys(STY)), KEYS = ['okuma', 'aktarim', 'tutus', 'kesme', 'yogunluk', 'hiz', 'cesaret', 'surme'];
const m = M.createMatch(seed, { tac: [tacOf(STY[A]), tacOf(STY[B])], len: 5400 }); for (const p of m.ps) for (const k of KEYS) p.a[k] = AYNA ? 10 : ri(6, 16);
const PASSK = new Set(['pas', 'önüne', 'aşırt', 'kenardan']), last = new Map(), dec0 = AD.decide;
AD.decide = function (h, src, ...rest) { const r = dec0.call(this, h, src, ...rest); if (r && r.best && PASSK.has(r.best.kind) && !globalThis.__AY_IN) last.set(h.id, { t: m.tick, kind: r.best.kind }); return r; };
const ARMS = { tazele: { FR: 1 }, ikinci: { M: 1 }, kayma: { E: 1 }, ongoru: { L: 1 }, hepsi: { FR: 1, M: 1, E: 1, L: 1 }, donuk: { FZ: 1 }, kovalayan: { FZ: 1, KEEP: 'kovala' }, yerlesen: { FZ: 1, KEEP: 'yerles' } };
function setArm(a) { globalThis.__AY_FR = !!(a && a.FR); globalThis.__AY_M = !!(a && a.M); globalThis.__AY_E = !!(a && a.E); SS.look = a && a.L ? 0 : LOOK0; }
let ARM = null;
function outcome(c, team) { const s0 = c.score.slice(), opp = c.ps.filter(p => p.team !== team), P0 = opp.map(p => [p.x, p.y]); if (ARM && ARM.GH) for (const p of opp) p.noTouch = true;
  for (let t = 0; t < 300; t++) { M.step(c); if (ARM && ARM.FZ) opp.forEach((p, k) => { const kov = p.job === 'kovala'; if (ARM.KEEP === 'kovala' && kov || ARM.KEEP === 'yerles' && !kov) { P0[k] = [p.x, p.y]; return; } p.x = P0[k][0]; p.y = P0[k][1]; p.vx = 0; p.vy = 0; }); if (ARM && ARM.GH) for (const p of opp) p.noTouch = true; if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) return c.score[team] !== s0[team] ? 1 : 0; if (c.holder) return c.holder.team === team ? 1 : 0; if (c.over) break; } return null; }
function arm(snap, team, k, a) { let y = 0, n = 0; globalThis.__AY_IN = true; setArm(a); ARM = a;
  for (let r = 0; r < N; r++) { const c = M.cloneMatch(snap); c.r.s = (k * 1000003 + r * 7919 + 12345) | 0; const o = outcome(c, team); if (o != null) { y += o; n++; } }
  setArm(null); ARM = null; globalThis.__AY_IN = false; return n ? +(y / n).toFixed(2) : null; }
function split(snap, team, a) { const c1 = M.cloneMatch(snap), c2 = M.cloneMatch(snap); c1.r.s = c2.r.s = 777; globalThis.__AY_IN = true;
  for (let t = 0; t < 120; t++) { setArm(null); ARM = null; M.step(c1); setArm(a); ARM = a; M.step(c2); let dd = 0; for (let j = 0; j < c1.ps.length; j++) dd = Math.max(dd, Math.abs(c1.ps[j].x - c2.ps[j].x) + Math.abs(c1.ps[j].y - c2.ps[j].y)); if (dd > .01) { setArm(null); ARM = null; globalThis.__AY_IN = false; return t + 1; } }
  setArm(null); ARM = null; globalThis.__AY_IN = false; return null; }
const out = []; let ball = null, cnt = 0;
while (!m.over && m.tick < 5400 && out.length < MX) { M.step(m);
  if (m.ball && m.ball !== ball && m.ball.from && !m.ball.defl) { const h = m.ball.from, d = last.get(h.id); if (d && m.tick - d.t < 40) { last.delete(h.id);
      if (cnt++ % EV === 0) { const snap = M.cloneMatch(m), team = h.team, base = arm(snap, team, 1, null), row = { t: m.tick, kind: d.kind, base };
        if (base != null && base <= .1) { let k = 2; row.ayr = {}; for (const [nm, a] of Object.entries(ARMS)) { row[nm] = arm(snap, team, k++, a); row.ayr[nm] = split(snap, team, a); } }
        out.push(row); } } }
  ball = m.ball; }
console.log(JSON.stringify({ i, tac: [A, B], p: out }));
