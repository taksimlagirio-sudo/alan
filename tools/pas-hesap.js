// Tutma tablosu (Bölüm 10) · hesap. pas-veri.js maçlarından P(pas tutar | race, recvOpp, len, arrV) tablosu çıkarır.
// Tutma = ilk sahip bizden biri (alıcı ya da arkadaş). Doğrulama: çift maçlarla kurulur, tek maçlarda sınanır.
// Kullanım: node tools/pas-hesap.js <maç klasörü> [çıktı tablo.js] [önceki tablo.js (sönüm %50; az örnekli hücrede yeni veri orantılı daha az)]   (FIZ ortam değişkeni: damga)
const fs = require('fs'), path = require('path');
const [dirIn, out, prevPath] = process.argv.slice(2), MINN = 40, RK = process.env.RACE || 'race'; // RACE=race2: alıcının topla buluştuğu noktaya kadar olan yarış payı
const games = fs.readdirSync(dirIn).filter(f => f.endsWith('.json')).map(f => { try { return JSON.parse(fs.readFileSync(path.join(dirIn, f), 'utf8')); } catch (e) { return null; } }).filter(Boolean);
const all = []; for (const g of games) for (const p of g.p) { if (p.err || p[RK] == null || p.T <= 2 || p[RK] >= 1e8 || p.first == null || p.first === 'yok' || p.first === 'sayı') continue; p.y = p.first === 'biz' ? 1 : 0; p.gi = g.i; all.push(p); }
// dilimler: ham ölçü → hücre; tablo okunurken aynı sınırlar kullanılır
const E = { [RK]: [-8, -3, 0, 3, 8, 16], recvOpp: [2, 4, 7], len: [12, 22], arrV: [.6] };
const DIMS = [RK, 'recvOpp', 'len', 'arrV'], NB = DIMS.map(k => E[k].length + 1), N = NB.reduce((a, b) => a * b, 1);
const bin = (k, x) => { const e = E[k]; let i = 0; while (i < e.length && x >= e[i]) i++; return i; };
const cellOf = p => DIMS.reduce((c, k, j) => c * NB[j] + bin(k, Math.min(p[k], 1e6)), 0);
const dec = c => { const o = {}; for (let j = DIMS.length - 1; j >= 0; j--) { o[DIMS[j]] = c % NB[j]; c = Math.floor(c / NB[j]); } return o; };
const enc = o => DIMS.reduce((c, k, j) => c * NB[j] + o[k], 0);
function fit(P) { const s = new Float64Array(N), n = new Float64Array(N); for (const p of P) { const c = cellOf(p); s[c] += p.y; n[c]++; }
  // az örnekli hücre: race×recvOpp marjinaline doğru çekilir (race en güçlü ölçü)
  const ms = {}, mn = {}; for (let c = 0; c < N; c++) { const o = dec(c), k = o[RK] + ',' + o.recvOpp; ms[k] = (ms[k] || 0) + s[c]; mn[k] = (mn[k] || 0) + n[c]; }
  const rs = {}, rn = {}; for (let c = 0; c < N; c++) { const o = dec(c); rs[o[RK]] = (rs[o[RK]] || 0) + s[c]; rn[o[RK]] = (rn[o[RK]] || 0) + n[c]; }
  const tot = P.reduce((a, p) => a + p.y, 0) / P.length, v = new Float64Array(N);
  for (let c = 0; c < N; c++) { const o = dec(c), k = o[RK] + ',' + o.recvOpp, pr0 = rn[o[RK]] ? (rs[o[RK]] + tot * 5) / (rn[o[RK]] + 5) : tot, pr = mn[k] ? (ms[k] + pr0 * 20) / (mn[k] + 20) : pr0; v[c] = (s[c] + pr * MINN) / (n[c] + MINN); }
  return { v, n }; }

