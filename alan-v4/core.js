// Okuma kanalları (sadece ölçüm için): ALAN_OKX = { eq: [kanallar], only: kanal, mid } verilirse, o kanallarda herkes mid Okuma'yla oynar. Verilmezse etkisiz.
self.ALAN_OKC = function (p, ch) { const v = (p && p.a && p.a.okuma) ?? 10, E = self.ALAN_OKX; if (!E) return v; if (E.only) return E.only === ch ? v : E.mid; return E.eq && E.eq.includes(ch) ? E.mid : v; };
(function () {
  // Math.hypot ile bit bit aynı sonucu veren iki argümanlı sürüm (V8'in kendi algoritması: en büyüğe bölüp karelerin toplamı); yerleşik çağrıdan ~5 kat hızlı.
  const hyp = (x, y) => { x = x < 0 ? -x : x; y = y < 0 ? -y : y; if (x === Infinity || y === Infinity) return Infinity; const m = x > y ? x : y; if (m !== m || x !== x || y !== y) return NaN; if (m === 0) return 0; const a = x / m, b = y / m; return Math.sqrt(a * a + b * b) * m; };
  const W = 100, H = 50, KR = 5.5, MOUTH = 2.75; // Kuyu: gövde yarıçapı ve ağzın yarı genişliği (ağız 5,5 birim)
  // Çekirdek fiziği parametreleri. Hepsi deneme sahasından ayarlanabilir.
  const P = {
    fric: .010,     // temel sürtünme (tik başına hız oranı)
    roll: .003,     // sabit yuvarlanma yavaşlaması (birim/tik); Çekirdeğin gerçekten durmasını sağlar
    oppDrag: .045,  // rakip alanında ek sürtünme (alan gücüyle çarpılır)
    pierce: 1,      // şarjın rakip direncini ne kadar azalttığı (1 = tam şarj direnci sıfırlar)
    chgV: .9,       // şarjın başlangıç hızına katkısı: v0 × (1 - chgV/2 + chgV × şarj)
    ownFlow: .4,    // kendi alanının sürtünmeyi ne kadar azalttığı
    attn: .003,     // yolda enerji kaybı (birim başına)
    drain: .06,     // rakip alanında enerji kaybı (birim × alan gücü)
    deadRamp: .004, // sönünce her tik eklenen sürtünme (yumuşak fren)
    deadMax: .06,   // sönme freninin tavanı
    rest: .72,      // kenar sekmesinde kalan hız oranı
    wallLoss: .08,  // kenara çarpınca kaybedilen enerji
    e0: 1
  };
  function infl(p, x, y) {
    const dx = x - p.x, dy = y - p.y, R = p.R || 8; if (dx * dx + dy * dy > R * R * 9) return 0; let q;
    if (p.cone) { const al = dx * p.fx + dy * p.fy, pe = -dx * p.fy + dy * p.fx, a = al > 0 ? al / (R * 1.4) : al / (R * .6); q = a * a + (pe / R) ** 2; }
    else q = (dx * dx + dy * dy) / (R * R);
    return (p.D ?? 1) * Math.exp(-q * 1.2);
  }
  // Paketlenmiş alan: oyuncular bir yörünge boyunca kımıldamıyorsa (predict, hız arama) alan aynı formülle, aynı sırayla düz bir diziden okunur.
  // Sonuç bit bit aynıdır; sadece her adımda farklı biçimli oyuncu nesnelerini okumanın maliyeti kalkar. Önbellekli (_kc) ve ızgaralı (_g) kaynaklar olduğu gibi kalır.
  function pack(src) { if (src.pk || src._kc || src._g) return src; const n = src.length, a = new Float64Array(n * 8); for (let i = 0; i < n; i++) { const p = src[i], o = i * 8; a[o] = p.x; a[o + 1] = p.y; a[o + 2] = p.R || 8; a[o + 3] = p.D ?? 1; a[o + 4] = p.cone ? 1 : 0; a[o + 5] = p.fx; a[o + 6] = p.fy; a[o + 7] = p.team; } return { pk: a, n }; }
  function Kpk(s, x, y, team) { const a = s.pk; let v = 0; for (let i = 0, o = 0; i < s.n; i++, o += 8) { const dx = x - a[o], dy = y - a[o + 1], R = a[o + 2]; let f = 0; if (!(dx * dx + dy * dy > R * R * 9)) { let q; if (a[o + 4]) { const fx = a[o + 5], fy = a[o + 6], al = dx * fx + dy * fy, pe = -dx * fy + dy * fx, aa = al > 0 ? al / (R * 1.4) : al / (R * .6); q = aa * aa + (pe / R) ** 2; } else q = (dx * dx + dy * dy) / (R * R); f = a[o + 3] * Math.exp(-q * 1.2); } v += (a[o + 7] === team ? 1 : -1) * f; } return Math.tanh(v); }
  // Kararın önbellekli alanı (_kc): yarım birimlik hücreler, hücrenin ilk sorulduğu noktadaki değer saklanır. Saha içindeki hücreler Map yerine düz dizide (aynı hücre eşlemesi, aynı ilk-değer kuralı).
  function KcT(src, c, x, y, team) { let T = src._kt; if (!T) T = src._kt = { v: new Float64Array(201 * 101), f: new Uint8Array(201 * 101) }; let v; if (T.f[c]) v = T.v[c]; else { v = 0; for (const p of src) v += (p.team === 0 ? 1 : -1) * infl(p, x, y); T.v[c] = v; T.f[c] = 1; } return Math.tanh(team === 0 ? v : -v); }
  function K(src, x, y, team) { if (src.pk) return Kpk(src, x, y, team); if (src._kc) { const i = (x * 2 + .5) | 0, j = (y * 2 + .5) | 0; if (i >= 0 && i <= 200 && j >= 0 && j <= 100) { return KcT(src, i * 101 + j, x, y, team); } const key = i * 1000 + j; let v = src._kc.get(key); if (v === undefined) { v = 0; for (const p of src) v += (p.team === 0 ? 1 : -1) * infl(p, x, y); src._kc.set(key, v); } return Math.tanh(team === 0 ? v : -v); } if (src._g) return Math.tanh(gridS(src._g, x, y) * (team === 0 ? 1 : -1)); let s = 0; for (const p of src) s += (p.team === team ? 1 : -1) * infl(p, x, y); return Math.tanh(s); }
  // hızlı alan: 1 birimlik ızgarada Mavi(+)/Turuncu(-) toplamı, çift doğrusal okunur (sadece kafadaki simülasyon için)
  function makeGrid(src) { const g = new Float32Array(101 * 51); for (let j = 0; j <= 50; j++) for (let i = 0; i <= 100; i++) { let s = 0; for (const p of src) { const dx = i - p.x, dy = j - p.y; if (dx * dx + dy * dy > 400) continue; s += (p.team === 0 ? 1 : -1) * infl(p, i, j); } g[j * 101 + i] = s; } return g; }
  function gridS(g, x, y) { x = Math.max(0, Math.min(99.999, x)); y = Math.max(0, Math.min(49.999, y)); const i = x | 0, j = y | 0, fx = x - i, fy = y - j, a = g[j * 101 + i], b = g[j * 101 + i + 1], c = g[(j + 1) * 101 + i], d = g[(j + 1) * 101 + i + 1]; return a + (b - a) * fx + (c - a) * fy + (a - b - c + d) * fx * fy; }
  function chgMul(ch, p) { p = p || P; return 1 - p.chgV / 2 + p.chgV * ch; }
  function makeBall(o) { return Object.assign({ x: W / 2, y: H / 2, vx: 0, vy: 0, e: P.e0, ch: .5, team: 0, alive: true, dead: 0, t: 0, dist: 0, walls: 0, done: null, log: [] }, o); }
  function ev(b, k, txt) { b.log.push({ t: b.t, k, txt }); }
  function stepBall(b, src, p) {
    p = p || P; if (b.done) return b;
    // Saha iki boyutlu: yükseklik yok. Çekirdek hep yerde, hep alanların içinden geçer; her oyuncu her an ona dokunabilir.
    const k = K(src, b.x, b.y, b.team), sp0 = hyp(b.vx, b.vy), res = 1 - p.pierce * b.ch;
    let drag = p.fric;
    if (k < 0) drag += -k * p.oppDrag * res; else drag *= 1 - p.ownFlow * k;
    if (b.alive) { b.e -= sp0 * p.attn + (k < 0 ? -k * sp0 * p.drain * res : 0); if (b.e <= 0) { b.e = 0; b.alive = false; b.dead = 0; ev(b, 'son', 'Çekirdek söndü · boşta'); } }
    else { b.dead++; drag += Math.min(p.deadMax, p.deadRamp * b.dead); }
    const f = 1 - drag; b.vx *= f; b.vy *= f;
    const s1 = hyp(b.vx, b.vy); if (s1 > 0) { const r = Math.max(0, s1 - p.roll) / s1; b.vx *= r; b.vy *= r; }
    let nx = b.x + b.vx, ny = b.y + b.vy;
    for (const ex of [0, W]) {
      const hit = ex === 0 ? nx <= .5 : nx >= W - .5; if (!hit) continue;
      const wx = ex === 0 ? .5 : W - .5, tt = Math.abs(b.vx) > 1e-9 ? (wx - b.x) / b.vx : 0, yy = b.y + b.vy * Math.max(0, Math.min(1, tt));
      if (b.alive && Math.abs(yy - H / 2) <= MOUTH) { b.x = wx; b.y = yy; b.done = ex === 0 ? 'kuyu-sol' : 'kuyu-sag'; ev(b, 'kuyu', `KUYU · ${ex === 0 ? 'sol' : 'sağ'} Kuyu'ya girdi`); return b; }
      nx = ex === 0 ? 1 - nx : 2 * W - 1 - nx; b.vx = -b.vx * p.rest; b.vy *= p.rest; b.walls++; if (b.alive) b.e = Math.max(0, b.e - p.wallLoss); ev(b, 'kenar', `Kenardan sekti (${ex === 0 ? 'sol' : 'sağ'} uç, Kuyu'nun dışı)`);
    }
    if (ny < .5 || ny > H - .5) { ny = ny < .5 ? 1 - ny : 2 * H - 1 - ny; b.vy = -b.vy * p.rest; b.vx *= p.rest; b.walls++; if (b.alive) b.e = Math.max(0, b.e - p.wallLoss); ev(b, 'kenar', `Kenardan sekti (${ny < H / 2 ? 'üst' : 'alt'})`); }
    b.dist += hyp(nx - b.x, ny - b.y); b.x = nx; b.y = ny; b.t++;
    if (hyp(b.vx, b.vy) < .015) { b.vx = 0; b.vy = 0; b.done = 'durdu'; ev(b, 'dur', b.alive ? 'Durdu' : 'Sönmüş hâlde durdu'); }
    return b;
  }
  // Hız arama için aynı fizik adımı, nesnesiz: stepBall'un birebir aynı işlemleri aynı sırayla, yerel değişkenlerle (kayıt ve mesafe sayacı tutulmaz, sonucu etkilemezler).
  // Çekirdek (hx,hy)'den L uzaklığa vardığında hızını döndürür; varamazsa -1. Pas hızı aramasında binlerce kez çağrılır.
  function rollTo(x, y, vx, vy, team, ch, src, hx, hy, L, maxT) {
    const p = P; let e = p.e0, alive = true, dead = 0;
    for (let t = 0; t < maxT; t++) {
      const k = K(src, x, y, team), sp0 = hyp(vx, vy), res = 1 - p.pierce * ch;
      let drag = p.fric;
      if (k < 0) drag += -k * p.oppDrag * res; else drag *= 1 - p.ownFlow * k;
      if (alive) { e -= sp0 * p.attn + (k < 0 ? -k * sp0 * p.drain * res : 0); if (e <= 0) { e = 0; alive = false; dead = 0; } }
      else { dead++; drag += Math.min(p.deadMax, p.deadRamp * dead); }
      const f = 1 - drag; vx *= f; vy *= f;
      const s1 = hyp(vx, vy); if (s1 > 0) { const r = Math.max(0, s1 - p.roll) / s1; vx *= r; vy *= r; }
      let nx = x + vx, ny = y + vy, done = false;
      if (nx <= .5) { const wx = .5, tt = Math.abs(vx) > 1e-9 ? (wx - x) / vx : 0, yy = y + vy * Math.max(0, Math.min(1, tt));
        if (alive && Math.abs(yy - H / 2) <= MOUTH) { x = wx; y = yy; done = true; }
        else { nx = 1 - nx; vx = -vx * p.rest; vy *= p.rest; if (alive) e = Math.max(0, e - p.wallLoss); } }
      if (!done && nx >= W - .5) { const wx = W - .5, tt = Math.abs(vx) > 1e-9 ? (wx - x) / vx : 0, yy = y + vy * Math.max(0, Math.min(1, tt));
        if (alive && Math.abs(yy - H / 2) <= MOUTH) { x = wx; y = yy; done = true; }
        else { nx = 2 * W - 1 - nx; vx = -vx * p.rest; vy *= p.rest; if (alive) e = Math.max(0, e - p.wallLoss); } }
      if (!done) {
        if (ny < .5 || ny > H - .5) { ny = ny < .5 ? 1 - ny : 2 * H - 1 - ny; vy = -vy * p.rest; vx *= p.rest; if (alive) e = Math.max(0, e - p.wallLoss); }
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
    src = pack(src); p = p || P; const b = clone(b0), pts = [{ x: b.x, y: b.y }], n = maxT || 900, team = b.team, ch = b.ch;
    let x = b.x, y = b.y, vx = b.vx, vy = b.vy, e = b.e, alive = b.alive, dead = b.dead, t = b.t, dist = b.dist, walls = b.walls, done = b.done;
    for (let i = 0; i < n && !done; i++) {
      const k = K(src, x, y, team), sp0 = hyp(vx, vy), res = 1 - p.pierce * ch;
      let drag = p.fric;
      if (k < 0) drag += -k * p.oppDrag * res; else drag *= 1 - p.ownFlow * k;
      if (alive) { e -= sp0 * p.attn + (k < 0 ? -k * sp0 * p.drain * res : 0); if (e <= 0) { e = 0; alive = false; dead = 0; b.log.push({ t, k: 'son', txt: 'Çekirdek söndü · boşta' }); } }
      else { dead++; drag += Math.min(p.deadMax, p.deadRamp * dead); }
      const f = 1 - drag; vx *= f; vy *= f;
      const s1 = hyp(vx, vy); if (s1 > 0) { const r = Math.max(0, s1 - p.roll) / s1; vx *= r; vy *= r; }
      let nx = x + vx, ny = y + vy;
      if (nx <= .5) { const wx = .5, tt = Math.abs(vx) > 1e-9 ? (wx - x) / vx : 0, yy = y + vy * Math.max(0, Math.min(1, tt));
        if (alive && Math.abs(yy - H / 2) <= MOUTH) { x = wx; y = yy; done = 'kuyu-sol'; b.log.push({ t, k: 'kuyu', txt: `KUYU · sol Kuyu'ya girdi` }); }
        else { nx = 1 - nx; vx = -vx * p.rest; vy *= p.rest; walls++; if (alive) e = Math.max(0, e - p.wallLoss); b.log.push({ t, k: 'kenar', txt: `Kenardan sekti (sol uç, Kuyu'nun dışı)` }); } }
      if (!done && nx >= W - .5) { const wx = W - .5, tt = Math.abs(vx) > 1e-9 ? (wx - x) / vx : 0, yy = y + vy * Math.max(0, Math.min(1, tt));
        if (alive && Math.abs(yy - H / 2) <= MOUTH) { x = wx; y = yy; done = 'kuyu-sag'; b.log.push({ t, k: 'kuyu', txt: `KUYU · sağ Kuyu'ya girdi` }); }
        else { nx = 2 * W - 1 - nx; vx = -vx * p.rest; vy *= p.rest; walls++; if (alive) e = Math.max(0, e - p.wallLoss); b.log.push({ t, k: 'kenar', txt: `Kenardan sekti (sağ uç, Kuyu'nun dışı)` }); } }
      if (!done) {
        if (ny < .5 || ny > H - .5) { ny = ny < .5 ? 1 - ny : 2 * H - 1 - ny; vy = -vy * p.rest; vx *= p.rest; walls++; if (alive) e = Math.max(0, e - p.wallLoss); b.log.push({ t, k: 'kenar', txt: `Kenardan sekti (${ny < H / 2 ? 'üst' : 'alt'})` }); }
        dist += hyp(nx - x, ny - y); x = nx; y = ny; t++;
        if (hyp(vx, vy) < .015) { vx = 0; vy = 0; done = 'durdu'; b.log.push({ t, k: 'dur', txt: alive ? 'Durdu' : 'Sönmüş hâlde durdu' }); }
      }
      pts.push({ x, y, e, sp: hyp(vx, vy) });
    }
    b.x = x; b.y = y; b.vx = vx; b.vy = vy; b.e = e; b.alive = alive; b.dead = dead; b.t = t; b.dist = dist; b.walls = walls; b.done = done;
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
    reachBody: .9, reachReact: 6, reachReactOk: .25, reachGrow: .12, // rakibin uzanması: gövde, tepki süresi (tik), Okuma etkisi, büyüme hızı
    reach: 1.6,     // temel kontrol menzili (birim)
    body: 1.15,     // oyuncunun gövdesi + Çekirdeğin yarıçapı: Çekirdek gövdenin içine giremez, tutulan Çekirdek gövdenin kenarında durur
    reachBek: .6,   // (eski) Bekçi ek menzili
    bekThru: .25, bekBody: .9, bekDive: 2.2, bekDiveV: .22, bekReact: 12, bekReactOk: .5, // Bekçi: gövde, en fazla uzanma, uzanma hızı (birim/tik), tepki süresi
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
  function bekR(b, p, q) { q = q || Q; const a = p.a || {}, re = q.bekReact - self.ALAN_OKC(p, 'bekci') * q.bekReactOk; return q.bekBody + Math.min(q.bekDive, Math.max(0, (b.t || 0) - re) * (q.bekDiveV + ((a.hiz ?? 10) - 10) * .01)); }
  // Dokunuşun fiziği: Çekirdeğin oyuncuya göre hızı (oyuncu ona doğru koşuyorsa artar, onunla aynı yöne gidiyorsa azalır) ve oyuncunun o an söndürebileceği hız.
  function touchCap(b, p, src, q) { q = q || Q; const mate = p.team === b.team, a = p.a || {}, sk = p.role === 'Bekçi' || mate ? (a.tutus ?? 10) : ((a.kesme ?? 10) + (a.tutus ?? 10)) / 2;
    let pr = 0; for (const o of src) if (o.team !== p.team) pr += infl(o, p.x, p.y);
    const cap = q.cap0 + (sk - 10) * q.capSk - (b.alive ? b.ch * q.capCh : 0) - q.capPr * (1 - Math.exp(-Math.max(0, pr - .4) * q.pressK / q.pressMax)), vrx = b.vx - (p.vx || 0), vry = b.vy - (p.vy || 0);
    return { cap: Math.max(.08, cap), vr: hyp(vrx, vry), vrx, vry, s: q.capS0 + (20 - sk) * q.capSsk, sk }; }
  // Pası kesmek için tepki süresi: rakip, son vuruştan hemen sonra yalnızca gövdesine çarpan Çekirdeği keser; uzanabilmek için tepki vermesi gerekir (Okuma kısaltır). Hızlı ve tekte oynanan pas bu yüzden presçinin yanından geçebilir.
  function oppReach(b, p, q, R) { if (p.role === 'Bekçi' || p.team === b.team) return R; q = q || Q; return Math.min(R, q.reachBody + Math.max(0, (b.t || 0) - (q.reachReact - self.ALAN_OKC(p, 'tepki') * q.reachReactOk)) * q.reachGrow); }
  function ctrlP(b, p, src, q) { const t = touchCap(b, p, src, q); return Math.max(.02, Math.min(.99, 1 / (1 + Math.exp((t.vr - t.cap) / t.s)))); }
  // Bir tikte Çekirdeğin geçtiği parçayı kontrol menzilinde kesen oyuncu var mı? Varsa tutma/sekme. Dönüş: null | {p, took:true} | {p, took:false}
  function contact(b, src, q, rnd, ox, oy) {
    q = q || Q; rnd = rnd || Math.random; const sx = b.x - ox, sy = b.y - oy, sl = sx * sx + sy * sy; let best = null, bd = 1e9;
    for (const p of src) {
      if (p.noTouch || (b.cds && (b.cds.get(p) || 0) > b.t)) continue; if (p === b.from && b.t < 8) continue; if (b.recv && p.team === b.team && p !== b.recv && !b.defl) continue; // arkadaş başkasına giden pası bırakır
      const t0 = sl > 0 ? ((p.x - ox) * sx + (p.y - oy) * sy) / sl : 1, tt = Math.max(0, Math.min(1, t0)), dd = hyp(ox + sx * tt - p.x, oy + sy * tt - p.y), R = oppReach(b, p, q, (p.role === 'Bekçi' ? bekR(b, p, q) : q.reach + (((p.a && p.a.yogunluk) ?? 10) - 10) * .04) * (t0 < 0 ? q.awayK : 1)); // kendisinden uzaklaşan Çekirdeğe ancak dibindeyse yetişir
      if (dd < R && dd < bd) { bd = dd; best = p; }
    }
    if (!best) return null;
    const T = touchCap(b, best, src, q), P = Math.max(.02, Math.min(.99, 1 / (1 + Math.exp((T.vr - T.cap) / T.s)))), u = rnd(), capE = T.cap + T.s * Math.log(u / (1 - u)); // bu dokunuşun gerçek kapasitesi (lojistik gürültü: ortalaması cap)
    if (T.vr <= capE) { b.done = 'tutuldu'; b.holder = best; { const gx = best.team === 0 ? 100 : 0, ix = best.tx != null ? best.tx - best.x : gx - best.x, iy = best.ty != null ? best.ty - best.y : 25 - best.y, il = hyp(ix, iy) || 1; b.x = best.x + ix / il * q.body; b.y = best.y + iy / il * q.body; } b.vx = b.vy = 0; ev(b, 'tut', `${best.name || (best.team === b.team ? 'Arkadaş' : 'Rakip')} tuttu (ihtimal %${Math.round(P * 100)})`); return { p: best, took: true, P }; }
    const mateT = best.team === b.team, sp0 = hyp(b.vx, b.vy) || 1e-6; if (!b.cds) b.cds = new Map(); b.cds.set(best, b.t + Math.max(q.cdMin, Math.round(q.cd * Math.min(1, sp0 / q.cdV)))); // yavaş Çekirdeği kaçıran hemen toparlar; hızlı olan zaten uzaklaşır
    // Söndürülemeyen hız gövdeden seker: çarpma noktasında gövdeye dik bileşen geri döner (kısmen), teğet bileşen sürer. Sıyırıp geçen Çekirdek az sapar, tam ortadan çarpan geri döner.
    // Oyuncu kalan hızı istediği yöne (arkadaşsa hedefine, rakipse kendi hücum yönüne) bir miktar çevirebilir; Tutuş ne kadar çevirebileceğini belirler. Gövdeyi delme (çok hızlı Çekirdek) ayrı.
    const thru = !mateT && rnd() < thruP(b, best, q);
    if (thru) { const an = Math.atan2(b.vy, b.vx) + (rnd() - .5) * .5; b.vx = Math.cos(an) * sp0 * q.thruKeep; b.vy = Math.sin(an) * sp0 * q.thruKeep; }
    else { const tt = sl > 0 ? Math.max(0, Math.min(1, ((best.x - ox) * sx + (best.y - oy) * sy) / sl)) : 1, cx = ox + sx * tt, cy = oy + sy * tt; let nx = cx - best.x, ny = cy - best.y, nl = hyp(nx, ny); if (nl < 1e-3) { nx = -T.vrx; ny = -T.vry; nl = hyp(nx, ny) || 1; } nx /= nl; ny /= nl;
      const left = Math.max(0, T.vr - Math.max(0, capE)), ux = T.vrx / (T.vr || 1), uy = T.vry / (T.vr || 1), dn = ux * nx + uy * ny; let ox2 = ux, oy2 = uy; if (dn < 0) { ox2 = ux - (1 + q.rest2) * dn * nx; oy2 = uy - (1 + q.rest2) * dn * ny; } const ol = hyp(ox2, oy2) || 1; ox2 /= ol; oy2 /= ol;
      const gxA = best.team === 0 ? 100 : 0, ix = mateT && best.tx != null ? best.tx - best.x : gxA - best.x, iy = mateT && best.ty != null ? best.ty - best.y : 25 - best.y, il = hyp(ix, iy) || 1, st = q.steer * Math.max(0, Math.min(1, capE / T.vr)) * (.5 + (T.sk - 10) * .04), jx = ox2 * (1 - st) + ix / il * st, jy = oy2 * (1 - st) + iy / il * st, jl = hyp(jx, jy) || 1, ns = (rnd() - .5) * (20 - T.sk) * .04, cs = Math.cos(ns), sn = Math.sin(ns), fx = jx / jl, fy = jy / jl;
      b.vx = (best.vx || 0) + (fx * cs - fy * sn) * left; b.vy = (best.vy || 0) + (fx * sn + fy * cs) * left; b.x = best.x + nx * (q.reach + .05); b.y = best.y + ny * (q.reach + .05); } if (b.alive) b.e = Math.max(0, b.e - q.deflE); b.defl = (b.defl || 0) + 1; ev(b, 'sek', `${best.name || (best.team === b.team ? 'Arkadaş' : 'Rakip')} dokundu, sekti (tutma ihtimali %${Math.round(P * 100)})`);
    return { p: best, took: false, P };
  }
  function thruP(b, p, q) { q = q || Q; if (p.team === b.team) return 0; const sp = hyp(b.vx, b.vy); return Math.max(0, Math.min(q.thruMax * (p.role === 'Bekçi' ? q.bekThru : 1), (sp - q.thruV) * q.thruK - (((p.a && p.a.kesme) ?? 10) - 10) * .02)); }
  function stepAll(b, src, p, q, rnd) { if (b.done) return null; const ox = b.x, oy = b.y; stepBall(b, src, p); if (b.done) return null; return contact(b, src, q, rnd, ox, oy); }
  // Dönüş: Çekirdek gövdenin içinden geçemez, kenarında bir açıda durur. Öbür yana geçmek için ya gövdenin etrafından dolaşır (Sürme hızında; taşıyıcı hızlandıkça yavaşlar), ya da fiskeyle bir an gövdeden ayrılır (hızlı ama boşta).
  // Rakip Çekirdeğe ancak gövde araya girmiyorsa uzanabilir. Yolu oyuncu kafasında seçer: süre + (Okuma ile görülen) açıkta kalma riski.
  const TQ = { w0: .1, wSk: .008, wV: .6, vRef: .25, flickW: .4, timeK: .003, react: 14, reactOk: .5, grab: 0, rivV: .2, pHold: .3, pHoldSk: .025, cdHold: 12, pLoose: .55, cdLoose: 3, tail: 6, shield: .8, minD: 1.7 };
  const wrapA = a => { while (a > Math.PI) a -= 2 * Math.PI; while (a < -Math.PI) a += 2 * Math.PI; return a; };
  const Turn = { TQ,
    omega(p) { const a = p.a || {}, v = hyp(p.vx || 0, p.vy || 0); return Math.max(.03, (TQ.w0 + ((a.surme ?? 10) - 10) * TQ.wSk) * (1 - TQ.wV * Math.min(1, v / TQ.vRef))); },
    shortDir(a0, a1) { return wrapA(a1 - a0) >= 0 ? 1 : -1; },
    arc(a0, a1, dir) { let d = wrapA(a1 - a0); if (dir > 0 && d < 0) d += 2 * Math.PI; if (dir < 0 && d > 0) d -= 2 * Math.PI; return Math.abs(d); },
    shielded(p, o, cx, cy) { const sx = cx - o.x, sy = cy - o.y, L2 = sx * sx + sy * sy || 1, t = Math.max(0, Math.min(1, ((p.x - o.x) * sx + (p.y - o.y) * sy) / L2)); return hyp(o.x + sx * t - p.x, o.y + sy * t - p.y) < TQ.shield; },
    flickMiss(p) { return Math.max(.02, .12 - (((p.a || {}).surme ?? 10) - 10) * .015); },
    // Tek kural, iki kullanım: rnd verilirse gerçek dönüş oynanır; rnd null ise aynı dönüşün Çekirdeği koruma ihtimali (beklenen değer) hesaplanır. Oyuncu kafasında bunu oynatır.
    simulate(p0, a0, a1, plan, opps0, rnd, rec) {
      const p = { ...p0 }, opps = opps0.map(o => ({ ...o, cd: 0 })), body = Q.body, R = Q.reach, A = Turn.arc(a0, a1, plan.dir), sur = (p.a || {}).surme ?? 10, fr = [];
      let done = 0, keep = 1, res = null, tEnd = null, landed = false;
      for (let t = 0; t < 200 && !res; t++) {
        p.x += p.vx || 0; p.y += p.vy || 0; const w = plan.mode === 'fiske' ? TQ.flickW : Turn.omega(p); done += Math.min(w, A - done);
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
  window.AlanCore = { hyp, pack, rollTo, oppReach, Turn, touchCap, W, H, KR, MOUTH, P, Q, bekR, infl, K, makeGrid, gridS, chgMul, makeBall, stepBall, predict, solveLaunch, ctrlP, contact, stepAll, thruP };
})();
