// Gönderme sonrası Çekirdek kime geçti (sayı değilse): gönderenin takımı / Bekçi tuttu / başka rakip. Aklın "bizde kalır" tahmini (P) ile.
const { oku } = require('./kayit-oku.js'); const S = {}; let n = 0, Pt = 0, biz = 0; const mesS = {};
for (const f of process.argv.slice(2)) { const k = oku(f);
  for (const p of k.pas.filter(p => p.tur === 'gönder' && p.kim != null)) { const T = k.oyuncular[p.kim].takim; const sayi = k.olay.find(o => o.tur === 'sayi' && o.t > p.t && o.t <= p.t + 90); const s = k.sahip.find(s => s.t > p.t && s.kim != null);
    let c; if (sayi && (!s || sayi.t <= s.t)) c = 'sayı'; else if (!s || s.t > p.t + 180) c = 'yok'; else { const q = k.oyuncular[s.kim]; c = q.takim === T ? (s.kim === p.kim ? 'gönderen yeniden aldı' : 'arkadaşı aldı') : q.rol === 'Bekçi' ? 'Bekçi tuttu' : 'başka rakip aldı'; }
    S[c] = (S[c] || 0) + 1; n++; if (c !== 'sayı') { Pt += p.P ?? 0; if (/aldı/.test(c) && !/rakip/.test(c)) biz++; }
    const W = k.saha.W, gx = T === 0 ? W : 0, mes = Math.hypot(gx - k.T.bx[p.t] / 100, k.saha.H / 2 - k.T.by[p.t] / 100), b = mes < 15 ? '0–15' : mes < 25 ? '15–25' : '25+'; const m = mesS[b] = mesS[b] || { n: 0, biz: 0, P: 0, Pk: 0 }; if (c !== 'sayı') { m.n++; m.P += p.P ?? 0; if (/aldı/.test(c) && !/rakip/.test(c)) m.biz++; } } }
console.log('gönderme', n); for (const [c, v] of Object.entries(S).sort((a, b) => b[1] - a[1])) console.log(`  ${c}: ${v} (%${(100 * v / n).toFixed(0)})`);
console.log(`sayı olmayanlarda Çekirdek bize döndü: gerçek %${(100 * biz / (n - (S['sayı'] || 0))).toFixed(0)} · aklın tahmini (P) %${(100 * Pt / (n - (S['sayı'] || 0))).toFixed(0)}`);
for (const [b, m] of Object.entries(mesS)) console.log(`  ${b}: n ${m.n} · bize döndü %${(100 * m.biz / m.n).toFixed(0)} · akıl %${(100 * m.P / m.n).toFixed(0)}`);
