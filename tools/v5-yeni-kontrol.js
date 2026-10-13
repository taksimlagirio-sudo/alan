// zip33 yeni sistemlerinin hızlı doğruluk kontrolleri (tek süreç, kısa maçlar).
// T0 maç içi taktik değişikliği (işçinin uygula'sı) gerçekten temel taktiğe geçiyor mu · T1 kopyadan yeniden oynatma baştan oynatmayla bit bit aynı mı · T2 paket adı davranışı değiştiriyor mu
// Kullanım: node tools/v5-yeni-kontrol.js <motor> <test: T0|T1|T2> [tohum=1] [tik=2400]
const { load } = require('./load'); const [dir, test, sS, lS] = process.argv.slice(2), seed = +sS || 1, LEN = +lS || 2400; const g = load(dir, seed), M = g.AlanMatch;
const STY = [['Alan', 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], ['Alan', 1, 'Düşük', 'Dar', .4, .3, 'Kontra']], tOf = a => ({ sistem: a[0], pres: a[1], blok: a[2], genislik: a[3], tempo: a[4], risk: a[5], kazaninca: a[6], siddet: .5, tetik: ['Geri pas', 'Kanatta'], kpSure: 3 });
const yeni = () => M.createMatch(seed, { len: 5400, tac: [tOf(STY[0]), tOf(STY[1])], diz: [{ sav: '2-2-2', huc: '2-1-3' }, { sav: '3-2-1', huc: '2-2-2' }] });
const f64 = new Float64Array(1), u32 = new Uint32Array(f64.buffer); const hash = m => { let h = 0; const add = x => { f64[0] = x ?? -1; h = (Math.imul(h, 31) + u32[0]) | 0; h = (Math.imul(h, 31) + u32[1]) | 0; }; for (const p of m.ps) { add(p.x); add(p.y); add(p.vx); add(p.vy); } if (m.ball) { add(m.ball.x); add(m.ball.y); } add(m.ch); add(m.score[0]); add(m.score[1]); return (h >>> 0).toString(16); };
const uygula = (m, c) => { m.tac[c.t] = { ...m.tac[c.t], ...c.tac }; }; /* mac-isci.js uygula ile aynı */
const DEG = { t: 0, tac: { blok: 'Yüksek', siddet: 1, tempo: .9 } }, TD = 900;
if (test === 'T0') { const m = yeni(); while (m.tick < TD) M.step(m); const once = { ...(m.tac0 && m.tac0[0]) }; uygula(m, DEG); let pj0 = 0, n0 = 0; for (let k = 0; k < 600; k++) { M.step(m); }
  const b = (m.tac0 && m.tac0[0]) || m.tac[0]; console.log(JSON.stringify({ test, once: { blok: once.blok, siddet: once.siddet, tempo: once.tempo }, sonra_tac0: { blok: b.blok, siddet: b.siddet, tempo: b.tempo }, sonra_tac: { blok: m.tac[0].blok, siddet: m.tac[0].siddet, tempo: m.tac[0].tempo }, gecti: b.blok === 'Yüksek' && b.siddet === 1 && b.tempo === .9 })); }
if (test === 'T1') { const a = yeni(), iz = []; let snap = null; while (a.tick < LEN) { if (a.tick === 600) snap = M.kopya(a); if (a.tick === TD) uygula(a, DEG); M.step(a); if (a.tick % 300 === 0) iz.push(hash(a)); }
  const b = M.kopya(snap), iz2 = []; while (b.tick < LEN) { if (b.tick === TD) uygula(b, DEG); M.step(b); if (b.tick % 300 === 0) iz2.push(hash(b)); }
  const ilk = iz.slice(-iz2.length); console.log(JSON.stringify({ test, ayni: JSON.stringify(ilk) === JSON.stringify(iz2), baştan: ilk.slice(-3), kopyadan: iz2.slice(-3), skor: [a.score, b.score] })); }
if (test === 'T2') { const run = ad => { const m = yeni(); if (ad) for (const p of m.ps) { p.rh = 'Deneme'; p.rs = 'Deneme'; } while (m.tick < LEN) M.step(m); return hash(m); }; const x = run(false), y = run(true); console.log(JSON.stringify({ test, ayni: x === y, normal: x, adDegisik: y })); }
