// v5 · Paralel kafa: bağımsız görevler (kafadaki gönderme denemeleri, yalancı hareket dünyalarındaki kararlar) yardımcı işçilerde oynanır. Her görev kendi tohumuyla oynar (decide.js altZar, gorevYap): aynı çekirdekte ya da yardımcıda sonuç birebir aynı.
// Ana işçi görevleri paylaşılan sayaçtan (SharedArrayBuffer) dağıtır, kendisi de görev alır, hepsi bitene kadar Atomics.wait ile bekler. Yardımcı takılırsa ya da hata verirse paralel kapanır, kalan görevler ana işçide oynanır.
(function () {
  const GEN = 0, NEXT = 1, DONE = 2, FIN = 3, SLOT = 5, TUR = ['pas', 'önüne', 'kenardan', 'gönder', 'sür', 'tut', 'çalım', 'fake'];
  const temizD = d => { const o = {}; for (const k in d) { const v = d[k]; if (k === '_mac') o[k] = v ? 1 : 0; else if (typeof v !== 'function') o[k] = v; } return o; };
  const dizi = (sab, kap) => [new Int32Array(sab, 0, 4), new Int32Array(sab, 16, kap), new Float64Array(sab, 16 + kap * 4, kap * SLOT)]; /* kap çift: Float64 hizalı */
  function yukle(j) { const o = { tip: j.tip, src: j.src, hI: j.src.indexOf(j.h), tohum: j.tohum, d: temizD(j.d) };
    if (j.tip === 'karar') { o.ch = j.ch; o.tac = j.tac; if (j.tac && typeof j.tac.macZ === 'function') { o.tac = { ...j.tac }; delete o.tac.macZ; o.tacMac = 1; } } /* zaman değeri (macZ) bir işlev: yardımcıda maçın kopyasından aynı yolla yeniden kurulur */ else { o.qI = j.q ? j.src.indexOf(j.q) : -1; o.seen = j.seen.map(s => ({ x: s.x, y: s.y })); o.b = { x: j.b.x, y: j.b.y, vx: j.b.vx, vy: j.b.vy, ch: j.b.ch }; o.to = j.to ? { x: j.to.x, y: j.to.y } : null; } return o; }
  function anaKur(yardimcilar) { // yardimcilar: [{ post(msg) }]
    const nY = yardimcilar.length; let kap = 0, ctl, st, res;
    const buyut = n => { kap = Math.max(64, n * 2); const sab = new SharedArrayBuffer(16 + kap * 4 + kap * SLOT * 8); [ctl, st, res] = dizi(sab, kap); for (const y of yardimcilar) y.post({ tip: 'sab', sab, kap }); };
    const P = { hazir: false, n: nY, say: { is: 0, yard: 0, bosalt: 0 } }; /* yardımcılar 'hazir' deyince açılır: tarayıcıda alt işçi, onu kuran işçi boşta değilse yüklenemeyebilir */ let hz = 0; P.hazirlandi = () => { if (++hz === nY) P.hazir = true; };
    P.oyna = (G, D, yap) => {
      const n = G.length; if (n > kap) buyut(n);
      const Mr = G.find(j => j.d._mac) ? G.find(j => j.d._mac).d._mac : null;
      const mac = Mr ? Object.assign({}, Mr, { events: [], lastDec: null, pre: null, r: { s: Mr.r.s } }) : null;
      const yuk = G.map(yukle);
      const gen = (Atomics.load(ctl, GEN) + 1) | 0; for (let i = 0; i < n; i++) st[i] = 0; Atomics.store(ctl, NEXT, 0); Atomics.store(ctl, DONE, 0); Atomics.store(ctl, FIN, 0); Atomics.store(ctl, GEN, gen);
      try { for (const y of yardimcilar) y.post({ tip: 'is', gen, n, mac, G: yuk }); } catch (e) { P.hazir = false; P.hata = String(e && e.message || e).slice(0, 300); for (const j of G) j.R = yap(j); return; }
      for (;;) { const i = Atomics.add(ctl, NEXT, 1); if (i >= n) break; G[i].R = yap(G[i]); st[i] = -1; Atomics.add(ctl, DONE, 1); } /* ana işçi de görev alır */
      const t0 = Date.now(); let dur = false;
      while (Atomics.load(ctl, FIN) < nY) { const f = Atomics.load(ctl, FIN); Atomics.wait(ctl, FIN, f, 200); if (Date.now() - t0 > 15000) { dur = true; break; } }
      if (dur) { P.hazir = false; P.hata = P.hata || "zaman aşımı"; } /* yardımcı yanıt vermiyor: paralel kapanır */
      let bozuk = false; for (let i = 0; i < n; i++) { const j = G[i]; if (st[i] === -1) continue; const b = i * SLOT;
        if (st[i] === 1) { j.R = j.tip === 'karar' ? { best: res[b + 2] ? { v: res[b], kind: TUR[res[b + 1]] } : null } : { m: res[b], o: res[b + 1], k: res[b + 2], x: res[b + 3], y: res[b + 4] }; P.say.yard++; }
        else { if (st[i] === 2) bozuk = true; j.R = yap(j); } }
      if (bozuk) self.ALAN_KUYRUK_BOZUK = true; P.say.is += n; P.say.bosalt++;
    };
    return P;
  }
  function yardimciKur(rng) { // yardımcı işçide: mesajları işler
    let ctl, st, res;
    return function mesaj(data) {
      if (data.tip === 'sab') { [ctl, st, res] = dizi(data.sab, data.kap); return; }
      if (data.tip !== 'is') return; const { gen, n, mac, G } = data; if (mac) { const r = rng(0); r.s = mac.r.s; mac.r = r; }
      const A = self.AlanDecide;
      try { for (;;) { if (Atomics.load(ctl, GEN) !== gen) break; const i = Atomics.add(ctl, NEXT, 1); if (i >= n) break; const j = G[i];
          for (const k in j.d) if (k === '_mac') j.d[k] = j.d[k] ? mac : null; const g = { ...j, h: j.src[j.hI], q: j.qI >= 0 ? j.src[j.qI] : null }; if (j.tacMac && mac) g.tac = self.AlanMatch.macEff(mac, g.h.team, j.tac); self.ALAN_KUYRUK_BOZUK = false; let R = null;
          try { R = A.gorevYap(g); } catch (e) { R = null; }
          const b = i * SLOT; let ok = !!R && !self.ALAN_KUYRUK_BOZUK;
          if (ok && j.tip === 'karar') { const kd = R.best ? TUR.indexOf(R.best.kind) : 0; if (R.best && kd < 0) ok = false; else { res[b] = R.best ? R.best.v : 0; res[b + 1] = kd; res[b + 2] = R.best ? 1 : 0; } }
          else if (ok) { res[b] = R.m; res[b + 1] = R.o; res[b + 2] = R.k; res[b + 3] = R.x; res[b + 4] = R.y; }
          Atomics.store(st, i, ok ? 1 : 2); Atomics.add(ctl, DONE, 1); } }
      finally { Atomics.add(ctl, FIN, 1); Atomics.notify(ctl, FIN); }
    };
  }
  self.AlanParalel = { anaKur, yardimciKur };
})();
