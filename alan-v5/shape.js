(function () {
  // Katman 4 · Yerleşim. Herkes kendi algısıyla (Okuma) okur: rakibi kusurlu görür, hareketini öngörür, arkadaşının hedefini bilir.
  const C = () => window.AlanCore, Dd = () => window.AlanDecide, W = 100, H = 50;
  const cl = (v, a, b) => v < a ? a : v > b ? b : v, dirOf = t => t === 0 ? 1 : -1, ownX = t => t === 0 ? 0 : W, oppX = t => t === 0 ? W : 0;
  const ok = p => (p.a && p.a.okuma) ?? 10, spd = p => .17 + ((p.a && p.a.hiz) ?? 10) * .006, SPR = 1.25;
  // Pres tetikleri: ilk presçi her zaman basar; ek presçiler (taktikteki sayıya kadar) yalnız seçili tetiklerden biri varken katılır. Tetik seçilmemişse sürekli basılır. Karşı pres penceresinde tetik aranmaz.
  // skor ve zaman farkındalığı (savunma): yenik takımın kalesini korumasının değeri düşer, Çekirdeği geri almanın değeri artar (macEff oranı). Bu yüzden daha çok kişiyle ve daha yüksekten basar; önde olan geri çekilir. Menajerin pres ayarı taban
  function macPres(m, t) { const ME = window.AlanMatch && window.AlanMatch.macEff; if (!ME || !(m.tac[t] && m.tac[t].yz)) return 0; /* sadece yapay zekâ yönettiği takım skora göre pres/blok ayarlar; kullanıcının takımında bu karar menajerin */ const w = ME(m, t, {}), o = w && w.macK, p0 = (m.tac[t] && m.tac[t].pres) ?? 1; return o == null ? 0 : o < .6 ? (p0 < 2 ? 1 : 0) : o > 1.6 && p0 > 1 ? -1 : 0; } /* ölçülü: yenik ve az basan takım (pres 0–1) bir kişi ekler, zaten 2+ basan takım sayıyı artırmaz (bloğu bir kademe yükseltir); önde olan çok basan takım bir kişi azaltır */
  function presLim(m, dt, h, att) { const n = 2, /* v5 · basan sayısı ayarı kapalı (ölçüm: 1–3 sahada aynı): sabit 2, kimin basacağına mekanik karar verir */ T = dt.tetik; if (!h || !T || !T.length || dt._kp || n <= 1) return n; const ps = m._pas, yeni = ps && ps.team === att && m.tick - ps.t < 90, dir = att === 0 ? 1 : -1;
    const on = (T.includes('Geri pas') && yeni && (ps.tx - ps.fx) * dir < -2) || (T.includes('Kanatta') && Math.abs(h.y - 25) > 15) || (T.includes('Zayıf kontrol') && (h.slowT || 0) > m.tick) || (T.includes('Arkası dönük') && h.ca != null && Math.cos(h.ca) * dir < -.3) || (T.includes('Uzun pas havada') && yeni && ps.len > 25);
    return on ? n : Math.min(n, 1); }
  const S = { bekSerbest: true,
    errPos: .14,      // algı hatası: (20-Okuma) × bu birim
    look: .6, lag0: 10, lagOk: .4,  // algı gecikmesi: 10 - Okuma×0.4 tik; öngörü: (Okuma-6) × 0.6 tik ileri
    react: .5,        // pası okuma gecikmesi: 14 - Okuma × bu tik
    reactLoose: 6, reactLooseOk: .25, // boşa çıkan Çekirdeği fark etme: 6 - Okuma × 0.25 tik
    marginT: .6,      // "ben de yetişirim" payı (tik): (20-Okuma) × bu
    chaseMargin: .015,// "ilk ben varırım" emin olma payı: (20-Okuma) × bu
    zoneR: 7, zoneOk: .5, zoneDng: 9, ayr: 4.5, ortakMin: .3, ortakR: 3.5, prT0: 22, prStill: 1.2, prCost: .55, prPmin: .3, prHat: { 'Düşük': 42, 'Orta': 62, 'Yüksek': 100 }, prDis: .08, prHold: 120, markOff: 2.5, markW: .75,
    pressR: { 'Düşük': 12, 'Orta': 20, 'Yüksek': 32 }, lock: 40,
    runK: .6, // Kontra koşusunda boşluk değerinin ağırlığı
    vacOk0: 6, vacOkStep: 4, vacLeft: 7, vacEmpty: 6, vacLeash: .4, // açılan boşluk: 1 sn önce bir savunmacının durduğu, şimdi 6 birimde kimsenin olmadığı yer (savunmacı 7+ uzaklaşmış). Okuma 8–9 bir, 12–13 iki, 16+ üç boşluk fark eder
    kanatK: .12, kurucuK: .08, pivotGap: 0, // deney: Pivot'un Kuyu'ya en az bu kadar uzak durması (0 = serbest) // rol talimatının dışına çıkmanın bedeli
    recT: 120, recOk: 1.2, rcvRisk: 1, rcvMarginT: 10, // alıcı: beklemenin bedeli rakibin o noktaya alıcıdan ne kadar geç vardığına (pay, tik) bağlı; pay büyükse beklemek ucuz, küçükse pahalı runK: .6, // Kontra koşusunda boşluk değerinin ağırlığı
    W: { runSprint: 4, deep: 6, tv: 1, tvPass: .9, tvDrive: .85, dlReach: 4, defReach: 5, leash: .12, hyst: .04, noise: .01 }
  };
  const BLOK = { 'Düşük': 22, 'Orta': 34, 'Yüksek': 50 }, GEN = { 'Dar': 28, 'Normal': 38, 'Geniş': 46 };
  const hsh = (a, b, c) => { let h = (a * 73856093) ^ (b * 19349663) ^ (c * 83492791); h = Math.imul(h ^ (h >>> 13), 1274126177); return ((h ^ (h >>> 16)) >>> 0) / 4294967296 - .5; };
  function threat(t, x, y) { return Dd().threat(t, x, y); }
  // p'nin gözünden rakipler: konum hatası (Okuma) + hareket öngörüsü
  function seen(m, p) {
    if (m._sn && m._snT === m.tick && m._sn.has(p)) return m._sn.get(p); if (m._snT !== m.tick) { m._sn = new Map(); m._snT = m.tick; }
    // gecikmeli algı: rakibi birkaç tik önceki hâliyle görür, o anki hızıyla ileri taşır. Ani yön değişimini geç fark eder (Okuma gecikmeyi kısaltır).
    const e = (20 - ok(p)) * S.errPos, dl = Math.max(1, Math.round(S.lag0 - ok(p) * S.lagOk)), la = dl + (ok(p) - 6) * S.look, b = Math.floor(m.tick / 20), out = [];
    for (const q of m.ps) if (q.team !== p.team) { const hh = q.hist && q.hist.length ? q.hist[Math.max(0, q.hist.length - 1 - dl)] : { x: q.x, y: q.y, vx: q.vx || 0, vy: q.vy || 0 }; out.push({ a: q.a, x: hh.x + hh.vx * la + hsh(p.id, q.id, b) * 2 * e, y: hh.y + hh.vy * la + hsh(q.id, p.id, b + 7) * 2 * e, R: q.R, D: q.D, team: q.team, role: q.role, press: q.press, ref: q }); }
    m._sn.set(p, out); return out;
  }
  const oppIn = (sn, x, y) => { let s = 0; for (const q of sn) s += C().infl(q, x, y); return s; };
  const laneIn = (sn, a, b, n) => { let s = 0; n = n || 4; for (let i = 1; i <= n; i++) { const t = i / (n + 1); s += oppIn(sn, a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); } return s / n; };
  const gside = (a, t, off) => { const gx = ownX(t), dx = gx - a.x, dy = H / 2 - a.y, L = Math.hypot(dx, dy) || 1; return { x: a.x + dx / L * off, y: a.y + dy / L * off }; };
  // Çekirdek uçarken: herkes yörüngede nereye, ne zaman varabileceğini hesaplar
  // Alıcı da Çekirdeğin gerçekte gideceği yeri okur (pası atanın niyetini değil): hedefi yörüngede yetişebildiği ilk nokta.
  // Alıcının karşılama noktası (maç ve akıl aynı kuralı kullanır): en erken yetiştiği yere değil, yetişebildiği yerlerin en rahatına gider: rakipten uzak (boşluğu korur), rakipten önce varabildiği, fazla beklemeden. Geniş duran oyuncu kendiliğinden Çekirdeğe doğru içeri dalmaz.
  // pts: Çekirdeğin yolu · ps: oyuncular · rc: alıcının tepki süresi · iMax: bu tikten sonrası seçilmez (alıcı Çekirdekten kaçmaz, ona gelir)
  function alici(pts, p, ps, rc, iMax) { const nb = p.role !== 'Bekçi', KR0 = window.AlanCore.KR + .3, inK = q => Math.hypot(q.x, q.y - 25) < KR0 || Math.hypot(q.x - 100, q.y - 25) < KR0, v = spd(p) * SPR, opp = ps.filter(o => o.team !== p.team && o.role !== 'Bekçi'), oT = opp.map(o => Math.max(0, 14 - ok(o) * S.react)), oV = opp.map(o => spd(o) * SPR), oX = opp.map(o => o.x), oY = opp.map(o => o.y), px = p.x, py = p.y; let bs = -1e9, best = null; /* hız: rakiplerin tepki süresi ve koşu hızı bir kez hesaplanır (aynı sayılar) */
    for (let i = 0; i < pts.length; i += 2) { if (i > iMax) break; /* hız: sonraki anlar zaten seçilemez */ const q = pts[i]; if (nb && inK(q)) continue; const tr = rc + Math.max(0, Math.hypot(q.x - px, q.y - py) - 1.4) / v; if (tr > i) continue; let td = 1e9; for (let k = 0; k < opp.length; k++) td = Math.min(td, oT[k] + Math.max(0, Math.hypot(q.x - oX[k], q.y - oY[k]) - 1.4) / oV[k]); /* hız: konumlar döngü dışında okunur */ if (td < i) continue; const margin = td - i, sc = Dd().freeAt(ps, p.team, q.x, q.y) - S.rcvRisk * Math.exp(-margin / S.rcvMarginT); if (sc > bs) { bs = sc; best = { t: i, x: q.x, y: q.y }; } }
    return best; }
  function flightRead(m) {
    const b = m.ball, K = C(), out = new Map(); if (!b) return null;
    // kafadaki oyunda (lite) yörünge her tik değil, 6 tikte bir yeniden hesaplanır (aradaki tiklerde aynı yörünge, Çekirdeğin ilerlediği kadar kaydırılarak kullanılır)
    let pts; if (b.done) pts = [{ x: b.x, y: b.y }]; else if (m._lite && b._pc && b.t - b._pc.t < 6) pts = b._pc.pts.slice(b.t - b._pc.t); else { pts = K.predict(b, m.ps, null, 240).pts; if (m._lite) b._pc = { t: b.t, pts }; }
    for (const p of m.ps) {
      if (p.role === 'Bekçi' && !S.bekSerbest && Math.hypot(p.x - ownX(p.team), p.y - 25) > 12) continue; /* v5 · Bekçi serbest: boşa çıkan Çekirdeğe ve arkaya atılan pasa herkes gibi yarışır (Kuyu'dan çıkmanın riski kendisinin) */
      const nb = p.role !== 'Bekçi', KR0 = window.AlanCore.KR + .3, inK = q => Math.hypot(q.x, q.y - 25) < KR0 || Math.hypot(q.x - 100, q.y - 25) < KR0; /* v5 · Bekçi dışında kimse Kuyu alanına giremez: o alandaki noktalara yetişemez (eskiden savunmacı Kuyu içinde yetişeceğini sanıyor, Bekçi de karışmıyordu) */ const rc = p.team === b.team && m.fl && m.fl.q === p ? 0 : Math.max(0, 14 - ok(p) * S.react - (m.tick - (m.fl ? m.fl.t0 || m.tick : m.tick))), v = spd(p) * SPR; let best = null;
      const isRecv = p.team === b.team && m.fl && m.fl.q === p;
      if (isRecv) {
        best = alici(pts, p, m.ps, rc, p._rcvB === b ? p._rcvAbs + 2 - m.tick : 1e9);
        if (best) { p._rcvB = b; p._rcvAbs = m.tick + best.t; } // seçilen karşılama anı sonradan ileri kaydırılmaz: alıcı Çekirdekten kaçmaz, ona gelir
      }
      if (!best) for (let i = 0; i < pts.length; i += 2) { const q = pts[i]; if (nb && inK(q)) continue; const tr = rc + Math.max(0, Math.hypot(q.x - p.x, q.y - p.y) - 1.4) / v; if (tr <= i) { best = { t: i, x: q.x, y: q.y }; break; } }
      if (!best) { let q = pts[pts.length - 1]; if (nb && inK(q)) { for (let i = pts.length - 1; i >= 0; i--) if (!inK(pts[i])) { q = pts[i]; break; } } best = { t: Math.max(pts.length, rc + Math.hypot(q.x - p.x, q.y - p.y) / v), x: q.x, y: q.y }; }
      best.est = best.t * (1 + hsh(p.id, m.fl ? m.fl.t0 || 0 : 0, 3) * (20 - ok(p)) * .04); out.set(p, best);
    }
    let first = null; for (const [p, r] of out) if (!first || r.t < out.get(first).t) first = p;
    return { arr: out, first, pts };
  }
  function slotBase(m, p, E, att) {
    const t = p.team, tac = m.tac[t], s = att && p.rslot ? p.rslot : p.slot, x0 = ownX(t), dir = dirOf(t), bd = (E.x - x0) * dir;
    let back, len, wid, sh;
    // hücum şekli: arka çizgi Çekirdeğe göre, ön çizgi rakibin son savunmacısının omzunda (forvet son çizgiyi iter)
    if (att) { back = cl(bd - 18 - (1 - tac.risk) * 8, 10, 62); let last = 0; for (const q of m.ps) if (q.team !== t && q.role !== 'Bekçi') last = Math.max(last, (q.x - x0) * dir); const front = Math.min(94, Math.max(back + 24, last + 1)); len = front - back; wid = GEN[tac.genislik] || 38; sh = .15; }
    else { back = cl(Math.min(BLOK[tac.blok] || 34, bd - 6), 8, 62); len = 22; wid = 26; sh = .4; }
    return { x: cl(x0 + dir * (back + (s.d - .2) / .6 * len), 2, 98), y: cl(25 + s.l * wid / 2 + (E.y - 25) * sh, 2, 48) };
  }
  function bekci(p, E, m) { const gx = ownX(p.team), dx = E.x - gx, dy = E.y - 25, L = Math.hypot(dx, dy) || 1; const Qc = window.AlanCore.Q, cov = (Qc.bekBody ?? .9) + (Qc.bekDive ?? 2) * .8, Mo0 = window.AlanCore.MOUTH; const MoE = Math.max(.4, Mo0 * Math.abs(dx) / L); let r = cl(L * cl(1 - cov / MoE, .1, .55) + (ok(p) - 10) * .08, 2.5, window.AlanCore.KR); /* ağzın Çekirdekten görünen genişliği: dar açıdan (kanat, köşe) ağız dar görünür, Bekçi kalesine yakın kalır; ortadan geniş görünür, öne çıkar */ /* derinlik Kuyu ağzına göre: ağız ne kadar genişse Bekçi o kadar öne çıkar ki uzanabildiği genişlik açıyı kapatsın (ağzın yarısı / uzanma). Okuma ince ayar */ const hh = m && m.holder; if (hh && hh.team !== p.team && L < 30) { let dmin = 1e9; for (const q of m.ps) { if (q.team !== p.team || q === p) continue; const ux = gx - hh.x, uy = 25 - hh.y, ul = Math.hypot(ux, uy) || 1, al = ((q.x - hh.x) * ux + (q.y - hh.y) * uy) / ul; if (al > 2 && al < ul - 1) dmin = Math.min(dmin, Math.abs(-(q.x - hh.x) * uy + (q.y - hh.y) * ux) / ul); } const w = Math.max(cl((dmin - 1.5) / 3, 0, 1), cl((24 - L) / 10, 0, 1)) * cl((30 - L) / 6, 0, 1); /* taşıyıcının dibindeki presçi göndermeyi kapatmaz: ancak taşıyıcının en az 2 birim önündeki savunmacı hattı kapatır. Taşıyıcı yaklaştıkça (24 birimden itibaren, 14'te tam) Bekçi baskı olsa da öne çıkar */ /* açıklık sürekli: hatta savunmacı yaklaştıkça öne çıkma azalır (eskiden aç-kapa sıçrıyordu, Bekçi yetişemiyordu) */ const KRx = window.AlanCore.KR, rOut = cl(L < KRx + 6 ? L - 1.2 - (20 - ok(p)) * .05 : Math.max(L * (.35 + ok(p) * .01), L - (6 - ok(p) * .15)), 2.5, Math.max(2.5, Math.min(KRx + 4, L - (L < KRx + 6 ? 1.2 : 2.5)))); /* taşıyıcı alanın dibine gelince Bekçi üstüne çıkar (ikili mesafesine) */ r = r + (rOut - r) * w; /* öne çıkış: Kuyu alanının dışına (en çok alan + 4), taşıyıcının 3–4,5 birim önüne kadar (Okuma yüksekse daha cesur) */ } /* v5 · karşı karşıya (arada savunmacı yok): Bekçi öne çıkıp açıyı daraltır; ne kadar çıkacağını Okuma belirler, taşıyıcının 3 birim önünde durur (gövdesine çarpıp geçilmemek için) */ const _r = r, Mo = window.AlanCore.MOUTH, a1x = gx - E.x, a1y = 25 - Mo - E.y, a2x = gx - E.x, a2y = 25 + Mo - E.y, l1 = Math.hypot(a1x, a1y) || 1, l2 = Math.hypot(a2x, a2y) || 1; let bx = a1x / l1 + a2x / l2, by = a1y / l1 + a2y / l2; const bl = Math.hypot(bx, by) || 1; bx /= bl; by /= bl; const st = Math.max(0, L - _r); return { x: E.x + bx * st, y: E.y + by * st }; } /* v5 · açıortay: topla iki direk arasındaki açının ortasında, derinlik r */
  function position(m) {
    const ps = m.ps, h = m.holder, sh = m.sh || (m.sh = {});
    for (const p of ps) { (p.hist || (p.hist = [])).push({ x: p.x, y: p.y, vx: p.vx || 0, vy: p.vy || 0 }); if (p.hist.length > 14) p.hist.shift(); }
    if (m.tick % 20 === 0) for (const p of ps) { (p.past || (p.past = [])).push({ x: p.x, y: p.y }); if (p.past.length > 4) p.past.shift(); } // 1 saniyelik hafıza (20 tikte bir)
    let att = h ? h.team : m.ball.team, E = h ? { x: h.x, y: h.y } : null, fr = null;
    if (!h) { fr = (sh.fr && sh.frT > m.tick - 3 && sh.frB === m.ball) ? sh.fr : (sh.fr = flightRead(m), sh.frT = m.tick, sh.frB = m.ball, sh.fr); const f = fr.arr.get(fr.first); E = { x: f.x, y: f.y }; const other = [...fr.arr].filter(([p]) => p.team !== fr.first.team).reduce((mn, [, r]) => Math.min(mn, r.t), 1e9); if (other > f.t * 1.25 + 4) att = fr.first.team; }
    if (sh.att !== att) { const fresh = sh.att != null; sh.att = att; sh.gain = m.tick; for (const p of ps) { p.lock = 0; p.rec = fresh && p.team !== att && p.role !== 'Bekçi' && (p.x - E.x) * dirOf(p.team) > 2 ? m.tick + Math.max(2, Math.round(26 - ok(p) * S.recOk)) : 0; } }
    if (sh.ball !== m.ball || (m.ball && (m.ball.defl || 0) !== sh.defl)) { sh.ball = m.ball; sh.ballT = m.tick; sh.defl = m.ball ? m.ball.defl || 0 : 0; }
    const since = m.tick - sh.gain;
    for (const p of ps) { p.sprint = false; p.press = p.lock > m.tick && !p.noTouch; p.job = p.noTouch ? 'geçildi' : ''; }
    // 1 · Uçan / boştaki Çekirdek: tepki süresinden sonra "ilk ben varırım" diye düşünen gider; bariz değilse ikinci de gider
    const chase = new Set();
    if (fr) for (const t of [0, 1]) {
      const own0 = m.fl && m.fl.q && !m.ball.defl && !m.ball.done && m.ball.team === t, mine = [...fr.arr].filter(([p]) => p.team === t && !p.noTouch && !(p.role === 'Bekçi' && own0 && m.fl.q !== p)); if (!mine.length) continue; /* v5 · kendi pasımıza Bekçi koşmaz (alıcısı değilse) */ const bestE = Math.min(...mine.map(([, r]) => r.est));
      const loose = !m.fl || !m.fl.q || m.ball.defl || m.ball.done, rq = m.fl && m.fl.q ? fr.arr.get(m.fl.q) : null, recvFirst = rq && rq.est <= bestE + 10; /* v5 · alıcının önceliği: bir arkadaş ancak belirgin şekilde (10+ tik) önce varacaksa pası o alır */
      const ownPass = !loose && m.ball.team === t, firstMate = ownPass && !recvFirst ? mine.reduce((A2, B2) => (B2[1].est < A2[1].est ? B2 : A2))[0] : null; /* v5 · kendi pasımız havadayken tek kişi gider */
      const toMate = !loose && m.ball.team === t && recvFirst;
      for (const [p, r] of mine) { const recv = m.fl && m.fl.q === p && !loose; if (toMate && !recv) continue; if (ownPass && !toMate && p !== firstMate) continue; const rt = loose ? S.reactLoose - ok(p) * S.reactLooseOk : 14 - ok(p) * S.react; if (!recv && p.chB !== m.ball && m.tick - sh.ballT < rt) continue; const kaleye = p.role === 'Bekçi' && fr.pts && fr.pts.some(q => Math.hypot(q.x - ownX(p.team), q.y - 25) < window.AlanCore.KR + 6); /* v5 · Çekirdek Kuyu'ya doğru geliyorsa Bekçi de herkes gibi (hatta uzanması geniş olduğu için daha erken) karışır */
          if (recv || r.est <= bestE + (p.role === 'Bekçi' ? (kaleye ? (20 - ok(p)) * S.marginT + 5 : 0) : (20 - ok(p)) * S.marginT + 1)) { /* v5 · Bekçi Kuyu'yu ancak topa takımında ilk o varacaksa bırakır: payı yok */ chase.add(p); p.tx = r.x; p.ty = r.y; p.sprint = true; p.job = recv ? 'alıcı' : 'kovala'; p.chB = m.ball; } }
    }
    if (m.fl && m.fl.kind === 'gönder' && m.ball && !m.ball.done) { const gx = oppX(m.ball.team), dr = dirOf(m.ball.team); for (const p of ps) { if (p.team !== m.ball.team || p.role === 'Bekçi' || p === m.ball.from || chase.has(p)) continue; if (Math.hypot(p.x - gx, p.y - 25) < 28 && m.tick - sh.ballT >= 14 - ok(p) * S.react) { chase.add(p); p.tx = gx - dr * (5 + (p.id % 3) * 2); p.ty = 25 + ((p.id % 4) - 1.5) * 4; p.sprint = true; p.job = 'sekme'; } } }
    const dt = 1 - att, dtac = window.AlanMatch && window.AlanMatch.tacEff ? window.AlanMatch.tacEff(m, dt) : m.tac[dt];
    // 2 · Savunma: arkada kalan, pres, geri koşu, bölge
    const defs = ps.filter(p => p.team === dt && p.role !== 'Bekçi' && !chase.has(p) && !p.noTouch);
    const nSup = defs.filter(p => p.rs === 'Süpürücü').length, nOn = defs.filter(p => p.rs === 'Önde').length, sweep = new Set(defs.filter(p => p.rs !== 'Önde').sort((u, v) => ((v.rs === 'Süpürücü') - (u.rs === 'Süpürücü')) || u.slot.d - v.slot.d).slice(0, Math.max(dtac.arkada, nSup))), up = new Set(defs.filter(p => !sweep.has(p)).sort((u, v) => ((v.rs === 'Önde') - (u.rs === 'Önde')) || v.slot.d - u.slot.d).slice(0, Math.max(nOn, dtac.onde || 0)));
    const press = [], pLim = presLim(m, dtac, h, att);
    if (h) {
      const mp = macPres(m, 1 - att), BL = ['Düşük', 'Orta', 'Yüksek'], bi = Math.max(0, Math.min(2, BL.indexOf(dtac.blok) + (mp > 0 || (dtac.yz && (window.AlanMatch.macEff(m, 1 - att, {}).macK ?? 1) < .6) ? 1 : 0))), still = Math.hypot(h.vx || 0, h.vy || 0) < .04 ? m.tick - (h._dur = h._dur ?? m.tick) : (h._dur = null, 0), sd = Math.min(1, (dtac.siddet ?? .5) + (mp > 0 ? .25 : 0)), /* YZ yenikken şiddet +0,25 */ Th = (S.prT0 + Math.max(0, m.decT || 0) + still * S.prStill) * (.6 + .8 * sd), /* pres şiddeti: sert takım daha uzaktan çıkar, temkinli takım ancak yakındayken */ gainH = .4 + threat(att, h.x, h.y), /* v5 · pres mekanik: taşıyıcıya onun bir sonraki hamlesinden önce varabilir miyim (Th: karar süresi + yerinde durduğu süre; uzun tutan taşıyıcıya varmak kolaylaşır) ve basmak, arkamda bırakacağım rakipten değerli mi? Blok mesafe sınırı değil, sadece takımın durduğu yer */ cand = defs.filter(p => !sweep.has(p) && !up.has(p)).sort((u, v) => ((v.rs === 'Presçi') - (u.rs === 'Presçi')) * 1e3 + Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y));
      const dG = Math.abs(h.x - ownX(dt)), hat = S.prHat[BL[bi]] ?? 62, hatK = dG < hat ? 1 : still > (h.role === 'Bekçi' ? S.prHold : S.prHold * 2.5) ? .9 : S.prDis * Math.max(0, 1 - (dG - hat) / 25) + (dG < hat + 8 ? .25 : 0), /* blok = presin başladığı hat: taşıyıcı hattı geçince basılır. Hattın gerisinde (rakip kendi sahasında) yalnız fırsat çok iyiyse ya da topu uzun tutuyorsa (zaman geçiriyor, Bekçi dahil) çıkılır: hiç basmamak yok */ atkP = ps.filter(q => q.team === att && q.role !== 'Bekçi' && q !== h), LO = window.AlanDecide.laneOk, prV = p => { const tR = Math.max(0, 14 - ok(p, 'tepki') * S.react) + Math.max(0, Math.hypot(p.x - h.x, p.y - h.y) - 2.4) / (spd(p) * SPR), P = 1 / (1 + Math.exp((tR - Th) / 8)), bs = slotBase(m, p, E, false); let cost = 0; for (const q of atkP) { if (Math.hypot(q.x - bs.x, q.y - bs.y) < 9) cost = Math.max(cost, threat(att, q.x, q.y) * (LO ? .3 + .7 * LO(ps, att, h, q) : 1)); } return { P, v: P * gainH * hatK * (p.rs === 'Presçi' ? 1.3 : 1) - cost * S.prCost * (1.4 - .8 * sd) }; }; /* sert takım arkasında bıraktığını daha az önemser */ /* sert takım arkasında bıraktığını daha az önemser */
      for (const p of cand.map(p => [p, prV(p)]).sort((a, b) => b[1].v - a[1].v)) { if (press.length >= pLim) break; const [q, e] = p; if ((q.lock > m.tick && hatK >= .5) || (e.P * Math.min(1, hatK * 2) > S.prPmin && e.v > 0)) press.push(q); } /* pres kilidi hattın gerisinde geçmez: taşıyıcı pası verip geri dönünce presçi peşinden kendi sahasının dışına çıkmaz */ /* değeri en yüksek olan önce basar; Presçi rolü daha istekli */
      press.sort((u, v) => (u.lock > m.tick ? u.lockAt : m.tick) - (v.lock > m.tick ? v.lockAt : m.tick) || Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y));
      for (const p of defs) if (!press.includes(p)) p.lock = 0;
    }
    const atkAll = ps.filter(q => q.team === att && q.role !== 'Bekçi'), atk = atkAll.filter(q => q !== h);
    const covered = h ? [Math.atan2(25 - h.y, ownX(dt) - h.x)] : [];
    press.forEach((p, k) => {
      if (!(p.lock > m.tick)) { p.lock = m.tick + S.lock; p.lockAt = m.tick + k * .01; } p.press = true; p.sprint = true; p.job = 'pres';
      const sn = seen(m, p);
      if (k === 0) { const DD = window.AlanDecide, cs = DD.coreSide(h), B = C().Q.body, co = { x: h.x + cs.ux * B, y: h.y + cs.uy * B }, d0 = Math.hypot(p.x - h.x, p.y - h.y), trap = dtac.sistem === 'Kenara sıkıştır', okp = (p.a && p.a.okuma) ?? 10;
        if (d0 > 4) { const g = trap ? { x: h.x - dirOf(dt) * 1.2, y: h.y + (h.y < 25 ? 2 : -2) } : gside(h, dt, 2); p.tx = g.x; p.ty = g.y; return; }
        // presçinin planı: Çekirdek açıktaysa doğrudan; gövdenin arkasındaysa dolaşarak. Bazen bir yanı gösterip öbür yandan girer (fake): taşıyıcı onun gidişini okuyup Çekirdeği ters yana saklar, fake bunu kullanır. Fake ihtimali Okuma ile artar.
        if (!p.pplan || m.tick >= p.pplan.until) { const shd = C().Turn.shielded(h, p, co.x, co.y), base = Math.atan2(p.y - h.y, p.x - h.x), ca = Math.atan2(cs.uy, cs.ux), side = Math.sign(Math.atan2(Math.sin(ca - base), Math.cos(ca - base))) || 1, rr = m.r ? m.r() : .5, fake = shd && rr < Math.max(0, okp - 6) * .08;
          p.pplan = fake ? { k: 'fake', s: -side, t0: m.tick, until: m.tick + 2 * DD.DU.fakeT } : { k: trap ? 'iç' : shd ? 'yan' : 'direkt', s: side, t0: m.tick, until: m.tick + 8 }; }
        const pl = p.pplan, base = Math.atan2(p.y - h.y, p.x - h.x), s2 = pl.k === 'fake' ? (m.tick - pl.t0 < DD.DU.fakeT ? pl.s : -pl.s) : pl.s;
        if (pl.k === 'direkt' || (pl.k === 'iç' && d0 < 2.4)) { p.tx = co.x; p.ty = co.y; } else if (pl.k === 'iç') { p.tx = h.x - dirOf(dt) * 1.2; p.ty = h.y + (h.y < 25 ? 1.6 : -1.6); } else { p.tx = h.x + Math.cos(base + s2 * 1.25) * 1.9; p.ty = h.y + Math.sin(base + s2 * 1.25) * 1.9; }
        p.job = pl.k === 'fake' ? (m.tick - pl.t0 < DD.DU.fakeT ? 'fake: gösteriyor' : 'fake: giriyor') : pl.k === 'iç' ? 'kenara itiyor' : 'pres'; return; }
      // sonrakiler: taşıyıcının en tehlikeli çıkışını (kendi okumasına göre) keser
      // kafes: sonrakiler taşıyıcıya yapışır, en tehlikeli çıkışını (kendi okumasına göre) dibinden kapatır. Bu sırada kendi adamını/bölgesini bırakır.
      const opts = atk.map(q => ({ q, v: threat(att, q.x, q.y) * Math.exp(-laneIn(sn, h, q) * 2), an: Math.atan2(q.y - h.y, q.x - h.x) })).sort((a, b) => b.v - a.v), dA = (x, y) => Math.abs(Math.atan2(Math.sin(x - y), Math.cos(x - y)));
      const q = opts.find(o => covered.every(c => dA(o.an, c) > .8)) || opts[0]; if (!q) return; covered.push(q.an);
      const dx = q.q.x - h.x, dy = q.q.y - h.y, L = Math.hypot(dx, dy) || 1; p.tx = h.x + dx / L * 2.5; p.ty = h.y + dy / L * 2.5;
    });
    // bölge eşleşmesi: her rakip, bölgesine düşen en yakın savunmacıya (Okuma bölgeyi büyütür); tehlikelisi önce
    const free = defs.filter(p => !sweep.has(p) && !up.has(p) && !press.includes(p)), base = new Map();
    for (const p of ps) if (!chase.has(p) && p !== h) base.set(p, p.role === 'Bekçi' ? bekci(p, E, m) : slotBase(m, p, E, p.team === att));
    /* v5 · her rakibi tek kişi tutar: arkada kalanlar en derindeki rakipleri sırayla paylaşır (iki süpürücü aynı forvete gitmez); bir süpürücünün tuttuğu rakibi bölge savunmacısı ayrıca tutmaz. Markajcı talimatı bundan bağımsız */
    const gD = q => Math.abs(q.x - ownX(dt)), swL = [...sweep].sort((u, v) => u.slot.d - v.slot.d), dng = q => threat(att, q.x, q.y) * (h && window.AlanDecide.laneOk ? .3 + .7 * window.AlanDecide.laneOk(ps, att, h, q) : 1), mkRef0 = new Set(free.filter(p => p.rs === 'Markajcı' && p.markRef && p.markRef.team === att).map(p => p.markRef)), deepA = atk.filter(q => gD(q) < gD(E) && !(h && q.id === h.id) && !mkRef0.has(q) /* Markajcının adamını arkadaki ayrıca tutmaz (Kuyu önünde iki kişi tek forvete kalıyordu) */ && Math.hypot(q.x - E.x, q.y - E.y) > 3) /* taşıyıcının kendisini arkadaki tutmaz; onu presçi karşılar */.sort((a, b) => dng(b) - dng(a)), dMax = deepA.length ? dng(deepA[0]) : 0, deepO = deepA.filter(q => dng(q) >= dMax * .35), swT = new Map(); swL.forEach((p, k) => swT.set(p, deepO[k] || null)); /* v5 · arkadaki rakibin tehlikesine göre tutar: Kuyu'ya yakınlık × Çekirdeğin ona ulaşabilmesi (pas hattı). Kuyu'nun dibinde ama Çekirdekten kopuk forvet az tehlikelidir; en tehlikelinin üçte birinden az tehlikeli rakibe adam ayrılmaz, süpürücü bölgesini tutar */
    const mark = new Map(), taken = new Set();
    for (const p of free) if (p.rs === 'Markajcı' && p.markRef && p.markRef.team === att && p.markRef !== h) { mark.set(p, p.markRef); taken.add(p); } // Markajcı bölgeye bakmaz, adamını takip eder
    const markedRef = new Set([...mark.values(), ...[...swT.values()].filter(Boolean)]);
    for (const q of atk.slice().sort((a, b) => threat(att, b.x, b.y) - threat(att, a.x, a.y))) {
      if (markedRef.has(q)) continue; const dr = dMax > 0 ? Math.min(1, dng(q) / dMax) : 0; let bp = null, bd = 1e9; for (const p of free) { if (taken.has(p)) continue; const sn = seen(m, p).find(o => o.ref === q), bs = base.get(p), d = Math.hypot(sn.x - bs.x, sn.y - bs.y); if (d < S.zoneR + ok(p) * S.zoneOk + S.zoneDng * dr && d < bd) { bd = d; bp = p; } } /* v5 · tehlikeli boş rakip bölgenin dışından da çeker (tehlike oranında): en tehlikeli rakip boş kalmaz */
      if (bp) { taken.add(bp); mark.set(bp, q); }
    }
    for (const p of defs) {
      if (press.includes(p)) continue; const bs = base.get(p), sn = seen(m, p); let tx = bs.x, ty = bs.y;
      if (!up.has(p) && p.rec && m.tick >= p.rec && since < S.recT && (p.x - E.x) * dirOf(p.team) > -1) { const g = { x: E.x + (ownX(dt) - E.x) * .35, y: cl(bs.y + (25 - bs.y) * .3, 3, 47) }; p.tx = g.x; p.ty = g.y; p.sprint = true; p.job = 'geri koşu'; continue; } /* v5 · geri koşu topun hizasına değil kendi bölgesinin hizasına: herkes Çekirdeğin olduğu köşeye koşmaz, savunma eni korunur */
      if (up.has(p)) { const ra = atkAll.filter(q => q !== h), rl = ra.length ? ra.reduce((u, v) => Math.abs(v.x - oppX(dt)) < Math.abs(u.x - oppX(dt)) ? v : u).x : oppX(dt) - dirOf(dt) * 30; let wx = rl - dirOf(dt) * 3; if (Math.abs(wx - oppX(dt)) < 12) wx = oppX(dt) - dirOf(dt) * 12; p.tx = wx; p.ty = 25 + p.slot.l * 11; p.job = 'önde bekle'; continue; }
      if (sweep.has(p)) {
        const o = swT.get(p), so = o ? sn.find(z => z.ref === o) : null, k = swL.indexOf(p);
        if (so) { const g = gside(so, dt, 3 + ok(p) * .2); p.tx = g.x; p.ty = g.y; p.job = 'arkada · ' + (o.name || ''); continue; }
        /* tutacak derin rakip kalmadıysa: Çekirdekle Kuyu arasındaki alanı kapatır; ikinci boştaki süpürücü daha derinde ve yana açık durur (üst üste binmez) */ const nFree = swL.filter(q => !swT.get(q)).indexOf(p), g = gside(E, dt, 3 + ok(p) * .2 + nFree * 7); p.tx = g.x; p.ty = cl(g.y + (nFree ? (k % 2 ? 6 : -6) : 0), 3, 47); p.job = 'arkada · alan'; continue;
      }
      const q = mark.get(p);
      if (q && p.rs === 'Markajcı') { const o = sn.find(o => o.ref === q), g = gside(o, dt, S.markOff); tx = g.x; ty = g.y; p.job = 'markaj · ' + (q.name || ''); p.tx = tx; p.ty = ty; continue; }
      if (q) { const o = sn.find(o => o.ref === q), g = gside(o, dt, S.markOff), w = S.markW * (.6 + ok(p) * .02); tx += (g.x - tx) * w; ty += (g.y - ty) * w; p.job = 'bölge · ' + (q.name || ''); }
      else { const dx = ownX(dt) - E.x, dy = 25 - E.y, L = Math.hypot(dx, dy) || 1, u = ((tx - E.x) * dx + (ty - E.y) * dy) / L, px = E.x + dx / L * u, py = E.y + dy / L * u; tx += (px - tx) * .3; ty += (py - ty) * .3; p.job = 'bölge'; }
      // Kuyu tarafı: Çekirdek yaklaştıkça Çekirdek ile Kuyu arasına
      const dk = Math.hypot(E.x - ownX(dt), E.y - 25), st = 26 + ok(p) * 1.4 - ((BLOK[dtac.blok] || 34) - 34) * .3, w = cl((st - dk) / Math.max(6, st - 10), 0, 1);
      if (w > 0) { const qo = q ? sn.find(o => o.ref === q) : null, T = qo || E, dT = Math.hypot(T.x - ownX(dt), T.y - 25), hold = 6 + p.slot.d * 10, f = Math.min(1, hold / Math.max(dT, 1)), gx = ownX(dt) + (T.x - ownX(dt)) * f, gy = 25 + (T.y - 25) * f + (qo ? 0 : p.slot.l * 8); tx += (gx - tx) * w * (qo ? .6 : 1); ty += (gy - ty) * w * (qo ? .6 : 1); } /* v5 · Kuyu tarafı: adamı olan, kendi adamıyla Kuyu arasına girer (Çekirdekle Kuyu arasına değil); herkes topa çökmesin, Kuyu önündeki boş forvet kalmasın */
      p.tx = tx; p.ty = ty;
    }
    /* v5 · alan paylaşımı: iki savunmacının hedefi birbirine yakınsa (aynı yeri kapatıyorlar), adamı olmayan ya da daha az tehlikeli rakibi tutan yana, açıkta kalan tarafa kayar. Tehdit tek kişiyle sayılır */
    { const zs = defs.filter(p => !press.includes(p) && !up.has(p) && p.job !== 'geri koşu'), pr = p => mark.get(p) ? threat(att, mark.get(p).x, mark.get(p).y) + 1 : sweep.has(p) ? .5 : 0;
      for (let it = 0; it < 2; it++) for (let i = 0; i < zs.length; i++) for (let j = i + 1; j < zs.length; j++) { const a = zs[i], b = zs[j], dx = b.tx - a.tx, dy = b.ty - a.ty, dd = Math.hypot(dx, dy); if (dd >= S.ayr) continue; const mv = pr(a) >= pr(b) ? b : a, st = mv === a ? b : a; if (mark.get(mv) && /markaj/.test(mv.job || '')) continue; let nx = mv.tx - st.tx, ny = mv.ty - st.ty, nl = Math.hypot(nx, ny); if (nl < .3) { nx = 0; ny = mv.ty < 25 ? -1 : 1; nl = 1; } const k = S.ayr - dd; mv.tx = cl(mv.tx + nx / nl * k, 2, 98); mv.ty = cl(mv.ty + ny / nl * k, 2, 48); } }
    /* v5 · savunma takım işi (ortak atama): bütün görevlerin (pres, geri koşu, bölge, arkada, kovalama) hedefleri seçildikten sonra tehlikeler tek elden dağıtılır. Tehlikeli bir rakip kimse tarafından kapatılmıyorsa, işi gereksiz olan savunmacı (Çekirdeğin dibindeki fazlalık, aynı yeri kapatan ikinci kişi, gereksiz geri koşan ya da kovalayan) onun Kuyu tarafına gider */
    if (h && h.team === att) { const all = ps.filter(p => p.team === dt && p.role !== 'Bekçi' && !p.noTouch), kilit = new Set([press[0]].filter(Boolean)), hA = atk.filter(q => q !== h).map(q => ({ q, v: dng(q) })).filter(x => x.v >= Math.max(.02, dMax * S.ortakMin)).sort((u, v) => v.v - u.v);
      const kapat = (p, q) => { const g = gside(q, dt, S.markOff); return Math.hypot(p.tx - g.x, p.ty - g.y) < S.ortakR || (Math.hypot(p.x - q.x, p.y - q.y) < S.ortakR && Math.hypot(p.x - ownX(dt), p.y - 25) < Math.hypot(q.x - ownX(dt), q.y - 25)); };
      const fazla = p => { if (kilit.has(p) || /markaj/.test(p.job || '')) return false; if (/geri koşu|kovala|sekme/.test(p.job || '')) return true; const nb = all.filter(o => o !== p && Math.hypot(o.tx - p.tx, o.ty - p.ty) < S.ayr).length; if (nb) return true; if (Math.hypot(p.tx - h.x, p.ty - h.y) < 7 && !press.includes(p)) return true; return !mark.get(p) && !sweep.has(p); };
      const kul = new Set();
      for (const { q } of hA) { if (all.some(p => kapat(p, q))) continue; let bp = null, bc = 1e9; for (const p of all) { if (kul.has(p) || !fazla(p)) continue; const g = gside(q, dt, S.markOff), c = Math.hypot(p.x - g.x, p.y - g.y) / (spd(p) * SPR); if (c < bc) { bc = c; bp = p; } } if (!bp) continue; kul.add(bp); const g = gside(q, dt, S.markOff); bp.tx = g.x; bp.ty = g.y; bp.sprint = bc > 20; bp.job = 'kapatma · ' + (q.name || ''); } }
    // 3 · Hücum: arkada güvence, sonra boşluk arama
    const hang = ps.filter(q => q.team === dt && q.role !== 'Bekçi' && (q.x - E.x) * dirOf(att) < -8), guard = new Map();
    { const kWin = m.tac[att].kazaninca === 'Kontra' && m.winTeam === att && m.tick - m.winT < 180, nG = Math.max(0, kWin ? Math.min(hang.length, m.tac[att].arkada ?? 1) : hang.length - (m.tac[att].risk > .7 ? 1 : 0)), // Kontrada geride sadece "Arkada kalan" kadar oyuncu güvence olur; kalanı ileri koşar
      pool = atk.filter(q => !chase.has(q)).sort((u, v) => u.slot.d - v.slot.d);
      for (const hq of hang.sort((a, b) => threat(dt, b.x, b.y) - threat(dt, a.x, a.y)).slice(0, nG)) { let bi = null, bd = 1e9; for (const q of pool) { if (guard.has(q)) continue; const d = Math.hypot(q.x - hq.x, q.y - hq.y) + q.slot.d * 30; if (d < bd) { bd = d; bi = q; } } if (bi) guard.set(bi, hq); } }
    const Wt = S.W, kz = m.tac[att].kazaninca || 'Dengeli', win = m.winTeam === att && m.tick - m.winT < 180;
    // Kazanınca talimatı (geçiş penceresi, 3 sn): Kontra = öndeki 3 topsuz oyuncu dizilişe bağlılığı gevşetip rakip Kuyu'ya doğru boşluğa koşar; Yerleş = dizilişteki yere sıkı bağlılık, ileri koşu yok.
    if (win && kz === 'Kontra' && m._runT !== m.winT) { m._runT = m.winT; m._run = new Set(atk.filter(q => !guard.has(q) && q !== h).sort((u, v) => (v.x - u.x) * dirOf(att) + (v.slot.d - u.slot.d) * 10).slice(0, m.tac[att].kontraN ?? 3)); } // koşucular geçiş başında bir kez seçilir (kaç kişi: Kontraya katılan)
    const runners = win && kz === 'Kontra' && m._run ? new Set([...m._run].filter(q => q !== h && !chase.has(q) && !guard.has(q))) : new Set();
    // Kontra koşusunun değeri: oraya rakipten önce varabilir miyim (tepki gecikmesiyle) × oranın tehlikesi. Pas seçeneği olmak değil, savunmanın arkasındaki boşluğu almak.
    const runV = (p, c, sn) => { const ta = Math.hypot(c.x - p.x, c.y - p.y) / (spd(p) * SPR); let td = 1e9; for (const o of sn) { if (o.role === 'Bekçi') continue; td = Math.min(td, Math.max(0, 14 - ok(o) * S.react) + Math.hypot(c.x - o.x, c.y - o.y) / (spd(o) * SPR)); } return threat(att, c.x, c.y) / (1 + Math.exp((ta - td) / 8)); };
    const roleLeash = p => p.rh === 'Kurucu' ? 2 : p.rh === 'Pivot' || p.rh === 'Kanat' ? 1.5 : p.rh === 'Koşucu' ? .7 : 1;
    const roleY = (p, c) => (p.rh === 'Pivot' && S.pivotGap ? 1 * Math.max(0, S.pivotGap - Math.hypot(c.x - oppX(att), c.y - 25)) / 4 : 0) + (p.rh === 'Kanat' ? S.kanatK * Math.max(0, 13 - Math.abs(c.y - 25)) / 13 : p.rh === 'Kurucu' ? S.kurucuK * Math.max(0, (c.x - E.x) * dirOf(att) + 2) / 10 : 0); // talimatın dışına çıkmanın bedeli
    const roleRun = p => p.rh === 'Koşucu' && !runners.has(p) ? S.runK * .6 : 0;
    const leashK = p => (win ? (kz === 'Kontra' ? (runners.has(p) ? .01 : .6) : kz === 'Yerleş' ? 2.5 : 1) : roleLeash(p));
    for (const p of atk) {
      if (chase.has(p)) continue;
      if (guard.has(p)) { const g = gside(guard.get(p), att, 3 + (20 - ok(p)) * .2); p.tx = g.x; p.ty = g.y; p.job = 'güvence'; p.otx = null; continue; }
      const every = Math.max(8, Math.round(24 - ok(p))); if (m._lite && p.otx == null) { const bs0 = base.get(p); p.otx = bs0.x; p.oty = bs0.y; } // kafadaki oyunda planı olmayan arkadaş görev yerine gider (yeniden tartmaz)
      if (win && p._winSeen !== m.winT && !m._lite) { p._winSeen = m.winT; p.otx = null; } // geçiş başladı: plan yeniden tartılır
      if (p.otx != null && (m._lite || (m.tick + p.id) % every)) { p.tx = p.otx; p.ty = p.oty; p.job = runners.has(p) ? 'kontra koşusu' : 'boşluk'; p.sprint = runners.has(p) || Math.hypot(p.tx - p.x, p.ty - p.y) > S.W.runSprint; continue; }
      const bs = base.get(p), sn = seen(m, p), cands = [];
      if (p.otx != null) cands.push({ x: p.otx, y: p.oty, keep: 1 });
      for (let gx = -16; gx <= 16; gx += 8) for (let gy = -16; gy <= 16; gy += 8) cands.push({ x: bs.x + gx, y: bs.y + gy }); if (p.otx != null) for (let k = 0; k < 6; k++) { const an = k * 1.047; cands.push({ x: p.otx + Math.cos(an) * 4, y: p.oty + Math.sin(an) * 4 }); }
      const lastD = sn.filter(o => o.role !== 'Bekçi').reduce((mx, o) => Math.max(mx, (o.x - ownX(att)) * dirOf(att)), 0);
      if (p.slot.d >= .6 || p.rh === 'Koşucu') for (const yy of [p.y, bs.y, 15, 35]) cands.push({ x: ownX(att) + dirOf(att) * Math.min(94, lastD + 4), y: yy });
      if (runners.has(p)) { const lastO = sn.filter(o => o.role !== 'Bekçi').reduce((mx, o) => Math.max(mx, (o.x - ownX(att)) * dirOf(att)), 0); for (const fx of [12, 22, 32]) for (const yy of [10, 18, 25, 32, 40]) cands.push({ x: cl(p.x + dirOf(att) * fx, 3, 97), y: yy }); for (const yy of [12, 25, 38]) cands.push({ x: cl(ownX(att) + dirOf(att) * (lastO + 4), 3, 97), y: yy }); }
      // Açılan boşluk: yerini bırakmış (başka birini takip eden, prese çıkan) savunmacının bölgesi. Oyuncu bunu kendi algısıyla görür (rakibi kusurlu ve gecikmeli görür);
      // kaç tanesini fark ettiğini Okuma belirler. Oraya gitmek talimattan sapma sayılmaz (bağ gevşer): boşluğu kullanmak hücumcunun işi.
      { const nV = Math.max(0, Math.round((ok(p) - S.vacOk0) / S.vacOkStep)); if (nV > 0) { const vac = []; for (const d of defs) { const bd = d.past && d.past.length >= 3 ? d.past[0] : null, sd = sn.find(o => o.ref === d); if (!bd || !sd) continue; if (Math.hypot(sd.x - bd.x, sd.y - bd.y) < S.vacLeft) continue; let near = 1e9; for (const o of sn) if (o.role !== 'Bekçi') near = Math.min(near, Math.hypot(o.x - bd.x, o.y - bd.y)); if (near < S.vacEmpty) continue; vac.push({ x: bd.x, y: bd.y, w: threat(att, bd.x, bd.y) }); }
        vac.sort((a, b) => b.w - a.w).slice(0, nV).forEach(v => { cands.push({ x: v.x, y: v.y, vac: 1 }); cands.push({ x: v.x + dirOf(att) * 4, y: v.y, vac: 1 }); }); } }
      if (p.rh === 'Kanat') for (const dx of [-8, 0, 8, 16]) cands.push({ x: cl(bs.x + dirOf(att) * dx, 3, 97), y: (p.rslot.l < 0 ? 4 : 46) });
      if (Math.abs(E.x - oppX(att)) < 40 && (p.slot.d >= .45 || p.rh === 'Pivot' || p.rh === 'Koşucu') && p.rh !== 'Kurucu') for (const r of [11, 19]) for (const an of [-.8, -.27, .27, .8]) cands.push({ x: oppX(att) - dirOf(att) * r * Math.cos(an), y: 25 + r * Math.sin(an) });
      const mates = atkAll.filter(q => q !== p).map(q => q === h ? { x: h.x, y: h.y } : { x: q.tx ?? q.x, y: q.ty ?? q.y });
      const nearOf = (x, y) => { let b = null, bd = 1e9; for (const o of sn) { if (o.role === 'Bekçi') continue; const d = Math.hypot(o.x - x, o.y - y); if (d < bd) { bd = d; b = o; } } return [b, bd]; };
      const mNear = mates.map(q => nearOf(q.x, q.y)), ePr = oppIn(sn, E.x, E.y), mateObjs = atk.filter(q => q !== p && !chase.has(q)).map(q => ({ x: q.tx ?? q.x, y: q.ty ?? q.y }));
      // iki aşama: önce dünyayı şu anki hâliyle (savunmanın cevabı olmadan) hızlıca tart, en umutlu birkaç noktada savunmanın cevabını da hesapla
      const pre = cands.filter(c => c.x >= 2 && c.x <= 98 && c.y >= 2 && c.y <= 48).map(c => { const lsh = Math.hypot(c.x - bs.x, c.y - bs.y) / 12 * (c.vac ? S.vacLeash : 1); return { c, q: (runners.has(p) ? S.runK * runV(p, c, sn) : 0) + roleRun(p) * runV(p, c, sn) - roleY(p, c) + teamV(m, sn, att, E, c, mateObjs, 0) - Wt.leash * leashK(p) * lsh * lsh + (c.keep ? Wt.hyst : 0) }; }).sort((a, b) => b.q - a.q).slice(0, Wt.deep);
      let best = null, bv = -1e9;
      for (const { c } of pre) {
        // Tek ölçü: burada durursam, savunma en iyi cevabını verdiğinde takımın en iyi seçeneği ne kadar iyi? (taşıyıcının kullandığı aynı değer ve hat hesabıyla)
        const lsh = Math.hypot(c.x - bs.x, c.y - bs.y) / 12 * (c.vac ? S.vacLeash : 1);
        const v = (runners.has(p) ? S.runK * runV(p, c, sn) : 0) + roleRun(p) * runV(p, c, sn) - roleY(p, c) + Wt.tv * teamV(m, sn, att, E, c, mateObjs, ok(p)) - Wt.leash * leashK(p) * lsh * lsh + (c.keep ? Wt.hyst : 0) + hsh(p.id, Math.round(c.x * 3 + c.y * 7), Math.floor(m.tick / 30)) * (20 - ok(p)) * Wt.noise * 2;
        if (v > bv) { bv = v; best = c; }
      }
      p.otx = best.x; p.oty = best.y; p.tx = best.x; p.ty = best.y; p.job = best.vac ? 'açılan boşluk' : p.rh && p.rh !== 'Serbest' ? p.rh.toLowerCase() : 'boşluk'; p.sprint = Math.hypot(p.tx - p.x, p.ty - p.y) > S.W.runSprint; // boşluğa koşu da koşudur: presçi nasıl koşarak geliyorsa, hücumcu da hedefi uzaksa koşarak gider
    }
    for (const p of ps) if (p.noTouch && p !== h) { if (p.stunD && p.stun0 != null && m.tick - p.stun0 < (p.stun - p.stun0) * .6) { p.tx = p.x + p.stunD.x * 6; p.ty = p.y + p.stunD.y * 6; p.sprint = true; p.job = 'geçildi · kayıyor'; } else { const g = gside(E, p.team, 5); p.tx = g.x; p.ty = g.y; p.sprint = true; p.job = 'geçildi · dönüyor'; } } /* v5 · çalım yiyen donmaz: atıldığı yöne kayar (momentum), sonra Kuyu tarafına döner */
    if (h) { const dd = ps.filter(q => q.team !== att && q.role !== 'Bekçi' && !chase.has(q) && !q.noTouch).sort((u, v) => Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y))[0]; const ddD = dd ? Math.hypot(dd.x - h.x, dd.y - h.y) : 1e9; if (dd && ddD < 9 && !dd.press) { if ((m.tick + dd.id) % 4 === 0 || dd._cb == null) dd._cb = coverBoth(m, dd, att, E, h);
        /* v5 · geri geri savunma (jokey): yakındaki savunmacı sabit bir noktada beklemez; taşıyıcıyla kendi Kuyu'su arasında, taşıyıcının hızına göre mesafe koruyarak geri çekilir ve onunla birlikte kayar. Uzaktayken iki işi (taşıyıcı + pas hattı) kapatır, yaklaştıkça taşıyıcıya döner. Taşıyıcı durursa mesafeyi kapatır (ikiliye hazırlanır) */
        const kx = ownX(dd.team), hsp = Math.hypot(h.vx || 0, h.vy || 0), gl = Math.hypot(kx - h.x, 25 - h.y) || 1, mes = cl(2.2 + hsp * 14 - (hsp < .05 ? 1 : 0), 1.6, 5), jx = h.x + (kx - h.x) / gl * mes + (h.vx || 0) * 6, jy = h.y + (25 - h.y) / gl * mes + (h.vy || 0) * 6, w = cl((9 - ddD) / 6, 0, 1);
        if (dd._cb) { dd.tx = dd._cb.x * (1 - w) + jx * w; dd.ty = dd._cb.y * (1 - w) + jy * w; dd.sprint = Math.hypot(dd.tx - dd.x, dd.ty - dd.y) > 1.5; dd.job = w > .5 ? 'geri geri' : 'ikisini kapat'; } } }
    if (h && h.role === 'Bekçi') { const kg = ownX(h.team), KRk = window.AlanCore.KR, cs = Dd().coreSide(h), cx = h.x + cs.ux * window.AlanCore.Q.body, cy = h.y + cs.uy * window.AlanCore.Q.body, dc = Math.hypot(cx - kg, cy - 25); if (dc < KRk + 1.5) { const q = ps.filter(z => z.team !== h.team && z.role !== 'Bekçi' && !z.noTouch).sort((u, v) => Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y))[0]; if (q && Math.hypot(q.x - h.x, q.y - h.y) < 6) { const ux = (cx - kg) / (dc || 1), uy = (cy - 25) / (dc || 1); q.tx = kg + ux * (KRk + .35); q.ty = 25 + uy * (KRk + .35); q.sprint = true; q.job = 'pres · Çekirdeğe uzan'; } } } /* v5 · Bekçi Çekirdeği alanın kenarında saklıyorsa en yakın rakip, alanın sınırında Çekirdeğe en yakın noktaya gider (gövdesi giremez, menzili girer) */
    for (const p of ps) if (p.role === 'Bekçi' && !chase.has(p) && p !== h) { const b = base.get(p); p.tx = b.x; p.ty = b.y; p.job = 'Bekçi'; p.sprint = Math.hypot(b.x - p.x, b.y - p.y) > 1.2; } /* v5 · Bekçi açıyı kapatmak için koşar (eskiden hep yürüyordu, geride kalıyordu) */
    for (const p of ps) { p.tx = cl(p.tx, 1.5, 98.5); p.ty = cl(p.ty, 1.5, 48.5); if (p.role !== 'Bekçi' && window.AlanMatch.kuyuSafe) { const q = window.AlanMatch.kuyuSafe(p.tx, p.ty); if (q) { p.tx = q.x; p.ty = q.y; } } } // hedef siyah alanın içindeyse kenarına alınır
    m.E = E; m.att = att;
  }
  // Hat açık mı (gövde ve zaman): Çekirdek A'dan B'ye v hızla giderken, bir rakip tepki süresinden sonra koşup hatta yetişebilir mi? Alanların kaba toplamı değil, gerçek yetişme.
  function laneOpen(w, att, A, B, v) { v = v || 1; const L = Math.hypot(B.x - A.x, B.y - A.y) || 1, ux = (B.x - A.x) / L, uy = (B.y - A.y) / L; let open = 1; for (const o of w) { if (o.team === att || o.role === 'Bekçi') continue; const al = (o.x - A.x) * ux + (o.y - A.y) * uy; if (al <= 0 || al >= L + 1) continue; const pe = Math.abs(-(o.x - A.x) * uy + (o.y - A.y) * ux), tb = al / v, R = 1.6 + Math.max(0, tb - (14 - (o.a.okuma ?? 10) * .5)) * (.17 + (o.a.hiz ?? 10) * .006) * 1.25; open *= 1 - 1 / (1 + Math.exp((pe - R) / .5)); } return open; }
  // Takım değeri: Çekirdek E'de. Seçenekler: taşıyıcı tutar ya da önü açıksa sürer; ya da bir arkadaşa verir.
  // Her seçeneğin değeri taşıyıcının kullandığı aynı değer (V: buradan topla oynamanın değeri, gönderme ihtimali dahil) × hattın açık olma ihtimali (gerçek yetişme).
  function teamBest(w2, att, E, mates, ch) {
    const D = Dd(), Wt = S.W, gx = oppX(att), gd = Math.hypot(gx - E.x, 25 - E.y) || 1, st = Math.min(6, Math.max(0, gd - 6)), F = { x: E.x + (gx - E.x) / gd * st, y: E.y + (25 - E.y) / gd * st };
    let best = D.V(w2, att, E.x, E.y, ch); if (st > 0) best = Math.max(best, laneOpen(w2, att, E, F, .3) * D.V(w2, att, F.x, F.y, ch) * Wt.tvDrive);
    for (const q of mates) best = Math.max(best, laneOpen(w2, att, E, q) * D.V(w2, att, q.x, q.y, D.recvCh(w2, att, q.x, q.y, ch)) * Wt.tvPass);
    return best;
  }
  // Savunmacının cevap seçenekleri: taşıyıcının önü ile bir hücumcunun önü arasındaki hat üzerinde (ikisini birden kapatmaya çalışan ara noktalar dahil), yetişebildiği kadar
  function coverSpots(o, att, E, targets, R) { const out = [{ x: o.x, y: o.y }], A = gside(E, 1 - att, 2.2); for (const P of targets) { const B = gside(P, 1 - att, 2.2); for (const t of [0, .5, 1]) { const g = { x: A.x + (B.x - A.x) * t, y: A.y + (B.y - A.y) * t }, d = Math.hypot(g.x - o.x, g.y - o.y); out.push(d <= R ? g : { x: o.x + (g.x - o.x) / d * R, y: o.y + (g.y - o.y) / d * R }); } } return out; }
  // Hücumcu için: burada durursam, savunmacılar (taşıyıcıya en yakın ve bana en yakın) en iyi cevaplarını verdiğinde takımın en iyisi ne kalır?
  // Dar durursam tek bir savunmacı ara bir noktadan ikimizi birden kapatır; biraz açılırsam kapatamaz, birini seçmek zorunda kalır. Bunu kural değil bu hesap söyler.
  // Okuma, savunmanın cevabını ne kadar hesaba kattığını belirler.
  function teamV(m, sn, att, E, c, mates, okP) {
    const ch = m.ch || .3, all = mates.concat([c]), w = cl((okP - 6) / 10, 0, 1); if (w <= 0) return teamBest(sn, att, E, all, ch);
    const opp = sn.filter(o => o.team !== att && o.role !== 'Bekçi'), byE = opp.slice().sort((u, v) => Math.hypot(u.x - E.x, u.y - E.y) - Math.hypot(v.x - E.x, v.y - E.y)), byC = opp.slice().sort((u, v) => Math.hypot(u.x - c.x, u.y - c.y) - Math.hypot(v.x - c.x, v.y - c.y));
    const rs = []; if (byE[0]) rs.push(byE[0]); if (byC[0] && byC[0] !== byE[0]) rs.push(byC[0]);
    const choices = rs.map(o => coverSpots(o, att, E, [c], S.W.dlReach).map(g => ({ o, ...g })));
    const combos = choices.length === 1 ? choices[0].map(k => [k]) : choices.length ? choices[0].flatMap(k => choices[1].map(j => [k, j])) : [[]];
    // savunmacıları geçici olarak cevap noktalarına koyar, tartar, geri alır (kopya üretmeden)
    const keep = rs.map(o => [o.x, o.y]); let worst = 1e9; for (const cmb of combos) { for (const k of cmb) { k.o.x = k.x; k.o.y = k.y; } worst = Math.min(worst, teamBest(sn, att, E, all, ch)); } rs.forEach((o, i) => { o.x = keep[i][0]; o.y = keep[i][1]; });
    // Ben buraya varınca Çekirdeği alırsam açılacak boşluk (aynı "açtığı" ölçü; savunmanın cevabından önce, dünya değiştirilmeden bir kez hesaplanır)
    const D2 = Dd(), opened = D2.D.useSpace && okP > 0 ? laneOpen(sn, att, E, c) * Math.max(D2.space(sn, att, c.x, c.y), D2.front(sn, att, c.x, c.y)) * S.W.tvPass : 0;
    return Math.max((1 - w) * teamBest(sn, att, E, all, ch) + w * worst, opened);
  }
  // Savunmacı için aynı hesap, ters yönden: taşıyıcıya en yakın savunmacı, hücumun en iyi seçeneğini en aza indiren noktayı seçer (taşıyıcının önü ile pas hattı arasında).
  // Okuma düşükse sadece taşıyıcının önüne geçer; yüksekse pas seçeneğini de hesaba katar.
  function coverBoth(m, d, att, E, h) {
    const okD = ok(d), w = cl((okD - 6) / 10, 0, 1); if (w <= 0) return null;
    const sn = seen(m, d), atk = m.ps.filter(q => q.team === att && q !== h && q.role !== 'Bekçi' && Math.hypot((q.tx ?? q.x) - E.x, (q.ty ?? q.y) - E.y) < 22).map(q => ({ x: q.x, y: q.y }));
    if (!atk.length) return null;
    const me = sn.find(o => o.id === d.id) || d, spots = coverSpots(me, att, E, atk, S.W.defReach), ch = m.ch || .3;
    let best = null, bv = 1e9; for (const g of spots) { const w2 = sn.map(o => o.id === d.id ? { ...o, x: g.x, y: g.y } : o); const v = teamBest(w2, att, E, atk, ch) + Math.hypot(g.x - me.x, g.y - me.y) * .004; if (v < bv) { bv = v; best = g; } }
    const g0 = gside(E, 1 - att, 2.2); return { x: g0.x + (best.x - g0.x) * w, y: g0.y + (best.y - g0.y) * w };
  }
  function metrics(m) { const bx = m.holder ? m.holder.x : m.ball.x, by = m.holder ? m.holder.y : m.ball.y; let n = 0; for (const p of m.ps) if (p !== m.holder && p.role !== 'Bekçi' && !p.press && Math.hypot(p.x - bx, p.y - by) < 8) n++; return { swarm: n }; }
  window.AlanShape = { S, position, metrics, seen, flightRead, bekci, alici };
})();
