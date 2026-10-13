// İkili ve top kazanma ölçümü (Liman = takım 0 savunurken): v5-iz2.js'deki 16 maç kurulumu (kadrolar, rakipler, taktikler, rastgele özellikler, maç içi taktik değişikliği). Kurulum sayfadaki işçiyle (mac-isci.js) aynı.
// Her ikili: kazandı mı, kazandıysa 60 tik içinde top Liman'a geçti mi, kaybettiyse 120 tik içinde rakip gönderdi mi ve Çekirdek ne kadar ilerledi. Her top kazanma: ikiliden mi, pas kesmeden mi, boşta toptan mı.
// Ortam değişkenleri: LSID (şiddet), LBLOK (blok). Kullanım: LSID=1 LBLOK=Orta node tools/v5-ikili.js <motor> <senaryo> [tohum kayması]
// Kullanım: node tools/v5-ikili.js <motor> <senaryo 0-15> [tohum kayması=0]
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
if (process.env.LSID != null) m.tac[0].siddet = +process.env.LSID; if (process.env.LBLOK) m.tac[0].blok = process.env.LBLOK;
const cp = c => M.corePos(c) || (c.ball ? { x: c.ball.x, y: c.ball.y } : { x: 50, y: 25 });
const IK = { n: 0, w: 0, wAl: 0, wKac: 0, l: 0, lGon: 0, lIl: 0, lKayip: 0 }, KAZ = { ikili: 0, pas: 0, bos: 0, diger: 0 }, ACIK = []; let hold = m.holder ? m.holder.team : -1, prevFl = null, prevBall = null, prevDuel = 0, prevW = 0;
const t0 = Date.now(); let ci = 0;
while (!m.over && m.tick < LEN) { while (ci < gec.length && gec[ci].tick <= m.tick) uygula(gec[ci++]); const flOnce = m.fl, ballOnce = m.ball, holdOnce = m.holder; M.step(m);
  const d = m.st.duel[0] - prevDuel, w = m.st.duelW[0] - prevW; prevDuel = m.st.duel[0]; prevW = m.st.duelW[0];
  for (let k = 0; k < d; k++) { IK.n++; const kaz = k < w; if (kaz) IK.w++; else IK.l++; const c0 = cp(m); ACIK.push({ kaz, t: m.tick, x0: c0.x, gon0: m.st.shot[1], sc1: m.score[1], son: null }); }
  const yeni = m.holder ? m.holder.team : -1;
  if (yeni === 0 && hold !== 0) { if (w > 0 && m.holder && m.holder.son && m.holder.son.ok && m.holder.son.t === m.tick) KAZ.ikili++; else if (flOnce && !holdOnce) KAZ.pas++; else if (!holdOnce && ballOnce) KAZ.bos++; else KAZ.diger++; }
  if (yeni !== -1) hold = yeni;
  for (const e of ACIK) { if (e.son) continue; const dt = m.tick - e.t;
    if (e.kaz) { if (yeni === 0) { e.son = 1; IK.wAl++; } else if (yeni === 1 || dt >= 60) { e.son = 2; if (yeni === 1) IK.wKac++; } }
    else if (dt >= 120 || yeni === 0 || m.score[1] !== e.sc1) { e.son = 3; if (m.st.shot[1] > e.gon0) IK.lGon++; IK.lIl += -(cp(m).x - e.x0); if (yeni === 0) IK.lKayip++; } } }
const r3 = x => +x.toFixed(3);
console.log(JSON.stringify({ sc, ad, kay: KAY, siddet: m.tac[0].siddet, blok: m.tac[0].blok, skor: m.score, sure: +((Date.now() - t0) / 1000).toFixed(1), ikili: IK.n, kazandi: IK.w, kazAl: IK.wAl, kazKac: IK.wKac, kaybetti: IK.l, kaybGon: IK.lGon, kaybIl: r3(IK.lIl / Math.max(1, IK.l)), kaybGeri: IK.lKayip, kazanma: KAZ, kapma: m.st.steal[0], rakipGon: m.st.shot[1], rakipPas: m.st.pass[1], rakipPasOk: m.st.passOk[1] }));
