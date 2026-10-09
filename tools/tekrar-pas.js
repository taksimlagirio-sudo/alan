// Kural ölçümü · pas tekrarı. Gerçek bir maçta atılan pasın hemen sonrası cloneMatch ile kopyalanır ve farklı zarlarla yeniden oynatılır.
// Aynı durum aynı sonucu veriyor mu (zar mı belirliyor, durum mu)? Alıcının Tutuş'u sonucu ne kadar değiştiriyor?
// Kollar (her biri N tekrar): hepsi = bütün zarlar serbest · fizik = oyuncu kararları sabit (aynı durumda aynı karar), sadece kural zarları serbest
//                           · once = vuruştan bir tik önceki kopyadan, kararlar sabit: vuruş hatası dahil kural zarları
//                           · tut6 / tut16 = bütün zarlar serbest, alıcının Tutuş'u 6 ya da 16.
// Sonuç: ilk sahip pası atan takımdan mı (sayı olursa atan takım lehine sayılır), 300 tik içinde sahip yoksa 'yok'.
// Kullanım: node tools/tekrar-pas.js <motor> <i> <vs tablo.js> <pas tablo.js> [N=20] [her kaçıncı pas=4] [en çok pas=20]
const fs = require('fs'), { load } = require('./load');
const [dir, iS, vsPath, ptPath, nS, evS, mxS] = process.argv.slice(2), i = +iS, N = +nS || 20, EV = +evS || 4, MX = +mxS || 20, seed = 70000 + i * 7919;
let r0 = seed ^ 0x9e3779b9; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296, pick = a => a[Math.floor(R() * a.length)], ri = (a, b) => a + Math.floor(R() * (b - a + 1));
const g = load(dir, seed), M = g.AlanMatch, AD = g.AlanDecide, D = AD.D;
if (vsPath) { (0, eval)(fs.readFileSync(vsPath, 'utf8')); D.useVS = true; } if (ptPath) { (0, eval)(fs.readFileSync(ptPath, 'utf8')); D._usePT = true; }
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
const A = { ad: pick(Object.keys(STY)) }, B = { ad: pick(Object.keys(STY)) }; A.t = tacOf(STY[A.ad]); B.t = tacOf(STY[B.ad]);
const KEYS = ['okuma', 'aktarim', 'tutus', 'kesme', 'yogunluk', 'hiz', 'cesaret', 'surme'];
const m = M.createMatch(seed, { tac: [A.t, B.t], len: 5400 });
for (const p of m.ps) for (const k of KEYS) p.a[k] = ri(6, 16);
// karar sabitleme: 'fizik' kolunda kararlar maçın zarı yerine (tik, oyuncu) ile tohumlanan ayrı bir akıştan beslenir; aynı durumda aynı karar
let FIX = 0; const sub = (t, id) => M.rng(((FIX * 2654435761) ^ (t * 40503) ^ (id * 9973)) | 0);
const dec0 = AD.decide, ot0 = AD.decideOT, PASSK = new Set(['pas', 'önüne', 'aşırt', 'kenardan']); let lastPass = new Map(), cur = null;
AD.decide = function (h, src, ch, rnd, ...rest) { if (FIX && cur) rnd = sub(cur.tick, h.id); const r = dec0.call(this, h, src, ch, rnd, ...rest); if (!FIX && r && r.best && PASSK.has(r.best.kind)) lastPass.set(h.id, { t: m.tick, kind: r.best.kind, q: r.best.q ? r.best.q.id : null, to: r.best.to, P: r.best.ex ? r.best.ex.P : null }); return r; };
AD.decideOT = function (h, src, ch, rnd, ...rest) { if (FIX && cur) rnd = sub(cur.tick, h.id + 50); return ot0.call(this, h, src, ch, rnd, ...rest); };
let DET = null; // ayrıntı istenirse doldurulur
function outcome(c, team) { const s0 = c.score.slice(); cur = c; const Q = DET && DET.q != null ? c.ps.find(p => p.id === DET.q) : null; let qm = 1e9;
  for (let t = 0; t < 300; t++) { if (Q && c.ball) { const dd = Math.hypot(Q.x - c.ball.x, Q.y - c.ball.y); if (dd < qm) { qm = dd; const vr = Math.hypot(c.ball.vx - (Q.vx || 0), c.ball.vy - (Q.vy || 0)); Object.assign(DET, { qMin: +dd.toFixed(2), qT: t, qVr: +vr.toFixed(2), qBv: +Math.hypot(c.ball.vx, c.ball.vy).toFixed(2), qCh: c.ball.ch != null ? +(+c.ball.ch).toFixed(2) : null }); } } M.step(c); if (Q && DET && (t === 4 || t === 15)) { const to = DET.to; DET['q' + t] = { tx: Q.tx != null ? +Q.tx.toFixed(1) : null, ty: Q.ty != null ? +Q.ty.toFixed(1) : null, dTo: to && Q.tx != null ? +Math.hypot(Q.tx - to.x, Q.ty - to.y).toFixed(1) : null, dQ: to ? +Math.hypot(Q.x - to.x, Q.y - to.y).toFixed(1) : null, sp: +Math.hypot(Q.vx, Q.vy).toFixed(3), spr: !!Q.sprint }; } if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) { if (DET) DET.how = 'sayı'; return c.score[team] !== s0[team] ? 1 : 0; } if (c.holder) { if (DET && DET.pts) { const k = Math.min(DET.pts.length - 1, t + 1), pp = DET.pts[k], hb = c.holder; DET.pErr = +Math.hypot(pp.x - hb.x, pp.y - hb.y).toFixed(2); DET.pAlong = DET.ux != null ? +(((hb.x - DET.ox) * DET.ux + (hb.y - DET.oy) * DET.uy) - ((pp.x - DET.ox) * DET.ux + (pp.y - DET.oy) * DET.uy)).toFixed(2) : null; delete DET.pts; } if (DET) Object.assign(DET, { dt: t + 1, hid: c.holder.id, hrole: c.holder.role, hx: +c.holder.x.toFixed(1), hy: +c.holder.y.toFixed(1) }); return c.holder.team === team ? 1 : 0; } if (c.over) break; } return null; }
