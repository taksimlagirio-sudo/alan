// ALAN v5 · paralel kafa (22 Ekim 2026). Aynı maçı oynayan işçiler (birbirinin kopyası: aynı tohum, aynı kurulum, aynı taktik değişiklikleri) kararın bağımsız işlerini paylaşımlı bellekle bölüşür.
/* Her işçi maçı baştan sona kendisi oynar; karar bir dalgaya (birbirinden bağımsız işler: seçeneklerin tartısı) gelince bütün işçiler dalganın başında buluşur, işleri paylaşımlı sayaçtan sırayla alır, sonucu paylaşımlı belleğe yazar, sonra herkes ötekilerin sonuçlarını okur ve kararına aynı sırayla ekler. İşler sıradan bağımsız olduğu için (decide.js: dalgaKos, sıradan bağımsız önbellekler) sonuç kimin hangi işi hesapladığından bağımsızdır: tek çekirdekle bit bit aynı maç.
   Kurulum: işçi, maçtan önce self.ALAN_PARALEL_KUR({ sab, n, id }) çağırır (sab: SharedArrayBuffer, n: işçi sayısı, id: 0 ana işçi). Paylaşımlı bellek yoksa (sayfa çapraz köken yalıtımlı değil) hiç kurulmaz: motor tek çekirdekte aynı sonucu verir.
   Buluşma: her işçi geldiği dalgayı ve kuşağını kendi yuvasına yazar; dalgayı ana işçi kurar, ancak bütün yardımcılar bu kuşakla bu dalgaya geldikten sonra (önceki dalgayı okumaları bitmiştir). Vazgeçme: ana işçi maçı yeniden kurunca (taktik değişikliği, yeni maç) kuşağı artırır ve yeni kuşağı iletiyle yardımcılara gönderir; ana işçinin kurduğu dalgayı yardımcı, kuşak değişmiş olsa da tamamlar (aynı zaman çizgisi), ana işçinin artık kurmayacağı dalgada ALAN_PARALEL_VAZGEC hatasıyla maçtan çıkar. Zaman aşımı (bir işçi takıldı ya da öldü): bekleyen işçi bitmemiş işleri kendisi hesaplar (sonuç aynı), paralel kip kapanır. */
