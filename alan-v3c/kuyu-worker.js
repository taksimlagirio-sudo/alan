// Kuyu Önü pozisyonlarını sayfayı dondurmadan ayrı bir iş parçacığında oynatır.
self.window = self;
importScripts('core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'scenes.js', 'match.js', 'shape.js');
let job = 0;
const slim = r => ({ res: r.res, shots: r.shots, passes: r.passes, events: r.events.map(e => ({ text: e.text })), frames: r.frames, names: r.names, teams: r.teams });
self.onmessage = e => {
  const m = e.data, SC = self.AlanScenes, list = SC.S.filter(s => s.group);
  if (m.type === 'run') {
    const my = ++job, N = m.N || 10;
    const jobs = []; list.forEach((sc, i) => { jobs.push({ i, dec: true }); for (let k = 0; k < N; k++) jobs.push({ i, k }); });
    let j = 0;
    const next = () => {
      if (my !== job || j >= jobs.length) { if (j >= jobs.length) self.postMessage({ type: 'done', job: m.job }); return; }
      const it = jobs[j++], sc = list[it.i];
      try {
        if (it.dec) { const d = SC.run(sc, 10, m.ok); self.postMessage({ type: 'dec', job: m.job, i: it.i, dist: d.dist }); }
        else { const q = SC.play(sc, 100 + it.k * 37, m.ok); self.postMessage({ type: 'play', job: m.job, i: it.i, r: it.k === 0 ? slim(q) : { res: q.res, shots: q.shots, passes: q.passes } }); }
      } catch (err) { self.postMessage({ type: 'err', job: m.job, i: it.i, msg: String(err && err.message || err) }); }
      setTimeout(next, 0);
    };
    next();
  } else if (m.type === 'replay') {
    try { const q = SC.play(list[m.i], m.seed, m.ok); self.postMessage({ type: 'replay', i: m.i, r: slim(q) }); }
    catch (err) { self.postMessage({ type: 'err', i: m.i, msg: String(err && err.message || err) }); }
  }
};
