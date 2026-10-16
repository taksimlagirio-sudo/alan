// v5 · Paralel kafa: kafadaki gönderme denemeleri (kuyruk) yardımcı işçilerde oynanır. Zarların hepsi ana işçide maçtaki sırayla çekilmiştir (decide.js kuyruk), her deneme yalnız kendi tohumuyla oynar: sonuç tek çekirdekle birebir aynı.
// Ana işçi işleri paylaşılan sayaçtan (SharedArrayBuffer) dağıtır, kendisi de iş alır, hepsi bitene kadar Atomics.wait ile bekler. Yardımcı takılırsa ya da hata verirse paralel kapanır, kalan işler ana işçide oynanır.
(function () {
  const GEN = 0, NEXT = 1, DONE = 2, FIN = 3, SLOT = 5;
  const temizD = d => { const o = {}; for (const k in d) { const v = d[k]; if (k === '_mac') o[k] = v ? 1 : 0; else if (typeof v !== 'function') o[k] = v; } return o; };
  function anaKur(yardimcilar) { // yardimcilar: [{ post(msg) }]
    const nY = yardimcilar.length; let kap = 0, sab, ctl, st, res;
    const buyut = n => { kap = Math.max(64, n * 2); sab = new SharedArrayBuffer(16 + kap * 4 + kap * SLOT * 8 + 8); ctl = new Int32Array(sab, 0, 4); st = new Int32Array(sab, 16, kap); res = new Float64Array(sab, 16 + kap * 4 + ((16 + kap * 4) % 8 ? 8 - (16 + kap * 4) % 8 : 0), kap * SLOT); for (const y of yardimcilar) y.post({ tip: 'sab', sab, kap }); };
    const P = { hazir: false, n: nY, say: { is: 0, yard: 0, bosalt: 0 } }; /* yardımcılar 'hazir' deyince açılır (P.hazirlandi): tarayıcıda alt işçi, onu kuran işçi boşta değilse yüklenemeyebilir */ let hz = 0; P.hazirlandi = () => { if (++hz === nY) P.hazir = true; };
    P.oyna = (isler, D, isOyna) => {
      const n = isler.length; if (n > kap) buyut(n);
      const Mr = isler.find(j => j.d._mac) ? isler.find(j => j.d._mac).d._mac : null;
      const mac = Mr ? Object.assign({}, Mr, { events: [], lastDec: null, pre: null, r: { s: Mr.r.s } }) : null;
      const yuk = isler.map(j => ({ src: j.src, hI: j.src.indexOf(j.h), qI: j.q ? j.src.indexOf(j.q) : -1, seen: j.seen.map(s => ({ x: s.x, y: s.y })), b: { x: j.b.x, y: j.b.y, vx: j.b.vx, vy: j.b.vy, ch: j.b.ch }, to: j.to ? { x: j.to.x, y: j.to.y } : null, tohum: j.tohum, d: temizD(j.d) }));
      const gen = (Atomics.load(ctl, GEN) + 1) | 0; for (let i = 0; i < n; i++) st[i] = 0; Atomics.store(ctl, NEXT, 0); Atomics.store(ctl, DONE, 0); Atomics.store(ctl, FIN, 0); Atomics.store(ctl, GEN, gen);
      try { for (const y of yardimcilar) y.post({ tip: 'is', gen, n, mac, isler: yuk }); } catch (e) { P.hazir = false; for (const j of isler) j.R = isOyna(j); return; }
      for (;;) { const i = Atomics.add(ctl, NEXT, 1); if (i >= n) break; isler[i].R = isOyna(isler[i]); st[i] = -1; Atomics.add(ctl, DONE, 1); } /* ana işçi de iş alır */
      const t0 = Date.now(); let dur = false;
      while (Atomics.load(ctl, FIN) < nY) { const f = Atomics.load(ctl, FIN); Atomics.wait(ctl, FIN, f, 200); if (Date.now() - t0 > 15000) { dur = true; break; } }
      if (dur) P.hazir = false; /* yardımcı yanıt vermiyor: paralel kapanır */
      let bozuk = false; for (let i = 0; i < n; i++) { const j = isler[i]; if (st[i] === -1) continue; if (st[i] === 1) { const b = i * SLOT; j.R = { m: res[b], o: res[b + 1], k: res[b + 2], x: res[b + 3], y: res[b + 4] }; P.say.yard++; } else { if (st[i] === 2) bozuk = true; j.R = isOyna(j); } }
      if (bozuk) self.ALAN_KUYRUK_BOZUK = true; P.say.is += n; P.say.bosalt++;
    };
    return P;
  }
  function yardimciKur(rng) { // yardımcı işçide: mesajları işler
    let sab, ctl, st, res; const A = () => self.AlanDecide;
    return function mesaj(data) {
      if (data.tip === 'sab') { sab = data.sab; const kap = data.kap; ctl = new Int32Array(sab, 0, 4); st = new Int32Array(sab, 16, kap); res = new Float64Array(sab, 16 + kap * 4 + ((16 + kap * 4) % 8 ? 8 - (16 + kap * 4) % 8 : 0), kap * SLOT); return; }
      if (data.tip !== 'is') return; const { gen, n, mac, isler } = data; if (mac) { const r = rng(0); r.s = mac.r.s; mac.r = r; }
      const D = A().D;
      try { for (;;) { if (Atomics.load(ctl, GEN) !== gen) break; const i = Atomics.add(ctl, NEXT, 1); if (i >= n) break; const j = isler[i];
          const kay = {}; for (const k in j.d) { kay[k] = D[k]; D[k] = k === '_mac' ? (j.d[k] ? mac : null) : j.d[k]; } kay._kuyruk = D._kuyruk; D._kuyruk = null; self.ALAN_KUYRUK_BOZUK = false; let R = null;
          try { R = A().kafaOyna(j.src[j.hI], j.src, j.seen, j.b, j.qI >= 0 ? j.src[j.qI] : null, j.to, j.tohum); } catch (e) { R = null; } finally { for (const k in kay) D[k] = kay[k]; }
          if (!R || self.ALAN_KUYRUK_BOZUK) { st[i] = 2; } else { const b = i * SLOT; res[b] = R.m; res[b + 1] = R.o; res[b + 2] = R.k; res[b + 3] = R.x; res[b + 4] = R.y; Atomics.store(st, i, 1); }
          Atomics.add(ctl, DONE, 1); } }
      finally { Atomics.add(ctl, FIN, 1); Atomics.notify(ctl, FIN); }
    };
  }
  self.AlanParalel = { anaKur, yardimciKur };
})();
