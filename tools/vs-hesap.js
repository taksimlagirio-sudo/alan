// Durum değeri tablosu (Bölüm 9) · hesap. vs-veri.js maçlarından TD(λ) ile V(hücre) = P(sıradaki sayıyı biz) − P(rakip) çıkarır.
// Kullanım: node tools/vs-hesap.js <maç klasörü> <çıktı tablo.js> [λ=0.8] [önceki tablo.js (sönüm %50, 30'dan az örnekli hücrede orantılı daha az)]
const fs = require('fs'), path = require('path'), crypto = require('crypto');
// SON=boot: maç sonunda yarım kalan anlar 0 sayılmaz; getiri son durumun değerinden sürer, kalibrasyon son sayıdan sonraki anları dışarıda bırakır
const [dirIn, out, lS, prevPath] = process.argv.slice(2), LAM = lS != null ? +lS : .8, N = 540, MINN = 30, SON = process.env.SON === 'boot';
const games = fs.readdirSync(dirIn).filter(f => f.endsWith('.json')).map(f => { try { return JSON.parse(fs.readFileSync(path.join(dirIn, f), 'utf8')); } catch (e) { return null; } }).filter(Boolean).sort((a, b) => a.i - b.i);
const ST = { nP: 10, nW: 3, nS: 3, nB: 3, nT: 2 }, dec = c => { const iT = c % 2; c = (c - iT) / 2; const iB = c % 3; c = (c - iB) / 3; const iS = c % 3; c = (c - iS) / 3; const iW = c % 3; const iP = (c - iW) / 3; return { iP, iW, iS, iB, iT }; };
const enc = o => (((o.iP * 3 + o.iW) * 3 + o.iS) * 3 + o.iB) * 2 + o.iT;
// bir maçın örnekleri için λ-getirileri (her örnek kendi takımının bakışından)
function returns(G, V, lam) { const s = G.s, g = G.g, out = new Float64Array(s.length); let gi = 0; const goalsAfter = t => { while (gi < g.length && g[gi][0] <= t) gi++; return gi; };
  // her örnek için (t_k, t_{k+1}] aralığında sayı var mı
  const goalIn = new Array(s.length).fill(null); let j = 0; for (let k = 0; k < s.length; k++) { const t0 = s[k][0], t1 = k + 1 < s.length ? s[k + 1][0] : 1e9; while (j < g.length && g[j][0] <= t0) j++; if (j < g.length && g[j][0] <= t1) goalIn[k] = g[j][1]; }
  for (let k = s.length - 1; k >= 0; k--) { const tm = s[k][1];
    if (goalIn[k] != null) out[k] = goalIn[k] === tm ? 1 : -1;
    else if (k === s.length - 1) out[k] = SON ? V[s[k][2]] : 0; // maç sonu: sıradaki sayı yok (SON=boot: maç sürüyormuş gibi, durumun kendi değeri)
    else { const sg = s[k + 1][1] === tm ? 1 : -1; out[k] = sg * ((1 - lam) * V[s[k + 1][2]] + lam * out[k + 1]); } }
  return out; }
function smooth(sum, cnt) { const v = new Float64Array(N), raw = new Float64Array(N); for (let c = 0; c < N; c++) raw[c] = cnt[c] ? sum[c] / cnt[c] : 0;
  // ilerleme dilimi başına genel ortalama (en kaba yedek)
  const pS = new Float64Array(10), pN = new Float64Array(10); for (let c = 0; c < N; c++) { const { iP } = dec(c); pS[iP] += sum[c]; pN[iP] += cnt[c]; }
  for (let c = 0; c < N; c++) { if (cnt[c] >= MINN) { v[c] = raw[c]; continue; } const o = dec(c); let s = 0, n = 0;
    for (const [k, mx] of [['iP', 9], ['iW', 2], ['iS', 2], ['iB', 2], ['iT', 1]]) for (const d of [-1, 1]) { const q = { ...o, [k]: o[k] + d }; if (q[k] < 0 || q[k] > mx) continue; const cc = enc(q); s += sum[cc]; n += cnt[cc]; }
    const nb = n >= MINN ? s / n : (pN[o.iP] ? pS[o.iP] / pN[o.iP] : 0), w = cnt[c] / MINN; v[c] = w * raw[c] + (1 - w) * nb; }
  return v; }
function fit(G, lam, it = 60) { let V = new Float64Array(N); const cnt = new Float64Array(N); for (const m of G) for (const s of m.s) cnt[s[2]]++;
  for (let r = 0; r < it; r++) { const sum = new Float64Array(N); for (const m of G) { const R = returns(m, V, lam); m.s.forEach((s, k) => { sum[s[2]] += R[k]; }); } const Vn = smooth(sum, cnt); let d = 0; for (let c = 0; c < N; c++) d = Math.max(d, Math.abs(Vn[c] - V[c])); V = Vn; if (d < 1e-5) break; }
  return { V, cnt }; }
