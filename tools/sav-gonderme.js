// Göndermelerde savunma: vuruş anında savunmacıların yeri (kaleye uzaklık, hücumcuyu kale tarafından tutuyor mu, görevi),
// vuruştan sonraki 30 tikte hareketi (hız, topa / kaleye doğru), top yeniden alınınca en yakınlar.
// Kullanım: node tools/sav-gonderme.js <kayıt...>   (ayrıntı için AYRINTI=1)
const { oku } = require('./kayit-oku.js'); const A = { n: 0, savKut: 0, hucKut: 0, tutulanHuc: 0, bosHuc: 0, savHiz: 0, hucHiz: 0, hzN: 0, savKaleye: 0, savTopa: 0, hucTopa: 0, gor: {}, gor10: {}, alisSav: 0, alisHuc: 0, alisN: 0, alisSavYakin: 0, kazanan: { sav: 0, huc: 0 } };
const GOR = g => (g || '').replace(/ · tepki/g, '').replace(/ · .*/, '') || '—';
for (const f of process.argv.slice(2)) { const k = oku(f), N = k.N, W = k.saha.W, H = k.saha.H, X = (t, i) => k.P.x[t * N + i] / 100, Y = (t, i) => k.P.y[t * N + i] / 100;
  for (const p of k.pas.filter(p => p.tur === 'gönder' && p.kim != null)) { const T = k.oyuncular[p.kim].takim, D = 1 - T, gk = D === 0 ? 0 : W, t = p.t; if (t + 30 >= k.n) continue; A.n++;
    const sav = [], huc = []; for (let i = 0; i < N; i++) { const o = k.oyuncular[i]; if (o.rol === 'Bekçi') continue; (o.takim === D ? sav : huc).push(i); }
    const kale = (tt, i) => Math.hypot(X(tt, i) - gk, Y(tt, i) - H / 2), kutu = i => kale(t, i) < 22;
    const sK = sav.filter(kutu), hK = huc.filter(kutu); A.savKut += sK.length; A.hucKut += hK.length;
    // kutudaki her hücumcu: bir savunmacı onun kale tarafında, 3 birim içinde mi
    for (const h of hK) { const tut = sav.some(s => Math.hypot(X(t, s) - X(t, h), Y(t, s) - Y(t, h)) < 3.5 && kale(t, s) < kale(t, h)); if (tut) A.tutulanHuc++; else A.bosHuc++; }
    for (const s of sK) { const g = GOR(k.gorevler[k.P.job[t * N + s]]), g10 = GOR(k.gorevler[k.P.job[(t + 12) * N + s]]); A.gor[g] = (A.gor[g] || 0) + 1; A.gor10[g10] = (A.gor10[g10] || 0) + 1; }
    // hareket: 30 tikte yol / tik, kaleye doğru (+) mı
    for (const s of sK) { let yol = 0; for (let u = t + 1; u <= t + 30; u++) yol += Math.hypot(X(u, s) - X(u - 1, s), Y(u, s) - Y(u - 1, s)); A.savHiz += yol / 30; A.savKaleye += kale(t, s) - kale(t + 30, s); A.hzN++; }
    for (const h of hK) { let yol = 0; for (let u = t + 1; u <= t + 30; u++) yol += Math.hypot(X(u, h) - X(u - 1, h), Y(u, h) - Y(u - 1, h)); A.hucHiz += yol / 30; }
    // yeniden alış (sayı ve Bekçi tutması hariç)
    const s = k.sahip.find(s => s.t > t && s.kim != null); if (!s || s.t > t + 180 || k.olay.find(o => o.tur === 'sayi' && o.t > t && o.t <= s.t) || k.oyuncular[s.kim].rol === 'Bekçi') continue;
    const bx = k.T.bx[s.t] / 100, by = k.T.by[s.t] / 100, en = (L, tt) => Math.min(...L.map(i => Math.hypot(X(tt, i) - bx, Y(tt, i) - by)));
    A.alisN++; A.alisSav += en(sav, s.t); A.alisHuc += en(huc, s.t); const kz = k.oyuncular[s.kim].takim === D ? 'sav' : 'huc'; A.kazanan[kz]++;
    if (process.env.AYRINTI) console.log(`${(t / 60).toFixed(2)} ${k.oyuncular[p.kim].ad}: kutuda sav ${sK.length} / hüc ${hK.length} · ${s.t - t} tik sonra ${k.oyuncular[s.kim].ad} aldı (${kz}) · o an en yakın sav ${en(sav, s.t).toFixed(1)} hüc ${en(huc, s.t).toFixed(1)} · vuruş anında o noktaya sav ${en(sav, t).toFixed(1)} hüc ${en(huc, t).toFixed(1)}`); } }
const r = (a, b) => (a / b).toFixed(2), yz = (a, b) => (100 * a / b).toFixed(0);
console.log(`gönderme ${A.n} · vuruş anında kutuda (kaleye <22) ortalama savunmacı ${r(A.savKut, A.n)} · hücumcu ${r(A.hucKut, A.n)}`);
console.log(`kutudaki hücumcuların kale tarafında 3,5 br içinde savunmacı olan: %${yz(A.tutulanHuc, A.tutulanHuc + A.bosHuc)}`);
console.log(`vuruştan sonraki 0,5 sn: savunmacı hızı ${r(A.savHiz, A.hzN)} br/tik (kaleye doğru net ${r(A.savKaleye, A.hzN)} br) · hücumcu hızı ${r(A.hucHiz, A.hucKut)} br/tik`);
console.log('kutudaki savunmacıların görevi vuruş anında:', Object.entries(A.gor).sort((a, b) => b[1] - a[1]).slice(0, 6).map(([g, v]) => `${g} %${yz(v, A.savKut)}`).join(' · '));
console.log('              12 tik sonra:', Object.entries(A.gor10).sort((a, b) => b[1] - a[1]).slice(0, 6).map(([g, v]) => `${g} %${yz(v, A.savKut)}`).join(' · '));
console.log(`seken top (${A.alisN}): savunma aldı %${yz(A.kazanan.sav, A.alisN)} · alış anında noktaya en yakın savunmacı ort. ${r(A.alisSav, A.alisN)} br, hücumcu ${r(A.alisHuc, A.alisN)} br`);
