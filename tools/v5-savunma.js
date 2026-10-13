// Savunma ölçümü: v5-iz2.js'deki 16 maç kurulumu (kadrolar, rakipler, taktikler, rastgele özellikler, maç içi taktik değişikliği). Kurulum sayfadaki işçiyle (mac-isci.js) aynı.
// Tek satır JSON: skor, süre, m.st ve her tik ölçülen savunma göstergeleri (taşıyıcının dibindeki savunmacı, boş adam, pres, topun kazanıldığı yer, sahiplik süresi).
// Kullanım: node tools/v5-savunma.js <motor> <senaryo 0-15> [tohum kayması=0]
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
const t0 = Date.now(); let ci = 0;
if (process.env.LPRES != null) m.tac[0].pres = +process.env.LPRES; if (process.env.LBLOK) m.tac[0].blok = process.env.LBLOK; if (process.env.LSID != null) m.tac[0].siddet = +process.env.LSID; /* tek değişkenli deney: Liman'ın pres/blok ayarı */
const TK = [0, 1].map(t => ({ ...m.tac[t] })), TT = [0, 1].map(() => ({ savN: 0, pres: 0, dip: 0, hucN: 0, dur: 0, kaz: [] }));
const G = { n: 0, dip: 0, dip3: 0, son: 0, son3: 0, bos: 0, bosN: 0, pres: 0, kazX: [], sah: [] }; let oh = null, ohT = 0, sahBas = 0;
while (!m.over && m.tick < LEN) { while (ci < gec.length && gec[ci].tick <= m.tick) uygula(gec[ci++]); M.step(m); const h = m.holder;
  if (h) { const t = h.team, gx = t === 0 ? 100 : 0, ileri = t === 0 ? h.x : 100 - h.x, def = m.ps.filter(p => p.team !== t && p.role !== 'Bekçi'), att = m.ps.filter(p => p.team === t && p !== h && p.role !== 'Bekçi');
    const d6 = def.filter(p => Math.hypot(p.x - h.x, p.y - h.y) < 6).length, d3 = def.filter(p => Math.hypot(p.x - h.x, p.y - h.y) < 3).length; G.n++; G.dip += d6; { const D = TT[1 - t], H = TT[t]; D.savN++; D.pres += d3; D.dip += d6; H.hucN++; if (Math.hypot(h.vx || 0, h.vy || 0) < .04) H.dur++; } if (d6 >= 3) G.dip3++; G.pres += d3;
    if (ileri > 70) { G.son++; if (d6 >= 3) G.son3++; let b = 0; for (const a of att) { if ((t === 0 ? a.x : 100 - a.x) < 60) continue; if (!def.some(p => Math.hypot(p.x - a.x, p.y - a.y) < 5)) b++; } G.bos += b; G.bosN++; }
    if (!oh || oh.team !== t) { if (oh) { G.kazX.push(+ileri.toFixed(1)); TT[t].kaz.push(ileri); G.sah.push(m.tick - sahBas); } sahBas = m.tick; } oh = h; } }
const ort = a => a.length ? +(a.reduce((s, x) => s + x, 0) / a.length).toFixed(2) : null;
const GS = { dip: +(G.dip / G.n).toFixed(3), dip3: +(G.dip3 / G.n).toFixed(3), son3: G.son ? +(G.son3 / G.son).toFixed(3) : null, sonPay: +(G.son / G.n).toFixed(3), bos: G.bosN ? +(G.bos / G.bosN).toFixed(3) : null, pres: +(G.pres / G.n).toFixed(3), kazX: ort(G.kazX), kazOn: G.kazX.length ? +(G.kazX.filter(x => x < 50).length / G.kazX.length).toFixed(3) : null, degis: G.kazX.length, sah: ort(G.sah), T: TT.map((x, t) => ({ pres: TK[t].pres, blok: TK[t].blok, tempo: TK[t].tempo, siddet: TK[t].siddet, sabir: TK[t].sabir, sistem: TK[t].sistem, presD: +(x.pres / Math.max(1, x.savN)).toFixed(3), dipD: +(x.dip / Math.max(1, x.savN)).toFixed(3), dur: +(x.dur / Math.max(1, x.hucN)).toFixed(3), pas: m.st.pass[t], kapma: m.st.steal[t], kazX: x.kaz.length ? +(x.kaz.reduce((a, b) => a + b, 0) / x.kaz.length).toFixed(1) : null, kazIleri: x.kaz.length ? +(x.kaz.filter(v => v >= 50).length / x.kaz.length).toFixed(3) : null, gon: m.st.shot[t], sayi: m.score[t] })) };
console.log(JSON.stringify({ sc, ad, kay: KAY, skor: m.score, sure: +((Date.now() - t0) / 1000).toFixed(1), st: m.st, sv: GS }));
