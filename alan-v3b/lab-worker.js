// Laboratuvar iş parçacığı: tam v3b beyniyle (kısıtlama yok) tek bir maçı oynatır, özetini döndürür.
self.window = self;
importScripts('core.js', 'shot-table.js', 'decide.js', 'match.js', 'shape.js');
self.onmessage = e => {
  const q = e.data, M = self.AlanMatch;
  try {
    const m = M.createMatch(q.seed, { tac: q.tac }); if (q.len) m.len = q.len;
    const t0 = Date.now(); M.run(m);
    const st = m.st;
    self.postMessage({ id: q.id, ok: true, ms: Date.now() - t0, score: m.score, poss: st.poss, shot: st.shot, pass: st.pass, passOk: st.passOk, steal: st.steal, lob: st.lob || [0, 0], ot: st.ot || [0, 0], cal: st.cal || null });
  } catch (err) { self.postMessage({ id: q.id, ok: false, msg: String(err && err.message || err) }); }
};
