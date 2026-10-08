(function () {
  // Katman 3 sahneleri: sabit durum + beklenen karar. Takım 0 sağa (Kuyu x=100) hücum eder.
  const A = (o) => ({ okuma: 10, aktarim: 10, tutus: 10, kesme: 10, yogunluk: 10, hiz: 10, cesaret: 10, surme: 10, ...(o || {}) });
  const P = (team, name, x, y, role, a) => ({ id: name, team, name, x, y, vx: 0, vy: 0, role: role || 'Orta', a: A(a), R: role === 'Bekçi' ? 7 : 8, D: role === 'Bekçi' ? 1.2 : 1 });
  const bek = () => P(1, 'Bekçi', 97, 25, 'Bekçi');
  const S = [
    { id: 's1', title: 'Baskı altında, boş arkadaş', why: 'Taşıyıcının dibinde rakip var; ileride hattı açık bir arkadaş. Basit pas vermeli.', ch: .3,
      expect: ['pas', 'önüne'], ps: () => [P(0, 'Taşıyıcı', 45, 25), P(0, 'Arkadaş', 60, 14), P(1, 'Presçi', 43.4, 26), P(1, 'Uzak', 70, 40), bek()] },
    { id: 's2', title: 'Baskı yok, arkadaşlar markajlı', why: 'Yakında rakip yok, bütün pas hatları kapalı. Tutup şarj toplamalı ya da sürmeli; pas atmamalı.', ch: .3,
      expect: ['tut', 'sür'], ps: () => [P(0, 'Taşıyıcı', 35, 25), P(0, 'Arkadaş 1', 58, 12), P(0, 'Arkadaş 2', 58, 38), P(1, 'Markaj 1', 59.5, 13), P(1, 'Markaj 2', 59.5, 37), P(1, 'Orta', 62, 25), bek()] },
    { id: 's3', title: 'Kuyu önünde, şarjlı', why: 'Kuyu\'ya 16 birim, şarj %90, önünde tek savunmacı yan tarafta. Göndermeli.', ch: .9,
      expect: ['gönder'], ps: () => [P(0, 'Taşıyıcı', 84, 25), P(0, 'Arkadaş', 70, 40), P(1, 'Savunmacı', 89, 31), bek()] },
    { id: 's4', title: 'Doğrudan hat kapalı', why: 'Arkadaş aynı hatta ileride, arada savunmacı. Kenardan sektirmeli ya da başka çözüm bulmalı; doğrudan pas atmamalı.', ch: .4,
      expect: ['kenardan', 'tut', 'sür'], forbid: ['pas'], ps: () => [P(0, 'Taşıyıcı', 45, 42), P(0, 'Arkadaş', 72, 42), P(1, 'Engel', 58.5, 42), P(1, 'Uzak', 60, 15), bek()] },
    { id: 's5', title: 'Doğrudan hat açık', why: 'Aynı durum, ama arada savunmacı yok. Kenardan sektirmeye gerek yok; doğrudan pas.', ch: .4,
      expect: ['pas', 'önüne'], forbid: ['kenardan'], ps: () => [P(0, 'Taşıyıcı', 45, 42), P(0, 'Arkadaş', 66, 42), P(1, 'Uzak', 60, 15), bek()] },
    { id: 's6', title: 'Uzakta, şarjsız', why: 'Kuyu\'ya 27 birim, şarj %20, önü kalabalık. Göndermemeli.', ch: .2,
      expect: ['tut', 'sür', 'pas', 'önüne', 'kenardan'], forbid: ['gönder'], ps: () => [P(0, 'Taşıyıcı', 73, 25), P(0, 'Arkadaş', 70, 8), P(1, 'Savunmacı 1', 84, 22), P(1, 'Savunmacı 2', 84, 29), bek()] },
    { id: 's7', title: 'Hat kapalı, sürme de kapalı', why: 'Önü bir savunmacıyla kapalı, arkadaşına giden hattın ortasında bir engel var; kenar ise engelden uzak. Tek temiz yol kenardan sektirmek. Okuma 10\'un altındaki oyuncu bu çözümü görmez; baskı altında başka bir şey dener (çoğu zaman kötü sonuçlanır).', ch: .4,
      expect: ['kenardan'], needOk: 10, ps: () => [P(0, 'Taşıyıcı', 45, 30), P(0, 'Arkadaş', 75, 30), P(1, 'Engel', 60, 30), P(1, 'Önde', 49.5, 31), P(1, 'İçeride', 48.5, 25), P(1, 'Uzak', 86, 14), bek()] },
    { id: 'k1', group: 'Kuyu önü', title: 'Geri pas', why: 'Taşıyıcı çizgiye yakın, dar açıda. Savunma Kuyu önüne yığılmış; bir arkadaş geride, ortada boş. Göndermemeli, geriye boştakine vermeli.', ch: .5,
      expect: ['pas', 'önüne'], forbid: ['gönder'], ps: () => [P(0, 'Taşıyıcı', 92, 9), P(0, 'Boştaki', 79, 25), P(0, 'Markajlı', 90, 36), P(1, 'Yığın 1', 93, 21), P(1, 'Yığın 2', 92, 27), P(1, 'Yığın 3', 89, 24), P(1, 'Markaj', 91, 34), bek()] },
    { id: 'k2', group: 'Kuyu önü', title: 'İkiye bir', why: 'Kuyu önünde tek savunmacıya karşı iki hücumcu. Savunmacı taşıyıcının hattında: arkadaşa vermeli. Ya da hat açıksa göndermeli.', ch: .5,
      expect: ['pas', 'önüne', 'gönder'], forbid: ['tut'], ps: () => [P(0, 'Taşıyıcı', 80, 20), P(0, 'Arkadaş', 82, 33), P(1, 'Tek savunmacı', 86, 22.5), bek()] },
    { id: 'k3', group: 'Kuyu önü', title: 'Bekçiyle bire bir', why: 'Arada savunmacı yok, Kuyu\'ya 14 birim, şarj orta. Göndermeli; Bekçi\'nin uzanamayacağı köşeye plase ya da güçlü.', ch: .55,
      expect: ['gönder'], ps: () => [P(0, 'Taşıyıcı', 86, 25), P(0, 'Arkadaş', 70, 40), P(1, 'Geride', 72, 20), bek()] },
    { id: 'k4', group: 'Kuyu önü', title: 'Karşı taraf', why: 'Blok taşıyıcının tarafına kaymış; karşı tarafta bir arkadaş açık açıda. Ona vermeli, kalabalığa göndermemeli.', ch: .5,
      expect: ['pas', 'önüne', 'kenardan'], forbid: ['gönder'], ps: () => [P(0, 'Taşıyıcı', 82, 12), P(0, 'Karşıdaki', 85, 37), P(1, 'Kayan 1', 87, 14), P(1, 'Kayan 2', 90, 19), P(1, 'Kayan 3', 85, 19), bek()] },
    { id: 'k5', group: 'Kuyu önü', title: 'Kalabalık, açık yok', why: 'Önü 5 savunmacıyla dolu, arkadaşlar markajlı. Kalabalığa göndermemeli; tutup, geri verip yeniden kurmalı.', ch: .4,
      expect: ['tut', 'sür', 'pas', 'önüne', 'kenardan'], forbid: ['gönder'], ps: () => [P(0, 'Taşıyıcı', 74, 25), P(0, 'Arkadaş 1', 86, 15), P(0, 'Arkadaş 2', 86, 35), P(0, 'Geride', 60, 30), P(1, 'Blok 1', 84, 22), P(1, 'Blok 2', 84, 28), P(1, 'Blok 3', 90, 25), P(1, 'Markaj 1', 87.5, 16), P(1, 'Markaj 2', 87.5, 34), bek()] }
  ];
  // Bekçi maçtaki gibi yerleşir: Kuyu'dan taşıyıcıya doğru, Çekirdek–Kuyu hattında
  function place(ps) { const h = ps[0], gx = h.team === 0 ? 100 : 0; for (const p of ps) if (p.role === 'Bekçi' && p.team !== h.team) { const dx = h.x - gx, dy = h.y - 25, L = Math.hypot(dx, dy) || 1, r = Math.min(6, L * .22); p.x = gx + dx / L * r; p.y = 25 + dy / L * r; } return ps; }

  // Pozisyon oynatma (her sahne oyuncusu derinlik ve kanat olarak en yakın görevi alır): sahnedeki oyuncular gerçek maç motoruna konur, hücum bitene kadar oynanır (sayı, kayıp ya da 6 sn)
  function play(sc, seed, okuma, maxT) {
    const M = window.AlanMatch, m = M.createMatch(seed, {}), sp = place(sc.ps()), used = new Set(), out = [];
    for (const s of sp) { const pool = m.ps.filter(p => p.team === s.team && !used.has(p)); const p = s.role === 'Bekçi' ? pool.find(p => p.role === 'Bekçi') : pool.filter(p => p.role !== 'Bekçi').sort((u, v) => (Math.abs(u.slot.d - s.x / 100) + Math.abs(u.slot.l - (s.y - 25) / 22)) - (Math.abs(v.slot.d - s.x / 100) + Math.abs(v.slot.l - (s.y - 25) / 22)))[0]; used.add(p); p.x = p.tx = s.x; p.y = p.ty = s.y; p.vx = p.vy = 0; p.name = s.name; Object.assign(p.a, s.a); if (okuma != null && s === sp[0]) p.a.okuma = okuma; out.push(p); }
    m.ps = out; m.holder = out[0]; m.ball = null; m.ch = sc.ch; m.decT = 1; m.fl = null;
    let res = 'süre'; const frames = [];
    for (let t = 0; t < (maxT || 360); t++) { try { M.step(m); } catch (e) {} frames.push({ ps: m.ps.map(p => [p.x, p.y]), tg: m.ps.map(p => [p.tx, p.ty]), job: m.ps.map(p => p.job || ''), b: m.holder ? [m.holder.x, m.holder.y] : m.ball ? [m.ball.x, m.ball.y] : null, air: !m.holder && m.ball ? m.ball.air || 0 : 0 });
      if (m.score[0] > 0) { res = 'SAYI'; break; } if (m.score[1] > 0) { res = 'kendi Kuyu'; break; }
      if (m.holder && m.holder.team === 1) { res = m.holder.role === 'Bekçi' ? 'Bekçi aldı' : 'kaybetti'; break; } }
    return { res, shots: m.st.shot[0], passes: m.st.pass[0], events: m.events.slice().reverse(), frames, names: m.ps.map(p => p.name), teams: m.ps.map(p => p.team) };
  }
  function run(sc, N, okuma) {
    const D = window.AlanDecide, rnd = (s => () => { s = (s * 1664525 + 1013904223) >>> 0; return s / 4294967296; })(7), out = {};
    let pred = 0, real = 0, n = 0, last = null;
    for (let i = 0; i < N; i++) {
      const ps = place(sc.ps()); const h = ps[0]; if (okuma != null) h.a.okuma = okuma;
      const r = D.decide(h, ps, sc.ch, rnd, null), b = r.best; out[b.kind] = (out[b.kind] || 0) + 1; last = r;
      if (b.launch) { const re = D.realize(h, b, ps, sc.ch, rnd); const good = b.kind === 'gönder' ? re.res === 'kuyu' : re.res === 'mate'; pred += b.kind === 'gönder' ? b.det.Pk : b.det.P; real += good ? 1 : 0; n++; }
    }
    const top = Object.entries(out).sort((a, b) => b[1] - a[1])[0][0];
    const lowOk = sc.needOk && (okuma ?? 10) < sc.needOk, okExp = lowOk ? top !== 'kenardan' : sc.expect.includes(top) && !(sc.forbid || []).some(k => (out[k] || 0) > N * .2);
    return { dist: out, top, pass: okExp, pred: n ? pred / n : null, real: n ? real / n : null, n, last };
  }
  window.AlanScenes = { S, run, P, A, place, play };
})();
