// Karar kalitesi ölçüsü: kaydı bir motorla oynatır (yükleme sırası motorun işçisinden; eski motorlarda sabit liste), pas, kayıp, şut, kafa ↔ maç sapması ve karar türlerini sayar.
// Kullanım: node zeka.js <motor> <kayit.json> <tohum|-> <tik>
const fs = require('fs'), path = require('path'); const [dir, kf, tS, lS] = process.argv.slice(2); globalThis.window = globalThis; globalThis.self = globalThis;
const mi = fs.readFileSync(path.join(dir, 'mac-isci.js'), 'utf8'), yuk = /importScripts\(([^)]*)\)/.exec(mi)[1].split(',').map(s => s.trim().replace(/['"]/g, ''));
for (const f of yuk) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const d = JSON.parse(fs.readFileSync(kf, 'utf8')).yeni, M = globalThis.AlanMatch, LEN = +lS || d.len; const m = M.createMatch(tS === '-' ? d.tohum : +tS, { tac: d.tac, len: d.len, diz: d.diz, devre: d.devre ?? false });
for (const t of [0, 1]) for (const p of m.ps) { if (p.team !== t) continue; const o = d.oyuncular[t][p.i]; if (o) { p.name = o.ad; p.a = { ...p.a, ...o.a }; } }
M.assignRoles(m, d.roller); for (const p of m.ps) { const r = d.roller[p.team] && d.roller[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } for (const t of [0, 1]) M.dizUygula(m, t, d.diz[t]); M.kafaAc && M.kafaAc(m);
const Z = [0, 1].map(() => ({ pas: 0, pasOk: 0, pasKes: 0, pasBos: 0, kayip: 0, kayip2: 0, sut: 0, sutUz: 0, xg: 0, karar: {}, sap: 0, sapK: 0, sapT: {}, kd: 0, yavas: 0 }));
let fl0 = m.fl, onceki = null, alT = 0, pasA = null, ms = 0; const T0 = Date.now();
try { while (!m.over && m.tick < LEN) { const t1 = Date.now(), h0 = m.holder; M.step(m); const dt = Date.now() - t1; const h = m.holder, b = m.ball;
  if (h && h !== h0) { if (onceki && onceki.team !== h.team) { Z[onceki.team].kayip++; if (m.tick - alT < 60) Z[onceki.team].kayip2++; } alT = m.tick; if (pasA) { const z = Z[pasA.t]; if (h.team === pasA.t) z.pasOk++; else z.pasKes++; pasA = null; } onceki = h; if (dt > 400) Z[h.team].yavas++; }
  if (pasA && b && b.done && !h) { Z[pasA.t].pasBos++; pasA = null; }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; const k = m.fl.kind; if (onceki) { const z = Z[onceki.team]; z.karar[k] = (z.karar[k] || 0) + 1; if (['pas', 'önüne', 'kenardan'].includes(k)) { z.pas++; pasA = { t: onceki.team }; } if (k === 'gönder' && b && b.shotPk != null) { z.sut++; z.xg += b.shotPk; z.sutUz += Math.hypot((onceki.team === 0 ? 120 : 0) - b.x, 30 - b.y); } } } }
  const KL = (m._kafa && m._kafa.L) || []; for (const r of KL) { const z = Z[r.team]; if (!z) continue; z.kd++; if (r.sapma && r.sapma.length) { z.sap++; for (const s of r.sapma) z.sapT[s.k] = (z.sapT[s.k] || 0) + 1; } } }
catch (e) { console.error('HATA', m.tick, String(e).slice(0, 300)); }
const out = Z.map(z => ({ pas: z.pas, isabet: z.pas ? +(z.pasOk / z.pas).toFixed(3) : null, kesilen: z.pasKes, bosta: z.pasBos, kayip: z.kayip, kayip2sn: z.kayip2, sut: z.sut, sutUz: z.sut ? +(z.sutUz / z.sut).toFixed(1) : null, xg: +z.xg.toFixed(2), karar: z.karar, kafaKarar: z.kd, sapma: z.kd ? +(z.sap / z.kd).toFixed(3) : null, sapT: z.sapT }));
console.log(JSON.stringify({ motor: path.basename(dir), tohum: tS, tik: m.tick, skor: m.score, sn: Math.round((Date.now() - T0) / 1000), L: out[0], K: out[1] }));
