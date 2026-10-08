(function () {
  // Değer ölçümü: motorun kendi maçlarından "bu bölgede Çekirdek bizdeyken bu hücum sayıyla biter mi" oranı.
  // Hücum = Çekirdek bir takımdayken, rakip onu alana ya da sayı olana kadar geçen süre. Kısa süreli kaçırmalar (boşta kalıp aynı takıma dönen) hücumu bitirmez.
  // İki tablo: tut (hücumun uğradığı her dilim bir kez: "Çekirdek buraya kadar gelirse") ve kazan (Çekirdeğin rakipten kazanıldığı an; kontra dahil). Mesafe: hücum edilen Kuyu'ya, 8 birimlik dilimler.
  // Durum = (Kuyu'ya mesafe dilimi, Çekirdek ile Kuyu arasında kaç rakip var). İkincisi kontranın özü: önde kaç kişi kaldığını ölçer. Konum tek başına yanıltır (kendi Kuyu'na yakın kazanılan Çekirdek, rakip önde kaldığı için değerlidir).
  const NB = 13, NG = 3, gsBin = n => n <= 1 ? 0 : n <= 3 ? 1 : 2;
  function goalSide(ps, team, x, y) { const gx = team === 0 ? 100 : 0, d = Math.hypot(gx - x, 25 - y); let n = 0; for (const p of ps) if (p.team !== team && p.role !== 'Bekçi' && Math.hypot(gx - p.x, 25 - p.y) < d) n++; return n; }
  // v3: ikinci boyut artık "boşluk" (decide.space): savunmanın ne kadar açık olduğu. Önde kaç rakip kaldığını da içerir (kontrada boşluk büyüktür).
  const spBin = s => s < .1 ? 0 : s < .25 ? 1 : 2;
  const bin = (team, x, y, ps) => Math.min(NB - 1, Math.floor(Math.hypot((team === 0 ? 100 : 0) - x, 25 - y) / 8)) * NG + (ps ? spBin(window.AlanDecide.space(ps.map(p => ({ x: p.x, y: p.y, team: p.team, role: p.role, a: p.a })), team, x, y)) : 0);
  function empty() { return { v: 6, raw: [], hold: Array.from({ length: NB * NG }, () => [0, 0]), win: Array.from({ length: NB * NG }, () => [0, 0]), matches: 0, goals: 0 }; }
  function play(seed, acc) {
    const M = window.AlanMatch, P = window.AlanDecide.D, m = M.createMatch(seed, {});
    let pos = null; const close = (scored) => { if (!pos) return; for (const r of pos.raw) acc.raw.push([r[0], r[1], scored ? 1 : 0]); for (const b of pos.hold) acc.hold[b][0]++, acc.hold[b][1] += scored ? 1 : 0; if (pos.win != null) acc.win[pos.win][0]++, acc.win[pos.win][1] += scored ? 1 : 0; pos = null; };
    let kick = true;
    while (!m.over) {
      const s0 = m.score[0] + m.score[1]; M.step(m);
      if (m.score[0] + m.score[1] > s0) { const sc = m.score[0] + m.score[1] - s0; acc.goals += sc; close(pos && m.score[pos.team] > (pos.s ?? 0)); kick = true; continue; }
      const h = m.holder; if (!h) continue;
      if (!pos || pos.team !== h.team) { close(false); pos = { team: h.team, raw: [], hold: [], win: kick ? null : bin(h.team, h.x, h.y, m.ps), s: m.score[h.team] }; kick = false; }
      { const b = bin(h.team, h.x, h.y, m.ps); if (!pos.hold.includes(b)) pos.raw.push([Math.round(Math.hypot((h.team === 0 ? 100 : 0) - h.x, 25 - h.y)), Math.round(window.AlanDecide.space(m.ps.map(p => ({ x: p.x, y: p.y, team: p.team, role: p.role, a: p.a })), h.team, h.x, h.y) * 1000) / 1000]); if (!pos.hold.includes(b)) pos.hold.push(b); } // her hücum bir dilime ilk girdiğinde bir kez sayılır (süreye göre değil, ziyarete göre)
    }
    close(false); acc.matches++; return acc;
  }
  function merge(a, b) { for (const k of ['hold', 'win']) for (let i = 0; i < NB * NG; i++) { a[k][i][0] += b[k][i][0]; a[k][i][1] += b[k][i][1]; } a.matches += b.matches; a.goals += b.goals; if (b.raw) a.raw = (a.raw || []).concat(b.raw); return a; }
  // Yumuşatma: komşu dilimlerle ağırlıklı ortalama + zayıf bir öncül (az örnekli dilim komşusuna yaslanır); mesafe arttıkça değer artamaz
  // Yumuşatma: aynı rakip-sayısı dilimindeki komşu mesafelerle (ve zayıfça komşu rakip dilimleriyle) ağırlıklı ortalama + küçük öncül; az örnekli hücre komşusuna yaslanır
  function smooth(t) { const out = []; for (let i = 0; i < NB; i++) for (let a = 0; a < NG; a++) { let n = 0, g = 0; for (let j = 0; j < NB; j++) for (let b = 0; b < NG; b++) { const w = Math.exp(-((i - j) ** 2) / 1.2 - ((a - b) ** 2) * 1.5); n += t[j * NG + b][0] * w; g += t[j * NG + b][1] * w; } out.push((g + 1 * .04) / (n + 1)); } return out; }
    // Tablo: her rakip-sayısı diliminde mesafeye göre azalmayan değil, artmayan (ileri gitmek hiçbir zaman zararlı sayılmaz) en iyi uyum (havuzlama), sayılarla ağırlıklı, küçük öncülle.
  // Ham ölçümde geride kazanılan Çekirdek, rakip önde kaldığı için orta sahadan daha değerli görünüyordu; rakip-sayısı dilimi bunu ayırır, havuzlama da kalan karışıklığı düzeltir.
  function table(acc) { const out = new Array(NB * NG).fill(0); for (let a = 0; a < NG; a++) { const blocks = []; for (let i = 0; i < NB; i++) { const c = acc.hold[i * NG + a], n = c[0] + 2, g = c[1] + 2 * .06; blocks.push({ n, g, from: i, to: i }); let k = blocks.length - 1; while (k > 0 && blocks[k - 1].g / blocks[k - 1].n < blocks[k].g / blocks[k].n) { const b = blocks.pop(), p = blocks[k - 1]; p.n += b.n; p.g += b.g; p.to = b.to; k--; } } for (const b of blocks) for (let i = b.from; i <= b.to; i++) out[i * NG + a] = b.g / b.n; }
    // önde daha az rakip olması da hiçbir zaman daha kötü sayılmaz
    for (let i = 0; i < NB; i++) for (let a = 1; a < NG; a++) out[i * NG + a] = Math.max(out[i * NG + a], out[i * NG + a - 1]); return out; } // v3: daha çok boşluk hiçbir zaman daha kötü sayılmaz
  // Sürekli eğri: P(sayı) = σ(a + b·boşluk + c·mesafe/100), ham örneklerden (her hücumun her mesafe dilimine ilk girişi) en çok olabilirlikle. Mesafe katsayısı ileri gitmeyi cezalandıramaz (c ≤ 0).
  function fit(raw) { let a = -2, b = 0, c = 0; const n = raw.length; for (let it = 0; it < 400; it++) { let ga = 0, gb = 0, gc = 0; for (const [d, s, y] of raw) { const z = a + b * s + c * d / 100, p = 1 / (1 + Math.exp(-z)), e = y - p; ga += e; gb += e * s; gc += e * d / 100; } a += 2 * ga / n; b += 8 * gb / n; c += 2 * gc / n; if (c > 0) c = 0; } return { a, b, c, n }; }
  window.AlanValueLab = { fit, table, NB, NG, goalSide, gsBin, bin, empty, play, merge, smooth };
})();
