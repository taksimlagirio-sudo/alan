// Bekçi değişikliği özeti: kovalama sayısı ve sonucu, sayılarda Bekçi'nin yerinde olma payı, kovalama sonrası sayılar, toplam sayı ve gönderme, hesap süresi
const { oku } = require('./kayit-oku.js'), fs = require('fs'), d = process.argv[2];
let mac = 0, ep = 0, biz = 0, rk = 0, say = 0, gon = 0, hs = 0, yer = 0, kov = 0, ksay = 0, kkac = 0, kac = 0; const pas = { pas: 0, tut: 0 };
for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), N = k.oyuncular.length; mac++; hs += k.hesapSn;
  for (const bi of k.oyuncular.map((p, i) => p.rol === 'Bekçi' ? i : -1).filter(i => i >= 0)) { const tm = k.oyuncular[bi].takim; let t = 0;
    while (t < k.LEN) { if (k.gorevler[k.P.job[t * N + bi]] !== 'kovala') { t++; continue; } const t0 = t; while (t < k.LEN && k.gorevler[k.P.job[t * N + bi]] === 'kovala') t++; if (t - t0 < 6) continue;
      // gönderme kovalaması değil: kovalama başında top rakibin gönderdiği top değilse say
      const g = k.olay.some(o => o.tur === 'gonder' && o.takim !== tm && o.t <= t0 && o.t > t0 - 40); if (g) continue; ep++;
      let u = t0; while (u < k.LEN && k.T.hold[u] === 255) u++; const h = k.T.hold[u]; if (h !== 255 && k.oyuncular[h] && k.oyuncular[h].takim === tm) biz++; else if (h !== 255) rk++; } }
  k.olay.forEach((o, oi) => { if (o.tur !== 'gonder') return; gon++;
    let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
    const t = o.t, bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'), q = t * N + bi;
    const hed = k.P.tx[q] === -32768 ? 0 : Math.hypot(k.P.x[q] / 100 - k.P.tx[q] / 100, k.P.y[q] / 100 - k.P.ty[q] / 100);
    let v = false; for (let u = t - 150; u < t - 1; u++) if (u > 0 && k.gorevler[k.P.job[u * N + bi]] === 'kovala') { v = true; break; }
    if (s) { say++; if (hed <= 3) yer++; if (v) ksay++; } else { kac++; if (v) kkac++; } });
  for (const o of k.olay) if (o.tur === 'pas') pas.pas++;
}
console.log(`${mac} maç · sayı/maç ${(say / mac).toFixed(2)} · gönderme/maç ${(gon / mac).toFixed(1)} · gönderme başına sayı %${(100 * say / gon).toFixed(0)} · hesap ${(hs / mac).toFixed(0)} sn`);
console.log(`Bekçi pas kovalaması/maç ${(ep / mac).toFixed(1)} · top bizde %${(100 * biz / ep).toFixed(0)} · rakipte %${(100 * rk / ep).toFixed(0)}`);
console.log(`sayılarda Bekçi yerinde %${(100 * yer / say).toFixed(0)} (${yer}/${say}) · gönderme öncesi 2,5 sn'de Bekçi kovalamış: sayılarda %${(100 * ksay / say).toFixed(0)}, kaçanlarda %${(100 * kkac / kac).toFixed(0)}`);
