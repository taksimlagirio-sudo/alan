// Pas fake'i: fake'ten sonraki pasın aklın tahmini (iki dünyada det.P) ile gerçeği, ve aynı pasın fake'siz hemen atılması (kopyalı ölçüm).
// Ölçüm için motorun kopyasına yalnız kayıt alanı eklenir (fake seçeneğine iki dünyanın planı, uçuşa seçilen hamle); akış değişmez.
// Eski açıklama (v5-fake.js): Maçta bir taşıyıcı fake seçtiği an kopyalanır; aynı an RN kez fake ile, RN kez aklın fake dışındaki en iyi seçeneğiyle (aynı karardaki) farklı zarla H tik oynatılır.
// Kaydedilen: kayıp (rakip topu aldı ya da sayı yedi), gönderme, sayı, ilerleme (Çekirdeğin rakip Kuyu'ya doğru yol aldığı birim; kayıpta kayıp anına kadar), aldanan oldu mu; aklın tahmini: fake'in ve alternatifin değeri (v), aldanma ihtimali (det.P).
// Kullanım: node tools/v5-fakepas.js <motor> <senaryo 0-15> [RN=12] [en çok=10]
const fs = require('fs'), path = require('path'); const [dir, sS, rS, cS] = process.argv.slice(2), sc = +sS, RN = +rS || 12, CAP = +cS || 10;
globalThis.window = globalThis; globalThis.self = globalThis;
const YAMA = { 'decide.js': ['det: { P: pf, sonra: r1.best.kind, T, c: sh.c } }', 'det: { P: pf, sonra: r1.best.kind, T, c: sh.c, r1b: r1.best, r0b: r0.best } }'], 'match.js': ['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'] };
for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'kadro.js']) { let t = fs.readFileSync(path.join(dir, f), 'utf8'); const y = YAMA[f]; if (y) { if (!t.includes(y[0])) throw new Error('yama yeri yok: ' + f); t = t.replace(y[0], y[1]); } (0, eval)(t + '\n//# sourceURL=' + f); }
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
function adim(c, hid, zorla) { const od = AD.decide; let ilk = true, yak = null;
  AD.decide = function (hh) { if (ilk && hh.id === hid) { ilk = false; if (zorla) { const oo = mapO(zorla, c); return { best: oo, opts: [oo] }; } yak = od.apply(this, arguments); return yak; } return od.apply(this, arguments); };
  try { M.step(c); } finally { AD.decide = od; } return yak; }
const PAS = { pas: 1, 'önüne': 1, kenardan: 1 }, ozt = o => o ? { k: o.kind + (o.kendine ? '·kendine' : ''), q: o.q ? o.q.id : null, ad: o.q ? o.q.name : null, P: o.det && o.det.P != null ? +o.det.P.toFixed(3) : null, v: o.v != null ? +o.v.toFixed(3) : null } : null;
// uçuştaki Çekirdek kime düşer: 1 = bizde kaldı (ya da sayı attık), 0 = rakipte (ya da sayı yedik)
function sonuc(c, team) { for (let t = 0; t < 400 && !c.over; t++) { const s0 = c.score.slice(); M.step(c); if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) return c.score[team] !== s0[team] ? 1 : 0; if (c.holder) return c.holder.team === team ? 1 : 0; } return null; }
// fake oynandıktan sonra ilk hamle: ne atıldı, tuttu mu
function fakeSonra(c, hid, team) { const ald = c.fake && c.fake.fooled.size ? 1 : 0, fl0 = c.fl; for (let t = 0; t < 200 && !c.over; t++) { const s0 = c.score.slice(); M.step(c); if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) return { ald, ne: 'sayı', tut: c.score[team] !== s0[team] ? 1 : 0 };
    if (c.fl && c.fl !== fl0 && c.fl.o) { const o = c.fl.o; return { ald, ne: o.kind + (o.kendine ? '·kendine' : ''), q: o.q ? o.q.id : null, tut: sonuc(c, team) }; } if (c.holder && c.holder.team !== team) return { ald, ne: 'kayıp', tut: 0 }; } return { ald, ne: 'yok', tut: null }; }
const ayni = (x, p) => x && p && x.kind === p.kind && !!x.kendine === !!p.kendine && (x.q ? x.q.id : null) === (p.q ? p.q.id : null);
let once = null, son = null, alinan = 0; const t0 = Date.now();
while (!m.over && m.tick < 5400 && alinan < CAP) { once = m.holder ? klon(m) : null; M.step(m);
  const F = m.fake; if (!F || F === son) continue; son = F; if (!once || F.k !== 'pas') continue;
  const hid = F.hid, team = F.team, c0 = klon(once, once.r.s), r = adim(c0, hid, null); if (!r || !r.best || r.best.kind !== 'fake') { console.error('yakalanamadı', m.tick); continue; }
  const of = r.best, d = of.det, plan1 = d.r1b, plan0 = d.r0b; alinan++;
  // aynı pas, fake'siz, hemen: bu karardaki aynı tür ve aynı alıcılı seçenek (iki dünyanın planı için ayrı ayrı)
  const hemen = [plan1, plan0].map(p => p && PAS[p.kind] ? r.opts.find(x => ayni(x, p)) : null);
  const FK = [], HM = [[], []];
  for (let k = 0; k < RN; k++) { const s = (k * 977 + 31337) ^ (seed + m.tick); const cf = klon(once, s); adim(cf, hid, of); FK.push(fakeSonra(cf, hid, team));
    hemen.forEach((o, w) => { if (!o || (w === 1 && hemen[0] && ayni(o, hemen[0]))) return; const ch = klon(once, s); adim(ch, hid, o); HM[w].push(ch.ball && !ch.holder ? sonuc(ch, team) : null); }); }
  const h = once.ps.find(p => p.id === hid); let nd = 1e9; for (const p of once.ps) if (p.team !== team && p.role !== 'Bekçi') nd = Math.min(nd, Math.hypot(p.x - h.x, p.y - h.y));
  console.log(JSON.stringify({ sc, tick: m.tick, team, ad: h.name, x: +((team === 0 ? h.x : 100 - h.x)).toFixed(1), rak: +nd.toFixed(1), pf: +d.P.toFixed(3), vF: +of.v.toFixed(3), plan1: ozt(plan1), plan0: ozt(plan0), hemen: hemen.map(ozt), FK, HM })); }
console.error(`${ad} · ${alinan} pas fake anı · ${((Date.now() - t0) / 1000).toFixed(0)} sn`);
