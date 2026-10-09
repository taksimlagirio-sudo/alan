// Kopya testi: aynı maç (1) kesintisiz, (2) her K tikte cloneMatch ile kopyalanıp kopyadan devam edilerek oynanır.
// Her tikte tam durumun parmak izi (konum, hız, hedef, Çekirdek, taşıyıcı, skor, son kararın seçenek değerleri) aynı olmalı.
// Kullanım: node tools/test-clone.js <motor> <tohum> <tik> [K=60]
const { load } = require('./load'); const [dir, sS, lS, kS] = process.argv.slice(2), seed = +sS || 1, len = +lS || 1500, K = +kS || 60;
const g = load(dir, seed), M = g.AlanMatch; if (!M.cloneMatch) { console.log('bu motorda cloneMatch yok'); process.exit(2); }
const tac = [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Düşük', genislik: 'Dar', tempo: .8, risk: .8, kazaninca: 'Kontra' }, { sistem: 'Adam adama', pres: 2, arkada: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Dengeli' }];
const f64 = new Float64Array(1), u32 = new Uint32Array(f64.buffer);
const fp = m => { let h = 2166136261 >>> 0; const mix = v => { f64[0] = v; h = Math.imul(h ^ u32[0], 16777619) >>> 0; h = Math.imul(h ^ u32[1], 16777619) >>> 0; };
  for (const p of m.ps) { mix(p.x); mix(p.y); mix(p.vx); mix(p.vy); mix(p.tx ?? -1); mix(p.ty ?? -1); } if (m.ball) { mix(m.ball.x); mix(m.ball.y); mix(m.ball.vx); mix(m.ball.vy); }
  mix(m.holder ? m.ps.indexOf(m.holder) : -1); mix(m.ch); mix(m.score[0]); mix(m.score[1]); if (m.lastDec) for (const o of m.lastDec.opts) { mix(o.v); mix(o.P ?? -1); } return h; };
// (1) kesintisiz
const rand0 = Math.random; const A = M.createMatch(seed, { tac, len }), ha = []; while (!A.over && A.tick < len) { M.step(A); ha.push(fp(A)); }
// (2) her K tikte kopyadan devam (Math.random kullanan bir yol varsa ayrı akışta olsun diye yeniden tohumlanır)
const g2 = load(dir, seed); let B = g2.AlanMatch.createMatch(seed, { tac, len }); const hb = [];
while (!B.over && B.tick < len) { if (B.tick > 0 && B.tick % K === 0) B = g2.AlanMatch.cloneMatch(B); g2.AlanMatch.step(B); hb.push(fp(B)); }
let i = 0; while (i < Math.min(ha.length, hb.length) && ha[i] === hb[i]) i++;
console.log(i === ha.length && ha.length === hb.length ? `AYNI · ${ha.length} tik · her ${K} tikte kopyadan devam · skor ${A.score.join('-')}` : `AYRILDI · tik ${i + 1} (ilk kopya ${K}. tikte) · kesintisiz ${ha.length} tik, kopyalı ${hb.length}`);
process.exit(i === ha.length && ha.length === hb.length ? 0 : 1);
