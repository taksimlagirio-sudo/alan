// Topsuz hücum (koşu ve pozisyon) ölçümü: v5-iz2.js'deki 16 maç kurulumu (kadrolar, rakipler, taktikler, rastgele özellikler, maç içi taktik değişikliği). Kurulum sayfadaki işçiyle (mac-isci.js) aynı.
// Takım topa sahipken her 6 tikte taşıyıcı dışındaki saha oyuncuları ölçülür: açıklık (en yakın savunmacı), pas hattı (decide.laneOk), yakında daha açık nokta varken kapalı kalma,
// topun önündeki oyuncu, son savunmacının gerisi, genişlik, kümelenme, koşu yönü; taşıyıcının her kararında tutma ihtimali ≥ .7 olan farklı alıcı sayısı. Takım başına tek satır JSON.
// Kullanım: node tools/v5-kosu.js <motor> <senaryo 0-15> [tohum kayması=0]
const path = require('path'), fs = require('fs'); const { load } = require('./load'); const [dir, sS, kS, lS] = process.argv.slice(2), sc = +sS, LEN = +lS || 5400, KAY = +kS || 0;
const g = load(dir, 100 + sc + KAY * 1000); (0, eval)(fs.readFileSync(path.join(dir, 'kadro.js'), 'utf8')); const M = g.AlanMatch, K = g.ALAN_KADRO, R = K.RAKIP;
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
const [ad, A0, B0, gec = []] = S, T = [A0, B0];
const m = M.createMatch(1000 + sc * 7 + KAY * 100003, { tac: T.map(x => x.tac), len: 5400, diz: T.map(x => x.diz) });
if (T.some(x => x.rast)) { let r0 = (sc * 2654435761) >>> 0; const Rn = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296; for (const p of m.ps) for (const k of Object.keys(p.a)) p.a[k] = 6 + Math.floor(Rn() * 11); }
for (const t of [0, 1]) { const Ao = T[t].oyuncular; if (Ao) for (const p of m.ps) if (p.team === t && Ao[p.i]) { p.name = Ao[p.i].ad; p.a = { ...p.a, ...Ao[p.i].a }; } }
const egUy = (Rr) => { for (const p of m.ps) { const r = Rr[p.team] && Rr[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } };
const rol = T.map(x => x.roller || null); if (rol.some(Boolean)) { M.assignRoles(m, rol); egUy(rol); } for (const t of [0, 1]) M.dizUygula(m, t, T[t].diz);
const uygula = c => { m.tac[c.t] = { ...m.tac[c.t], ...c.tac }; if (c.diz) M.dizUygula(m, c.t, c.diz); };
const AD = g.AlanDecide, LO = AD.laneOk, Z = () => ({ n: 0, k: 0, acik: 0, acik5: 0, hat: 0, secAcik: 0, kacir: 0, kapali: 0, onde: 0, arka: 0, gen: 0, kume: 0, dibe: 0, kos: 0, kosAcik: 0, kosIleri: 0, kar: 0, sec: 0, sec0: 0 }), TT = [Z(), Z()], TK = [0, 1].map(t => ({ ...m.tac[t] }));
const yakin = (x, y, D) => { let b = 1e9; for (const o of D) b = Math.min(b, Math.hypot(o.x - x, o.y - y)); return b; }, DIR = [...Array(12)].map((_, i) => [Math.cos(i * Math.PI / 6), Math.sin(i * Math.PI / 6)]);
let sonDec = null; const t0 = Date.now(); let ci = 0;
while (!m.over && m.tick < LEN) { while (ci < gec.length && gec[ci].tick <= m.tick) uygula(gec[ci++]); M.step(m); const h = m.holder;
  if (h && m.lastDec && m.lastDec !== sonDec && m.lastDec.team === h.team) { sonDec = m.lastDec; const S = TT[h.team], al = new Set(); for (const o of m.lastDec.opts || []) if (o.q && (o.P ?? 0) >= .7) al.add(o.q); S.kar++; S.sec += al.size; if (!al.size) S.sec0++; }
  if (!h || m.tick % 6) continue; const t = h.team, S = TT[t], dir = t === 0 ? 1 : -1, ileri = x => (x - h.x) * dir, D = m.ps.filter(p => p.team !== t), Ds = D.filter(p => p.role !== 'Bekçi'), T = m.ps.filter(p => p.team === t && p !== h && p.role !== 'Bekçi');
  if (!T.length) continue; S.k++; const sonX = Ds.reduce((b, o) => dir > 0 ? Math.max(b, o.x) : Math.min(b, o.x), dir > 0 ? -1e9 : 1e9); let ymin = 1e9, ymax = -1e9;
  for (const q of T) { S.n++; const a = yakin(q.x, q.y, Ds), l = LO ? LO(m.ps, t, h, q) : 1; S.acik += Math.min(a, 15); if (a >= 5) S.acik5++; if (l >= .5) S.hat++; if (a >= 5 && l >= .5) S.secAcik++;
    if (!(a >= 5 && l >= .5)) { S.kapali++; let bul = false; for (const r of [2.5, 5]) { for (const [ux, uy] of DIR) { const x = q.x + ux * r, y = q.y + uy * r; if (x < 2 || x > 98 || y < 2 || y > 48) continue; if (yakin(x, y, Ds) >= 5 && (!LO || LO(m.ps, t, h, { x, y }) >= .5)) { bul = true; break; } } if (bul) break; } if (bul) S.kacir++; }
    if (ileri(q.x) > 3) S.onde++; if ((q.x - sonX) * dir > 0) S.arka++; ymin = Math.min(ymin, q.y); ymax = Math.max(ymax, q.y); if (Math.hypot(q.x - h.x, q.y - h.y) < 5) S.dibe++;
    for (const r of T) if (r !== q && Math.hypot(r.x - q.x, r.y - q.y) < 4) { S.kume++; break; }
    if (Math.hypot(q.vx || 0, q.vy || 0) > .1 && q.tx != null) { S.kos++; if (yakin(q.tx, q.ty, Ds) > a) S.kosAcik++; if (ileri(q.tx) > ileri(q.x) + 1) S.kosIleri++; } }
  S.gen += ymax - ymin; }
const r3 = x => +x.toFixed(3);
const out = TT.map((S, t) => ({ tac: { tempo: TK[t].tempo, genislik: TK[t].genislik, sabir: TK[t].sabir, sistem: TK[t].sistem }, an: S.k, acik: r3(S.acik / S.n), acik5: r3(S.acik5 / S.n), hat: r3(S.hat / S.n), secAcik: r3(S.secAcik / S.n), kacir: r3(S.kacir / Math.max(1, S.kapali)), kapali: r3(S.kapali / S.n), onde: r3(S.onde / S.k), arka: r3(S.arka / S.k), gen: r3(S.gen / S.k), kume: r3(S.kume / S.n), dibe: r3(S.dibe / S.k), kos: r3(S.kos / S.n), kosAcik: r3(S.kosAcik / Math.max(1, S.kos)), kosIleri: r3(S.kosIleri / Math.max(1, S.kos)), sec: r3(S.sec / Math.max(1, S.kar)), sec0: r3(S.sec0 / Math.max(1, S.kar)), kar: S.kar, pas: m.st.pass[t], gon: m.st.shot[t], sayi: m.score[t] }));
console.log(JSON.stringify({ sc, ad, kay: KAY, skor: m.score, sure: +((Date.now() - t0) / 1000).toFixed(1), T: out }));