// Alıcının Tutuş'u (Bölüm 10): hücre değerinin logit'ine b × (Tutuş − c) eklenir. Veride alıcı özelliği (aQ, KEYS sırasıyla; Tutuş = 2. indis) varsa kestirilir.
const lg = x => Math.log(Math.max(1e-4, Math.min(1 - 1e-4, x)) / (1 - Math.max(1e-4, Math.min(1 - 1e-4, x)))), sg = x => 1 / (1 + Math.exp(-x));
function fitTut(P, V) { const Q = P.filter(p => p.aQ); if (Q.length < 500) return null; const c = Q.reduce((a, p) => a + p.aQ[2], 0) / Q.length; let b = 0;
  for (let it = 0; it < 30; it++) { let g = 0, h = 0; for (const p of Q) { const x = p.aQ[2] - c, q = sg(lg(V[cellOf(p)]) + b * x); g += (p.y - q) * x; h += q * (1 - q) * x * x; } const st = g / (h || 1); b += st; if (Math.abs(st) < 1e-6) break; }
  return { b: +b.toFixed(4), c: +c.toFixed(2) }; }
const pRead = (V, tu, p) => tu && p.aQ ? sg(lg(V[cellOf(p)]) + tu.b * (p.aQ[2] - tu.c)) : V[cellOf(p)];
const pct = x => (100 * x).toFixed(0) + '%';
console.log(`maç ${games.length} · pas ${all.length} · tutma ${pct(all.reduce((a, p) => a + p.y, 0) / all.length)} · keşif pası ${all.filter(p => p.ex).length}`);
// 1) modelin P'si ne kadar şişik
const calib = (P, f, lab) => { const b = Array.from({ length: 10 }, () => [0, 0, 0]); for (const p of P) { const x = f(p); if (x == null) continue; const k = Math.min(9, Math.max(0, Math.floor(x * 10))); b[k][0] += x; b[k][1] += p.y; b[k][2]++; }
  console.log(lab); for (const [s, o, n] of b) if (n >= 20) console.log(`  tahmin ${(s / n).toFixed(2)} → gerçek ${(o / n).toFixed(2)} (${n})`); };
