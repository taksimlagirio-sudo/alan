(function () {
  // Kalibrasyon: taşıyıcının kafasındaki tahmin ile canlı maçta gerçekte olan, pas türü başına.
  // "Tuttu" = Çekirdek arkadaşa ulaştı ve arkadaş onu 8 tik sonra hâlâ elinde tutuyor (karşılamadan sonraki ikili mücadele dahil).
  const KEEP = 8;
  function kindOf(o) { return o.kind === 'gönder' ? 'gönder' : o.kind === 'pas' ? (o.firm ? 'sert pas' : 'ayağına pas') : o.kind === 'önüne' ? 'önüne pas' : o.kind === 'kenardan' ? 'kenardan' : null; }
  function watch(m, step, maxT, rec) {
    const D = window.AlanDecide, od = D.decide; let last = null;
    D.decide = (h, ps, ch, r, tac) => { const res = od(h, ps, ch, r, tac); last = { h, o: res.best }; return res; };
    const open = [];
    try {
      for (let t = 0; t < maxT && !m.over; t++) {
        const hb = m.holder; last = null; step(m);
        if (hb && !m.holder && m.ball && last && last.h === hb && last.o.launch) { const k = kindOf(last.o); if (k && k !== 'gönder') open.push({ k, P: last.o.det.P, team: hb.team, b: m.ball, got: null, t0: m.tick }); }
        for (const e of open) {
          if (e.done) continue;
          if (e.got == null) { if (m.holder) { if (m.holder.team === e.team && m.ball !== e.b) e.got = m.tick; else { e.done = true; e.ok = false; } } else if (m.ball !== e.b && !m.holder) { e.done = true; e.ok = false; } else if (m.tick - e.t0 > 300) { e.done = true; e.ok = false; } }
          else if (m.tick - e.got >= KEEP) { e.done = true; e.ok = !!(m.holder && m.holder.team === e.team); }
          else if (!m.holder || m.holder.team !== e.team) { if (!m.holder && m.ball && m.ball.team === e.team && m.ball.from && m.ball.from.team === e.team) { e.done = true; e.ok = true; } else { e.done = true; e.ok = false; } }
        }
        if (m._stop && m._stop()) break;
      }
    } finally { D.decide = od; }
    for (const e of open) if (e.done) { const r = rec[e.k] || (rec[e.k] = { n: 0, pred: 0, real: 0 }); r.n++; r.pred += e.P; r.real += e.ok ? 1 : 0; }
  }
  function run(o) {
    o = o || {}; const M = window.AlanMatch, SC = window.AlanScenes, rec = {};
    for (let i = 0; i < (o.matches ?? 4); i++) { const m = M.createMatch(5000 + i * 7919, {}); m.len = o.len || 2700; watch(m, M.step, m.len, rec); }
    if (o.scenes !== false && SC && SC.play) for (const sc of SC.S.filter(s => s.group)) for (let i = 0; i < (o.perScene ?? 12); i++) { const r = { m: null }; const M2 = window.AlanMatch, os = M2.step; let mm = null; M2.step = x => { mm = x; os(x); }; try { SC.play(sc, 900 + i * 41, null, 1); } finally { M2.step = os; } if (mm) { mm.over = false; let stop = false; mm._stop = () => (mm.score[0] + mm.score[1] > 0) || (mm.holder && mm.holder.team === 1 && mm.tick > 2); try { watch(mm, x => { try { os(x); } catch (e) { x.over = true; } }, 360, rec); } catch (e) {} } }
    return Object.entries(rec).map(([k, r]) => ({ kind: k, n: r.n, pred: r.pred / r.n, real: r.real / r.n }));
  }
  window.AlanCalib = { run, KEEP };
})();
