// sut-test.js çıktılarının özeti: uzaklık dilimine göre beynin değeri, hayal (dört koşul) ve gerçek; gerçek maç sonuçları ve Bekçi'nin konumu.
const fs = require('fs'), path = require('path'), d = process.argv[2]; const R = [];
for (const f of fs.readdirSync(d)) { const t = fs.readFileSync(path.join(d, f), 'utf8'); if (!t.trim()) continue; const j = JSON.parse(t); R.push(...j.rows); }
const mean = (a, f) => a.length ? a.reduce((s, x) => s + f(x), 0) / a.length : NaN, f2 = x => isNaN(x) ? '–' : (x >= 0 ? '+' : '') + x.toFixed(2);
const DIL = [[0, 20, '≤20'], [20, 35, '20–35'], [35, 50, '35–50'], [50, 999, '50+']];
console.log(`gönderme ${R.length}`);
console.log('dilim     n   | beynin değeri | hayal: varsayılan  hafif yok  algı yok  Bekçi planı içte | gerçek (20 tekrar) | maçtaki sonuç (sayı oranı)');
for (const [lo, hi, ad] of DIL) { const s = R.filter(r => r.uz >= lo && r.uz < hi); if (!s.length) continue;
  console.log(`${ad.padEnd(8)} ${String(s.length).padStart(3)}   | ${f2(mean(s, r => r.beyin)).padStart(6)}        | ${f2(mean(s, r => r.h_varsayilan)).padStart(6)}            ${f2(mean(s, r => r.h_hafifYok)).padStart(6)}     ${f2(mean(s, r => r.h_algiYok)).padStart(6)}    ${f2(mean(s, r => r.h_bekIc)).padStart(6)}           | ${f2(mean(s, r => r.g.v)).padStart(6)} (sayı %${(100 * mean(s, r => r.g.sayi)).toFixed(0)}, Bekçi %${(100 * mean(s, r => r.g.bek)).toFixed(0)}) | %${(100 * s.filter(r => r.sonuc === 'sayi').length / s.length).toFixed(0)}`); }
console.log('\nBekçi (gönderme anında), uzaklık dilimine göre: Kuyu\'ya uzaklık medyanı · gönderme hattına dik uzaklık medyanı · hattan 1 birimden uzak olanların oranı');
const med = a => { a = a.filter(x => x != null).sort((x, y) => x - y); return a.length ? a[a.length >> 1] : null; };
for (const [lo, hi, ad] of DIL) { const s = R.filter(r => r.uz >= lo && r.uz < hi); if (!s.length) continue; console.log(`${ad.padEnd(8)} Kuyu'ya ${med(s.map(r => r.bekKuyu))} · hatta ${med(s.map(r => r.bekHat))} · hattan uzak %${(100 * s.filter(r => r.bekHat > 1).length / s.length).toFixed(0)}`); }
// algı hatasının etkisi: Bekçi hattın tam üstündeyken (≤0,5) hayal ile gerçek farkı
const on = R.filter(r => r.bekHat <= .5 && r.uz >= 20), off = R.filter(r => r.bekHat > .5 && r.uz >= 20);
console.log(`\n20+ birim: Bekçi hattın üstünde (≤0,5) n=${on.length}: hayal ${f2(mean(on, r => r.h_varsayilan))} · algı yok ${f2(mean(on, r => r.h_algiYok))} · gerçek ${f2(mean(on, r => r.g.v))}`);
console.log(`20+ birim: Bekçi hattan uzak (>0,5)  n=${off.length}: hayal ${f2(mean(off, r => r.h_varsayilan))} · algı yok ${f2(mean(off, r => r.h_algiYok))} · gerçek ${f2(mean(off, r => r.g.v))}`);
const sh = {}; for (const r of R) { sh[r.shot] = sh[r.shot] || []; sh[r.shot].push(r); } console.log('\ngönderme türü:', Object.entries(sh).map(([k, a]) => `${k} ${a.length} (gerçek sayı %${(100 * mean(a, r => r.g.sayi)).toFixed(0)})`).join(' · '));
