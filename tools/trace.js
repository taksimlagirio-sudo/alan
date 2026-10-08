// Bir maçı tik tik oynatır ve her tikin durumunun parmak izini çıkarır (konumlar, hızlar, Çekirdek, taşıyıcı, skor, son karar).
// Kullanım: node tools/trace.js <motor klasörü> <tohum> <tik> [çıktı.json]
const { load } = require('./load');
const [dir, seedS, lenS, out] = process.argv.slice(2), seed = +seedS || 1, len = +lenS || 600;
const ctx = load(dir, seed), M = ctx.AlanMatch;
const tac = [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Düşük', genislik: 'Dar', tempo: .8, risk: .8, kazaninca: 'Kontra' }, { sistem: 'Adam adama', pres: 2, arkada: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Dengeli' }];
const m = M.createMatch(seed, { tac: seed % 2 ? tac : undefined, len });
const f64 = new Float64Array(1), u32 = new Uint32Array(f64.buffer);
let h = 2166136261 >>> 0; const mix = v => { f64[0] = v; h = Math.imul(h ^ u32[0], 16777619) >>> 0; h = Math.imul(h ^ u32[1], 16777619) >>> 0; };
const hs = [], t0 = Date.now();
while (!m.over && m.tick < len) {
  M.step(m); h = 2166136261 >>> 0;
  for (const p of m.ps) { mix(p.x); mix(p.y); mix(p.vx); mix(p.vy); mix(p.tx ?? -1); mix(p.ty ?? -1); }
  if (m.ball) { mix(m.ball.x); mix(m.ball.y); mix(m.ball.vx); mix(m.ball.vy); }
  mix(m.holder ? m.ps.indexOf(m.holder) : -1); mix(m.ch); mix(m.score[0]); mix(m.score[1]);
  if (m.lastDec) for (const o of m.lastDec.opts) { mix(o.v); mix(o.P ?? -1); }
  hs.push(h.toString(16));
}
const res = { seed, len, ms: Date.now() - t0, score: m.score, st: m.st, hs };
if (out) require('fs').writeFileSync(out, JSON.stringify(res));
console.log(JSON.stringify({ seed, ticks: hs.length, ms: res.ms, score: m.score, last: hs[hs.length - 1] }));
