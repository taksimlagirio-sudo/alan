(function () {
  let CUR = null;
  const OUT = .35, BUS = 1.2, ATTR_K = .5, ATTR_K_OK = .8, ATTR_K_HIZ = .7, MOUTH = 1.7, SHOT_S = 5, ATTN = .012, BLOCK_MAX = 40, PANT = 1.5, PLAG = 2, SHADOW_D = 8, BACKP = .6, TURN_D = 4, REST0 = .25, REST_K = .4, RUN_D = .006, TEMPO_RUN = .08, PR_S = 5, HURRY = .25, DIK_T = 150, DIK_D = 3, DIK_F = .5, DIK_B = .2, BRK = 0, GUARD_D = 6, COMPACT = .85, POSS_K = 0, COMBO = .9, RV0 = .25, RV1 = .35, PRED_N = 2, PRED_X = .25, PRED_Y = .5, PRED_W = .7, HV0 = .55, LOB_K = .6, CHG2 = 0.003, CH_MV = 0.4, CH_SP0 = 0.25, GLH = .35, OV_R = 12, OV_W = .5, GL_W = .4, FEINT_R = 7, FEINT_V = 0.9, FEINT_C = 0.04, FEINT_CD = 70, SH_PIERCE = .7, PL_S = .6, SHOT_DR = .45, PL_D = 1, PERC = .06, REB_V = .35, FG_K = .25, LEAD_SPD = 1.05, LOOSE_T = 110, TB_S = 6, SPACE_B = .1, SP_MARGIN = 8, CH_GIVEUP = 1.6, ANT_T = 20, ANT2 = .25, ANT3 = .45, SHAPE_LAG = .035, RAD0 = 11, RAD_K = .8, ANTIC = .7, REL_D = 3.5, REL_K = .5, SP_OPEN = .35, SP_GAIN = .04, PERC_R = .04, WALL_L = .12, BANK_ACC = .7, FRIC = .055, OPP_DRAG = .05, PIERCE_P = .6, OWN_FLOW = .35, DEAD_FRIC = .25, STOP_V = .03, REST = .7, CTRL_R = 1.4, CTRL0 = 1.02, CTRL_V = .32, CTRL_CH = .25, DEFL_A = 1.8, DEFL_K = .5, SH_V = 2.5, PL_V = 1.7, V_MAX = 1.9, VA_PASS = .5, RCV_B = .25, PASS_DR = .45, SIM_ERR = .04, HYST = .06, SP_W = .5, LN_W = .35, TH_W = .6, SPACE_D = 11, SPC_W = .5, LOCK_R = 16, ZONE_W = .55, REP_D = 6, LOCK0 = 15, LOCK1 = 60, PSPD = .15, CAGE1 = .5, CAGE = 1, CONTAIN = .62, REC_T = 150, SPRINT = 1.12, PRESS_R = 18, PRESS_COST = .0004, SUP = .15, HOLDV = .08, CARRY = .7, CARRY_L = .3, CARRY_MIN = .55, GECIS = .12, CHG = .006, DRAIN = .2, COVER = .45, KEEP_M = .05, DUEL = .3, BACK = .55, TURN = .08, LOB_MIN = .55, OPEN_T = 1.2, OPEN_N = 1.0, KOP_P = .1, KES_P = .7, SHOT_D = 34, KESR = 6, W = 100, H = 50, KR = 5, EV = 2400, EVRE_SN = 240, DELAY = 90;
  const A = '#1F3FD1', B = '#E0531F', P = '#F6F3EC', INK = '#16150F';
  const ROLES = {
    'Bekçi': { k: 'Bk', R: 1, D: 1 },
    'Çapa': { k: 'Ça', R: .6, D: 1.5 },
    'Gerer': { k: 'Ge', R: 1.6, D: .6 },
    'Mıknatıs': { k: 'Mı', R: .9, D: .8, pull: true },
    'Yankı': { k: 'Ya', R: 1, D: 1, echo: true },
    'Delici': { k: 'De', R: 1, D: 1, cone: true },
    'Gölge': { k: 'Gö', R: .9, D: 1, shadow: true }
  };
  const ORI = ['Dengeli', 'Taşı', 'Aktar', 'Sız', 'Destek', 'Önde bekle'];
  const ATTR = ['menzil', 'yogunluk', 'hiz', 'dayaniklilik', 'okuma', 'aktarim', 'tutus', 'kesme', 'cesaret', 'surme'];
  const DEFAULT_TAC = { blok: .5, hatlar: .5, genislik: .75, mesafe: .5, tasima: .5, tempo: .5, risk: .5, pres: .5, arkada: '1 kişi', savunma: 'Alan', presTetik: 'Her zaman', ritim: 'Dengeli', bosluk: 'Serbest' };
  const ENUMS = { arkada: ['Yok', '1 kişi', '2 kişi'], savunma: ['Alan', 'Adam adama', 'Ön alan', 'Kuyu önü'], presTetik: ['Her zaman', 'Kanatta', 'Kayıptan sonra', 'Kendi yarımızda'], ritim: ['Dengeli', 'Sabırlı', 'Dikine', 'Kontra', 'Kanat'], bosluk: ['Serbest', 'Rotasyon', 'Yükleme', 'Tuzak', 'Genişlik'] };
  const FORMATIONS = {
    '2-2-2': [[.25, .3], [.25, .7], [.55, .2], [.55, .8], [.85, .35], [.85, .65]],
    '3-2-1': [[.2, .2], [.2, .5], [.2, .8], [.5, .35], [.5, .65], [.85, .5]],
    '1-3-2': [[.2, .5], [.5, .15], [.5, .5], [.5, .85], [.85, .3], [.85, .7]],
    '2-1-3': [[.25, .35], [.25, .65], [.5, .5], [.85, .15], [.85, .5], [.85, .85]],
    'Elmas': [[.15, .5], [.4, .25], [.4, .75], [.65, .3], [.65, .7], [.92, .5]]
  };
  const ZONES = { 'Bekçi': [0, .2], 'Çapa': [.05, .6], 'Gerer': [.25, 1], 'Mıknatıs': [.3, 1], 'Yankı': [.15, 1], 'Delici': [.55, 1], 'Gölge': [.4, 1] };
  const inZone = p => { const z = ZONES[p.role]; return p.sx >= z[0] - .02 && p.sx <= z[1] + .02; };
  const okuma = p => p.a.okuma - (inZone(p) ? 0 : 4);
  function formationPatch(players, name) {
    const F = FORMATIONS[name]; if (!F) return {};
    const out = players.filter(p => p.role !== 'Bekçi').sort((a, b) => a.sx - b.sx || a.sy - b.sy);
    const slots = F.slice().sort((a, b) => a[0] - b[0] || a[1] - b[1]); const res = {};
    out.forEach((p, i) => { const sl = slots[i] || slots[slots.length - 1]; res[p.id] = { sx: sl[0], sy: sl[1] }; });
    return res;
  }
  const TEAMS = [
    { name: 'Liman Kolektifi', short: 'LİM', color: A, tacText: 'Yüksek blok · Geniş · Cesur aktarım', tac: { blok: .65, hatlar: .5, genislik: .9, mesafe: .5, tasima: .5, tempo: .5, risk: .6, pres: .5 }, players: [
      ['Ece Tan', 'Bekçi', 0, .5, [12, 15, 9, 14, 13, 12, 13, 16, 8]],
      ['Mert Ayaz', 'Çapa', .3, .5, [10, 17, 10, 15, 12, 13, 16, 15, 9]],
      ['Defne Kor', 'Gerer', .45, .12, [17, 9, 14, 13, 14, 14, 10, 10, 12]],
      ['Arda Sel', 'Mıknatıs', .55, .62, [13, 12, 12, 12, 16, 13, 12, 12, 13]],
      ['İpek Nur', 'Yankı', .5, .9, [12, 12, 14, 16, 15, 15, 12, 11, 12]],
      ['Rüzgâr Ak', 'Delici', .9, .35, [14, 13, 18, 11, 15, 12, 13, 8, 17]],
      ['Sarp Demir', 'Gölge', .8, .72, [12, 13, 15, 12, 17, 14, 14, 9, 14]]
    ] },
    { name: 'Volta 09', short: 'VLT', color: B, tacText: 'Derin blok · Dar · Temkinli aktarım', tac: { blok: .35, hatlar: .4, genislik: .6, mesafe: .4, tasima: .5, tempo: .5, risk: .3, pres: .6, savunma: 'Alan', presTetik: 'Kanatta', ritim: 'Kontra', bosluk: 'Genişlik' }, players: [
      ['Nil Vural', 'Bekçi', 0, .5, [13, 16, 9, 15, 12, 11, 14, 17, 7]],
      ['Kaan Oruç', 'Çapa', .25, .3, [9, 18, 10, 16, 11, 12, 17, 16, 8]],
      ['Lale Erim', 'Çapa', .25, .7, [10, 17, 11, 15, 12, 12, 16, 15, 9]],
      ['Bora Işık', 'Gerer', .5, .5, [16, 10, 13, 14, 13, 14, 11, 11, 11]],
      ['Selin Ova', 'Mıknatıs', .6, .15, [13, 12, 12, 13, 15, 13, 12, 12, 12]],
      ['Can Yel', 'Delici', .85, .35, [15, 12, 17, 12, 14, 12, 13, 9, 15]],
      ['Duru Taş', 'Delici', .85, .75, [14, 13, 16, 13, 13, 13, 12, 9, 16]]
    ] }
  ];
  function rng(seed) { let a = seed >>> 0; return () => { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
  const ownX = t => t === 0 ? 0 : W, oppX = t => t === 0 ? W : 0, dirOf = t => t === 0 ? 1 : -1;
  const cl = (v, a, b) => Math.max(a, Math.min(b, v));
  const d2 = (a, b) => (a.x - b.x) ** 2 + (a.y - b.y) ** 2;

  function prep(ps) {
    for (const p of ps) {
      const r = ROLES[p.role]; let Rm = r.R, Dm = r.D, cone = !!r.cone;
      if (r.echo) {
        let best = null, bd = 1e9;
        for (const q of ps) { if (q === p || q.team !== p.team || ROLES[q.role].echo) continue; const d = d2(p, q); if (d < bd) { bd = d; best = q; } }
        if (best) { const rb = ROLES[best.role]; Rm = rb.R * .85 + .15; Dm = rb.D * .8; cone = !!rb.cone; }
      }
      let R = (6.4 + p.a.menzil * .15) * Rm, D = (.74 + p.a.yogunluk * .01) * Math.sqrt(8.2 / (6.4 + p.a.menzil * .15)) * Dm * (.75 + .25 * (p.st ?? 1));
      D *= (p.outF ?? 1) * (p.bus ? BUS : 1);
      if (p.role === 'Bekçi') { const dk = Math.hypot(p.x - ownX(p.team), p.y - H / 2); D *= dk < 16 ? 1.15 : .7; }
      p._R = R; p._D = D; p._cone = cone; p._ex = p.x; p._ey = p.y;
      if (p.fx === undefined) { p.fx = dirOf(p.team); p.fy = 0; }
    }
    for (const m of ps) {
      if (!ROLES[m.role].pull) continue;
      for (const o of ps) { if (o.team === m.team) continue; const dx = m.x - o.x, dy = m.y - o.y, f = .35 * Math.exp(-(dx * dx + dy * dy) / 144); o._ex += dx * f; o._ey += dy * f; }
    }
  }
  function infl(p, x, y) {
    const dx = x - p._ex, dy = y - p._ey; let q;
    if (p._cone) { const al = dx * p.fx + dy * p.fy, pe = -dx * p.fy + dy * p.fx, a = al > 0 ? al / 1.9 : al / .7; q = a * a + (pe / .65) ** 2; }
    else { const al = dx * p.fx + dy * p.fy, pe = -dx * p.fy + dy * p.fx, a = al < 0 ? al / BACK : al; q = a * a + pe * pe; }
    return p._D * Math.exp(-q / (p._R * p._R));
  }
  function K(ps, x, y, view) {
    let s = 0;
    for (const p of ps) { if (view != null && p.team !== view && hid(p)) continue; s += (p.team === 0 ? 1 : -1) * infl(p, x, y); }
    return Math.tanh(s * 1.3);
  }
  function lineSafety(ps, a, b, view, sign) {
    let mn = 9;
    for (let i = 1; i <= 12; i++) { const t = i / 12, v = K(ps, a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, view) * sign; if (v < mn) mn = v; }
    return mn;
  }
  function clock(m) {
    const s = Math.floor(((m.tick % EV) / EV) * EVRE_SN);
    return `E${m.evre} ${String(Math.floor(s / 60)).padStart(2, '0')}:${String(s % 60).padStart(2, '0')}`;
  }
  function ev(m, type, team, text, x, y) {
    m.events.unshift({ time: clock(m), type, team, text, n: m.evN++ }); if (m.events.length > 80) m.events.pop();
    if (x != null) m.st.marks.push({ type, team, x, y });
  }
  function kickoff(m, t) {
    for (const p of m.ps) { p.x = p.team === 0 ? 4 + p.sx * 40 : 96 - p.sx * 40; p.y = 4 + p.sy * 42; p.tx = p.x; p.ty = p.y; p.fx = dirOf(p.team); p.fy = 0; }
    const cands = m.ps.filter(p => p.team === t && p.role !== 'Bekçi');
    const h = cands.reduce((a, b) => Math.abs(b.sx - .55) < Math.abs(a.sx - .55) ? b : a);
    h.x = 50; h.y = 25;
    m.core = { x: 50, y: 25, holder: h, flight: null, holdT: 0 };
  }
  // opts: { teams, ai: [bool,bool], attrDelta: [n,n] }
  function createMatch(seed, opts) {
    opts = opts || {};
    const teams = opts.teams || TEAMS;
    const r = rng(seed || 1), ps = []; let id = 0;
    teams.forEach((T, t) => T.players.forEach(([name, role, sx, sy, at, ori]) => {
      const a = {}, p_raw = {}, dl = (opts.attrDelta || [0, 0])[t];
      ATTR.forEach((k, i) => { const raw = cl((at[i] ?? Math.round((at[6] + at[2]) / 2)) + dl, 1, 20); a[k] = 12 + (raw - 12) * (k === 'hiz' ? ATTR_K_HIZ : k === 'okuma' ? ATTR_K_OK : ATTR_K); p_raw[k] = raw; });
      ps.push({ id: id++, team: t, name, role, sx, sy, ori: ori || 'Dengeli', a, raw: p_raw, st: 1, outF: 1, x: 0, y: 0, tx: 0, ty: 0 });
    }));
    const m = { ps, r, seed, tick: 0, evre: 1, score: [0, 0], events: [], evN: 0, over: false, first: r() < .5 ? 0 : 1,
      teams, tac: teams.map(T => Object.assign({}, DEFAULT_TAC, T.tac)), ai: opts.ai || [false, true], pending: [],
      st: { ctrl: [0, 0], ctrlN: 0, now: [0, 0], pass: [0, 0], passOk: [0, 0], passLen: [0, 0], kesme: [0, 0], kopma: [0, 0], shot: [0, 0], lob: [0, 0], lobOk: [0, 0], beat: [0, 0], lead: [0, 0], gecis: [0, 0], intent: [{}, {}], miss: [0, 0], devSum: [0, 0], devN: [0, 0], orta: [0, 0], ortaOk: [0, 0], duelLost: [0, 0], touchLost: [0, 0],
        etkiNow: ps.map(() => 0), heat: new Float32Array(300), heatN: 0, marks: [], blockX: [0, 0], blockN: [0, 0], lost: [[0, 0, 0], [0, 0, 0]] } };
    kickoff(m, m.first); ev(m, 'evre', null, `Maç başladı · ${teams[m.first].name} başlıyor`);
    prep(ps); stats(m);
    return m;
  }
  // Manager: queue a tactic change; it reaches the arena after DELAY ticks
  function order(m, t, patch, label) {
    m.pending.push({ t, patch, at: m.tick + DELAY, label });
  }
  function applyPending(m) {
    if (!m.pending.length) return;
    const keep = [];
    for (const o of m.pending) {
      if (o.at > m.tick) { keep.push(o); continue; }
      const pa = o.patch;
      if (pa.tac) Object.assign(m.tac[o.t], pa.tac);
      if (pa.players) for (const [id, pp] of Object.entries(pa.players)) Object.assign(m.ps[id], pp);
      ev(m, 'talimat', o.t, `Talimat sahada · ${o.label || 'taktik güncellendi'}`);
    }
    m.pending = keep;
  }
  function aiAdapt(m, t) {
    const tac = m.tac[t], diff = m.score[t] - m.score[1 - t];
    if (diff < 0) { tac.blok = cl(tac.blok + .2, 0, 1); tac.tempo = cl(tac.tempo + .2, 0, 1); tac.savunma = 'Ön alan'; tac.ritim = 'Dikine'; ev(m, 'talimat', t, `${m.teams[t].name} geride: ön alan baskısına ve dikine oyuna geçti`); }
    else if (diff > 0) { tac.blok = cl(tac.blok - .15, 0, 1); tac.hatlar = cl(tac.hatlar - .15, 0, 1); tac.savunma = 'Kuyu önü'; tac.ritim = 'Kontra'; ev(m, 'talimat', t, `${m.teams[t].name} önde: Kuyu önüne çekildi, kontraya oynuyor`); }
  }
  function carrierAim(m, p) {
    if (p.feintGo > m.tick && p.feintBy) { const gx = oppX(p.team), sd = p.feintBy.y > p.y ? -1 : 1, vx = gx - p.x, vy = H / 2 - p.y, dn = Math.hypot(vx, vy) || 1; p.tx = p.x + vx / dn * 8 - vy / dn * 4 * sd; p.ty = cl(p.y + vy / dn * 8 + vx / dn * 4 * sd, 2, H - 2); p.sprint = true; return; }
    const dir = dirOf(p.team), wing = m.amode === 'Kanat' && Math.abs(p.x - oppX(p.team)) > 30, ty = wing ? (p.y < H / 2 ? 6 : H - 6) : H / 2, base = Math.atan2(ty - p.y, oppX(p.team) - p.x); let best = -9, ba = base;
    for (let i = -3; i <= 3; i++) {
      const a = base + i * .45, x = cl(p.x + Math.cos(a) * 5, 1, W - 1), y = cl(p.y + Math.sin(a) * 5, 1, H - 1);
      const v = K(m.ps, x, y, p.team) * dir + Math.cos(i * .45) * .5; if (v > best) { best = v; ba = a; }
    }
    p.tx = p.x + Math.cos(ba) * 5; p.ty = p.y + Math.sin(ba) * 5;
  }
  function oppInf(ps, q, view) { let s = 0; for (const p of ps) { if (p.team === q.team) continue; if (view !== -1 && hid(p)) continue; s += infl(p, q.x, q.y); } return s; }
  function hurry(ps, h) { if (!h.a) return 0; const pr = cl((oppAt(ps, h.team, h.x, h.y, null) - .3) / .7, 0, 1); return HURRY * pr * cl(1.3 - h.a.tutus / 20 - (h.a.cesaret - 10) * .02, .3, 1.2); }
  function lineCharge(ps, a, b, view, sign, akt, chg, dm) {
    let e = .27 + akt * .045 + .4 * (chg ?? .5) - hurry(ps, a); dm = dm ?? 1; const L = Math.hypot(b.x - a.x, b.y - a.y), seg = L / 12;
    for (let i = 1; i <= 12; i++) { const t = i / 12, k = K(ps, a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, view) * sign; if (k < 0) e -= -k * seg * DRAIN * 1.35 * dm; e -= seg * ATTN; }
    return e;
  }
  function passReq(h, tac) { return .15 - tac.risk * .45 - (h.a.cesaret - 10) * .01; }
  function threat(t, x, y) { const d = Math.hypot(x - oppX(t), y - H / 2); return Math.exp(-d / 22) * (1 - .35 * Math.abs(y - H / 2) / (H / 2)); }
  function oppAt(ps, t, x, y, view) { let s = 0; for (const p of ps) { if (p.team === t) continue; if (view != null && hid(p)) continue; s += infl(p, x, y); } return s; }
  function hid(p) { return ROLES[p.role].shadow && !p.press && (!CUR || Math.hypot(p.x - CUR.x, p.y - CUR.y) > SHADOW_D); }
  function holdVal(ps, t, x, y) { const gx = oppX(t); let l = 0; for (let k = 1; k <= 2; k++) { const tt = k / 3, lx = x + (gx - x) * tt, ly = y + (H / 2 - y) * tt; for (const q of ps) if (q.team !== t && q.role !== 'Bekçi') l += infl(q, lx, ly); } return threat(t, x, y) * (HV0 + (1 - HV0) * Math.exp(-oppAt(ps, t, x, y, t) * 2)) * (1 - GLH + GLH * Math.exp(-l)); }
  function cageMul(n) { return n <= 1 ? CAGE1 : 1 + CAGE * (n - 1); }
  function lossCost(t, x, y) { return threat(1 - t, x, y); }
  // Every option = P(success) × value after − P(fail) × cost of losing it there
  // İsabet: bırakma anındaki sakinlik. Baskı ve aşırı şarj saptırır; Aktarım ve Okuma toparlar.
  function spread(m, h, d) {
    const pr = cl((oppAt(m.ps, h.team, h.x, h.y, null) - .3) / .7, 0, 1), ch = m.core.charge ?? .5, thr = .6 + (h.a.tutus - 10) * .02;
    return .6 * (d / 20) * (1 + PR_S * pr) * (1 - h.a.aktarim / 24) * (1 - (okuma(h) - 10) * .02) * (1 + Math.max(0, ch - thr) * 1.5);
  }

  function physStep(m, att) {
    const c = m.core, f = c.flight, ps = m.ps, dir = dirOf(att), ch = f.ch ?? .5;
    if (m.tick - f.t0 > 600) { const nr = ps.reduce((a, b) => (d2(b, c) < d2(a, c) ? b : a)); c.flight = null; c.holder = nr; c.holdT = 0; return; }
    let vx = f.vx, vy = f.vy, sp = Math.hypot(vx, vy), fr = FRIC;
    if (!f.dead) {
      const kt = K(ps, c.x, c.y, null) * dir;
      if (kt < 0) { fr += -kt * OPP_DRAG * (1 - PIERCE_P * ch); f.e -= -kt * sp * DRAIN * (f.kuyu ? SHOT_DR * (f.pl ? PL_D : 1 - SH_PIERCE * ch) : PASS_DR); }
      else fr *= 1 - OWN_FLOW * Math.min(1, kt);
      f.e -= sp * ATTN;
      if (f.e <= 0) { f.dead = true; f.looseT = m.tick; m.st.dead = m.st.dead || [0, 0]; m.st.dead[att]++; ev(m, 'kesme', 1 - att, 'Çekirdek söndü, boşta kaldı', c.x, c.y); }
    } else fr = DEAD_FRIC;
    vx *= 1 - fr; vy *= 1 - fr; sp = Math.hypot(vx, vy);
    if (sp < STOP_V) { vx = 0; vy = 0; sp = 0; if (!f.looseT) f.looseT = m.tick; }
    const ox = c.x, oy = c.y; let nx = ox + vx, ny = oy + vy;
    for (const ex of [0, W]) {
      if ((ex === 0 && nx <= .5) || (ex === W && nx >= W - .5)) {
        const tt = Math.abs(vx) > 1e-6 ? ((ex === 0 ? .5 : W - .5) - ox) / vx : 0, yy = oy + vy * cl(tt, 0, 1);
        if (Math.abs(yy - H / 2) <= MOUTH && !f.dead) {
          const sc = ex === oppX(att) ? att : 1 - att; m.score[sc]++;
          ev(m, 'sayi', sc, sc === att ? `SAYI · ${f.from.name} Çekirdeği Kuyu'ya ${f.kuyu ? (f.pl ? 'plase ' : 'güçlü ') : ''}gönderdi · ${m.score[0]}–${m.score[1]}` : `Kendi Kuyu'suna · ${m.score[0]}–${m.score[1]}`, f.from.x, f.from.y);
          kickoff(m, 1 - sc); return;
        }
        if (f.kuyu && !f.missed && !f.defl) { f.missed = true; m.st.miss[att]++; ev(m, 'kesme', 1 - att, `Iska · ${f.from.name} Kuyu'yu tutturamadı, Çekirdek kenardan döndü`, f.from.x, f.from.y); }
        nx = ex === 0 ? 1 - nx : 2 * W - 1 - nx; vx = -vx * REST; if (!f.dead) f.e -= WALL_L; f.walls = (f.walls || 0) + 1;
      }
    }
    if (ny < .5 || ny > H - .5) { ny = ny < .5 ? 1 - ny : 2 * H - 1 - ny; vy = -vy * REST; if (!f.dead) f.e -= WALL_L; f.walls = (f.walls || 0) + 1;
      if (f.via && !f.banked) { f.banked = true; ev(m, 'pass', att, `${f.from.name} kenardan sektirdi → ${f.recv ? f.recv.name : ''}`, nx, ny); } }
    c.x = nx; c.y = ny; f.vx = vx; f.vy = vy; sp = Math.hypot(vx, vy);
    const sx = nx - ox, sy = ny - oy, sl = sx * sx + sy * sy;
    let best = null, bd = 1e9;
    for (const p of ps) {
      if (p.stun > m.tick || (p.ctrlCD || 0) > m.tick) continue;
      if (p === f.from && m.tick - f.t0 < 10 && !f.defl) continue;
      if (p.team === att && p !== f.recv && !f.defl && !f.dead && sp > .4) continue;
      const tt = sl > 0 ? cl(((p.x - ox) * sx + (p.y - oy) * sy) / sl, 0, 1) : 1, dd = Math.hypot(ox + sx * tt - p.x, oy + sy * tt - p.y), R = (CTRL_R + (p.a.yogunluk - 10) * .04 + (p.role === 'Bekçi' ? .5 : 0)) * (p.team !== att && f.from && Math.hypot(ox + sx * tt - f.from.x, oy + sy * tt - f.from.y) < REL_D ? REL_K : 1);
      if (dd < R && dd < bd) { bd = dd; best = p; }
    }
    if (best) {
      const p = best, mate = p.team === att, sk = mate || p.role === 'Bekçi' ? p.a.tutus : (p.a.kesme + p.a.tutus) / 2, pr = oppInf(ps, p, -1);
      const P = cl(CTRL0 - sp * (mate && p === f.recv ? CTRL_V * .7 : CTRL_V) - (f.dead ? 0 : ch * (mate ? CTRL_CH * .6 : CTRL_CH)) * (sp > .3 ? 1 : .3) + (sk - 10) * .035 + (mate && p === f.recv ? RCV_B : 0) - Math.max(0, pr - .4) * (mate ? .2 : .3), .04, .98);
      if (m.r() < P) {
        c.flight = null; c.holder = p; c.holdT = 0; c.x = p.x; c.y = p.y;
        if (mate && (f.pr >= 2 || (ps.filter(q => q.team !== att && q.role !== 'Bekçi' && Math.hypot(q.x - p.x, q.y - p.y) < 9).length >= 2 && m.r() < .3 + okuma(p) * .035))) { p.quick = m.tick + Math.round(8 + okuma(p)); m.st.esc = m.st.esc || [0, 0]; m.st.esc[att]++; }
        if (mate) {
          if (f.defl && f.kuyu) { m.st.rebW = m.st.rebW || [0, 0]; m.st.rebW[att]++; ev(m, 'pass', att, `Seken Çekirdeği ${p.name} kaptı`, c.x, c.y); }
          else if (!f.kuyu && p !== f.from) { m.st.passOk[att]++; if (f.lob) m.st.lobOk[att]++;
            if (f.banked) ev(m, 'pass', att, `${f.from.name} → ${p.name} · kenardan sektirerek aktarım`);
            else if (f.w) ev(m, 'pass', att, `${f.from.name} → ${p.name} · boş alana, koşu yoluna aktarım${f.why && f.why.byp >= 2 ? ` · ${f.why.byp} savunmacıyı aştı` : ''}`);
            else if (f.why && f.why.byp >= 2) ev(m, 'pass', att, `${f.from.name} → ${p.name} · ${f.why.byp} savunmacıyı aşan aktarım`); }
        } else {
          m.st.kesme[p.team]++; m.st.lost[att][lane(c.y)]++;
          if (f.kuyu && p.role === 'Bekçi' && !f.defl) ev(m, 'kesme', p.team, `${p.name} ${f.from.name}'in göndermesini tuttu`, c.x, c.y);
          else if (f.banked || f.w) ev(m, 'kesme', p.team, `${p.name} ${f.banked ? 'kenardan seken' : 'boş alana atılan'} aktarımı süpürdü`, c.x, c.y);
          else if (!f.dead) ev(m, 'kesme', p.team, `${p.name} ${f.from.name}'in aktarımını kesti`, c.x, c.y);
        }
        return;
      }
      p.ctrlCD = m.tick + 10; const a = (m.r() - .5) * DEFL_A, ca = Math.cos(a), sa = Math.sin(a), k = DEFL_K * (.6 + m.r() * .6);
      f.vx = (vx * ca - vy * sa) * k; f.vy = (vx * sa + vy * ca) * k; if (!f.defl) f.looseT = m.tick; f.defl = true; f.recv = null;
      if (f.kuyu && !f.reb) { f.reb = true; m.st.reb = m.st.reb || [0, 0]; m.st.reb[att]++; ev(m, 'kesme', p.team === att ? 1 - att : p.team, p.role === 'Bekçi' ? `${p.name} çeldi, Çekirdek sekti` : `Çekirdek ${p.name}'den sekti`, c.x, c.y); }
      else if (sp > .6) ev(m, mate ? 'kopma' : 'kesme', mate ? 1 - att : p.team, mate ? `${p.name} Çekirdeği kontrol edemedi` : `${p.name} dokundu, Çekirdek sekti`, c.x, c.y);
    }
  }

  function bodyP(ps, h, G, t, v0) { let surv = 1; const L = Math.hypot(G.x - h.x, G.y - h.y) || 1, ux = (G.x - h.x) / L, uy = (G.y - h.y) / L, k = FRIC / (1 - FRIC);
    for (const o of ps) { if (o.team === t) continue; const rx = o.x - h.x, ry = o.y - h.y, al = rx * ux + ry * uy; if (al < -1 || al > L + 1) continue; const pe = Math.abs(-rx * uy + ry * ux), v = Math.max(.1, v0 - k * Math.max(0, al)), tt = Math.max(0, al) / Math.max(.3, (v0 + v) / 2), reach = CTRL_R + (o.a.yogunluk - 10) * .04 + (o.role === 'Bekçi' ? .5 : 0) + (.226 + o.a.hiz * .0055) * Math.max(0, tt - 3) * .8;
      if (pe < reach) { const Pc = cl(CTRL0 - v * CTRL_V + ((o.a.kesme + o.a.tutus) / 2 - 10) * .035, .04, .98); surv *= 1 - Pc * (1 - pe / reach * .5); } }
    return surv; }

  function simBall(m, h, A, v0, t, q, shotY) {
    const ps = m.ps, dir = dirOf(t), ch = m.core.charge ?? .5, kF = FRIC, gx = oppX(t); let x = h.x, y = h.y; const an = Math.hypot(A.x - x, A.y - y) || 1; let vx = (A.x - x) / an * v0, vy = (A.y - y) / an * v0, e = .27 + h.a.aktarim * .045 + .4 * ch - hurry(ps, h), surv = 1; const used = new Set();
    for (let k = 1; k <= 50; k++) {
      const tk = k * 2, kt = K(ps, x, y, null) * dir; let sp = Math.hypot(vx, vy), fr = kF;
      if (kt < 0) { fr += -kt * OPP_DRAG * (1 - PIERCE_P * ch); e -= -kt * sp * 2 * DRAIN * (shotY != null ? SHOT_DR * (1 - SH_PIERCE * ch) : PASS_DR); } else fr *= 1 - OWN_FLOW * Math.min(1, kt);
      e -= sp * 2 * ATTN; if (e <= 0) return shotY != null ? 0 : surv * .35;
      const dm = (1 - fr) * (1 - fr); vx *= dm; vy *= dm; x += vx * 2; y += vy * 2; sp = Math.hypot(vx, vy);
      if (y < .5 || y > H - .5) { y = y < .5 ? 1 - y : 2 * H - 1 - y; vy = -vy * REST; vx *= REST; e -= WALL_L; }
      if (x < .5 || x > W - .5) { const ex = x < .5 ? 0 : W; if (shotY != null && ex === gx) return Math.abs(y - H / 2) <= MOUTH ? surv : 0; x = ex === 0 ? 1 - x : 2 * W - 1 - x; vx = -vx * REST; e -= WALL_L; }
      for (const o of ps) { if (o.team === t || used.has(o) || o.stun > m.tick) continue; const reach = (CTRL_R + (o.a.yogunluk - 10) * .04 + (o.role === 'Bekçi' ? .5 : 0)) * (Math.hypot(x - h.x, y - h.y) < REL_D ? REL_K : 1) + (.226 + o.a.hiz * .0055) * Math.max(0, tk - 4) * .85, dd = Math.hypot(o.x - x, o.y - y);
        if (dd < reach) { used.add(o); const Pc = cl(CTRL0 - sp * CTRL_V + ((o.role === 'Bekçi' ? o.a.tutus : (o.a.kesme + o.a.tutus) / 2) - 10) * .035 - ch * CTRL_CH, .04, .98); surv *= 1 - Pc * (1 - dd / reach * .5); } }
      if (shotY != null) continue;
      if (sp < STOP_V * 2) { let dM = 1e9, dO = 1e9; for (const o of ps) { const dd = Math.hypot(o.x - x, o.y - y); if (o.team === t) { if (o !== h && dd < dM) dM = dd; } else if (dd < dO) dO = dd; } return surv * (dM < dO ? .8 : .2); }
      if (q) { const rq = CTRL_R + (.226 + q.a.hiz * .0055) * Math.max(0, tk - 2); if (Math.hypot(q.x - x, q.y - y) < rq) { const Pm = cl(CTRL0 - sp * CTRL_V * .7 - ch * CTRL_CH * .6 + (q.a.tutus - 10) * .035 + RCV_B, .04, .98); return surv * (Pm + (1 - Pm) * .4); } }
    }
    return shotY != null ? 0 : surv * .3;
  }
  function launch(m, h, f) {
    f.ch = m.core.charge ?? .5;
    f.e = .27 + h.a.aktarim * .045 + (f.pl ? .28 : .4) * (m.core.charge ?? .5) - hurry(m.ps, h); f.t0 = m.tick;
    const sg = spread(m, h, f.len || 10), g = () => (m.r() + m.r() + m.r() - 1.5) * 1.15 * sg;
    m.st.devSum[h.team] += sg; m.st.devN[h.team]++;
    if (f.kuyu) { f.to = { x: f.to.x, y: f.to.y + g() * SHOT_S * (f.pl ? PL_S * cl(1 - (okuma(h) - 10) * .03, .6, 1.3) : 1) }; }
    else {
      if (!f.recv) { f.recv = f.to; f.lead = true; f.to = { x: f.to.x, y: f.to.y }; }
      let dx = g(), dy = g(); if (f.w) { const ux = f.to.x - h.x, uy = f.to.y - h.y, un = Math.hypot(ux, uy) || 1, lerr = g() * TB_S * (f.len || 10) / 20; dx += ux / un * lerr; dy += uy / un * lerr; } f.dev = Math.hypot(dx, dy); f.to = { x: cl(f.to.x + dx, 1, W - 1), y: cl(f.to.y + dy, 1, H - 1) };
    }
    f.pr = m.ps.filter(q => q.team !== h.team && q.role !== 'Bekçi' && Math.hypot(q.x - h.x, q.y - h.y) < 7).length;
    if (!f.lob && !f.orta) { f.phys = true; const ax = f.to.x - h.x, ay = f.to.y - h.y, an = Math.hypot(ax, ay) || 1; let v0;
      if (f.kuyu) v0 = (f.pl ? PL_V : SH_V) * (.75 + .5 * f.ch) * (1 + (h.a.aktarim - 10) * .02);
      else v0 = Math.min(V_MAX + h.a.aktarim * .04 + f.ch * .5, (f.w ? .08 : VA_PASS) + an * FRIC / (1 - FRIC)) * (1 + g() * .25);
      f.vx = ax / an * v0; f.vy = ay / an * v0; m.core.x = h.x; m.core.y = h.y; }
    m.core.flight = f; m.core.charge = Math.max(.15, (m.core.charge ?? .5) * (f.kuyu ? 1 : cl(1 - CARRY_L * Math.hypot(f.to.x - f.from.x, f.to.y - f.from.y) / 30, CARRY_MIN, 1)));
  }
  function chooseIntent(m, h, md) {
    const t = h.team, dir = dirOf(t), ps = m.ps, tac = m.tac[t], ok = okuma(h);
    const fx = oppX(t) - h.x, fy = H / 2 - h.y, fn = Math.hypot(fx, fy) || 1;
    const gkR = oppX(t), aheadQ = ps.filter(q => q.team !== t && !hid(q) && Math.abs(q.x - gkR) < Math.abs(h.x - gkR) + 2);
    let run = 0; for (let k = 2; k <= 24; k += 2) { const rx = h.x + fx / fn * k, ry = h.y + fy / fn * k; if (rx < 1 || rx > W - 1 || ry < 1 || ry > H - 1) break; let o = 0; for (const q of aheadQ) o += infl(q, rx, ry); if (o > .45) break; run = k; }
    let ex = 0; for (let i = 0; i < 8; i++) { const a = i * Math.PI / 4, x = h.x + Math.cos(a) * 6, y = h.y + Math.sin(a) * 6; if (x > 1 && x < W - 1 && y > 1 && y < H - 1 && oppAt(ps, t, x, y, t) < .45) ex++; }
    const np = ps.filter(p => p.team !== t && d2(p, h) < 36).length;
    const opn = (x, y) => Math.exp(-oppAt(ps, t, cl(x, 1, W - 1), cl(y, 1, H - 1), t) * 2);
    const farY = h.y < H / 2 ? H * .8 : H * .2, sw = (opn(h.x + dir * 4, farY) + opn(h.x + dir * 12, farY)) / 2 - (opn(h.x + dir * 4, h.y) + opn(h.x + dir * 12, h.y)) / 2;
    const arriving = ps.some(q => q.team === t && q !== h && ((q.vx || 0) * dir > .15) && oppAt(ps, t, q.tx, q.ty, t) < .4 && Math.abs(q.tx - h.x) < 40);
    const n = () => (m.r() - .5) * (20 - ok) * .03, ch = m.core.charge ?? .5;
    const S = {
      'İlerle': run / 24 * 1.2 + (md === 'Dikine' ? .2 : 0) + (h.a.hiz - 10) * .02 + n(),
      'Çek': (np <= 1 ? .35 : .05) * (h.a.surme / 12) + (tac.bosluk === 'Tuzak' ? .25 : 0) + (md === 'Sabırlı' ? .25 : 0) + (h.ori === 'Taşı' ? .1 : 0) + n(),
      'Değiştir': sw * 1.2 + (md === 'Sabırlı' ? .15 : 0) + (tac.mesafe - .5) * .2 + n(),
      'Bekle': (arriving ? .35 : 0) + (ch < .6 && np === 0 ? .25 : 0) + (md === 'Sabırlı' ? .1 + (ch < .85 && np <= 1 ? .3 : 0) : 0) + n(),
      'Kaç': (ex <= 2 ? .8 : ex <= 4 ? .3 : 0) + (ok - 10) * .03 * np + n()
    };
    let bi = 'İlerle', bs = -9; for (const [k, v] of Object.entries(S)) if (v > bs) { bs = v; bi = k; }
    h.intent = bi; h.intentT = m.tick; const I = m.st.intent[t]; I[bi] = (I[bi] || 0) + 1;
  }
  function predictField(m, t, B, T) {
    const ps = m.ps, out = [], opp = ps.filter(q => q.team !== t), near = opp.filter(q => q.role !== 'Bekçi').sort((a, b) => d2(a, B) - d2(b, B)).slice(0, PRED_N);
    for (const q of ps) {
      let nx = q.x, ny = q.y; const sp = .226 + q.a.hiz * .0055;
      if (q.team !== t) {
        const lag = Math.max(8, 44 - 1.6 * okuma(q)), maxD = sp * Math.max(0, T - lag); let gx, gy;
        if (q.role === 'Bekçi') { gx = q.x; gy = H / 2 + (B.y - H / 2) * .35; }
        else if (near.includes(q)) { gx = B.x; gy = B.y; }
        else { gx = q.x + (B.x - q.x) * PRED_X; gy = q.y + (B.y - q.y) * PRED_Y; }
        const dx = gx - q.x, dy = gy - q.y, dd = Math.hypot(dx, dy); if (dd > .01) { const k = Math.min(1, maxD / dd); nx = q.x + dx * k; ny = q.y + dy * k; }
      } else if (q.tx != null) { const dx = q.tx - q.x, dy = q.ty - q.y, dd = Math.hypot(dx, dy); if (dd > .01) { const k = Math.min(1, sp * T / dd); nx = q.x + dx * k; ny = q.y + dy * k; } }
      if (nx === q.x && ny === q.y) { out.push(q); continue; }
      const o = Object.create(q); o.x = nx; o.y = ny; o._ex = q._ex + (nx - q.x); o._ey = q._ey + (ny - q.y); o._orig = q; out.push(o);
    }
    return out;
  }
  function routeVal(ps, t, x, y, akt, view) { const th = threat(t, x, y), kp = { x: oppX(t), y: H / 2 }, ch = lineCharge(ps, { x, y }, kp, view ?? t, dirOf(t), akt, .5) - .15, Ps = 1 / (1 + Math.exp(-ch * 7)), fr = Math.exp(-oppAt(ps, t, x, y, t) * 2); return th * (RV0 + RV1 * fr + (1 - RV0 - RV1) * Ps); }
  function tryPass(m, h) {
    const t = h.team, dir = dirOf(t), ps = m.ps, tac = m.tac[t], md = m.amode || 'Dengeli', ok = okuma(h);
    const noise = () => (m.r() - .5) * (20 - ok) * .02;
    const riskW = Math.max(.2, 1.2 - .8 * tac.risk - .02 * (h.a.cesaret - 10) + (md === 'Sabırlı' ? .4 : md === 'Dikine' ? -.3 : 0));
    const pw = md === 'Dikine' ? .3 : md === 'Sabırlı' ? 0 : .1, gainV = x => pw * ((x - h.x) * dir) / 50;
    const oiH = oppAt(ps, t, h.x, h.y, t);
    const w2 = cl((ok - 6) / 10, 0, 1);
    const V0 = POSS_K * holdVal(ps, t, h.x, h.y), lossV = (tt, x, y) => lossCost(tt, x, y) + V0;
    const ax = h.tx - h.x, ay = h.ty - h.y, an = Math.hypot(ax, ay) || 1, sp = .226 + h.a.hiz * .0055;
    const gkR = oppX(t), aheadQ = ps.filter(q => q.team !== t && !hid(q) && Math.abs(q.x - gkR) < Math.abs(h.x - gkR) + 2), oppA = (x, y) => { let o = 0; for (const q of aheadQ) o += infl(q, x, y); return o; };
    const runCap = 16 + ok * 1.2; let run = 0; for (let k = 2; k <= runCap; k += 2) { const rx = h.x + ax / an * k, ry = h.y + ay / an * k; if (rx < 1 || rx > W - 1 || ry < 1 || ry > H - 1 || oppA(rx, ry) > .45) break; run = k; }
    const reach = Math.max(sp * 14, run), px = cl(h.x + ax / an * reach, 1, W - 1), py = cl(h.y + ay / an * reach, 1, H - 1), openS = cl(run / 20, 0, 1);
    const nearD = Math.min(...ps.filter(p => p.team !== t).map(p => Math.sqrt(d2(p, h))));
    const dri = h.a.surme * 1.6 + h.a.hiz * .4;
    const dfdK = ps.filter(p => p.team !== t && !(p.stun > m.tick)).reduce((a, b) => d2(b, h) < d2(a, h) ? b : a), ndK = Math.sqrt(d2(dfdK, h));
    const supAK = ps.filter(q => q.team === t && q !== h && d2(q, h) < 64).length, supDK = ps.filter(q => q.team !== t && q !== dfdK && d2(q, h) < 64).length;
    const cageK = ps.filter(q => q.team !== t && !(q.stun > m.tick) && Math.sqrt(d2(q, h)) < 4 + ok * .15).length;
    const aSK = (h.a.surme * 1.6 + h.a.hiz * .4 + (h.ori === 'Taşı' ? 3 : 0)) * (1 + SUP * supAK), dSK = Math.max(1, dfdK.a.kesme + dfdK.a.yogunluk * .5 + (dfdK.outF < .6 ? -6 : 0)) * (1 + SUP * supDK);
    const ktK = K(ps, h.x, h.y, null) * dir, kthK = .6 - tac.tempo * .1 + h.a.tutus * .015 + (md === 'Sabırlı' ? .12 : 0), kopK = ndK < 4 ? Math.min(.6, 24 * KOP_P / (1 + Math.exp((ktK + kthK) * 10))) : 0;
    const pLoseK = (ndK < 6 ? dSK / (aSK + dSK) * DUEL * cageMul(cageK) * (ndK < 2.4 ? 1 : .6) : 0) + kopK;
    let nIn = 0; for (const q of ps) { if (q.team === t || q.role === 'Bekçi') continue; const dx = h.x - q.x, dy = h.y - q.y, dd = Math.hypot(dx, dy); if (dd > 14 || dd < .1) continue; const cv = ((q.vx || 0) * dx + (q.vy || 0) * dy) / dd; if (dd < 4 || (cv > .1 && (dd - 3) / cv < ANT_T)) nIn++; }
    const antL = cl((ok - 6) / 12, 0, 1) * (nIn >= 3 ? ANT3 : nIn === 2 ? ANT2 : 0);
    const pk = cl(1 - (pLoseK + antL) * (1 + (m.r() - .5) * (20 - ok) * .06), 0, 1);
    const it = h.intent || 'İlerle', np = ps.filter(p => p.team !== t && d2(p, h) < 36).length;
    const iKeep = it === 'İlerle' ? (run > 4 ? .08 : 0) : it === 'Çek' ? (np < 2 ? .1 : -.05) : it === 'Bekle' ? (oiH < .5 ? .06 : 0) : it === 'Değiştir' ? -.05 : -.1;
    const gk = oppX(t), gsN = ps.filter(q => q.team !== t && q.role !== 'Bekçi' && Math.abs(q.x - gk) < Math.abs(h.x - gk)).length;
    const chD = Math.min(...ps.filter(q => q.team !== t).map(q => Math.sqrt(d2(q, h)))), brk = gsN === 0 ? BRK * cl((chD - 2.5) / 8, 0, 1) * (1 + (h.a.hiz - 12) * .05) : 0;
    const hvK = w2 > 0 ? routeVal(ps, t, px, py, h.a.aktarim) * (1 - w2) + routeVal(predictField(m, t, { x: px, y: py }, reach / sp + 10), t, px, py, h.a.aktarim) * w2 : routeVal(ps, t, px, py, h.a.aktarim);
    const keep = pk * hvK * (1 - RUN_D * reach) + brk + gainV(px) - (1 - pk) * lossV(t, h.x, h.y) * riskW + (tac.tasima - .5) * .1 + (h.ori === 'Taşı' ? .08 : h.ori === 'Aktar' ? -.08 : 0) + iKeep + (oiH < .35 ? HOLDV * (1 - (m.core.charge ?? .5)) : 0) - (tac.tempo - .5) * .12 * (1 - openS) + tac.tempo * TEMPO_RUN * openS + (md === 'Sabırlı' ? (m.core.charge < .85 ? .06 : -.04) : 0) + noise();
    let bv = keep + KEEP_M + (h.feintGo > m.tick ? FG_K : 0), kind = null, best = null, X = null;
    const gd = Math.hypot(h.x - oppX(t), h.y - H / 2);
    if (gd < SHOT_D) {
      const chg0 = m.core.charge ?? .5, bk = ps.find(q => q.team !== t && q.role === 'Bekçi'), kp0 = { x: oppX(t), y: H / 2 }, sk0 = cl(.35 + okuma(h) / 25 - (h.a.cesaret - 10) * .03, .4, 1.2), sgP = spread(m, h, gd) * SHOT_S * sk0, vP = SH_V * (.75 + .5 * chg0) * (1 + (h.a.aktarim - 10) * .02), sE = Math.exp((m.r() - .5) * (20 - ok) * SIM_ERR), PP = cl(sE * [-.8, 0, .8].reduce((a, o) => a + simBall(m, h, { x: kp0.x, y: kp0.y + o * sgP }, vP, t, null, 0), 0) / 3, 0, 1);
      const kpl = { x: oppX(t), y: H / 2 + (bk && bk.y > H / 2 ? -1 : 1) * MOUTH * .6 }, sgL = spread(m, h, gd) * SHOT_S * sk0 * PL_S * cl(1 - (okuma(h) - 10) * .03, .6, 1.3), vL = PL_V * (.75 + .5 * chg0) * (1 + (h.a.aktarim - 10) * .02), PL = cl(sE * [-.8, 0, .8].reduce((a, o) => a + simBall(m, h, { x: kpl.x, y: kpl.y + o * sgL }, vL, t, null, 0), 0) / 3, 0, 1);
      const pl = PL > PP, kp = pl ? kpl : kp0, P = pl ? PL : PP; kp.pl = pl; if (m.dbgS) m.dbgS.push([gd, PP, PL]);
      const lane1 = !ps.some(p => p.team !== t && p.role !== 'Bekçi' && (p.x - h.x) * dir > 0 && Math.abs(p.y - (h.y + (H / 2 - h.y) * Math.abs(p.x - h.x) / Math.max(1, gd))) < 6);
      const pReb = cl(.25 + (m.core.charge ?? .5) * .45 - 12 * .012, .1, .65), Pp = cl(P * Math.exp((m.r() - .5) * (20 - okuma(h)) * PERC), 0, 1), rbx = oppX(t) - dir * 8, rbA = ps.filter(q => q.team === t && q !== h && Math.hypot(q.x - rbx, q.y - H / 2) < 12).length, rbD = ps.filter(q => q.team !== t && Math.hypot(q.x - rbx, q.y - H / 2) < 12).length, rbS = (rbA + .3) / (rbA + rbD + .6), v = Pp * 1.5 * (1 + (h.a.cesaret - 10) * .03) + (1 - Pp) * pReb * REB_V * rbS * 2 - (1 - Pp) * (1 - pReb) * lossV(t, h.x, h.y) * riskW * .3 + (lane1 ? .15 : 0) + (md === 'Sabırlı' ? .12 * (m.core.charge ?? .5) : 0) + noise(); if (v > bv) { bv = v; kind = 'shot'; X = kp; }
    }
    let fB = null, fP = 0;
    if (gd < SHOT_D + 6 && !(h.feintT > m.tick)) {
      fB = ps.filter(p => p.team !== t && p.role !== 'Bekçi' && !(p.stun > m.tick) && Math.hypot(p.x - h.x, p.y - h.y) < FEINT_R && (p.x - h.x) * dir > -1).sort((u, v) => d2(u, h) - d2(v, h))[0] || null;
      if (fB) { const cred = cl((m.core.charge ?? .5) * 1.2 + (1 - gd / SHOT_D) * .6, .15, 1), sk = (h.a.surme + okuma(h) - okuma(fB) - fB.a.kesme) / 4; fP = cred * (1 / (1 + Math.exp(-sk)));
        const v = fP * FEINT_V * (1.2 - gd / (SHOT_D + 6) * .6) - (1 - fP) * FEINT_C + noise(); if (v > bv) { bv = v; kind = 'feint'; } }
    }
    const defs = ps.filter(p => p.team !== t && p.role !== 'Bekçi'), deep = Math.max(...defs.map(p => p.x * dir)), lastLT = deep * dir;
    const wingCross = (md === 'Kanat' ? lane(h.y) !== 1 : (h.y < 9 || h.y > H - 9)) && Math.abs(h.x - oppX(t)) < 34;
    for (const q of ps) {
      if (q === h || q.team !== t) continue; const d = Math.sqrt(d2(q, h)); if (d < 5) continue;
      const oiQ = oppAt(ps, t, q.x, q.y, t), byp = defs.filter(p => (p.x - h.x) * dir > 0 && (q.x - p.x) * dir > 0).length;
      if (d <= 20 + 25 * tac.mesafe) {
        const spdQ = Math.hypot(q.vx || 0, q.vy || 0), T = d / 1.5;
        const targets = [{ x: q.x, y: q.y, lead: false }];
        if (spdQ > .1) targets.push({ x: cl(q.x + q.vx * T * .9, 2, W - 2), y: cl(q.y + q.vy * T * .9, 2, H - 2), lead: true });
        if (ok >= 8 && q.role !== 'Bekçi') { const gx = oppX(t) - q.x, gy = H / 2 - q.y, gn = Math.hypot(gx, gy) || 1, ux = gx / gn, uy = gy / gn, angs = ok >= 12 ? [0, .6, -.6] : [0], vq = .226 + q.a.hiz * .0055, vb = LEAD_SPD * (.8 + h.a.aktarim * .02), rq = Math.max(2, 12 - okuma(q) * .5);
          for (const a of angs) for (const L of [5, 9, 14, 20]) { const dx = ux * Math.cos(a) - uy * Math.sin(a), dy = ux * Math.sin(a) + uy * Math.cos(a), G = { x: cl(q.x + dx * L, 2, W - 2), y: cl(q.y + dy * L, 2, H - 2), lead: true, space: true }, dG = Math.hypot(G.x - h.x, G.y - h.y);
            if (dG > 30 + 25 * tac.mesafe) continue; const Tb = dG / vb, tR = Math.hypot(G.x - q.x, G.y - q.y) / vq + rq; let tD = 1e9; for (const o of ps) if (o.team !== t) { const v = Math.hypot(G.x - o.x, G.y - o.y) / (.226 + o.a.hiz * .0055) + 4; if (v < tD) tD = v; }
            tD *= Math.exp((m.r() - .5) * (20 - ok) * PERC_R * (1 + dG / 20));
            if (tR <= Tb + LOOSE_T * .6 && tR < tD - SP_MARGIN && oppAt(ps, t, G.x, G.y, t) < SP_OPEN && holdVal(ps, t, G.x, G.y) > holdVal(ps, t, q.x, q.y) + SP_GAIN) { G.run = cl((tD - tR) / 12, 0, 1); targets.push(G); } } }
        if (ok >= 10 && q.role !== 'Bekçi') for (const wy of [0, H]) { const qy2 = wy === 0 ? -q.y : 2 * H - q.y, tt = (wy - h.y) / (qy2 - h.y); if (tt <= .15 || tt >= .85) continue; const B = { x: h.x + (q.x - h.x) * tt, y: wy === 0 ? .6 : H - .6 }, L1 = Math.hypot(B.x - h.x, B.y - h.y), L2 = Math.hypot(q.x - B.x, q.y - B.y); if (L1 + L2 > 28 + 25 * tac.mesafe || L1 < 4) continue; targets.push({ x: q.x, y: q.y, lead: true, bank: B, blen: L1 + L2, l1: L1, l2: L2 }); }
        for (const G of targets) {
          const oiG = G.lead ? oppAt(ps, t, G.x, G.y, t) : oiQ;
          const ch = G.bank ? lineCharge(ps, h, G.bank, t, dir, h.a.aktarim, m.core.charge) + lineCharge(ps, G.bank, G, t, dir, h.a.aktarim, m.core.charge) - (.27 + h.a.aktarim * .045 + .4 * (m.core.charge ?? .5)) - WALL_L : lineCharge(ps, h, G, t, dir, h.a.aktarim, m.core.charge), Pl = 1 / (1 + Math.exp(-ch * 7)), Pt = 1 - Math.max(0, oiG - .5) * .6 * (1 - q.a.tutus / 30);
          const v0e = (G.space ? .08 : VA_PASS) + Math.hypot(G.x - h.x, G.y - h.y) * FRIC / (1 - FRIC), v0s = G.bank ? (VA_PASS + G.l2 * FRIC / (1 - FRIC)) / REST + G.l1 * FRIC / (1 - FRIC) + .05 : Math.min(V_MAX + h.a.aktarim * .04 + (m.core.charge ?? .5) * .5, v0e), Pb = cl(simBall(m, h, G.bank || G, v0s, t, q) * Math.exp((m.r() - .5) * (20 - ok) * SIM_ERR), 0, 1), P = Pb * Math.min(1, Pt * (.9 + h.a.aktarim * .005) * (G.lead ? .8 + ok * .01 : 1) * (G.space ? .75 + okuma(q) * .0125 : 1) * (G.bank ? cl(BANK_ACC + (h.a.aktarim - 10) * .025 + (ok - 10) * .015, .3, 1) : 1) * Math.exp(-spread(m, h, d) * cl(.35 + ok / 25, .4, 1.2) / 5));
          const bypG = G.lead ? defs.filter(p => (p.x - h.x) * dir > 0 && (G.x - p.x) * dir > 0).length : byp;
          let iB = G.space ? SPACE_B * (G.run || 0) * threat(t, G.x, G.y) * 2 : 0;
          if (md === 'Dikine' && (G.x - lastLT) * dir > 0 && (h.x - lastLT) * dir < 0) iB += DIK_B;
          if (it === 'Değiştir' && Math.abs(G.y - h.y) > 18) iB += .12;
          if (it === 'Çek' && np >= 2) iB += .1 * Math.exp(-oiG * 2);
          if (it === 'Bekle' && G.lead) iB += .08;
          if (md === 'Sabırlı' && d > 20) iB += .12 * (m.core.charge ?? .5);
          let after2 = 0; const after0 = routeVal(ps, t, G.x, G.y, q.a.aktarim) + bypG * .04 + gainV(G.x) + (md === 'Kanat' && lane(G.y) !== 1 ? .05 : 0) + (tac.bosluk === 'Yükleme' && Math.abs(G.y - h.y) > 22 ? .04 : 0) + iB;
          if (w2 > 0) { const pp = predictField(m, t, G, d / 1.5 + 6), hs = routeVal(ps, t, G.x, G.y, q.a.aktarim); let nb = 0;
            for (const r of pp) { const ro = r._orig || r; if (r.team !== t || ro === h || ro === q || r.role === 'Bekçi') continue; const ch2 = lineCharge(pp, G, r, t, dir, q.a.aktarim, .3), P2 = 1 / (1 + Math.exp(-ch2 * 7)), v2 = P2 * holdVal(pp, t, r.x, r.y); if (v2 > nb) nb = v2; }
            after2 = w2 * (Math.max(routeVal(pp, t, G.x, G.y, q.a.aktarim), nb * COMBO) - hs); }
          const after = after0 + after2;
          const v = P * after - (1 - P) * lossV(t, (h.x + G.x) / 2, (h.y + G.y) / 2) * riskW + noise();
          if (v > bv) { bv = v; kind = G.lead ? 'lead' : 'pass'; best = q; X = { P, byp: bypG, G, it, np }; }
        }
      }
      const lobR = 10 + h.a.aktarim * .5 + 26 * (m.core.charge ?? .5);
      if (d >= 14 && d <= lobR && (q.x - h.x) * dir > -12) {
        const k0 = K(ps, h.x + (q.x - h.x) * .1, h.y + (q.y - h.y) * .1, t) * dir, Tf = d / 1.1, lnd = [[0, 0], [dir * 2.5, 0]];
        if (Math.hypot(q.vx || 0, q.vy || 0) > .05) lnd.push([q.vx * Tf * .8, q.vy * Tf * .8]);
        for (const [ox, oy] of lnd) {
          const L = { x: cl(q.x + ox, 2, W - 2), y: cl(q.y + oy, 2, H - 2) }; let ls = k0;
          for (const tt of [.8, .9, 1]) ls = Math.min(ls, K(ps, h.x + (L.x - h.x) * tt, h.y + (L.y - h.y) * tt, t) * dir);
          const pf = predictField(m, t, L, Tf); let ds = 0; for (const p of pf) { if (p.team === t) continue; const dp = Math.hypot(p.x - L.x, p.y - L.y); if (dp < 6) ds += (p.a.kesme * .5 + p.a.yogunluk * .3) * (1 - dp / 6); }
          const dq = Math.hypot(ox, oy) * .5, rs = (q.a.hiz * .4 + q.a.surme * .3 + 4) * (1 - dq / 6), Pc = rs / (rs + ds + .01);
          const P = Pc * (1 / (1 + Math.exp(-(ls + LOB_K) * 5))) * (.6 + h.a.aktarim * .015) * (ox || oy ? .9 + ok * .005 : 1);
          const v = P * (routeVal(ps, t, L.x, L.y, q.a.aktarim) + byp * .04 + gainV(L.x)) - (1 - P) * lossV(t, L.x, L.y) * riskW * .7 + (tac.mesafe - .5) * .15 + noise();
          if (v > bv) { bv = v; kind = 'lob'; best = q; X = L; }
        }
      }
      if (wingCross && Math.hypot(q.x - oppX(t), q.y - H / 2) < 15 && Math.hypot(q.x - oppX(t), q.y - H / 2) > 9) {
        const nd = ps.filter(p => p.team !== t && d2(p, q) < 49);
        const blind = nd.length ? nd.filter(p => (q.x - p.x) * p.fx + (q.y - p.y) * p.fy < 0).length / nd.length : 1;
        const rs = q.a.hiz * .5 + q.a.surme * .3 + blind * 8 + 3, P = rs / (rs + (nd.length ? 9 + nd.length * 2 : 0));
        const v = P * .9 * (.75 + q.a.aktarim * .012) - (1 - P) * lossV(t, q.x, q.y) * riskW * .4 + (md === 'Kanat' ? .08 : 0) + noise();
        if (v > bv) { bv = v; kind = 'orta'; best = q; X = { blind }; }
      }
    }
    m.st.dec = m.st.dec || { keep: 0, pass: 0, lead: 0, lob: 0, orta: 0, shot: 0, feint: 0 }; m.st.feint = m.st.feint || [[0, 0], [0, 0]];
    if (!kind) { m.st.dec.keep++; return; }
    m.st.dec[kind]++; m.core.holdT = 0;
    if (kind === 'feint') { h.feintT = m.tick + FEINT_CD; m.st.feint[t][0]++;
      if (m.r() < fP) { m.st.feint[t][1]++; fB.stun = m.tick + cl(Math.round(28 + (okuma(h) - okuma(fB)) * 2), 18, 50); fB.outF = OUT; h.feintGo = m.tick + 24; h.feintBy = fB; ev(m, 'surme', t, `Gönderme çalımı · ${h.name} göndermeyi gösterdi, ${fB.name} hattı kapatmaya atıldı ve yerinden oynadı`); }
      else { fB.pLock = m.tick + 40; ev(m, 'kopma', 1 - t, `${fB.name}, ${h.name}'in gönderme çalımını okudu`); }
      return; }
    if (kind === 'shot') { m.st.shot[t]++; ev(m, 'gonder', t, `${h.name} Kuyu'ya ${X.pl ? 'plase' : 'güçlü'} gönderdi`); launch(m, h, { team: t, from: h, to: X, kuyu: true, pl: !!X.pl, akt: h.a.aktarim, trav: 0, len: gd }); return; }
    m.st.pass[t]++;
    if (kind === 'orta') {
      m.st.orta[t]++; const dv = (20 - h.a.aktarim) * .25, L = { x: cl(best.x + dir * 1.5 + (m.r() - .5) * dv, 2, W - 2), y: cl(best.y + (m.r() - .5) * dv, 2, H - 2) }, len = Math.hypot(L.x - h.x, L.y - h.y); m.st.passLen[t] += len;
      launch(m, h, { team: t, from: h, to: L, recv: best, orta: true, blind: X.blind, akt: h.a.aktarim, trav: 0, len }); return;
    }
    if (kind === 'lob') {
      const L = X, dev = (20 - h.a.aktarim) * .3; L.x = cl(L.x + (m.r() - .5) * dev, 2, W - 2); L.y = cl(L.y + (m.r() - .5) * dev, 2, H - 2);
      const len = Math.hypot(L.x - h.x, L.y - h.y); m.st.passLen[t] += len; m.st.lob[t]++;
      launch(m, h, { team: t, from: h, to: L, recv: best, lob: true, akt: h.a.aktarim, trav: 0, len }); return;
    }
    if (X.it === 'Değiştir' && Math.abs(X.G.y - h.y) > 18) ev(m, 'pass', t, `${h.name} oyunun yönünü değiştirdi → ${best.name}`);
    else if (X.it === 'Çek' && X.np >= 2) ev(m, 'pass', t, `${h.name} ${X.np} rakibi üstüne çekip ${best.name}'e bıraktı`);
    if (kind === 'lead') { const L = { x: X.G.x, y: X.G.y }, len = Math.hypot(L.x - h.x, L.y - h.y); m.st.passLen[t] += len; m.st.lead[t]++; launch(m, h, { team: t, from: h, to: L, recv: best, lead: true, w: !!X.G.space, akt: h.a.aktarim, trav: 0, len, why: X }); if (X.G.space) { m.st.space = m.st.space || [0, 0]; m.st.space[t]++; } if (X.G.bank) { const f = m.core.flight, B = { x: X.G.bank.x + (m.r() - .5) * (24 - h.a.aktarim) * .25, y: X.G.bank.y }, ax = B.x - h.x, ay = B.y - h.y, an = Math.hypot(ax, ay) || 1, v0 = (VA_PASS + X.G.l2 * FRIC / (1 - FRIC)) / REST + X.G.l1 * FRIC / (1 - FRIC) + .05; f.vx = ax / an * v0; f.vy = ay / an * v0; f.via = true; f.len = X.G.blen; m.st.bank = m.st.bank || [0, 0]; m.st.bank[t]++; } return; }
    const len = Math.sqrt(d2(best, h)); m.st.passLen[t] += len;
    launch(m, h, { team: t, from: h, to: best, akt: h.a.aktarim, trav: 0, len, why: X });
  }
  function stats(m) {
    let a = 0, b = 0, n = 0; const e = m.ps.map(() => 0);
    for (let j = 0; j < 12; j++) for (let i = 0; i < 25; i++) {
      const x = i * 4 + 2, y = j * 4.17 + 2, k = K(m.ps, x, y, null); n++;
      m.st.heat[j * 25 + i] += k;
      if (k > .15) a++; else if (k < -.15) b++; else continue;
      let bi = -1, bv = 0; const tm = k > 0 ? 0 : 1;
      for (const p of m.ps) { if (p.team !== tm) continue; const v = infl(p, x, y); if (v > bv) { bv = v; bi = p.id; } }
      if (bi >= 0) e[bi]++;
    }
    m.st.heatN++;
    m.st.ctrl[0] += a / n; m.st.ctrl[1] += b / n; m.st.ctrlN++; m.st.now = [a / n, b / n]; m.st.etkiNow = e;
    const c = m.core, att = c.flight ? c.flight.team : c.holder.team, dt = 1 - att;
    const ds = m.ps.filter(p => p.team === dt && p.role !== 'Bekçi');
    const avg = ds.reduce((s, p) => s + Math.abs(p.x - ownX(dt)), 0) / ds.length;
    m.st.blockX[dt] += avg; m.st.blockN[dt]++;
  }
  const lane = y => y < H / 3 ? 0 : y < 2 * H / 3 ? 1 : 2;
  const COUNTER = {
    'Alan': { ritim: 'Kontra', bosluk: 'Yükleme', why: 'Bölgelerini koruyorlar, Çekirdeğe göre kayıyorlar.', not: 'Testte Sabırlı (2,0–4,2) ve Kanat (3,3–4,2) burada kaybetti.' },
    'Adam adama': { ritim: 'Dengeli', bosluk: 'Genişlik', why: 'Her savunmacı bir hücumcuyu takip ediyor; yer değil, adam tutuyorlar.', not: 'Geniş durmak markajı sahaya yayıyor: testte 5,0–1,5.' },
    'Ön alan': { ritim: 'Dikine', bosluk: 'Yükleme', why: 'Senin yarında yüksek hat tutup Çekirdeği erken kapıyorlar.', not: 'Sabırlı oynamak felaket: testte 2,7–6,3. Dikine 4,8–1,3.' },
    'Kuyu önü': { ritim: 'Dikine', bosluk: 'Yükleme', why: 'Bütün oyuncular kendi Kuyu’larının önünde, sıkışık.', not: 'Yükleme en az sayı yediren seçenek (2,3–2,7); Kanat ritmi 2,0–4,3 ile kötü.' }
  };
  const PRESS_COUNTER = {
    'Her zaman': ['Taşıyana sürekli basıyorlar.', 'Aktar yönelimi ve yüksek tempo ile presi tek dokunuşta geç.'],
    'Kanatta': ['Çekirdek kanada gidince iki-üç kişi basıyor, merkezde tek kişi.', 'Kanat ritminden kaçın, merkezden oyna.'],
    'Kayıptan sonra': ['Çekirdeği kaybettikleri ilk anlarda toplu basıyor, sonra bloğa dönüyorlar.', 'Kazanınca ilk aktarımı uzun yap ya da kısa bir an sabret.'],
    'Kendi yarımızda': ['Kendi yarılarına gelene kadar hiç basmıyorlar.', 'Kendi yarında rahat kur, onların yarısına girince hızlan.']
  };
  const RITIM_COUNTER = {
    'Dikine': ['Çekirdeği kazanınca hemen ileri oynuyorlar.', 'Ön alan baskısı yapma (testte Dikine’ye karşı 1,3–4,8 kaybetti); Alan ya da Kuyu önü.'],
    'Sabırlı': ['Yavaş ve güvenli dolaşıyorlar.', 'Ön alan baskısı kur: Sabırlı ritim buna karşı 6,3–2,7 kaybetti.'],
    'Kontra': ['Kazanınca hızlanıyor, yoksa yavaş kuruyorlar.', 'Hücumda Kuyu önüne çok kalabalık girme; kayıpta geri dönüş presi kullan.'],
    'Kanat': ['Kanatlardan taşıyıp içeri kesiyorlar.', 'Kanatta pres tetikleyicisi kur.'],
    'Dengeli': ['Belirgin bir ritimleri yok.', 'Kendi planını uygula.']
  };
  function attackMode(m, t, since) {
    const r = m.tac[t].ritim; let md = r;
    if (r === 'Kontra') md = since < 150 ? 'Dikine' : 'Sabırlı';
    m.amode = md; return md;
  }
  function assignMarks(m, att) {
    const atk = m.ps.filter(p => p.team === att && p.role !== 'Bekçi'), dfs = m.ps.filter(p => p.team !== att && p.role !== 'Bekçi');
    const free = new Set(atk); for (const p of m.ps) p.mark = null;
    for (const d of dfs.sort((a, b) => a.sx - b.sx)) { let best = null, bd = 1e9; for (const a of free) { const dd = d2(a, d); if (dd < bd) { bd = dd; best = a; } } if (best) { d.mark = best; free.delete(best); } }
  }
  function step(m) {
    CUR = m.core;
    if (m.over) return;
    m.tick++;
    if (m.tick % EV === 0) {
      if (m.evre >= 3) { m.over = true; m.tick--; ev(m, 'end', null, `Maç bitti · ${m.score[0]}–${m.score[1]}`); return; }
      m.evre++; for (const p of m.ps) p.st = Math.min(1, p.st + .35);
      kickoff(m, (m.first + m.evre - 1) % 2); ev(m, 'evre', null, `Evre ${m.evre} başladı · kondisyon kısmen yenilendi`);
      [0, 1].forEach(t => { if (m.ai[t]) aiAdapt(m, t); });
    }
    applyPending(m);
    const ps = m.ps, c = m.core;
    const att = c.flight ? c.flight.team : c.holder.team, dir = dirOf(att);
    for (const p of ps) { p.bus = p.team !== att && m.tac[p.team].savunma === 'Kuyu önü' && Math.abs(p.x - ownX(p.team)) < 26; if (p.team === att || p.role === 'Bekçi') { p.outF = 1; continue; } const behind = (c.x - p.x) * dir > 2; p.outF = p.stun > m.tick ? OUT : (p.outF ?? 1) + ((behind ? OUT : 1) - (p.outF ?? 1)) * .08; }
    prep(ps);
    if (att !== m.lastAtt) {
      const lost = 1 - att, ahead = ps.filter(p => p.team === lost && p.role !== 'Bekçi' && (p.x - c.x) * dirOf(lost) > 2).length;
      const fresh = m.lastAtt != null;
      for (const p of ps) p.pLock = 0;
      for (const p of ps) p.rec = p.team === lost && p.role !== 'Bekçi' && p.ori !== 'Önde bekle' && (p.x - c.x) * dirOf(lost) > 2 ? m.tick + Math.max(2, Math.round(26 - okuma(p) * 1.2)) : 0;
      m.lastAtt = att; m.gainT = m.tick; assignMarks(m, att); c.charge = fresh ? Math.min(1, .3 + GECIS * ahead) : .3;
      if (fresh && ahead >= 3) { m.st.gecis[att]++; ev(m, 'pass', att, `Geçiş · ${ahead} rakip önde yakalandı, Çekirdek %${Math.round(c.charge * 100)} şarjla başladı`); }
    }
    const since = m.tick - (m.gainT || 0);
    { const dt = 1 - att, k = ({ 'Yok': 0, '1 kişi': 1, '2 kişi': 2 })[m.tac[dt].arkada] ?? 1; for (const p of ps) p.sweep = false; ps.filter(p => p.team === dt && p.role !== 'Bekçi' && p.ori !== 'Önde bekle').sort((u, v) => (u.sx0 ?? u.sx) - (v.sx0 ?? v.sx)).slice(0, k).forEach(p => p.sweep = true); }
    const defs = ps.filter(p => p.team !== att && p.role !== 'Bekçi' && p.ori !== 'Önde bekle' && !p.sweep && !(p.stun > m.tick)).sort((a, b) => d2(a, c) - d2(b, c));
    const dtac = m.tac[1 - att], inOwnHalf = (c.x - W / 2) * dir > 0;
    let np = 1 + Math.round(dtac.pres * dtac.pres * 5);
    if (dtac.presTetik === 'Kanatta') np = lane(c.y) !== 1 ? np + 1 : 1;
    else if (dtac.presTetik === 'Kayıptan sonra') np = since < 90 ? np + 2 : Math.max(1, np - 1);
    else if (dtac.presTetik === 'Kendi yarımızda') np = inOwnHalf ? np + 1 : 0;
    if (dtac.savunma === 'Ön alan' && !inOwnHalf) np += 1;
    if (dtac.savunma === 'Kuyu önü') np = Math.min(np, 1);
    const ownKx = ownX(att), deepB = Math.abs(c.x - ownKx) < 32;
    const allowDeep = dtac.savunma === 'Ön alan' || dtac.presTetik === 'Her zaman';
    const pR = 8 + 22 * dtac.pres + (dtac.savunma === 'Ön alan' ? 10 : 0) + (dtac.presTetik === 'Her zaman' ? 6 : 0);
    const npC = cl(np, 0, 6), locked = defs.filter(p => p.pLock > m.tick && since > 0 && Math.sqrt(d2(p, c)) < LOCK_R).slice(0, npC);
    const pressers = (deepB && !allowDeep) ? locked : locked.concat(defs.filter(p => !locked.includes(p) && Math.sqrt(d2(p, c)) < pR).slice(0, npC - locked.length));
    for (const p of defs) if (!pressers.includes(p)) p.pLock = 0;
    const amode = attackMode(m, att, since);
    const exitsV = [];
    if (!c.flight && c.holder) { const h = c.holder, cand = []; for (let i = 0; i < 8; i++) { const a = i * Math.PI / 4, ux = Math.cos(a), uy = Math.sin(a), x = h.x + ux * 6, y = h.y + uy * 6; if (x < 1 || x > W - 1 || y < 1 || y > H - 1) continue; cand.push([ux, uy, holdVal(ps, att, x, y)]); } cand.sort((a, b) => b[2] - a[2]); for (const e of cand.slice(0, 2)) exitsV.push(e); }
    const lobC = c.flight && c.flight.lob ? ps.filter(q => q.team !== att && q.role !== 'Bekçi' && Math.hypot(q.x - c.flight.to.x, q.y - c.flight.to.y) < 10).sort((u, v) => d2(u, c.flight.to) - d2(v, c.flight.to))[0] : null;
    const rcvs = ps.filter(q => q.team === att && q !== c.holder && q.role !== 'Bekçi').sort((u, v) => d2(u, c) - d2(v, c));
    const lanesT = (!c.flight && c.holder) ? ps.filter(q => q.team === att && q !== c.holder && q.role !== 'Bekçi') : [];
    const oppL = ps.filter(q => q.team !== att && q.role !== 'Bekçi'), lastL = oppL.length ? oppL.reduce((a, b) => Math.abs(b.x - oppX(att)) < Math.abs(a.x - oppX(att)) ? b : a).x : oppX(att);
    for (const p of ps) { if (p.sx0 == null) { p.sx0 = p.sx; p.sy0 = p.sy; } p.sx = p.sx0; p.sy = p.sy0; }
    { const dd = ps.filter(p => p.team !== att && p.role !== 'Bekçi' && p.ori !== 'Önde bekle'); if (dd.length >= 2) { const bx = dd.slice().sort((u, v) => u.sx0 - v.sx0), by = dd.slice().sort((u, v) => u.sy0 - v.sy0), lo = bx[0].sx0, hi = Math.max(...dd.map(p => p.sx0)), n = dd.length;
      bx.forEach((p, i) => p.sx = lo + (hi - lo) * i / (n - 1) * COMPACT); by.forEach((p, i) => p.sy = .12 + .76 * i / (n - 1)); } }
    const ownA = ownX(att), hang = ps.filter(q => q.team !== att && q.role !== 'Bekçi' && Math.abs(q.x - ownA) < Math.abs(c.x - ownA) - 8), guard = new Map();
    if (hang.length) { const pool = ps.filter(q => q.team === att && q.role !== 'Bekçi' && q !== c.holder && !(c.flight && q === c.flight.recv)).sort((u, v) => u.sx - v.sx); const nG = m.tac[att].risk > .75 ? hang.length - 1 : hang.length;
      for (const hq of hang.slice(0, nG)) { let bi = -1, bd = 1e9; pool.forEach((q, i) => { if (guard.has(q)) return; const dd = Math.abs(q.x - hq.x) + Math.abs(q.y - hq.y) + q.sx * 30; if (dd < bd) { bd = dd; bi = i; } }); if (bi >= 0) guard.set(pool[bi], hq); } }
    const runners = (lane(c.y) !== 1 && Math.abs(c.x - oppX(att)) < 34) ? ps.filter(p => p.team === att && p !== c.holder && (p.role === 'Delici' || p.role === 'Gölge' || p.ori === 'Sız')).slice(0, 2) : [];
    const cf = c.flight, chasers = new Set(), rebSet = new Set(); let antic = null;
    if (cf && cf.phys && (cf.dead || cf.defl || Math.hypot(cf.vx, cf.vy) < .25)) {
      const bp = { x: c.x + cf.vx * 4, y: c.y + cf.vy * 4 }, tOf = p => Math.hypot(p.x - bp.x, p.y - bp.y) / (.226 + p.a.hiz * .0055), byT = [0, 1].map(tm => ps.filter(p => p.team === tm && (p.role !== 'Bekçi' || Math.hypot(bp.x - ownX(tm), bp.y - H / 2) < 12)).sort((u, v) => tOf(u) - tOf(v)));
      for (const tm of [0, 1]) { const mine = byT[tm], opp = byT[1 - tm]; if (!mine.length) continue; const b = mine[0], tb = tOf(b), to = opp.length ? tOf(opp[0]) : 1e9;
        if (m.tick - (cf.looseT ?? cf.t0) < Math.round(Math.max(1, 10 - okuma(b) * .4))) continue;
        const ratio = tb / Math.max(1, to) * Math.exp((m.r() - .5) * (20 - okuma(b)) * .03);
        if (ratio < CH_GIVEUP) chasers.add(b);
        if (mine[1] && ratio > .75 && ratio < 1.3 && tOf(mine[1]) < tb * 1.35) chasers.add(mine[1]); }
    }
    if (cf && cf.kuyu) { const rx = cf.to.x - dirOf(cf.team) * 6; for (const tm of [0, 1]) ps.filter(p => p.team === tm && p.role !== 'Bekçi' && p !== cf.from).sort((u, v) => Math.hypot(u.x - rx, u.y - H / 2) - Math.hypot(v.x - rx, v.y - H / 2)).slice(0, 2).forEach(p => rebSet.add(p)); }
    const c0x = c.x, c0y = c.y;
    for (const p of ps) if (!p.press) { p.hx = p.x; p.hy = p.y; }
    const underPr = !cf && c.holder && ps.filter(q => q.team !== c.holder.team && q.role !== 'Bekçi' && Math.hypot(q.x - c.holder.x, q.y - c.holder.y) < 7).length >= 2, vac = [];
    if (!cf && c.holder) for (const q of ps) if (q.team !== c.holder.team && q.press && q.hx != null && Math.hypot(q.x - q.hx, q.y - q.hy) > 7) vac.push({ x: cl(q.hx, 2, W - 2), y: cl(q.hy, 2, H - 2) });
    let preN = 0; const pp = cf && cf.phys && !cf.kuyu && !cf.defl && !cf.dead ? cf.to : (!cf && c.holder ? c.holder : null), pT = cf ? cf.team : c.holder ? c.holder.team : null;
    if (pp) { for (const q of ps) { if (q.team === pT || q.role === 'Bekçi' || q.press) continue; const dx = pp.x - q.x, dy = pp.y - q.y, dd = Math.hypot(dx, dy), cv = ((q.vx || 0) * dx + (q.vy || 0) * dy) / (dd || 1); if (dd < 4 || dd > 14) continue; if ((cf && dd < 11) || (cv > .08 && (dd - 3) / cv < 22)) { preN++; vac.push({ x: cl(q.x, 2, W - 2), y: cl(q.y, 2, H - 2), pre: true }); } } }
    const underPr2 = underPr || preN >= 2;
    if (cf && cf.phys && !cf.kuyu && !cf.defl && !cf.dead) { const dst = cf.to; c.x = c.x + (dst.x - c.x) * ANTIC; c.y = c.y + (dst.y - c.y) * ANTIC; antic = true; }
    m.tcx = m.tcx || [c.x, c.x]; for (const tm of [0, 1]) m.tcx[tm] += (c.x - m.tcx[tm]) * SHAPE_LAG;
    for (const p of ps) {
      const tac = m.tac[p.team];
      if (chasers.has(p)) { const f = c.flight, px = c0x + f.vx * 4, py = c0y + f.vy * 4; { p.tx = px; p.ty = py; p.aT = false; p._otx = p.tx; p._oty = p.ty; p._ov = true; } }
      if (c.flight && (c.flight.w || c.flight.final || c.flight.via) && p.team !== c.flight.team && (p.role !== 'Bekçi' || Math.hypot((c.flight.final || c.flight.to).x - ownX(p.team), (c.flight.final || c.flight.to).y - H / 2) < 16 + okuma(p) * .4)) { const f = c.flight, lp = f.final || f.to; const rq = f.recv && f.recv.x != null ? f.recv : null, tMe = Math.hypot(p.x - lp.x, p.y - lp.y) / (.226 + p.a.hiz * .0055) * (1 + (m.r() - .5) * (20 - okuma(p)) * .04), tRc = rq ? Math.hypot(rq.x - lp.x, rq.y - lp.y) / (.226 + rq.a.hiz * .0055) : 1e9;
        if (tMe < tRc + 4 && Math.hypot(p.x - lp.x, p.y - lp.y) < 26 && m.tick - (f.t0 ?? m.tick) >= Math.round(Math.max(1, 10 - okuma(p) * .5))) { p.tx = lp.x; p.ty = lp.y; p.aT = false; p._otx = p.tx; p._oty = p.ty; p._ov = true; } }
      if (c.flight && c.flight.kuyu && rebSet.has(p)) { const f = c.flight, gx = f.to.x, fd = dirOf(f.team); if (Math.hypot(p.x - gx, p.y - H / 2) < 26 && m.tick - (f.t0 ?? m.tick) >= Math.round(14 - okuma(p) * .5)) { p.tx = gx - fd * (5 + (p.id % 3) * 2); p.ty = H / 2 + ((p.id % 4) - 1.5) * 4; p.aT = false; p._otx = p.tx; p._oty = p.ty; p._ov = true; } }
      if (p.team === att) {
        if (p === c.holder && !c.flight) { if (m.tick % 4 === p.id % 4) carrierAim(m, p); }
        else if (p.role === 'Bekçi') { p.tx = ownX(p.team) + dir * 7; p.ty = H / 2 + (c.y - H / 2) * .35; }
        else if ((m.tick + p.id) % Math.max(5, Math.round(18 - okuma(p) / 2)) === 0 || (underPr2 && (m.tick + p.id) % 3 === 0 && Math.hypot(p.x - c.x, p.y - c.y) < 30 && m.r() < .4 + okuma(p) * .03)) {
          let push = (p.role === 'Delici' ? 6 : 0) + (p.ori === 'Sız' ? 10 : 0) - (p.ori === 'Destek' ? 10 : 0) + (amode === 'Dikine' ? 6 : 0);
          let sy = p.sy, wide = .55 + .45 * tac.genislik;
          if (tac.bosluk === 'Rotasyon' && Math.floor(m.tick / 300) % 2 === 1) sy = 1 - sy;
          if (tac.bosluk === 'Genişlik') wide = 1.15;
          if (amode === 'Kanat') wide = Math.max(wide, 1.05);
          if (tac.bosluk === 'Tuzak') { if (p.role === 'Mıknatıs' || p.role === 'Gerer' || p.ori === 'Destek') push -= 9; else if (p.role === 'Delici' || p.ori === 'Sız') push += 6; }
          const restO = REST0 + REST_K * (1 - tac.risk) - (amode === 'Dikine' ? .06 : 0) + (tac.ritim === 'Kontra' ? .08 : 0);
          let bx = m.tcx[p.team] + dir * ((p.sx - restO) * (24 + tac.hatlar * 32) + (p.sx < restO ? 0 : tac.blok * 6 + push)), by = H / 2 + (sy - .5) * H * wide;
          if (tac.bosluk === 'Yükleme') { const far = m.ps.filter(q => q.team === p.team && q.role !== 'Bekçi' && q !== c.holder).reduce((a, b) => Math.abs(b.y - c.y) > Math.abs(a.y - c.y) ? b : a); by = p === far ? (c.y < H / 2 ? H - 4 : 4) : by + (c.y - by) * .5; }
          if (p.ori === 'Destek') by = by + (c.y - by) * .5;
          if (runners.includes(p)) { bx = oppX(att) - dir * 10; by = c.y < H / 2 ? H / 2 + 6 : H / 2 - 6; }
          else if (amode === 'Dikine' && p.sx >= (since < DIK_T ? restO : .7)) { const lx = lastL + (oppX(att) - lastL) * cl(DIK_F + (p.a.hiz - 12) * .02, .1, .8) + dir * DIK_D; bx = Math.abs(lx - oppX(att)) < 8 ? oppX(att) - dir * 8 : lx; by = H / 2 + (sy - .5) * H * .75; p.dik = true; }
          const ok = okuma(p), wP = cl((ok - 6) / 10, 0, 1) * PRED_W, pt = PANT * ok - PLAG * (20 - ok), n = 2 + Math.round(ok / 1.5), rad = RAD0 + ok * RAD_K, hd = c.flight ? c.flight.to : c.holder;
          const mates = ps.filter(q => q.team === p.team && q !== p && q.role !== 'Bekçi');
          const oD = ps.filter(q => q.team !== p.team && q.role !== 'Bekçi' && !hid(q)), aP = mates.map(q => q === c.holder ? [q.x, q.y] : [q.tx ?? q.x, q.ty ?? q.y]), nOf = (x, y) => { let bq = null, bd = 1e9; for (const q of oD) { const dd = Math.hypot(q.x - x, q.y - y); if (dd < bd) { bd = dd; bq = q; } } return [bq, bd]; }, aNear = aP.map(([x, y]) => nOf(x, y)), wO = cl((ok - 4) / 12, 0, 1), gkx = oppX(p.team);
          let best = -9, btx = bx, bty = by; const keepT = p.aT && Math.hypot(p.tx - bx, p.ty - by) < rad * 1.2;
          const vS = vac.filter(v => okuma(p) >= (v.pre ? 10 : 8));
          for (let i = -1; i < n + vS.length; i++) {
            if (i < 0 && !keepT) continue;
            const ox = i >= n ? vS[i - n].x : i < 0 ? p.tx : cl(i ? bx + (m.r() - .5) * 2 * rad : bx, 2, W - 2), oy = i >= n ? vS[i - n].y : i < 0 ? p.ty : cl(i ? by + (m.r() - .5) * 2 * rad : by, 2, H - 2);
            let oi = 0; for (const q of ps) if (q.team !== p.team && !hid(q)) oi += infl(q, ox - (q.vx || 0) * pt, oy - (q.vy || 0) * pt);
            if (wP > 0) { const pf = predictField(m, p.team, { x: ox, y: oy }, Math.hypot(ox - c.x, oy - c.y) / 1.5 + 6); let oiP = 0; for (const q of pf) if (q.team !== p.team && !hid(q)) oiP += infl(q, ox, oy); oi = oi * (1 - wP) + oiP * wP; }
            let md2 = 99; for (const q of mates) { const qx = q === c.holder ? q.x : q.tx, qy = q === c.holder ? q.y : q.ty, dd = Math.hypot(qx - ox, qy - oy); if (dd < md2) md2 = dd; }
            let ln = 0; if (hd) { for (let k = 1; k <= 5; k++) { const t = k / 6, lx = hd.x + (ox - hd.x) * t, ly = hd.y + (oy - hd.y) * t; for (const q of ps) if (q.team !== p.team && !hid(q)) ln += infl(q, lx - (q.vx || 0) * pt, ly - (q.vy || 0) * pt); } ln /= 5; }
            let ovB = 0, glB = 0; if (wO > 0 && oD.length) { const [nd, dO] = nOf(ox, oy); if (dO < OV_R && aNear.some(([q, dd]) => q === nd && dd < OV_R)) ovB = 1; let l2 = 0; for (let k = 1; k <= 3; k++) { const tt = k / 4, lx = ox + (gkx - ox) * tt, ly = oy + (H / 2 - oy) * tt; for (const q of oD) l2 += infl(q, lx, ly); } glB = Math.exp(-l2 * 2 / 3); }
            const thO = threat(p.team, ox, oy);
            const sc = (i < 0 ? HYST : 0) + Math.exp(-oi * 2) * SP_W + wO * thO * (ovB * OV_W + glB * GL_W) + Math.exp(-ln * 2.5) * LN_W + threat(p.team, ox, oy) * TH_W - Math.max(0, SPACE_D - md2) / SPACE_D * SPC_W - Math.hypot(ox - bx, oy - by) * .012 + (m.r() - .5) * (20 - ok) * .02;
            if (sc > best) { best = sc; btx = ox; bty = oy; }
          }
          p.tx = btx; p.ty = bty; p.aT = true;
          if (guard.has(p)) { const hq = guard.get(p), gx = ownA - hq.x, gy = H / 2 - hq.y, gn = Math.hypot(gx, gy) || 1, gs = GUARD_D + (20 - okuma(p)) * .25; p.tx = hq.x + gx / gn * gs; p.ty = hq.y + gy / gn * gs; p.aT = false; }
        }
      } else {
        p.aT = false; const ox = ownX(p.team);
        if (p.role === 'Bekçi') { const vx = c.x - ox, vy = c.y - H / 2, dk = Math.hypot(vx, vy) || 1, out = cl(dk * (.14 + okuma(p) * .004), 3, 9); p.tx = ox + vx / dk * out; p.ty = H / 2 + vy / dk * out; }
        else if (p.ori === 'Önde bekle') { const gk = oppX(p.team), dd = dirOf(p.team), rear = ps.filter(q => q.team === att && q.role !== 'Bekçi'), rl = rear.length ? rear.reduce((u, v) => Math.abs(v.x - gk) < Math.abs(u.x - gk) ? v : u).x : gk - dd * 30; let wx = rl + dd * (3 + p.a.hiz * .1); if (Math.abs(wx - gk) < 12) wx = gk - dd * 12; p.tx = wx; p.ty = H / 2 + (p.sy - .5) * 22; }
        else if (p.sweep) { const rear = ps.filter(q => q.team === att && q.role !== 'Bekçi'), rl = rear.length ? rear.reduce((u, v) => Math.abs(v.x - ox) < Math.abs(u.x - ox) ? v : u).x : c.x, tw = Math.sign(ox - rl) || 1; const thr = rear.slice().sort((u, v) => (Math.abs(u.x - ox) + Math.abs(u.y - H / 2) * .5) - (Math.abs(v.x - ox) + Math.abs(v.y - H / 2) * .5))[0], tgt = (Math.abs(c.x - ox) <= Math.abs(thr ? thr.x - ox : 1e9) + 4) ? c : thr, vx = ox - tgt.x, vy = H / 2 - tgt.y, dk = Math.hypot(vx, vy) || 1, off = Math.min(dk * .45, 3 + okuma(p) * .25); p.tx = tgt.x + vx / dk * off; p.ty = tgt.y + vy / dk * off; if (tgt === c) p.press = true; }
        else if (pressers.includes(p)) {
          p.press = true; if (!(p.pLock > m.tick)) p.pLock = m.tick + Math.round(LOCK0 + LOCK1 * dtac.pres);
          const k = pressers.indexOf(p);
          if (k === 0) { const gx = ox - c.x, gy = H / 2 - c.y, gn = Math.hypot(gx, gy) || 1; p.tx = c.x + gx / gn * 2.2; p.ty = c.y + gy / gn * 2.2; }
          else if (k <= 2 && exitsV.length >= k) { const e = exitsV[k - 1]; p.tx = c.x + e[0] * 6; p.ty = c.y + e[1] * 6; }
          else if (rcvs.length) { const q = rcvs[(k - 1 - Math.min(2, exitsV.length)) % rcvs.length]; const pt = PANT * okuma(p) - PLAG * (20 - okuma(p)), qx = q.x + (q.vx || 0) * pt, qy = q.y + (q.vy || 0) * pt; p.tx = qx + (c.x - qx) * .25; p.ty = qy + (c.y - qy) * .25; }
          else { p.tx = c.x; p.ty = c.y; }
        }
        else if (p.rec && m.tick >= p.rec && since < REC_T && !((p.x - c.x) * (ox - c.x) > 0 && Math.abs(p.x - ox) < Math.abs(c.x - ox))) {
          p.tx = c.x + (ox - c.x) * (.3 + (1 - p.sx) * .25); p.ty = c.y + (H / 2 - c.y) * .55 + (p.sy - .5) * 12; p.sprint = true;
        }
        else {
          const lag = Math.max(8, Math.round(44 - 1.6 * okuma(p))), sv = tac.savunma, sg = p.team === 0 ? 1 : -1;
          if (sv === 'Adam adama' && p.mark) { if ((m.tick + p.id) % Math.max(4, lag >> 1) === 0) { const mk = p.mark, gx = ox - mk.x, gy = H / 2 - mk.y, gn = Math.hypot(gx, gy) || 1; p.tx = mk.x + gx / gn * 2.5; p.ty = mk.y + gy / gn * 2.5; const mx = BLOCK_MAX + 10 + tac.blok * 25, lim = ox + (p.team === 0 ? 1 : -1) * mx; p.tx = p.team === 0 ? Math.min(p.tx, lim) : Math.max(p.tx, lim); } }
          else if ((m.tick + p.id) % lag === 0) {
            const depth = 1 - p.sx;
            if (sv === 'Kuyu önü') { p.tx = ox + sg * (6 + p.sx * 14); p.ty = H / 2 + (p.sy - .5) * 30 * (.6 + .4 * tac.genislik) + (c.y - H / 2) * .3; }
            else {
              p.tx = c.x + (ox - c.x) * (.12 + depth * (.25 + .35 * tac.hatlar) * (1.15 - tac.blok * .5));
              const cb = .2 + .5 * Math.max(0, 1 - Math.abs(c.x - ox) / 60); p.ty = c.y + (H / 2 - c.y) * cb + (p.sy - .5) * 28 * (.6 + .4 * tac.genislik);
              if ((sv === 'Alan' || sv === 'Ön alan') && lanesT.length) {
                const tg = lanesT.reduce((a, b) => (Math.hypot(b.x - p.tx, b.y - p.ty) - threat(att, b.x, b.y) * 20 < Math.hypot(a.x - p.tx, a.y - p.ty) - threat(att, a.x, a.y) * 20 ? b : a));
                const lx = c.x + (tg.x - c.x) * .55, ly = c.y + (tg.y - c.y) * .55, w = sv === 'Alan' ? COVER : COVER * .6;
                p.tx = p.tx * (1 - w) + lx * w; p.ty = p.ty * (1 - w) + ly * w;
              }
              if (sv !== 'Ön alan') { const mx = BLOCK_MAX + tac.blok * 25, lim = ox + (p.team === 0 ? 1 : -1) * mx; p.tx = p.team === 0 ? Math.min(p.tx, lim) : Math.max(p.tx, lim); }
              if (sv === 'Ön alan' && !inOwnHalf) p.tx = sg > 0 ? Math.max(p.tx, 38 + p.sx * 14) : Math.min(p.tx, 62 - p.sx * 14);
            }
            { const ok = okuma(p), pt = PANT * ok - PLAG * (20 - ok), zr = 5 + ok * .6, ax = p.tx, ay = p.ty; let tgt = null, tv = 0;
              for (const q of ps) { if (q.team !== att || q === c.holder || q.role === 'Bekçi') continue; const qx = q.x + (q.vx || 0) * pt, qy = q.y + (q.vy || 0) * pt, dz = Math.hypot(qx - ax, qy - ay); if (dz > zr) continue;
                const cov = ps.some(o => o !== p && o.team === p.team && !o.press && Math.hypot(o.x - qx, o.y - qy) < dz * .7);
                const v = (threat(att, qx, qy) + .1) * (1 - dz / zr) * (cov ? .3 : 1) * (1 + (ok - 10) * .03); if (v > tv) { tv = v; tgt = { x: qx, y: qy }; } }
              if (tgt) { const gx = ox - tgt.x, gy = H / 2 - tgt.y, gn = Math.hypot(gx, gy) || 1, gs = 2.5 + (1 - p.sx) * 2, zx = tgt.x + gx / gn * gs, zy = tgt.y + gy / gn * gs, w = ZONE_W * (.6 + ok * .02);
                p.tx = ax * (1 - w) + zx * w; p.ty = ay * (1 - w) + zy * w; } }
            if (sv !== 'Kuyu önü') {
              const dk = Math.hypot(c.x - ox, c.y - H / 2), start = 26 + okuma(p) * 1.4 - tac.blok * 8;
              const w = cl((start - dk) / Math.max(6, start - 10), 0, 1);
              if (w > 0) {
                const hold = 6 + tac.blok * 8 + p.sx * 6, f = Math.min(1, hold / Math.max(dk, 1)), gx = ox + (c.x - ox) * f, gy = H / 2 + (c.y - H / 2) * f + (p.sy - .5) * 16 * (1 - w * .5);
                p.tx = p.tx * (1 - w) + gx * w; p.ty = p.ty * (1 - w) + gy * w;
              }
            }
          }
        }
      }
      if (p !== c.holder && !p.press && p.role !== 'Bekçi') for (const q of ps) { if (q === p || q.team !== p.team || q === c.holder) continue; const ex = p.tx - q.tx, ey = p.ty - q.ty, ed = Math.hypot(ex, ey); if (ed < REP_D && ed > .01) { const f = (REP_D - ed) / REP_D * .5; p.tx += ex / ed * f * REP_D * .5; p.ty += ey / ed * f * REP_D * .5; } }
      if (c.flight && (c.flight.lob || c.flight.orta || c.flight.lead) && p === c.flight.recv) { p.tx = (c.flight.final || c.flight.to).x; p.ty = (c.flight.final || c.flight.to).y; if (c.flight.w) p.sprint = true; }
      else if (c.flight && c.flight.lob && p.team !== att && p.role !== 'Bekçi' && p === lobC) { p.tx = c.flight.to.x; p.ty = c.flight.to.y; }
      p.tx = cl(p.tx, 1.5, W - 1.5); p.ty = cl(p.ty, 1.5, H - 1.5); if (p.team === att || !(p.pLock > m.tick)) p.press = false; if (p.team === att || !p.rec || since >= REC_T) p.sprint = false; if (p._ov) { p.tx = cl(p._otx, 1.5, W - 1.5); p.ty = cl(p._oty, 1.5, H - 1.5); p.sprint = true; p._ov = false; }
      let sp = (.226 + p.a.hiz * .0055) * (.75 + .25 * p.st) * (p.sprint ? SPRINT : 1) * (p.press && Math.abs(p.x - ownX(p.team)) < Math.abs(c.x - ownX(p.team)) + 1 ? 1 + PSPD * m.tac[p.team].pres : 1); if (p === c.holder && !c.flight) { const gk0 = oppX(p.team); let oi = 0; for (const q of ps) if (q.team !== p.team && Math.abs(q.x - gk0) < Math.abs(p.x - gk0) + 1) oi += infl(q, p.x, p.y); sp *= oi < .15 ? (p.ori === 'Taşı' ? OPEN_T : OPEN_N) : .8; const gk = ownX(1 - p.team); if (ps.some(q => q.team !== p.team && !(q.stun > m.tick) && d2(q, p) < 9 && Math.abs(q.x - gk) < Math.abs(p.x - gk))) sp *= CONTAIN + p.a.surme * .008; }
      const dx = p.tx - p.x, dy = p.ty - p.y, d = Math.hypot(dx, dy);
      if (d > .05 && (p.fx * dx + p.fy * dy) / d < -.2) sp *= BACKP;
      { let gx, gy; if (p.team !== att || d < sp * 2) { gx = c.x - p.x; gy = c.y - p.y; if (p.team === att) { gx = oppX(p.team) - p.x; gy = H / 2 - p.y; } else if (d > TURN_D) { const bn = Math.hypot(gx, gy) || 1; if ((dx * gx + dy * gy) / (d * bn) < -.2) { gx = dx; gy = dy; } } } else { gx = dx; gy = dy; }
        const gn = Math.hypot(gx, gy) || 1; p.fx += (gx / gn - p.fx) * TURN; p.fy += (gy / gn - p.fy) * TURN; const nn = Math.hypot(p.fx, p.fy) || 1; p.fx /= nn; p.fy /= nn; }
      if (d <= .05) { p.vx = 0; p.vy = 0; }
      if (d > .05) {
        const s = Math.min(d, sp); p.x += dx / d * s; p.y += dy / d * s; p.vx = dx / d * s; p.vy = dy / d * s;
        if (s > sp * .6) p.st = Math.max(.3, p.st - .00003 * (32 - p.a.dayaniklilik) * .5 * (.5 + 1.2 * tac.tempo) * (p.sprint ? 2.5 : 1));
      }
    }
    c.x = c0x; c.y = c0y;
    if (c.flight && c.flight.phys) physStep(m, att);
    else if (c.flight) {
      if (c.flight.final && Math.hypot(c.flight.to.x - c.x, c.flight.to.y - c.y) <= 1.6) { const f = c.flight; c.x = f.to.x; c.y = f.to.y; f.to = f.final; f.final = null; f.banked = true; if (f.e != null) f.e -= WALL_L; ev(m, 'pass', f.team, `${f.from.name} kenardan sektirdi → ${f.recv ? f.recv.name : ''}`, c.x, c.y); return; }
      if (c.flight.loose) { const nr = ps.reduce((a, b) => (d2(b, c) < d2(a, c) ? b : a)); if (Math.sqrt(d2(nr, c)) > 2.5 && m.tick - c.flight.looseT < LOOSE_T) return; }
      const f = c.flight, spd = (f.w ? LEAD_SPD : f.kuyu ? (f.pl ? 1.9 : 2.6) : f.lob ? 1.1 : f.orta ? 1.3 : 1.5) * (.8 + f.akt * .02), dx = f.to.x - c.x, dy = f.to.y - c.y, d = Math.hypot(dx, dy);
      if (d <= spd && f.kuyu && Math.abs(f.to.y - H / 2) > MOUTH) {
        const df = ps.filter(p => p.team !== att).reduce((a, b) => d2(b, f.to) < d2(a, f.to) ? b : a);
        c.x = f.to.x; c.y = f.to.y; c.flight = null; c.holder = df; c.holdT = 0; m.st.miss[att]++;
        ev(m, 'kesme', df.team, `Iska · ${f.from.name} Kuyu'yu tutturamadı`, f.from.x, f.from.y); return;
      }
      if (d <= spd && f.kuyu) { m.score[att]++; ev(m, 'sayi', att, `SAYI · ${f.from.name} Çekirdeği Kuyu'ya gönderdi · ${m.score[0]}–${m.score[1]}`, f.from.x, f.from.y); kickoff(m, 1 - att); return; }
      if (d <= spd && f.orta) {
        const q = f.recv, df = ps.filter(p => p.team !== att).reduce((a, b) => d2(b, f.to) < d2(a, f.to) ? b : a), dd = Math.sqrt(d2(df, f.to));
        const rs = q.a.hiz * .4 + q.a.surme * .25 + f.blind * 4 + (Math.sqrt(d2(q, f.to)) < 2.5 ? 3 : 0); let ds = 0; for (const p of ps) { if (p.team === att) continue; const dp = Math.sqrt(d2(p, f.to)); if (dp < 7) ds += (p.a.kesme * .5 + p.a.yogunluk * .3) * (1 - dp / 7) * (1 + (p.bus ? .3 : 0)); }
        c.x = f.to.x; c.y = f.to.y; c.flight = null; c.holdT = 0;
        if (m.r() < rs / (rs + ds)) {
          c.holder = q; c.holdT = 20; m.st.passOk[att]++; m.st.ortaOk[att]++;
          ev(m, 'pass', att, `Orta · ${f.from.name} → ${q.name}, savunmanın arkasından`);
        } else { c.holder = df; m.st.kesme[df.team]++; m.st.lost[att][lane(c.y)]++; ev(m, 'kesme', df.team, `${df.name} ortayı karşıladı`, c.x, c.y); }
        return;
      }
      if (d <= spd && f.lead) {
        const nr = ps.reduce((a, b) => (d2(b, f.to) < d2(a, f.to) ? b : a)), rcv = Math.sqrt(d2(f.recv, f.to)) < 3 ? f.recv : nr;
        if (f.w && !f.loose && Math.sqrt(d2(nr, f.to)) > 3) { f.loose = true; f.looseT = m.tick; c.x = f.to.x; c.y = f.to.y; return; }
        c.x = f.to.x; c.y = f.to.y; c.flight = null; c.holder = rcv; c.holdT = 0;
        if (f.reb) { m.st.rebW = m.st.rebW || [0, 0]; if (rcv.team === att) m.st.rebW[att]++; ev(m, rcv.team === att ? 'pass' : 'kesme', rcv.team, `Seken Çekirdeği ${rcv.name} kaptı`, c.x, c.y); }
        else if (rcv.team === att) { m.st.passOk[att]++; if (f.banked) ev(m, 'pass', att, `${f.from.name} → ${rcv.name} · kenardan sektirerek aktarım`); else if (f.w) ev(m, 'pass', att, `${f.from.name} → ${rcv.name} · boş alana, koşu yoluna aktarım${f.why && f.why.byp >= 2 ? ` · ${f.why.byp} savunmacıyı aştı` : ''}`); else if (f.why && f.why.byp >= 2) ev(m, 'pass', att, `${f.from.name} → ${rcv.name} · koşu yoluna, ${f.why.byp} savunmacıyı aşan aktarım`); }
        else { m.st.kesme[rcv.team]++; m.st.lost[att][lane(c.y)]++; if (f.banked || f.w) ev(m, 'kesme', rcv.team, `${rcv.name} ${f.banked ? 'kenardan seken' : 'boş alana atılan'} aktarımı süpürdü`, c.x, c.y); else ev(m, 'kesme', rcv.team, f.dev > 2.5 ? `İsabetsiz aktarım · ${f.from.name} acele etti, ${rcv.name} aldı` : `${rcv.name} koşu yoluna atılan Çekirdeği aldı`, c.x, c.y); }
        return;
      }
      if (d <= spd && f.lob) {
        const q = f.recv, dq = Math.sqrt(d2(q, f.to)); let ds = 0, bestD = null, bd = 1e9;
        for (const p of ps) { if (p.team === att) continue; const dp = Math.sqrt(d2(p, f.to)); if (dp < 6) ds += (p.a.kesme * .5 + p.a.yogunluk * .3) * (1 - dp / 6); if (dp < bd) { bd = dp; bestD = p; } }
        const rs = dq < 4 ? (q.a.hiz * .4 + q.a.surme * .3 + 4) * (1 - dq / 6) : 0;
        const nr = rs + ds === 0 ? ps.reduce((a, b) => (d2(b, f.to) < d2(a, f.to) ? b : a)) : (m.r() < rs / (rs + ds) ? q : bestD);
        c.x = f.to.x; c.y = f.to.y; c.flight = null; c.holder = nr; c.holdT = 0;
        if (nr.team === att) { m.st.passOk[att]++; m.st.lobOk[att]++; ev(m, 'pass', att, `${f.from.name} → ${nr.name} · bloğun arkasına aşırtma`); }
        else { m.st.kesme[nr.team]++; m.st.lost[att][lane(c.y)]++; ev(m, 'kesme', nr.team, `${nr.name} aşırtmayı karşıladı`, c.x, c.y); }
        return;
      }
      if (d <= spd) {
        const rc = f.to, pr = oppInf(ps, rc, -1);
        if (pr > .5 && m.r() < (pr - .5) * .6 * (1 - rc.a.tutus / 30)) {
          const tk = ps.filter(p => p.team !== att).reduce((a, b) => d2(b, rc) < d2(a, rc) ? b : a);
          c.flight = null; c.holder = tk; c.holdT = 0; m.st.touchLost[att]++; m.st.lost[att][lane(rc.y)]++; ev(m, 'kopma', tk.team, `İlk dokunuş kaçtı · ${rc.name} baskı altında aldı, ${tk.name} kaptı`, rc.x, rc.y); return;
        }
        c.holder = rc; c.flight = null; c.holdT = 0; m.st.passOk[att]++; if (f.why && f.why.byp >= 2) ev(m, 'pass', att, `${f.from.name} → ${rc.name} · ${f.why.byp} savunmacıyı aşan aktarım`); else if (f.len > 26) ev(m, 'pass', att, `${f.from.name} → ${rc.name} · uzun aktarım`);
      }
      else {
        c.x += dx / d * spd; c.y += dy / d * spd; f.trav += spd; if (f.e != null) f.e -= spd * ATTN;
        const kt = K(ps, c.x, c.y, null) * dir;
        if (f.e == null) f.e = .6 + f.akt * .03;
        let ic = null;
        if (kt < 0 && (!(f.lob || f.orta) || f.trav > f.len * .75)) {
          let sv = 0; for (const p of ps) if (p.team !== att) { const v = infl(p, c.x, c.y) * (1 + p.a.kesme * .05); if (v > sv) { sv = v; ic = p; } }
          f.e -= -kt * spd * DRAIN * (.5 + (ic ? ic.a.kesme * .05 : 0)) * (f.kuyu ? SHOT_DR * (f.pl ? PL_D : 1 - SH_PIERCE * (f.ch ?? .5)) : 1);
        }
        if (f.e <= 0 && !ic) { const nr = ps.reduce((a, b) => (d2(b, c) < d2(a, c) ? b : a)); c.flight = null; c.holder = nr; c.holdT = 0; if (nr.team !== att) { m.st.kesme[nr.team]++; m.st.lost[att][lane(c.y)]++; ev(m, 'kesme', nr.team, `Çekirdek gücünü yitirdi · ${nr.name} aldı`, c.x, c.y); } else m.st.passOk[att]++; return; }
        if (f.e <= 0 && ic) {
          const opp = ps.filter(p => p.team !== att);
          const hidden = opp.some(p => hid(p) && infl(p, c.x, c.y) > .3);
          if (f.kuyu && m.r() < cl(.25 + (f.ch ?? .5) * .45 - ic.a.tutus * .012, .1, .65)) { const an = Math.atan2(c.y - ic.y, c.x - ic.x) + Math.PI + (m.r() - .5) * 2.4, dl = 4 + m.r() * 6, to = { x: cl(c.x + Math.cos(an) * dl, 2, W - 2), y: cl(c.y + Math.sin(an) * dl, 2, H - 2) }; c.flight = { team: att, from: ic, to, recv: ic, lead: true, reb: true, akt: 8, trav: 0, len: dl, e: 2 }; m.st.reb = m.st.reb || [0, 0]; m.st.reb[att]++; ev(m, 'kesme', ic.team, `Çekirdek ${ic.name}'den sekti`, c.x, c.y); return; }
          c.flight = null; c.holder = ic; c.holdT = 0; m.st.kesme[ic.team]++; m.st.lost[att][lane(c.y)]++;
          ev(m, 'kesme', ic.team, `Çekirdek ${ic.name}'in alanında söndü${hidden ? ' · Gölge alanı görünmüyordu' : ''}`, c.x, c.y);
        }
      }
    } else {
      const h = c.holder; c.x = h.x; c.y = h.y; c.holdT++;
      { const oiC = oppAt(ps, att, h.x, h.y, null), kc = K(ps, h.x, h.y, null) * dir; if (c.charge == null) c.charge = .5; const mvS = Math.hypot(h.vx || 0, h.vy || 0), still = cl(1 - mvS / .25, 0, 1), spaceF = CH_SP0 + (1 - CH_SP0) * Math.exp(-oiC * 2); c.charge = cl(c.charge + CHG2 * (1 + (h.a.tutus - 10) * .05) * (CH_MV + (1 - CH_MV) * still) * spaceF * (1 - c.charge * .5), 0, 1); }
      if (c.holdT === 1 || m.tick - (h.intentT || 0) > 30) chooseIntent(m, h, amode);
      if (Math.hypot(h.x - oppX(att), h.y - H / 2) < KR + 1.5) {
        m.score[att]++; ev(m, 'sayi', att, `SAYI · ${h.name} Kuyu'ya ulaştı · ${m.score[0]}–${m.score[1]}`, h.x, h.y);
        kickoff(m, 1 - att); return;
      }
      const kt = K(ps, h.x, h.y, null) * dir;
      const kth = .6 - m.tac[att].tempo * .1 + h.a.tutus * .015 + (m.amode === 'Sabırlı' ? .12 : 0);
      if (c.holdT > 10 && m.r() < KOP_P / (1 + Math.exp((kt + kth) * 10))) {
        const tk = ps.filter(p => p.team !== att).reduce((a, b) => d2(b, h) < d2(a, h) ? b : a);
        if (Math.sqrt(d2(tk, h)) < 3) { c.holder = tk; c.holdT = 0; m.st.kopma[att]++; m.st.lost[att][lane(h.y)]++; ev(m, 'kopma', tk.team, `Kopma · ${h.name} alanını kaybetti, ${tk.name} aldı`, h.x, h.y); return; }
      }
      if (m.tick % 24 === 0) {
        const dfd = ps.filter(p => p.team !== att && !(p.stun > m.tick)).reduce((a, b) => d2(b, h) < d2(a, h) ? b : a);
        if (Math.sqrt(d2(dfd, h)) < 2.4) {
          const supA = ps.filter(q => q.team === att && q !== h && d2(q, h) < 64).length, supD = ps.filter(q => q.team !== att && q !== dfd && d2(q, h) < 64).length;
          const aS = (h.a.surme * 1.6 + h.a.hiz * .4 + (h.ori === 'Taşı' ? 3 : 0)) * (1 + SUP * supA), dS = Math.max(1, dfd.a.kesme + dfd.a.yogunluk * .5 + (dfd.outF < .6 ? -6 : 0)) * (1 + SUP * supD);
          const pD = dS / (aS + dS), cg = ps.filter(q => q.team !== att && !(q.stun > m.tick) && Math.sqrt(d2(q, h)) < 4).length;
          if (m.r() < pD * DUEL * cageMul(cg)) { c.holder = dfd; c.holdT = 0; m.st.duelLost[att]++; m.st.lost[att][lane(h.y)]++; ev(m, 'kopma', dfd.team, `İkili mücadele · ${dfd.name}, ${h.name}'dan Çekirdeği aldı`, h.x, h.y); return; }
          if (m.r() < (1 - pD) * .6) { dfd.stun = m.tick + 45; dfd.outF = OUT; m.st.beat[att]++; ev(m, 'surme', att, `Sürme · ${h.name}, ${dfd.name}'ı geçti`); }
        }
      }
      const tac = m.tac[att], every = Math.round(12 - tac.tempo * 6);
      const minHold = Math.round((3 + tac.tasima * 6) * (1.4 - tac.tempo * .8)) + (amode === 'Sabırlı' ? 3 : amode === 'Dikine' ? -2 : 0) + (h.ori === 'Taşı' ? 8 : 0) - (h.ori === 'Aktar' ? 3 : 0);
      if ((c.holdT > minHold || h.quick > m.tick) && (m.tick % every === 0 || h.quick > m.tick || (m.tick % 2 === 0 && oppAt(ps, att, h.x, h.y, att) > .9))) tryPass(m, h);
    }
    if (m.tick % 10 === 0) stats(m);
  }
  // Scouting report on team o, from the point of view of team t
  function analyze(m, t) {
    const o = 1 - t, st = m.st, sg = o === 0 ? 1 : -1, out = [];
    if (st.heatN < 5) return out;
    const half = i => (o === 0 ? i < 12 : i > 12);
    const laneCtrl = [0, 0, 0], laneN = [0, 0, 0];
    for (let j = 0; j < 12; j++) for (let i = 0; i < 25; i++) { if (!half(i)) continue; const l = j < 4 ? 0 : j < 8 ? 1 : 2; laneCtrl[l] += Math.max(0, st.heat[j * 25 + i] / st.heatN * sg); laneN[l]++; }
    const lc = laneCtrl.map((v, i) => v / laneN[i]), names = ['Üst kanat', 'Merkez', 'Alt kanat'], loc = ['üst kanatta', 'merkezde', 'alt kanatta'], dat = ['üst kanada', 'merkeze', 'alt kanada'];
    if (!lc.every(Number.isFinite)) return out;
    const weak = lc.indexOf(Math.min(...lc)), strong = lc.indexOf(Math.max(...lc));
    out.push({ k: 'zayif', title: `${names[weak]} zayıf`, body: `Kendi yarılarında kontrolleri ${names[weak].toLowerCase()} için ${Math.round(lc[weak] * 100)}, ${names[strong].toLowerCase()} için ${Math.round(lc[strong] * 100)}.`, fix: weak === 1 ? 'Hatlar arası mesafeyi aç, merkeze Delici ya da Sız talimatı ver.' : `Genişliği artır, bir Delici'yi ${dat[weak]} yerleştir.` });
    const bx = st.blockN[o] ? st.blockX[o] / st.blockN[o] : 0;
    if (bx) out.push({ k: 'blok', title: bx < 30 ? 'Derin blok' : bx > 42 ? 'Yüksek blok' : 'Orta blok', body: `Savunmadayken oyuncuları ortalama Kuyu'larından ${Math.round(bx)} birim uzakta.`, fix: bx < 30 ? 'Arkada yer yok: kısa aktarım, sabır, Mıknatıs ile bloğu kaydır.' : 'Arkalarında boşluk var: uzun aktarım ve Sız talimatı.' });
    const pl = st.pass[o] ? st.passLen[o] / st.pass[o] : 0;
    if (pl) out.push({ k: 'aktarim', title: pl > 22 ? 'Uzun aktarım oynuyorlar' : 'Kısa aktarım oynuyorlar', body: `Aktarımlarının ortalaması ${pl.toFixed(1)} birim, başarı oranı %${Math.round(100 * st.passOk[o] / Math.max(1, st.pass[o]))}.`, fix: pl > 22 ? 'Hatları sıkıştır; uzun hatlar kesmeye daha açık.' : 'Pres yap; kısa hatlarda taşıyanın etrafını boya.' });
    const lost = st.lost[t], li = lost.indexOf(Math.max(...lost));
    if (lost[li] > 1) out.push({ k: 'kayip', title: `Çekirdeği en çok ${loc[li]} kaybediyoruz`, body: `${lost[li]} kesme ya da kopma bu bölgede oldu.`, fix: 'O bölgede risk eşiğini düşür ya da oraya bir Çapa kaydır.' });
    if (st.heatN >= 60) {
      const ot = m.tac[o], C = COUNTER[ot.savunma];
      if (C) out.unshift({ k: 'sistem', title: `Savunma sistemi: ${ot.savunma}`, body: C.why, fix: `Ritim: ${C.ritim} · Boşluk: ${C.bosluk}. ${C.not}` });
      const P = PRESS_COUNTER[ot.presTetik]; if (P) out.splice(1, 0, { k: 'pres', title: `Pres tetikleyicisi: ${ot.presTetik}`, body: P[0], fix: P[1] });
      const RC = RITIM_COUNTER[ot.ritim]; if (RC) out.push({ k: 'ritim', title: `Hücum ritmi: ${ot.ritim}`, body: RC[0], fix: RC[1] });
    }
    const tired = m.ps.filter(p => p.team === o && p.st < .6);
    if (tired.length) out.push({ k: 'yorgun', title: `${tired.length} rakip yorgun`, body: tired.map(p => `${p.name} (%${Math.round(p.st * 100)})`).join(', ') + '. Alanları soluyor.', fix: 'Tempoyu artır, yorgun oyuncunun bölgesine yüklen.' });
    return out;
  }
  function render(ctx, s, o) {
    o = o || {};
    const w = s.w || W, h = s.h || H, cv = ctx.canvas, sc = cv.width / w, ps = s.ps, view = o.view ?? null;
    prep(ps);
    ctx.fillStyle = P; ctx.fillRect(0, 0, cv.width, cv.height);
    if (o.heatArr) {
      for (let j = 0; j < 12; j++) for (let i = 0; i < 25; i++) {
        const k = o.heatArr[j * 25 + i], ak = Math.abs(k); if (ak < .05) continue;
        ctx.globalAlpha = Math.min(.7, ak * .9); ctx.fillStyle = k > 0 ? A : B; ctx.fillRect(i * 4 * sc, j * (H / 12) * sc, 4 * sc - 1, (H / 12) * sc - 1);
      }
      ctx.globalAlpha = 1;
    } else if (o.heat !== false) {
      const cs = o.cell || 2;
      for (let y = 0; y < h; y += cs) for (let x = 0; x < w; x += cs) {
        const k = K(ps, x + cs / 2, y + cs / 2, view), ak = Math.abs(k); if (ak < .08) continue;
        ctx.globalAlpha = Math.min(.6, ak * .62); ctx.fillStyle = k > 0 ? A : B; ctx.fillRect(x * sc, y * sc, cs * sc - 1, cs * sc - 1);
      }
      ctx.globalAlpha = 1;
    }
    ctx.fillStyle = INK;
    for (const kx of (s.kuyu || [0, w])) { ctx.beginPath(); ctx.arc(kx * sc, h / 2 * sc, KR * sc, 0, 7); ctx.fill(); }
    ctx.strokeStyle = INK; ctx.lineWidth = 3; ctx.strokeRect(1.5, 1.5, cv.width - 3, cv.height - 3);
    if (o.lanes) {
      ctx.save(); ctx.setLineDash([6, 10]); ctx.strokeStyle = INK; ctx.globalAlpha = .4; ctx.lineWidth = 2;
      for (const y of [H / 3, 2 * H / 3]) { ctx.beginPath(); ctx.moveTo(0, y * sc); ctx.lineTo(cv.width, y * sc); ctx.stroke(); }
      ctx.beginPath(); ctx.moveTo(cv.width / 2, 0); ctx.lineTo(cv.width / 2, cv.height); ctx.stroke(); ctx.restore();
    }
    if (o.marks) for (const mk of o.marks) {
      const x = mk.x * sc, y = mk.y * sc, col = mk.team === 0 ? A : B; ctx.strokeStyle = col; ctx.fillStyle = col; ctx.lineWidth = 4;
      if (mk.type === 'kesme') { ctx.beginPath(); ctx.moveTo(x - 9, y - 9); ctx.lineTo(x + 9, y + 9); ctx.moveTo(x + 9, y - 9); ctx.lineTo(x - 9, y + 9); ctx.stroke(); }
      else if (mk.type === 'kopma') { ctx.beginPath(); ctx.arc(x, y, 8, 0, 7); ctx.stroke(); }
      else if (mk.type === 'sayi') { ctx.save(); ctx.translate(x, y); ctx.rotate(Math.PI / 4); ctx.fillRect(-10, -10, 20, 20); ctx.restore(); }
    }
    if (!ps.length) return;
    const c = s.core;
    if (o.lines && c && c.holder && !c.flight) {
      const h0 = c.holder, t = h0.team, dir = dirOf(t), tac = (s.tac && s.tac[t]) || TEAMS[t].tac, req = passReq(h0, tac), maxD = 20 + 25 * (tac.mesafe ?? .5);
      for (const q of ps) {
        if (q === h0 || q.team !== t) continue; const d = Math.sqrt(d2(q, h0)); if (d < 6 || d > maxD) continue;
        const ok = lineSafety(ps, h0, q, t, dir) >= req;
        ctx.save(); ctx.strokeStyle = INK; ctx.globalAlpha = ok ? .9 : .4; ctx.lineWidth = ok ? 4 : 2; if (!ok) ctx.setLineDash([10, 10]);
        ctx.beginPath(); ctx.moveTo(h0.x * sc, h0.y * sc); ctx.lineTo(q.x * sc, q.y * sc); ctx.stroke(); ctx.restore();
      }
    }
    if (c && c.flight) {
      ctx.save(); ctx.setLineDash([12, 10]); ctx.strokeStyle = INK; ctx.lineWidth = 3;
      ctx.beginPath(); ctx.moveTo(c.x * sc, c.y * sc); ctx.lineTo(c.flight.to.x * sc, c.flight.to.y * sc); ctx.stroke(); ctx.restore();
    }
    const r = 1.45 * sc;
    for (const p of ps) {
      const hidden = view != null && p.team !== view && hid(p);
      ctx.globalAlpha = hidden ? .3 : 1;
      ctx.beginPath(); ctx.arc(p.x * sc, p.y * sc, r, 0, 7); ctx.fillStyle = p.team === 0 ? A : B; ctx.fill();
      ctx.strokeStyle = P; ctx.lineWidth = 3; ctx.stroke();
      ctx.fillStyle = P; ctx.font = `600 ${Math.round(r * .8)}px "JetBrains Mono", monospace`; ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
      ctx.fillText(ROLES[p.role].k, p.x * sc, p.y * sc + 1);
      ctx.globalAlpha = 1;
    }
    if (c && c.holder && !c.flight) { ctx.strokeStyle = INK; ctx.lineWidth = 4; ctx.beginPath(); ctx.arc(c.holder.x * sc, c.holder.y * sc, r + 7, 0, 7); ctx.stroke(); }
    if (c && !c.flight && c.charge != null) { ctx.strokeStyle = INK; ctx.lineWidth = 5; ctx.beginPath(); ctx.arc(c.x * sc, c.y * sc, 1.3 * sc, -Math.PI / 2, -Math.PI / 2 + c.charge * Math.PI * 2); ctx.stroke(); }
    if (c) { ctx.save(); ctx.translate(c.x * sc, c.y * sc); ctx.rotate(Math.PI / 4); ctx.fillStyle = INK; const q = .7 * sc; ctx.fillRect(-q, -q, 2 * q, 2 * q); ctx.strokeStyle = P; ctx.lineWidth = 3; ctx.strokeRect(-q, -q, 2 * q, 2 * q); ctx.restore(); }
    ctx.textAlign = 'start'; ctx.textBaseline = 'alphabetic';
  }
  function makePlayer(team, role, x, y, at, extra) {
    const a = {}; ATTR.forEach((k, i) => a[k] = (at || [12, 12, 12, 12, 12, 12, 12, 12, 12, 12])[i] ?? 12);
    return Object.assign({ id: Math.random(), team, role, name: role, a, st: 1, x, y, fx: team === 0 ? 1 : -1, fy: 0, ori: 'Dengeli', outF: 1 }, extra || {});
  }
  window.AlanEngine = { ENUMS, FORMATIONS, ZONES, formationPatch, inZone, SHOT_D, KESR, DELAY, W, H, KR, EV, EVRE_SN, ROLES, ORI, TEAMS, ATTR, DEFAULT_TAC, createMatch, step, order, analyze, render, prep, K, infl, clock, makePlayer, colors: { A, B, P, INK } };
})();
