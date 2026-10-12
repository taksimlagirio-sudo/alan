// Fake'in gerçek değeri (kopyalı ölçüm). Maçta bir taşıyıcı fake seçtiği an kopyalanır; aynı an RN kez fake ile, RN kez aklın fake dışındaki en iyi seçeneğiyle (aynı karardaki) farklı zarla H tik oynatılır.
// Kaydedilen: kayıp (rakip topu aldı ya da sayı yedi), gönderme, sayı, ilerleme (Çekirdeğin rakip Kuyu'ya doğru yol aldığı birim; kayıpta kayıp anına kadar), aldanan oldu mu; aklın tahmini: fake'in ve alternatifin değeri (v), aldanma ihtimali (det.P).
// Kullanım: node tools/v5-fake.js <motor> <senaryo 0-15> [örnekleme=2] [RN=12] [H=150] [en çok=10]
const fs = require('fs'), path = require('path'); const [dir, sS, oS, rS, hS, cS] = process.argv.slice(2), sc = +sS, ORN = +oS || 2, RN = +rS || 12, H = +hS || 150, CAP = +cS || 10;
globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'kadro.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = AlanMatch, AD = AlanDecide, K = ALAN_KADRO, R = K.RAKIP;
const mk = s => { const f = () => { f.s = (f.s + 0x6D2B79F5) | 0; let a = f.s, t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; f.s = s | 0; return f; };
const klon = (m, s) => { const r = m.r; m.r = null; let c; try { c = structuredClone(m); } finally { m.r = r; } c.r = mk(s == null ? r.s : s); return c; };
const ids = ['aras', 'deniz', 'mert', 'ece', 'cansu', 'kaan', 'tuna'], liman = ids.map(id => { const p = K.KADRO.find(k => k.id === id); return { ad: p.ad, a: p.a }; });
const LT = { sistem: 'Alan', pres: 1, arkada: 1, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli' };
const rk = r => ({ tac: r.tac, diz: r.diz, oyuncular: r.oyuncular.map(([ad, a]) => ({ ad, a })), roller: r.roller });
const lm = (tac, diz) => ({ tac: { ...LT, ...tac }, diz: diz || { sav: '2-2-2', huc: '2-1-3' }, oyuncular: liman, roller: null });
const S = [
  ['Liman–Demir Blok', lm(), rk(R[0])], ['Liman–Fırtına', lm(), rk(R[1])], ['Liman–Saat Kulesi', lm(), rk(R[2])],
  ['Liman–Demir Blok (2)', lm(), rk(R[0])], ['Liman–Fırtına (2)', lm(), rk(R[1])], ['Liman–Saat Kulesi (2)', lm(), rk(R[2])],
  ['Demir Blok–Fırtına', rk(R[0]), rk(R[1])], ['Fırtına–Saat Kulesi', rk(R[1]), rk(R[2])], ['Saat Kulesi–Demir Blok', rk(R[2]), rk(R[0])],
  ['Liman adam adama yüksek pres–Demir Blok', lm({ sistem: 'Adam adama', pres: 3, blok: 'Yüksek', genislik: 'Geniş', tempo: .8, risk: .8 }, { sav: '2-2-2', huc: '2-2-2' }), rk(R[0])],
  ['Liman düşük blok yerleş–Fırtına', lm({ pres: 0, arkada: 2, blok: 'Düşük', genislik: 'Dar', tempo: .2, risk: .2, kazaninca: 'Yerleş' }, { sav: '3-2-1', huc: '2-3-1' }), rk(R[1])],
  ['Liman kontra–Saat Kulesi', lm({ pres: 2, blok: 'Orta', tempo: .9, risk: .9, kazaninca: 'Kontra' }, { sav: '2-3-1', huc: '1-2-3' }), rk(R[2])],
  ['Rastgele özellikler 1', { ...lm(), rast: 1 }, { ...rk(R[1]), rast: 1 }], ['Rastgele özellikler 2', { ...lm({ pres: 2 }), rast: 1 }, { ...rk(R[0]), rast: 1 }],
  ['Maç içi değişiklik: Liman–Demir Blok', lm(), rk(R[0]), [{ tick: 1800, t: 0, tac: { pres: 3, blok: 'Yüksek', tempo: .9 } }, { tick: 3600, t: 1, tac: { pres: 2, blok: 'Orta' }, diz: { sav: '2-2-2', huc: '2-2-2' } }]],
  ['Maç içi değişiklik: Fırtına–Liman', rk(R[1]), lm(), [{ tick: 1200, t: 1, tac: { kazaninca: 'Kontra', risk: .9 }, diz: { sav: '3-2-1', huc: '2-3-1' } }, { tick: 3000, t: 0, tac: { pres: 0, blok: 'Düşük' } }, { tick: 4200, t: 1, tac: { sistem: 'Adam adama', pres: 3 } }]],
][sc];
const [ad, A0, B0] = S, T = [A0, B0], seed = 2000 + sc * 13;
const m = M.createMatch(seed, { tac: T.map(x => x.tac), len: 5400, diz: T.map(x => x.diz) }); m.r = mk(seed);
if (T.some(x => x.rast)) { let r0 = (sc * 2654435761) >>> 0; const Rn = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296; for (const p of m.ps) for (const k of Object.keys(p.a)) p.a[k] = 6 + Math.floor(Rn() * 11); }
for (const t of [0, 1]) { const Ao = T[t].oyuncular; if (Ao) for (const p of m.ps) if (p.team === t && Ao[p.i]) { p.name = Ao[p.i].ad; p.a = { ...p.a, ...Ao[p.i].a }; } }
const rol = T.map(x => x.roller || null); if (rol.some(Boolean)) { M.assignRoles(m, rol); for (const p of m.ps) { const r = rol[p.team] && rol[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } } for (const t of [0, 1]) M.dizUygula(m, t, T[t].diz);
const mapO = (o, c) => { const r = { ...o }; if (o.q) r.q = c.ps.find(p => p.id === o.q.id); return r; };
// taşıyıcının bu kopyadaki ilk kararı: yakala (orijinal karar) ya da zorla (verilen hamle)
function adim(c, hid, zorla) { const od = AD.decide; let ilk = true, yak = null;
  AD.decide = function (hh) { if (ilk && hh.id === hid) { ilk = false; if (zorla) { const oo = mapO(zorla, c); return { best: oo, opts: [oo] }; } yak = od.apply(this, arguments); return yak; } return od.apply(this, arguments); };
  try { M.step(c); } finally { AD.decide = od; } return yak; }
const corePos = c => M.corePos(c) || (c.ball ? { x: c.ball.x, y: c.ball.y } : null);
function oyna(c, team) { const dir = team === 0 ? 1 : -1, p0 = corePos(c), x0 = p0 ? p0.x : 50, sc0 = c.score.slice(), sh0 = c.st.shot[team], F = c.fake && c.fake.hid != null && c.fake.t0 === c.tick - 1 ? c.fake : null, ald = c.fake && c.fake.fooled.size ? 1 : 0, aldN = c.fake ? c.fake.fooled.size : 0; let kayip = 0, xs = x0;
  for (let t = 0; t < H && !c.over; t++) { if (c.score[1 - team] !== sc0[1 - team]) { kayip = 1; break; } if (c.score[team] !== sc0[team]) break; if (c.holder && c.holder.team !== team) { kayip = 1; break; } const p = corePos(c); if (p) xs = p.x; M.step(c); }
  return { kayip, gon: c.st.shot[team] > sh0 ? 1 : 0, sayi: c.score[team] > sc0[team] ? 1 : 0, yedi: c.score[1 - team] > sc0[1 - team] ? 1 : 0, il: (xs - x0) * dir, ald, aldN }; }
const ort = a => { const k = Object.keys(a[0]), o = {}; for (const x of k) o[x] = +(a.reduce((s, r) => s + r[x], 0) / a.length).toFixed(3); o.n = a.length; return o; };
let once = null, son = null, n = 0, alinan = 0; const t0 = Date.now();
while (!m.over && m.tick < 5400 && alinan < CAP) { once = m.holder ? klon(m) : null; M.step(m);
  const F = m.fake; if (!F || F === son || F.t0 !== m.tick - 1 && F.t0 !== m.tick) continue; son = F; if (!once || (n++ % ORN) !== 0) continue;
  const hid = F.hid, team = F.team, c0 = klon(once, once.r.s), r = adim(c0, hid, null); if (!r || !r.best || r.best.kind !== 'fake') { console.error('yakalanamadı', m.tick); continue; }
  const of = r.best, oa = r.opts.find(o => o.kind !== 'fake'); if (!oa) continue; alinan++;
  const h = once.ps.find(p => p.id === hid), gx = team === 0 ? 100 : 0; let nd = 1e9; for (const p of once.ps) if (p.team !== team && p.role !== 'Bekçi') nd = Math.min(nd, Math.hypot(p.x - h.x, p.y - h.y));
  const sonF = [], sonA = []; for (let k = 0; k < RN; k++) { const s = (k * 977 + 31337) ^ (seed + m.tick); const cf = klon(once, s); adim(cf, hid, of); sonF.push(oyna(cf, team)); const ca = klon(once, s); adim(ca, hid, oa); const x = oyna(ca, team); x.ald = 0; x.aldN = 0; sonA.push(x); }
  console.log(JSON.stringify({ sc, tick: m.tick, team, ad: h.name, oku: h.a.okuma, sur: h.a.surme, x: +((team === 0 ? h.x : 100 - h.x)).toFixed(1), rak: +nd.toFixed(1), fake: of.show.k, sonra: of.det.sonra, pf: +of.det.P.toFixed(2), vF: +of.v.toFixed(3), alt: oa.kind + (oa.q ? '→' + oa.q.name : ''), vA: +oa.v.toFixed(3), F: ort(sonF), A: ort(sonA) })); }
console.error(`${ad} · ${alinan} an · ${((Date.now() - t0) / 1000).toFixed(0)} sn`);
