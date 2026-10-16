const fs = require('fs'), [dir, anF, adlar, n] = process.argv.slice(2); const { M, kurMac } = require('./yukle')(dir); const { oku } = require('./graf');
const A = JSON.parse(fs.readFileSync(anF)); const m0 = kurMac(A.kurulum.yeni), m = oku(A.kopya, () => M.kopya(m0.r)); for (const k of Object.keys(m0)) if (!(k in m)) m[k] = m0[k]; let son = m.events[0], sd = m.lastDec; const nm = x => m.ps.find(p => p.name === x), N = adlar.split(',');
for (let i = 0; i < +n; i++) { M.step(m); if (i % 6 === 0) { const h = m.holder; console.log(`${m.tick} top:${h ? h.name : '-'} ` + N.map(x => { const p = nm(x); return `${x}(${p.x.toFixed(0)},${p.y.toFixed(0)})[${p.job || ''}]` + (h && p !== h ? ` ${Math.hypot(p.x - h.x, p.y - h.y).toFixed(1)}br` : ''); }).join(' ')); }
  const d = m.lastDec; if (d && d !== sd) { sd = d; console.log(`   KARAR ${m.tick} ${d.name}: ${d.opts.slice(0, 3).map(o => `${o.kind}${o.q ? '→' + o.q : ''} ${(o.v ?? 0).toFixed(3)}`).join(' | ')}`); }
  if (m.events[0] !== son) { let k = 0; while (k < m.events.length && m.events[k] !== son) k++; for (let j = k - 1; j >= 0; j--) console.log('   OLAY', m.tick, m.events[j].text); son = m.events[0]; } }
process.exit(0);
