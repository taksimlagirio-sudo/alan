// Topsuz pozisyon (kopyalı ölçüm). Takım topa sahipken bir arkadaş kapalı (en yakın savunmacı < 5 ya da pas hattı kapalı) ve 5 birim yakınında açık bir nokta (savunmacı ≥ 5, hat açık) varsa an kopyalanır;
// aynı an RN kez doğal (motorun yerleşimi) ve RN kez o oyuncu açık noktaya koşarak (vardıktan sonra orada bekler, top ona gelirse ya da el değişirse bırakılır) aynı zarlarla H tik oynatılır.
// Kaydedilen: kayıp, o oyuncu pas aldı mı, gönderme, sayı, ilerleme (Çekirdeğin rakip Kuyu'ya doğru yol aldığı birim).
// Kullanım: node tools/v5-yer.js <motor> <senaryo 0-15> [RN=10] [H=150] [en çok=10]
const fs = require('fs'), path = require('path'); const [dir, sS, rS, hS, cS] = process.argv.slice(2), sc = +sS, RN = +rS || 10, H = +hS || 150, CAP = +cS || 10;
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
const AS = AlanShape, POS0 = AS.position, LO = AD.laneOk; let ZOR = null; /* ZOR: { c, id, bitis } — seçilen oyuncunun hedefi açık noktaya çevrilir */
AS.position = function (mm) { const r = POS0.apply(this, arguments); if (ZOR && mm === ZOR.c) { const q = mm.ps.find(p => p.id === ZOR.id); if (q && mm.holder === q) ZOR.bitis = 0; const bizde = mm.holder ? mm.holder.team === q.team : !!(mm.ball && mm.ball.team === q.team && !/alıcı|kovala/.test(q.job || '')); /* pas yoldayken de sürer; pas ona geliyorsa ya da top ona geçtiyse bırakılır */ if (q && bizde && mm.tick < ZOR.bitis) { q.tx = ZOR.x; q.ty = ZOR.y; q.sprint = Math.hypot(q.x - ZOR.x, q.y - ZOR.y) > 2; q.job = 'deney: açık nokta'; if (Math.hypot(q.x - ZOR.x, q.y - ZOR.y) < 1) ZOR.bitis = Math.min(ZOR.bitis, mm.tick + 30); /* vardıktan sonra en çok yarım saniye bekler, sonra motor yönetir */ } } return r; };
const corePos = c => M.corePos(c) || (c.ball ? { x: c.ball.x, y: c.ball.y } : null);
const yakin = (ps, t, x, y) => { let b = 1e9; for (const o of ps) if (o.team !== t && o.role !== 'Bekçi') b = Math.min(b, Math.hypot(o.x - x, o.y - y)); return b; };
const DIR = [...Array(12)].map((_, i) => [Math.cos(i * Math.PI / 6), Math.sin(i * Math.PI / 6)]);
function oyna(c, team, qid, hx, hy) { let yak = 1e9; const dir = team === 0 ? 1 : -1, p0 = corePos(c), x0 = p0 ? p0.x : 50, sc0 = c.score.slice(), sh0 = c.st.shot[team]; let kayip = 0, aldi = 0, xs = x0;
  for (let t = 0; t < H && !c.over; t++) { M.step(c); if (c.score[1 - team] !== sc0[1 - team]) { kayip = 1; break; } if (c.score[team] !== sc0[team]) break; if (c.holder && c.holder.team !== team) { kayip = 1; break; } if (c.holder && c.holder.id === qid) aldi = 1; { const qq = c.ps.find(p => p.id === qid); if (qq && hx != null) yak = Math.min(yak, Math.hypot(qq.x - hx, qq.y - hy)); } const p = corePos(c); if (p) xs = p.x; }
  return { kayip, aldi, gon: c.st.shot[team] > sh0 ? 1 : 0, sayi: c.score[team] > sc0[team] ? 1 : 0, il: (xs - x0) * dir, yak: Math.min(yak, 99) }; }
const ort = a => { const o = {}; for (const k of Object.keys(a[0])) o[k] = +(a.reduce((s, r) => s + r[k], 0) / a.length).toFixed(3); o.n = a.length; return o; };
let alinan = 0, sonT = -1e9; const t0 = Date.now();
while (!m.over && m.tick < 5400 && alinan < CAP) { M.step(m); const h = m.holder; if (!h || m.tick - sonT < 120 || m.tick % 6) continue; const t = h.team;
  let sec = null; for (const q of m.ps) { if (q.team !== t || q === h || q.role === 'Bekçi') continue; const a = yakin(m.ps, t, q.x, q.y), l = LO(m.ps, t, h, q); if (a >= 5 && l >= .5) continue;
    for (const r of [2.5, 5]) for (const [ux, uy] of DIR) { const x = q.x + ux * r, y = q.y + uy * r; if (x < 2 || x > 98 || y < 2 || y > 48) continue; const a2 = yakin(m.ps, t, x, y); if (a2 < 5) continue; const l2 = LO(m.ps, t, h, { x, y }); if (l2 < .5) continue; const k = (a2 - a) + 5 * (l2 - l); if (!sec || k > sec.k) sec = { q, x, y, k, a, l, a2, l2, r }; } }
  if (!sec) continue; sonT = m.tick; alinan++; const q = sec.q, once = klon(m), D0 = [], D1 = [];
  for (let k = 0; k < RN; k++) { const s = (k * 977 + 31337) ^ (seed + m.tick); const c0 = klon(once, s); D0.push(oyna(c0, t, q.id, sec.x, sec.y)); const c1 = klon(once, s); ZOR = { c: c1, id: q.id, x: sec.x, y: sec.y, bitis: c1.tick + H }; try { D1.push(oyna(c1, t, q.id, sec.x, sec.y)); } finally { ZOR = null; } }
  const ileri = (t === 0 ? 1 : -1) * (sec.x - q.x);
  console.log(JSON.stringify({ sc, tick: m.tick, team: t, ad: q.name, rol: q.role, job: q.job, x: +((t === 0 ? q.x : 100 - q.x)).toFixed(1), acik: +sec.a.toFixed(1), hat: +sec.l.toFixed(2), acik2: +sec.a2.toFixed(1), hat2: +sec.l2.toFixed(2), r: sec.r, ileri: +ileri.toFixed(1), dogal: ort(D0), acikNokta: ort(D1) })); }
console.error(`${ad} · ${alinan} an · ${((Date.now() - t0) / 1000).toFixed(0)} sn`);
