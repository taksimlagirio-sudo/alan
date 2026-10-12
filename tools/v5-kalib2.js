// Pas tahmininin iyimserliğini ikiye ayırır. Seçilen pasın anı (pastan önceki tik) kopyalanır:
//   secim  : maçta seçilirken aklın tahmini (o.det.P, az denemeyle)
//   model  : aynı anda aynı hamle için aklın tahmini, çok denemeyle (D.trials = TN)  → secim − model = en iyiyi seçme yanılgısı
//   gercek : aynı an, hamle zorla aynı, gerçek maç kurallarıyla RN kez farklı zarla  → model − gercek = aklın hesabının hatası
// Kullanım: node tools/v5-kalib2.js <motor> <i> [örnekleme=4] [RN=20] [TN=30]
const fs = require('fs'), path = require('path'); const [dir, iS, oS, rS, tS] = process.argv.slice(2), i = +iS, ORN = +oS || 4, RN = +rS || 20, TN = +tS || 30, seed = 410000 + i * 7919;
globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) { let s = fs.readFileSync(path.join(dir, f), 'utf8');
  if (f === 'match.js') { const a = 'm.fl = { t0: m.tick, kind: o.kind,'; if (!s.includes(a)) throw new Error('yama yeri yok'); s = s.replace(a, 'm.fl = { o, t0: m.tick, kind: o.kind,'); }
  (0, eval)(s + '\n//# sourceURL=' + f); }
const M = AlanMatch, AD = AlanDecide, D = AD.D;
const mk = s => { const f = () => { f.s = (f.s + 0x6D2B79F5) | 0; let a = f.s, t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; f.s = s | 0; return f; };
const klon = (m, s) => { const r = m.r; m.r = null; let c; try { c = structuredClone(m); } finally { m.r = r; } c.r = mk(s == null ? r.s : s); return c; };
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const ks = Object.keys(STY), tOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7], pasSurme: .5, sabir: .5, sut: .5, tetik: ['Geri pas', 'Kanatta'], kontraN: 3, kayip: .6, kpSure: 3 });
const m = M.createMatch(seed, { len: 5400, tac: [tOf(STY[ks[i % 6]]), tOf(STY[ks[(i * 5 + 2) % 6]])], diz: [{ sav: '2-2-2', huc: '2-2-2' }, { sav: '3-2-1', huc: '2-3-1' }] }); m.r = mk(seed);
const ayni = (x, o) => x.kind === o.kind && !!x.kendine === !!o.kendine && (x.q ? x.q.id : null) === (o.q ? o.q.id : null) && (!o.to || (x.to && Math.hypot(x.to.x - o.to.x, x.to.y - o.to.y) < .5));
const mapO = (o, c) => { const byId = id => c.ps.find(p => p.id === id); const r = { ...o }; if (o.q) r.q = byId(o.q.id); return r; };
// zorla: bu kopyada taşıyıcının ilk kararı verilen hamle olur; ayrıca istenirse o andaki model tahmini (çok denemeyle) okunur
function zorlaAdim(c, hid, o, modelOku) { const od = AD.decide; let model = null, ilk = true;
  AD.decide = function (hh, ps, ch, rnd, tac) { if (ilk && hh.id === hid) { ilk = false; if (modelOku) { const tr = D.trials; D.trials = TN; try { const r = od.call(this, hh, ps, ch, mk(12345), tac), x = r.opts.find(x => ayni(x, o)); model = x && x.det ? x.det.P : null; } finally { D.trials = tr; } } const oo = mapO(o, c); return { best: oo, opts: [oo] }; } return od.apply(this, arguments); };
  try { M.step(c); } finally { AD.decide = od; } return model; }
function sonuc(c, team) { for (let t = 0; t < 400; t++) { const sc = c.score.slice(); M.step(c); if (c.score[0] !== sc[0] || c.score[1] !== sc[1]) return c.score[team] !== sc[team] ? 1 : 0; if (c.holder) return c.holder.team === team ? 1 : 0; if (c.over) return null; } return null; }
let once = null, fl = null, n = 0;
while (!m.over && m.tick < 5400) { once = m.holder ? klon(m) : null; M.step(m);
  if (m.fl && m.fl !== fl && m.fl.o) { fl = m.fl; const o = fl.o; if (o.kind === 'gönder' || o.kind === 'çalım' || !o.det || o.det.P == null || o.ot || !once) continue; if ((n++ % ORN) !== 0) continue;
    const hid = (o.kendine ? o.q : fl.o && m.ball.from).id, team = m.ball.team; let nd = 1e9; for (const p of m.ps) if (p.team !== team && p.role !== 'Bekçi') nd = Math.min(nd, Math.hypot(p.x - m.ball.x, p.y - m.ball.y));
    const c0 = klon(once, once.r.s); const model = zorlaAdim(c0, hid, o, true); if (!c0.ball) continue; // zorla atılamadı
    let g = 0, gn = 0; for (let r = 0; r < RN; r++) { const c = klon(once, (r * 977 + 31337) ^ seed); zorlaAdim(c, hid, o, false); if (!c.ball) continue; const s = sonuc(c, team); if (s != null) { g += s; gn++; } }
    const to = o.lead || o.to || o.q; console.log(JSON.stringify({ tur: o.kendine ? 'kendine' : o.kind, secim: +o.det.P.toFixed(3), model: model == null ? null : +model.toFixed(3), gercek: gn ? +(g / gn).toFixed(3) : null, gn, uz: +Math.hypot(to.x - m.ball.x, to.y - m.ball.y).toFixed(1), rak: +nd.toFixed(1), t: m.tick })); } }
