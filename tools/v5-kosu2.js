// Arkaya koşu ölçümü: son çizginin (en geriki Bekçi dışı savunmacı) arkasına kaç koşu atılıyor, kaçına pas gidiyor, pas tutuyor mu.
// Koşu başı: topa sahip takımın topsuz oyuncusu (Bekçi değil) hedefini çizginin en az 2 birim arkasına koyar, kendisi henüz çizgiyi geçmemiştir ve ileri hareket eder.
// Koşu 150 tik (2,5 sn) açık sayılır; bu sürede koşucuya giden pas "koşuya pas"tır. Çizgiyi geçerse "geçti".
// LKOSU=<0..1>: Liman'ın (takım 0) Bekçi dışı oyuncularına koşu talimatı. Kullanım: node tools/v5-kosu2.js <motor> <senaryo> [tohum kayması]
const { kur } = require('./kurulum'); const [dir, sS, kS] = process.argv.slice(2), sc = +sS, KAY = +kS || 0;
const { M, m, ad, adim } = kur(dir, sc, KAY);
if (process.env.LKOSU != null) for (const p of m.ps) if (p.team === 0 && p.role !== 'Bekçi') p.tl = { ...(p.tl || {}), huc: { ...((p.tl && p.tl.huc) || {}), kosu: +process.env.LKOSU } };
const PK = ['pas', 'önüne', 'aşırt', 'kenardan'], T = [0, 1].map(() => ({ kosu: 0, gecti: 0, pas: 0, pasTut: 0, pasArka: 0, pasArkaTut: 0, kosuArka: 0, pasHep: 0, arkadaAn: 0, hucAn: 0 }));
const acik = new Map(); let fl = null, bek = null; /* acik: oyuncu → { t0, gecti } · bek: sonucu beklenen pas { team, q, koşuya, arkaya } */
const ileri = (t, x) => t === 0 ? x : 100 - x;
while (!m.over && m.tick < 5400) { adim(); const h = m.holder;
  if (bek && (h || m.ball && m.ball.done)) { if (h) { const ok = h.team === bek.team; if (bek.kosuya && ok) T[bek.team].pasTut++; if (bek.arkaya && ok) T[bek.team].pasArkaTut++; } bek = null; }
  if (m.fl && m.fl !== fl) { fl = m.fl; const b = m.ball, t = b ? b.team : null, q = fl.q; if (t != null && PK.includes(fl.kind) && q && q.team === t && fl.to) { const P = T[t]; P.pasHep++; let line = 0; for (const o of m.ps) if (o.team !== t && o.role !== 'Bekçi') line = Math.max(line, ileri(t, o.x)); const kz = acik.get(q), kosuya = !!(kz && m.tick - kz.t0 <= 150), arkaya = ileri(t, fl.to.x) > line + 1; if (kosuya) P.pas++; if (arkaya) P.pasArka++; if (kosuya && arkaya) P.kosuArka++; bek = { team: t, kosuya, arkaya }; } }
  if (!h) continue; const t = h.team, P = T[t]; let line = 0; for (const o of m.ps) if (o.team !== t && o.role !== 'Bekçi') line = Math.max(line, ileri(t, o.x)); P.hucAn++;
  for (const p of m.ps) { if (p.team !== t || p === h || p.role === 'Bekçi') { if (p.team !== t) acik.delete(p); continue; } const f = ileri(t, p.x), ft = p.tx != null ? ileri(t, p.tx) : f, v = t === 0 ? (p.vx || 0) : -(p.vx || 0);
    if (f > line) P.arkadaAn++;
    const kz = acik.get(p); if (kz && m.tick - kz.t0 > 150) acik.delete(p); if (kz && !kz.gecti && f > line + 1) { kz.gecti = 1; P.gecti++; }
    if (!acik.has(p) && ft > line + 2 && f <= line + 1 && v > .02) { acik.set(p, { t0: m.tick, gecti: 0 }); P.kosu++; } } }
console.log(JSON.stringify({ sc, ad, kay: KAY, lkosu: process.env.LKOSU ?? null, skor: m.score, T: T.map(x => ({ ...x, arkadaPay: +(x.arkadaAn / Math.max(1, x.hucAn)).toFixed(3) })) }));
