// Bir takımın göndermeleri tek tek: kim, nereden (mesafe, açı), aklın girme tahmini, o karardaki diğer seçenekler, öncesi (sekme mi, pas mı), sonuç.
// Kullanım: node tools/gonderme-iz.js <kayıt.json.gz> <takım>
const { oku } = require('./kayit-oku.js'); const [f, tS] = process.argv.slice(2), T = +tS, k = oku(f), N = k.N, W = k.saha.W, H = k.saha.H;
const AD = i => i == null ? '—' : k.oyuncular[i].ad, gx = T === 0 ? W : 0;
for (const p of k.pas.filter(p => p.tur === 'gönder' && p.kim != null && k.oyuncular[p.kim].takim === T)) { const t = p.t, x = k.T.bx[t] / 100, y = k.T.by[t] / 100, mes = Math.hypot(gx - x, H / 2 - y), aci = Math.atan2(Math.abs(H / 2 - y), Math.abs(gx - x)) * 180 / Math.PI;
  const al = k.sahip.filter(s => s.t <= t && s.kim === p.kim).pop(), once = al ? k.sahip.filter(s => s.t < al.t && s.kim != null).pop() : null, tutus = al ? ((t - al.t) / 60).toFixed(2) : '?';
  const onceT = once ? (k.oyuncular[once.kim].takim === T ? (once.kim === p.kim ? 'kendi' : 'pas: ' + AD(once.kim)) : 'rakipten: ' + AD(once.kim)) : '—';
  const prev = k.olay.filter(o => o.t < t && o.t > t - 90 && o.tur === 'gonder').length;
  const kr = k.karar.filter(c => c.kim === p.kim && c.t <= t && c.t >= t - 30).pop(); const sec = kr ? kr.s.map(s => `${s[0]}${s[1] ? '→' + s[1] : ''} ${s[3].toFixed(3)}`).join(' · ') : (p.ot ? 'tek dokunuş' : '—');
  const son = k.olay.find(o => o.t > t && o.t <= t + 120 && ['sayi', 'kesme', 'gonder', 'direk'].includes(o.tur)); let yakin = 1e9; for (let i = 0; i < N; i++) if (k.oyuncular[i].takim !== T && k.oyuncular[i].rol !== 'Bekçi') yakin = Math.min(yakin, Math.hypot(k.P.x[t * N + i] / 100 - x, k.P.y[t * N + i] / 100 - y));
  console.log(`${(t / 60).toFixed(2)} ${AD(p.kim).padEnd(6)} mes ${mes.toFixed(1)} açı ${aci.toFixed(0)}° · akıl %${(100 * (p.Pk || 0)).toFixed(0)} · en yakın rakip ${yakin.toFixed(1)} · topu ${tutus} sn tuttu, öncesi ${onceT}${prev ? ` · 1,5 sn içinde ${prev}. gönderme` : ''}${p.ot ? ' · tek dokunuş' : ''}\n      seçenekler: ${sec}\n      sonra: ${son ? son.metin : '—'}`); }
