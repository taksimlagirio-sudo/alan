// ALAN Lab işçisi: bir maçı görüntüsüz, baştan sona oynar ve özet istatistiği döndürür.
self.window = self;
importScripts('core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js');
const M = self.AlanMatch;
function egUy(m, R) { for (const p of m.ps) { const r = R[p.team] && R[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } }
function kurUygula(m, d) { if (d.oyuncular) for (const t of [0, 1]) { const A = d.oyuncular[t]; if (!A) continue; for (const p of m.ps) { if (p.team !== t) continue; const o = A[p.i]; if (o) { p.name = o.ad; p.a = { ...p.a, ...o.a }; } } } if (d.roller) { M.assignRoles(m, d.roller); egUy(m, d.roller); } if (d.diz) for (const t of [0, 1]) if (d.diz[t]) M.dizUygula(m, t, d.diz[t]); }
onmessage = e => { const d = e.data, t0 = Date.now();
  try { const m = M.createMatch(d.tohum, { tac: d.tac, len: d.len || 5400, diz: d.diz }); kurUygula(m, d);
    const sahip = [0, 0], kaz = [0, 0], yz = [0, 0]; let son = -1;
    while (!m.over) { M.step(m); const h = m.holder; if (h && h.role !== 'Bekçi') { sahip[h.team]++; if (son !== -1 && son !== h.team) kaz[h.team]++; son = h.team; } }
    const st = m.st, s2 = k => st[k] ? [st[k][0], st[k][1]] : [0, 0];
    postMessage({ id: d.id, ok: true, skor: [m.score[0], m.score[1]], sahip, kaz, pas: s2('pass'), tutan: s2('passOk'), gonderme: s2('shot'), fake: s2('fake'), ms: Date.now() - t0 });
  } catch (x) { postMessage({ id: d.id, ok: false, err: String(x && x.stack || x) }); } };
