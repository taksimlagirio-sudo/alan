// Birleşik ölçüm, adım 2: kayit.js kayıtlarını okur ve ölçer. Maç yeniden oynatılmaz.
// Modül: oku(dosya) → kayıt (diziler açılmış). Ölçümler: ol.genel, ol.durma, ol.salinim, ol.kosu, ol.pasTahmin, ol.evre
const fs = require('fs'), zlib = require('zlib');
const TIP = { hold: Uint8Array, bx: Int16Array, by: Int16Array, bt: Int8Array, ch: Uint8Array, ev0: Int8Array, ev1: Int8Array, gcT: Int8Array, gcGit: Uint8Array, gcOn: Uint8Array, kp: Uint8Array, kpT: Int8Array, x: Int16Array, y: Int16Array, vx: Int16Array, vy: Int16Array, tx: Int16Array, ty: Int16Array, job: Uint8Array };
const ac = (s, C) => { const b = Buffer.from(s, 'base64'), a = new Uint8Array(b.length); a.set(b); return new C(a.buffer); };
function oku(f) { const k = JSON.parse(zlib.gunzipSync(fs.readFileSync(f))); for (const [g, o] of [['T', k.T], ['P', k.P]]) for (const key of Object.keys(o)) o[key] = ac(o[key], TIP[key]); k.lane = ac(k.lane, Uint8Array); return k; }
const ileri = (t, x) => t === 0 ? x : 100 - x;

