// Baskı altında pas: v5-iz2.js'deki 16 maç kurulumu (kadrolar, rakipler, taktikler, rastgele özellikler, maç içi taktik değişikliği). Kurulum sayfadaki işçiyle (mac-isci.js) aynı.
// Her pas (gönderme hariç) atıldığı anda: pasçıya en yakın rakip (Bekçi hariç), 4 birim içindeki rakip sayısı, görevi pres olan var mı; pas türü, mesafesi, ileri/geri; aklın tutma tahmini (det.P); sonuç (Çekirdek bizde kaldı mı).
// Çıktı: satır başına bir pas (JSON). Ölçüm için motorun kopyasına yalnız kayıt alanı eklenir (uçuşa seçilen hamle); akış değişmez.
// Kullanım: node tools/v5-baski.js <motor> <senaryo 0-15> [tohum kayması=0]
const path = require('path'), fs = require('fs'); const { load } = require('./load'); const [dir, sS, kS, lS] = process.argv.slice(2), sc = +sS, LEN = +lS || 5400, KAY = +kS || 0;
const g = load(dir, 100 + sc + KAY * 1000, { 'match.js': ['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'] }); (0, eval)(fs.readFileSync(path.join(dir, 'kadro.js'), 'utf8')); const M = g.AlanMatch, K = g.ALAN_KADRO, R = K.RAKIP;
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
const PAS = { pas: 1, 'önüne': 1, kenardan: 1, 'aşırt': 1 }; let bekle = null, fl0 = null; const out = [];
const t0 = Date.now(); let ci = 0;
while (!m.over && m.tick < LEN) { while (ci < gec.length && gec[ci].tick <= m.tick) uygula(gec[ci++]); const sc0 = m.score.slice(); M.step(m);
  if (bekle) { const g2 = m.score[0] !== sc0[0] || m.score[1] !== sc0[1]; if (g2 || m.holder) { bekle.tut = g2 ? (m.score[bekle.team] !== sc0[bekle.team] ? 1 : 0) : (m.holder.team === bekle.team ? 1 : 0); out.push(bekle); bekle = null; } else if (m.tick - bekle.tick > 400) bekle = null; }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; const o = m.fl.o; if (o && PAS[o.kind] && !bekle) { const b = m.ball, h = b && b.from; if (h) { const t = h.team, dir = t === 0 ? 1 : -1, to = o.lead || o.to || o.q; let nd = 1e9, n4 = 0, pr = 0; for (const q of m.ps) { if (q.team === t || q.role === 'Bekçi') continue; const d = Math.hypot(q.x - h.x, q.y - h.y); nd = Math.min(nd, d); if (d < 4) n4++; if (typeof q.job === 'string' && q.job.startsWith('pres')) pr = 1; }
    bekle = { team: t, tick: m.tick, k: o.kind + (o.kendine ? '·kendine' : ''), nd: +nd.toFixed(2), n4, pr, uz: to ? +Math.hypot(to.x - h.x, to.y - h.y).toFixed(1) : null, ileri: to ? +((to.x - h.x) * dir).toFixed(1) : null, x: +((t === 0 ? h.x : 100 - h.x)).toFixed(1), P: o.det && o.det.P != null ? +o.det.P.toFixed(3) : null, ak: h.a.aktarim ?? 10, ot: o.ot ? 1 : 0 }; } } } }
for (const r of out) console.log(JSON.stringify({ sc, kay: KAY, ...r }));
console.error(`${ad} · ${out.length} pas · ${((Date.now() - t0) / 1000).toFixed(0)} sn`);
