// taktik-iz.js özeti: her eşleşmede iki tarafın mekanik izi (maç başına ortalama).
// Kullanım: node tools/taktik-iz-ozet.js <sonuç klasörü>
const fs = require('fs'), path = require('path'); const dir = process.argv[2]; const R = {};
for (const f of fs.readdirSync(dir)) { if (!f.endsWith('.json')) continue; let o; try { o = JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8').trim().split('\n').pop()); } catch (e) { continue; } if (!o.ta) continue; (R[o.a + '|' + o.b] = R[o.a + '|' + o.b] || []).push(o); }
const AD = { T_kale: 'Kale önü', T_kontra: 'Kontra', T_sahip: 'Topa sahip', T_presci: 'Ön alan pres', T_dikine: 'Dikine', T_kanat: 'Kanat tuzağı', varsayilan: 'Varsayılan' };
const ms = a => { const n = a.length, m = a.reduce((s, x) => s + x, 0) / n, sd = Math.sqrt(a.reduce((s, x) => s + (x - m) ** 2, 0) / Math.max(1, n - 1)); return [m, sd / Math.sqrt(n)]; };
const top = (L, k, f) => L.reduce((s, o) => s + f(o)[k], 0), oran = (L, f, a, b) => { const x = top(L, a, f), y = top(L, b, f); return y ? x / y : NaN; };
const sat = (L, f) => { const n = L.length, sh = L.reduce((s, o) => s + f(o).sahip, 0) / L.reduce((s, o) => s + o.ta.sahip + o.tb.sahip, 0);
  return [`${(sh * 100).toFixed(0)}%`, (100 * oran(L, f, 'hatT', 'hatN')).toFixed(0) + '%', (top(L, 'pas', f) / n).toFixed(1), `${(100 * oran(L, f, 'arka', 'pas')).toFixed(0)}% / ${(100 * oran(L, f, 'arkaOk', 'arka')).toFixed(0)}%`,
    (top(L, 'sut', f) / n).toFixed(2), (top(L, 'xg', f) / n).toFixed(3), (oran(L, f, 'xg', 'sut') || 0).toFixed(3), (oran(L, f, 'sutMes', 'sut') || 0).toFixed(1), `${(100 * oran(L, f, 'sutGec', 'sut') || 0).toFixed(0)}%`, `${(100 * oran(L, f, 'sutArka', 'sut') || 0).toFixed(0)}%`,
    (top(L, 'kayip', f) / n).toFixed(1), `${(100 * oran(L, f, 'kayipKendi', 'kayip')).toFixed(0)}%`, (top(L, 'kazanRakip', f) / n).toFixed(1), (top(L, 'gol', f) / n).toFixed(2)]; };
console.log('| eşleşme | taraf | maç | top bizde | rakibin son hattı (bizim yönümüzde) | pas | arkaya pas payı / isabeti | şut | xG | xG / şut | şut uzaklığı | geçiş şutu | arkaya pastan sonra şut | kayıp | kayıp kendi yarımızda | rakip yarısında kazanma | gol |');
console.log('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|');
for (const [k, L] of Object.entries(R)) { const [a, b] = k.split('|'); const xa = ms(L.map(o => o.ta.xg - o.tb.xg));
  console.log(`| **${AD[a]} – ${AD[b]}** (xG farkı ${xa[0] >= 0 ? '+' : '−'}${Math.abs(xa[0]).toFixed(3)} ±${xa[1].toFixed(3)}${Math.abs(xa[0]) > 2 * xa[1] ? ' *' : ''}) | ${AD[a]} | ${L.length} | ${sat(L, o => o.ta).join(' | ')} |`);
  console.log(`| | ${AD[b]} | | ${sat(L, o => o.tb).join(' | ')} |`); }
console.log('\nRakibin son hattı: top bizdeyken rakibin en derindeki saha oyuncusunun bizim kalemizden uzaklığı (saha boyunun %; yüksek = rakip hattı kendi kalesine yakın, derin). Geçiş şutu: top kazanıldıktan sonraki 6 sn içinde. xG: motorun şut anındaki gol ihtimali (shotPk).');
