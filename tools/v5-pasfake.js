// Her pasın atıldığı anda fake durumu: bu taşıyıcının fake'i sürüyor mu (m.fake), aldanmış sayılan rakip var mı, fake başlayalı kaç tik oldu.
// kalib3 satırlarıyla (sc, t) eşleştirmek için. Kullanım: node tools/v5-pasfake.js <motor> <senaryo>
const { kur } = require('./kurulum'); const [dir, sS] = process.argv.slice(2), sc = +sS;
const { M, m, adim } = kur(dir, sc, 0, { 'match.js': ['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'] });
let fl = null, F = null; const out = [];
while (!m.over && m.tick < 5400) { const f0 = m.fake; if (f0) F = { hid: f0.hid, t0: f0.t0, n: f0.fooled.size, react: f0.react }; adim();
  if (m.fl && m.fl !== fl && m.fl.o) { fl = m.fl; const from = m.ball && m.ball.from; if (from && F && F.hid === from.id && m.tick - F.t0 < 120) out.push({ t: m.tick, fakeT: m.tick - F.t0, aldanan: F.n, tepkiSonra: F.react - m.tick, kind: fl.o.kind }); } }
console.log(JSON.stringify({ sc, paslar: out }));
