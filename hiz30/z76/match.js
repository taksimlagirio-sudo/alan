(function () {
  const { W: GW, H: GH, CY: GCY, KX: GKX, KY: GKY } = self.ALAN_SAHA; /* saha ölçüsü tek kaynaktan (core.js); sınıflar: alan-v5/SAHA-SABITLERI.md */
  // Katman 4 (basit) · Yerleşim + maç döngüsü. Talimata sadık, kusursuz değil.
  const C = () => window.AlanCore, Dd = () => window.AlanDecide, W = GW, H = GH;
  const TAC0 = { blok: 'Orta', pres: 1, onde: 0, genislik: 'Normal', risk: .5, tempo: .5, kazaninca: 'Dengeli' };
  /* v5 · Geçiş: bir süre ya da bölge değil, durumdan doğan an (m.gc). Top el değiştirdiğinde kaybeden takım dağınıksa (topun gerisinde Bekçi dışında 4'ten az oyuncu) başlar; kaybeden takım yerleşince (4+) ya da top yeniden el değiştirince biter. Her tik bakılır.
     Hücum geçişi (kazanan): kural tac.gecisH — 'ustun' (önde sayıca üstünsek git) · 'esit' (eşitsek de) · 'her' · 'hic'. Sayı sahadan: topun önündeki bizden (Bekçi ve taşıyıcı hariç) ile topun önündeki rakipten (Bekçi hariç). Git derse katılanlar topun önündekiler + taşıyıcı; top kendi bölgemizde (ilk 35) kazanıldıysa en yakın bir destek. Kalanlar evlerinde güvence. Katılan sayısı her tik yeniden bakılır. Kaç kişi ayarı yok.
     Savunma geçişi (kaybeden): karşı pres kuralı tac.kpYer — 'rakip' (rakip yarısında kaybedersek) · 'her' · 'hic'; süre tac.kpSure (1–3 sn). Basanlar kayıp anında topun 8 birim içindekiler (en fazla 3); kimse yoksa basılmaz. Süre dolunca ya da geçiş bitince döner. Diğerleri savunma planındaki evlerine */
  const ALAN_G = { R: .25, D: .03 }; /* Alan gücü 6 → R 7, D 0,88 · 14 → R 9, D 1,12 (Claude Code ölçümüyle oturur) */
  const GECIS_VARS = tq => tq.gecisH || ({ Kontra: 'esit', 'Yerleş': 'hic' })[tq.kazaninca] || (self.ALAN_GECIS_ESKI ? 'ustun' : 'esit') /* v5 · C3: varsayılan "eşitsek de git" (kullanıcı onayı). Deney: self.ALAN_GECIS_ESKI = true → "sayıca üstünsek" */, KP_VARS = tq => tq.kpYer || ((tq.kayip ?? .5) < .34 ? 'hic' : (tq.kayip ?? .5) > .66 ? 'her' : 'rakip');
  const geriSay = (m, lz, E) => { const ox = ownX(lz); let n = 0; for (const q of m.ps) if (q.team === lz && q.role !== 'Bekçi' && Math.abs(q.x - ox) < Math.abs(E.x - ox)) n++; return n; };
  /* v5 · karşı pres geçişten ayrıldı: kaybeden takım dağınık olmasa da menajerin kuralı geçerli (zip42: yalnız dağınıkken başlıyordu, 8 birim içinde çoğu zaman tek kişi vardı). Basanlar kayıp anında topa en yakın KP_N saha oyuncusu, KP_R birim içinde (≈1 sn koşu) */
  const KP_N = 3, KP_R = 16, gecisEsik = tq => tq.gecisEsik ?? 4;
  function gecisBasla(m, p) { const lz = 1 - p.team, geri = geriSay(m, lz, p); m.gecisGeri = geri; m.gc = null; m.kp = null;
    { const tL = (m.tac0 && m.tac0[lz]) || m.tac[lz] || {}, yer = KP_VARS(tL), rakYar = Math.abs(p.x - ownX(lz)) > GW / 2, kpOn = yer === 'her' || (yer === 'rakip' && rakYar);
      const kpIds = kpOn ? m.ps.filter(q => q.team === lz && q.role !== 'Bekçi' && Math.hypot(q.x - p.x, q.y - p.y) < KP_R).sort((a, b) => Math.hypot(a.x - p.x, a.y - p.y) - Math.hypot(b.x - p.x, b.y - p.y)).slice(0, KP_N).map(q => q.id) : [];
      if (kpIds.length) m.kp = { team: lz, ids: kpIds, bitis: m.tick + Math.round((tL.kpSure ?? 2) * 60) }; }
    const tW = (m.tac0 && m.tac0[p.team]) || m.tac[p.team] || {};
    if (geri < gecisEsik(tW)) { m.gc = { team: p.team, t0: m.tick, kendi: Math.abs(p.x - ownX(p.team)) < 35 * GKX, git: false, on: [] }; gecisGuncelle(m); }
    m.gecis = !!m.gc; }
  function gecisGuncelle(m) { const g = m.gc; if (!g) { m.gecis = false; return; } const h = m.holder, b = m.ball, bt = h ? h.team : b ? b.team : null;
    if (bt !== g.team) { m.gc = null; m.gecis = false; return; } const E = h || b, lz = 1 - g.team, geri = geriSay(m, lz, E); if (geri >= gecisEsik(tq0(m, g.team))) { m.gc = null; m.gecis = false; return; }
    const tq = (m.tac0 && m.tac0[g.team]) || m.tac[g.team] || {}, kural = GECIS_VARS(tq), dir = dirOf(g.team), on = m.ps.filter(q => q.team === g.team && q.role !== 'Bekçi' && q !== h && (q.x - E.x) * dir > 0), n = on.length;
    g.biz = n; g.rakip = geri; g.git = kural === 'her' || (kural === 'esit' && n >= geri) || (kural === 'ustun' && n > geri);
    if (g.git && g.kendi && h) { const d = m.ps.filter(q => q.team === g.team && q.role !== 'Bekçi' && q !== h && !on.includes(q)).sort((u, v) => Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y))[0]; if (d) on.push(d); }
    g.on = g.git ? on.map(q => q.id) : []; m.gecis = true; }
  const tq0 = (m, t) => (m.tac0 && m.tac0[t]) || m.tac[t] || {};
  const inWin = (m, t) => !!(m.gc && m.gc.team === t && m.gc.git);
  function kpAkt(m, t) { const k = m.kp; if (!k) return null; const h = m.holder, b = m.ball, bt = h ? h.team : b ? b.team : null; if (m.tick >= k.bitis || bt === k.team) { m.kp = null; return null; } /* zip43 hatası: çağıranın takımına bakıyordu, topu tutan takım için çağrılınca karşı pres siliniyordu */ return k.team === t ? k.ids : null; } /* top geri kazanılınca ya da süre dolunca biter */
  function kpEff(m, t, r) { return kpAkt(m, t) ? { ...r, _kp: 'pres' } : r; }
  // YZ uyumu (sadece tac.yz): rakip düşük blokta bekliyorsa (son 4 sn'de taşıyıcımıza basan yok, rakibin saha oyuncuları kendi yarısında) ve skor berabere ya da gerideyse tempo bir kademe artar; gerideyken son 30 sn'de sabır da bir kademe düşer. Önde iken dokunmaz. Kullanıcının takımında karar menajerin
  function yzUyum(m, t, r) { if (!(m.tac[t] && m.tac[t].yz)) return r; const d = m.score[t] - m.score[1 - t]; if (d > 0) return r; const h = m.holder;
    if (h && h.team === t) { const og = t === 0 ? GW : 0, geri = m.ps.filter(p => p.team !== t && p.role !== 'Bekçi' && Math.abs(p.x - og) < GW / 2).length, yakin = m.ps.some(p => p.team !== t && p.role !== 'Bekçi' && Math.hypot(p.x - h.x, p.y - h.y) < 5); if (yakin) m._yzBas = m.tick; m._yzBekle = geri >= 5 && m.tick - (m._yzBas || 0) > 240; }
    if (!m._yzBekle) return r; const kal = (m.len || self.ALAN_MAC_LEN) - m.tick; return { ...r, tempo: Math.min(.9, (r.tempo ?? .5) + .4), sabir: Math.max(.1, (r.sabir ?? .5) - (d < 0 ? (kal < 1800 ? .5 : .35) : .2)) }; } /* beklemek işe yaramıyor: daha hızlı çevir, ileri pası daha çok dene (gerideyken ve sona doğru daha çok) */
  function tacEff(m, t) { return yzUyum(m, t, macEff(m, t, kpEff(m, t, tacEff0(m, t)))); }
  // Zaman ve skor farkındalığı: bir sayı atmanın ve yemenin değeri, kalan süre ve skor farkına göre maç puanına (galibiyet 3, beraberlik 1) etkisinden ölçülür.
  // Kalan sayılar Poisson (iki takım için tik başına MAC.oran). g = bir sayı atmanın beklenen puan kazancı, c = bir sayı yemenin beklenen puan kaybı. Kaybın bedeli g'ye göre c kadar tartılır: önde ve sona yakın takım kayıptan çok korkar (risk düşer, blok geri çekilir); geride olan riske girer.
  const MAC = { oran: 1.8 / 5400, uc: [.35, 2.8] };
  function macW(d, kal) { const lam = MAC.oran * Math.max(0, kal), pk = []; let s = 0; for (let k = 0; k < 12; k++) { const p = Math.exp(-lam) * Math.pow(lam, k) / fak(k); pk.push(p); s += p; } pk[11] += Math.max(0, 1 - s);
    const E = dd => { let e = 0; for (let a = 0; a < 12; a++) for (let b = 0; b < 12; b++) { const x = dd + a - b; e += pk[a] * pk[b] * (x > 0 ? 3 : x === 0 ? 1 : 0); } return e; };
    const e0 = E(d), g = E(d + 1) - e0, c = e0 - E(d - 1); return { g, c, e0 }; }
  const fak = k => { let r = 1; for (let i = 2; i <= k; i++) r *= i; return r; };
  function macEff(m, t, tq) { if (MAC.kapali || !m.len) return tq; const kal = m.len - m.tick, d = m.score[t] - m.score[1 - t], key = d + ':' + Math.floor(kal / 120); m._macC = m._macC || {}; let w = m._macC[key]; if (!w) w = m._macC[key] = macW(d, kal);
    const n = macW(0, m.len), oran = Math.max(MAC.uc[0], Math.min(MAC.uc[1], (w.c / Math.max(1e-3, w.g)) / (n.c / Math.max(1e-3, n.g))));
    m.macDurum = m.macDurum || [null, null]; m.macDurum[t] = { oran: +oran.toFixed(2), g: +w.g.toFixed(3), c: +w.c.toFixed(3) }; const birim = Math.max(.05, (w.g + w.c) / 2), eK = k => { const k2 = d + ':' + k; let x = m._macE && m._macE[k2]; if (x == null) { m._macE = m._macE || {}; x = m._macE[k2] = macW(d, k * 60).e0; } return x; }, eT = dt => { const r = Math.max(0, kal - dt) / 60, k = Math.floor(r), f = r - k; return eK(k) * (1 - f) + eK(k + 1) * f; }; /* v5 · kalan süre 60 tiklik kovaya yuvarlanmaz, iki kova arasında doğrusal: 0,5 sn ile 1 sn beklemenin bedeli artık farklı */
    // geçen zamanın değeri: seçenek dt tik sürerse maç puanı beklentisi ne kadar değişir (sayı birimine çevrilmiş). Önde olana zaman lehte, geride olana aleyhte
    const zaman = dt => (eT(dt) - eT(0)) / birim; return { ...tq, macK: oran, macZ: zaman }; } /* menajerin risk ayarı korunur; kaybın bedeli kararda doğrudan bu oranla tartılır */
  function tacEff0(m, t) { const tq = m.tac[t], kon = inWin(m, t), yer = false; /* geçiş oyunu (git): risk +0,25, tempo en az 0,8; "yerleş" geçiş oynamaz, yerleşik plan */ return kon ? { ...tq, risk: Math.min(1, (tq.risk ?? .5) + .25), tempo: Math.max(tq.tempo ?? .5, .8) } : yer ? { ...tq, risk: Math.max(0, (tq.risk ?? .5) - .25), tempo: Math.min(tq.tempo ?? .5, .3) } : tq; }
  const BLOK = { 'Düşük': 22 * GKX, 'Orta': 34 * GKX, 'Yüksek': 50 * GKX }, GEN = { 'Dar': 28 * GKY, 'Normal': 38 * GKY, 'Geniş': 46 * GKY };
  const SLOTS = [{ role: 'Bekçi', d: 0, l: 0 }, { role: 'Bek', d: .2, l: -.5 }, { role: 'Bek', d: .2, l: .5 }, { role: 'Orta', d: .45, l: -.35 }, { role: 'Orta', d: .45, l: .35 }, { role: 'Kanat', d: .75, l: -.85 }, { role: 'Uç', d: .8, l: .25 }];
  const NAMES = [['Aras', 'Deniz', 'Ece', 'Kaan', 'Mert', 'Selin', 'Tuna'], ['Bora', 'Cem', 'Duru', 'Ilgaz', 'Nehir', 'Rüzgâr', 'Yaz']];
  const cl = (v, a, b) => v < a ? a : v > b ? b : v, dirOf = t => t === 0 ? 1 : -1, ownX = t => t === 0 ? 0 : W;
  function rng(seed) { const f = () => { const a = f.s = (f.s + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; f.s = seed >>> 0; return f; } /* durumu f.s'de: maç kopyalanabilir (aynı sayıları üretir) */
  function kopya(o, Mp = new Map()) { if (typeof o === 'function') { if (o.s === undefined) return o; if (Mp.has(o)) return Mp.get(o); const f = rng(0); f.s = o.s; Mp.set(o, f); return f; } if (o === null || typeof o !== 'object') return o; if (Mp.has(o)) return Mp.get(o); if (ArrayBuffer.isView(o)) { const c = o.slice(); Mp.set(o, c); return c; } if (o instanceof Map) { const c = new Map(); Mp.set(o, c); for (const [k, v] of o) c.set(kopya(k, Mp), kopya(v, Mp)); return c; } if (o instanceof Set) { const c = new Set(); Mp.set(o, c); for (const v of o) c.add(kopya(v, Mp)); return c; } const c = Array.isArray(o) ? [] : Object.create(Object.getPrototypeOf(o)); Mp.set(o, c); for (const k of Object.keys(o)) c[k] = kopya(o[k], Mp); return c; } /* maçın tam kopyası (döngüsel bağlar korunur) */
  const spd = p => .17 + (p.a.hiz ?? 10) * .006;
  function createMatch(seed, o) {
    o = o || {}; const m = { r: rng(seed || 1), seed, tick: 0, len: o.len || self.ALAN_MAC_LEN, devre: o.devre ?? true, devre2: false, score: [0, 0], ps: [], ball: null, holder: null, ch: .3, decT: 0, over: false, events: [], fl: null,
      tac: [0, 1].map(t => ({ ...TAC0, ...((o.tac && o.tac[t]) || {}) })), st: { duel: [0, 0], duelW: [0, 0], pass: [0, 0], passOk: [0, 0], shot: [0, 0], steal: [0, 0], poss: [0, 0], bank: [0, 0], lead: [0, 0], loose: [0, 0] } };
    for (const t of [0, 1]) SLOTS.forEach((s, i) => { const a = { hiz: 10, ceviklik: 10, guc: 10, uzanma: 10, alan: 10, kontrol: 10, surme: 10, pas: 10, gonderme: 10, yukleme: 10, tepki: 10, gorus: 10, yeralma: 10, sogukkanlilik: 10, kesme: 10, refleks: 10, yerlesim: 10, hakimiyet: 10, taraf: 'sag', ...((o.attrs && o.attrs[t]) || {}) }; m.ps.push({ id: t * 10 + i, team: t, i, role: s.role, slot: s, name: NAMES[t][i], a, R: s.role === 'Bekçi' ? 7 : 8, D: s.role === 'Bekçi' ? 1.2 : 1, x: GW / 2, y: GCY, vx: 0, vy: 0, tx: GW / 2, ty: GCY, cd: 0 }); });
    if (o.roles) assignRoles(m, o.roles);
    if (o.diz) for (const t of [0, 1]) if (o.diz[t]) dizUygula(m, t, o.diz[t]);
    kickoff(m, 0); return m;
  }
  // Diziliş (v5): savunma ve hücum için ayrı. "2-2-2" gibi satırlar arkadan öne; her satır soldan sağa. Oyuncu i (1–6) her iki dizilişte de aynı sırayla yer alır. v3d'nin yerleşimi zaten hücumda rslot, savunmada slot kullanır.
  function dizSlots(str) { const rows = String(str).split('-').map(Number).filter(n => n > 0), R = rows.length, LS = { 1: [0], 2: [-.5, .5], 3: [-.8, 0, .8], 4: [-.9, -.3, .3, .9], 5: [-.9, -.45, 0, .45, .9] }, out = []; rows.forEach((n, k) => { const d = R > 1 ? .2 + .6 * k / (R - 1) : .45; for (const l of (LS[n] || LS[3])) out.push({ d, l }); }); return out; }
  function dizUygula(m, t, dz) { const sl = v => !v ? null : Array.isArray(v) ? v : dizSlots(v), sv = sl(dz.sav), hu = sl(dz.huc); /* serbest yerleşim: menajer her oyuncuyu sahada istediği yere koyabilir ({ d: 0 kendi Kuyu → 1 rakip, l: −1 … 1 }) */ for (const p of m.ps) { if (p.team !== t || p.role === 'Bekçi') continue; const k = p.i - 1; if (p.slot0 !== undefined) { p.slot = p.slot0; p.rslot = p.rslot0; } if (sv && sv[k]) p.slot = { role: p.slot.role, d: sv[k].d, l: sv[k].l }; if (hu && hu[k]) p.rslot = { d: hu[k].d, l: hu[k].l }; p.slot0 = p.slot; p.rslot0 = p.rslot; } m.diz = m.diz || [null, null]; m.diz[t] = { ...dz }; if (m.evre) m.evre[t] = null; } /* temel diziliş değişince evre şekilleri yeniden uygulanır */
  // Roller: oyuncuya verilen talimat. Neyi, nerede, ne sıklıkla deneyeceğini belirler; başarısı özelliklerden gelir.
  // Hücum: Kurucu (topun gerisinde kalır, oyunu kurar) · Kanat (çizgide, geniş) · Koşucu (son çizginin arkasına koşar) · Pivot (Kuyu önünde, ortada) · Serbest
  // Savunma: Süpürücü (en arkada) · Presçi (ilk basan) · Markajcı (belli bir rakibi takip) · Önde (kontra için ileride bekler) · Bölge
  // v5 · rolsüz sistem (Aşama 1): davranışı oyuncu başına TALİMAT belirler, başarıyı özellik. Talimatlar evreye bağlı (şimdilik 'huc' = top bizde, 'sav' = top rakipte); verilmeyen değer üst seviyeden gelir (oyuncu → takım evresi → varsayılan). Paketler (Kurucu, Markajcı…) yalnız başlangıç önerisi: talimat değerleri üretir, motor paket adına bakmaz.
  const TLV = { huc: { kosu: .5, istek: .5, destek: .5, risk: .5, ritim: .5, tempo: .5, gonder: .5, serbest: .5, doldur: null }, sav: { adam: 0, kimi: null, pres: .5 } };
  const PAKET = { h: { Kurucu: { destek: .2, serbest: 0, kosu: .3 }, Kanat: { serbest: .21 }, Koşucu: { kosu: 1, serbest: .76 }, Pivot: { serbest: .21 }, Serbest: {} }, s: { Süpürücü: {}, Markajcı: { adam: 1 }, Presçi: { pres: 1 }, Önde: {}, Bölge: {} } };
  // v5 · Evreler (yalnız bilgi): top bizde geçiş (m.gc git) · kurma (kendi Kuyu'muza uzaklık < 35) · ilerleme · son; top rakipte geçiş (karşı pres) · kurma · orta · son. Ayar taşımaz.
  const EVRE = { huc: ['gecis', 'kurma', 'ilerleme', 'son'], sav: ['gecis', 'kurma', 'orta', 'son'] }, EVKOD = ['huc.gecis', 'huc.kurma', 'huc.ilerleme', 'huc.son', 'sav.gecis', 'sav.kurma', 'sav.orta', 'sav.son'];
  function evreBul(m, t) { const h = m.holder, bt = h ? h.team : (m.ball ? m.ball.team : null), E = h || m.ball || { x: GW / 2, y: GCY }, dG = Math.abs(E.x - (t === 0 ? 0 : W)), b = (m.tac0 && m.tac0[t]) || m.tac[t] || {};
    if (bt === t) { if (inWin(m, t)) return 'huc.gecis'; return dG < 35 * GKX ? 'huc.kurma' : dG < 65 * GKX ? 'huc.ilerleme' : 'huc.son'; }
    if (kpAkt(m, t)) return 'sav.gecis'; return dG > 65 * GKX ? 'sav.kurma' : dG > 35 * GKX ? 'sav.orta' : 'sav.son'; }
  function evreUygula(m) { m.tac0 = m.tac0 || [null, null]; m.evre = m.evre || [null, null];
    for (const t of [0, 1]) { const cur = m.tac[t]; if (!cur) continue; if (!cur._ev) m.tac0[t] = cur; const b = m.tac0[t] || cur, key = evreBul(m, t), ev = key.slice(0, 3), ad = key.slice(4);
      /* v5 · evreler artık yalnız bilgi (Canlı'daki "ŞU AN"); ayar anahtarı değil. Evre başına ayar, diziliş ve talimat kalktı: yer topa bağlı plandan (tac.durus), geçiş durumdan (m.gc). Eski kayıtlardaki tac.evre yok sayılır */
      if (!m.evre[t] || m.evre[t].key !== key) m.evre[t] = { key, ev, ad, t0: m.tick };
      let mg = b;
      /* v5 · Katman 5 · koşullar: menajerin açık "eğer–ise" kuralları (skor farkı, kalan süre); evre ve takım ayarının üstüne yazar */
      if (b.kosul && b.kosul.length) { const fark = m.score[t] - m.score[1 - t], kal = (m.len - m.tick) / 60, akt = []; for (const k of b.kosul) { const okS = k.ne === 'onde' ? fark > 0 : k.ne === 'geride' ? fark < 0 : k.ne === 'esit' ? fark === 0 : true, okT = k.son == null || kal <= k.son; if (okS && okT && k.ayar) { mg = { ...mg, ...k.ayar, _ev: 'kosul' }; akt.push(k.ne); } } m.kosulAkt = m.kosulAkt || [[], []]; m.kosulAkt[t] = akt; }
      m.tac[t] = mg; } }
  function tl(p, ev, k) { /* talimat: oyuncu (mod geneli) → takım → varsayılan; evre başına talimat kalktı */ const o = p && p.tl && p.tl[ev]; if (o && o[k] != null) return o[k]; const T = p && p.tlT && p.tlT[ev]; if (T && T[k] != null) return T[k]; return TLV[ev] ? TLV[ev][k] : null; }
  function tlKur(r) { const h = r && r.tl && r.tl.huc, s = r && r.tl && r.tl.sav; return { huc: { ...(PAKET.h[r && r.h] || {}), ...(h || {}) }, sav: { ...(PAKET.s[r && r.s] || {}), ...(s || {}) } }; }
  const RSLOT = { 'Kurucu': s => ({ d: .1, l: s.l * .4 }), 'Kanat': s => ({ d: Math.max(s.d, .55), l: s.l < 0 ? -1 : 1 }), 'Koşucu': s => ({ d: .8, l: s.l }), 'Pivot': () => ({ d: .85, l: 0 }) };
  const STYLES = {
    'Hücumcu': { tac: { blok: 'Yüksek', pres: 2, arkada: 1, onde: 0, genislik: 'Geniş', tempo: .8, risk: .8, kazaninca: 'Dengeli' },
      roles: [{ h: 'Serbest', s: 'Bölge' }, { h: 'Kurucu', s: 'Süpürücü' }, { h: 'Koşucu', s: 'Presçi' }, { h: 'Kanat', s: 'Presçi' }, { h: 'Kanat', s: 'Bölge' }, { h: 'Pivot', s: 'Presçi' }] },
    'Savunmacı': { tac: { blok: 'Düşük', pres: 1, arkada: 2, onde: 1, genislik: 'Dar', tempo: .4, risk: .3, kazaninca: 'Kontra' },
      roles: [{ h: 'Kurucu', s: 'Süpürücü' }, { h: 'Serbest', s: 'Süpürücü' }, { h: 'Serbest', s: 'Markajcı' }, { h: 'Serbest', s: 'Bölge' }, { h: 'Koşucu', s: 'Presçi' }, { h: 'Koşucu', s: 'Önde' }] }
  };
  function assignRoles(m, roles) {
    for (const t of [0, 1]) { const R = roles[t]; if (!R) continue; for (const p of m.ps) { if (p.team !== t || p.role === 'Bekçi') continue; const r = R[p.i - 1]; if (!r) continue; p.rh = r.h || 'Serbest'; p.rs = r.s || 'Bölge'; p.tl = tlKur(r); } } /* paket adı yalnız etiket; yer dizilişten (rslot = hücum dizilişi) */
    for (const t of [0, 1]) { const taken = new Set(); for (const p of m.ps.filter(q => q.team === t && q.role !== 'Bekçi' && tl(q, 'sav', 'adam') >= .5)) { const ki = tl(p, 'sav', 'kimi'), sec = ki != null ? m.ps.find(q => q.team !== t && q.i === ki && q.role !== 'Bekçi' && !taken.has(q)) : null, opp = sec ? [sec] : m.ps.filter(q => q.team !== t && q.role !== 'Bekçi' && !taken.has(q)).sort((u, v) => (tl(v, 'huc', 'kosu') - tl(u, 'huc', 'kosu')) || ((v.rslot || v.slot).d - (u.rslot || u.slot).d)); if (opp[0]) { taken.add(opp[0]); p.markRef = opp[0]; } } /* kimi tutar: belirli rakip (talimat) ya da en çok koşu atan / en ileride duran */ }
    m.ra = [0, 1].map(() => ({}));
  }
  // Talimata uyum: her 10 tikte, talimatın geçerli olduğu anlarda, oyuncu talimatın istediği yerde mi?
  function roleSample(m) {
    const h = m.holder; if (!h || m.tick % 10) return; const E = { x: h.x, y: h.y };
    for (const p of m.ps) { if (!p.tl || p === h) continue; const t = p.team, dir = dirOf(t), atk = h.team === t, A = m.ra[t][p.i] || (m.ra[t][p.i] = { h: [0, 0], s: [0, 0] });
      let k = null, ok = false;
      if (atk) { k = 'h';
        if (Math.abs((p.rslot || p.slot).l) >= .75) ok = Math.abs(p.y - GCY) >= 13 * GKY;
        else if (tl(p, 'huc', 'destek') < .35) ok = (p.x - E.x) * dir < -2;
        else if (tl(p, 'huc', 'kosu') >= .75) { const ox = W - ownX(t); let last = null; for (const q of m.ps) if (q.team !== t && q.role !== 'Bekçi' && (last == null || Math.abs(q.x - ox) < Math.abs(last - ox))) last = q.x; ok = last != null && (p.x - last) * dir >= -3; }
        else if ((p.rslot || p.slot).d >= .8 && Math.abs((p.rslot || p.slot).l) < .3) { if ((E.x - GW / 2) * dir <= 0) k = null; else ok = Math.hypot(p.x - (W - ownX(t)), p.y - GCY) < 28 && Math.abs(p.y - GCY) < 12; }
        else k = null;
      } else { k = 's';
        if (p.slot.d <= .25) { let deep = 1e9; for (const q of m.ps) if (q.team === t && q.role !== 'Bekçi') deep = Math.min(deep, Math.abs(q.x - ownX(t))); ok = Math.abs(p.x - ownX(t)) <= deep + 3; }
        else if (tl(p, 'sav', 'pres') >= .75) ok = !!p.press;
        else if (tl(p, 'sav', 'adam') >= .5) ok = p.markRef ? Math.hypot(p.x - p.markRef.x, p.y - p.markRef.y) < 6 : false;
        else if (p.slot.d >= .7) ok = (p.x - GW / 2) * dir > 0;
        else k = null; }
      if (k) { A[k][0]++; if (ok) A[k][1]++; } }
  }
  function ev(m, type, team, text) { const s = Math.floor(m.tick / 60); m.events.unshift({ t: `${String(Math.floor(s / 60)).padStart(2, '0')}:${String(s % 60).padStart(2, '0')}`, type, team, text }); if (m.events.length > 120) m.events.pop(); }
  function kickoff(m, t) {
    for (const p of m.ps) { const tq = (m.tac0 && m.tac0[p.team]) || m.tac[p.team] || {}, huc = p.team === t, s = (huc ? (p.rslot0 || p.rslot) : null) || p.slot0 || p.slot, x0 = ownX(p.team), dir = dirOf(p.team), bl = { 'Düşük': [8, 24], 'Orta': [10, 30], 'Yüksek': [13, 33] }[tq.blok] || [10, 30], d = p.role === 'Bekçi' ? 3 : (huc ? Math.min(46, 12 + s.d * 34) : Math.min(46, bl[0] + s.d * bl[1])) * GKX; p.x = x0 + dir * d; p.y = GCY + s.l * (huc ? 20 : 16) * GKY; /* santra dizilişi sahaya oranlı; Bekçi mutlak */ p.vx = p.vy = 0; p.tx = p.x; p.ty = p.y; } m.winT = null; m.gecis = false; m.gc = null; /* santra: başlayan takım hücum dizilişiyle, savunan takım savunma dizilişiyle ve bloğuna göre derinlikte dizilir */
    const h = m.ps.find(p => p.team === t && p.i === 3); h.x = GW / 2; h.y = GCY; m.holder = h; m.ball = null; m.ch = .3; m.decT = 30; m.fl = null;
  }
  // takımın şekli: savunmada kompakt ve blok yüksekliğinde, hücumda uzun ve genişlik talimatında
  function slotPos(m, p, bx, by, att) {
    const t = p.team, tac = m.tac[t], s = p.slot, x0 = ownX(t), dir = dirOf(t), bd = (bx - x0) * dir;
    if (s.role === 'Bekçi') { const gx = x0, gy = GCY, dx = bx - gx, dy = by - gy, L = Math.hypot(dx, dy) || 1, r = Math.min(6, L * .22); return { x: gx + dx / L * r, y: gy + dy / L * r }; }
    let back, len, wid, sh;
    if (att) { back = cl(bd - 18 * GKX, 12 * GKX, 62 * GKX); len = 48 * GKX; wid = GEN[tac.genislik] || 38 * GKY; sh = .15; }
    else { back = cl(Math.min(BLOK[tac.blok] || 34 * GKX, bd - 4), 9 * GKX, 62 * GKX); len = 24 * GKX; wid = 26 * GKY; sh = .4; }
    if (!att) { const front = bd - 7; if (back + len > front) len = Math.max(6, front - back); }
    let depth = back + (s.d - .2) / .6 * len; if (!att) depth = Math.min(depth, Math.max(back, bd - 7));
    return { x: cl(x0 + dir * depth, 2, (GW - 2)), y: cl(GCY + s.l * wid / 2 + (by - GCY) * sh, 2, (GH - 2)) };
  }
  function laneOpp(m, t, a, b) { let mx = 0; for (let i = 1; i <= 4; i++) { const x = a.x + (b.x - a.x) * i / 5, y = a.y + (b.y - a.y) * i / 5; mx = Math.max(mx, Dd().oppAt(m.ps, t, x, y)); } return mx; }
  // topsuz hücumcu: slotunun yakınında açık, pas hattı görünen, ileri değerli ve arkadaşlardan ayrık nokta
  function offBall(m, p, base, h) {
    const D = Dd(), t = p.team, dir = dirOf(t), ok = self.ALAN_OZ(p, 'yeralma'), cands = [base];
    for (let k = 0; k < 8; k++) { const a = k * Math.PI / 4; for (const r of [5, 9]) cands.push({ x: base.x + Math.cos(a) * r, y: base.y + Math.sin(a) * r }); }
    if (p.slot.d >= .7) { const lastD = m.ps.filter(q => q.team !== t && q.role !== 'Bekçi').reduce((mx, q) => Math.max(mx, (q.x - ownX(t)) * dir), 0); cands.push({ x: ownX(t) + dir * (lastD + 5), y: p.y }, { x: ownX(t) + dir * (lastD + 5), y: base.y }); }
    let best = base, bv = -1e9;
    for (const c of cands) {
      if (c.x < 2 || c.x > (GW - 2) || c.y < 2 || c.y > (GH - 2)) continue;
      const open = Math.exp(-D.oppAt(m.ps, t, c.x, c.y) * 2), lane = h ? Math.exp(-laneOpp(m, t, h, c) * 1.5) : 1, fwd = D.threat(t, c.x, c.y);
      let crowd = 0; for (const q of m.ps) if (q.team === t && q !== p && q.role !== 'Bekçi') crowd += Math.exp(-((q.tx - c.x) ** 2 + (q.ty - c.y) ** 2) / 36);
      const leash = Math.hypot(c.x - base.x, c.y - base.y) / 12, v = .5 * open + .6 * lane + 1.2 * fwd - .5 * crowd - .4 * leash * leash + (m.r() - .5) * (20 - ok) * .02;
      if (v > bv) { bv = v; best = c; }
    }
    return best;
  }
  function position(m) {
    const ps = m.ps, h = m.holder, b = m.ball, bx = h ? h.x : b.x, by = h ? h.y : b.y, attT = h ? h.team : b.team;
    // boşta / uçan Çekirdek: her takımdan en yakın 1-2 kişi kovalar; alıcı koşar
    const chase = new Set();
    if (!h) { const tgt = m.fl && m.fl.end ? m.fl.end : { x: b.x, y: b.y }; for (const t of [0, 1]) { const c = ps.filter(p => p.team === t && p.role !== 'Bekçi').sort((u, v) => Math.hypot(u.x - tgt.x, u.y - tgt.y) - Math.hypot(v.x - tgt.x, v.y - tgt.y)); const n = b.done === 'durdu' || b.defl ? 2 : (t === b.team ? 0 : 1); c.slice(0, n).forEach(p => { chase.add(p); p.tx = tgt.x; p.ty = tgt.y; p.sprint = true; }); } if (m.fl && m.fl.q && !chase.has(m.fl.q)) { chase.add(m.fl.q); const q = m.fl.q; q.tx = m.fl.to.x; q.ty = m.fl.to.y; q.sprint = true; } }
    for (const t of [0, 1]) {
      const att = t === attT, tac = m.tac[t], mine = ps.filter(p => p.team === t);
      let press = new Set(), cover = new Set(); const marked = new Set();
      if (!att) {
        const field = mine.filter(p => p.role !== 'Bekçi').sort((u, v) => u.slot.d - v.slot.d);
        field.slice(0, tac.arkada).forEach(p => cover.add(p));
        if (h) mine.filter(p => p.role !== 'Bekçi' && !cover.has(p)).sort((u, v) => Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y)).slice(0, tac.pres).forEach(p => press.add(p));
      }
      for (const p of mine) {
        if (p === h || chase.has(p)) continue; p.sprint = false; const base = slotPos(m, p, bx, by, att);
        if (p.role === 'Bekçi') { p.tx = base.x; p.ty = base.y; continue; }
        if (att) { if ((m.tick + p.id) % 15 === 0 || p.otx == null) { const c = offBall(m, p, base, h); p.otx = c.x; p.oty = c.y; } p.tx = p.otx; p.ty = p.oty; continue; }
        const gx = ownX(t), opps = ps.filter(q => q.team !== t && q.role !== 'Bekçi');
        if (press.has(p)) { const dx = gx - h.x, dy = GCY - h.y, L = Math.hypot(dx, dy) || 1, dd = press.size > 1 && [...press].indexOf(p) > 0 ? 3.2 : 2.2, side = [...press].indexOf(p) === 1 ? 1 : [...press].indexOf(p) === 2 ? -1 : 0, nx = -dy / L, ny = dx / L; p.tx = h.x + dx / L * dd + nx * side * 2.5; p.ty = h.y + dy / L * dd + ny * side * 2.5; p.sprint = Math.hypot(p.x - p.tx, p.y - p.ty) > 2;
          continue; }
        if (cover.has(p)) { const a = opps.reduce((u, v) => Math.abs(v.x - gx) < Math.abs(u.x - gx) ? v : u), dx = gx - a.x, dy = GCY - a.y, L = Math.hypot(dx, dy) || 1; p.tx = a.x + dx / L * 4; p.ty = a.y + dy / L * 4; if (Math.abs(p.tx - gx) > Math.abs(base.x - gx)) { p.tx = base.x; } continue; }
        // bölge: slotunu tutar, bölgesindeki rakibin Kuyu tarafına geçer, top–Kuyu hattına biraz kayar
        let tx = base.x, ty = base.y; const near = opps.filter(q => q !== h && !marked.has(q) && Math.hypot(q.x - base.x, q.y - base.y) < 9).sort((u, v) => Math.hypot(u.x - base.x, u.y - base.y) - Math.hypot(v.x - base.x, v.y - base.y))[0];
        if (near) { marked.add(near); const gxx = (gx + bx) / 2, gyy = (GCY + by) / 2, dx = gxx - near.x, dy = gyy - near.y, L = Math.hypot(dx, dy) || 1; tx = near.x + dx / L * 3; ty = near.y + dy / L * 3; const off = Math.hypot(tx - base.x, ty - base.y); if (off > 7) { tx = base.x + (tx - base.x) * 7 / off; ty = base.y + (ty - base.y) * 7 / off; } }
        else { const dx = gx - bx, dy = GCY - by, L = Math.hypot(dx, dy) || 1, u = ((tx - bx) * dx + (ty - by) * dy) / L, px = bx + dx / L * u, py = by + dy / L * u; tx += (px - tx) * .3; ty += (py - ty) * .3; }
        p.tx = tx; p.ty = ty;
      }
    }
  }
  // v5 · topla hız Sürme'ye bağlı: Sürme 6 → %80, 10 → %88, 16 → %100 (koşu hızına göre); ikiliden sonraki yavaşlama da Sürme'ye bağlı
  function topHiz(m, p) { const s = (p.a && p.a.surme) ?? 10, k = Math.min(1, .68 + .02 * s), sl = (p.slowT || 0) > m.tick ? Math.max(.4, Math.min(.85, ((Dd().DU && Dd().DU.slowK) || .55) + (s - 10) * .025)) : 1; return k * sl; }
  // Hareket ataleti (v5): gittiği yönde hızlanma eskisi gibi (tik başına farkın %18'i). Fren (ters yön) ve dönüş (yan) daha yavaş ve hız arttıkça daha da yavaş: tam koşuda fren %18 × 0,45, dönüş %18 × 0,55.
  // Çeviklik bu oranları ölçekler: 6 → ×0,8, 10 → ×1, 16 → ×1,3. Hantal oyuncu ani yön değişiminde kayar, çevik oyuncu çabuk döner.
  function ivme(p, wx, wy, vmax) { const sp = Math.hypot(p.vx, p.vy), k0 = .18; if (sp < 1e-3) { p.vx += (wx - p.vx) * k0; p.vy += (wy - p.vy) * k0; return; }
    const ux = p.vx / sp, uy = p.vy / sp, dvx = wx - p.vx, dvy = wy - p.vy, al = dvx * ux + dvy * uy, px = dvx - al * ux, py = dvy - al * uy, r = Math.min(1, sp / Math.max(.05, vmax)), ag = Math.max(.6, Math.min(1.35, 1 + (((p.a && p.a.ceviklik) ?? 10) - 10) * .05));
    const kA = al >= 0 ? k0 : k0 * ag * (1 - .55 * r), kT = k0 * ag * (1 - .45 * r); p.vx += al * ux * kA + px * kT; p.vy += al * uy * kA + py * kT; }
  // Dolanma: presçinin (pres, fake) hedefi taşıyıcının öbür yanındaysa gövdenin içinden geçilemez (katı gövde); presçi Çekirdeğin olduğu yandan, gövdenin etrafından dolaşır.
  // Eskiden arkadan gelen presçi gövdeye takılıp arkada kalıyor, taşıyıcı da arkasında rakip varken rahatça durabiliyordu.
  function dolan(m) { const h = m.holder; if (!h) return; const R0 = 2.4, ca = h.ca != null ? h.ca : 0, bx = Math.cos(ca), by = Math.sin(ca);
    for (const p of m.ps) { if (p.team === h.team || p.role === 'Bekçi' || !/^(pres|fake|kovala)/.test(p.job || '') || p.tx == null) continue; const ax = p.tx - p.x, ay = p.ty - p.y, al = Math.hypot(ax, ay); if (al < .3) continue;
      const fx = h.x - p.x, fy = h.y - p.y, fd = Math.hypot(fx, fy); if (fd > 4) continue; const tx = p.tx - h.x, ty = p.ty - h.y; if (fx * tx + fy * ty <= 0) continue; /* hedef taşıyıcının öbür yanında */
      const px = -fy / (fd || 1), py = fx / (fd || 1), sg = (px * bx + py * by) >= 0 ? 1 : -1; p.tx = h.x + px * sg * R0; p.ty = h.y + py * sg * R0; p.sprint = true; p.job = (p.job || '') + ' · dolanıyor'; }
    // başka bir gövde (perdeci, kendi arkadaşı) yolu kesiyorsa: hedefe en yakın yanından dolaşır
    for (const p of m.ps) { if (p.team === h.team || p.role === 'Bekçi' || !/^(pres|fake|kovala)/.test(p.job || '') || p.tx == null) continue; const ax = p.tx - p.x, ay = p.ty - p.y, L = Math.hypot(ax, ay); if (L < .5) continue; let ob = null, ou = 1; for (const c of m.ps) { if (c === p || c === h) continue; const u = ((c.x - p.x) * ax + (c.y - p.y) * ay) / (L * L); if (u <= 0 || u >= 1 || u >= ou) continue; if (Math.hypot(p.x + ax * u - c.x, p.y + ay * u - c.y) < 1.8) { ob = c; ou = u; } } if (!ob || Math.hypot(ob.x - p.x, ob.y - p.y) > 4) continue;
      const nx = -ay / L, ny = ax / L, sg = ((p.tx - ob.x) * nx + (p.ty - ob.y) * ny) >= 0 ? 1 : -1; p.tx = ob.x + nx * sg * 2.4 + ax / L * .8; p.ty = ob.y + ny * sg * 2.4 + ay / L * .8; p.sprint = true; if (!/dolanıyor/.test(p.job)) p.job = (p.job || '') + ' · dolanıyor'; } }
  // Perde (v5): taşıyıcıya bir presçi gelirken yakındaki bir arkadaş presçinin yoluna, taşıyıcıyla presçi arasına girer. Katı gövde yüzünden presçi dolanmak zorunda kalır, taşıyıcı zaman kazanır.
  // Kim ve ne sıklıkla: presçiye 7 birimden yakın, işi olmayan arkadaşlardan en yakını; ihtimal Okuma'sıyla artar (Okuma 10: %36, 16: %72). Karar presçinin her gelişinde bir kez verilir.
  const PERDE = 1.9;
  // v5 · Katman 4 · ilişkiler (oyuncular arası ortak hareket; yerleşim kuralından sonra, hedefleri düzeltir)
  // Boşluk doldurma: talimat huc.doldur = arkadaşın numarası. O arkadaş yerinden 8 birimden fazla ileri koşmuşsa, bu oyuncu onun yerine geçer (yer = arkadaşın hücum dizilişindeki noktası).
  // Pres tuzağı: takım ayarı tuzak = 'kenar'. Taşıyıcı kenara (ortadan 13+ birim) inince ikinci savunmacı içeriyi kapatır (taşıyıcı ile orta arasında); ilk presçi dışarıdan bastığı için taşıyıcı çizgiye sıkışır.
  function iliski(m) { const h = m.holder, ps = m.ps, SH = window.AlanShape;
    if (h) { const att = h.team, dt = 1 - att, dir = att === 0 ? 1 : -1;
      for (const p of ps) { if (p.team !== att || p === h || p.role === 'Bekçi') continue; const j = tl(p, 'huc', 'doldur'); if (!j) continue; const A = ps.find(q => q.team === att && q.i === j); if (!A || A === h) continue; const s = A.rslot || A.slot, bx = (att === 0 ? 0 : W) + dir * (8 + s.d * 84) * GKX, by = GCY + s.l * 20 * GKY; if ((A.x - bx) * dir > 8 && !/kovala|alıcı|koşu/.test(p.job || '')) { p.tx = bx; p.ty = by; p.sprint = Math.hypot(p.x - bx, p.y - by) > 6; p.job = 'boşluk doldurma · ' + (A.name || j); } }
      const tq = m.tac[dt] || {}; if (tq.tuzak === 'kenar' && Math.abs(h.y - GCY) > 13 * GKY && h.role !== 'Bekçi') { const defs = ps.filter(p => p.team === dt && p.role !== 'Bekçi' && !p.noTouch), pr = defs.filter(p => p.press).sort((u, v) => Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y)); let p2 = pr[1] || defs.filter(p => !p.press && p.slot.d > .25 && Math.hypot(p.x - h.x, p.y - h.y) < 14).sort((u, v) => Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y))[0]; const sg = h.y < GCY ? 1 : -1, own = dt === 0 ? 0 : W;
        if (pr[0]) { pr[0].ty = h.y + sg * .8; } if (p2) { p2.tx = h.x + (own - h.x) * .1; p2.ty = h.y + sg * 3.6; p2.sprint = true; p2.press = true; p2.job = 'pres tuzağı · içi kapat'; } } } }
  function perde(m) { const h = m.holder; if (!h || h.role === 'Bekçi') { m._perde = null; return; } const P0 = m._perde;
    if (P0) { const c = m.ps.find(z => z.id === P0.cid), q = m.ps.find(z => z.id === P0.qid); if (!c || !q || m.holder.team !== c.team || m.tick > P0.until || Math.hypot(q.x - h.x, q.y - h.y) > 9) { m._perde = null; } else { const L = Math.hypot(h.x - q.x, h.y - q.y) || 1; c.tx = q.x + (h.x - q.x) / L * PERDE; c.ty = q.y + (h.y - q.y) / L * PERDE; c.sprint = true; c.job = 'perde'; return; } }
    let q = null, qd = 8; for (const z of m.ps) if (z.team !== h.team && /^(pres|fake|kovala)/.test(z.job || '')) { const dd = Math.hypot(z.x - h.x, z.y - h.y); if (dd < qd && dd > 2.6) { qd = dd; q = z; } } if (!q || (m._perdeQ && m._perdeQ[q.id] > m.tick)) return;
    (m._perdeQ = m._perdeQ || {})[q.id] = m.tick + 60; let c = null, cd = 7; for (const z of m.ps) if (z.team === h.team && z !== h && z.role !== 'Bekçi' && !/alıcı|koşu|kovala/.test(z.job || '')) { const dd = Math.hypot(z.x - q.x, z.y - q.y); if (dd < cd) { cd = dd; c = z; } } if (!c) return;
    const ok = self.ALAN_OZ(c, 'yeralma'); if ((m.r ? m.r() : .5) < Math.max(0, Math.min(.9, (ok - 4) * .06))) m._perde = { cid: c.id, qid: q.id, until: m.tick + 90 }; }
  // Fake (v5): taşıyıcı bir niyet gösterir (Çekirdek o yana geçer, kısa hazırlık); yakındaki her rakip Okuma farkıyla aldanır ya da okur. Aldanan gösterilen topa tepki verir. Fake bitince taşıyıcı yeniden karar verir (çoğu zaman öbür yana).
  // v5 · yanılma (tek mekanik): fake'e aldanan da, ikilide geçilen de aynı şeyi yaşar. Yanlış noktaya bir kez yönelir (hedef sabit), Okuma'sı kadar sonra fark eder ve döner (Çeviklik kısaltır). Bu sürede yönünü değiştiremez; boşa çıkan Çekirdeğe uzanamaz.
  function yanil(m, p, hedef, neden) { const ok = self.ALAN_OZ(p, 'tepki'), cv = ((p.a && p.a.ceviklik) ?? 10) - 10; p.adandi = Math.max(p.adandi || 0, m.tick + Math.max(3, Math.round(3 + Math.max(0, 20 - ok) * .45 - cv * .2))); p.adT = { x: hedef.x, y: hedef.y }; p.adNeden = neden; p.cd = Math.max(p.cd || 0, p.adandi); }
  function fakeBasla(m, h, o) { const D = Dd(), sh = o.show, T = D.fakeT(h), fooled = new Set(), ad = [];
    for (const p of m.ps) if (D.fakeIlgili(p, h, sh) && m.r() < D.fakeP(h, p, sh.k, sh.c)) { fooled.add(p.id); ad.push(p.role === 'Bekçi' ? 'Bekçi yattı' : p.name + ' aldandı'); }
    const tg = {}, ra = {}; for (const p of m.ps) if (fooled.has(p.id)) { tg[p.id] = D.fakeHedef(p, h, sh); yanil(m, p, tg[p.id], 'aldandı'); ra[p.id] = p.adandi; } /* aldanan, gösterilen hedefe bir kez yönelir (hedef sabit, onunla birlikte kaymaz) ve Okuma'sı kadar sonra fark eder: Okuma 10 → 8 tik, 18 → 4 tik */
    m.fake = { team: h.team, hid: h.id, k: sh.k, to: { ...sh.to }, t0: m.tick, until: m.tick + T, react: m.tick + T + 8, fooled, tg, ra }; m.st.fake = m.st.fake || [0, 0]; m.st.fake[h.team]++; if (fooled.size) { m.st.fakeOk = m.st.fakeOk || [0, 0]; m.st.fakeOk[h.team]++; }
    h.fkBlock = true; h.fkCool = m.tick + T + 45; h.driveTo = null; h.tx = h.x; h.ty = h.y; m.decT = T + 1; /* Çekirdek gösterilen yana ışınlanmaz: gövdenin etrafından döner (fakeYonet) */
    ev(m, 'fake', h.team, `${h.name} ${sh.k === 'gönder' ? 'göndermeyi' : sh.k === 'pas' ? 'pası' : 'yana gidişi'} gösterdi${ad.length ? ' · ' + ad.join(', ') : ' · kimse aldanmadı'}`); }
  function fakeYonet(m) { for (const p of m.ps) if (p.fkBlock && m.tick > (p.fkCool || 0)) p.fkBlock = false; const F = m.fake; if (!F) return; const h = m.holder;
    if (m.tick > F.react || !h || h.id !== F.hid && m.tick <= F.until) { m.fake = null; return; } const D = Dd(), hh = m.ps.find(p => p.id === F.hid);
    if (h.id === F.hid && m.tick <= F.until) { h.driveTo = null; const fz = (m.tick - F.t0) / Math.max(1, F.until - F.t0), su = Math.atan2(F.to.y - h.y, F.to.x - h.x);
      /* v5 · fake fiziksel: gövde fake'inde taşıyıcı gösterdiği yana gerçekten ağırlık verir (kısa bir adım); pas ve gönderme fake'inde yerinde durur, Çekirdeği önce geri çeker (vuruş hazırlığı) sonra gösterdiği yöne savurur */
      if (F.k === 'gövde') { if (F.ax == null) { F.ax = h.x + Math.cos(su) * .8; F.ay = h.y + Math.sin(su) * .8; } h.tx = F.ax; h.ty = F.ay; h.sprint = false; } else { h.tx = h.x; h.ty = h.y; }
      const ta = F.k === 'gövde' ? su : fz < .4 ? su + Math.PI : su, c0 = h.ca ?? ta, dd = Math.atan2(Math.sin(ta - c0), Math.cos(ta - c0)), w = C().Turn.omega(h) * 1.6; h.ca = c0 + Math.sign(dd) * Math.min(Math.abs(dd), w); } /* gösterme: Çekirdek gövdenin etrafından o yana döner (Sürme hızında, biraz hızlı) */
    for (const p of m.ps) { if (F.fooled.has(p.id) && m.tick > F.ra[p.id]) F.fooled.delete(p.id); } } /* hareketi yanil yönetir */
  // Katı gövdeler: iç içe giren iki oyuncu yarı yarıya ayrılır, birbirine doğru hızları silinir (3 geçiş: üç kişi üst üste binince tek geçiş yetmez)
  let GX = new Float64Array(64), GY = new Float64Array(64); /* hız: gövde ayırmada konumların düz kopyası */
  function govdeAyir(ps) {
    const BD = C().Turn.TQ.minD, n = ps.length; if (n > GX.length) { GX = new Float64Array(n * 2); GY = new Float64Array(n * 2); } for (let i = 0; i < n; i++) { GX[i] = ps[i].x; GY[i] = ps[i].y; } /* hız: konumlar düz diziden okunur (farklı biçimli oyuncu nesnelerini her çiftte okumak yavaş); kımıldayan oyuncunun yeri hemen diziye geri yazılır, sonuç aynı */
    for (let it = 0; it < 3; it++) { let degdi = false; for (let i = 0; i < n; i++) for (let j = i + 1; j < n; j++) { const dx = GX[j] - GX[i], dy = GY[j] - GY[i]; if (dx >= BD || dx <= -BD || dy >= BD || dy <= -BD) continue; /* hız: x ya da y farkı tek başına gövde mesafesini aşıyorsa uzaklık zaten büyüktür (sonuç aynı) */ const d = Math.hypot(dx, dy); if (d >= BD) continue; degdi = true; const a = ps[i], b = ps[j];
      const ux = d > 1e-4 ? dx / d : 1, uy = d > 1e-4 ? dy / d : 0, ga = 1 + (self.ALAN_OZ(a, 'guc') - 10) * .05, gb = 1 + (self.ALAN_OZ(b, 'guc') - 10) * .05, ka = gb / (ga + gb), kb = 1 - ka, o = BD - d; /* v5 · Güç: çarpışmada güçlü olan daha az itilir (eşitse yarı yarıya, eskisi gibi) */ a.x -= ux * o * ka; a.y -= uy * o * ka; b.x += ux * o * kb; b.y += uy * o * kb; const rv = (b.vx - a.vx) * ux + (b.vy - a.vy) * uy; if (rv < 0) { a.vx += ux * rv * ka; a.vy += uy * rv * ka; b.vx -= ux * rv * kb; b.vy -= uy * rv * kb; }
      for (const p of [a, b]) { p.x = cl(p.x, .8, W - .8); p.y = cl(p.y, .8, H - .8); if (p.role !== 'Bekçi') outOfKuyu(p, true); } GX[i] = a.x; GY[i] = a.y; GX[j] = b.x; GY[j] = b.y; }
      if (!degdi) break; } /* hız: bu turda kimse iç içe değildiyse sonraki turlar da hiçbir şey değiştirmez */
  }
  function move(m) {
    for (const p of m.ps) {
      const dx = p.tx - p.x, dy = p.ty - p.y, L = Math.hypot(dx, dy), v = spd(p) * (p.sprint ? 1.25 : 1) * (p === m.holder ? topHiz(m, p) : 1) , want = L < .3 ? 0 : Math.min(v, L * .25);
      const wx = L ? dx / L * want : 0, wy = L ? dy / L * want : 0; ivme(p, wx, wy, v); p.x = cl(p.x + p.vx, .8, W - .8); p.y = cl(p.y + p.vy, .8, H - .8);
      if (p.role !== 'Bekçi') outOfKuyu(p, true);
    }
    { const b = m.ball, K2 = C(); if (b && !m.holder && !b.done) K2.govdeIt(b, m.ps, p => (p === b.from && (b.t || 0) < 4) || (p.role === 'Bekçi' && Math.hypot(p.x - (p.team === 0 ? 0 : W), p.y - GCY) < K2.KR), BODY_R); }
    // v5 · Çekirdek de katı: taşıyıcının gövdesinin kenarındaki Çekirdeğin içine rakip giremez; dayanır (oradan uzanıp alabilir, ikili bunu tartar)
    { const h = m.holder; if (h && h.ca != null) { const B = C().Q.body, CR = C().Turn.TQ.minD / 2 + .3, cx = h.x + Math.cos(h.ca) * B, cy = h.y + Math.sin(h.ca) * B; for (const q of m.ps) { if (q.team === h.team) continue; const dx = q.x - cx, dy = q.y - cy, dd = Math.hypot(dx, dy); if (dd >= CR) continue; const ux = dd > 1e-3 ? dx / dd : Math.cos(h.ca), uy = dd > 1e-3 ? dy / dd : Math.sin(h.ca); q.x = cx + ux * CR; q.y = cy + uy * CR; const vn = (q.vx || 0) * ux + (q.vy || 0) * uy; if (vn < 0) { q.vx -= vn * ux; q.vy -= vn * uy; } } } }
    // v5 · katı gövdeler: iç içe giren iki oyuncu yarı yarıya ayrılır, birbirine doğru hızları silinir (3 geçiş: üç kişi üst üste binince tek geçiş yetmez)
    govdeAyir(m.ps); /* v5 · katı gövdeler: iç içe giren iki oyuncu yarı yarıya ayrılır (aklın denemesi de aynı fonksiyonu kullanır) */
    { const b = m.ball, K2 = C(); if (b && !m.holder && !b.done) K2.govdeIt(b, m.ps, p => (p === b.from && (b.t || 0) < 4) || (p.role === 'Bekçi' && Math.hypot(p.x - (p.team === 0 ? 0 : W), p.y - GCY) < K2.KR), BODY_R); } /* gövdeler ayrıldıktan sonra bir kez daha */
  }
  // v5 · topa yığılma yok: kendi pasımız havadayken alıcı dışındaki arkadaşlar buluşma noktasından en az 7 birim uzakta kalır
  const AYR = 7; function topaYigilma(m) { const fl = m.fl, b = m.ball; if (m.holder || !b || !fl || !fl.q || b.defl || b.done || fl.kind === 'gönder') return; const t = b.team, P = fl.q, ax = P.tx ?? P.x, ay = P.ty ?? P.y;
    for (const p of m.ps) { if (p.team !== t || p === P || p.role === 'Bekçi' || /alıcı|kovala/.test(p.job || '')) continue; const tx = p.tx ?? p.x, ty = p.ty ?? p.y, dx = tx - ax, dy = ty - ay, d = Math.hypot(dx, dy); if (d >= AYR) continue; const l0 = Math.hypot(p.x - ax, p.y - ay) || 1, ux = d > .1 ? dx / d : (p.x - ax) / l0, uy = d > .1 ? dy / d : (p.y - ay) / l0; p.tx = cl(ax + ux * AYR, 2, (GW - 2)); p.ty = cl(ay + uy * AYR, 2, (GH - 2)); } }
  const GON = { taban: 2, baski: 1.5, hareket: 1, guc: .6 };
  const BEK_SURE = 150; // Bekçinin kendi alanında Çekirdeği tutabileceği süre (tik; 1× hızda 5 sn)
  const BODY_R = 1.0, BODY_E = .35; // Çekirdeğin çarptığı gövde yarıçapı, sekme esnekliği
  // Siyah alan (Kuyu dairesi): Bekçi dışında kimse giremez. Oyuncu dairenin kenarına itilir, içeri doğru hızı silinir (kenarı boyunca kayabilir).
  const KUYU_PAD = .6;
  function outOfKuyu(p, vel) { const R = C().KR + KUYU_PAD; for (const kx of [0, W]) { const dx = p.x - kx, dy = p.y - GCY, d = Math.hypot(dx, dy); if (d < R) { const ux = d > 1e-6 ? dx / d : (kx ? -1 : 1), uy = d > 1e-6 ? dy / d : 0; p.x = kx + ux * R; p.y = GCY + uy * R; if (vel) { const vn = p.vx * ux + p.vy * uy; if (vn < 0) { p.vx -= vn * ux; p.vy -= vn * uy; } } } } }
  function kuyuSafe(x, y) { const R = C().KR + KUYU_PAD + .3; for (const kx of [0, W]) { const dx = x - kx, dy = y - GCY, d = Math.hypot(dx, dy); if (d < R) { const ux = d > 1e-6 ? dx / d : (kx ? -1 : 1), uy = d > 1e-6 ? dy / d : 0; return { x: kx + ux * R, y: GCY + uy * R }; } } return null; }
  // Tutulan Çekirdek taşıyıcının üstünde değil, gövdesinin kenarında durur: hareket ettiği yönde, duruyorsa gitmek istediği yönde (ikilideki "Çekirdeğin tarafı" ile aynı).
  function corePos(m) { const h = m.holder; if (!h) return m.ball ? { x: m.ball.x, y: m.ball.y } : null; const cs = Dd().coreSide(h), r = C().Q.body; return { x: h.x + cs.ux * r, y: h.y + cs.uy * r }; }
  function launch(m, h, o) {
    if (o.kind !== 'gönder') { const tp = o.to || o.q || h; m._pas = { team: h.team, fx: h.x, tx: tp.x, len: Math.hypot(tp.x - h.x, tp.y - h.y), t: m.tick }; } // pres tetikleri için son pas
    const D = Dd(), K = C(); let ek = o.ot || 1; if (o.kind === 'gönder') ek *= D.gonEk(h, m.ps, o.shot); else ek *= D.pasEk(h, m.ps); /* baskı altında pas dağılır */ /* v5 · gönderme isabeti: baskı, hareket ve güç dağıtır; Aktarım küçültür */
    const v = D.applyError(h, o.launch, m.r, ek, o.kind === 'gönder'), t = h.team; /* v5 · göndermenin isabeti Gönderme'den (zip42'de maç Pas'ı kullanıyordu; yalnız akıl Gönderme'yi görüyordu) */ const tk = o.ot ? ' · tek dokunuş' : ''; if (o.ot) { m.st.ot = m.st.ot || [0, 0]; m.st.ot[t]++; if (o.kind !== 'gönder' && o.kind !== 'aşırt' && o.kind !== 'kenardan') ev(m, 'pas', t, `${h.name} tek dokunuşla → ${o.q ? o.q.name : ''} · şarj %${Math.round(m.ch * 100)}`); }
    const cp = corePos(m) || { x: h.x, y: h.y }; m.ball = K.makeBall({ x: cp.x, y: cp.y, vx: v.vx, vy: v.vy, team: t, ch: m.ch, from: h, recv: o.q || null }); if (o.kind === 'aşırt') { m.st.lob = m.st.lob || [0, 0]; m.st.lob[t]++; ev(m, 'pas', t, `${h.name} aşırttı → ${o.q.name}${tk}`); } m.holder = null;
    const pr = K.predict(m.ball, m.ps, null, 600); m.fl = { t0: m.tick, kind: o.kind, q: o.q || null, to: o.to, end: { x: pr.end.x, y: pr.end.y }, shot: o.shot };
    if (o.kind === 'gönder') { m.ball.gon = true; m.st.shot[t]++; m.ball.shotPk = o.det ? o.det.Pk : null; if (!m.lite) { const c = m.st.cal || (m.st.cal = { n: 0, pk: 0, g: 0 }); if (m.ball.shotPk != null) { c.n++; c.pk += m.ball.shotPk; } } ev(m, 'gonder', t, `${h.name} ${o.shot} gönderdi${tk} · şarj %${Math.round(m.ch * 100)}`); }
    else if (o.kind === 'çalım') { m.st.calim = m.st.calim || [0, 0]; m.st.calim[t]++; h.kenarW = { x: o.wp1 ? o.wp1.x : o.wp.x, y: o.wp1 ? o.wp1.y : o.wp.y, sonra: o.wp1 ? { x: o.wp.x, y: o.wp.y } : null, until: m.tick + 60 }; ev(m, 'pas', t, `${h.name} kenardan çalım`); }
    else { m.st.pass[t]++; if (o.kind === 'kenardan') { m.st.bank[t]++; if (o.kendine) { m.st.calim = m.st.calim || [0, 0]; m.st.calim[t]++; ev(m, 'pas', t, `${h.name} kenardan çalım · kendine sektirdi`); } else ev(m, 'pas', t, `${h.name} kenardan sektirdi → ${o.q.name}${tk}`); } if (o.kind === 'önüne') m.st.lead[t]++; if (o.why === 'kombinasyon') { m.st.combo = m.st.combo || [0, 0]; m.st.combo[t]++; } }
  }
  const D0 = () => window.AlanDecide.D && window.AlanDecide.freeAt ? Object.assign({ freeAt: window.AlanDecide.freeAt }, window.AlanDecide.D) : null;
  function take(m, p, how) {
    m._gelenB = m.ball;
    m._inSp = m.ball ? Math.hypot(m.ball.vx, m.ball.vy) : 0;
    const prev = m.ball ? m.ball.team : null, fl = m.fl; p.ca = m.ball ? Math.atan2(m.ball.y - p.y, m.ball.x - p.x) : m.holder ? (hh => { const hc = hh.ca ?? 0; return Math.atan2(hh.y + Math.sin(hc) * C().Q.body - p.y, hh.x + Math.cos(hc) * C().Q.body - p.x); })(m.holder) : (p.ca ?? 0); /* Çekirdek geldiği yanda başlar */ m.holder = p; m.ball = null; m.decT = Math.max(2, Math.round(9 - (self.ALAN_OZ(p, 'tepki')) * .4 + m.r() * 3)); if (m.pre && m.pre.b !== m._gelenB) m.pre = null; if (!self.ALAN_ONCEDEN_KARAR_ESKI && m.pre && m.pre.q === p) { m.pre.t = m.tick; m.decT = 1; } /* B: karar hazır */ m.fl = null; p.driveTo = null; p._lc = null; // karşılamadan sonra karar: Okuma'sı yüksek oyuncu Çekirdek gelmeden bakmıştır, hemen oynar
    if (prev === p.team) { if (fl && fl.kind !== 'gönder') { m.st.passOk[p.team]++;
      // Tek dokunuş: alıcı Çekirdeği durdurmadan yönlendirebilir (pas ya da gönderme). Şarj hiç azalmaz, ama isabet gelen hıza ve Aktarım'a göre düşer.
      // Kontrol ederse şarjın bir kısmı gider (boşta karşılayan daha çok korur). Hangisinin iyi olduğuna oyuncu kendi karar verir.
      const D = Dd(), inSp = m._inSp || 0, chFull = m.ch, chCtrl = D.recvCh(m.ps, p.team, p.x, p.y, chFull, p), ot = D.otFactor(p, inSp);
      let one = null; if (!m._lite && (self.ALAN_OZ(p, 'gorus')) >= D.D.otOk) { const tE = tacEff(m, p.team), r = D.decideOT(p, m.ps, chFull, m.r, tE, ot), rc = D.decide(p, m.ps, chCtrl, m.r, tE); const ctrlBest = rc.opts.length ? rc.opts[0].v : 0; const b0 = rc.best; p._plan = b0 && b0.q && b0.to && b0.det && ['pas', 'önüne', 'aşırt', 'kenardan'].includes(b0.kind) ? { kind: b0.kind, q: b0.q, to: { x: b0.to.x, y: b0.to.y }, P: b0.det.P ?? 0, t: m.tick } : null; if (r && r.v > ctrlBest + (D.D.otMargin + (tl(p, 'huc', 'ritim') - .5) * .06)) one = r; }
      if (one) { m.ch = chFull; one.ot = ot; launch(m, p, one); return; }
      m.ch = chCtrl; } }
    else { m.st.steal[p.team]++; m.ch = .3; m.winT = m.tick; m.winTeam = p.team; gecisBasla(m, p); /* geçiş yalnız kaybeden takım dağınıkken: topun gerisinde 4+ oyuncusu varsa yerleşiktir, geçiş oyunu (kontra / karşı pres) olmaz */ ev(m, 'kesme', p.team, how || `${p.name} Çekirdeği aldı`); }
  }
  function step(m) {
    if (m.over) return; if (m.devre && !m.devre2 && m.tick >= (m.len >> 1)) { m.devre2 = true; ev(m, 'devre', -1, `DEVRE ARASI · ${m.score[0]}–${m.score[1]}`); kickoff(m, 1); } /* v5 · devre arası: maçın yarısında ikinci yarı santrası, bu kez 2. takımdan (ilk santra 1. takımın). Kural adımın başında: devredeki taktik değişikliği (kopya tam devre tikinde) santraya da uyar */
    const K = C(), D = Dd(), ps = m.ps; m.tick++;
    /* v5 · B · top gelmeden karar: pas havadayken alıcı, Çekirdeği karşılayacağı noktada ve anda olacakmış gibi aynı akılla (D.decide) bir sonraki hamlesini seçer. Düşünme süresi alıştaki gecikmeyle aynı (9 − Tepki × 0,4 + rastgele 0–3); karar o kadar tik önce verilir, alışta hazırdır. Alıştan sonra yeniden karar yalnız durum belirgin değiştiyse (en yakın rakip 1,5 birimden çok yaklaştı ya da alıcı karşılama noktasından 4,5 birim saptı: dokunma menzili + Çekirdeğin son tikteki yolu). Kafadaki maçta (_lite) yok. Deney: self.ALAN_ONCEDEN_KARAR_ESKI = true */
    if (!self.ALAN_ONCEDEN_KARAR_ESKI && !m._lite && !m.holder && m.fl && m.fl.q && m.fl.kind !== 'gönder' && m.ball && !m.ball.done && m.ball.team === m.fl.q.team) { const q = m.fl.q, b = m.ball; if (q._rcvB === b && q._rcvX != null && !(m.pre && m.pre.b === b)) { if (q._preB !== b) { q._preB = b; q._preT = Math.max(2, Math.round(9 - self.ALAN_OZ(q, 'tepki') * .4 + m.r() * 3)); } if (q._rcvAbs - m.tick <= q._preT) { const hc = { ...q, x: q._rcvX, y: q._rcvY, tx: q._rcvX, ty: q._rcvY }, ps2 = ps.map(p => p === q ? hc : p); let nd = 1e9; for (const o of ps) if (o.team !== q.team && !o.noTouch) nd = Math.min(nd, Math.hypot(o.x - hc.x, o.y - hc.y)); const ch2 = D.recvCh ? D.recvCh(ps2, q.team, hc.x, hc.y, b.ch ?? m.ch, hc) : m.ch; const r = D.decide(hc, ps2, ch2, m.r, tacEff(m, q.team)); if (r && r.best) { if (r.best.q === hc) r.best.q = q; m.pre = { b, q, r, x: hc.x, y: hc.y, nd }; } } } } if (!m._lite) { D.D._tick = m.tick; D.D._mac = m; } /* v5 · kafa = maç: akıl uçuşu maçın kopyasında oynatır */ for (const p of ps) if (p.role !== 'Bekçi') { const al = self.ALAN_OZ(p, 'alan'); p.R = 8 + (al - 10) * ALAN_G.R; p.D = 1 + (al - 10) * ALAN_G.D; } /* v5 · Alan gücü: oyuncunun alanının genişliği (R) ve gücü (D) */ gecisGuncelle(m); evreUygula(m);
    // Bekçi süresi (kural): Bekçi Çekirdeği kendi alanında (Kuyu dairesi + 1,5) en fazla BEK_SURE tik tutabilir; aşarsa Çekirdek alan sınırında en yakın rakibe geçer
    { const h = m.holder; if (h && h.role === 'Bekçi') { const kg = h.team === 0 ? 0 : W, ic = Math.hypot(h.x - kg, h.y - GCY) < K.KR + 1.5; if (ic) { if (h.bekBas == null) h.bekBas = m.tick; if (m.tick - h.bekBas >= BEK_SURE) { h.bekBas = null; const ux = (W / 2 - kg) / (W / 2), sx = kg + ux * (K.KR + 2.5), sy = GCY; let q = null, qd = 1e9; for (const o of ps) if (o.team !== h.team && o.role !== 'Bekçi') { const dd = Math.hypot(o.x - sx, o.y - sy); if (dd < qd) { qd = dd; q = o; } } if (q) { q.x = sx; q.y = sy; q.vx = q.vy = 0; h.stun = m.tick + 20; ev(m, 'kesme', q.team, `${h.name} süreyi aştı · Çekirdek ${q.name}'e geçti`); take(m, q, `${q.name} serbest başlıyor`); return; } } } else h.bekBas = null; } for (const p of ps) if (p !== m.holder) p.bekBas = null; }
    for (const p of ps) { p.noTouch = p.stun > m.tick || (p.adandi > m.tick && !m.holder); if (p.adandi > m.tick && !p.hamle && p.adT) { p.tx = p.adT.x; p.ty = p.adT.y; p.sprint = true; p.job = p.adNeden || 'adandı · dönemiyor'; } if (p.hamle && (!m.holder || m.holder.id !== p.hamle.hid || m.tick > p.hamle.t0 + 30)) p.hamle = null; }
    if (window.AlanShape) window.AlanShape.position(m); else position(m); iliski(m); topaYigilma(m); perde(m); dolan(m); fakeYonet(m);
    /* v5 · Çekirdeğin yön değişimine tepki: vuruş, duvar ya da sekmeyle yeni bir yol başladığında rakipler bunu ancak tepki süreleri dolunca görür (12 − Okuma × 0,5 tik). O zamana kadar eski hedeflerine (taşıyıcıya, eski yola) gitmeyi sürdürürler; taşıyıcı bu arada arkalarına geçebilir */
    { const b = m.ball; for (const p of ps) { if (p.role === 'Bekçi') continue; const opp = b && !m.holder && p.team !== b.team; if (!opp) { p._kt = { x: p.tx, y: p.ty }; continue; } const tau = Math.max(1, C().Q.tepki0 - self.ALAN_OKC(p, 'tepki') * C().Q.tepkiOk); if ((b.tp ?? b.t ?? 99) < tau && p._kt && !(p.adandi > m.tick)) { p.tx = p._kt.x; p.ty = p._kt.y; if (!/tepki/.test(p.job || '')) p.job = (p.job || '') + ' · tepki'; } else p._kt = { x: p.tx, y: p.ty }; } } for (const p of ps) if (p.adandi > m.tick && !p.hamle && p.adT) { p.tx = p.adT.x; p.ty = p.adT.y; p.sprint = true; p.job = p.adNeden || 'adandı'; } for (const p of ps) if (p.kenarW) { if (m.holder || m.tick > p.kenarW.until) p.kenarW = null; else if (Math.hypot(p.kenarW.x - p.x, p.kenarW.y - p.y) < 1.2) { if (p.kenarW.sonra) { p.kenarW = { ...p.kenarW, x: p.kenarW.sonra.x, y: p.kenarW.sonra.y, sonra: null }; p.tx = p.kenarW.x; p.ty = p.kenarW.y; p.sprint = true; p.job = 'çalım: arkasına'; } else p.kenarW = null; } else { p.tx = p.kenarW.x; p.ty = p.kenarW.y; p.sprint = true; p.job = 'çalım: dolaşıyor'; } } /* kenardan çalım: önce rakibin öbür yanından dolaş, sonra Çekirdeğe */ move(m); if (m.ra) roleSample(m); if (window.AlanShape) { const sw = window.AlanShape.metrics(m).swarm; m.st.swarm = (m.st.swarm || 0) + sw; m.st.swN = (m.st.swN || 0) + 1; }
    const h = m.holder;
    if (h) {
      m.st.poss[h.team]++;
      const pr = D.oppAt(ps, h.team, h.x, h.y), still = cl(1 - Math.hypot(h.vx, h.vy) / ((.17 + ((h.a && h.a.hiz) ?? 10) * .006) * 1.25), 0, 1); /* v5 · kendi en yüksek koşu hızına göre */
      m.ch = Math.min(1, m.ch + D.chargeRate(h, ps, still, m.ch));
      // Topu döndürme (v3f'ten): Çekirdek gövdenin bir yanında durur ve bir yandan öbürüne ancak gövdenin etrafından, Sürme'ye bağlı hızla (Turn.omega) dolaşarak geçer; içinden geçemez.
      // Hedef yan: yürüdüğü yön; dibinde rakip varsa rakipten uzak yan (Okuma'sı kadar önceden kestirerek). Dönüş yönü, yolun yarısı rakibe denk geliyorsa öbür yöne çevrilir.
      { const w0 = a => { while (a > Math.PI) a -= 2 * Math.PI; while (a < -Math.PI) a += 2 * Math.PI; return a; }; let nq = null, nd = 1e9; for (const q of ps) if (q.team !== h.team && q.role !== 'Bekçi') { const dd = Math.hypot(q.x - h.x, q.y - h.y); if (dd < nd) { nd = dd; nq = q; } }
        const sp = Math.hypot(h.vx, h.vy), mv = sp > .04 ? Math.atan2(h.vy, h.vx) : (h.tx != null && Math.hypot(h.tx - h.x, h.ty - h.y) > .5 ? Math.atan2(h.ty - h.y, h.tx - h.x) : Math.atan2(GCY - h.y, (h.team === 0 ? GW : 0) - h.x));
        let ta = mv, pa = null; if (nq && nd < 6) { const la = 2 + (self.ALAN_OZ(h, 'tepki')) * .4, aw = Math.atan2(h.y - (nq.y + (nq.vy || 0) * la), h.x - (nq.x + (nq.vx || 0) * la)); pa = w0(aw + Math.PI); ta = sp > .04 ? mv + cl(w0(aw - mv), -1, 1) * .7 : aw; }
        if (h.ca == null) h.ca = mv; const dd = w0(ta - h.ca); if (Math.abs(dd) > .02) { let dir = Math.sign(dd); if (pa != null) { const m1 = w0(h.ca + dd / 2), m2 = w0(h.ca + (dd - dir * 2 * Math.PI) / 2); if (Math.abs(w0(m1 - pa)) < .9 && Math.abs(w0(m2 - pa)) > Math.abs(w0(m1 - pa))) dir = -dir; } let w = K.Turn.omega(h); const arc = dir === Math.sign(dd) ? Math.abs(dd) : 2 * Math.PI - Math.abs(dd);
          if (h.fiske && m.tick < h.fiske) w = K.Turn.TQ.flickW; else if (nq && nd < 4 && arc > 1.2 && !(h.fiskeCd > m.tick)) { const vq = .17 + ((nq.a && nq.a.hiz) ?? 10) * .006, tq = Math.max(0, nd - K.Q.reach) / (vq * 1.25), tDol = arc / w; if (tq < tDol * ((self.ALAN_OZ(h, 'tepki')) / 10)) { h.fiskeCd = m.tick + 40; if (m.r() < K.Turn.flickMiss(h)) { const a = h.ca + dir * arc * .5; m.ball = K.makeBall({ x: h.x + Math.cos(a) * K.Q.body, y: h.y + Math.sin(a) * K.Q.body, vx: Math.cos(a) * .35, vy: Math.sin(a) * .35, team: h.team, ch: m.ch, from: h, defl: 1 }); m.holder = null; m.fl = null; m.st.loose[h.team]++; ev(m, 'kesme', h.team, `${h.name} fiskeyi kaçırdı`); return; } h.fiske = m.tick + Math.ceil(arc / K.Turn.TQ.flickW); w = K.Turn.TQ.flickW; ev(m, 'pas', h.team, `${h.name} fiske`); } } /* fiske: dolaşmak presçi yetişmeden bitmeyecekse */
          const ca0 = h.ca, ca1 = w0(h.ca + dir * (dir === Math.sign(dd) ? Math.min(w, Math.abs(dd)) : w)), B = K.Q.body, CR = K.Turn.TQ.minD / 2 + .3, cx1 = h.x + Math.cos(ca1) * B, cy1 = h.y + Math.sin(ca1) * B;
          /* v5 · Çekirdek dönerken rakibin gövdesinin içinden geçemez: yoluna giren gövdeye çarpar. Fiskede Çekirdek ona çarpıp seker; yavaş dönüşte gövdeye dayanıp durur (rakip uzanıp alabilir) */
          let carp = null; for (const q of ps) { if (q.team === h.team || q.noTouch) continue; if (Math.hypot(cx1 - q.x, cy1 - q.y) < CR) { carp = q; break; } }
          if (!carp) h.ca = ca1; else if (h.fiske && m.tick < h.fiske) { const cx0 = h.x + Math.cos(ca0) * B, cy0 = h.y + Math.sin(ca0) * B, nx = cx0 - carp.x, ny = cy0 - carp.y, nl = Math.hypot(nx, ny) || 1, tx = -Math.sin(ca0) * dir, ty = Math.cos(ca0) * dir, vn = tx * nx / nl + ty * ny / nl, sp = .32, vx = (tx - 1.6 * Math.min(0, vn) * nx / nl) * sp, vy = (ty - 1.6 * Math.min(0, vn) * ny / nl) * sp;
            h.fiske = 0; h.fiskeCd = m.tick + 30; m.ball = K.makeBall({ x: cx0, y: cy0, vx, vy, team: h.team, ch: m.ch, from: carp, defl: 1 }); m.holder = null; m.fl = null; m.st.loose[h.team]++; ev(m, 'kesme', carp.team, `${h.name}'in fiskesi ${carp.name}'e çarptı`); return; } } }
      // ikili mücadele: yakındaki savunmacı girip girmemeye kendisi karar verir. Tutarsa alır (ya da Çekirdek boşa çıkar), tutmazsa geçilir.
      // v5 · ikili fiziksel bir hamle: savunmacı girmeye karar verince önce yüklenir (kısa hazırlık, Çevikliğe bağlı), sonra vurur. Çekirdek gövdenin öbür yanındaysa uzanamaz, omuz verir (gövde koyma).
      // Vuruş anında sonuç o anki geometriye bağlı: taşıyıcı hazırlık sırasında Çekirdeği çekip menzilden çıkarabilir; çıkardıysa hamle boşa gider.
      for (const q of ps) { const H = q.hamle; if (!H || H.hid !== h.id) continue; const kx = h.x + Math.cos(h.ca ?? 0) * K.Q.body, ky = h.y + Math.sin(h.ca ?? 0) * K.Q.body;
        if (m.tick < H.t0 + H.W) { const ux = (H.k === 'omuz' ? h.x : kx) - q.x, uy = (H.k === 'omuz' ? h.y : ky) - q.y, ul = Math.hypot(ux, uy) || 1; q.tx = q.x + ux / ul * 2; q.ty = q.y + uy / ul * 2; q.sprint = true; if (ul > K.Turn.TQ.minD + .1) { const ek = Math.min(.09, ul - K.Turn.TQ.minD - .1); q.x += ux / ul * ek; q.y += uy / ul * ek; } /* atılma: hamlenin hazırlığında savunmacı ileri atılır */ q.job = H.k === 'omuz' ? 'omuz veriyor' : 'uzanıyor'; continue; }
        q.hamle = null; const yet = H.k === 'omuz' ? Math.hypot(q.x - h.x, q.y - h.y) < K.Turn.TQ.minD + .45 : Math.hypot(q.x - kx, q.y - ky) < K.Q.reach + 1;
        const P = yet ? D.duelP(q, h, ps) * (H.k === 'omuz' ? (.7 + (((self.ALAN_OZ(q, 'guc')) - (self.ALAN_OZ(h, 'guc'))) * .03)) : 1) : 0; m.st.duel[q.team]++; q.son = { k: H.k, t: m.tick, ok: false, x: kx, y: ky };
        if (yet && m.r() < P) { m.st.duelW[q.team]++; q.son.ok = true;
          if (H.k === 'omuz') { const nx = h.x - q.x, ny = h.y - q.y, nl = Math.hypot(nx, ny) || 1; h.x += nx / nl * .7; h.y += ny / nl * .7; h.vx = (h.vx || 0) * .3 + nx / nl * .22; h.vy = (h.vy || 0) * .3 + ny / nl * .22; h.slowT = m.tick + 25; m.ball = K.makeBall({ x: kx, y: ky, vx: (h.vx || 0) * .2 + (q.vx || 0) * .5, vy: (h.vy || 0) * .2 + (q.vy || 0) * .5, team: h.team, ch: m.ch, from: h, defl: 1 }); m.holder = null; m.fl = null; m.st.loose[h.team]++; ev(m, 'kesme', q.team, `${q.name} omuz verdi, ${h.name} Çekirdekten ayrıldı`); return; }
          if (m.r() < .6) { q.ca = Math.atan2(ky - q.y, kx - q.x); take(m, q, `İkili · ${q.name}, ${h.name}'den Çekirdeği aldı`); return; } let a = Math.atan2(ky - q.y, kx - q.x) + (m.r() - .5) * 1.6; { let bm = null, bv = -1e9; for (const z of ps) { if (z.team !== q.team || z === q || z.role === 'Bekçi' || z.noTouch) continue; const dz = Math.hypot(z.x - kx, z.y - ky); if (dz > 12) continue; let nd = 1e9; for (const o of ps) if (o.team === h.team) nd = Math.min(nd, Math.hypot(o.x - z.x, o.y - z.y)); const v = nd - dz * .4; if (v > bv) { bv = v; bm = z; } } if (bm) { const ta = Math.atan2(bm.y - ky, bm.x - kx), ok2 = Math.max(0, Math.min(1, ((self.ALAN_OZ(q, 'kesme')) + (q.a.kesme ?? 10)) / 40)); let df = ta - a; while (df > Math.PI) df -= 2 * Math.PI; while (df < -Math.PI) df += 2 * Math.PI; a += df * ok2 * .8; } } /* dürtme yönü: savunmacı topu yakındaki boş arkadaşına doğru dürtmeye çalışır (Okuma + Kesme ne kadar isabetli); rastgele yerine */ m.ball = K.makeBall({ x: kx + Math.cos(a) * .3, y: ky + Math.sin(a) * .3, vx: Math.cos(a) * .5, vy: Math.sin(a) * .5, team: h.team, ch: m.ch, from: q, defl: 1 }); m.holder = null; m.fl = null; m.st.loose[h.team]++; ev(m, 'kesme', q.team, `${q.name} uzanıp dürttü, Çekirdek boşta`); return; }
        { const dx = (H.k === 'omuz' ? h.x : kx) - q.x, dy = (H.k === 'omuz' ? h.y : ky) - q.y, dl = Math.hypot(dx, dy) || 1; yanil(m, q, { x: q.x + dx / dl * 2.6, y: q.y + dy / dl * 2.6 }, 'geçildi'); } /* geçilen, fake'e aldanan gibi atıldığı yöne gider ve Okuma'sı kadar sonra döner */ ev(m, 'pas', h.team, yet ? `${h.name}, ${q.name}'i geçti` : `${h.name} Çekirdeği çekti, ${q.name} boşa ${H.k === 'omuz' ? 'yüklendi' : 'uzandı'}`); }
      for (const q of ps) {
        if (q.team === h.team || q.noTouch || q.cd > m.tick || q.hamle) continue; if (Math.hypot(q.x - h.x, q.y - h.y) > D.DU.engage + .6) continue; /* v5 · Bekçi de ikiliye girer; hamlenin hazırlığı mesafeyi kapatır */
        const P = D.duelP(q, h, ps), est = P + (m.r() - .5) * (20 - (self.ALAN_OZ(q, 'gorus'))) * .03; q.cd = m.tick + D.DU.cd;
        const tq = m.tac[q.team] || {}, sdq = Math.min(1, (tq.siddet ?? .5) + (tq.yz && (macEff(m, q.team, {}).macK ?? 1) < .6 ? .25 : 0)); /* YZ yenikken şiddet +0,25 ikiliye de (pres kararıyla aynı) */ if (est < D.commitThr(q, h, ps) - (sdq - .5) * .2) continue; /* pres şiddeti: sert takım ikiliye daha kolay girer (kazanma ihtimali daha düşükken bile), temkinli takım emin olmadan girmez */
        const kx = h.x + Math.cos(h.ca ?? 0) * K.Q.body, ky = h.y + Math.sin(h.ca ?? 0) * K.Q.body, shd = K.Turn.shielded(h, q, kx, ky), cv = ((q.a.ceviklik ?? 10) - 10) * .25;
        q.hamle = { k: shd ? 'omuz' : 'uzan', hid: h.id, t0: m.tick, W: Math.max(2, Math.round((shd ? 4 : 5) - cv)) }; q.cd = m.tick + q.hamle.W + D.DU.cd;
        /* v5 · kendini adama: hamleye giren savunmacı hazırlık + toparlanma süresince yönünü değiştiremez (Çeviklik kısaltır); o sürede boşa çıkan Çekirdeğe uzanamaz. Taşıyıcı hamleyi Okuma farkıyla sezerse hemen karar verir (kenar çalımı, fiske, pas ile cevap) */
        q.adandi = m.tick + q.hamle.W + Math.max(3, Math.round(8 - cv)); q.adT = { x: q.x + (kx - q.x) * 1.6, y: q.y + (ky - q.y) * 1.6 }; const okH = self.ALAN_OZ(h, 'tepki'), okQ = self.ALAN_OZ(q, 'tepki'); if (m.r() < Math.max(.1, Math.min(.9, .35 + (okH - okQ) * .04 + (okH - 10) * .02))) { m.decT = Math.min(m.decT, 1); h.okudu = q.id; }
      }
      if (h.holdD) { let nd = 1e9; for (const q of ps) if (q.team !== h.team && !q.noTouch) nd = Math.min(nd, Math.hypot(q.x - h.x, q.y - h.y)); if (nd < 3.5) { m.decT = Math.min(m.decT, 1); h.holdD = false; } } /* bekleme planı rakip dibine girince bozulur */
      if (--m.decT <= 0) { h.holdD = false;
        const tE = tacEff(m, h.team), pr = m.pre; let r = null; if (pr && pr.q === h && pr.t != null && m.tick - pr.t <= 3) { let nd = 1e9; for (const o of ps) if (o.team !== h.team && !o.noTouch) nd = Math.min(nd, Math.hypot(o.x - h.x, o.y - h.y)); if (nd >= pr.nd - 1.5 && Math.hypot(h.x - pr.x, h.y - pr.y) < 4.5) { r = pr.r; m.st.onceden = m.st.onceden || [0, 0]; m.st.onceden[h.team]++; } } m.pre = null; /* B: önceden verilen karar, durum değişmediyse */ if (!r) r = D.decide(h, ps, m.ch, m.r, tE); let o = r.best;
        // Karşılamadan önce görülen pas: alıcı Çekirdek gelmeden bakmıştı (plan). Kontrol süresi geçince, aynı hedef hâlâ makulse (tutma ihtimali en fazla 0,15 düşmüşse) onu oynar; kontrol sırasında yeniden fikir değiştirmez.
        const pl = h._plan; h._plan = null; if (pl && m.tick - pl.t < 30) { const same = r.opts.find(x => x.kind === pl.kind && x.q === pl.q && x.to && Math.hypot(x.to.x - pl.to.x, x.to.y - pl.to.y) < 4); if (same && same.det && same.det.P >= pl.P - .15 && same.v > 0) o = same; } m.lastDec = { name: h.name, team: h.team, x: h.x, y: h.y, opts: r.opts.slice(0, 4).map(o => ({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), to: o.to, q: o.q ? o.q.name : '', P: o.det ? (o.kind === 'gönder' ? o.det.Pk : o.det.P) : null, v: o.v , hw: o.hw, bulus: o.bulus})) }; m.decT = Math.round(5 + (1 - m.tac[h.team].tempo) * 8 + m.r() * 3); // tutarken ve sürerken düşünmeye devam eder (tempo sıklığı belirler)
        if (o.kind === 'sür') { h.driveTo = { x: o.to.x, y: o.to.y }; h.tx = o.to.x; h.ty = o.to.y; h.drive = true; }
        else if (o.kind === 'fake') fakeBasla(m, h, o);
        else if (o.kind === 'tut') { h.driveTo = null; h.tx = h.x; h.ty = h.y; if (o.hold) { m.decT = o.hold; h.holdD = true; } if (o.why === 'çek' && h.lastWhy !== 'çek') ev(m, 'pas', h.team, `${h.name} rakibi üstüne çekiyor${o.det.lookTo ? ' · ' + o.det.lookTo.name + ' açılacak' : ''}`); }
        else launch(m, h, o);
        h.lastWhy = o.why || o.kind;
      }
      // sürüş kararı bir sonraki karara kadar sürer (eskiden tek tik sürüp taşıyıcı duruyordu)
      if (h === m.holder && !h.drive) { if (h.driveTo && Math.hypot(h.driveTo.x - h.x, h.driveTo.y - h.y) > .8) { h.tx = h.driveTo.x; h.ty = h.driveTo.y; } else { h.driveTo = null; h.tx = h.x; h.ty = h.y; } }
      return;
    }
    const b = m.ball;
    if (b.done === 'durdu') { let best = null, bd = 1e9; for (const p of ps) { const d = Math.hypot(p.x - b.x, p.y - b.y); if (d < bd) { bd = d; best = p; } } if (bd < K.Q.reach && m.r() < K.ctrlP(b, best, ps)) take(m, best, `${best.name} boştaki Çekirdeği aldı`); }
    else {
      const bx0 = b.x, by0 = b.y, r = K.stepAll(b, ps, null, null, m.r); if ((b.direk || 0) !== (b._dk0 || 0)) { b._dk0 = b.direk; const st = m.st.direk || (m.st.direk = [0, 0]); st[b.team]++; ev(m, 'direk', b.team, `${b.from ? b.from.name + ' · ' : ''}Direkten sekti`); } /* v5 · direk çarpması maç olayı (sayım kayıttan kesin) */
      // v5 · Çekirdek bir gövdenin içinden geçemez: tutamadığı (ya da tutmaya çalışmadığı) gövdeden seker
      if (!(r && r.took) && !b.done) K.govde(b, bx0, by0, ps, p => p === b.from && (b.t || 0) < 4, BODY_R, BODY_E); /* Bekçi de dahil: tutamadığı (sersem, aldanmış, hazırlıksız) Çekirdek onun gövdesinden seker, içinden geçmez */ /* süpürmeli: hızlı Çekirdek de gövdeyi delip geçemez */
      if (r && r.took) { const fl = m.fl; take(m, r.p, fl && fl.kind === 'gönder' && r.p.role === 'Bekçi' ? `${r.p.name} göndermeyi tuttu` : `${r.p.name} aktarımı kesti`); return; }
      if (r && !r.took && m.fl) { m.fl.end = { x: b.x, y: b.y }; }
      if (b.done && String(b.done).startsWith('kuyu')) { const sc = b.done === 'kuyu-sag' ? 0 : 1; m.score[sc]++; if (b.shotPk != null && m.st.cal) m.st.cal.g++; ev(m, 'sayi', sc, `SAYI · ${b.from ? b.from.name : ''} · ${m.score[0]}–${m.score[1]}`); kickoff(m, 1 - sc); }
      else if (b.done === 'durdu' && m.fl) { m.fl.end = { x: b.x, y: b.y }; }
    }
    for (const p of ps) p.drive = false;
    if (m.tick >= m.len) m.over = true;
  }
  function run(m) { while (!m.over) step(m); return m; }
  // Maç notu (v5): her an durumun değeri (sıradaki sayının bizim olma beklentisi, akıldaki V) Mavi gözünden izlenir. Değerin her tikteki değişimi o anki "faile" yazılır: Çekirdek ayaktaysa taşıyıcıya, havadaysa son dokunana.
  // Çekirdek takım değiştirirse değişimin yarısı kaybedene (eksi), yarısı kazanana (artı) yazılır. Sayı: göndereni +(1 − önceki değer), yiyen takımın Bekçi'si yarısını eksi alır. Not = 6,3 + (puan − 0,13) × 3, 3 ile 10 arasında.
  const NOT_K = 3, NOT_0 = .13; /* ölçüm (4 maç): puan medyanı ~0,13, %90'ı ~0,64 → not medyanı 6,3, iyi maç ~8, en iyi ~9,7 */
  function notTik(m) { if (m._ic || m._lite) return; const D = Dd(), N = m.not || (m.not = { p: {}, s: null, actor: null, team: null }); const h = m.holder, cp = corePos(m); let s = N.s, team = h ? h.team : (m.ball ? m.ball.team : N.team);
    if (cp && team != null) { const v = D.V(m.ps, team, cp.x, cp.y, m.ch || 0); s = team === 0 ? v : -v; }
    const add = (p, x) => { if (!p) return; N.p[p.id] = (N.p[p.id] || 0) + x; };
    const actor = h || (m.ball && m.ball.from) || N.actor;
    if (N.s != null && s != null) { const d = s - N.s;
      if (N.team != null && team != null && team !== N.team && h) { add(N.actor, (N.actor && N.actor.team === 0 ? 1 : -1) * d * .5); add(h, (h.team === 0 ? 1 : -1) * d * .5); }
      else if (actor) add(actor, (actor.team === 0 ? 1 : -1) * d); }
    N.s = s; N.team = team; if (actor) N.actor = actor; }
  function notSayi(m, t, atan) { const N = m.not; if (!N) return; const pre = N.s == null ? 0 : (t === 0 ? N.s : -N.s); if (atan) N.p[atan.id] = (N.p[atan.id] || 0) + (1 - pre); const bk = m.ps.find(p => p.team !== t && p.role === 'Bekçi'); if (bk) N.p[bk.id] = (N.p[bk.id] || 0) - .5 * (1 - pre); N.s = null; N.actor = null; }
  function notlar(m) { const N = m.not || { p: {} }; return m.ps.map(p => ({ id: p.id, ad: p.name, team: p.team, rol: p.role, puan: +(N.p[p.id] || 0).toFixed(3), not: +Math.max(3, Math.min(10, 6.3 + ((N.p[p.id] || 0) - NOT_0) * NOT_K)).toFixed(1) })); }
  const _step = step; step = function (m) { const s0 = m.score[0], s1 = m.score[1], son = m.not && m.not.actor; _step(m); if (m.score[0] > s0) notSayi(m, 0, son && son.team === 0 ? son : null); else if (m.score[1] > s1) notSayi(m, 1, son && son.team === 1 ? son : null); else notTik(m); };
  window.AlanMatch = { GECIS_VARS, KP_VARS, kpAkt, inWin, gecisGuncelle, kopya, EVRE, EVKOD, evreBul, evreUygula, tl, TLV, PAKET, tlKur,  macEff, notlar, NOT_K, govdeAyir, ivme, spd, tacEff, dizUygula, dizSlots, corePos, outOfKuyu, kuyuSafe, STYLES, assignRoles, inWin, TAC0, BLOK, GEN, createMatch, step, run, position, move, slotPos };
})();
