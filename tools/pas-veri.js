// Tutma tablosu (Bölüm 10) · veri üretimi. Bir maç oynatır; atılan her pasın kararda gördüğü ölçüleri (AlanPass.feat, kararın kendi dünyasıyla)
// ve sonucunu kaydeder. Kurallar vs-veri.js ile aynı (taktik karışımı, özellikler 6–16, karar/algı kanallarında kusursuz okuma) + keşif (D._explore).
// Sonuç: ilk sahip (top kimin oyuncusunda ilk durdu), alıcı/takım 60 tik tuttu mu.
// Kullanım: node tools/pas-veri.js <motor> <i> <tur> <değer tablosu.js> [keşif=0.1] → JSON satırı
const fs = require('fs'), { load } = require('./load');
const [dir, iS, turS, tabPath, exS] = process.argv.slice(2), i = +iS, tur = +turS || 0, seed = 50000 + i * 7919 + tur * 1000003, EXPL = exS != null ? +exS : .1;
let r0 = seed ^ 0x9e3779b9; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296, pick = a => a[Math.floor(R() * a.length)], ri = (a, b) => a + Math.floor(R() * (b - a + 1));
globalThis.ALAN_OKX = { eq: ['karar', 'kararHizi', 'rakipModel', 'gonder', 'kontrol', 'kenar', 'algi', 'yerlesim'], mid: 20 };
const g = load(dir, seed), M = g.AlanMatch, AD = g.AlanDecide, D = AD.D, AP = g.AlanPass;
if (tabPath) { (0, eval)(fs.readFileSync(tabPath, 'utf8')); D.useVS = true; }
D._explore = EXPL;
// PT=tablo.js: tutma tablosu kararda açık (Bölüm 10)
if (process.env.PT) { (0, eval)(fs.readFileSync(process.env.PT, 'utf8')); D._usePT = true; }
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const BL = ['Düşük', 'Orta', 'Yüksek'], SIS = ['Alan', 'Adam adama', 'Kenara sıkıştır'], KOM = { Alan: 'Kenara sıkıştır', 'Adam adama': 'Kenara sıkıştır', 'Kenara sıkıştır': null };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
function core() { const n = pick(Object.keys(STY)), t = tacOf(STY[n]); t.pres = Math.max(0, Math.min(3, t.pres + ri(-1, 1))); t.blok = BL[Math.max(0, Math.min(2, BL.indexOf(t.blok) + ri(-1, 1)))];
  if (R() < 1 / 3) t.sistem = KOM[t.sistem] || pick(['Alan', 'Adam adama']); return { ad: n, t }; }
function rnd() { return { ad: 'rastgele', t: { sistem: pick(SIS), pres: ri(0, 3), arkada: ri(0, 2), blok: pick(BL), genislik: pick(['Dar', 'Normal', 'Geniş']), tempo: pick([.2, .5, .8]), risk: pick([.2, .5, .8]), kazaninca: pick(['Yerleş', 'Dengeli', 'Kontra']) } }; }
let A, B; const u = R(); if (u < .8) { A = core(); B = core(); } else if (u < .97) { A = rnd(); B = core(); if (R() < .5) [A, B] = [B, A]; } else { A = rnd(); B = rnd(); }
const KEYS = ['okuma', 'aktarim', 'tutus', 'kesme', 'yogunluk', 'hiz', 'cesaret', 'surme'];
const m = M.createMatch(seed, { tac: [A.t, B.t], len: 5400 });
for (const p of m.ps) for (const k of KEYS) p.a[k] = ri(6, 16);
const PASSK = new Set(['pas', 'önüne', 'aşırt', 'kenardan']), r3 = x => x == null || !Number.isFinite(x) ? null : Math.round(x * 1000) / 1000;
// ek ölçü (teşhis): race2 = alıcının topu karşılayacağı noktaya kadar olan yolda yarış payı (alıcı tepkisiz koşar, maçtaki recvPoint kuralı);
// rF = race'i belirleyen noktanın yoldaki yeri (0–1), rq = alıcının topla buluştuğu tik
const K = g.AlanCore, hyp = K.hyp;
function race2(h, src, o, T) { const Q = K.Q, cs = AD.coreSide(h), org = { x: h.x + cs.ux * Q.body, y: h.y + cs.uy * Q.body }, b = K.makeBall({ x: org.x, y: org.y, vx: o.launch.vx, vy: o.launch.vy, team: h.team, ch: .5, from: h, recv: o.q || null }), pts = K.predict(b, src, null, 200).pts;
  const q = o.q ? src.find(p => p.id === o.q.id) || o.q : null; let j = T; if (q) { const v = (.17 + ((q.a && q.a.hiz) ?? 10) * .006) * 1.25; for (let i = 1; i < pts.length; i++) if (Math.max(0, hyp(pts[i].x - q.x, pts[i].y - q.y) - 1.4) / v <= i) { j = i; break; } }
  let r2 = 1e9, r1 = 1e9, iw = 0; for (const p of src) { if (p.team === h.team) continue; const rc = Math.max(0, 14 - self.ALAN_OKC(p, 'tepki') * .5), v = (.17 + ((p.a && p.a.hiz) ?? 10) * .006) * 1.25;
    for (let i = 1; i <= T && i < pts.length; i++) { const df = rc + Math.max(0, hyp(pts[i].x - p.x, pts[i].y - p.y) - Q.reach) / v - i; if (df < r1) { r1 = df; iw = i; } if (i <= j && df < r2) r2 = df; } }
  return { race2: r2, rF: T ? iw / T : null, rq: j }; }
