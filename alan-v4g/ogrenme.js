(function () {
  // Hafıza (v4f): durum değeri tablosu beynin kendi maçlarından öğrenilir. V(durum) = P(sıradaki sayı bizim) − P(rakibin).
  // Kayıt: gerçek maçta (ileri oynatmada değil) her ADIM tikte top sahibinin durumu. Sayı olunca o ana kadarki bütün kayıtlar +1 (sayıyı atan takım) / −1 ile güncellenir; maç bitince sonucu belli olmayan kayıtlar atılır.
  // Güncelleme: hücre başına ortalama (α = 1/n), en az AMIN; böylece tablo hem oturur hem de değişen oyuna uyar.
  const O = { ADIM: 6, AMIN: .02 };
  function fnv(s) { let h = 2166136261 >>> 0; for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 16777619) >>> 0; } return h.toString(16); }
  function basla(m) { m._og = { bek: [], s0: m.score.slice() }; }
  function guncelle(i, y) { const T = window.ALAN_VS; T.n[i] = (T.n[i] || 0) + 1; const a = Math.max(1 / T.n[i], O.AMIN); T.v[i] += a * (y - T.v[i]); }
  function adim(m) { const g = m._og; if (!g || m._ic) return;
    if (m.score[0] !== g.s0[0] || m.score[1] !== g.s0[1]) { const sc = m.score[0] > g.s0[0] ? 0 : 1; for (const e of g.bek) guncelle(e.i, e.t === sc ? 1 : -1); g.bek = []; g.s0 = m.score.slice(); }
    const h = m.holder; if (h && m.tick % O.ADIM === 0) { const tr = m.winTeam === h.team && (m.tick - (m.winT ?? -1e9)) < 180; g.bek.push({ i: window.AlanState.idx(m.ps, h.team, h.x, h.y, tr), t: h.team }); } }
  // Fizik damgası: motor dosyalarının içeriği. Değişince hafıza eskimiş sayılır: değerler başlangıç olarak kalır ama sayımlar küçültülür, yeni maçlar hızla üstüne yazar.
  async function damga(dir) { const fs = ['core.js', 'match.js', 'decide.js', 'shape.js', 'beyin.js']; let s = ''; for (const f of fs) s += await (await fetch(dir + f)).text(); return fnv(s); }
  window.AlanOgren = { O, fnv, basla, adim, guncelle, damga };
})();
