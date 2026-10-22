// Bekçi'nin uzanma payı (dalış) sadece kendi Kuyu'suna bu kadar yakınken geçerli; dışarıda herkes gibi uzanır. 0: her yerde.
/* Saha ölçüsü: tek kaynak. Varsayılan 120 × 60 (zip47, kullanıcı kararı). Deney için core.js'ten önce self.ALAN_SAHA_BOY = [100, 50] verilir. Geometri sabitleri KX = W / 100, KY = H / 50 ile ölçeklenir; beden ve fizik sabitleri mutlak kalır (alan-v5/SAHA-SABITLERI.md). 100 × 50'de KX = KY = 1: bütün çarpımlar tam, oyun birebir aynı */
self.ALAN_SAHA = (() => { const b = self.ALAN_SAHA_BOY || [120, 60], W = b[0], H = b[1]; return { W, H, CY: H / 2, KX: W / 100, KY: H / 50 }; })();
self.ALAN_BEK_ALAN = 12;
// Arkadaşın kontrol edemediği Çekirdek: true → gelişi yönünde kaçar (sekmez); false → eski hâl (gövdeden seker).
self.ALAN_KACIR = true;
// Okuma kanalları (sadece ölçüm için): ALAN_OKX = { eq: [kanallar], only: kanal, mid } verilirse, o kanallarda herkes mid Okuma'yla oynar. Verilmezse etkisiz.
/* v5 · özellik standardı: her özellik tek iş. Eski kayıtlar (okuma, aktarim, tutus, yogunluk) eşlemeyle okunur. Bağlama tablosu: alan-v5/OZELLIKLER.md */
self.ALAN_OZ_ESKI = { tepki: 'okuma', gorus: 'okuma', yeralma: 'okuma', pas: 'aktarim', gonderme: 'aktarim', kontrol: 'tutus', yukleme: 'tutus', guc: 'yogunluk', uzanma: 'yogunluk', refleks: 'okuma', yerlesim: 'okuma', hakimiyet: 'tutus' };
self.ALAN_OZ = function (p, k) { const a = p && p.a; if (!a) return 10; const v = a[k]; if (v != null) return v; const e = self.ALAN_OZ_ESKI[k], w = e && a[e]; return w != null ? w : 10; };
self.ALAN_OKC_K = { tepki: 'tepki', bekci: 'refleks', kontrol: 'gorus' }; /* eski kanal adı → özellik (ölçüm deneyleri ALAN_OKX ile sürer) */
self.ALAN_OKC = function (p, ch) { const v = self.ALAN_OZ(p, self.ALAN_OKC_K[ch] || ch), E = self.ALAN_OKX; if (!E) return v; if (E.only) return E.only === ch ? v : E.mid; return E.eq && E.eq.includes(ch) ? E.mid : v; };
self.ALAN_OKY = function (p, ch, mek) { const v = self.ALAN_Y(p, mek), E = self.ALAN_OKX; if (!E) return v; if (E.only) return E.only === ch ? v : E.mid; return E.eq && E.eq.includes(ch) ? E.mid : v; }; /* v5 · yetenek: kanalın bileşik değeri (ALAN_Y), ölçüm deneyi ALAN_OKX'e ALAN_OKC gibi uyar */
(function () {
  const { W: GW, H: GH, CY: GCY } = self.ALAN_SAHA;
  // Math.hypot ile bit bit aynı sonucu veren iki argümanlı sürüm (V8'in kendi algoritması: en büyüğe bölüp karelerin toplamı); yerleşik çağrıdan ~5 kat hızlı.
  const hyp = (x, y) => { x = x < 0 ? -x : x; y = y < 0 ? -y : y; if (x === Infinity || y === Infinity) return Infinity; const m = x > y ? x : y; if (m !== m || x !== x || y !== y) return NaN; if (m === 0) return 0; const r = (m === x ? y : x) / m; return Math.sqrt(1 + r * r) * m; }; /* hız: büyük olan kendisine bölününce tam 1 olur (1·1 = 1); toplam ve sonuç aynı, bir bölme ve bir çarpma eksik */
  self.ALAN_EN = { alan: 1 / 50, alanMin: .12, sprint: 1 / 150, ivme: .1, ikili: .08, dol: 1 / 90, dolEsik: 1, hizAlt: .82, hizEsik: .3, sprEsik: .2, ikiliMin: .1, santra: .3, sakinUzak: 10, sakinEn: .5, surme: 1 / 500, fake: .03, fiske: .02, kalkan: .0015, dolas: 1 / 400, dolT: 1 / 400, sprBos: .6, sprPres: 1.3, fizK: 1.5 }; self.ALAN_SPR_IS = (p, h) => { if (self.ALAN_EN_ISYUK_ESKI) return 1; if (p === h) return 1; const j = p.job || ''; return /^(pres|kapatma|ikisini kapat|kenara itiyor|omuz|uzanıyor|oyala)/.test(j) ? self.ALAN_EN.sprPres : self.ALAN_EN.sprBos; }; /* v5 · işe göre sprint bedeli (kullanıcı kararı, 20 Ekim 2026): topsuz koşu (kontra, arkaya, geri koşu, boşa çıkma, topa yarış) sprintin %60'ı, pres ve ikiliye hazırlık %130'u; taşıyıcı tam. Fiziksel mücadele (ikiliye girmek, gövde teması) fizK kat. Boş sprint pahalı olunca kontralar zayıflıyordu. Deney: self.ALAN_EN_ISYUK_ESKI */ /* dolT: temel hızda (tempoda) koşarken dolum, tik başına (boş bar ~13 sn; dururken dol ile ~3 sn) */ /* v5 · harcama çeşitleri: topla sürme (temel hızda tik başına), fake ve fiske (bir kez), gövde teması (taşıyıcıyla temasın her tiki, iki taraf), baskı altında Çekirdeği gövde etrafında döndürmek (tam hızla dönüşte tik başına) */ /* v5 · enerji barı (tasarım: TASARIM-enerji-ve-alan.md): tam sprint 150 tikte (5,5 sn) bitirir (yalnız temel hızın üstü harcar); ivmelenme (hız değişimi × ivme) ve ikiliye girmek ayrıca harcar; temel hızın dolEsik katından yavaşken, yavaşladığı oranda dolar (dururken boş bar 90 tikte, 3,3 sn); sıradan yer değiştirmede (acil olmayan iş) oyuncu ancak hedefine sakinUzak'tan uzaksa ve barı sakinEn'den doluysa sprint atar; bar hizEsik'in altında en çok %18 yavaşlar, sprEsik'in altında sprint gücü azalır, ikiliMin'in altında ikiliye girilmez; her santrada santra kadar toparlanır */
  self.ALAN_EN_S = p => .7 + .03 * self.ALAN_OZ(p, 'enerji'); /* Enerji özelliği: barın büyüklüğü ve dolma hızı (10 → 1, 20 → 1,3) */
  self.ALAN_SPR = p => self.ALAN_SPRINT_HIZ_ESKI ? 1.25 : Math.max(1.05, Math.min(1.45, 1.25 + ((((p && p.a && p.a.hiz) ?? 10)) - 10) * .02)); /* v5 · sprintin temel hıza katı Hız'la büyür: 10 → 1,25 (eskisi gibi), 20 → 1,45, 0 → 1,05. Önce herkese 1,25: Hız 20 oyuncunun sprinti Hız 10'unkinden yalnız %26 hızlıydı (şimdi %47). Bütün koşu hesapları (maç, yerleşim, kafa) bundan. Deney: self.ALAN_SPRINT_HIZ_ESKI */
  self.ALAN_HZ = p => { const h = (p && p.a && p.a.hiz) ?? 10; if (self.ALAN_ENERJI_ESKI || !p || p.en == null || p.en >= self.ALAN_EN.hizEsik) return h; const E = self.ALAN_EN, f = E.hizAlt + (1 - E.hizAlt) * Math.max(0, p.en) / E.hizEsik; return ((.17 + h * .006) * f - .17) / .006; }; /* v5 · enerjiye göre etkin Hız: bütün koşu hızı hesapları (maç, yerleşim, kafa) bundan; barı düşük oyuncu yavaşlar. Deney: self.ALAN_ENERJI_ESKI */
  self.ALAN_TPS = self.ALAN_TPS || 30; /* v5 · bir maç saniyesi kaç tik (1× oynatma hızı): 30 (19 Ekim 2026'da bir süre 27 denendi, menajer eski hıza döndürdü). Saat, olay zamanları, saniyeli talimatlar (karşı pres, koşullar) ve sitedeki oynatma bundan. Deney: core.js'ten önce self.ALAN_TPS = 27 */
  self.ALAN_MAC_LEN = self.ALAN_MAC_LEN || 300 * self.ALAN_TPS; /* v5 · maç süresi tek kaynak: 300 maç saniyesi (5 dk; 30 tik/sn ile 9000 tik), devreler 150 + 150 sn (19 Ekim 2026; önce 5400 = 180 sn). Deney: core.js'ten önce self.ALAN_MAC_LEN = 9000 */
  const W = GW, H = GH, KR = 9, MOUTH = 5, DIREK = self.ALAN_DIREK ?? .3 /* v5 · ağzın uçlarındaki direk yarıçapı (mutlak). Deney: core.js'ten önce self.ALAN_DIREK = .6; 0 = direksiz (zip48 ile birebir) */, GX1 = W * 2 + 1, GY1 = H * 2 + 1, GW1 = W + 1, GH1 = H + 1, GXM = W === 100 ? 99.999 : W - .001, GYM = H === 50 ? 49.999 : H - .001; /* ızgaralar saha ölçüsünden (100 × 50: 201 × 101 yarım birim, 101 × 51 tam birim) */ /* Kuyu alanı yarıçapı 9 (Bekçi dışında kimse giremez), ağız 10 birim */
  // Çekirdek fiziği parametreleri. Hepsi deneme sahasından ayarlanabilir.
  const P = {
    fric: .010,     // temel sürtünme (tik başına hız oranı)
    roll: .003,     // sabit yuvarlanma yavaşlaması (birim/tik); Çekirdeğin gerçekten durmasını sağlar
    oppDrag: .07,  // rakip alanında ek sürtünme (alan gücüyle çarpılır)
    pierce: 1,      // şarjın rakip direncini ne kadar azalttığı (1 = tam şarj direnci sıfırlar)
    chgV: .9,       // şarjın başlangıç hızına katkısı: v0 × (1 - chgV/2 + chgV × şarj)
    ownFlow: .55,    // kendi alanının sürtünmeyi ne kadar azalttığı
    attn: .003,     // yolda enerji kaybı (birim başına)
    drain: .09,     // rakip alanında enerji kaybı (birim × alan gücü)
    deadRamp: .004, // sönünce her tik eklenen sürtünme (yumuşak fren)
    deadMax: .06,   // sönme freninin tavanı
    rest: .72,      // kenar sekmesinde kalan hız oranı
    wallLoss: .08,  // kenara çarpınca kaybedilen enerji
    e0: 1,
    tekS: .35,      // v5 · tek enerji: uçuştaki kayıplar şarj biriminde (eski e0 = 1 ölçeğinin %35i; önce .5: toplar sık sönüyor, şarj eriyordu)
    tekGain: .025,  // v5 · tek enerji: takım arkadaşının hızlandıran alanından geçerken şarj kazancı (birim × alan gücü)
    tekDrain: .6,   // v5 · tek enerji: rakip alanının şarjı eritmesi (drain) bu kadarı
    tekDrag: .35,    // v5 · tek enerji: rakip alanının yavaşlatması (oppDrag) bu kadarı: şarj uçuşta azaldığı için direnç artıyordu, Çekirdek fazla yavaşlıyordu (goller azaldı)
    tekTaban: .2    // v5 · tek enerji: şarjsız vuruşun da en az bu kadar enerjisi var (vuruşun kendisi)
  };
  const tekS0 = () => P.tekS; /* temas kodunda P yerel bir ihtimal: fizik parametresi buradan */
  const tekU = () => !self.ALAN_TEK_ENERJI_ESKI && !self.ALAN_ENERJI_ESKI; /* v5 · Çekirdeğin tek enerjisi (enerji planı, 20 Ekim 2026): şarj ile uçuş enerjisi aynı sayı. Vuruşta şarjdan (en az tekTaban) başlar; yol, rakip alanı (yavaşlatan), duvar, direk ve sekme şarjdan düşer; takım arkadaşının hızlandıran alanı şarj ekler; sıfırlanınca söner. Kalan şarj alıcıya geçer. Deney: self.ALAN_TEK_ENERJI_ESKI */
  function infl(p, x, y) {
    const dx = x - p.x, dy = y - p.y, R = p.R || 8; if (dx * dx + dy * dy > R * R * 9) return 0; let q;
    if (p.cone) { const al = dx * p.fx + dy * p.fy, pe = -dx * p.fy + dy * p.fx, a = al > 0 ? al / (R * 1.4) : al / (R * .6); q = a * a + (pe / R) ** 2; }
    else q = (dx * dx + dy * dy) / (R * R);
    return (p.D ?? 1) * Math.exp(-q * 1.2);
  }
  // Paketlenmiş alan: oyuncular bir yörünge boyunca kımıldamıyorsa (predict, hız arama) alan aynı formülle, aynı sırayla düz bir diziden okunur.
  // Sonuç bit bit aynıdır; sadece her adımda farklı biçimli oyuncu nesnelerini okumanın maliyeti kalkar. Önbellekli (_kc) ve ızgaralı (_g) kaynaklar olduğu gibi kalır.
  const secAcik = () => !self.ALAN_ALAN_SECIM_ESKI && !self.ALAN_ENERJI_ESKI; /* v5 · alanın etkin kullanımı (enerji 3. aşama). Deney: self.ALAN_ALAN_SECIM_ESKI → pasif alan (takım arkadaşı hızlandırır, rakip yavaşlatır, hep) */
  const amOf = (p, team) => p.amA ? (p.am || 0) : (p.team !== team && (p.en == null || p.en > .15) ? -1 : 0); /* oyuncunun alan seçimi (+1 hızlandır, −1 yavaşlat, 0 kapalı). Top havadayken maç her oyuncu için seçer (amA); seçim yokken (kafanın vuruş öncesi tahmini) beklenen: yakındaki rakip yavaşlatır, takım arkadaşı bırakır, barı boş olan katılmaz */
  const dimS = s => s <= 1 ? s : s <= 2 ? 1 + .5 * (s - 1) : 1.5 + .25 * (s - 2); /* üst üste binen alanlar azalan getiriyle: ilk birim tam, ikinci yarım, sonrası çeyrek */
  function Ksec(src, x, y, team) { let sp = 0, sn = 0; for (const p of src) { const a = amOf(p, team); if (!a) continue; const f = infl(p, x, y); if (a > 0) sp += f; else sn += f; } return Math.tanh(dimS(sp) - dimS(sn)); } /* net etki: hızlandıranlar − yavaşlatanlar (düello), topun takımından bağımsız */
  function pack(src) { if (src.pk || src._kc || src._g) return src; const SA = secAcik(), st = SA ? 16 : 12, n = src.length, a = new Float64Array(n * st); for (let i = 0; i < n; i++) { const p = src[i], o = i * st, R = p.R || 8; a[o] = p.x; a[o + 1] = p.y; a[o + 2] = R; a[o + 3] = p.D ?? 1; a[o + 4] = p.cone ? 1 : 0; a[o + 5] = p.fx; a[o + 6] = p.fy; a[o + 7] = p.team; a[o + 8] = R * R * 9; a[o + 9] = R * R; a[o + 10] = R * 1.4; a[o + 11] = R * .6; if (SA) { a[o + 12] = p.amA ? 1 : 0; a[o + 13] = p.am || 0; a[o + 14] = p.en == null ? 1 : p.en; } } return { pk: a, n, sa: SA }; } /* hız: yarıçapın çarpımları bir kez hesaplanır (aynı işlemler, aynı sayılar) */
  function KpkSec(s, x, y, team) { const a = s.pk; let sp = 0, sn = 0; for (let i = 0, o = 0; i < s.n; i++, o += 16) { const am = a[o + 12] ? a[o + 13] : (a[o + 7] !== team && a[o + 14] > .15 ? -1 : 0); if (!am) continue; const dx = x - a[o], dy = y - a[o + 1], d2 = dx * dx + dy * dy; if (d2 > a[o + 8]) continue; let q; if (a[o + 4]) { const R = a[o + 2], fx = a[o + 5], fy = a[o + 6], al = dx * fx + dy * fy, pe = -dx * fy + dy * fx, aa = al > 0 ? al / a[o + 10] : al / a[o + 11]; q = aa * aa + (pe / R) ** 2; } else q = d2 / a[o + 9]; const f = a[o + 3] * Math.exp(-q * 1.2); if (am > 0) sp += f; else sn += f; } return Math.tanh(dimS(sp) - dimS(sn)); }
  function Kpk(s, x, y, team) { if (s.sa) return KpkSec(s, x, y, team); const a = s.pk; let v = 0; for (let i = 0, o = 0; i < s.n; i++, o += 12) { const dx = x - a[o], dy = y - a[o + 1], d2 = dx * dx + dy * dy; if (d2 > a[o + 8]) continue; /* hız: menzil dışındaki oyuncunun katkısı 0; 0 eklemek toplamı değiştirmez */ let q; if (a[o + 4]) { const R = a[o + 2], fx = a[o + 5], fy = a[o + 6], al = dx * fx + dy * fy, pe = -dx * fy + dy * fx, aa = al > 0 ? al / a[o + 10] : al / a[o + 11]; q = aa * aa + (pe / R) ** 2; } else q = d2 / a[o + 9]; const f = a[o + 3] * Math.exp(-q * 1.2); if (a[o + 7] === team) v += f; else v -= f; } return Math.tanh(v); }
  // Kararın önbellekli alanı (_kc): yarım birimlik hücreler, hücrenin ilk sorulduğu noktadaki değer saklanır. Saha içindeki hücreler Map yerine düz dizide (aynı hücre eşlemesi, aynı ilk-değer kuralı).
  function KcT(src, c, x, y, team) { let T = src._kt; if (!T) T = src._kt = { v: new Float64Array(GX1 * GY1), f: new Uint8Array(GX1 * GY1) }; let v; if (T.f[c]) v = T.v[c]; else { v = 0; for (const p of src) v += (p.team === 0 ? 1 : -1) * infl(p, x, y); T.v[c] = v; T.f[c] = 1; } return Math.tanh(team === 0 ? v : -v); }
  function K(src, x, y, team) { if (src.pk) return Kpk(src, x, y, team); if (!src._kc && !src._g && secAcik()) return Ksec(src, x, y, team); if (src._kc) { const i = (x * 2 + .5) | 0, j = (y * 2 + .5) | 0; if (i >= 0 && i <= GX1 - 1 && j >= 0 && j <= GY1 - 1) { return KcT(src, i * GY1 + j, x, y, team); } const key = i * 1000 + j; let v = src._kc.get(key); if (v === undefined) { v = 0; for (const p of src) v += (p.team === 0 ? 1 : -1) * infl(p, x, y); src._kc.set(key, v); } return Math.tanh(team === 0 ? v : -v); } if (src._g) return Math.tanh(gridS(src._g, x, y) * (team === 0 ? 1 : -1)); let s = 0; for (const p of src) s += (p.team === team ? 1 : -1) * infl(p, x, y); return Math.tanh(s); }
  // hızlı alan: 1 birimlik ızgarada Mavi(+)/Turuncu(-) toplamı, çift doğrusal okunur (sadece kafadaki simülasyon için)
  function makeGrid(src) { const g = new Float32Array(GW1 * GH1); for (let j = 0; j <= H; j++) for (let i = 0; i <= W; i++) { let s = 0; for (const p of src) { const dx = i - p.x, dy = j - p.y; if (dx * dx + dy * dy > 400) continue; s += (p.team === 0 ? 1 : -1) * infl(p, i, j); } g[j * GW1 + i] = s; } return g; }
  function gridS(g, x, y) { x = Math.max(0, Math.min(GXM, x)); y = Math.max(0, Math.min(GYM, y)); const i = x | 0, j = y | 0, fx = x - i, fy = y - j, a = g[j * GW1 + i], b = g[j * GW1 + i + 1], c = g[(j + 1) * GW1 + i], d = g[(j + 1) * GW1 + i + 1]; return a + (b - a) * fx + (c - a) * fy + (a - b - c + d) * fx * fy; }
  // v5 · Direk: Kuyu ağzının iki ucunda yuvarlak engel, merkezi duvar çizgisinde (x = 0,5 | W − 0,5; y = orta ± MOUTH). Bu tikteki yol (x,y)→(x+vx,y+vy) direğe duvar çizgisinden önce değerse
  // Çekirdek değdiği noktada durur ve duvar sekmesinin kuralıyla seker (hız yüzeye göre yansır, rest ile azalır, enerji wallLoss kadar düşer). Dar açıdan ağzın kenarını sıyıran yol önce direğe değer: açı eşiği yok, geometri.
  // Dönüş: null ya da [x, y, vx, vy]. Üç adım fonksiyonu (stepBall, rollTo, predict) aynı fonksiyonu çağırır.
  // v5 · sönmüş Çekirdek de ağızdan geçince Kuyu'ya girer (sayı). Önce yalnız canlı Çekirdek giriyordu, sönmüşü ağzın içinden görünmez bir duvar gibi geri sekiyordu. Deney: self.ALAN_OLU_GOL_ESKI
  function direk(x, y, vx, vy, p) { if (DIREK <= 0) return null; const nx = x + vx; if (x > DIREK + 4 && nx > DIREK + 4 && x < W - DIREK - 4 && nx < W - DIREK - 4) return null; const L2 = vx * vx + vy * vy; if (L2 < 1e-12) return null;
    let tw = 2; if (nx <= .5 && Math.abs(vx) > 1e-9) tw = (.5 - x) / vx; else if (nx >= W - .5 && Math.abs(vx) > 1e-9) tw = (W - .5 - x) / vx;
    let bt = 2, bx = 0, by = 0; for (const px of [.5, W - .5]) for (const py of [H / 2 - MOUTH, H / 2 + MOUTH]) { const fx = x - px, fy = y - py, c0 = fx * fx + fy * fy - DIREK * DIREK; if (c0 <= 0) continue; const bb = fx * vx + fy * vy; if (bb >= 0) continue; const disc = bb * bb - L2 * c0; if (disc < 0) continue; const t = (-bb - Math.sqrt(disc)) / L2; if (t < 0 || t > 1 || t >= bt) continue; bt = t; bx = px; by = py; }
    if (bt > 1 || bt > tw) return null; const cx = x + vx * bt, cy = y + vy * bt; let nnx = cx - bx, nny = cy - by; const nl = hyp(nnx, nny) || 1; nnx /= nl; nny /= nl; const vn = vx * nnx + vy * nny;
    return [bx + nnx * (DIREK + 1e-3), by + nny * (DIREK + 1e-3), (vx - 2 * vn * nnx) * p.rest, (vy - 2 * vn * nny) * p.rest]; }
  function chgMul(ch, p) { p = p || P; return tekU() ? .75 + .7 * ch : 1 - p.chgV / 2 + p.chgV * ch; } /* v5 · tek enerji: şarj uçuşta azaldığı ve alışta bedava şarj kalktığı için tipik şarj düştü; vuruş gücünün tabanı yükseldi (0 → 0,75, 0,5 → 1,1, 1 → 1,45; önce 0,55 / 1 / 1,45) */
  // v5 · Çekirdek sahanın dışında olamaz: duvar çizgisinin (0,5) arkasına konan Çekirdek (çizgideki taşıyıcının gövdesinin duvar yanı, gövdeden sekme, gövdenin itmesi) çizgiye geri alınır. Kuyu denetimi Çekirdeğin içeriden geldiğini varsayar; arkadan gelen Çekirdek ağızdan geçmiş sayılıyordu (zip51, Kartal 0–1)
  function sahaIci(b) { const e = 1e-3; if (b.x < .5 + e) b.x = .5 + e; else if (b.x > W - .5 - e) b.x = W - .5 - e; if (b.y < .5 + e) b.y = .5 + e; else if (b.y > H - .5 - e) b.y = H - .5 - e; return b; }
  function makeBall(o) { if (tekU() && o && o.e == null) { const c0 = Math.max(P.tekTaban, o.ch ?? .5); o = { ...o, e: c0, ch: c0 }; } return sahaIci(Object.assign({ x: W / 2, y: H / 2, vx: 0, vy: 0, e: P.e0, ch: .5, team: 0, alive: true, dead: 0, t: 0, dist: 0, walls: 0, done: null, log: [] }, o)); }
  function ev(b, k, txt) { b.log.push({ t: b.t, k, txt }); }
  function stepBall(b, src, p) {
    p = p || P; if (b.done) return b;
    // Saha iki boyutlu: yükseklik yok. Çekirdek hep yerde, hep alanların içinden geçer; her oyuncu her an ona dokunabilir.
    const k = K(src, b.x, b.y, b.team), sp0 = hyp(b.vx, b.vy), res = 1 - p.pierce * b.ch;
    let drag = p.fric;
    if (k < 0) drag += -k * p.oppDrag * res * (tekU() ? (p.tekDrag ?? P.tekDrag) : 1); else drag *= 1 - p.ownFlow * k;
    const TU = tekU(); if (b.alive) { b.e -= (sp0 * p.attn + (k < 0 ? -k * sp0 * p.drain * res * (TU ? (p.tekDrain ?? P.tekDrain) : 1) : 0)) * (TU ? (p.tekS ?? P.tekS) : 1); if (TU && k > 0) b.e = Math.min(1, b.e + k * sp0 * (p.tekGain ?? P.tekGain)); if (b.e <= 0) { b.e = 0; b.alive = false; b.dead = 0; ev(b, 'son', 'Çekirdek söndü · boşta'); } if (TU) b.ch = b.alive ? b.e : 0; }
    else { b.dead++; drag += Math.min(p.deadMax, p.deadRamp * b.dead); }
    const f = 1 - drag; b.vx *= f; b.vy *= f;
    const s1 = hyp(b.vx, b.vy); if (s1 > 0) { const r = Math.max(0, s1 - p.roll) / s1; b.vx *= r; b.vy *= r; }
    let nx = b.x + b.vx, ny = b.y + b.vy;
    const dk = direk(b.x, b.y, b.vx, b.vy, p); if (dk) { nx = dk[0]; ny = dk[1]; b.vx = dk[2]; b.vy = dk[3]; b.walls++; b.direk = (b.direk || 0) + 1; if (b.alive) b.e = Math.max(0, b.e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); ev(b, 'direk', 'Direkten sekti'); } else
    for (const ex of [0, W]) {
      const hit = ex === 0 ? nx <= .5 : nx >= W - .5; if (!hit) continue;
      const wx = ex === 0 ? .5 : W - .5, tt = Math.abs(b.vx) > 1e-9 ? (wx - b.x) / b.vx : 0, yy = b.y + b.vy * Math.max(0, Math.min(1, tt));
      if ((b.alive || !self.ALAN_OLU_GOL_ESKI) && Math.abs(yy - H / 2) <= MOUTH) { b.x = wx; b.y = yy; b.done = ex === 0 ? 'kuyu-sol' : 'kuyu-sag'; ev(b, 'kuyu', `KUYU · ${ex === 0 ? 'sol' : 'sağ'} Kuyu'ya girdi`); return b; }
      nx = ex === 0 ? 1 - nx : 2 * W - 1 - nx; b.vx = -b.vx * p.rest; b.vy *= p.rest; b.walls++; if (b.alive) b.e = Math.max(0, b.e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); ev(b, 'kenar', `Kenardan sekti (${ex === 0 ? 'sol' : 'sağ'} uç, Kuyu'nun dışı)`);
    }
    if (ny < .5 || ny > H - .5) { ny = ny < .5 ? 1 - ny : 2 * H - 1 - ny; b.vy = -b.vy * p.rest; b.vx *= p.rest; b.walls++; if (b.alive) b.e = Math.max(0, b.e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); ev(b, 'kenar', `Kenardan sekti (${ny < H / 2 ? 'üst' : 'alt'})`); }
    b.dist += hyp(nx - b.x, ny - b.y); b.x = nx; b.y = ny; b.t++; if (b.walls !== b._w0 || (b.defl || 0) !== b._d0) { b.tp = 0; b._w0 = b.walls; b._d0 = b.defl || 0; } else b.tp = (b.tp ?? b.t - 1) + 1; /* yolun yaşı: duvara çarpınca ya da sekince sıfırlanır */
    if (hyp(b.vx, b.vy) < .015) { b.vx = 0; b.vy = 0; b.done = 'durdu'; ev(b, 'dur', b.alive ? 'Durdu' : 'Sönmüş hâlde durdu'); }
    return b;
  }
  // Hız arama için aynı fizik adımı, nesnesiz: stepBall'un birebir aynı işlemleri aynı sırayla, yerel değişkenlerle (kayıt ve mesafe sayacı tutulmaz, sonucu etkilemezler).
  // Çekirdek (hx,hy)'den L uzaklığa vardığında hızını döndürür; varamazsa -1. Pas hızı aramasında binlerce kez çağrılır.
  function rollTo(x, y, vx, vy, team, ch, src, hx, hy, L, maxT) {
    const p = P, TU = tekU(); let e = TU ? Math.max((p.tekTaban ?? P.tekTaban), ch) : p.e0, alive = true, dead = 0; if (TU) ch = e;
    for (let t = 0; t < maxT; t++) {
      const k = K(src, x, y, team), sp0 = hyp(vx, vy), res = 1 - p.pierce * ch;
      let drag = p.fric;
      if (k < 0) drag += -k * p.oppDrag * res * (tekU() ? (p.tekDrag ?? P.tekDrag) : 1); else drag *= 1 - p.ownFlow * k;
      if (alive) { e -= (sp0 * p.attn + (k < 0 ? -k * sp0 * p.drain * res * (TU ? (p.tekDrain ?? P.tekDrain) : 1) : 0)) * (TU ? (p.tekS ?? P.tekS) : 1); if (TU && k > 0) e = Math.min(1, e + k * sp0 * (p.tekGain ?? P.tekGain)); if (e <= 0) { e = 0; alive = false; dead = 0; } if (TU) ch = alive ? e : 0; }
      else { dead++; drag += Math.min(p.deadMax, p.deadRamp * dead); }
      const f = 1 - drag; vx *= f; vy *= f;
      const s1 = hyp(vx, vy); if (s1 > 0) { const r = Math.max(0, s1 - p.roll) / s1; vx *= r; vy *= r; }
      let nx = x + vx, ny = y + vy, done = false;
      const dk = direk(x, y, vx, vy, p); if (dk) { nx = dk[0]; ny = dk[1]; vx = dk[2]; vy = dk[3]; if (alive) e = Math.max(0, e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); } else {
      if (nx <= .5) { const wx = .5, tt = Math.abs(vx) > 1e-9 ? (wx - x) / vx : 0, yy = y + vy * Math.max(0, Math.min(1, tt));
        if ((alive || !self.ALAN_OLU_GOL_ESKI) && Math.abs(yy - H / 2) <= MOUTH) { x = wx; y = yy; done = true; }
        else { nx = 1 - nx; vx = -vx * p.rest; vy *= p.rest; if (alive) e = Math.max(0, e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); } }
      if (!done && nx >= W - .5) { const wx = W - .5, tt = Math.abs(vx) > 1e-9 ? (wx - x) / vx : 0, yy = y + vy * Math.max(0, Math.min(1, tt));
        if ((alive || !self.ALAN_OLU_GOL_ESKI) && Math.abs(yy - H / 2) <= MOUTH) { x = wx; y = yy; done = true; }
        else { nx = 2 * W - 1 - nx; vx = -vx * p.rest; vy *= p.rest; if (alive) e = Math.max(0, e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); } } }
      if (!done) {
        if (ny < .5 || ny > H - .5) { ny = ny < .5 ? 1 - ny : 2 * H - 1 - ny; vy = -vy * p.rest; vx *= p.rest; if (alive) e = Math.max(0, e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); }
        x = nx; y = ny;
        if (hyp(vx, vy) < .015) { vx = 0; vy = 0; done = true; }
      }
      if (hyp(x - hx, y - hy) >= L) return hyp(vx, vy);
      if (done) return -1;
    }
    return -1;
  }
  function clone(b) { return Object.assign({}, b, { log: [] }); }
  // Yörünge tahmini: stepBall'un birebir aynı işlemleri yerel değişkenlerle (rollTo ile aynı gövde); sonunda Çekirdeğin son durumu ve olay kaydı kopyaya yazılır.
  function predict(b0, src, p, maxT) {
    src = pack(src); p = p || P; const b = clone(b0), pts = [{ x: b.x, y: b.y }], n = maxT || 900, team = b.team, TU = tekU(); let ch = b.ch;
    let x = b.x, y = b.y, vx = b.vx, vy = b.vy, e = b.e, alive = b.alive, dead = b.dead, t = b.t, dist = b.dist, walls = b.walls, done = b.done;
    for (let i = 0; i < n && !done; i++) {
      const k = K(src, x, y, team), sp0 = hyp(vx, vy), res = 1 - p.pierce * ch;
      let drag = p.fric;
      if (k < 0) drag += -k * p.oppDrag * res * (tekU() ? (p.tekDrag ?? P.tekDrag) : 1); else drag *= 1 - p.ownFlow * k;
      if (alive) { e -= (sp0 * p.attn + (k < 0 ? -k * sp0 * p.drain * res * (TU ? (p.tekDrain ?? P.tekDrain) : 1) : 0)) * (TU ? (p.tekS ?? P.tekS) : 1); if (TU && k > 0) e = Math.min(1, e + k * sp0 * (p.tekGain ?? P.tekGain)); if (e <= 0) { e = 0; alive = false; dead = 0; b.log.push({ t, k: 'son', txt: 'Çekirdek söndü · boşta' }); } if (TU) ch = alive ? e : 0; }
      else { dead++; drag += Math.min(p.deadMax, p.deadRamp * dead); }
      const f = 1 - drag; vx *= f; vy *= f;
      const s1 = hyp(vx, vy); if (s1 > 0) { const r = Math.max(0, s1 - p.roll) / s1; vx *= r; vy *= r; }
      let nx = x + vx, ny = y + vy;
      const dk = direk(x, y, vx, vy, p); if (dk) { nx = dk[0]; ny = dk[1]; vx = dk[2]; vy = dk[3]; walls++; b.direk = (b.direk || 0) + 1; if (alive) e = Math.max(0, e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); b.log.push({ t, k: 'direk', txt: 'Direkten sekti' }); } else {
      if (nx <= .5) { const wx = .5, tt = Math.abs(vx) > 1e-9 ? (wx - x) / vx : 0, yy = y + vy * Math.max(0, Math.min(1, tt));
        if ((alive || !self.ALAN_OLU_GOL_ESKI) && Math.abs(yy - H / 2) <= MOUTH) { x = wx; y = yy; done = 'kuyu-sol'; b.log.push({ t, k: 'kuyu', txt: `KUYU · sol Kuyu'ya girdi` }); }
        else { nx = 1 - nx; vx = -vx * p.rest; vy *= p.rest; walls++; if (alive) e = Math.max(0, e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); b.log.push({ t, k: 'kenar', txt: `Kenardan sekti (sol uç, Kuyu'nun dışı)` }); } }
      if (!done && nx >= W - .5) { const wx = W - .5, tt = Math.abs(vx) > 1e-9 ? (wx - x) / vx : 0, yy = y + vy * Math.max(0, Math.min(1, tt));
        if ((alive || !self.ALAN_OLU_GOL_ESKI) && Math.abs(yy - H / 2) <= MOUTH) { x = wx; y = yy; done = 'kuyu-sag'; b.log.push({ t, k: 'kuyu', txt: `KUYU · sağ Kuyu'ya girdi` }); }
        else { nx = 2 * W - 1 - nx; vx = -vx * p.rest; vy *= p.rest; walls++; if (alive) e = Math.max(0, e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); b.log.push({ t, k: 'kenar', txt: `Kenardan sekti (sağ uç, Kuyu'nun dışı)` }); } } }
      if (!done) {
        if (ny < .5 || ny > H - .5) { ny = ny < .5 ? 1 - ny : 2 * H - 1 - ny; vy = -vy * p.rest; vx *= p.rest; walls++; if (alive) e = Math.max(0, e - p.wallLoss * (tekU() ? (p.tekS ?? P.tekS) : 1)); b.log.push({ t, k: 'kenar', txt: `Kenardan sekti (${ny < H / 2 ? 'üst' : 'alt'})` }); }
        dist += hyp(nx - x, ny - y); x = nx; y = ny; t++;
        if (hyp(vx, vy) < .015) { vx = 0; vy = 0; done = 'durdu'; b.log.push({ t, k: 'dur', txt: alive ? 'Durdu' : 'Sönmüş hâlde durdu' }); }
      }
      pts.push({ x, y, e, sp: hyp(vx, vy) });
    }
    if (TU) b.ch = ch; b.x = x; b.y = y; b.vx = vx; b.vy = vy; b.e = e; b.alive = alive; b.dead = dead; b.t = t; b.dist = dist; b.walls = walls; b.done = done;
    return { pts, end: b };
  }
  // Hedefe pas: hedef noktaya `arrive` hızıyla ulaşan başlangıç hızını bul (alanlar dahil, düz hat varsayımı).
  function solveLaunch(b0, to, arrive, src, p, vmax) {
    const dx = to.x - b0.x, dy = to.y - b0.y, L = hyp(dx, dy) || 1, ux = dx / L, uy = dy / L; vmax = vmax || 3; src = pack(src);
    const speedAt = v0 => { const b = clone(b0); b.vx = ux * v0; b.vy = uy * v0; b.done = null; let last = 0; for (let i = 0; i < 900 && !b.done; i++) { stepBall(b, src, p); const d = (b.x - b0.x) * ux + (b.y - b0.y) * uy; last = hyp(b.vx, b.vy); if (d >= L) return last; if (b.walls) return -1; } return -1; };
    let lo = .05, hi = vmax; if (speedAt(hi) < arrive) return { v0: hi, ok: false };
    for (let i = 0; i < 18; i++) { const mid = (lo + hi) / 2; if (speedAt(mid) < arrive) lo = mid; else hi = mid; }
    return { v0: hi, ok: true };
  }
  // Katman 2 · Karşılama. Oyuncu: {x,y,team,R,D, a:{tutus,kesme,yogunluk,hiz}, role}
  const Q = {
    tepki0: 12, tepkiOk: .5, capHaz: .4, // hazırlık (herkes için): Çekirdeğin şimdiki yolu başlayalı (vuruş, duvar, sekme) geçen süre / oyuncunun tepki süresi (12 − Okuma × 0,5 tik). Hazır olmayan oyuncu Çekirdeği ancak capHaz + (1 − capHaz) × hazırlık kadar söndürebilir ve uzanması bu yolun başından sayılır
    reachBody: .9, reachReact: 6, reachReactOk: .25, reachGrow: .12, // rakibin uzanması: gövde, tepki süresi (tik), Okuma etkisi, büyüme hızı
    reach: 1.6,     // temel kontrol menzili (birim)
    body: 1.15,     // oyuncunun gövdesi + Çekirdeğin yarıçapı: Çekirdek gövdenin içine giremez, tutulan Çekirdek gövdenin kenarında durur
    reachBek: .6,   // (eski) Bekçi ek menzili
    bekThru: .25, bekBody: .9, bekDive: 2.0, bekDiveV: .32, bekHarT: 4, bekHarD: .25, /* v5 · hareket halindeki Bekçi: şut anındaki hızı (en çok hızına oranla) kadar tepki gecikir (tam koşuda +8 tik), dalış menzili kısalır (tam koşuda yarıya) */ bekEl: 1.1, /* temiz test: 15 birimden ~%55, 25 birimden ~%27 sayı (köşeye nişanla) */ bekSteer: 2.5, // Bekçi elleriyle oynar: daha uzağa uzanır (dive), daha sert Çekirdeği söndürür (bekEl × kapasite), tutamadığını Kuyu'dan uzağa, yana çeler (bekSteer)
     bekReact: 8, bekReactOk: .5, // Bekçi: gövde, en fazla uzanma, uzanma hızı (birim/tik), tepki süresi
    // İlk dokunuş: oyuncu tek dokunuşta Çekirdeğin (kendisine göre) en fazla "cap" kadar hızını söndürebilir. Gelen hız bunun altındaysa tutar; üstündeyse kalan hız gövdeden seker.
    cap0: 1.3,      // ortalama Tutuş'la söndürülebilen göreli hız (birim/tik)
    capSk: .06,    // Tutuş/Kesme puanı başına
    capCh: .6,      // canlı Çekirdeğin şarjı söndürmeyi zorlaştırır (şarj × bu)
    capPr: .5,      // üstündeki rakip alanının en fazla kestiği kapasite
    capS0: .1, capSsk: .012, // dokunuşun belirsizliği: (20 − puan) × capSsk + capS0
    rest2: .55,     // gövdeden sekmede, söndürülemeyen hızın geri kalan kısmı (dik bileşen)
    steer: .5,      // söndürülemeyen hızı istenen yöne çevirebilme (Tutuş ile)
    pressK: .35,    // üstündeki rakip alanının cezası (başlangıç eğimi)
    pressMax: .45,  // baskı cezasının tavanı: kalabalıkta bile Çekirdeği karşılamak mümkündür, sadece zordur
    deflE: .15,     // sekmede kaybedilen enerji
    cd: 12, cdMin: 3, cdV: .9,         // aynı oyuncu tekrar deneyemeden geçen tik
    awayK: .45,     // kendisinden uzaklaşan Çekirdek için menzil oranı
    thruV: .9, thruK: .6, thruMax: .55, thruKeep: .6 // hızlı Çekirdeğin gövdeyi delmesi: hız eşiği, eğim, tavan, kalan hız
  };
  // Bekçi'nin menzili: gövde + uzanma. Uzanmak için zaman gerekir: Çekirdek yola çıktıktan sonra tepki süresi (Okuma) geçince menzil büyür (Hız hızlandırır).
  /* Bekçi'nin tepki payı (tik; eksi: top çıkmadan davranır). Yerleşmiş Bekçi (Öneri A.4) payın %40'ı kadar erken davranır. v5 · önce pay ×0,6 ile çarpılıyordu: Refleks 16'nın üstünde pay eksi, çarpınca sıfıra yaklaşıyor, yerleşmiş Bekçi yerleşmemişten geç uzanıyordu. Formül (decide.js) aynı fonksiyonu kullanır. Deney: self.ALAN_BEKCI_REFLEKS_ESKI */
  function bekRe(p, q) { q = q || Q; const r = q.bekReact - self.ALAN_OKC(p, 'bekci') * q.bekReactOk; if (!(p._yer && !self.ALAN_BEKCI_A_ESKI)) return r; return self.ALAN_BEKCI_REFLEKS_ESKI ? r * .6 : r - .4 * Math.abs(r); }
  /* v5 · Bekçi'nin vuruş anındaki hareketinin topa göre cezası (0–1): koşu hızı oranı × yön. Topun yoluna doğru koşan Bekçi'ye ceza yok, yoldan uzaklaşana tam, yola dik koşana (açı daraltırken öne çıkan) yarım. Ters ayağa kalan köşe açılır; her şutta ceza yok */
  function bekHar(p, x, y, vx, vy) { const sp = hyp(p.vx || 0, p.vy || 0), s = Math.min(1, sp / ((.17 + self.ALAN_HZ(p) * .006) * self.ALAN_SPR(p))); if (s <= 1e-6) return 0; const L2 = vx * vx + vy * vy, t = L2 > 1e-9 ? Math.max(0, ((p.x - x) * vx + (p.y - y) * vy) / L2) : 0, nx = x + vx * t - p.x, ny = y + vy * t - p.y, nl = hyp(nx, ny), c = nl > .3 ? ((p.vx || 0) * nx + (p.vy || 0) * ny) / (sp * nl) : 0; return s * (1 - c) / 2; }
  function bekR(b, p, q) { q = q || Q; const a = p.a || {}, hm = !self.ALAN_BEKCI_HAREKET_ESKI && !b.defl && b.bekId === p.id && b.bekM > 0 ? b.bekM : 0, re = bekRe(p, q) + q.bekHarT * hm; /* v5 · koşarken (açı daraltırken bile) Bekçi hemen uzanamaz: önce durup yön değiştirir; tepkisi bekHarT, dalışı bekHarD × hareket cezası (bekHar: vuruş yönüne göre, topun yoluna koşana yok, yoldan uzaklaşana tam). Önce yerinde duran Bekçi gibi tepki veriyordu (Tepki 20'de −2 tik: şut çıkmadan dalış başlıyordu), kalesinden çıkıp koşan Bekçi kalenin tamamını kapatıyordu (722701 t5323, Tufan 0,31 hızla Kaan'a koşarken: 16 sert şutta 1 gol). Deney: self.ALAN_BEKCI_HAREKET_ESKI */ /* v5 · Öneri A.4: yerleşmiş Bekçi (hızı < 0,05, hedefine 1'den yakın) daha çabuk tepki verir: yer tutmanın değeri */ return q.bekBody + Math.min(q.bekDive * (1 - q.bekHarD * hm), Math.max(0, (b.tp ?? b.t ?? 0) - re) * (q.bekDiveV + ((self.ALAN_BEKCI_DALIS_HZ_ESKI ? (a.hiz ?? 10) : self.ALAN_HZ(p)) - 10) * .01)); } /* v5 · dalış hızı yorgunluğa göre (ALAN_HZ), kafadaki gibi. Deney: self.ALAN_BEKCI_DALIS_HZ_ESKI */
  // Dokunuşun fiziği: Çekirdeğin oyuncuya göre hızı (oyuncu ona doğru koşuyorsa artar, onunla aynı yöne gidiyorsa azalır) ve oyuncunun o an söndürebileceği hız.
  function touchCap(b, p, src, q) { q = q || Q; const mate = p.team === b.team, a = p.a || {}, bek = p.role === 'Bekçi', TK = self.ALAN_YET.acik(3), DZ = !bek && self.ALAN_YET.acik(2), sk = bek ? self.ALAN_OZ(p, 'hakimiyet') : mate ? (TK ? self.ALAN_Y(p, 'kontrolKap') : self.ALAN_OZ(p, 'kontrol')) : TK ? self.ALAN_Y(p, 'kesmeKap') : self.ALAN_OZ(p, 'kesme'); /* v5 · Bekçi: Alan hakimiyeti · arkadaş: Kontrol · rakip (pas arası): Kesme. Yetenek Aşama 3: arkadaşta İlk kontrol 65, Teknik 20, Denge 15; rakipte Kesme 60, Okuma 25, Uzanma 15; hazırlık süresi Okuma ve Tepki'den (Deney: self.ALAN_YET_TEKNIK_ESKI). Aşama 2: baskının kestiği kapasiteyi Soğukkanlılık yumuşatır; denge kaybı da baskı kadar keser (tam dengesiz = tam baskı); arkadaş koşarken alırsa da keser (ALAN_DURUM.alis; Deney: self.ALAN_YET_DURUM_ESKI) */
    let pr = 0; for (const o of src) if (o.team !== p.team) pr += infl(o, p.x, p.y);
    const tau = Math.max(1, q.tepki0 - (TK && !bek ? self.ALAN_OKY(p, 'tepki', 'hazirlik') : self.ALAN_OKC(p, p.role === 'Bekçi' ? 'bekci' : 'tepki')) * q.tepkiOk), haz = Math.min(1, (b.tp ?? b.t ?? 99) / tau), prK = 1 - Math.exp(-Math.max(0, pr - .4) * q.pressK / q.pressMax), cap = (DZ ? q.cap0 + (sk - 10) * q.capSk - (b.alive ? b.ch * q.capCh : 0) - q.capPr * self.ALAN_DURUM.alis(p, mate, prK) : q.cap0 + (sk - 10) * q.capSk - (b.alive ? b.ch * q.capCh : 0) - q.capPr * prK) * (q.capHaz + (1 - q.capHaz) * haz), vrx = b.vx - (p.vx || 0), vry = b.vy - (p.vy || 0);
    return { cap: Math.max(.08, cap * (p.role === 'Bekçi' && !mate ? q.bekEl * (self.ALAN_KALECI_ESKI ? 1 : Math.max(.2, 1 + (sk - 10) * .05)) : 1)), /* v5 · kaleci: elleri Alan hakimiyeti'yle güçlenir (20'de ×1,5, 10'da ×1, 0'da ×0,5): iyi kaleci az çeler, tutar. Deney: self.ALAN_KALECI_ESKI */ vr: hyp(vrx, vry), vrx, vry, s: q.capS0 + (20 - sk) * q.capSsk, sk }; }
  // Pası kesmek için tepki süresi: rakip, son vuruştan hemen sonra yalnızca gövdesine çarpan Çekirdeği keser; uzanabilmek için tepki vermesi gerekir (Okuma kısaltır). Hızlı ve tekte oynanan pas bu yüzden presçinin yanından geçebilir.
  function oppReach(b, p, q, R) { if (p.role === 'Bekçi' || p.team === b.team) return R; q = q || Q; return Math.min(R, q.reachBody + Math.max(0, (b.tp ?? b.t ?? 0) - (q === Q ? self.ALAN_TEPKI(p, 'uzanma') : q.reachReact - (self.ALAN_YET.acik(3) ? self.ALAN_OKY(p, 'tepki', 'uzanmaBasla') : self.ALAN_OKC(p, 'tepki')) * q.reachReactOk)) * q.reachGrow); }
  /* v5 · topa uzanma menzili (gövde + bacak): Uzanma'dan. Yetenek Aşama 3: Uzanma 70, Çeviklik 30 (Deney: self.ALAN_YET_TEKNIK_ESKI). Aşama 2: dengesizken gövdenin dışındaki payı, maçın dengesizken yavaşlama ölçüsünde kısalır (ALAN_DURUM.dengeK; Bekçi'de yok; Deney: self.ALAN_YET_DURUM_ESKI). Maç, kafa ve yerleşim aynı fonksiyondan */
  function menzil(p, q) { q = q || Q; const R = q.reach + ((self.ALAN_YET.acik(3) ? self.ALAN_Y(p, 'uzanmaMenzil') : self.ALAN_OZ(p, 'uzanma')) - 10) * .04; return p.den != null && p.den < 1 && p.role !== 'Bekçi' && self.ALAN_YET.acik(2) ? 1 + (R - 1) * self.ALAN_DURUM.dengeK(p.den) : R; }
  function ctrlP(b, p, src, q) { const t = touchCap(b, p, src, q); return Math.max(.02, Math.min(.99, 1 / (1 + Math.exp((t.vr - t.cap) / t.s)))); }
  // Bir tikte Çekirdeğin geçtiği parçayı kontrol menzilinde kesen oyuncu var mı? Varsa tutma/sekme. Dönüş: null | {p, took:true} | {p, took:false}
  function contact(b, src, q, rnd, ox, oy) {
    q = q || Q; rnd = rnd || Math.random; const sx = b.x - ox, sy = b.y - oy, sl = sx * sx + sy * sy, sq = Math.sqrt(sl); let best = null, bd = 1e9, be = 1e9;
    for (const p of src) {
      if (p.noTouch || (b.cds && (b.cds.get(p) || 0) > b.t)) continue; if (p === b.from && b.t < 8) continue; if (b.recv && p.team === b.team && p !== b.recv && !b.defl && !(b.sap && !self.ALAN_ISABETSIZ_ESKI)) continue; if (b.gon && p.team === b.team && p.role !== 'Bekçi' && !b.defl) continue; /* v5 · arkadaş kendi takımının göndermesini de bırakır (Uzanma arttıkça önündeki arkadaşın kendi şutunu kesmesi buradandı) */ // arkadaş başkasına giden pası bırakır
      const t0 = sl > 0 ? ((p.x - ox) * sx + (p.y - oy) * sy) / sl : 1, tt = Math.max(0, Math.min(1, t0)), dd = hyp(ox + sx * tt - p.x, oy + sy * tt - p.y), R = oppReach(b, p, q, (p.role === 'Bekçi' && (!self.ALAN_BEK_ALAN_ESKI || !self.ALAN_BEK_ALAN || hyp(p.x - (p.team === 0 ? 0 : GW), p.y - GCY) < self.ALAN_BEK_ALAN) ? bekR(b, p, q) : menzil(p, q)) * (t0 < 0 ? q.awayK : 1)); const R2 = Math.max(R, 1.15); // kendisinden uzaklaşan Çekirdeğe ancak dibindeyse yetişir · v5: ama gövde yarıçapının (1,0) altına inmez; yoksa yanında koşan oyuncu, gövdesinden seken Çekirdeğe hiç dokunamıyordu (ikisi de topla süzülüyordu)
      if (self.ALAN_TEMAS_SIRA_ESKI) { if (dd < R2 && dd < bd) { bd = dd; best = p; } } else if (dd < R2) { const en = sl > 0 ? Math.max(0, t0 * sq - Math.sqrt(R2 * R2 - dd * dd)) / sq : 0; if (en < be - 1e-9 || (en < be + 1e-9 && dd < bd)) { be = en; bd = dd; best = p; } } /* v5 · bu tikteki yolda topa ilk değen oyuncu (menziline ilk giren; eşitse yola en yakın). Önce yola en yakın oyuncu seçiliyordu: aynı tikte öndeki savunmacının yerine arkadaki oyuncu topa dokunabiliyordu. Deney: self.ALAN_TEMAS_SIRA_ESKI */
    }
    if (!best) return null;
    const T = touchCap(b, best, src, q), P = Math.max(.02, Math.min(.99, 1 / (1 + Math.exp((T.vr - T.cap) / T.s)))), u = rnd(), capE = T.cap + T.s * Math.log(u / (1 - u)); // bu dokunuşun gerçek kapasitesi (lojistik gürültü: ortalaması cap)
    if (T.vr <= capE) { b.done = 'tutuldu'; b.holder = best; { const gx = best.team === 0 ? GW : 0, ix = best.tx != null ? best.tx - best.x : gx - best.x, iy = best.ty != null ? best.ty - best.y : GCY - best.y, il = hyp(ix, iy) || 1; b.x = best.x + ix / il * q.body; b.y = best.y + iy / il * q.body; sahaIci(b); } b.gvx = b.vx; b.gvy = b.vy; b.vx = b.vy = 0; /* v5 · geliş hızı saklanır: alış (take) bunu tek dokunuşta kullanır; önce sıfırlandıktan sonra okunuyordu */ ev(b, 'tut', `${best.name || (best.team === b.team ? 'Arkadaş' : 'Rakip')} tuttu (ihtimal %${Math.round(P * 100)})`); return { p: best, took: true, P }; }
    const mateT = best.team === b.team, sp0 = hyp(b.vx, b.vy) || 1e-6; if (!b.cds) b.cds = new Map(); b.cds.set(best, b.t + Math.max(q.cdMin, Math.round((best.role !== 'Bekçi' && self.ALAN_YET.canli(3) ? q.cd - (self.ALAN_Y(best, 'yenidenDokun') - 10) * .5 : q.cd) * Math.min(1, sp0 / q.cdV)))); // yavaş Çekirdeği kaçıran hemen toparlar; hızlı olan zaten uzaklaşır · v5 yetenek Aşama 3: bekleme İlk kontrol ve Teknik'ten, alıştaki tepki süresinin eğimiyle (puan başına 0,5 tik: 10 → 12 eskisi gibi, 20 → 7); Bekçi'de sabit (Deney: self.ALAN_YET_CANLI_ESKI)
    // Söndürülemeyen hız gövdeden seker: çarpma noktasında gövdeye dik bileşen geri döner (kısmen), teğet bileşen sürer. Sıyırıp geçen Çekirdek az sapar, tam ortadan çarpan geri döner.
    // Oyuncu kalan hızı istediği yöne (arkadaşsa hedefine, rakipse kendi hücum yönüne) bir miktar çevirebilir; Tutuş ne kadar çevirebileceğini belirler. Gövdeyi delme (çok hızlı Çekirdek) ayrı.
    const thru = !mateT && (self.ALAN_GOVDE_DELME_ESKI || bd >= 1) && rnd() < thruP(b, best, q); /* v5 · gövdeye çarpan top delip geçmez (gövde yarıçapı 1, match BODY_R): yalnız gövdenin dışından, uzanan bacağın menzilinden geçen top sızar. Önce sert top oyuncunun tam ortasından %55'e kadar geçiyordu (kafa zaten geçmez sayıyordu). Deney: self.ALAN_GOVDE_DELME_ESKI */
    if (thru) { const an = Math.atan2(b.vy, b.vx) + (rnd() - .5) * .5; b.vx = Math.cos(an) * sp0 * q.thruKeep; b.vy = Math.sin(an) * sp0 * q.thruKeep; }
    else { const tt = sl > 0 ? Math.max(0, Math.min(1, ((best.x - ox) * sx + (best.y - oy) * sy) / sl)) : 1, cx = ox + sx * tt, cy = oy + sy * tt; let nx = cx - best.x, ny = cy - best.y, nl = hyp(nx, ny); if (nl < 1e-3) { nx = -T.vrx; ny = -T.vry; nl = hyp(nx, ny) || 1; } nx /= nl; ny /= nl;
      const left = Math.max(0, T.vr - Math.max(0, capE)), ux = T.vrx / (T.vr || 1), uy = T.vry / (T.vr || 1), dn = ux * nx + uy * ny; let ox2 = ux, oy2 = uy; if (dn < 0 && !(mateT && self.ALAN_KACIR)) { ox2 = ux - (1 + q.rest2) * dn * nx; oy2 = uy - (1 + q.rest2) * dn * ny; } /* arkadaşın kontrol edemediği Çekirdek gövdeden sekmez: söndürülemeyen hız gelişi yönünde sürer (top ayaktan kaçar), oyuncu onu Tutuş'u kadar hedefine çevirir */ const ol = hyp(ox2, oy2) || 1; ox2 /= ol; oy2 /= ol;
      const bkP = best.role === 'Bekçi' && !mateT, gxA = best.team === 0 ? GW : 0; let bkx = null, bky = null; if (bkP && !self.ALAN_KALECI_ESKI) { const own = best.team === 0 ? 0 : GW, out = own === 0 ? 1 : -1, Lp = Math.max(3, Math.min(14, Math.max(left, .8 + (T.sk - 10) * .04) * 15)); let bs = -1e9; for (let a = -80; a <= 80; a += 10) { const r = a * Math.PI / 180, dx = Math.cos(r) * out, dy = Math.sin(r), px = Math.max(1, Math.min(GW - 1, best.x + dx * Lp)), py = Math.max(1, Math.min(GH - 1, best.y + dy * Lp)); let od = 12; for (const o of src) { if (o.team === best.team || o.noTouch) continue; od = Math.min(od, hyp(o.x - px, o.y - py)); } const sc = od - (Math.abs(px - own) < 22 && Math.abs(py - GCY) < 12 ? 6 : 0) - (self.ALAN_CELME_YON_ESKI ? 0 : .01 * Math.acos(Math.max(-1, Math.min(1, dx * ox2 + dy * oy2)))); if (sc > bs) { bs = sc; bkx = dx; bky = dy; } } } /* v5 · eşit yönlerden topun kendi sekme yönüne en yakını (en az çevirdiği): önce eşitlikte döngüdeki ilk yön (−80°) seçiliyor, çelme hep y = 0 kenarına gidiyordu. Deney: self.ALAN_CELME_YON_ESKI */ /* v5 · kaleci çelmeyi rakipten uzağa yöneltir: kalesinden dışarı 17 yön (−80°…+80°) dener, topun düşeceği noktaya en yakın rakibin uzaklığı (en çok 12) − kale önü cezası (6); en iyi yön hedeflenir. Ne kadar çevirebildiği ve isabeti Hakimiyet'le (st, ns) */ const ix = mateT && best.tx != null ? best.tx - best.x : bkP ? (bkx != null ? bkx : (gxA - best.x) * .35) : gxA - best.x, iy = mateT && best.ty != null ? best.ty - best.y : bkP ? (bky != null ? bky : (best.y >= GCY ? 1 : -1) * 25) : GCY - best.y, il = hyp(ix, iy) || 1, st = Math.min(1, q.steer * (bkP ? q.bekSteer : 1) * (bkP && !self.ALAN_BEK_CELME_ESKI ? 1 : Math.max(0, Math.min(1, capE / T.vr))) * (.5 + (T.sk - 10) * .04)) /* v5 · zip62: Bekçi'nin çelme payı kapasite oranından bağımsız, yalnız Alan hakimiyeti'ne bağlı. Kapasite tutar mı'yı, Hakimiyet nereye çeler'i belirler; önce güçlü ve yakın şutta en az çeliyordu, kalan hız Kuyu'ya gidiyordu (Kuyu'ya seken 12 temasın 10'u sayı). Deney: self.ALAN_BEK_CELME_ESKI = true */, /* Bekçi tutamadığını yana çeler */ jx = ox2 * (1 - st) + ix / il * st, jy = oy2 * (1 - st) + iy / il * st, jl = hyp(jx, jy) || 1, ns = (rnd() - .5) * (20 - T.sk) * .04, cs = Math.cos(ns), sn = Math.sin(ns), fx = jx / jl, fy = jy / jl;
      const leftB = bkP && !self.ALAN_KALECI_ESKI ? Math.max(left, .8 + (T.sk - 10) * .04) : left; /* v5 · kaleci çelerken topu eliyle iter: en az 0,8 + (Hakimiyet − 10) × 0,04 hız (yumuşak çelme kale dibine düşmesin) */ b.vx = (best.vx || 0) + (fx * cs - fy * sn) * leftB; b.vy = (best.vy || 0) + (fx * sn + fy * cs) * leftB; if (mateT && self.ALAN_KACIR && self.ALAN_KACIR_ICINDEN_ESKI) { b.x = best.x + fx * (q.reach + .05); b.y = best.y + fy * (q.reach + .05); } else { /* v5 · arkadaşın kaçırdığı top da değdiği tarafta kalır, kalan hızıyla sürer; gövdeye doğru gidiyorsa gövde çarpışması (dokunan oyuncudan kayar) onu yanından dolaştırır. Önce top kaçış yönünde, oyuncunun öbür tarafına konuyordu: tam karşıdan gelen pas tek tikte gövdenin içinden arkasına geçiyordu (218269 02:14, Mert → Ece). Deney: self.ALAN_KACIR_ICINDEN_ESKI */ b.x = best.x + nx * (q.reach + .05); b.y = best.y + ny * (q.reach + .05); } sahaIci(b); } if (b.alive) b.e = Math.max(0, b.e - (tekU() ? (q.deflE ?? Q.deflE) * tekS0() : q.deflE)); if (tekU()) b.ch = b.e; /* tek enerji: maçın temas parametrelerinde deflE yoksa çekirdeğinki (eskiden e NaN oluyordu, uçuş enerjisi kullanılmadığı için görünmüyordu) */ /* sekme hasarı şarjdan */ b.defl = (b.defl || 0) + 1; ev(b, 'sek', `${best.name || (best.team === b.team ? 'Arkadaş' : 'Rakip')} dokundu, sekti (tutma ihtimali %${Math.round(P * 100)})`);
    return { p: best, took: false, P, thru };
  }
  function thruP(b, p, q) { q = q || Q; if (p.team === b.team) return 0; const sp = hyp(b.vx, b.vy); return Math.max(0, Math.min(q.thruMax * (p.role === 'Bekçi' ? q.bekThru : 1), (sp - q.thruV) * q.thruK - ((self.ALAN_YET.acik(3) ? self.ALAN_Y(p, 'sizma') : ((p.a && p.a.kesme) ?? 10)) - 10) * .02)); } /* v5 · yetenek Aşama 3: Kesme 50, Denge 30, Gövde 20 (Deney: self.ALAN_YET_TEKNIK_ESKI) */
  function stepAll(b, src, p, q, rnd) { if (b.done) return null; const ox = b.x, oy = b.y; stepBall(b, src, p); if (b.done) return null; return contact(b, src, q, rnd, ox, oy); }
  // Dönüş: Çekirdek gövdenin içinden geçemez, kenarında bir açıda durur. Öbür yana geçmek için ya gövdenin etrafından dolaşır (Sürme hızında; taşıyıcı hızlandıkça yavaşlar), ya da fiskeyle bir an gövdeden ayrılır (hızlı ama boşta).
  // Rakip Çekirdeğe ancak gövde araya girmiyorsa uzanabilir. Yolu oyuncu kafasında seçer: süre + (Okuma ile görülen) açıkta kalma riski.
  const TQ = { tarafIyi: 1.12, tarafKotu: .9, w0: .1, wSk: .008, wV: .6, vRef: .25, flickW: .2, timeK: .003, react: 14, reactOk: .5, grab: 0, rivV: .2, pHold: .3, pHoldSk: .025, cdHold: 12, pLoose: .55, cdLoose: 3, tail: 6, shield: .8, minD: 1.7 };
  const wrapA = a => { while (a > Math.PI) a -= 2 * Math.PI; while (a < -Math.PI) a += 2 * Math.PI; return a; };
  const Turn = { TQ,
    /* v5 · Taraf: oyuncu Çekirdeği taşımayı tercih ettiği yanda (gidiş yönüne göre sol / sağ) daha hızlı döner ve fiske atar, öbür yanda biraz yavaş; iki yanlı ortalama. Puan değil, ödünleşim */
    taraf(p) { const t = p.a && p.a.taraf; if (!self.ALAN_TARAF_ESKI || !t || t === 'iki' || p.ca == null) return 1; /* v5 · ayak tercihi yok: her oyuncu iki yana da aynı döner (menajer kararı, 21 Ekim 2026). Deney: self.ALAN_TARAF_ESKI */ const vx = p.vx || 0, vy = p.vy || 0, sp = hyp(vx, vy); if (sp < .04) return 1; const cr = vx * Math.sin(p.ca) - vy * Math.cos(p.ca), sol = cr < 0; return (t === 'sol') === sol ? TQ.tarafIyi : TQ.tarafKotu; },
    omega(p, yakin) { const a = p.a || {}, v = hyp(p.vx || 0, p.vy || 0), TK = self.ALAN_YET.acik(3), dK = p.den != null && p.den < 1 && p.role !== 'Bekçi' && self.ALAN_YET.acik(2) ? self.ALAN_DURUM.dengeK(p.den) : 1; return Math.max(.03, (TQ.w0 + ((TK ? self.ALAN_Y(p, yakin ? 'dolas' : 'savurmaHiz') : (a.surme ?? 10)) - 10) * TQ.wSk) * (1 - TQ.wV * Math.min(1, v / TQ.vRef)) * Turn.taraf(p) * dK); }, /* v5 · yetenek Aşama 3: topu döndürme Teknik 50, Çeviklik 30, Sürme 20; dibinde rakipken (yakin: 4 br) dolaş Teknik 50, Denge 30, Güç 20 (Deney: self.ALAN_YET_TEKNIK_ESKI). Aşama 2: dengesizken yavaş döner (ALAN_DURUM.dengeK; Deney: self.ALAN_YET_DURUM_ESKI) */
    shortDir(a0, a1) { return wrapA(a1 - a0) >= 0 ? 1 : -1; },
    arc(a0, a1, dir) { let d = wrapA(a1 - a0); if (dir > 0 && d < 0) d += 2 * Math.PI; if (dir < 0 && d > 0) d -= 2 * Math.PI; return Math.abs(d); },
    shielded(p, o, cx, cy) { const sx = cx - o.x, sy = cy - o.y, L2 = sx * sx + sy * sy || 1, t = Math.max(0, Math.min(1, ((p.x - o.x) * sx + (p.y - o.y) * sy) / L2)); return hyp(o.x + sx * t - p.x, o.y + sy * t - p.y) < TQ.shield; },
    flickMiss(p) { return Math.max(.02, .12 - ((self.ALAN_YET.acik(3) ? self.ALAN_Y(p, 'fiskeKacir') : ((p.a || {}).surme ?? 10)) - 10) * .015); }, /* v5 · yetenek Aşama 3: Teknik 50, Sürme 30, Soğukkanlılık 20 (Deney: self.ALAN_YET_TEKNIK_ESKI) */
    flickW(p) { return self.ALAN_YET.canli(3) ? TQ.flickW * (1 + (self.ALAN_Y(p, 'fiskeHiz') - 10) * .05) : TQ.flickW; }, /* v5 · fiskenin dönüş hızı: yetenek Aşama 3'te Teknik 60, Çeviklik 40; Çeviklik'in fren ve dönüş ivmesi eğimiyle (puan başına %5: 10 → 0,2 eskisi gibi, 20 → 0,3). Deney: self.ALAN_YET_CANLI_ESKI */
    // Tek kural, iki kullanım: rnd verilirse gerçek dönüş oynanır; rnd null ise aynı dönüşün Çekirdeği koruma ihtimali (beklenen değer) hesaplanır. Oyuncu kafasında bunu oynatır.
    simulate(p0, a0, a1, plan, opps0, rnd, rec) {
      const p = { ...p0 }, opps = opps0.map(o => ({ ...o, cd: 0 })), body = Q.body, R = Q.reach, A = Turn.arc(a0, a1, plan.dir), sur = (p.a || {}).surme ?? 10, fr = [];
      let done = 0, keep = 1, res = null, tEnd = null, landed = false;
      for (let t = 0; t < 200 && !res; t++) {
        p.x += p.vx || 0; p.y += p.vy || 0; const w = plan.mode === 'fiske' ? TQ.flickW * Turn.taraf(p) : Turn.omega(p); done += Math.min(w, A - done);
        const an = a0 + plan.dir * done, cx = p.x + Math.cos(an) * body, cy = p.y + Math.sin(an) * body, fly = plan.mode === 'fiske' && done < A;
        if (plan.mode === 'fiske' && done >= A && !landed) { landed = true; const m = Turn.flickMiss(p); if (rnd) { if (rnd() < m) res = 'fiske kaçtı'; } else keep *= 1 - m; }
        for (const o of opps) { if (res) break; const ok = self.ALAN_OKC(o, 'tepki');
          if (t > TQ.react - ok * TQ.reactOk) { const dx = cx - o.x, dy = cy - o.y, L = hyp(dx, dy); if (L > R * .8) { const v = Math.min(TQ.rivV, L - R * .8); o.x += dx / L * v; o.y += dy / L * v; } const pd = hyp(o.x - p.x, o.y - p.y) || 1; if (pd < TQ.minD) { o.x = p.x + (o.x - p.x) / pd * TQ.minD; o.y = p.y + (o.y - p.y) / pd * TQ.minD; } }
          const d = hyp(cx - o.x, cy - o.y); if (t >= TQ.grab && d < R && (fly || !Turn.shielded(p, o, cx, cy)) && t >= o.cd) { o.cd = t + (fly ? TQ.cdLoose : TQ.cdHold); const P = fly ? TQ.pLoose : Math.max(.05, Math.min(.8, TQ.pHold + (((o.a || {}).kesme ?? 10) - sur) * TQ.pHoldSk)); if (rnd) { if (rnd() < P) res = 'rakip aldı'; } else keep *= 1 - P; } }
        if (!res && done >= A && tEnd == null) tEnd = t;
        if (rec) fr.push([p.x, p.y, cx, cy, fly ? 1 : 0, ...opps.flatMap(o => [o.x, o.y])]);
        if (!res && tEnd != null && t - tEnd >= TQ.tail) res = 'döndü';
      }
      return { res: res || 'döndü', t: tEnd, keep, fr }; },
    choose(p, a0, a1, opps, rnd) { const ok = self.ALAN_OKC(p, 'kontrol'), err = Math.max(0, 16 - ok) * .2, lam = Math.max(.3, Math.min(1.1, .3 + (ok - 5) * .08)), sd = Turn.shortDir(a0, a1), seen = opps.map(o => ({ ...o, x: o.x + (rnd() - .5) * err, y: o.y + (rnd() - .5) * err })), C = [{ mode: 'dolaş', dir: sd }, { mode: 'dolaş', dir: -sd }, { mode: 'fiske', dir: sd }, { mode: 'fiske', dir: -sd }];
      for (const c of C) { const s = Turn.simulate(p, a0, a1, c, seen, null); c.keep = s.keep; c.cost = (s.t ?? 200) * TQ.timeK + lam * (1 - s.keep) + (rnd() - .5) * Math.max(0, 16 - ok) * .02; }
      return C.sort((x, y) => x.cost - y.cost)[0]; } };
  /* v5 · tepki süreleri tek yerden (tik): her mekanik kendi türüyle; maç, yerleşim ve kafa aynı formülü kullanır. Konsantrasyon, Önsezi gibi yeni özellikler buraya bağlanacak.
     algi: rakibi ve topu gecikmeli görme · karar: topu alınca ilk karar · yeniTop: vuruştan sonra eski hedefte kalma · kovala: topa/prese koşmaya başlama · bosTop: sahipsiz topa tepki · uzanma: uzanmanın büyümeye başlaması */
  self.ALAN_TEPKI = (p, k) => { const S = (window.AlanShape && window.AlanShape.S) || {};
    switch (k) { case 'algi': return (S.lag0 ?? 10) - (self.ALAN_YET.acik(5) ? self.ALAN_Y(p, 'algiGecikme') : self.ALAN_OZ(p, 'tepki')) * (S.lagOk ?? .4); /* v5 · yetenek Aşama 5: algı gecikmesi Tepki 60, Okuma 40; karar süresi Karar 50, Tepki 30, Soğukkanlılık 20; yeni topa tepki algı gecikmesinden (Deney: self.ALAN_YET_ZIHIN_ESKI) */ case 'karar': return 9 - (self.ALAN_YET.acik(5) ? self.ALAN_Y(p, 'kararSure') : self.ALAN_OZ(p, 'tepki')) * .4; case 'yeniTop': return Q.tepki0 - (self.ALAN_YET.acik(5) ? self.ALAN_OKY(p, 'tepki', 'algiGecikme') : self.ALAN_OKC(p, 'tepki')) * Q.tepkiOk; case 'kovala': return 14 - (self.ALAN_YET.acik(6) ? self.ALAN_Y(p, 'presBasla') : self.ALAN_OZ(p, 'tepki')) * (S.react ?? .5); /* v5 · yetenek Aşama 6: prese ve boş topa koşmaya başlamak Tepki 40, Okuma 30, Çalışkanlık 30 (Deney: self.ALAN_YET_TOPSUZ_ESKI) */ case 'bosTop': return (S.reactLoose ?? 6) - (self.ALAN_YET.acik(6) ? self.ALAN_Y(p, 'presBasla') : self.ALAN_OZ(p, 'tepki')) * (S.reactLooseOk ?? .25); case 'uzanma': return Q.reachReact - (self.ALAN_YET.acik(3) ? self.ALAN_OKY(p, 'tepki', 'uzanmaBasla') : self.ALAN_OKC(p, 'tepki')) * Q.reachReactOk; /* v5 · yetenek Aşama 3: Okuma 50, Tepki 50 (Deney: self.ALAN_YET_TEKNIK_ESKI) */ } return 14 - self.ALAN_OZ(p, 'tepki') * .5; };
  window.AlanCore = { tekU, hyp, pack, rollTo, oppReach, menzil, Turn, touchCap, bekRe, bekHar, W, H, KR, MOUTH, DIREK, direk, sahaIci, P, Q, bekR, infl, K, makeGrid, gridS, chgMul, makeBall, stepBall, predict, solveLaunch, ctrlP, contact, stepAll, thruP };
})();

