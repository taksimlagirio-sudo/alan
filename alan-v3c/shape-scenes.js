(function () {
  // Katman 4 sahneleri: tam 7'ye 7 dizilim, oyuncular hedeflerine yürür (taşıyıcı sabit), sonra ölçülür.
  // Slot sırası: Bekçi, Bek, Bek, Orta, Orta, Kanat, Uç. Mavi (0) sağa hücum eder.
  const M = () => window.AlanMatch, SH = () => window.AlanShape, K = () => window.AlanCore;
  const spd = p => .17 + (p.a.hiz ?? 10) * .006;
  function build(sc, tacOv) {
    const m = M().createMatch(sc.seed || 11, { tac: [{ ...(sc.tac && sc.tac[0]) , ...(tacOv && tacOv[0]) }, { ...(sc.tac && sc.tac[1]), ...(tacOv && tacOv[1]) }] });
    for (const t of [0, 1]) sc.pos[t].forEach((c, i) => { const p = m.ps.find(q => q.team === t && q.i === i); p.x = c[0]; p.y = c[1]; p.vx = p.vy = 0; p.tx = p.x; p.ty = p.y; p.otx = null; });
    m.sh = { att: sc.prevAtt ?? null, gain: 0 }; m.tick = 100;
    if (sc.holder) { m.holder = m.ps.find(q => q.team === sc.holder[0] && q.i === sc.holder[1]); m.ball = null; }
    if (sc.flight) { const f = sc.flight, from = m.ps.find(q => q.team === f.team && q.i === f.from), q = f.to != null ? m.ps.find(r => r.team === f.team && r.i === f.to) : null; m.holder = null;
      const tgt = f.at || { x: q.x, y: q.y }, lf = window.AlanDecide.launchFor(from, tgt, f.arrive ?? .3, m.ps, .3); m.ball = K().makeBall({ x: from.x, y: from.y, vx: lf.vx, vy: lf.vy, team: f.team, ch: .3, from, recv: q }); m.fl = { t0: m.tick, kind: f.at ? 'önüne' : 'pas', q, to: tgt }; }
    m.start = m.ps.map(p => ({ x: p.x, y: p.y }));
    return m;
  }
  function settle(m, n) {
    const tr = m.ps.map(p => [{ x: p.x, y: p.y }]);
    for (let k = 0; k < n; k++) {
      m.tick++; SH().position(m);
      if (m.ball && !m.ball.done) K().stepBall(m.ball, m.ps);
      m.ps.forEach((p, i) => { if (p === m.holder) return; const dx = p.tx - p.x, dy = p.ty - p.y, L = Math.hypot(dx, dy), v = Math.min(spd(p) * (p.sprint ? 1.25 : 1), L); if (L > .05) { p.vx = dx / L * v; p.vy = dy / L * v; p.x += p.vx; p.y += p.vy; } else p.vx = p.vy = 0; if (k % 4 === 3) tr[i].push({ x: p.x, y: p.y }); });
    }
    m.trail = tr; return m;
  }
  const P = (m, t, i) => m.ps.find(q => q.team === t && q.i === i), field = (m, t) => m.ps.filter(p => p.team === t && p.role !== 'Bekçi');
  function spaceVal(m, t) { // hücumcuların bulduğu en iyi "açık + hattı görünür + tehlikeli" nokta
    const D = window.AlanDecide, E = m.holder ? m.holder : m.E; let best = 0, who = null;
    for (const p of field(m, t)) { if (p === m.holder) continue; const open = Math.exp(-D.oppAt(m.ps, t, p.x, p.y) * 2); let ln = 0; for (let i = 1; i <= 4; i++) ln += D.oppAt(m.ps, t, E.x + (p.x - E.x) * i / 5, E.y + (p.y - E.y) * i / 5); const v = open * Math.exp(-ln / 4 * 2.5) * D.threat(t, p.x, p.y); if (v > best) { best = v; who = p; } }
    return { v: best, who };
  }
  const BASE = {
    blue: [[4, 25], [28, 14], [28, 36], [45, 25], [48, 39], [60, 8], [64, 30]],
    orange: [[96, 25], [74, 16], [74, 34], [62, 20], [62, 32], [55, 10], [54, 38]]
  };
  const S = [
    { id: 'y1', title: 'Pres alan bırakır, hücum onu görür', why: 'Mavi taşıyıcı geride, dört hücumcu önde. Turuncu önce 1 kişiyle, sonra 3 kişiyle basıyor. Basanların bıraktığı bölgeyi kimse kendiliğinden doldurmamalı; Mavi hücumcular o alanı bulmalı. Her basan, kendi adamını ya da bölgesini bırakır. Ölçü: her Turuncu en fazla bir Mavi\'yi tutar (9 birim içinde); Çekirdeğin önünde kimsenin tutmadığı Mavi sayısı 3 kişilik preste daha fazla olmalı.',
      pos: [[[4, 25], [26, 20], [28, 38], [45, 22], [50, 38], [62, 8], [64, 30]], BASE.orange], holder: [0, 1], tac: [{}, { pres: 3, blok: 'Yüksek' }], n: 90,
      check(m) { const free = mm => { const A = field(mm, 0).filter(p => p !== mm.holder && p.x > mm.holder.x), D = field(mm, 1).filter(p => !p.press), pr = []; for (const a of A) for (const d of D) { const x = Math.hypot(a.x - d.x, a.y - d.y); if (x < 9) pr.push([x, a, d]); } pr.sort((u, v) => u[0] - v[0]); const ua = new Set(), ud = new Set(); for (const [, a, d] of pr) if (!ua.has(a) && !ud.has(d)) { ua.add(a); ud.add(d); } return A.filter(a => !ua.has(a)); };
        const a = free(m), m1 = settle(build(this, [{}, { pres: 1, blok: 'Yüksek' }]), this.n), b = free(m1); return { pass: a.length > b.length, info: `Çekirdeğin önünde kimsenin tutmadığı Mavi: 3 kişi preste ${a.length} (${a.map(p => p.name).join(', ') || '—'}) · 1 kişi preste ${b.length} (${b.map(p => p.name).join(', ') || '—'})` }; } },
    { id: 'y2', title: 'Sürü yok', why: 'Mavi taşıyıcı ortada, Turuncu 1 kişiyle basıyor. Çekirdeğin 8 birim çevresinde basan dışında en fazla 1 Turuncu, taşıyıcı dışında en fazla 1 Mavi olmalı.',
      pos: [BASE.blue, BASE.orange], holder: [0, 3], tac: [{}, { pres: 1 }], n: 90,
      check(m) { const h = m.holder, n = t => field(m, t).filter(p => p !== h && !p.press && Math.hypot(p.x - h.x, p.y - h.y) < 8).length; const a = n(0), b = n(1); return { pass: a <= 1 && b <= 1, info: `çevrede Mavi ${a}, Turuncu (basan hariç) ${b}` }; } },
    { id: 'y3', title: 'Kuyu tarafına geçme', why: 'Mavi taşıyıcı Turuncu Kuyu\'ya 20 birim. Turuncu\'nun önde bekleyen uçları hariç herkes Çekirdek ile Kuyu arasında (Kuyu tarafında) olmalı.',
      pos: [[[4, 25], [40, 14], [40, 36], [80, 18], [70, 30], [86, 8], [88, 30]], [[96, 25], [84, 16], [84, 34], [70, 20], [70, 32], [60, 12], [58, 36]]], holder: [0, 3], tac: [{}, { pres: 1 }], n: 100,
      check(m) { const h = m.holder, bad = field(m, 1).filter(p => p.slot.d < .7 && !p.press && p.x < h.x - 1.5); return { pass: bad.length === 0, info: bad.length ? `Çekirdeğin önünde kalan: ${bad.map(p => p.name).join(', ')}` : 'uçlar dışında herkes Kuyu tarafında' }; } },
    { id: 'y4', title: 'Önde bekleyen rakibe güvence', why: 'Turuncu taktikte 1 oyuncuyu önde bırakıyor; Turuncu Uç Mavi yarı sahada bekliyor. Risk orta: bir Mavi onun Kuyu tarafında durmalı. Risk yüksekken (sahnede ikinci ölçüm) Mavi bu güvenceyi bırakabilir.',
      pos: [BASE.blue, [[96, 25], [74, 16], [74, 34], [62, 20], [62, 32], [55, 10], [30, 25]]], holder: [0, 3], tac: [{ risk: .5 }, { pres: 0, onde: 1 }], n: 90,
      check(m) { const hq = P(m, 1, 6), g = field(m, 0).filter(p => p.job === 'güvence'); const m2 = settle(build(this, [{ risk: .9 }, { pres: 0, onde: 1 }]), this.n), hq2 = P(m2, 1, 6), g2 = field(m2, 0).filter(p => p.job === 'güvence');
        return { pass: g.length >= 1 && g2.length === 0, info: `risk orta: güvence ${g.map(p => p.name).join(', ') || 'yok'} · risk yüksek: ${g2.map(p => p.name).join(', ') || 'yok'}` }; } },
    { id: 'y5', title: 'Geri koşu', why: 'Turuncu hücumdaydı, Mavi Çekirdeği x=55\'te kazandı. Çekirdeğin önünde kalan Turunculer Okuma gecikmesinden sonra Kuyu tarafına koşmalı.',
      pos: [[[4, 25], [40, 14], [40, 36], [55, 25], [60, 40], [70, 8], [72, 30]], [[96, 25], [62, 16], [62, 34], [44, 20], [40, 32], [30, 12], [26, 36]]], holder: [0, 3], prevAtt: 1, tac: [{}, { pres: 1 }], n: 70,
      check(m) { const h = m.holder, ahead = field(m, 1).filter((p, i) => m.start[m.ps.indexOf(p)].x < h.x - 2 && !p.press), back = ahead.filter(p => p.x > m.start[m.ps.indexOf(p)].x + 6); return { pass: back.length >= ahead.length - 1, info: `önde kalan ${ahead.length}, geri koşan ${back.length}` }; } },
    { id: 'y6', title: 'Boştaki Çekirdek: kim gidecek?', why: 'Mavi, kanattaki arkadaşının önüne uzun aktarım yaptı. Alıcı koşar. Diğerleri "ben önce varırım" diye düşünmüyorsa gitmez. Her takımdan en fazla 2 kişi kovalamalı.',
      pos: [BASE.blue, BASE.orange], flight: { team: 0, from: 3, to: 5, at: { x: 74, y: 6 }, arrive: .08 }, n: 1,
      check(m) { const c = t => m.ps.filter(p => p.team === t && (p.job === 'kovala' || p.job === 'alıcı')); const a = c(0), b = c(1); return { pass: a.length <= 2 && b.length <= 2 && a.some(p => p.job === 'alıcı'), info: `Mavi: ${a.map(p => p.name + (p.job === 'alıcı' ? ' (alıcı)' : '')).join(', ') || '—'} · Turuncu: ${b.map(p => p.name).join(', ') || '—'}` }; } },
    { id: 'y7', title: 'Arkadaşın alacağı yeri öngörmek', why: 'Pas uçarken Mavi hücumcular, pası verene değil, alıcının Çekirdeği alacağı noktaya göre yerleşmeli; kimse pası verenin yanına yığılmamalı.',
      pos: [BASE.blue, BASE.orange], flight: { team: 0, from: 1, to: 6 }, n: 20,
      check(m) { const E = P(m, 0, 6), pas = P(m, 0, 1), rp = m.E, oth = field(m, 0).filter(p => p !== E && p !== pas), dE = oth.reduce((s, p) => s + Math.hypot(p.tx - rp.x, p.ty - rp.y), 0) / oth.length, dP = oth.reduce((s, p) => s + Math.hypot(p.tx - pas.x, p.ty - pas.y), 0) / oth.length, crowd = oth.filter(p => Math.hypot(p.tx - pas.x, p.ty - pas.y) < 7);
        return { pass: dE < dP && crowd.length === 0, info: `hedeflerin alıcının alacağı noktaya ort. uzaklığı ${dE.toFixed(1)}, pası verene ${dP.toFixed(1)} · pası verenin yanına yığılan: ${crowd.length}` }; } }
  ];
  function run(sc) { const m = settle(build(sc), sc.n); const r = sc.check(m); return { m, ...r }; }
  window.AlanShapeScenes = { S, run, build, settle };
})();
