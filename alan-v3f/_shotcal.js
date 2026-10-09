window.__shotCal = function (seed0, budget) { const M = window.AlanMatch, A = window.AlanDecide, P = A.D, rows = []; const od = A.decide; let pend = null;
  A.decide = (h, ps, ch, r, t) => { const res = od(h, ps, ch, r, t); if (!P._inLook && res.best && res.best.kind === 'gönder') { const o = res.best, sh = res.opts.filter(x => x.kind === 'gönder'); const true30 = A.imagine({ ...h, a: { ...h.a, okuma: 20 } }, ps, { x: h.x, y: h.y, ...o.launch, ch }, Math.random, 30).Pk; pend = { pk: o.det.Pk, true30, nOpt: sh.length, d: Math.hypot((h.team === 0 ? 100 : 0) - h.x, 25 - h.y), team: h.team, res: null }; } return res; };
  const t0 = performance.now(); let k = 0; const tac = [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Düşük', genislik: 'Dar', tempo: .8, risk: .8, kazaninca: 'Kontra' }, { sistem: 'Adam adama', pres: 2, arkada: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Dengeli' }];
  try { while (performance.now() - t0 < budget) { const m = M.createMatch(seed0 + k++, { len: 1200, tac: k % 2 ? tac : [tac[1], tac[0]] }); let cur = null, s = [0, 0];
    while (!m.over) { const hb = m.holder; M.step(m); if (pend && hb && !m.holder && m.ball) { cur = { ...pend, ball: m.ball }; pend = null; }
      if (cur) { if (m.score[cur.team] > s[cur.team]) { cur.res = 'sayı'; } else if (m.holder || m.ball !== cur.ball) cur.res = m.holder && m.holder.role === 'Bekçi' ? 'Bekçi' : m.holder && m.holder.team !== cur.team ? 'savunmacı' : 'diğer'; if (cur.res) { delete cur.ball; rows.push(cur); cur = null; } }
      s = [m.score[0], m.score[1]]; } } } finally { A.decide = od; }
  return { rows, k }; };
window.__shotCal2 = function (seed0, budget) { const M = window.AlanMatch, A = window.AlanDecide, P = A.D, rows = []; const od = A.decide; let pend = null;
  A.decide = (h, ps, ch, r, t) => { const res = od(h, ps, ch, r, t); if (!P._inLook && res.best && res.best.kind === 'gönder') { const o = res.best; const ok0 = h.a.okuma; h.a = { ...h.a, okuma: 20 }; const tr = A.imagine(h, ps, { x: h.x, y: h.y, ...o.launch, ch }, Math.random, 30).Pk; h.a = { ...h.a, okuma: ok0 }; const gx = h.team === 0 ? 100 : 0, L = Math.hypot(o.launch.vx, o.launch.vy) || 1, ux = o.launch.vx / L, uy = o.launch.vy / L; let nd = 99; for (const p of ps) { if (p.team === h.team || p.role === 'Bekçi') continue; const al = (p.x - h.x) * ux + (p.y - h.y) * uy; if (al <= 0) continue; nd = Math.min(nd, Math.abs(-(p.x - h.x) * uy + (p.y - h.y) * ux)); } pend = { pk: o.det.Pk, true30: tr, nd, v: L, d: Math.hypot((h.team === 0 ? 100 : 0) - h.x, 25 - h.y), team: h.team, shot: o.shot, ok: h.a.okuma, res: null }; } return res; };
  const t0 = performance.now(); let k = 0;
  try { while (performance.now() - t0 < budget) { const m = M.createMatch(seed0 + k++, { len: 99999 }); for (const p of m.ps) { if (p.role === 'Bekçi') continue; p.x = p.team === 0 ? Math.min(92, p.x + 42) : Math.max(60, p.x - 8); p.tx = p.x; p.ty = p.y; }
      const h = m.ps.filter(p => p.team === 0 && p.role !== 'Bekçi').sort((a, b) => b.x - a.x)[2]; m.holder = h; m.ball = null; m.ch = .45; m.decT = 10;
      let cur = null; const s0 = m.score[0];
      for (let t = 0; t < 420; t++) { const hb = m.holder; M.step(m); if (pend && hb && !m.holder && m.ball) { cur = { ...pend, ball: m.ball }; pend = null; }
        if (cur) { if (m.score[0] > s0) cur.res = 'sayı'; else if (m.holder || m.ball !== cur.ball) cur.res = m.holder && m.holder.role === 'Bekçi' ? 'Bekçi' : m.holder && m.holder.team !== cur.team ? 'savunmacı' : 'diğer'; if (cur.res) { delete cur.ball; rows.push(cur); break; } }
        if (m.holder && m.holder.team === 1) break; } } } finally { A.decide = od; }
  return { rows, k }; };
window.__passCal = function (seed0, budget, tacF) { const M = window.AlanMatch, A = window.AlanDecide, P = A.D, rows = []; const od = A.decide; let pend = null;
  A.decide = (h, ps, ch, r, t) => { const res = od(h, ps, ch, r, t); const o = res.best; if (!P._inLook && o && o.q && o.det && o.det.P != null) pend = { k: o.kind + (o.drib ? '·' + o.drib : '') + (o.ot ? '·tek' : ''), P: o.det.P, team: h.team, L: Math.hypot((o.T || o.to).x - h.x, (o.T || o.to).y - h.y) }; return res; };
  const t0 = performance.now(); let k = 0;
  try { while (performance.now() - t0 < budget) { const m = M.createMatch(seed0 + k++, { len: 900, tac: tacF ? tacF(k) : undefined }); let cur = null;
    while (!m.over) { const hb = m.holder; M.step(m); if (pend && hb && !m.holder && m.ball) { cur = { ...pend, ball: m.ball }; pend = null; } else if (m.holder) pend = null;
      if (cur && m.holder) { cur.ok = m.holder.team === cur.team; delete cur.ball; rows.push(cur); cur = null; } } } } finally { A.decide = od; }
  return { rows, k }; };