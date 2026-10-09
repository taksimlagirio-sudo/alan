window.__shotLab = function (sc, N, ch) { const M = window.AlanMatch, K = window.AlanCore, A = window.AlanDecide, out = { kuyu: 0, bek: 0, def: 0, miss: 0, n: 0 };
  for (let i = 0; i < N; i++) { const m = M.createMatch(500 + i, { len: 99999 }); const t0 = m.ps.filter(p => p.team === 0 && p.role !== 'Bekçi'), t1 = m.ps.filter(p => p.team === 1 && p.role !== 'Bekçi'), bk1 = m.ps.find(p => p.team === 1 && p.role === 'Bekçi');
    t0.forEach((p, k) => { p.x = 20; p.y = 5 + k * 8; }); t1.forEach((p, k) => { p.x = 30; p.y = 5 + k * 8; }); for (const p of m.ps) { p.vx = p.vy = 0; p.tx = p.x; p.ty = p.y; }
    const h = t0[0]; h.x = sc.h[0]; h.y = sc.h[1]; sc.d.forEach((q, k) => { t1[k].x = q[0]; t1[k].y = q[1]; t1[k].tx = q[0]; t1[k].ty = q[1]; }); bk1.x = sc.k[0]; bk1.y = sc.k[1];
    m.holder = h; m.ball = null; m.ch = ch; m.fl = null; h.ca = Math.atan2(25 - h.y, 100 - h.x);
    A.D.look = false; const r = A.decide(h, m.ps, ch, Math.random, null); const o = r.opts.filter(o => o.kind === 'gönder' && o.shot === sc.shot)[0]; if (!o) continue;
    const cp = M.corePos(m), v = A.applyError(h, o.launch, Math.random, 0); m.ball = K.makeBall({ x: cp.x, y: cp.y, vx: v.vx, vy: v.vy, team: 0, ch, from: h }); m.holder = null; m.fl = { t0: m.tick, kind: 'gönder', to: o.to, end: { x: 100, y: 25 }, shot: o.shot };
    out.pk = (out.pk || 0) + o.det.Pk; out.n++; const s0 = m.score[0];
    for (let t = 0; t < 120; t++) { M.step(m); if (m.score[0] > s0) { out.kuyu++; break; } if (m.holder) { if (m.holder.role === 'Bekçi') out.bek++; else if (m.holder.team === 1) out.def++; else out.miss++; break; } if (t === 119) out.miss++; } }
  return out; };