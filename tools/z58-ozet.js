// zip51–58 ölçümleri tek kayıt klasöründen. Sayılar dakika başına (maç süreleri farklı olabilir).
const { oku, ol } = require('./kayit-oku.js'), fs = require('fs'), d = process.argv[2];
const A = { mac: 0, dk: 0, say: 0, gon: 0, pas: 0, hs: 0, tik: 0, disari: 0, yakinCizgi: 0, devre10: 0, kaz: 0, kazTut: 0, kazSut: 0, basT: 0, basN: 0, tutSec: 0, tutKay: 0, tutN: 0, verN: 0, verIleri: 0, verGeri: 0, presYok: 0, holdT: 0, kapma: 0, dirN: 0, dirUz: 0 };
for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length, L = k.LEN, tm = i => k.oyuncular[i].takim;
  A.mac++; A.dk += L / 3600; A.hs += k.hesapSn; A.tik += L;
  const ileri = (t, x) => t === 0 ? x : W - x;
  // 52: saha dışında serbest Çekirdek
  for (let t = 0; t < L; t++) if (k.T.hold[t] === 255) { const x = k.T.bx[t] / 100, y = k.T.by[t] / 100; if (x < .49 || x > W - .49 || y < .49 || y > H - .49) { if (!(Math.abs(y - H / 2) <= 5.5)) A.disari++; } }
  k.olay.forEach((o, oi) => {
    if (o.tur === 'pas') A.pas++;
    if (o.tur === 'gonder') { A.gon++;
      let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
      const bx = k.T.bx[o.t] / 100, by = k.T.by[o.t] / 100, gx = o.takim === 0 ? W : 0, a = Math.atan2(Math.abs(by - H / 2), Math.abs(gx - bx)) * 180 / Math.PI;
      if (s && Math.abs(gx - bx) < 1) A.yakinCizgi++;
      if (a >= 60) { const bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'), q = o.t * N + bi, py = by < H / 2 ? H / 2 - 5 : H / 2 + 5, px = gx === 0 ? .5 : W - .5; A.dirN++; A.dirUz += Math.hypot(k.P.x[q] / 100 - px, k.P.y[q] / 100 - py); }
    }
    if (o.tur === 'sayi') { A.say++; if (o.t >= L / 2 && o.t < L / 2 + 600) A.devre10++; }
  });
  // sahiplik dizisi: (takım, oyuncu) değişimleri
  let sonT = -1, sonH = 255;
  for (let t = 1; t < L; t++) { const h = k.T.hold[t]; if (h === 255) continue; const ht = tm(h);
    if (sonT >= 0 && ht !== sonT) { // 54/57: top kazanıldı: ilk 3 sn
      A.kaz++; let tut = 0; for (let u = t; u < t + 180 && u < L; u++) { if (k.T.hold[u] === h) tut++; else if (k.T.hold[u] !== 255) break; }
      A.kazTut += tut; if (k.olay.some(o => o.tur === 'gonder' && o.takim === ht && o.t >= t && o.t < t + 180)) A.kazSut++; }
    // 56: pas veren: tutan h'den sonra top serbest ve sonra arkadaşa geçerse pas; pasçının 1 sn ileri hareketi
    if (h !== sonH && sonH !== 255 && sonT === ht) { let t0 = t - 1; while (t0 > 0 && k.T.hold[t0] === 255) t0--; const p0 = t0 * N + sonH, p1 = Math.min(L - 1, t0 + 60) * N + sonH; const dx = ileri(ht, k.P.x[p1] / 100) - ileri(ht, k.P.x[p0] / 100); A.verN++; A.verIleri += dx; if (dx < -2) A.verGeri++; }
    sonT = ht; sonH = h;
    // 55: taşıyıcının 12 birim içinde rakip yokken geçen süre (kendi yarısı dışında)
    A.holdT++; const hx = k.P.x[t * N + h] / 100, hy = k.P.y[t * N + h] / 100; let yak = false; for (let i = 0; i < N; i++) if (tm(i) !== ht && k.oyuncular[i].rol !== 'Bekçi' && Math.hypot(k.P.x[t * N + i] / 100 - hx, k.P.y[t * N + i] / 100 - hy) < 12) { yak = true; break; } if (!yak && ileri(ht, hx) > W / 2) A.presYok++;
  }
  // 57: baskı altında "tut" seçimi: kararın ilk seçeneği "tut", en yakın rakip < 4; sonra 1 sn içinde top rakibe geçti mi
  for (const kr of k.karar) { if (kr.kim == null || !kr.s || !kr.s.length) continue; const t = kr.t; if (t >= L) continue; const h = kr.kim; if (k.T.hold[t] !== h) continue;
    const hx = k.P.x[t * N + h] / 100, hy = k.P.y[t * N + h] / 100; let en = 1e9; for (let i = 0; i < N; i++) if (tm(i) !== tm(h)) en = Math.min(en, Math.hypot(k.P.x[t * N + i] / 100 - hx, k.P.y[t * N + i] / 100 - hy)); if (en >= 4) continue;
    A.tutN++; if (/^tut/.test(kr.s[0][0])) { A.tutSec++; for (let u = t; u < t + 60 && u < L; u++) { const g = k.T.hold[u]; if (g !== 255 && tm(g) !== tm(h)) { A.tutKay++; break; } } } }
  // kapma
  A.kapma += k.olay.filter(o => o.tur === 'kesme').length;
}
const dk = A.dk, f1 = x => x.toFixed(1), f2 = x => x.toFixed(2);
console.log(`${A.mac} maç (${f1(dk / A.mac * 60)} sn) · sayı/maç ${f2(A.say / A.mac)} · sayı/dk ${f2(A.say / dk)} · gönderme/dk ${f1(A.gon / dk)} · pas/dk ${f1(A.pas / dk)} · kesme/dk ${f1(A.kapma / dk)} · hesap ${(1000 * A.hs / A.tik).toFixed(1)} ms/tik`);
console.log(`52 · saha dışında serbest Çekirdek: ${A.disari} tik · arka çizgiye 1 birimden yakın gönderilip giren: ${A.yakinCizgi}`);
console.log(`51 · devreden sonraki ilk 10 sn'de sayı: ${A.devre10} (maç başına ${f2(A.devre10 / A.mac)}; ortalama 10 sn'de ${f2(A.say / A.mac / (A.tik / A.mac / 600))})`);
console.log(`54/57 · top kazanınca ilk 3 sn: taşıyıcı ort. ${f1(A.kazTut / A.kaz)} tik tutuyor · 3 sn içinde gönderme %${(100 * A.kazSut / A.kaz).toFixed(0)} (${A.kaz} kazanma)`);
console.log(`57 · baskı altında (rakip < 4) kararlar: "tut" seçimi %${(100 * A.tutSec / A.tutN).toFixed(0)} · "tut"tan sonra 1 sn içinde kayıp %${A.tutSec ? (100 * A.tutKay / A.tutSec).toFixed(0) : '-'} (${A.tutN} karar)`);
console.log(`56 · pas veren 1 sn'de ileri ort. ${f1(A.verIleri / A.verN)} birim · 2 birimden çok geri koşan %${(100 * A.verGeri / A.verN).toFixed(0)} (${A.verN} pas)`);
console.log(`55 · rakip yarısında taşıyıcının 12 birim içinde (Bekçi hariç) rakip yok: tutuşun %${(100 * A.presYok / A.holdT).toFixed(1)}`);
console.log(`58 · 60°+ göndermelerde Bekçi'nin yakın direğe uzaklığı ort. ${f1(A.dirUz / A.dirN)} (${A.dirN} gönderme)`);
