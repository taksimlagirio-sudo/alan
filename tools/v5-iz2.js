// Birebir aynılık kontrolü (geniş): 16 farklı maç kurulumu (kadrolar, rakipler, taktikler, rastgele özellikler, maç içi taktik değişikliği). Kurulum sayfadaki işçiyle (mac-isci.js) aynı.
// Her tik bütün oyuncuların konum/hız, Çekirdek, yük, skor, top sahibi ve olay sayısı sayının bitleriyle özete katılır; her 60 tikte bir satır.
// Kullanım: node tools/v5-iz2.js <motor> <senaryo 0-15>
const path = require('path'), fs = require('fs'); const { load } = require('./load'); const [dir, sS, lS] = process.argv.slice(2), sc = +sS, LEN = +lS || 5400;
const g = load(dir, 100 + sc); (0, eval)(fs.readFileSync(path.join(dir, 'kadro.js'), 'utf8')); const M = g.AlanMatch, K = g.ALAN_KADRO, R = K.RAKIP;
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
const m = M.createMatch(1000 + sc * 7, { tac: T.map(x => x.tac), len: 5400, diz: T.map(x => x.diz) });
if (T.some(x => x.rast)) { let r0 = (sc * 2654435761) >>> 0; const Rn = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296; for (const p of m.ps) for (const k of Object.keys(p.a)) p.a[k] = 6 + Math.floor(Rn() * 11); }
for (const t of [0, 1]) { const Ao = T[t].oyuncular; if (Ao) for (const p of m.ps) if (p.team === t && Ao[p.i]) { p.name = Ao[p.i].ad; p.a = { ...p.a, ...Ao[p.i].a }; } }
const egUy = (Rr) => { for (const p of m.ps) { const r = Rr[p.team] && Rr[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } };
const rol = T.map(x => x.roller || null); if (rol.some(Boolean)) { M.assignRoles(m, rol); egUy(rol); } for (const t of [0, 1]) M.dizUygula(m, t, T[t].diz);
const uygula = c => { m.tac[c.t] = { ...m.tac[c.t], ...c.tac }; if (c.diz) M.dizUygula(m, c.t, c.diz); };
const f64 = new Float64Array(1), u32 = new Uint32Array(f64.buffer); let h = 0; const add = x => { f64[0] = x ?? -1; h = (Math.imul(h, 31) + u32[0]) | 0; h = (Math.imul(h, 31) + u32[1]) | 0; };
const t0 = Date.now(), out = []; let ci = 0;
while (!m.over && m.tick < LEN) { while (ci < gec.length && gec[ci].tick <= m.tick) uygula(gec[ci++]); M.step(m);
  for (const p of m.ps) { add(p.x); add(p.y); add(p.vx); add(p.vy); } if (m.ball) { add(m.ball.x); add(m.ball.y); add(m.ball.vx); add(m.ball.vy); } add(m.ch); add(m.score[0]); add(m.score[1]); add(m.holder ? m.ps.indexOf(m.holder) : -2); add(m.events.length);
  if (m.tick % 60 === 0) out.push(m.tick + ' ' + (h >>> 0).toString(16)); }
console.log(out.join('\n')); console.error(`${ad} · süre ${((Date.now() - t0) / 1000).toFixed(1)} sn · skor ${m.score}`);
