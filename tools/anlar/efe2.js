const fs = require('fs'), [dir, kp, t1] = process.argv.slice(2); const { M, kurMac } = require('./yukle')(dir); const { oku } = require('./graf'); const K = require('../alp-kayit.json');
const m0 = kurMac(K.yeni), m = oku(JSON.parse(fs.readFileSync(kp)), () => M.kopya(m0.r)); for (const k of Object.keys(m0)) if (!(k in m)) m[k] = m0[k]; let sd = m.lastDec, son = m.events[0];
while (m.tick < +t1) { M.step(m); const d = m.lastDec; if (d && d !== sd) { sd = d; if (['Efe', 'Volkan'].includes(d.name)) console.log(`KARAR ${m.tick} ${d.name}: ${d.opts.slice(0, 4).map(o => `${o.kind}${o.q ? '→' + o.q : ''}${o.to ? `(${o.to.x.toFixed(0)},${o.to.y.toFixed(0)})` : ''} ${(o.v ?? 0).toFixed(3)}`).join(' | ')}`); }
  if (m.events[0] !== son) { let k = 0; while (k < m.events.length && m.events[k] !== son) k++; for (let j = k - 1; j >= 0; j--) console.log('   OLAY', m.tick, m.events[j].text); son = m.events[0]; } }
process.exit(0);
