window.__beh = function (tacA, seeds, len) { const M = window.AlanMatch, R = { spread: 0, sN: 0, hold: 0, hN: 0, seqT: 0, seqP: 0, seqN: 0, deep: 0 };
  for (const sd of seeds) { const m = M.createMatch(sd, { len, tac: [tacA, {}] }); let seq = null, hT = 0, prevH = null;
    while (!m.over) { M.step(m); const h = m.holder, poss = h ? h.team : m.ball ? m.ball.team : -1;
      if (h && h.team === 0) { if (h === prevH) hT++; else { if (prevH && prevH.team === 0) { R.hold += hT; R.hN++; } hT = 0; } } else if (prevH && prevH.team === 0) { R.hold += hT; R.hN++; hT = 0; }
      prevH = h;
      if (poss === 0) { if (!seq) seq = { t: m.tick, p: m.st.pass[0] }; } else if (seq && poss === 1) { R.seqT += m.tick - seq.t; R.seqP += m.st.pass[0] - seq.p; R.seqN++; seq = null; }
      if (h && h.team === 0 && h.x > 50 && m.tick % 5 === 0) { const ys = m.ps.filter(p => p.team === 0 && p.role !== 'Bekçi').map(p => p.y); R.spread += Math.max(...ys) - Math.min(...ys); R.sN++; const yd = m.ps.filter(p => p.team === 1 && p.role !== 'Bekçi').map(p => p.y); R.dsp = (R.dsp || 0) + Math.max(...yd) - Math.min(...yd); const bs = m.ps.filter(p => p.team === 0 && p.role !== 'Bekçi').map(p => Math.abs(p.ty - p.y)); R.off = (R.off || 0) + bs.reduce((a, x) => a + x, 0) / bs.length; } } }
  return R; };