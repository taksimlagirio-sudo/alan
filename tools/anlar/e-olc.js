// E ölçümü: anı oynatır; Şimşek'in yeri, en yakın savunmacıya uzaklığı, görevi; Rüzgâr'ın kararları ve olaylar
const fs = require('fs'), [dir, anF, t1s] = process.argv.slice(2); const { M, kurMac } = require('./yukle')(dir); const { oku } = require('./graf');
const A = JSON.parse(fs.readFileSync(anF)); const m0 = kurMac(A.kurulum.yeni), m = oku(A.kopya, () => M.kopya(m0.r)); let sd = m.lastDec, son = m.events[0];
while (m.tick < +t1s) { M.step(m); const S = m.ps.find(p => p.name === 'Şimşek'), R = m.ps.find(p => p.name === 'Rüzgâr');
  if (m.tick % 8 === 0 && m.tick > 6840) { let nd = 1e9, nk = ''; for (const o of m.ps) if (o.team !== S.team && o.role !== 'Bekçi') { const d = Math.hypot(o.x - S.x, o.y - S.y); if (d < nd) { nd = d; nk = o.name; } } console.log(`${m.tick} Şimşek (${S.x.toFixed(1)},${S.y.toFixed(1)}) en yakın ${nk} ${nd.toFixed(1)} br · Rüzgâr'a ${Math.hypot(S.x - R.x, S.y - R.y).toFixed(1)} br · ${S.job} · top: ${m.holder ? m.holder.name : '-'}`); }
  const d = m.lastDec; if (d && d !== sd) { sd = d; console.log(`   KARAR ${m.tick} ${d.name}: ${d.opts.slice(0, 3).map(o => `${o.kind}${o.q ? '→' + (o.q.name || o.q) : ''} ${(o.v ?? 0).toFixed(3)}`).join(' | ')}`); }
  if (m.events[0] !== son) { let k = 0; while (k < m.events.length && m.events[k] !== son) k++; for (let j = k - 1; j >= 0; j--) console.log(`   OLAY ${m.tick} ${m.events[j].text}`); son = m.events[0]; } }
process.exit(0);
