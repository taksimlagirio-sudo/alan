// oz-olc.js sonuçlarının özeti. Her özellik ve değer için A − rakip farkları (ortalama ± standart hata), G-B-M, mekanik göstergeler.
// Kullanım: node tools/oz-ozet2.js <sonuç klasörü> [md]
const fs = require('fs'), path = require('path'); const dir = process.argv[2];
const AD = { taban: 'Taban (herkes 10)', hiz: 'Hız', ceviklik: 'Çeviklik', guc: 'Güç', uzanma: 'Uzanma', alan: 'Alan gücü', kontrol: 'Kontrol', surme: 'Sürme', pas: 'Pas', gonderme: 'Gönderme', yukleme: 'Yükleme', tepki: 'Tepki', gorus: 'Görüş', yeralma: 'Yer alma', sogukkanlilik: 'Soğukkanlılık', kesme: 'Kesme', refleks: 'Refleks (B)', yerlesim: 'Yerleşim (B)', hakimiyet: 'Alan hakimiyeti (B)' };
const R = {}; for (const f of fs.readdirSync(dir)) { if (!f.endsWith('.json')) continue; let o; try { o = JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8').trim().split('\n').pop()); } catch (e) { continue; } if (!o.gol) continue; (R[o.oz + ' ' + o.v] = R[o.oz + ' ' + o.v] || []).push(o); }
const ms = a => { const n = a.length, m = a.reduce((s, x) => s + x, 0) / n, sd = Math.sqrt(a.reduce((s, x) => s + (x - m) ** 2, 0) / Math.max(1, n - 1)); return [m, sd / Math.sqrt(n)]; };
const f2 = (x, d = 2) => (x >= 0 ? '+' : '−') + Math.abs(x).toFixed(d), pm = ([m, se], d = 2) => `${f2(m, d)} ±${se.toFixed(d)}${Math.abs(m) > 2 * se ? ' *' : ''}`;
const oran = (o, k, ok) => o[ok] && o[k] && o[k][0] ? o[ok][0] / o[k][0] : null;
const satir = (k, L) => { const n = L.length, G = L.map(o => o.gol[0] - o.gol[1]), w = G.filter(x => x > 0).length, d = G.filter(x => x === 0).length;
  const sut = L.map(o => o.sut[0] - o.sut[1]), kap = L.map(o => o.kapma[0] - o.kapma[1]), sah = L.map(o => o.sahip - .5), pas = L.map(o => oran(o, 'pas', 'pasOk')).filter(x => x != null), pasB = L.map(o => o.pas && o.pas[1] ? o.passOk : null),
    pasR = L.map(o => o.pasOk && o.pas[1] ? o.pasOk[1] / o.pas[1] : null).filter(x => x != null), ik = L.map(o => o.ikili && o.ikili[0] + o.ikili[1] ? (o.ikiliK[0] + (o.ikili[1] - o.ikiliK[1])) / (o.ikili[0] + o.ikili[1]) - .5 : null).filter(x => x != null), hz = L.map(o => o.hiz[0] - o.hiz[1]);
  const [oz, v] = k.split(' '); return { oz, v: +v, n, gf: ms(G), gbm: `${w}-${d}-${n - w - d}`, sut: ms(sut), kap: ms(kap), sah: ms(sah), pasA: ms(pas)[0], pasR: ms(pasR)[0], ik: ik.length ? ms(ik) : [0, 0], hz: ms(hz), golA: ms(L.map(o => o.gol[0]))[0], golB: ms(L.map(o => o.gol[1]))[0] }; };
const S = Object.entries(R).map(([k, L]) => satir(k, L)), sira = Object.keys(AD);
S.sort((a, b) => sira.indexOf(a.oz) - sira.indexOf(b.oz) || b.v - a.v);
console.log('| Özellik | değer | maç | gol farkı | G-B-M | şut farkı | kapma farkı | top bizde (−50%) | pas isabeti A / rakip | ikili payı (−50%) | hız farkı |');
console.log('|---|---|---|---|---|---|---|---|---|---|---|');
for (const s of S) console.log(`| ${AD[s.oz] || s.oz} | ${s.v} | ${s.n} | ${pm(s.gf)} | ${s.gbm} | ${pm(s.sut, 1)} | ${pm(s.kap, 1)} | ${pm([s.sah[0] * 100, s.sah[1] * 100], 1)} | ${(s.pasA * 100).toFixed(0)} / ${(s.pasR * 100).toFixed(0)} | ${pm([s.ik[0] * 100, s.ik[1] * 100], 0)} | ${pm([s.hz[0] * 1000, s.hz[1] * 1000], 1)} |`);
// puan başına etki: (16 − 4) / 12
console.log('\n| Özellik | gol farkı / puan | şut farkı / puan | 4 → 10 → 16 tek yönlü mü |'); console.log('|---|---|---|---|');
const T = S.find(s => s.oz === 'taban'); const egim = [];
for (const oz of sira) { const a = S.find(s => s.oz === oz && s.v === 16), b = S.find(s => s.oz === oz && s.v === 4); if (!a || !b) continue; const e = (a.gf[0] - b.gf[0]) / 12, se = Math.hypot(a.gf[1], b.gf[1]) / 12, es = (a.sut[0] - b.sut[0]) / 12, ses = Math.hypot(a.sut[1], b.sut[1]) / 12;
  const t0 = T ? T.gf[0] : 0, yon = (t0 - b.gf[0]) * (a.gf[0] - t0) >= 0 ? 'evet' : 'hayır'; egim.push([oz, e]); console.log(`| ${AD[oz]} | ${pm([e, se], 3)} | ${pm([es, ses], 2)} | ${yon} |`); }
console.log('\n`*`: fark standart hatanın iki katından büyük. Farklar A − rakip; A, tek özelliği değiştirilen takım.');
