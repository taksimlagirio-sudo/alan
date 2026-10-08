(function () {
  const W = 100, H = 50, KR = 5.5, MOUTH = 2.75; // Kuyu: gövde yarıçapı ve ağzın yarı genişliği (ağız 5,5 birim)
  // Çekirdek fiziği parametreleri. Hepsi deneme sahasından ayarlanabilir.
  const P = {
    airFric: .006, airGoal: 3, landK: .5, landScatter: .35, // aşırtma: hava direnci, Kuyu'ya inişte girebilmesi için en fazla kalan hava tiki, inişte kalan hız, inişte sapma
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
  function K(src, x, y, team) { if (src._kc) { const key = ((x * 2 + .5) | 0) * 1000 + ((y * 2 + .5) | 0); let v = src._kc.get(key); if (v === undefined) { v = 0; for (const p of src) v += (p.team === 0 ? 1 : -1) * infl(p, x, y); src._kc.set(key, v); } return Math.tanh(team === 0 ? v : -v); } if (src._g) return Math.tanh(gridS(src._g, x, y) * (team === 0 ? 1 : -1)); let s = 0; for (const p of src) s += (p.team === team ? 1 : -1) * infl(p, x, y); return Math.tanh(s); }
  // hızlı alan: 1 birimlik ızgarada Mavi(+)/Turuncu(-) toplamı, çift doğrusal okunur (sadece kafadaki simülasyon için)
  function makeGrid(src) { const g = new Float32Array(101 * 51); for (let j = 0; j <= 50; j++) for (let i = 0; i <= 100; i++) { let s = 0; for (const p of src) { const dx = i - p.x, dy = j - p.y; if (dx * dx + dy * dy > 400) continue; s += (p.team === 0 ? 1 : -1) * infl(p, i, j); } g[j * 101 + i] = s; } return g; }
  function gridS(g, x, y) { x = Math.max(0, Math.min(99.999, x)); y = Math.max(0, Math.min(49.999, y)); const i = x | 0, j = y | 0, fx = x - i, fy = y - j, a = g[j * 101 + i], b = g[j * 101 + i + 1], c = g[(j + 1) * 101 + i], d = g[(j + 1) * 101 + i + 1]; return a + (b - a) * fx + (c - a) * fy + (a - b - c + d) * fx * fy; }
  function chgMul(ch, p) { p = p || P; return 1 - p.chgV / 2 + p.chgV * ch; }
  function makeBall(o) { return Object.assign({ x: W / 2, y: H / 2, vx: 0, vy: 0, e: P.e0, ch: .5, team: 0, alive: true, dead: 0, t: 0, dist: 0, walls: 0, done: null, air: 0, log: [] }, o); }
  function ev(b, k, txt) { b.log.push({ t: b.t, k, txt }); }
  function stepBall(b, src, p) {
    p = p || P; if (b.done) return b;
    // Aşırtma: havadaki Çekirdek alanlardan etkilenmez, yuvarlanma sürtünmesi yoktur, sadece hava direnci; kimse dokunamaz. İnince hızının bir kısmını kaybeder ve biraz sapar.
    const air = b.air > 0, k = air ? 0 : K(src, b.x, b.y, b.team), sp0 = Math.hypot(b.vx, b.vy), res = 1 - p.pierce * b.ch;
    let drag = air ? p.airFric : p.fric;
    if (!air) { if (k < 0) drag += -k * p.oppDrag * res; else drag *= 1 - p.ownFlow * k; }
    if (b.alive) { b.e -= sp0 * p.attn + (k < 0 ? -k * sp0 * p.drain * res : 0); if (b.e <= 0) { b.e = 0; b.alive = false; b.dead = 0; ev(b, 'son', 'Çekirdek söndü · boşta'); } }
    else { b.dead++; drag += Math.min(p.deadMax, p.deadRamp * b.dead); }
    const f = 1 - drag; b.vx *= f; b.vy *= f;
    const s1 = Math.hypot(b.vx, b.vy); if (s1 > 0 && !air) { const r = Math.max(0, s1 - p.roll) / s1; b.vx *= r; b.vy *= r; }
    let nx = b.x + b.vx, ny = b.y + b.vy;
    for (const ex of [0, W]) {
      const hit = ex === 0 ? nx <= .5 : nx >= W - .5; if (!hit) continue;
      const wx = ex === 0 ? .5 : W - .5, tt = Math.abs(b.vx) > 1e-9 ? (wx - b.x) / b.vx : 0, yy = b.y + b.vy * Math.max(0, Math.min(1, tt));
      if (b.alive && Math.abs(yy - H / 2) <= MOUTH && !(b.air > p.airGoal)) { b.x = wx; b.y = yy; b.done = ex === 0 ? 'kuyu-sol' : 'kuyu-sag'; ev(b, 'kuyu', `KUYU · ${ex === 0 ? 'sol' : 'sağ'} Kuyu'ya girdi`); return b; }
      nx = ex === 0 ? 1 - nx : 2 * W - 1 - nx; b.vx = -b.vx * p.rest; b.vy *= p.rest; b.walls++; if (b.alive) b.e = Math.max(0, b.e - p.wallLoss); ev(b, 'kenar', `Kenardan sekti (${ex === 0 ? 'sol' : 'sağ'} uç, Kuyu'nun dışı)`);
    }
    if (ny < .5 || ny > H - .5) { ny = ny < .5 ? 1 - ny : 2 * H - 1 - ny; b.vy = -b.vy * p.rest; b.vx *= p.rest; b.walls++; if (b.alive) b.e = Math.max(0, b.e - p.wallLoss); ev(b, 'kenar', `Kenardan sekti (${ny < H / 2 ? 'üst' : 'alt'})`); }
    b.dist += Math.hypot(nx - b.x, ny - b.y); b.x = nx; b.y = ny; b.t++;
    if (air && --b.air === 0) { const sc = Math.sin(b.t * 12.9898 + b.x * 78.233) * 43758.5453, a = ((sc - Math.floor(sc)) - .5) * p.landScatter, ca = Math.cos(a), sa = Math.sin(a), vx = b.vx * ca - b.vy * sa, vy = b.vx * sa + b.vy * ca; b.vx = vx * p.landK; b.vy = vy * p.landK; b.landT = b.t; ev(b, 'iniş', 'Aşırtma yere indi'); }
    if (Math.hypot(b.vx, b.vy) < .015) { b.vx = 0; b.vy = 0; b.done = 'durdu'; ev(b, 'dur', b.alive ? 'Durdu' : 'Sönmüş hâlde durdu'); }
    return b;
  }
  function clone(b) { return Object.assign({}, b, { log: [] }); }
  function predict(b0, src, p, maxT) { const b = clone(b0); const pts = [{ x: b.x, y: b.y, air: b.air }]; for (let i = 0; i < (maxT || 900) && !b.done; i++) { stepBall(b, src, p); pts.push({ x: b.x, y: b.y, e: b.e, sp: Math.hypot(b.vx, b.vy), air: b.air }); } return { pts, end: b }; }
  // Hedefe pas: hedef noktaya `arrive` hızıyla ulaşan başlangıç hızını bul (alanlar dahil, düz hat varsayımı).
  function solveLaunch(b0, to, arrive, src, p, vmax) {
    const dx = to.x - b0.x, dy = to.y - b0.y, L = Math.hypot(dx, dy) || 1, ux = dx / L, uy = dy / L; vmax = vmax || 3;
    const speedAt = v0 => { const b = clone(b0); b.vx = ux * v0; b.vy = uy * v0; b.done = null; let last = 0; for (let i = 0; i < 900 && !b.done; i++) { stepBall(b, src, p); const d = (b.x - b0.x) * ux + (b.y - b0.y) * uy; last = Math.hypot(b.vx, b.vy); if (d >= L) return last; if (b.walls) return -1; } return -1; };
    let lo = .05, hi = vmax; if (speedAt(hi) < arrive) return { v0: hi, ok: false };
    for (let i = 0; i < 18; i++) { const mid = (lo + hi) / 2; if (speedAt(mid) < arrive) lo = mid; else hi = mid; }
    return { v0: hi, ok: true };
  }
  // Katman 2 · Karşılama. Oyuncu: {x,y,team,R,D, a:{tutus,kesme,yogunluk,hiz}, role}
  const Q = {
    reach: 1.6,     // temel kontrol menzili (birim)
    reachBek: .6,   // (eski) Bekçi ek menzili
    airTouch: 2, lobCtrl: .18, lobT: 5, // aşırtma: inişe bu kadar tik kala dokunulabilir; inişte karşılama cezası ve süresi
    bekThru: .25, bekBody: .9, bekDive: 2.2, bekDiveV: .22, bekReact: 12, bekReactOk: .5, // Bekçi: gövde, en fazla uzanma, uzanma hızı (birim/tik), tepki süresi
    ctrl0: 1.0,     // sıfır hızda, ortalama Tutuş'la tutma ihtimali tabanı
    ctrlV: .30,     // hızın tutmayı zorlaştırması (birim/tik başına)
    ctrlCh: .35,    // şarjın tutmayı zorlaştırması (canlı Çekirdek)
    skill: .035,    // Tutuş/Kesme puanı başına etki
    pressK: .35,    // üstündeki rakip alanının cezası (başlangıç eğimi)
    pressMax: .45,  // baskı cezasının tavanı: kalabalıkta bile Çekirdeği karşılamak mümkündür, sadece zordur
    deflA: 1.6,     // sekme açısı (radyan, ±yarısı)
    deflK: .5,      // sekmede kalan hız (rakip dokunuşu)
    deflMate: .3,   // arkadaşın kontrol edemediği Çekirdek yakına düşer: deflK × bu
    deflE: .15,     // sekmede kaybedilen enerji
    cd: 12, cdMin: 3, cdV: .9,         // aynı oyuncu tekrar deneyemeden geçen tik
    awayK: .45,     // kendisinden uzaklaşan Çekirdek için menzil oranı
    thruV: .9, thruK: .6, thruMax: .55, thruKeep: .6 // hızlı Çekirdeğin gövdeyi delmesi: hız eşiği, eğim, tavan, kalan hız
  };
  // Bekçi'nin menzili: gövde + uzanma. Uzanmak için zaman gerekir: Çekirdek yola çıktıktan sonra tepki süresi (Okuma) geçince menzil büyür (Hız hızlandırır).
  function bekR(b, p, q) { q = q || Q; const a = p.a || {}, re = q.bekReact - (a.okuma ?? 10) * q.bekReactOk; return q.bekBody + Math.min(q.bekDive, Math.max(0, (b.t || 0) - re) * (q.bekDiveV + ((a.hiz ?? 10) - 10) * .01)); }
  function ctrlP(b, p, src, q) {
    q = q || Q; const sp = Math.hypot(b.vx, b.vy), mate = p.team === b.team, a = p.a || {}, sk = p.role === 'Bekçi' || mate ? (a.tutus ?? 10) : ((a.kesme ?? 10) + (a.tutus ?? 10)) / 2;
    let pr = 0; for (const o of src) if (o.team !== p.team) pr += infl(o, p.x, p.y);
    const lob = (b.air > 0 || (b.landT != null && b.t - b.landT < q.lobT)) ? q.lobCtrl : 0;
    return Math.max(.03, Math.min(.98, q.ctrl0 - lob - sp * q.ctrlV - (b.alive ? b.ch * q.ctrlCh : 0) * Math.min(1, sp / .3 + .3) + (sk - 10) * q.skill - q.pressMax * (1 - Math.exp(-Math.max(0, pr - .4) * q.pressK / q.pressMax))));
  }
  // Bir tikte Çekirdeğin geçtiği parçayı kontrol menzilinde kesen oyuncu var mı? Varsa tutma/sekme. Dönüş: null | {p, took:true} | {p, took:false}
  function contact(b, src, q, rnd, ox, oy) {
    q = q || Q; rnd = rnd || Math.random; if (b.air > q.airTouch) return null; const sx = b.x - ox, sy = b.y - oy, sl = sx * sx + sy * sy; let best = null, bd = 1e9;
    for (const p of src) {
      if (p.noTouch || (b.cds && (b.cds.get(p) || 0) > b.t)) continue; if (p === b.from && b.t < 8) continue; if (b.recv && p.team === b.team && p !== b.recv && !b.defl) continue; // arkadaş başkasına giden pası bırakır
      const t0 = sl > 0 ? ((p.x - ox) * sx + (p.y - oy) * sy) / sl : 1, tt = Math.max(0, Math.min(1, t0)), dd = Math.hypot(ox + sx * tt - p.x, oy + sy * tt - p.y), R = (p.role === 'Bekçi' ? bekR(b, p, q) : q.reach + (((p.a && p.a.yogunluk) ?? 10) - 10) * .04) * (t0 < 0 ? q.awayK : 1); // kendisinden uzaklaşan Çekirdeğe ancak dibindeyse yetişir
      if (dd < R && dd < bd) { bd = dd; best = p; }
    }
    if (!best) return null;
    const P = ctrlP(b, best, src, q);
    if (rnd() < P) { b.done = 'tutuldu'; b.holder = best; b.x = best.x; b.y = best.y; b.vx = b.vy = 0; ev(b, 'tut', `${best.name || (best.team === b.team ? 'Arkadaş' : 'Rakip')} tuttu (ihtimal %${Math.round(P * 100)})`); return { p: best, took: true, P }; }
    const mateT = best.team === b.team, sp0 = Math.hypot(b.vx, b.vy) || 1e-6; if (!b.cds) b.cds = new Map(); b.cds.set(best, b.t + Math.max(q.cdMin, Math.round(q.cd * Math.min(1, sp0 / q.cdV)))); // yavaş Çekirdeği kaçıran hemen toparlar; hızlı olan zaten uzaklaşır
    const k = q.deflK * (.7 + rnd() * .6) * (mateT ? q.deflMate : 1);
    // arkadaşın kötü dokunuşu: Çekirdek önüne kaçar (dar açı). Rakibin bloğu: gövdeden geri/yana seker (geldiği yöne doğru, geniş açı)
    // çok hızlı Çekirdek rakibin gövdesini delip geçebilir (güçlü vuruşun karşılığı); Kesme bunu azaltır
    const thru = !mateT && rnd() < thruP(b, best, q);
    const base = mateT || thru ? Math.atan2(b.vy, b.vx) : Math.atan2(-b.vy, -b.vx), an = base + (rnd() - .5) * (thru ? .5 : mateT ? q.deflA * .6 : q.deflA * 1.4), kk = thru ? q.thruKeep : k;
    b.vx = Math.cos(an) * sp0 * kk; b.vy = Math.sin(an) * sp0 * kk; if (b.alive) b.e = Math.max(0, b.e - q.deflE); b.defl = (b.defl || 0) + 1; ev(b, 'sek', `${best.name || (best.team === b.team ? 'Arkadaş' : 'Rakip')} dokundu, sekti (tutma ihtimali %${Math.round(P * 100)})`);
    return { p: best, took: false, P };
  }
  function thruP(b, p, q) { q = q || Q; if (p.team === b.team) return 0; const sp = Math.hypot(b.vx, b.vy); return Math.max(0, Math.min(q.thruMax * (p.role === 'Bekçi' ? q.bekThru : 1), (sp - q.thruV) * q.thruK - (((p.a && p.a.kesme) ?? 10) - 10) * .02)); }
  function stepAll(b, src, p, q, rnd) { if (b.done) return null; const ox = b.x, oy = b.y; stepBall(b, src, p); if (b.done) return null; return contact(b, src, q, rnd, ox, oy); }
  window.AlanCore = { W, H, KR, MOUTH, P, Q, bekR, infl, K, makeGrid, gridS, chgMul, makeBall, stepBall, predict, solveLaunch, ctrlP, contact, stepAll, thruP };
})();
