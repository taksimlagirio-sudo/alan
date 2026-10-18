// Maç incelemesi: kaydedilen maçı (tohum + kurulum + taktik değişiklikleri) baştan oynatır, savunmanın davranışını ölçer. mac-isci.js'in kurulum fonksiyonlarını kullanır (aynı maç).
importScripts('mac-isci.js');
self.onmessage = e => { const d = e.data, df = d.savunan ?? 1, at = 1 - df; try {
  kur = d.yeni; len = d.yeni.len; gecmis = (d.degisiklik || []).map(c => ({ tick: c.snapT, t: c.t, tac: c.tac, diz: c.diz, roller: c.roller || null })).sort((a, b) => a.tick - b.tick);
  m = M.createMatch(kur.tohum, { tac: kur.tac, len, diz: kur.diz, devre: kur.devre ?? false }); /* maç işçisiyle aynı: devresi yazılmamış eski kayıt devresiz */ if (kur.oz === 'rastgele') rastgele(m); kurUygula(m, kur);
  const R = { bek: { n: 0, yakin: 0, n15: 0, dmin: [], skor: {} }, sav: { n: 0, goalSide: 0, presJob: 0, presAyar: 0, ustUste: 0, bosTehlike: 0, presMax: 0, presDag: {} }, olay: [], seg: [] };
  const SW = self.ALAN_SAHA.W, SC = self.ALAN_SAHA.CY, gx = df === 0 ? 0 : SW, hyp = Math.hypot; let ci = 0, segK = null;
  while (!m.over && m.tick < len) { while (ci < gecmis.length && gecmis[ci].tick <= m.tick) uygula(m, gecmis[ci++]); M.step(m);
    const h = m.holder; if (!h || h.team !== at) { segK = null; continue; } const D = m.ps.filter(p => p.team === df && p.role !== 'Bekçi'), A = m.ps.filter(p => p.team === at && p.role !== 'Bekçi'), sk = `${m.score[0]}-${m.score[1]}`;
    if (h.role === 'Bekçi') { const ds = D.map(p => hyp(p.x - h.x, p.y - h.y)).sort((a, b) => a - b); R.bek.n++; if (ds[0] < 8) R.bek.yakin++; R.bek.n15 += ds.filter(x => x < 18).length; if (m.tick % 6 === 0) R.bek.dmin.push(+ds[0].toFixed(1)); R.bek.skor[sk] = (R.bek.skor[sk] || 0) + 1;
      if (!segK || segK.t !== m.tick - 1) { segK = { t0: m.tick, sk, presler: 0, jobs: {} }; R.seg.push(segK); } segK.t = m.tick; segK.dur = m.tick - segK.t0; for (const p of D) { const j = (p.job || '-').split(' ')[0]; segK.jobs[j] = (segK.jobs[j] || 0) + 1; } segK.yakinMin = Math.min(segK.yakinMin ?? 1e9, ds[0]); continue; }
    segK = null; if (m.tick % 3) continue; R.sav.n++;
    const ex = hyp(gx - h.x, SC - h.y); R.sav.goalSide += D.filter(p => hyp(gx - p.x, SC - p.y) < ex).length / D.length;
    const pr = D.filter(p => /^pres|fake|kovala/.test(p.job || '')).length; R.sav.presJob += pr; R.sav.presMax = Math.max(R.sav.presMax, pr); R.sav.presDag[pr] = (R.sav.presDag[pr] || 0) + 1; R.sav.presAyar += m.tac[df].pres ?? 1;
    let uu = 0; for (let i = 0; i < D.length; i++) for (let j = i + 1; j < D.length; j++) if (hyp(D[i].x - D[j].x, D[i].y - D[j].y) < 3) uu++; R.sav.ustUste += uu ? 1 : 0;
    let bos = 0; for (const a of A) { if (a === h) continue; const da = hyp(gx - a.x, SC - a.y); if (da > 35) continue; const nd = Math.min(...D.map(p => hyp(p.x - a.x, p.y - a.y))); if (nd > 6) bos++; } R.sav.bosTehlike += bos ? 1 : 0; let kb = 0; for (const a of A) { if (a === h || Math.abs(a.y - SC) < 15 * self.ALAN_SAHA.KY) continue; const nd = Math.min(...D.map(p => hyp(p.x - a.x, p.y - a.y))); if (nd > 8 && (a.y - SC) * (h.y - SC) > 0) kb++; } R.sav.kB = (R.sav.kB || 0) + (kb ? 1 : 0); }
  R.skor = m.score; R.tick = m.tick; R.seg = R.seg.filter(s => s.dur > 30).slice(-10).map(s => ({ dk: Math.floor(s.t0 / 60), sure: (s.dur / 60).toFixed(1), skor: s.sk, enYakin: +(s.yakinMin || 0).toFixed(1), isler: s.jobs }));
  postMessage({ ok: true, R }); } catch (x) { postMessage({ ok: false, err: x.message + ' ' + String(x.stack).slice(0, 300) }); } };
