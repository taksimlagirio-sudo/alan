// JS motorundan izleme kareleri (alan-kos kare ile aynı biçim).
// node sina/kare-js.js ../alan-v5 <kayit.json> <tik> <cikti.json>
const path = require('path'), fs = require('fs');
const { M, kurMac } = require('./yukle')(path.resolve(process.argv[2]));
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni)), N = +process.argv[4];
const r2 = v => (v == null || !isFinite(v)) ? null : Math.round(v * 100) / 100;
const isler = [], isNo = new Map(), no = j => { j = j || ''; if (!isNo.has(j)) { isNo.set(j, isler.length); isler.push(j); } return isNo.get(j); };
const k = [], olay = []; let e0 = m.events[0];
while (m.tick < N && !m.over) {
  M.step(m);
  const b = m.holder ? null : m.ball;
  const tx = m.holder ? m.holder.x : (b ? b.x : null), ty = m.holder ? m.holder.y : (b ? b.y : null);
  const s = [m.tick, m.holder ? m.ps.indexOf(m.holder) : -1, r2(m.ball ? m.ball.x : tx), r2(m.ball ? m.ball.y : ty), m.score[0], m.score[1]];
  for (const p of m.ps) s.push(r2(p.x), r2(p.y), no(p.job), r2(p.en));
  k.push(s);
  const yeni = []; for (const e of m.events) { if (e === e0) break; yeni.push(e); } e0 = m.events[0];
  for (const e of yeni.reverse()) olay.push([m.tick, e.team, e.text]);
}
fs.writeFileSync(process.argv[5], JSON.stringify({ motor: "JS motoru", ad: m.ps.map(p => p.name), takim: m.ps.map(p => p.team), rol: m.ps.map(p => p.role), isler, olay, k }));