(function () {
  const self = globalThis;
  const WID = 0, WGEN = 1, NEXT = 2, DONE = 3, NN = 4, PTR = 5, GEN = 6, BOZUK = 7, AW = 8, AG = 40, SQ = 72, IT = 80, MAXIS = 4096, DATA = (IT + 4 * MAXIS) * 4; /* Int32 kontrol alanı: kurulan dalga ve kuşağı, sıradaki iş, biten iş, iş sayısı, veri imleci, kuşak, bozuk bayrağı; işçi başına geldiği dalga (AW), kuşağı (AG) ve dalga dışında geçirdiği süre (SQ, 0,1 ms), en çok 8 işçi; iş tablosu (veri başı, uzunluk, hesap süresi µs, hesaplayan işçi) */
  const ENC = new TextEncoder(), DEC = new TextDecoder(), DILIM = 50, SURE = 15000, PENCERE = 4000; /* bekleme dilimi (ms; kuşak ve bozuk bayrağı aralarda okunur), en uzun bekleme, kazanç denetiminin penceresi (ana işçinin meşgul süresi) */
  let P = null;
  const vazgec = () => { const e = new Error('ALAN_PARALEL_VAZGEC'); e.vazgec = true; return e; };
  /* bekle: koşul sağlanana kadar dilim dilim bekler; koşul sağlanmamışken kuşak değiştiyse vazgeçer (önce koşul: ana işçinin kurup bitirdiği dalga, kuşak sonradan değişse de tamamlanır); süre dolarsa paralel kip bozulur, false döner */
  function bekle(i, kosul) { const C = P.C, t0 = Date.now(); for (;;) { if (kosul()) return true; if (Atomics.load(C, GEN) !== P.gen) throw vazgec(); if (Atomics.load(C, BOZUK)) return false; if (Date.now() - t0 > SURE) { Atomics.store(C, BOZUK, 1); Atomics.notify(C, i); P.neden = 'süre doldu (yuva ' + i + ', dalga ' + P.w + ')'; return false; } Atomics.wait(C, i, Atomics.load(C, i), DILIM); } }
  /* kodlama: oyuncu ve dünya başvuruları korunur (ref: dalganın başında her işçide aynı sırayla kurulan nesne tablosu), aynı nesne iki kez görülürse bir kez yazılır, -0, NaN, sonsuzlar ve tanımsız alanlar korunur. Düz olmayan nesne (Map, sınıf, işlev) hatadır: o iş paylaşılamaz */
  function kodla(v, ref, taban, refA) { /* ref: nesne → tablo sırası (Map); refA: tablo (dizi); taban(x): kopyanın asıl oyuncusunun tablo sırası ya da -1 */
    const ids = new Map(); let nid = 0;
    const K = x => {
      if (x === null) return null; const t = typeof x;
      if (t === 'number') return x === x && x !== Infinity && x !== -Infinity ? (x === 0 && 1 / x < 0 ? { $: 'z' } : x) : { $: x !== x ? 'n' : x > 0 ? 'i' : 'j' };
      if (t === 'string' || t === 'boolean') return x; if (t === 'undefined') return { $: 'u' };
      if (t !== 'object') throw new Error('paralel: kodlanamayan değer (' + t + ')');
      const r = ref.get(x); if (r !== undefined) return { $: 'r', r };
      const g = ids.get(x); if (g !== undefined) return { $: 'g', i: g };
      const id = nid++; ids.set(x, id);
      if (Array.isArray(x)) { const a = new Array(x.length); for (let k = 0; k < x.length; k++) a[k] = K(x[k]); return { $: 'a', id, a }; }
      const pr = Object.getPrototypeOf(x); if (pr !== Object.prototype && pr !== null) throw new Error('paralel: düz olmayan nesne');
      const ks = Object.keys(x), bi = taban ? taban(x) : -1;
      if (bi >= 0) { const b = refA[bi], kb = Object.keys(b); let ayni = kb.length <= ks.length; for (let k = 0; ayni && k < kb.length; k++) ayni = ks[k] === kb[k]; if (ayni) { const c = {}; for (const k of ks) if (!(k in b) || x[k] !== b[k] || (x[k] === 0 && 1 / x[k] !== 1 / b[k])) c[k] = K(x[k]); return { $: 'd', id, b: bi, c }; } } /* oyuncu kopyası ({ ...p, x, y }): asıl oyuncunun alanları aynı sırayla, yalnız farklar yazılır */
      const o = {}; for (const k of ks) o[k] = K(x[k]); return { $: 'o', id, o };
    };
    return K(v);
  }
  function coz(v, ref) {
    const ids = [];
    const C0 = x => {
      if (x === null || typeof x !== 'object') return x;
      switch (x.$) { case 'z': return -0; case 'n': return NaN; case 'i': return Infinity; case 'j': return -Infinity; case 'u': return undefined; case 'r': return ref[x.r]; case 'g': return ids[x.i];
        case 'a': { const a = new Array(x.a.length); ids[x.id] = a; for (let k = 0; k < x.a.length; k++) a[k] = C0(x.a[k]); return a; }
        case 'o': { const o = {}; ids[x.id] = o; for (const k in x.o) o[k] = C0(x.o[k]); return o; }
        case 'd': { const o = {}, b = ref[x.b]; ids[x.id] = o; for (const k of Object.keys(b)) o[k] = b[k]; for (const k in x.c) o[k] = C0(x.c[k]); return o; } }
      throw new Error('paralel: bozuk paket');
    };
    return C0(v);
  }
  /* dagit(n, hesapla, uygula): dalga. hesapla(i, yerel): işi bu işçide hesaplar (kararın nesnesine yazar), yerel değilse paketini döndürür; uygula(i, paket): başka işçinin hesapladığı işi bu işçinin kararına ekler. Bütün işçiler aynı dalgaları aynı sırayla çağırır (aynı maç).
     Kazanç denetimi (ana işçi): paralelin bedeli (yardımcıyı bekleme, paketleme, paket çözme) ile kazancı (başka işçinin hesapladığı işlerin süresi, o işçinin hızından ana işçinin hızına çevrilmiş) sayılır. Hız oranı aynı işten: her işçi dalga dışındaki süresini (iki dalga arası, hepsinde aynı hesap) yazar. Ana işçinin 4 sn'lik meşgul süresinde bedel kazancı geçerse paralel bir yük demektir (yavaş çekirdeğe düşmüş ya da çekirdek yetmeyen yardımcı): paralel kip kapanır, maç tek çekirdekte aynı sonuçla sürer */
  function dagit(n, hesapla, uygula) {
    const C = P.C, U = P.U, N = P.n, id = P.id, hepsi = () => { for (let i = 0; i < n; i++) hesapla(i, true); };
    if (n > MAXIS || P.bozuk || Atomics.load(C, BOZUK)) { P.bozuk = true; hepsi(); return; }
    const t0 = performance.now(), ara = P.sonT != null ? t0 - P.sonT : 0; P.seqT += ara;
    try { dalga(n, hesapla, uygula, hepsi, C, U, N, id); }
    catch (e) { if (e && e.vazgec) P.seqT -= ara; throw e; } /* vazgeçilen dalganın arası ana işçide sayılmadı (o dalgayı kurmadı): burada da sayılmaz */
    finally { P.sonT = performance.now(); P.dalgaT += P.sonT - t0; }
  }
  function dalga(n, hesapla, uygula, hepsi, C, U, N, id) {
    const w = ++P.w;
    Atomics.store(C, SQ + id, Math.round(P.seqT * 10)); Atomics.store(C, AG + id, P.gen); Atomics.store(C, AW + id, w); Atomics.notify(C, AW + id);
    let Z = null, hq = null;
    if (id === 0) { /* ana işçi: bütün yardımcılar bu kuşakla bu dalgaya gelince dalgayı kurar */
      const a0 = performance.now();
      for (let j = 1; j < N; j++) if (!bekle(AW + j, () => Atomics.load(C, AG + j) === P.gen && Atomics.load(C, AW + j) >= w)) { P.bozuk = true; hepsi(); return; }
      hq = new Float64Array(N); for (let j = 1; j < N; j++) hq[j] = Atomics.load(C, SQ + j) / 10; /* yardımcıların bu dalgaya kadarki dalga dışı süresi */
      Z = P.ist || (P.ist = { m0: P.seqT, d0: P.dalgaT, h0: hq, yuk: 0, ham: new Float64Array(N) }); Z.yuk += performance.now() - a0;
      Atomics.store(C, NEXT, 0); Atomics.store(C, DONE, 0); Atomics.store(C, NN, n); Atomics.store(C, PTR, 0); for (let i = 0; i < n; i++) { C[IT + 4 * i] = 0; C[IT + 4 * i + 1] = -1; }
      Atomics.store(C, WGEN, P.gen); Atomics.store(C, WID, w); Atomics.notify(C, WID);
    } else if (!bekle(WID, () => Atomics.load(C, WID) === w && Atomics.load(C, WGEN) === P.gen)) { P.bozuk = true; hepsi(); return; }
    const benim = new Uint8Array(n);
    for (;;) { const i = Atomics.add(C, NEXT, 1); if (i >= n) break; benim[i] = 1; let len = -2, off = 0, us = 0;
      try { const h0 = performance.now(), pk = hesapla(i, false), h1 = performance.now(); us = Math.round((h1 - h0) * 1000); let b = null; if (pk !== undefined) try { b = ENC.encode(JSON.stringify(pk)); } catch (e) { b = null; } if (!b) len = -2; else { off = Atomics.add(C, PTR, b.length); if (DATA + off + b.length <= U.length) { U.set(b, DATA + off); len = b.length; } else len = -3; } if (Z) Z.yuk += performance.now() - h1; } /* -2: sonuç paketlenemedi, -3: paket sığmadı; ikisinde de okuyan kendisi hesaplar (hesaplayanın sonucu yerinde) */
      finally { if (Atomics.load(C, GEN) === P.gen) { C[IT + 4 * i] = off; C[IT + 4 * i + 2] = us; C[IT + 4 * i + 3] = id; Atomics.store(C, IT + 4 * i + 1, len); Atomics.add(C, DONE, 1); Atomics.notify(C, DONE); } } }
    const b0 = performance.now(), tamam = bekle(DONE, () => Atomics.load(C, DONE) >= n); if (!tamam) P.bozuk = true; if (Z) Z.yuk += performance.now() - b0;
    for (let i = 0; i < n; i++) if (!benim[i]) { const len = Atomics.load(C, IT + 4 * i + 1); if (tamam && len >= 0) { const c0 = performance.now(), off = C[IT + 4 * i]; uygula(i, JSON.parse(DEC.decode(U.slice(DATA + off, DATA + off + len)))); if (Z) { Z.yuk += performance.now() - c0; const j = C[IT + 4 * i + 3]; if (j > 0 && j < N) Z.ham[j] += C[IT + 4 * i + 2] / 1000; } } else hesapla(i, true); } /* -2 (hata) ya da -3, ya da süre doldu: bu işçi kendisi hesaplar; hata gerçekse burada da atılır (tek çekirdekteki gibi) */
    if (Z && (P.seqT - Z.m0) + (P.dalgaT - Z.d0) >= PENCERE) { /* pencere doldu: kazanç = yardımcıların işleri × (ana işçinin dalga dışı süresi ÷ yardımcınınki) */
      const dm = P.seqT - Z.m0; let kaz = 0; for (let j = 1; j < N; j++) { const dh = hq[j] - Z.h0[j]; kaz += Z.ham[j] * (dh > 0 && dm > 0 ? dm / dh : 1); }
      P.topYuk += Z.yuk; P.topKaz += kaz; P.ist = null;
      if (Z.yuk > kaz) self.ALAN_PARALEL.boz('kazanç yok: bedel ' + Math.round(Z.yuk) + ' ms, kazanç ' + Math.round(kaz) + ' ms (dalga ' + w + ')'); }
  }
  self.ALAN_PARALEL_KUR = o => { const C = new Int32Array(o.sab, 0, DATA / 4), U = new Uint8Array(o.sab); P = { C, U, n: Math.min(o.n, 8), id: o.id, w: 0, gen: Atomics.load(C, GEN), bozuk: false, seqT: 0, sonT: null, dalgaT: 0, ist: null, topYuk: 0, topKaz: 0 }; self.ALAN_PARALEL = { get n() { return P.n; }, get id() { return P.id; }, dagit, kodla, coz, aktif: () => !!P && P.n > 1 && !P.bozuk, kapali: () => !P || P.bozuk || !!Atomics.load(P.C, BOZUK), boz: neden => { Atomics.store(P.C, BOZUK, 1); P.bozuk = true; P.neden = neden; for (let i = 0; i < 64; i++) Atomics.notify(P.C, i); }, /* paralel kip kalıcı kapanır (bekleyenler uyanır, herkes kendi hesaplar) */ ara: () => { if (P) P.sonT = performance.now(); }, /* işçi bekledi (kare gönderip olay döngüsüne döndü, maça yeniden başladı): dalga dışı süre buradan sayılır, aradaki boşluk sayılmaz (yardımcılar da aynı yerde) */
    durum: () => P ? { n: P.n, w: P.w, gen: P.gen, GEN: Atomics.load(P.C, GEN), bozuk: P.bozuk || !!Atomics.load(P.C, BOZUK), neden: P.neden || null, yuk: Math.round(P.topYuk), kaz: Math.round(P.topKaz), aw: Array.from({ length: P.n }, (_, j) => Atomics.load(P.C, AW + j)), ag: Array.from({ length: P.n }, (_, j) => Atomics.load(P.C, AG + j)) } : null }; };
  /* kuşak: ana işçi maçı yeniden kurarken yeni=true ile çağırır, yeni kuşağı döndürür (bekleyen yardımcılar vazgeçer); yardımcı o iletiyi işlerken iletideki kuşağı (g) alır: paylaşımlı bellekteki kuşak o arada yine artmış olabilir, eski iletinin zaman çizgisi yeni kuşağın dalgalarına karışmaz. İkisi de dalga sayacını sıfırlar */
  self.ALAN_PARALEL_KUSAK = (yeni, g) => { if (!P) return null; const C = P.C; if (yeni) { g = Atomics.add(C, GEN, 1) + 1; for (let i = 0; i < 64; i++) Atomics.notify(C, i); } else if (g == null) g = Atomics.load(C, GEN); /* bozuk bayrağı kalıcı: bir yardımcı öldüyse sonraki maçlar da tek çekirdekte */ P.gen = g; P.w = 0; P.bozuk = !!Atomics.load(C, BOZUK); P.ist = null; P.sonT = null; return g; }; /* yeniden kurmada kazanç penceresi baştan (yeniden oynatmanın başı sayılmaz) */
  self.ALAN_PARALEL_BOYUT = DATA + (32 << 20); /* paylaşımlı belleğin boyu: kontrol alanı + 32 MB veri */
})();