// doğrulama: çift maçlarla kur, tek maçlarda gerçek sonuçla (λ=1: sıradaki sayı) karşılaştır
const tr = games.filter(m => m.i % 2 === 0), te = games.filter(m => m.i % 2 === 1);
const fT = fit(tr, LAM), Z = new Float64Array(N); const bins = Array.from({ length: 8 }, () => [0, 0, 0]);
for (const m of te) { const R = returns(m, Z, 1), tEnd = SON ? (m.g.length ? m.g[m.g.length - 1][0] : -1) : 1e9; m.s.forEach((s, k) => { if (s[0] >= tEnd) return; const v = fT.V[s[2]], b = Math.max(0, Math.min(7, Math.floor((v + .4) / .1))); bins[b][0] += v; bins[b][1] += R[k]; bins[b][2]++; }); }
// tam tablo
let { V, cnt } = fit(games, LAM);
let fark = '';
if (prevPath) { const w = {}; (new Function('window', fs.readFileSync(prevPath, 'utf8')))(w); const P = w.ALAN_VS.v; let sd = 0, sn = 0; const dd = [];
  for (let c = 0; c < N; c++) { const d = Math.abs(V[c] - P[c]); if (cnt[c]) { sd += d * cnt[c]; sn += cnt[c]; } if (cnt[c] >= MINN) dd.push(d); const a = .5 * Math.min(1, cnt[c] / MINN); V[c] = a * V[c] + (1 - a) * P[c]; } /* sönüm %50; az örnekli hücrede yeni veri daha az söz sahibi (yeni oyunun gitmediği bölgeler eski bilgisini korur) */
  dd.sort((a, b) => a - b); fark = `önceki tabloya göre ham fark (sönümden önce): örnek ağırlıklı ort. ${(sd / sn).toFixed(3)} · 30+ hücrelerde medyan ${dd[dd.length >> 1].toFixed(3)}, %90 ${dd[Math.floor(.9 * dd.length)].toFixed(3)}`; }
// rapor
const goals = games.reduce((s, m) => s + m.g.length, 0), samp = games.reduce((s, m) => s + m.s.length, 0);
console.log(`maç ${games.length} · örnek ${samp} · sayı ${goals} (${(goals / games.length).toFixed(2)}/maç) · λ ${LAM}${prevPath ? ' · sönüm %50' : ''}`);
if (fark) console.log(fark);
console.log(`hücre: 30+ örnekli ${[...cnt].filter(n => n >= MINN).length}/${N} · hiç örneksiz ${[...cnt].filter(n => n === 0).length}`);
const marg = (key, nb) => { const s = new Float64Array(nb), n = new Float64Array(nb); for (let c = 0; c < N; c++) { const k = dec(c)[key]; s[k] += V[c] * cnt[c]; n[k] += cnt[c]; } return [...s].map((x, k) => n[k] ? (x / n[k]).toFixed(3) : '–'); };
console.log('ilerleme (0–10 … 90–100):', marg('iP', 10).join(' '));
console.log('kanat/orta/kanat:', marg('iW', 3).join(' '), '· yerleşiklik (≤2/3–4/5–6 savunmacı topun gerisinde):', marg('iS', 3).join(' '));
console.log('baskı (<3/3–6/>6 br):', marg('iB', 3).join(' '), '· geçiş (hayır/evet):', marg('iT', 2).join(' '));
console.log('kalibrasyon (çift maçla kurulan tablo, tek maçlarda gerçek sıradaki sayı):'); for (const [s, o, n] of bins) if (n) console.log(`  tahmin ${(s / n).toFixed(2).padStart(6)} → gerçek ${(o / n).toFixed(2).padStart(6)} (${n} örnek)`);
// damga: tabloyu etkileyen parametrelerin özeti
const fizSrc = process.env.FIZ || '';
const tab = { v: [...V].map(x => +x.toFixed(4)), n: [...cnt], tur: games[0] ? games[0].tur : 0, mac: games.length, lambda: LAM, fiz: fizSrc, not: 'V = P(sıradaki sayı biz) − P(rakip); karar/algı kanallarında kusursuz okuma ile oynanmış maçlardan TD(λ). Bkz. tools/vs-veri.js, tools/vs-hesap.js' };
fs.writeFileSync(out, '// Durum değeri tablosu (Bölüm 9) · Claude Code tarafından üretildi. Elle düzenlemeyin; tools/vs-hesap.js ile yeniden üretilir.\nwindow.ALAN_VS = ' + JSON.stringify(tab) + ';\n');
console.log('yazıldı:', out);
