// Kalibrasyonun "çöküş" vakaları: akıl ≥%95 dedi, pas 12 tekrarın hepsinde kaybedildi. Gerçek maçta o pastan sonra ne olduğunu anlatır.
// Kullanım: node tools/v5-kalib-iz.js <motor> <senaryo> <tik,tik,...>
const { kur } = require('./kurulum'); const [dir, sS, tS] = process.argv.slice(2), sc = +sS, TS = tS.split(',').map(Number);
const { M, m, adim } = kur(dir, sc, 0, { 'match.js': ['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'] });
const r1 = v => Math.round(v * 10) / 10, yer = p => `(${r1(p.x)},${r1(p.y)})`;
for (const T of TS) { while (m.tick < T) adim(); const o = m.fl && m.fl.o; if (!o) { console.log(JSON.stringify({ sc, T, hata: 'pas yok' })); continue; }
  const b = m.ball, team = b.team, pasci = b.from, q = o.q, ev0 = m.events.length ? m.events[0] : null, yol = [];
  const kim = m.ps.filter(p => p.team !== team).map(p => ({ ad: p.name, rol: p.role, d: r1(Math.hypot(p.x - b.x, p.y - b.y)) })).sort((a, c) => a.d - c.d).slice(0, 3);
  let t = 0, son = null; while (t++ < 400 && !m.over) { const s0 = m.score.slice(); adim(); if (t % 10 === 0 && m.ball) yol.push(yer(m.ball)); if (m.score[0] !== s0[0] || m.score[1] !== s0[1]) { son = 'gol'; break; } if (m.holder) { son = m.holder; break; } }
  const evs = []; for (const e of m.events) { if (e === ev0) break; evs.push(e.text); }
  console.log(JSON.stringify({ sc, T, tur: o.kendine ? 'kendine' : o.kind, P: o.det.P, Pk: o.det.Pk, Pl: o.det.Pl, aklinSonu: o.det.end && yer(o.det.end), pasci: pasci && pasci.name + yer(pasci), alici: q && q.name + yer(q), hedef: o.to && yer(o.to), yakinRakip: kim,
    sonuc: son === 'gol' ? 'gol' : son ? `${son.name} (${son.team === team ? 'bizden' : 'rakip'}${son.role === 'Bekçi' ? ', Bekçi' : ''}) ${yer(son)} ${t} tik sonra` : 'sahipsiz', top: yol.slice(0, 8).join(' '), olaylar: evs.reverse() })); }