calib(all.filter(p => !p.ex), p => p.P, 'kafadaki P (seçilen paslar):');
// 2) tek tek ölçüler
const uni = (k, e) => { const b = e.map(() => [0, 0]).concat([[0, 0]]); for (const p of all) { let i = 0; while (i < e.length && p[k] >= e[i]) i++; b[i][0] += p.y; b[i][1]++; } return b.map(([y, n], i) => `${i === 0 ? '<' + e[0] : i === e.length ? '≥' + e[e.length - 1] : e[i - 1] + '…' + e[i]}: ${n ? pct(y / n) : '–'} (${n})`).join(' · '); };
console.log('race (tik):', uni('race', [-15, -8, -3, 0, 3, 8, 16, 30]));
if (all[0] && all[0].race2 != null) console.log('race2 (alıcıya kadar):', uni('race2', [-15, -8, -3, 0, 3, 8, 16, 30]));
console.log('recvOpp:', uni('recvOpp', [1, 2, 4, 7, 12]));
console.log('len:', uni('len', [8, 12, 18, 22, 30, 40]));
console.log('arrV:', uni('arrV', [.4, .6, .8, 1]));
console.log('laneD:', uni('laneD', [1, 2.5, 5, 8, 15]));
console.log('race ≥ 8 iken laneD (race = tabloda kullanılan yarış ölçüsü):', (() => { const s = all.filter(p => p[RK] >= 8); const b = [[0, 0], [0, 0]]; for (const p of s) { const i = p.laneD < 2.5 ? 0 : 1; b[i][0] += p.y; b[i][1]++; } return `<2.5: ${pct(b[0][0] / b[0][1])} (${b[0][1]}) · ≥2.5: ${pct(b[1][0] / b[1][1])} (${b[1][1]})`; })());
const kinds = {}; for (const p of all) { kinds[p.kind] = kinds[p.kind] || [0, 0]; kinds[p.kind][0] += p.y; kinds[p.kind][1]++; } console.log('tür:', Object.entries(kinds).map(([k, [y, n]]) => `${k} ${pct(y / n)} (${n})`).join(' · '));
// 3) doğrulama
const tr = all.filter(p => p.gi % 2 === 0), te = all.filter(p => p.gi % 2 === 1), fT = fit(tr);
const brier = (P, f) => P.reduce((a, p) => a + (f(p) - p.y) ** 2, 0) / P.length;
const tuT = fitTut(tr, fT.v); if (tuT) console.log(`alıcı Tutuş katsayısı (çift maçlar): b ${tuT.b} logit/puan, merkez ${tuT.c}`);
calib(te, p => fT.v[cellOf(p)], 'tablo (çift maçla kurulan, tek maçlarda):');
if (tuT) { calib(te, p => pRead(fT.v, tuT, p), 'tablo + Tutuş:'); const tb = [[0, 9], [10, 12], [13, 99]].map(([lo, hi]) => { const s = te.filter(p => p.aQ && p.aQ[2] >= lo && p.aQ[2] <= hi), f = g => (s.reduce((a, p) => a + p.y - g(p), 0) / s.length).toFixed(3); return `Tutuş ${lo}–${hi === 99 ? 16 : hi}: tablo ${f(p => fT.v[cellOf(p)])} → +Tutuş ${f(p => pRead(fT.v, tuT, p))} (${s.length})`; }); console.log('gerçek − tahmin:', tb.join(' · ')); console.log(`Brier tablo + Tutuş ${brier(te, p => pRead(fT.v, tuT, p)).toFixed(4)}`); }
console.log(`Brier (düşük iyi): tablo ${brier(te, p => fT.v[cellOf(p)]).toFixed(4)} · kafadaki P ${brier(te.filter(p => p.P != null), p => p.P).toFixed(4)} · sabit oran ${brier(te, () => tr.reduce((a, p) => a + p.y, 0) / tr.length).toFixed(4)}`);
// 4) tam tablo
const F = fit(all), TU = fitTut(all, F.v);
if (prevPath) { const w = {}; (new Function('window', fs.readFileSync(prevPath, 'utf8')))(w); const P = w.ALAN_PT.v; let d = 0, dn = 0; for (let c = 0; c < N; c++) { const a = .5 * Math.min(1, F.n[c] / MINN); d += Math.abs(F.v[c] - P[c]) * F.n[c]; dn += F.n[c]; F.v[c] = a * F.v[c] + (1 - a) * P[c]; } console.log(`önceki tabloya göre ham fark (örnek ağırlıklı ort.): ${(d / dn).toFixed(3)} · sönüm %50`); }
const filled = [...F.n].filter(x => x >= MINN).length;
console.log(`hücre ${N} · ${MINN}+ örnekli ${filled} · boş ${[...F.n].filter(x => !x).length}`);
if (out) { const tab = { dims: DIMS, edges: E, v: [...F.v].map(x => +x.toFixed(4)), n: [...F.n], mac: games.length, pas: all.length, tutus: TU, tur: prevPath ? ((() => { const w = {}; (new Function('window', fs.readFileSync(prevPath, 'utf8')))(w); return (w.ALAN_PT.tur || 0) + 1; })()) : 0, fiz: process.env.FIZ || '', not: 'P(pas tutar: ilk sahip bizden) · AlanPass.feat ölçüleriyle, kusursuz algı. Hücre = DIMS sırasıyla, her ölçü edges sınırlarıyla dilimlenir (x ≥ sınır → üst dilim). tutus varsa: P = sigmoid(logit(hücre) + tutus.b × (alıcı Tutuş − tutus.c)). Bkz. tools/pas-veri.js, tools/pas-hesap.js' };
  fs.writeFileSync(out, '// Tutma tablosu (Bölüm 10) · Claude Code tarafından üretildi. Elle düzenlemeyin; tools/pas-hesap.js ile yeniden üretilir.\nwindow.ALAN_PT = ' + JSON.stringify(tab) + ';\n'); console.log('yazıldı:', out); }
