// Pas kalibrasyonu (zip33 ve sonrası): v5-kalib2.js'in yöntemi, 16 maç kurulumuyla (kurulum.js) ve motorun kendi kopyasıyla (AlanMatch.kopya).
// Seçilen pasın anı (pastan önceki tik) kopyalanır:
//   secim  : maçta seçilirken aklın tahmini (o.det.P, az denemeyle)
//   model  : aynı anda aynı hamle için aklın tahmini, çok denemeyle (D.trials = TN)  → secim − model = en iyiyi seçme yanılgısı
//   gercek : aynı an, hamle zorla aynı, gerçek maç kurallarıyla RN kez farklı zarla  → model − gercek = aklın hesabının hatası
// Sonuç: pastan sonra topa ilk sahip olan takım (gol = atan takım), 400 tik içinde.
// Kullanım: node tools/v5-kalib3.js <motor> <senaryo 0-15> [örnekleme=3] [RN=16] [TN=24]
const { kur } = require('./kurulum'); const [dir, sS, oS, rS, tS] = process.argv.slice(2), sc = +sS, ORN = +oS || 3, RN = +rS || 16, TN = +tS || 24;
const { M, m, adim, g } = kur(dir, sc, 0, { 'match.js': ['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'] }); const AD = g.AlanDecide, D = AD.D;
const klon = (x, s) => { const c = M.kopya(x); if (s != null) c.r.s = s >>> 0; return c; };
const ayni = (x, o) => x.kind === o.kind && !!x.kendine === !!o.kendine && (x.q ? x.q.id : null) === (o.q ? o.q.id : null) && (!o.to || (x.to && Math.hypot(x.to.x - o.to.x, x.to.y - o.to.y) < .5));
const mapO = (o, c) => { const r = { ...o }; if (o.q) r.q = c.ps.find(p => p.id === o.q.id); return r; };
function zorlaAdim(c, hid, o, modelOku) { const od = AD.decide; let model = null, ilk = true;
  AD.decide = function (hh, ps, ch, rnd, tac) { if (ilk && hh.id === hid) { ilk = false; if (modelOku) { const tr = D.trials; D.trials = TN; try { const rr = M.kopya(rnd); rr.s = 12345; const r = od.call(this, hh, ps, ch, rr, tac), x = r.opts.find(x => ayni(x, o)); model = x && x.det ? x.det.P : null; } finally { D.trials = tr; } } const oo = mapO(o, c); return { best: oo, opts: [oo] }; } return od.apply(this, arguments); };
  try { M.step(c); } finally { AD.decide = od; } return { model, atildi: !ilk }; }
function sonuc(c, team) { for (let t = 0; t < 400; t++) { const s0 = c.score.slice(); M.step(c); if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) return c.score[team] !== s0[team] ? 1 : 0; if (c.holder) return c.holder.team === team ? 1 : 0; if (c.over) return null; } return null; }
let once = null, fl = null, n = 0;
while (!m.over && m.tick < 5400) { once = m.holder ? klon(m) : null; adim();
  if (m.fl && m.fl !== fl && m.fl.o) { fl = m.fl; const o = fl.o; if (o.kind === 'gönder' || o.kind === 'çalım' || o.kind === 'fake' || !o.det || o.det.P == null || o.ot || !once) continue; if ((n++ % ORN) !== 0) continue;
    const pasci = m.ball.from || o.q, hid = (o.kendine ? o.q : pasci).id, team = m.ball.team, P0 = once.ps.find(p => p.id === hid); let nd = 1e9; for (const p of once.ps) if (p.team !== team && p.role !== 'Bekçi') nd = Math.min(nd, Math.hypot(p.x - P0.x, p.y - P0.y));
    const c0 = klon(once); const z = zorlaAdim(c0, hid, o, true); if (!z.atildi || !c0.ball) continue;
    let gg = 0, gn = 0; for (let r = 0; r < RN; r++) { const c = klon(once, (r * 977 + 31337) ^ (sc * 7919)); const zz = zorlaAdim(c, hid, o, false); if (!zz.atildi) continue; const s = sonuc(c, team); if (s != null) { gg += s; gn++; } }
    const to = o.lead || o.to || o.q, ileri = team === 0 ? P0.x : 100 - P0.x, hedefIleri = team === 0 ? to.x : 100 - to.x, ev = M.evreBul(once, team);
    console.log(JSON.stringify({ sc, tur: o.kendine ? 'kendine' : o.kind, secim: +o.det.P.toFixed(3), model: z.model == null ? null : +z.model.toFixed(3), gercek: gn ? +(gg / gn).toFixed(3) : null, gn, uz: +Math.hypot(to.x - P0.x, to.y - P0.y).toFixed(1), rak: +nd.toFixed(1), ileri: +ileri.toFixed(1), hileri: +hedefIleri.toFixed(1), evre: ev, t: m.tick })); } }