// v5 · Çekirdek gövdelerin içinden geçemez (süpürmeli çarpışma): bu tikteki yol (x0,y0)→(b.x,b.y) boyunca ilk değdiği gövdede durur.
// Kontrol edemediği gövde (az önce dokunan, cds'de) Çekirdeği yanından kaçırır (sekmez, yüzeyden kayar); diğerlerinden seker.
window.AlanCore.govde = function (b, x0, y0, list, skip, R, E) { const sx = b.x - x0, sy = b.y - y0, L2 = sx * sx + sy * sy; let hit = null, ht = 2;
  for (const p of list) { if (skip && skip(p)) continue; const fx = x0 - p.x, fy = y0 - p.y, c0 = fx * fx + fy * fy - R * R; let t;
    if (c0 <= 0) t = 0; else { if (L2 < 1e-9) continue; const bb = fx * sx + fy * sy, disc = bb * bb - L2 * c0; if (disc < 0) continue; t = (-bb - Math.sqrt(disc)) / L2; if (t < 0 || t > 1) continue; }
    if (t < ht) { ht = t; hit = p; } }
  if (!hit) return null; let px = x0 + sx * ht, py = y0 + sy * ht, nx = px - hit.x, ny = py - hit.y, nl = Math.hypot(nx, ny); if (nl < 1e-4) { const vl = Math.hypot(b.vx, b.vy) || 1; nx = -b.vx / vl; ny = -b.vy / vl; nl = 1; } nx /= nl; ny /= nl;
  b.x = hit.x + nx * R; b.y = hit.y + ny * R; window.AlanCore.sahaIci(b); const pvx = hit.vx || 0, pvy = hit.vy || 0, vn = (b.vx - pvx) * nx + (b.vy - pvy) * ny, kac = b.cds && (self.ALAN_GOVDE_CD_ESKI ? b.cds.has(hit) : (b.cds.get(hit) || 0) > b.t); /* v5 · topu kontrol edemeyen oyuncunun gövdesinden top yalnız dokunuş bekleme süresince kayar (sonra herkes gibi seker); önce o topun ömrü boyunca hiç sekmiyordu. Deney: self.ALAN_GOVDE_CD_ESKI */ /* göreli hız: oyuncu Çekirdeğe yürürse onu iter */
  if (vn < 0) { if (kac) { b.vx -= vn * nx; b.vy -= vn * ny; } else { b.vx -= (1 + E) * vn * nx; b.vy -= (1 + E) * vn * ny; b.defl = true; } } return hit; };
// oyuncular hareket ettikten sonra: boştaki Çekirdeğin içine giren gövde onu önüne iter (oyuncu topun içinden yürüyemez)
window.AlanCore.govdeIt = function (b, list, skip, R) { for (const p of list) { if (skip && skip(p)) continue; const dx = b.x - p.x, dy = b.y - p.y, d = Math.hypot(dx, dy); if (d >= R) continue; const vl = Math.hypot(p.vx || 0, p.vy || 0), nx = d > 1e-3 ? dx / d : (vl > 1e-3 ? p.vx / vl : 1), ny = d > 1e-3 ? dy / d : (vl > 1e-3 ? p.vy / vl : 0); b.x = p.x + nx * R; b.y = p.y + ny * R; window.AlanCore.sahaIci(b); const vn = (b.vx - (p.vx || 0)) * nx + (b.vy - (p.vy || 0)) * ny; if (vn < 0) { b.vx -= vn * nx; b.vy -= vn * ny; } } };
