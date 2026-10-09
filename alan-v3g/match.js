(function () {
  const hyp = window.AlanCore.hyp;
  // Katman 4 (basit) · Yerleşim + maç döngüsü. Talimata sadık, kusursuz değil.
  const C = () => window.AlanCore, Dd = () => window.AlanDecide, W = 100, H = 50;
  const TAC0 = { blok: 'Orta', pres: 1, arkada: 1, onde: 0, genislik: 'Normal', risk: .5, tempo: .5, kazaninca: 'Dengeli', sistem: 'Alan' };
  const WIN_T = 180; // geçiş penceresi: Çekirdek kazanıldıktan sonraki 3 saniye
  const inWin = (m, t) => m.winTeam === t && m.tick - m.winT < WIN_T;
  function tacEff(m, t) { const tq = m.tac[t], kon = tq.kazaninca === 'Kontra' && inWin(m, t), yer = tq.kazaninca === 'Yerleş' && inWin(m, t); return kon ? { ...tq, risk: Math.min(1, (tq.risk ?? .5) + .25), tempo: Math.max(tq.tempo ?? .5, .8) } : yer ? { ...tq, risk: Math.max(0, (tq.risk ?? .5) - .25), tempo: Math.min(tq.tempo ?? .5, .3) } : tq; }
  const BLOK = { 'Düşük': 22, 'Orta': 34, 'Yüksek': 50 }, GEN = { 'Dar': 28, 'Normal': 38, 'Geniş': 46 };
  const SLOTS = [{ role: 'Bekçi', d: 0, l: 0 }, { role: 'Bek', d: .2, l: -.5 }, { role: 'Bek', d: .2, l: .5 }, { role: 'Orta', d: .45, l: -.35 }, { role: 'Orta', d: .45, l: .35 }, { role: 'Kanat', d: .75, l: -.85 }, { role: 'Uç', d: .8, l: .25 }];
  const NAMES = [['Aras', 'Deniz', 'Ece', 'Kaan', 'Mert', 'Selin', 'Tuna'], ['Bora', 'Cem', 'Duru', 'Ilgaz', 'Nehir', 'Rüzgâr', 'Yaz']];
  const cl = (v, a, b) => v < a ? a : v > b ? b : v, dirOf = t => t === 0 ? 1 : -1, ownX = t => t === 0 ? 0 : W;
  // Zar üreteci durumunu taşır (f.s): maç kopyalanınca zar da kaldığı yerden devam eder.
  function rng(seed, st) { const f = () => { f.s = (f.s + 0x6D2B79F5) | 0; let a = f.s, t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; f.s = st != null ? st : (seed >>> 0); f.isRng = true; return f; }
  // Maçın tam kopyası: bütün nesneler (oyuncular, Çekirdek, uçuş, olaylar, önbellekler) kimlikleri korunarak kopyalanır; zar üreteci durumuyla yeniden kurulur.
  // WeakMap önbellekleri boş başlar (saf hesap önbellekleri, sonucu değiştirmez). Kopya ile asıl birbirinden bağımsızdır.
  function cloneMatch(m) { const memo = new Map();
    const cp = v => { if (v === null || typeof v !== 'object' && typeof v !== 'function') return v; if (memo.has(v)) return memo.get(v);
      if (typeof v === 'function') { const f = v.isRng ? rng(0, v.s) : v; memo.set(v, f); return f; }
      if (v instanceof WeakMap) { const w = new WeakMap(); memo.set(v, w); return w; }
      if (v instanceof Map) { const o = new Map(); memo.set(v, o); for (const [k, x] of v) o.set(cp(k), cp(x)); return o; }
      if (v instanceof Set) { const o = new Set(); memo.set(v, o); for (const x of v) o.add(cp(x)); return o; }
      if (ArrayBuffer.isView(v)) { const o = v.slice(); memo.set(v, o); return o; }
      if (Array.isArray(v)) { const o = new Array(v.length); memo.set(v, o); for (let k = 0; k < v.length; k++) o[k] = cp(v[k]); for (const k of Object.keys(v)) if (!/^\d+$/.test(k)) o[k] = cp(v[k]); return o; }
      const o = Object.create(Object.getPrototypeOf(v)); memo.set(v, o); for (const k of Object.keys(v)) o[k] = cp(v[k]); return o; };
    return cp(m); }
  // Maçın parmak izi: oyuncular, Çekirdek, skor, zar. İki maç aynı izi veriyorsa aynı durumdadır.
  function matchHash(m) { let h = 0; const add = x => { h = (Math.imul(h, 31) + Math.round(x * 1e6)) | 0; }; for (const p of m.ps) { add(p.x); add(p.y); add(p.vx); add(p.vy); } if (m.ball) { add(m.ball.x); add(m.ball.y); add(m.ball.vx); add(m.ball.vy); } add(m.holder ? m.ps.indexOf(m.holder) : -1); add(m.score[0]); add(m.score[1]); add(m.ch); add(m.r.s); add(m.tick); return h; }
  const spd = p => .17 + (p.a.hiz ?? 10) * .006;
  function createMatch(seed, o) {
    o = o || {}; const m = { r: rng(seed || 1), seed, tick: 0, len: o.len || 7200, score: [0, 0], ps: [], ball: null, holder: null, ch: .3, decT: 0, over: false, events: [], fl: null,
      tac: [0, 1].map(t => ({ ...TAC0, ...((o.tac && o.tac[t]) || {}) })), st: { duel: [0, 0], duelW: [0, 0], pass: [0, 0], passOk: [0, 0], shot: [0, 0], steal: [0, 0], poss: [0, 0], bank: [0, 0], lead: [0, 0], loose: [0, 0] } };
    for (const t of [0, 1]) SLOTS.forEach((s, i) => { const a = { okuma: 10, aktarim: 10, tutus: 10, kesme: 10, yogunluk: 10, hiz: 10, cesaret: 10, surme: 10, ...((o.attrs && o.attrs[t]) || {}) }; m.ps.push({ id: t * 10 + i, team: t, i, role: s.role, slot: s, name: NAMES[t][i], a, R: s.role === 'Bekçi' ? 7 : 8, D: s.role === 'Bekçi' ? 1.2 : 1, x: 50, y: 25, vx: 0, vy: 0, tx: 50, ty: 25, cd: 0 }); });
    if (o.roles) assignRoles(m, o.roles);
    kickoff(m, 0); return m;
  }
  // Roller: oyuncuya verilen talimat. Neyi, nerede, ne sıklıkla deneyeceğini belirler; başarısı özelliklerden gelir.
  // Hücum: Kurucu (topun gerisinde kalır, oyunu kurar) · Kanat (çizgide, geniş) · Koşucu (son çizginin arkasına koşar) · Pivot (Kuyu önünde, ortada) · Serbest
  // Savunma: Süpürücü (en arkada) · Presçi (ilk basan) · Markajcı (belli bir rakibi takip) · Önde (kontra için ileride bekler) · Bölge
  const RSLOT = { 'Kurucu': s => ({ d: .1, l: s.l * .4 }), 'Kanat': s => ({ d: Math.max(s.d, .55), l: s.l < 0 ? -1 : 1 }), 'Koşucu': s => ({ d: .8, l: s.l }), 'Pivot': () => ({ d: .85, l: 0 }) };
  const STYLES = {
    'Hücumcu': { tac: { blok: 'Yüksek', pres: 2, arkada: 1, onde: 0, genislik: 'Geniş', tempo: .8, risk: .8, kazaninca: 'Dengeli' },
      roles: [{ h: 'Serbest', s: 'Bölge' }, { h: 'Kurucu', s: 'Süpürücü' }, { h: 'Koşucu', s: 'Presçi' }, { h: 'Kanat', s: 'Presçi' }, { h: 'Kanat', s: 'Bölge' }, { h: 'Pivot', s: 'Presçi' }] },
    'Savunmacı': { tac: { blok: 'Düşük', pres: 1, arkada: 2, onde: 1, genislik: 'Dar', tempo: .4, risk: .3, kazaninca: 'Kontra' },
      roles: [{ h: 'Kurucu', s: 'Süpürücü' }, { h: 'Serbest', s: 'Süpürücü' }, { h: 'Serbest', s: 'Markajcı' }, { h: 'Serbest', s: 'Bölge' }, { h: 'Koşucu', s: 'Presçi' }, { h: 'Koşucu', s: 'Önde' }] }
  };
  function assignRoles(m, roles) {
    for (const t of [0, 1]) { const R = roles[t]; if (!R) continue; for (const p of m.ps) { if (p.team !== t || p.role === 'Bekçi') continue; const r = R[p.i - 1]; if (!r) continue; p.rh = r.h || 'Serbest'; p.rs = r.s || 'Bölge'; p.rslot = RSLOT[p.rh] ? RSLOT[p.rh](p.slot) : null; } }
    for (const t of [0, 1]) { const taken = new Set(); for (const p of m.ps.filter(q => q.team === t && q.rs === 'Markajcı')) { const opp = m.ps.filter(q => q.team !== t && q.role !== 'Bekçi' && !taken.has(q)).sort((u, v) => ((v.rh === 'Pivot' || v.rh === 'Koşucu') - (u.rh === 'Pivot' || u.rh === 'Koşucu')) || ((v.rslot || v.slot).d - (u.rslot || u.slot).d)); if (opp[0]) { taken.add(opp[0]); p.markRef = opp[0]; } } }
    m.ra = [0, 1].map(() => ({}));
  }
  // Talimata uyum: her 10 tikte, talimatın geçerli olduğu anlarda, oyuncu talimatın istediği yerde mi?
  function roleSample(m) {
    const h = m.holder; if (!h || m.tick % 10) return; const E = { x: h.x, y: h.y };
    for (const p of m.ps) { if (!p.rh || p === h) continue; const t = p.team, dir = dirOf(t), atk = h.team === t, A = m.ra[t][p.i] || (m.ra[t][p.i] = { h: [0, 0], s: [0, 0] });
      let k = null, ok = false;
      if (atk) { k = 'h';
        if (p.rh === 'Kanat') ok = Math.abs(p.y - 25) >= 13;
        else if (p.rh === 'Kurucu') ok = (p.x - E.x) * dir < -2;
        else if (p.rh === 'Koşucu') { const ox = W - ownX(t); let last = null; for (const q of m.ps) if (q.team !== t && q.role !== 'Bekçi' && (last == null || Math.abs(q.x - ox) < Math.abs(last - ox))) last = q.x; ok = last != null && (p.x - last) * dir >= -3; }
        else if (p.rh === 'Pivot') { if ((E.x - 50) * dir <= 0) k = null; else ok = hyp(p.x - (W - ownX(t)), p.y - 25) < 28 && Math.abs(p.y - 25) < 12; }
        else k = null;
      } else { k = 's';
        if (p.rs === 'Süpürücü') { let deep = 1e9; for (const q of m.ps) if (q.team === t && q.role !== 'Bekçi') deep = Math.min(deep, Math.abs(q.x - ownX(t))); ok = Math.abs(p.x - ownX(t)) <= deep + 3; }
        else if (p.rs === 'Presçi') ok = !!p.press;
        else if (p.rs === 'Markajcı') ok = p.markRef ? hyp(p.x - p.markRef.x, p.y - p.markRef.y) < 6 : false;
        else if (p.rs === 'Önde') ok = (p.x - 50) * dir > 0;
        else k = null; }
      if (k) { A[k][0]++; if (ok) A[k][1]++; } }
  }
  function ev(m, type, team, text) { const s = Math.floor(m.tick / 60); m.events.unshift({ t: `${String(Math.floor(s / 60)).padStart(2, '0')}:${String(s % 60).padStart(2, '0')}`, type, team, text }); if (m.events.length > 120) m.events.pop(); }
  function kickoff(m, t) {
    for (const p of m.ps) { const s = p.slot, x0 = ownX(p.team), dir = dirOf(p.team), d = s.role === 'Bekçi' ? 3 : 10 + s.d * 32; p.x = x0 + dir * d; p.y = 25 + s.l * 16; p.vx = p.vy = 0; p.tx = p.x; p.ty = p.y; }
    const h = m.ps.find(p => p.team === t && p.i === 3); h.x = 50; h.y = 25; m.holder = h; m.ball = null; m.ch = .3; m.decT = 30; m.fl = null;
  }
  // takımın şekli: savunmada kompakt ve blok yüksekliğinde, hücumda uzun ve genişlik talimatında
  function slotPos(m, p, bx, by, att) {
    const t = p.team, tac = m.tac[t], s = p.slot, x0 = ownX(t), dir = dirOf(t), bd = (bx - x0) * dir;
    if (s.role === 'Bekçi') { const gx = x0, gy = 25, dx = bx - gx, dy = by - gy, L = hyp(dx, dy) || 1, r = Math.min(6, L * .22); return { x: gx + dx / L * r, y: gy + dy / L * r }; }
    let back, len, wid, sh;
    if (att) { back = cl(bd - 18, 12, 62); len = 48; wid = GEN[tac.genislik] || 38; sh = .15; }
    else { back = cl(Math.min(BLOK[tac.blok] || 34, bd - 4), 9, 62); len = 24; wid = 26; sh = .4; }
    if (!att) { const front = bd - 7; if (back + len > front) len = Math.max(6, front - back); }
    let depth = back + (s.d - .2) / .6 * len; if (!att) depth = Math.min(depth, Math.max(back, bd - 7));
    return { x: cl(x0 + dir * depth, 2, 98), y: cl(25 + s.l * wid / 2 + (by - 25) * sh, 2, 48) };
  }
  function laneOpp(m, t, a, b) { let mx = 0; for (let i = 1; i <= 4; i++) { const x = a.x + (b.x - a.x) * i / 5, y = a.y + (b.y - a.y) * i / 5; mx = Math.max(mx, Dd().oppAt(m.ps, t, x, y)); } return mx; }
  // topsuz hücumcu: slotunun yakınında açık, pas hattı görünen, ileri değerli ve arkadaşlardan ayrık nokta
  function offBall(m, p, base, h) {
    const D = Dd(), t = p.team, dir = dirOf(t), ok = self.ALAN_OKC(p, 'yerlesim'), cands = [base];
    for (let k = 0; k < 8; k++) { const a = k * Math.PI / 4; for (const r of [5, 9]) cands.push({ x: base.x + Math.cos(a) * r, y: base.y + Math.sin(a) * r }); }
    if (p.slot.d >= .7) { const lastD = m.ps.filter(q => q.team !== t && q.role !== 'Bekçi').reduce((mx, q) => Math.max(mx, (q.x - ownX(t)) * dir), 0); cands.push({ x: ownX(t) + dir * (lastD + 5), y: p.y }, { x: ownX(t) + dir * (lastD + 5), y: base.y }); }
    let best = base, bv = -1e9;
    for (const c of cands) {
      if (c.x < 2 || c.x > 98 || c.y < 2 || c.y > 48) continue;
      const open = Math.exp(-D.oppAt(m.ps, t, c.x, c.y) * 2), lane = h ? Math.exp(-laneOpp(m, t, h, c) * 1.5) : 1, fwd = D.threat(t, c.x, c.y);
      let crowd = 0; for (const q of m.ps) if (q.team === t && q !== p && q.role !== 'Bekçi') crowd += Math.exp(-((q.tx - c.x) ** 2 + (q.ty - c.y) ** 2) / 36);
      const leash = hyp(c.x - base.x, c.y - base.y) / 12, v = .5 * open + .6 * lane + 1.2 * fwd - .5 * crowd - .4 * leash * leash + (m.r() - .5) * (20 - ok) * .02;
      if (v > bv) { bv = v; best = c; }
    }
    return best;
  }
  function position(m) {
    const ps = m.ps, h = m.holder, b = m.ball, bx = h ? h.x : b.x, by = h ? h.y : b.y, attT = h ? h.team : b.team;
    // boşta / uçan Çekirdek: her takımdan en yakın 1-2 kişi kovalar; alıcı koşar
    const chase = new Set();
    if (!h) { const tgt = m.fl && m.fl.end ? m.fl.end : { x: b.x, y: b.y }; for (const t of [0, 1]) { const c = ps.filter(p => p.team === t && p.role !== 'Bekçi').sort((u, v) => hyp(u.x - tgt.x, u.y - tgt.y) - hyp(v.x - tgt.x, v.y - tgt.y)); const n = b.done === 'durdu' || b.defl ? 2 : (t === b.team ? 0 : 1); c.slice(0, n).forEach(p => { chase.add(p); p.tx = tgt.x; p.ty = tgt.y; p.sprint = true; }); } if (m.fl && m.fl.q && !chase.has(m.fl.q)) { chase.add(m.fl.q); const q = m.fl.q; q.tx = m.fl.to.x; q.ty = m.fl.to.y; q.sprint = true; } }
    for (const t of [0, 1]) {
      const att = t === attT, tac = m.tac[t], mine = ps.filter(p => p.team === t);
      let press = new Set(), cover = new Set(); const marked = new Set();
      if (!att) {
        const field = mine.filter(p => p.role !== 'Bekçi').sort((u, v) => u.slot.d - v.slot.d);
        field.slice(0, tac.arkada).forEach(p => cover.add(p));
        if (h) mine.filter(p => p.role !== 'Bekçi' && !cover.has(p)).sort((u, v) => hyp(u.x - h.x, u.y - h.y) - hyp(v.x - h.x, v.y - h.y)).slice(0, tac.pres).forEach(p => press.add(p));
      }
      for (const p of mine) {
        if (p === h || chase.has(p)) continue; p.sprint = false; const base = slotPos(m, p, bx, by, att);
        if (p.role === 'Bekçi') { p.tx = base.x; p.ty = base.y; continue; }
        if (att) { if ((m.tick + p.id) % 15 === 0 || p.otx == null) { const c = offBall(m, p, base, h); p.otx = c.x; p.oty = c.y; } p.tx = p.otx; p.ty = p.oty; continue; }
        const gx = ownX(t), opps = ps.filter(q => q.team !== t && q.role !== 'Bekçi');
        if (press.has(p)) { const dx = gx - h.x, dy = 25 - h.y, L = hyp(dx, dy) || 1, dd = press.size > 1 && [...press].indexOf(p) > 0 ? 3.2 : 2.2, side = [...press].indexOf(p) === 1 ? 1 : [...press].indexOf(p) === 2 ? -1 : 0, nx = -dy / L, ny = dx / L; p.tx = h.x + dx / L * dd + nx * side * 2.5; p.ty = h.y + dy / L * dd + ny * side * 2.5; p.sprint = hyp(p.x - p.tx, p.y - p.ty) > 2; continue; }
        if (cover.has(p)) { const a = opps.reduce((u, v) => Math.abs(v.x - gx) < Math.abs(u.x - gx) ? v : u), dx = gx - a.x, dy = 25 - a.y, L = hyp(dx, dy) || 1; p.tx = a.x + dx / L * 4; p.ty = a.y + dy / L * 4; if (Math.abs(p.tx - gx) > Math.abs(base.x - gx)) { p.tx = base.x; } continue; }
        // bölge: slotunu tutar, bölgesindeki rakibin Kuyu tarafına geçer, top–Kuyu hattına biraz kayar
        let tx = base.x, ty = base.y; const near = opps.filter(q => q !== h && !marked.has(q) && hyp(q.x - base.x, q.y - base.y) < 9).sort((u, v) => hyp(u.x - base.x, u.y - base.y) - hyp(v.x - base.x, v.y - base.y))[0];
        if (near) { marked.add(near); const gxx = (gx + bx) / 2, gyy = (25 + by) / 2, dx = gxx - near.x, dy = gyy - near.y, L = hyp(dx, dy) || 1; tx = near.x + dx / L * 3; ty = near.y + dy / L * 3; const off = hyp(tx - base.x, ty - base.y); if (off > 7) { tx = base.x + (tx - base.x) * 7 / off; ty = base.y + (ty - base.y) * 7 / off; } }
        else { const dx = gx - bx, dy = 25 - by, L = hyp(dx, dy) || 1, u = ((tx - bx) * dx + (ty - by) * dy) / L, px = bx + dx / L * u, py = by + dy / L * u; tx += (px - tx) * .3; ty += (py - ty) * .3; }
        p.tx = tx; p.ty = ty;
      }
    }
  }
  function move(m) {
    for (const p of m.ps) {
      const dx = p.tx - p.x, dy = p.ty - p.y, L = hyp(dx, dy), v = spd(p) * (p.sprint ? 1.25 : 1) * (p === m.holder ? .85 * ((p.slowT || 0) > m.tick ? Dd().DU.slowK : 1) : 1) * (p.noTouch ? .3 : 1), want = L < .3 ? 0 : Math.min(v, L * .25);
      const wx = L ? dx / L * want : 0, wy = L ? dy / L * want : 0; p.vx += (wx - p.vx) * .18; p.vy += (wy - p.vy) * .18; p.x = cl(p.x + p.vx, .8, W - .8); p.y = cl(p.y + p.vy, .8, H - .8);
      if (p.role !== 'Bekçi') outOfKuyu(p, true);
    }
    // gövdeler üst üste binemez: iç içe geçen iki oyuncu yarı yarıya ayrılır (taşıyıcı ile ona dokunan presçi dahil)
    const ps = m.ps, BD = C().Turn.TQ.minD; /* kafadaki dönüş hesabıyla aynı gövde sınırı */ for (let i = 0; i < ps.length; i++) for (let j = i + 1; j < ps.length; j++) { const a = ps[i], b = ps[j], dx = b.x - a.x, dy = b.y - a.y, dd = hyp(dx, dy); if (dd >= BD) continue; const ux = dd > 1e-3 ? dx / dd : 1, uy = dd > 1e-3 ? dy / dd : 0, push = (BD - dd) / 2; a.x = cl(a.x - ux * push, .8, W - .8); a.y = cl(a.y - uy * push, .8, H - .8); b.x = cl(b.x + ux * push, .8, W - .8); b.y = cl(b.y + uy * push, .8, H - .8); }
  }
  // Siyah alan (Kuyu dairesi): Bekçi dışında kimse giremez. Oyuncu dairenin kenarına itilir, içeri doğru hızı silinir (kenarı boyunca kayabilir).
  const KUYU_PAD = .6;
  function outOfKuyu(p, vel) { const R = C().KR + KUYU_PAD; for (const kx of [0, W]) { const dx = p.x - kx, dy = p.y - 25, d = hyp(dx, dy); if (d < R) { const ux = d > 1e-6 ? dx / d : (kx ? -1 : 1), uy = d > 1e-6 ? dy / d : 0; p.x = kx + ux * R; p.y = 25 + uy * R; if (vel) { const vn = p.vx * ux + p.vy * uy; if (vn < 0) { p.vx -= vn * ux; p.vy -= vn * uy; } } } } }
  function kuyuSafe(x, y) { const R = C().KR + KUYU_PAD + .3; for (const kx of [0, W]) { const dx = x - kx, dy = y - 25, d = hyp(dx, dy); if (d < R) { const ux = d > 1e-6 ? dx / d : (kx ? -1 : 1), uy = d > 1e-6 ? dy / d : 0; return { x: kx + ux * R, y: 25 + uy * R }; } } return null; }
  // Tutulan Çekirdek taşıyıcının üstünde değil, gövdesinin kenarında durur: hareket ettiği yönde, duruyorsa gitmek istediği yönde (ikilideki "Çekirdeğin tarafı" ile aynı).
  function corePos(m) { const h = m.holder; if (!h) return m.ball ? { x: m.ball.x, y: m.ball.y } : null; const cs = Dd().coreSide(h), r = C().Q.body; return { x: h.x + cs.ux * r, y: h.y + cs.uy * r }; }
  // Ver-kaç: pası veren, dibinde presçi varken kısa pas verirse presçinin arkasındaki boşluğa koşar (alıcı tekte geri verebilir).
  function verKac(m, h, o) { if (!o.q || o.q === h || o.kind === 'gönder' || o.drib) return; const g = dirOf(h.team); let nq = null, nd = 1e9; for (const q of m.ps) if (q.team !== h.team && q.role !== 'Bekçi') { const dd = hyp(q.x - h.x, q.y - h.y); if (dd < nd) { nd = dd; nq = q; } } if (!nq || nd > 4 || hyp(o.q.x - h.x, o.q.y - h.y) > 14) return; h.vkW = { x: cl(nq.x + g * 3, 2, 98), y: cl(nq.y + (h.y < nq.y ? -2.4 : 2.4), 2, 48) }; h.vkUntil = m.tick + 45; }
  function launch(m, h, o) { verKac(m, h, o);
    const D = Dd(), K = C(), v = D.applyError(h, o.launch, m.r, o.ot || 0), t = h.team; const tk = o.ot ? ' · tek dokunuş' : ''; if (o.ot) { m.st.ot = m.st.ot || [0, 0]; m.st.ot[t]++; if (o.kind !== 'gönder' && o.kind !== 'aşırt' && o.kind !== 'kenardan') ev(m, 'pas', t, `${h.name} tek dokunuşla → ${o.q ? o.q.name : ''} · şarj %${Math.round(m.ch * 100)}`); }
    const cp = corePos(m) || { x: h.x, y: h.y }; m.ball = K.makeBall({ x: cp.x, y: cp.y, vx: v.vx, vy: v.vy, team: t, ch: m.ch, from: h, recv: o.q || null }); if (o.kind === 'aşırt') { m.st.lob = m.st.lob || [0, 0]; m.st.lob[t]++; ev(m, 'pas', t, `${h.name} aşırttı → ${o.q.name}${tk}`); } m.holder = null;
    const pr = K.predict(m.ball, m.ps, null, 600); m.fl = { t0: m.tick, route: o.W ? [{ ...o.W }] : null, drib: o.drib || null, kind: o.kind, q: o.q || null, to: o.T || o.to, end: { x: pr.end.x, y: pr.end.y }, shot: o.shot };
    if (o.kind === 'gönder') { m.st.shot[t]++; m.ball.shotPk = o.det ? o.det.Pk : null; if (!m.lite) { const c = m.st.cal || (m.st.cal = { n: 0, pk: 0, g: 0 }); if (m.ball.shotPk != null) { c.n++; c.pk += m.ball.shotPk; } } ev(m, 'gonder', t, `${h.name} ${o.shot} gönderdi${tk} · şarj %${Math.round(m.ch * 100)}`); }
    else { m.st.pass[t]++; if (o.kind === 'kenardan') { m.st.bank[t]++; ev(m, 'pas', t, `${h.name} kenardan sektirdi → ${o.q.name}${tk}`); } if (o.kind === 'önüne') m.st.lead[t]++; if (o.why === 'kombinasyon') { m.st.combo = m.st.combo || [0, 0]; m.st.combo[t]++; } }
  }
  const D0 = () => window.AlanDecide.D && window.AlanDecide.freeAt ? Object.assign({ freeAt: window.AlanDecide.freeAt }, window.AlanDecide.D) : null;
  function take(m, p, how) {
    m._inSp = m.ball ? hyp(m.ball.vx, m.ball.vy) : 0;
    const prev = m.ball ? m.ball.team : null, fl = m.fl; if (m.ball) p.ca = Math.atan2(m.ball.y - p.y, m.ball.x - p.x); else if (m.holder) p.ca = Math.atan2(m.holder.y - p.y, m.holder.x - p.x); m.holder = p; m.ball = null; m.decT = Math.max(2, Math.round(9 - self.ALAN_OKC(p, 'kararHizi') * .4 + m.r() * 3)); m.fl = null; p.driveTo = null; p._lc = null; // karşılamadan sonra karar: Okuma'sı yüksek oyuncu Çekirdek gelmeden bakmıştır, hemen oynar
    if (prev === p.team) { if (fl && fl.kind !== 'gönder') { m.st.passOk[p.team]++;
      // Tek dokunuş: alıcı Çekirdeği durdurmadan yönlendirebilir (pas ya da gönderme). Şarj hiç azalmaz, ama isabet gelen hıza ve Aktarım'a göre düşer.
      // Kontrol ederse şarjın bir kısmı gider (boşta karşılayan daha çok korur). Hangisinin iyi olduğuna oyuncu kendi karar verir.
      const D = Dd(), inSp = m._inSp || 0, chFull = m.ch, chCtrl = D.recvCh(m.ps, p.team, p.x, p.y, chFull), ot = D.otFactor(p, inSp);
      let one = null; if (!m._lite && self.ALAN_OKC(p, 'karar') >= D.D.otOk) { const tE = tacEff(m, p.team), r = D.decideOT(p, m.ps, chFull, m.r, tE, ot), rc = (() => { /* Kontrol etmenin bedeli zamandır: Çekirdeği durdurup gitmek istediği tarafa döndürene kadar (tepki + Çekirdeği gövdenin etrafından geçirme) savunma kapanır. Kontrol sonrası seçenekler o süre sonraki dünyada değerlendirilir; dönüş süresi, gelen pasın tarafı ile o an en umutlu hedefin yönü arasındaki açıdan çıkar. */
          let tTurn = 0; const K2 = C(), aim = r && (r.T || r.to); if (aim && p.ca != null) { const ta = Math.atan2(aim.y - p.y, aim.x - p.x); tTurn = K2.Turn.arc(p.ca, ta, K2.Turn.shortDir(p.ca, ta)) / K2.Turn.omega(p); }
          const w = D.D.diagCtrlReal ? m.ps : D.predictWorld(m.ps, p.team, p, D.D.ctrlT + tTurn + 9) /* diagCtrlReal: sadece teşhis (f7 öncesi: kontrol kararı gerçek dünyada) */, rc1 = D.decide(p, w, chCtrl, m.r, tE); for (const o of rc1.opts) if (o.q) o.q = m.ps.find(z => z.id === o.q.id) || o.q; return rc1; })(); const ctrlBest = rc.opts.length ? rc.opts[0].v : 0; const b0 = rc.best; p._plan = b0 && b0.q && b0.to && b0.det && ['pas', 'önüne', 'aşırt', 'kenardan'].includes(b0.kind) ? { kind: b0.kind, q: b0.q, to: { x: b0.to.x, y: b0.to.y }, P: b0.det.P ?? 0, t: m.tick } : null; if (r && r.v > ctrlBest + D.D.otMargin) one = r; }
      if (one) { m.ch = chFull; one.ot = ot; launch(m, p, one); return; }
      m.ch = chCtrl; } }
    else { m.st.steal[p.team]++; m.ch = .3; m.winT = m.tick; m.winTeam = p.team; ev(m, 'kesme', p.team, how || `${p.name} Çekirdeği aldı`); }
  }
  function step(m) {
    if (m.over) return; const K = C(), D = Dd(), ps = m.ps; m.tick++; if (!m._lite && !D.D._inLook) { D.D._tick = m.tick; D.D._m = m; } /* hayaldeki kopyalar gerçek saati ve gerçek maç referansını ezmesin */
    for (const p of ps) p.noTouch = p.stun > m.tick;
    if (window.AlanShape) window.AlanShape.position(m); else position(m); move(m); if (m.ra) roleSample(m); if (window.AlanShape) { const sw = window.AlanShape.metrics(m).swarm; m.st.swarm = (m.st.swarm || 0) + sw; m.st.swN = (m.st.swN || 0) + 1; }
    const h = m.holder;
    if (h) {
      m.st.poss[h.team]++;
      const pr = D.oppAt(ps, h.team, h.x, h.y), still = cl(1 - hyp(h.vx, h.vy) / .2, 0, 1);
      m.ch = Math.min(1, m.ch + D.chargeRate(h, ps, still, m.ch));
      // Çekirdeğin gövde üstündeki açısı: gidiş yönünde, en yakın rakibin (Okuma kadar önceden okunan) gelişinden uzağa kaydırılmış. Dönüş hızı Sürme'ye bağlı, hızlandıkça yavaşlar; rakip tarafından geçirmemeye çalışır.
      { const w0 = (a => { while (a > Math.PI) a -= 2 * Math.PI; while (a < -Math.PI) a += 2 * Math.PI; return a; }); let nq = null, nd = 1e9; for (const q of ps) if (q.team !== h.team && q.role !== 'Bekçi') { const dd = hyp(q.x - h.x, q.y - h.y); if (dd < nd) { nd = dd; nq = q; } }
        const sp = hyp(h.vx, h.vy), mv = sp > .04 ? Math.atan2(h.vy, h.vx) : (h.tx != null && hyp(h.tx - h.x, h.ty - h.y) > .5 ? Math.atan2(h.ty - h.y, h.tx - h.x) : Math.atan2(25 - h.y, (h.team === 0 ? 100 : 0) - h.x));
        let ta = mv, pa = null; if (nq && nd < 6) { const la = 2 + self.ALAN_OKC(h, 'ikili') * .4, aw = Math.atan2(h.y - (nq.y + (nq.vy || 0) * la), h.x - (nq.x + (nq.vx || 0) * la)); pa = w0(aw + Math.PI); ta = sp > .04 ? mv + cl(w0(aw - mv), -1, 1) * .7 : aw; }
        if (h.ca == null) h.ca = mv; const dd = w0(ta - h.ca); if (Math.abs(dd) > .02) { let dir = Math.sign(dd); if (pa != null) { const m1 = w0(h.ca + dd / 2), m2 = w0(h.ca + (dd - dir * 2 * Math.PI) / 2); if (Math.abs(w0(m1 - pa)) < .9 && Math.abs(w0(m2 - pa)) > Math.abs(w0(m1 - pa))) dir = -dir; } const w = K.Turn.omega(h); h.ca = w0(h.ca + dir * (dir === Math.sign(dd) ? Math.min(w, Math.abs(dd)) : w)); } }
      // ikili mücadele: yakındaki savunmacı girip girmemeye kendisi karar verir. Tutarsa alır (ya da Çekirdek boşa çıkar), tutmazsa geçilir.
      for (const q of ps) {
        if (q.team === h.team || q.role === 'Bekçi' || q.noTouch || q.cd > m.tick) continue; if (hyp(q.x - h.x, q.y - h.y) > D.DU.engage) continue;
        const P = D.duelP(q, h, ps), est = P + (m.r() - .5) * (20 - self.ALAN_OKC(q, 'ikili')) * .03; q.cd = m.tick + D.DU.cd;
        if (est < D.commitThr(q, h, ps)) continue;
        m.st.duel[q.team]++; h.slowT = m.tick + D.DU.slow; h.vx *= .45; h.vy *= .45;
        if (m.r() < P) { m.st.duelW[q.team]++; if (m.r() < .6) { take(m, q, `İkili · ${q.name}, ${h.name}'den Çekirdeği aldı`); return; } const a = m.r() * 6.28; m.ball = K.makeBall({ x: h.x, y: h.y, vx: Math.cos(a) * .5, vy: Math.sin(a) * .5, team: h.team, ch: m.ch, from: q, defl: 1 }); m.holder = null; m.fl = null; m.st.loose[h.team]++; ev(m, 'kesme', q.team, `${q.name} dokundu, Çekirdek boşta`); return; }
        q.stun = m.tick + Math.round(D.DU.stun - self.ALAN_OKC(q, 'ikili') * D.DU.stunOk); q.noTouch = true; ev(m, 'pas', h.team, `${h.name}, ${q.name}'i geçti`);
      }
      if (--m.decT <= 0) {
        const tE = tacEff(m, h.team), r = D.decide(h, ps, m.ch, m.r, tE); let o = r.best;
        // Karşılamadan önce görülen pas: alıcı Çekirdek gelmeden bakmıştı (plan). Kontrol süresi geçince, aynı hedef hâlâ makulse (tutma ihtimali en fazla 0,15 düşmüşse) onu oynar; kontrol sırasında yeniden fikir değiştirmez.
        const pl = h._plan; h._plan = null; if (pl && m.tick - pl.t < 30) { const same = r.opts.find(x => x.kind === pl.kind && x.q === pl.q && x.to && hyp(x.to.x - pl.to.x, x.to.y - pl.to.y) < 4); if (same && same.det && same.det.P >= pl.P - .15 && same.v > 0) o = same; } m.lastDec = { name: h.name, team: h.team, x: h.x, y: h.y, opts: r.opts.slice(0, 4).map(o => ({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), to: o.to, q: o.q ? o.q.name : '', P: o.det ? (o.kind === 'gönder' ? o.det.Pk : o.det.P) : null, v: o.v })) }; m.decT = Math.round(5 + (1 - m.tac[h.team].tempo) * 8 + m.r() * 3); // tutarken ve sürerken düşünmeye devam eder (tempo sıklığı belirler)
        if (o.kind === 'sür') { h.driveTo = { x: o.to.x, y: o.to.y }; h.tx = o.to.x; h.ty = o.to.y; h.drive = true; }
        else if (o.kind === 'tut') { h.driveTo = null; h.tx = h.x; h.ty = h.y; if (o.why === 'çek' && h.lastWhy !== 'çek') ev(m, 'pas', h.team, `${h.name} rakibi üstüne çekiyor${o.det.lookTo ? ' · ' + o.det.lookTo.name + ' açılacak' : ''}`); }
        else launch(m, h, o);
        h.lastWhy = o.why || o.kind;
      }
      // sürüş kararı bir sonraki karara kadar sürer (eskiden tek tik sürüp taşıyıcı duruyordu)
      if (h === m.holder && !h.drive) { if (h.driveTo && hyp(h.driveTo.x - h.x, h.driveTo.y - h.y) > .8) { h.tx = h.driveTo.x; h.ty = h.driveTo.y; } else { h.driveTo = null; h.tx = h.x; h.ty = h.y; } }
      return;
    }
    const b = m.ball;
    if (b.done === 'durdu') { let best = null, bd = 1e9; for (const p of ps) { const d = hyp(p.x - b.x, p.y - b.y); if (d < bd) { bd = d; best = p; } } if (bd < K.Q.reach && m.r() < K.ctrlP(b, best, ps)) take(m, best, `${best.name} boştaki Çekirdeği aldı`); }
    else {
      const r = K.stepAll(b, ps, null, null, m.r);
      if (r && r.took) { const fl = m.fl; take(m, r.p, fl && fl.kind === 'gönder' && r.p.role === 'Bekçi' ? `${r.p.name} göndermeyi tuttu` : `${r.p.name} aktarımı kesti`); return; }
      if (r && !r.took && m.fl) { m.fl.end = { x: b.x, y: b.y }; }
      if (b.done && String(b.done).startsWith('kuyu')) { const sc = b.done === 'kuyu-sag' ? 0 : 1; m.score[sc]++; if (b.shotPk != null && m.st.cal) m.st.cal.g++; ev(m, 'sayi', sc, `SAYI · ${b.from ? b.from.name : ''} · ${m.score[0]}–${m.score[1]}`); kickoff(m, 1 - sc); }
      else if (b.done === 'durdu' && m.fl) { m.fl.end = { x: b.x, y: b.y }; }
    }
    for (const p of ps) p.drive = false;
    if (m.tick >= m.len) m.over = true;
  }
  function run(m) { while (!m.over) step(m); return m; }
  window.AlanMatch = { rng, cloneMatch, matchHash, corePos, outOfKuyu, kuyuSafe, STYLES, assignRoles, WIN_T, inWin, TAC0, BLOK, GEN, createMatch, step, run, position, move, slotPos };
})();