const ol = {
  genel(k) { const s = k.st; return { sayi: k.skor[0] + k.skor[1], pas: s.pass[0] + s.pass[1], pasTut: (s.passOk[0] + s.passOk[1]) / (s.pass[0] + s.pass[1]), gon: s.shot[0] + s.shot[1], kapma: s.steal[0] + s.steal[1], ikili: s.duel[0] + s.duel[1], elDegis: k.sahip.filter((x, i, a) => x.takim != null && i && (() => { for (let j = i - 1; j >= 0; j--) if (a[j].takim != null) return a[j].takim !== x.takim; return false; })()).length, hesapSn: k.hesapSn }; },
  // Durma: v5-durma.js ile aynı tanım (hız < 0,04 birim/tik; 1 sn = 60 tik; uzun tutuş = taşıyıcı ≥ 120 tik). Ek: uzun tutuşu Bekçi mi yapıyor.
  durma(k) { const { N, n, T, P, oyuncular: O } = k, dur = new Int32Array(N); let hT = 0, h0 = 255; const S = { n: 0, dH: 0, dS: 0, uz: { n: 0, dH: 0, dS: 0, uH: 0, yol: 0, yolN: 0 }, uzB: { n: 0, dH: 0, dS: 0, uH: 0 }, yol: 0, yolN: 0 };
    for (let t = 0; t < n; t++) { const hi = T.hold[t]; for (let i = 0; i < N; i++) { const q = t * N + i; dur[i] = Math.hypot(P.vx[q], P.vy[q]) < 40 ? dur[i] + 1 : 0; } if (hi !== h0) { h0 = hi; hT = t; } if (hi === 255) continue; const ht = O[hi].takim, bek = O[hi].rol === 'Bekçi';
      let dH = 0, dS = 0, uH = 0; for (let i = 0; i < N; i++) { if (i === hi || O[i].rol === 'Bekçi') continue; if (dur[i] > 0) O[i].takim === ht ? dH++ : dS++; if (dur[i] >= 60 && O[i].takim === ht) uH++; } S.n++; S.dH += dH; S.dS += dS;
      let y = null; if (t % 6 === 0) { y = 0; for (let i = 0; i < N; i++) { const v = k.lane[(t / 6) * N + i]; if (v !== 255 && v >= 70) y++; } S.yol += y; S.yolN++; }
      if (t - hT >= 120) { const U = bek ? S.uzB : S.uz; U.n++; U.dH += dH; U.dS += dS; U.uH += uH; if (!bek && y != null) { U.yol += y; U.yolN++; } } }
    const o = (a, b) => b ? a / b : null; return { hucDuran: o(S.dH, S.n), savDuran: o(S.dS, S.n), acikYol: o(S.yol, S.yolN), uzunPay: o(S.uz.n + S.uzB.n, S.n), uzunBekciPay: o(S.uzB.n, S.uz.n + S.uzB.n),
      uzun: { pay: o(S.uz.n, S.n), hucDuran: o(S.uz.dH, S.uz.n), uzunDuran: o(S.uz.uH, S.uz.n), savDuran: o(S.uz.dS, S.uz.n), acikYol: o(S.uz.yol, S.uz.yolN) }, uzunBekci: { pay: o(S.uzB.n, S.n), hucDuran: o(S.uzB.dH, S.uzB.n), savDuran: o(S.uzB.dS, S.uzB.n) } }; },
  // Salınım: v5-durma.js ile aynı (hücumda topsuzlar; savunmada bölge/kapatma/markaj görevindekiler)
  salinim(k) { const { N, n, T, P, oyuncular: O, gorevler: J } = k, SL = { huc: { h: 0, v: 0, g: 0, n: 0 }, sav: { h: 0, v: 0, g: 0, n: 0 } }, HD = new Array(N).fill(null);
    for (let t = 0; t < n; t++) { const hi = T.hold[t]; for (let i = 0; i < N; i++) { const q = t * N + i; if (hi === 255 || i === hi || O[i].rol === 'Bekçi' || P.tx[q] === -32768) { HD[i] = null; continue; } const G = O[i].takim === O[hi].takim ? SL.huc : /^(bölge|kapatma|markaj)/.test(J[P.job[q]]) ? SL.sav : null; if (!G) { HD[i] = null; continue; }
      G.n++; const tx = P.tx[q] / 100, ty = P.ty[q] / 100; let s = HD[i]; if (!s) { s = HD[i] = { x: tx, y: ty, t, v: 0, g: [] }; }
      if (Math.hypot(tx - s.x, ty - s.y) > 2) { G.h++; if (s.v) G.v++; const g2 = s.g[s.g.length - 1]; if (g2 && t - g2.t < 180 && Math.hypot(tx - g2.x, ty - g2.y) < 2) G.g++; s.g.push({ x: s.x, y: s.y, t: s.t }); if (s.g.length > 3) s.g.shift(); s.x = tx; s.y = ty; s.t = t; s.v = 0; }
      else if (Math.hypot(P.x[q] / 100 - s.x, P.y[q] / 100 - s.y) < 1.5) s.v = 1; } }
    const f = G => ({ hedefDk: G.n ? G.h * 3600 / G.n : null, varma: G.h ? G.v / G.h : null, geriDonus: G.h ? G.g / G.h : null }); return { huc: f(SL.huc), sav: f(SL.sav) }; },
  // Koşu: v5-kosu2.js ile aynı tanım (son çizgi = en geriki Bekçi dışı savunmacı; koşu 150 tik açık)
  kosu(k) { const W = (k.saha && k.saha.W) || 100, ileri = (t, x) => t === 0 ? x : W - x; const { N, n, T, P, oyuncular: O } = k, R = [0, 1].map(() => ({ kosu: 0, gecti: 0, pas: 0, pasTut: 0, pasArka: 0, pasArkaTut: 0, kosuArka: 0, pasHep: 0, arkadaAn: 0, hucAn: 0 })), acik = new Array(N).fill(null), PK = ['pas', 'önüne', 'aşırt', 'kenardan'];
    const line = (t, tm) => { let l = 0; for (let i = 0; i < N; i++) if (O[i].takim !== tm && O[i].rol !== 'Bekçi') l = Math.max(l, ileri(tm, P.x[t * N + i] / 100)); return l; };
    const pasT = new Map(k.pas.map(p => [p.t, p])), sonraki = t => { for (const s of k.sahip) if (s.t > t && s.kim != null) return s; return null; };
    for (let t = 0; t < n; t++) { const p = pasT.get(t); if (p && PK.includes(p.tur) && p.alici != null && p.to && p.kim != null) { const tm = O[p.kim].takim; if (O[p.alici].takim === tm) { const Q = R[tm]; Q.pasHep++; const l = line(t, tm), kz = acik[p.alici], kosuya = !!(kz && t - kz.t0 <= 150), arkaya = ileri(tm, p.to[0] / 100) > l + 1, s = sonraki(t), ok = s && s.takim === tm; if (kosuya) { Q.pas++; if (ok) Q.pasTut++; } if (arkaya) { Q.pasArka++; if (ok) Q.pasArkaTut++; } if (kosuya && arkaya) Q.kosuArka++; } }
      const hi = T.hold[t]; if (hi === 255) continue; const tm = O[hi].takim, Q = R[tm], l = line(t, tm); Q.hucAn++;
      for (let i = 0; i < N; i++) { if (O[i].takim !== tm) { acik[i] = null; continue; } if (i === hi || O[i].rol === 'Bekçi') continue; const q = t * N + i, f = ileri(tm, P.x[q] / 100), ft = P.tx[q] === -32768 ? f : ileri(tm, P.tx[q] / 100), v = (tm === 0 ? P.vx[q] : -P.vx[q]) / 1000;
        if (f > l) Q.arkadaAn++; const kz = acik[i]; if (kz && t - kz.t0 > 150) acik[i] = null; if (kz && !kz.gecti && f > l + 1) { kz.gecti = 1; Q.gecti++; } if (!acik[i] && ft > l + 2 && f <= l + 1 && v > .02) { acik[i] = { t0: t, gecti: 0 }; Q.kosu++; } } }
    return R; },
  // Pasın maçtaki tahmini ile gerçeği (kopyasız): P ile pastan sonra topa ilk sahip olan takım. Kopyalı kalibrasyon için v5-kalib3.js.
  pasTahmin(k) { const O = k.oyuncular, out = []; for (const p of k.pas) { if (p.tur === 'gönder' || p.P == null || p.kim == null) continue; const tm = O[p.kim].takim; let s = null; for (const x of k.sahip) if (x.t > p.t && x.kim != null) { s = x; break; } if (!s) continue; out.push({ tur: p.tur, P: p.P, ok: s.takim === tm ? 1 : 0, fake: !!p.fake, ot: p.ot }); } return out; },
  evre(k) { const c = [0, 1].map(() => new Array(8).fill(0)); for (let t = 0; t < k.n; t++) { if (k.T.ev0[t] >= 0) c[0][k.T.ev0[t]]++; if (k.T.ev1[t] >= 0) c[1][k.T.ev1[t]]++; } return c.map(a => a.map(x => x / k.n)); }
};
module.exports = { oku, ol };