function arm(snap, team, k, opt) { let y = 0, n = 0; for (let r = 0; r < N; r++) { const c = M.cloneMatch(snap); c.r.s = (k * 1000003 + r * 7919 + 12345) | 0; FIX = opt.fix ? 1 : 0; // sabit: her tekrarda aynı karar akışı
    if (opt.tut != null) { const q = c.ps.find(p => p.id === opt.q); if (q) q.a.tutus = opt.tut; } const o = outcome(c, team); FIX = 0; cur = null; if (o != null) { y += o; n++; } } return n ? +(y / n).toFixed(3) : null; }
const out = []; let ball = null, cnt = 0;
let pre = null;
while (!m.over && m.tick < 5400 && out.length < MX) { pre = m.holder && m.decT <= 1 ? M.cloneMatch(m) : null; M.step(m);
  if (m.ball && m.ball !== ball && m.ball.from && !m.ball.defl) { const h = m.ball.from, d = lastPass.get(h.id); if (d && m.tick - d.t < 40) { lastPass.delete(h.id);
      if (cnt++ % EV === 0) { const snap = M.cloneMatch(m), team = h.team, q = d.q, aq = q != null ? (m.ps.find(p => p.id === q) || { a: {} }).a.tutus : null;
        const K0 = g.AlanCore, bb = snap.ball, LL = Math.hypot(bb.vx, bb.vy) || 1; DET = { q, to: d.to ? { x: d.to.x, y: d.to.y } : null, pts: K0.predict(K0.makeBall({ x: bb.x, y: bb.y, vx: bb.vx, vy: bb.vy, team: bb.team, ch: bb.ch, from: bb.from, recv: bb.recv }), snap.ps, null, 300).pts, ox: bb.x, oy: bb.y, ux: bb.vx / LL, uy: bb.vy / LL }; const real = outcome(M.cloneMatch(snap), team), det = DET; DET = null;
        const H = det.hid != null ? snap.ps.find(p => p.id === det.hid) : null, Q = q != null ? snap.ps.find(p => p.id === q) : null, b = snap.ball, f = (() => { try { return g.AlanPass.feat(h, m.ps, { kind: d.kind, to: d.to, q: Q, launch: { vx: b.vx, vy: b.vy } }); } catch (e) { return {}; } })();
        // kesen oyuncunun pas anındaki yeri: topun yoluna dik uzaklığı, yolun neresinde, hızı
        const L = Math.hypot(b.vx, b.vy) || 1, ux = b.vx / L, uy = b.vy / L, rel = H ? { al: +(((H.x - b.x) * ux + (H.y - b.y) * uy)).toFixed(1), pe: +(Math.abs(-(H.x - b.x) * uy + (H.y - b.y) * ux)).toFixed(1), v: +Math.hypot(H.vx || 0, H.vy || 0).toFixed(2), d0: +Math.hypot(H.x - b.x, H.y - b.y).toFixed(1) } : null;
        Object.assign(det, { rel, qd: Q ? +Math.hypot(Q.x - b.x, Q.y - b.y).toFixed(1) : null, race2: f.race2 != null ? +(+f.race2).toFixed(1) : null, raceId: f.raceId, recvOpp: f.recvOpp != null ? +(+f.recvOpp).toFixed(1) : null, T: f.T, bv: +L.toFixed(2) }); // aynı zarla kopya: gerçek sonuç (zar akışı aynı)
        out.push({ t: m.tick, Pk: d.P != null ? +d.P.toFixed(2) : null, kind: d.kind, team, q, tutus: aq, real, det, hepsi: arm(snap, team, 1, {}), ...(process.env.SADE ? {} : { fizik: arm(snap, team, 2, { fix: true }), once: pre && pre.tick === m.tick - 1 ? arm(pre, team, 5, { fix: true }) : null, tut6: q != null ? arm(snap, team, 3, { tut: 6, q }) : null, tut16: q != null ? arm(snap, team, 4, { tut: 16, q }) : null }) }); } } }
  ball = m.ball; }
console.log(JSON.stringify({ i, tac: [A.ad, B.ad], p: out }));
