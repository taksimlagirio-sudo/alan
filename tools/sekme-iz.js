// Gönderme sonrası Çekirdek gönderen takıma döndüyse: Bekçi'ye değdi mi (tem kaydı), kaç tik sonra, geri alan nerede (kaleye uzaklık), gönderenin vuruştan sonra koştuğu yol.
const { oku } = require('./kayit-oku.js'); const R = { bekDegdi: 0, degmedi: 0, n: 0, tik: [], mes: [], gonderenKos: [], tutuldu: 0, deflKimde: {} };
for (const f of process.argv.slice(2)) { const k = oku(f), N = k.N, W = k.saha.W, H = k.saha.H;
  for (const p of k.pas.filter(p => p.tur === 'gönder' && p.kim != null)) { const T = k.oyuncular[p.kim].takim, gx = T === 0 ? W : 0; const s = k.sahip.find(s => s.t > p.t && s.kim != null); if (!s || s.t > p.t + 180) continue; const sayi = k.olay.find(o => o.tur === 'sayi' && o.t > p.t && o.t <= s.t); if (sayi) continue;
    const tem = k.tem.find(e => e.t >= p.t && e.t <= s.t && e.gon); const biz = k.oyuncular[s.kim].takim === T;
    if (tem) { const key = (biz ? (s.kim === p.kim ? 'gönderen' : 'arkadaşı') : (k.oyuncular[s.kim].rol === 'Bekçi' ? 'Bekçi' : 'rakip')); R.deflKimde[key] = (R.deflKimde[key] || 0) + 1; }
    if (!biz) continue; R.n++; if (tem) R.bekDegdi++; else R.degmedi++; R.tik.push(s.t - p.t); R.mes.push(Math.hypot(gx - k.P.x[s.t * N + s.kim] / 100, H / 2 - k.P.y[s.t * N + s.kim] / 100)); } }
const md = a => { a.sort((x, y) => x - y); return a[a.length >> 1]; };
console.log(`top gönderen takıma döndü: ${R.n} · Bekçi'ye değmişti ${R.bekDegdi} · değmemişti (gövde, savunmacı, direk) ${R.degmedi}`);
console.log(`dönüş: medyan ${md(R.tik)} tik sonra, geri alan kaleye ${md(R.mes).toFixed(1)} birimde`);
console.log('Bekçi\'ye değen göndermelerde top kimde:', JSON.stringify(R.deflKimde));