// her oyuncunun son kararı: pas seçtiyse ölçüleri kararın gördüğü dünyayla, o anda
const last = new Map(), dec0 = AD.decide;
AD.decide = function (h, src, ...rest) { const r = dec0.call(this, h, src, ...rest), o = r && r.best;
  if (o && PASSK.has(o.kind) && o.launch) { let f = null; try { f = AP.feat(h, src, o); const c2 = race2(h, src, o, f.T); f.race2c = c2.race2; } catch (e) { f = { err: String(e).slice(0, 80) }; }
    last.set(h.id, { t: m.tick, kind: o.kind, ex: !!o._explored, P: r3(o.ex ? o.ex.P : o.det && o.det.P), Pr: r3(o.det && o.det.P), f, q: o.q ? o.q.id : null }); }
  else last.delete(h.id); return r; };
const out = [], open = []; let ball = null, sc = [0, 0];
while (!m.over && m.tick < 5400) { M.step(m);
  const goal = m.score[0] !== sc[0] || m.score[1] !== sc[1]; if (goal) sc = m.score.slice();
  // yeni pas: yeni Çekirdek nesnesi, atan oyuncunun kararı pas
  if (m.ball && m.ball !== ball && m.ball.from && !m.ball.defl) { const h = m.ball.from, d = last.get(h.id); if (d && m.tick - d.t < 40) { last.delete(h.id); const f = d.f || {};
      open.push({ t: m.tick, dt: m.tick - d.t, team: h.team, kind: d.kind, ex: d.ex, P: d.P, Pr: d.Pr, q: d.q, race: r3(f.race), recvOpp: r3(f.recvOpp), arrV: r3(f.arrV), len: r3(f.len), T: f.T, laneD: r3(f.laneD), laneF: r3(f.laneF), race2: r3(f.race2), race2c: r3(f.race2c), rF: r3(f.rF), rq: f.rq, raceRole: f.raceId != null ? (m.ps.find(p => p.id === f.raceId) || {}).role : null, err: f.err, first: null, who: null, keep: null, sc: m.score[h.team] }); } }
  ball = m.ball;
  for (const p of open) { if (p.first == null) { if (goal) { p.first = 'sayı'; p.keep = 0; } else if (m.holder) { p.first = m.holder.team === p.team ? 'biz' : 'rakip'; p.who = m.holder.id === p.q ? 'alıcı' : m.holder.team === p.team ? 'arkadaş' : 'rakip'; p.tF = m.tick; } else if (m.tick - p.t > 400) { p.first = 'yok'; p.keep = 0; } }
    else if (p.keep == null) { if (p.first !== 'biz') p.keep = 0; else if (goal) p.keep = m.score[p.team] > (p.sc ?? 0) ? 1 : 0; else if (m.holder && m.holder.team !== p.team) p.keep = 0; else if (m.tick - p.tF >= 60) p.keep = 1; } }
  for (let k = open.length - 1; k >= 0; k--) if (open[k].first != null && open[k].keep != null) out.push(open.splice(k, 1)[0]); }
for (const p of open) { if (p.keep == null) p.keep = p.first === 'biz' ? null : 0; out.push(p); }
out.sort((a, b) => a.t - b.t);
console.log(JSON.stringify({ i, tur, tac: [A, B], score: m.score, p: out }));
