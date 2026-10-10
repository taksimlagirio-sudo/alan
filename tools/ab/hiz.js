// 8. madde ön ölçümü: tik başına maliyet (refleks maç · tam beyin · ilk bakışla hücum sonuna kadar oynatma)
const fs = require('fs'), { load } = require('../load'); const dir = process.argv[2] || 'alan-v4g';
const g = load(dir, 5); for (const f of ['vs-table.js', 'pas-table.js']) (0, eval)(fs.readFileSync(dir + '/' + f, 'utf8'));
const M = g.AlanMatch, B = g.AlanBeyin;
let t = Date.now(); B.A.bak = false; const a = M.createMatch(5, { len: 1800 }); while (!a.over) M.step(a); console.log('refleks', ((Date.now() - t) / 1800).toFixed(2), 'ms/tik skor', a.score);
B.A.bak = true; t = Date.now(); const b = M.createMatch(5, { len: 600 }); const cnt = {}; while (!b.over) { let p = B.planla(b); while (p) { cnt[p.tip] = (cnt[p.tip] || 0) + p.jobs.length; const t0 = Date.now(); p = p.bitir(p.jobs.map(j => B.rollout(b, j))) || null; cnt[p ? 'x' : 'ms'] = 0; } M.step(b); } console.log('tam beyin', ((Date.now() - t) / 600).toFixed(1), 'ms/tik', JSON.stringify(cnt));
