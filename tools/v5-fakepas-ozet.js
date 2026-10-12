// v5-fakepas.js özeti. Kullanım: node tools/v5-fakepas-ozet.js lab/fakepas/*.jsonl
const fs = require('fs'); const R = process.argv.slice(2).flatMap(f => fs.readFileSync(f, 'utf8').split('\n').filter(Boolean).map(l => JSON.parse(l)));
const PAS = { pas: 1, 'önüne': 1, kenardan: 1 }, pct = x => x == null || isNaN(x) ? '–' : (x * 100).toFixed(0) + '%', mean = a => a.length ? a.reduce((s, x) => s + x, 0) / a.length : NaN;
const se = a => { const m = mean(a); return Math.sqrt(a.reduce((s, x) => s + (x - m) ** 2, 0) / Math.max(1, a.length - 1) / a.length); };
const plk = p => p && PAS[p.k.split('·')[0]] && !p.k.includes('kendine');
function rapor(ad, rs) { if (!rs.length) return; console.log(`\n## ${ad} (${rs.length} an, ${rs.reduce((s, r) => s + r.FK.length, 0)} fake denemesi)`);
  // 1 · fake'ten sonra ne oldu
  const ne = {}; for (const r of rs) for (const f of r.FK) { const plan = f.ald ? r.plan1 : r.plan0; const k = f.ne === 'kayıp' ? 'pastan önce kayıp' : f.ne === 'yok' ? 'pas yok (sürdü/tuttu)' : f.ne === 'sayı' ? 'sayı' : plan && f.ne === plan.k && f.q === plan.q ? 'planlanan pas' : PAS[f.ne] ? 'başka pas' : f.ne; ne[k] = (ne[k] || 0) + 1; }
  const top = Object.values(ne).reduce((s, x) => s + x, 0); console.log('Fake\'ten sonra: ' + Object.entries(ne).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${pct(v / top)}`).join(', '));
  // 2 · planlanan pas atıldığında: aklın P'si ile gerçek (dünyaya göre)
  console.log('\n| | akıl (P) | gerçek | deneme |\n|---|---|---|---|');
  for (const [w, ad2] of [[1, 'fake → planlanan pas · rakip aldandı'], [0, 'fake → planlanan pas · kimse aldanmadı']]) { const P = [], G = []; for (const r of rs) { const plan = w ? r.plan1 : r.plan0; if (!plk(plan)) continue; for (const f of r.FK) if (f.ald === w && f.ne === plan.k && f.q === plan.q && f.tut != null) { P.push(plan.P); G.push(f.tut); } } console.log(`| ${ad2} | ${pct(mean(P))} | ${pct(mean(G))} | ${G.length} |`); }
  // 3 · aynı pas fake'siz hemen
  { const P = [], G = []; for (const r of rs) r.hemen.forEach((h, w) => { if (!h) return; const g = r.HM[w].filter(x => x != null); if (!g.length) return; P.push(h.P); G.push(mean(g)); }); console.log(`| aynı pas fake'siz, hemen | ${pct(mean(P))} | ${pct(mean(G))} | ${G.length} an |`); }
  // 4 · aynı anlarda: fake ile bu alıcıya pasın tutması (pastan önce kayıp = tutmadı) ile hemen atılması
  { const d = []; for (const r of rs) { const h = r.hemen[0] || r.hemen[1], w = r.hemen[0] ? 0 : 1; if (!h) continue; const g = r.HM[w].filter(x => x != null); if (!g.length) continue; const fk = r.FK.map(f => f.ne === 'kayıp' ? 0 : f.q === h.q && PAS[f.ne] ? f.tut : null).filter(x => x != null); if (!fk.length) continue; d.push(mean(fk) - mean(g)); } console.log(`\nAynı alıcıya pasın gerçek tutması, fake ile − fake'siz: ${(mean(d) * 100).toFixed(0)} ± ${(se(d) * 100).toFixed(0)} puan (${d.length} an; fake'li tarafta pastan önce kayıp "tutmadı" sayılır)`); }
  console.log(`Aldanma: akıl ${pct(mean(rs.map(r => r.pf)))} → gerçek ${pct(mean(rs.flatMap(r => r.FK.map(f => f.ald))))}`); }
rapor('Hepsi', R); rapor('Rakip yakın (< 4)', R.filter(r => r.rak < 4)); rapor('Rakip uzak (≥ 4)', R.filter(r => r.rak >= 4));
rapor('Plan "önüne"', R.filter(r => r.plan1 && r.plan1.k === 'önüne')); rapor('Plan "kenardan"', R.filter(r => r.plan1 && r.plan1.k === 'kenardan')); rapor('Plan "pas"', R.filter(r => r.plan1 && r.plan1.k === 'pas'));
console.log('\nPlanlar (aldandı dünyası): ' + Object.entries(R.reduce((o, r) => { const k = r.plan1 ? r.plan1.k : '-'; o[k] = (o[k] || 0) + 1; return o; }, {})).map(([k, v]) => k + ' ' + v).join(', '));
