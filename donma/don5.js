// Sayfanın kurduğu maç (varsayılan Liman – Demir Blok), işçideki kurulumla. Herkesin hareketsiz kaldığı anları ve yavaş adımları arar.
const path = require('path'), fs = require('fs'); const [dir, sS] = process.argv.slice(2), seed = +sS;
const { load } = require(path.resolve(__dirname, '../tools/load')); const g = load(dir, seed); (0, eval)(fs.readFileSync(path.join(dir, 'kadro.js'), 'utf8'));
const M = g.AlanMatch, K = g.ALAN_KADRO, R = K.RAKIP.find(r => r.ad === 'Demir Blok'), ids = ['aras', 'deniz', 'mert', 'ece', 'cansu', 'kaan', 'tuna'];
const c = { tac: [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli' }, R.tac], diz: [{ sav: '2-2-2', huc: '2-1-3' }, R.diz], oyuncular: [ids.map(id => { const p = K.KADRO.find(k => k.id === id); return { ad: p.ad, a: p.a }; }), R.oyuncular.map(([ad, a]) => ({ ad, a }))], roller: [null, R.roller] };
const m = M.createMatch(seed, { tac: c.tac, len: 5400, diz: c.diz });
for (const t of [0, 1]) { const A = c.oyuncular[t]; for (const p of m.ps) if (p.team === t && A[p.i]) { p.name = A[p.i].ad; p.a = { ...p.a, ...A[p.i].a }; } }
M.assignRoles(m, c.roller); for (const p of m.ps) { const r = c.roller[p.team] && c.roller[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } for (const t of [0, 1]) if (c.diz[t]) M.dizUygula(m, t, c.diz[t]);
let still = 0, maxStill = 0, stillAt = -1, maxStep = 0, stepAt = -1, slow = 0; const prev = m.ps.map(p => [p.x, p.y]); const t0 = Date.now(), olay = [];
const STOP = +process.argv[4]; while (m.tick < STOP && !m.over && m.tick < 5400) { const a = Date.now(); M.step(m); const d = Date.now() - a; if (d > maxStep) { maxStep = d; stepAt = m.tick; } if (d > 500) slow++;
  let mv = 0; m.ps.forEach((p, k) => { mv = Math.max(mv, Math.hypot(p.x - prev[k][0], p.y - prev[k][1])); prev[k] = [p.x, p.y]; });
  if (mv < 0.01) { still++; if (still > maxStill) { maxStill = still; stillAt = m.tick; } } else still = 0; }
console.log(`tohum ${seed} · skor ${m.score} · süre ${((Date.now() - t0) / 1000).toFixed(0)} sn · en uzun hareketsiz ${maxStill} tik (tik ${stillAt}) · en yavaş adım ${maxStep} ms (tik ${stepAt}) · 0,5 sn üstü adım ${slow}`);
const AD = g.AlanDecide, h = m.holder, t = h.team, tE = M.tacEff ? M.tacEff(m, t) : null; console.log('tacEff var mı', !!M.tacEff, 'macZ(30)', tE && tE.macZ && tE.macZ(30).toFixed(4), 'macZ(10)', tE && tE.macZ && tE.macZ(10).toFixed(4), 'macZ(60)', tE && tE.macZ && tE.macZ(60).toFixed(4));
const r0 = m.r; const sh = o => o.slice(0, 6).map(o => `${o.kind}${o.shot ? ' ' + o.shot : ''}${o.q ? '→' + (o.q.name || o.q) : ''} v=${(o.v || 0).toFixed(3)} z=${(o.zaman || 0).toFixed(3)} sonra=${o.sonra || ''}`).join(' | ');
const cl = () => { const s = structuredClone({ ps: m.ps }); return s.ps; };
let r = AD.decide(h, m.ps, m.ch, m.r, tE); console.log('KARAR:', sh(r.opts));
const W2 = AD.predictWorld(m.ps, t, h, 30), h2 = W2[m.ps.indexOf(h)] || h; AD.D._noHold = true; const r2 = AD.decide(h2, W2, 1, m.r, tE); AD.D._noHold = false; console.log('30 TİK SONRA DÜNYA:', sh(r2.opts));
console.log('hareket farkı', W2.map((p, k) => Math.hypot(p.x - m.ps[k].x, p.y - m.ps[k].y).toFixed(2)).join(' '));
AD.D._noHold = true; const r3 = AD.decide(h, m.ps, m.ch, m.r, tE); AD.D._noHold = false; console.log('ŞİMDİ, bekleme hesabı kapalı:', sh(r3.opts));
