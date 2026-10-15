// Bir sekme anı, maçı birebir yeniden oynatarak: her tikte Çekirdek, Bekçi'nin dokunma yasağı, savunan takımın oyuncuları (topa varış tahmini est, kovalıyor mu, görev, hız).
// Kullanım: node tools/sekme-an.js <motor> <senaryo> <t0> <t1> <savunan takım>
const { kur } = require('./kurulum'); const [dir, sS, a, b, dS] = process.argv.slice(2), DT = +dS;
const { m, adim, g } = kur(dir, +sS, 0, { 'shape.js': [["    const dt = 1 - att, dtac =", "    if (self.__ch && !m._lite) self.__ch(m, fr, chase);\n    const dt = 1 - att, dtac ="]] });
let son = null; g.__ch = (mm, fr, chase) => { if (mm === m) son = { fr, chase: new Set(chase) }; };
while (m.tick < +b) { son = null; adim(); if (m.tick < +a) continue; const B = m.ball, H = m.holder;
  const satir = m.ps.filter(p => p.team === DT).map(p => { const r = son && son.fr && son.fr.arr.get(p); const cd = B && B.cds && B.cds.get(p); return `${p.name}${son && son.chase.has(p) ? '*' : ''} ${r ? 'est ' + r.est.toFixed(0) : ''}${cd != null && B.t < cd ? ' yasak ' + (cd - B.t) : ''} ${(p.job || '').replace(' · tepki', '~')} v${Math.hypot(p.vx || 0, p.vy || 0).toFixed(2)}`; }).join(' | ');
  const hz = m.ps.filter(p => p.team !== DT && son && son.fr && son.fr.arr.get(p)).sort((u, v) => son.fr.arr.get(u).est - son.fr.arr.get(v).est).slice(0, 3).map(p => `${p.name}${son.chase.has(p) ? '*' : ''} est ${son.fr.arr.get(p).est.toFixed(0)} ${(p.job || '').replace(' · tepki', '~')} v${Math.hypot(p.vx || 0, p.vy || 0).toFixed(2)}`).join(', ');
  console.log(`${(m.tick / 60).toFixed(2)} ${H ? 'TOP: ' + H.name : B ? `top (${B.x.toFixed(1)},${B.y.toFixed(1)}) v${Math.hypot(B.vx, B.vy).toFixed(2)}${B.defl ? ' sekti×' + B.defl : ''}` : '-'} || SAV: ${satir} || HÜC: ${hz}`); }
