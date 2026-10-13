// Yeni taktik sistemlerinin etkisi (zip33+): tek maç, CFG ortam değişkeniyle Liman'a (takım 0) ayar verilir; ölçüler JSON.
// CFG örneği: {"tl":{"5":{"huc":{"kosu":1}}},"tac":{"tuzak":"kenar","evre":{"sav.kurma":{"blok":"Yüksek"}},"kosul":[{"ne":"esit","ayar":{"blok":"Yüksek"}}],"plan":{"taraf":"sol"}}}
// Ölçüler: oyuncu başına (Liman) top bizdeyken taşıyıcıya göre ileri konum ve mesafe, koşu yolu; top rakipteyken pres görevi oranı, tuttuğu rakibe mesafe; ilişki etiketleri; evre süreleri ve evre başına presçi; koşul etkin oranı; pas hedefinin kanadı.
// Kullanım: CFG='…' node tools/v5-etki.js <motor> <rakip 0|1|2> <tohum> [tik=2700]
const fs = require('fs'), path = require('path'); const { load } = require('./load'); const [dir, rS, sS, lS] = process.argv.slice(2), rk = +rS, seed = +sS, LEN = +lS || 2700, CFG = JSON.parse(process.env.CFG || '{}');
const g = load(dir, seed, { 'match.js': ['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'] }); (0, eval)(fs.readFileSync(path.join(dir, 'kadro.js'), 'utf8')); const M = g.AlanMatch, K = g.ALAN_KADRO, R = K.RAKIP[rk];
const ids = ['aras', 'deniz', 'mert', 'ece', 'cansu', 'kaan', 'tuna'], liman = ids.map(id => K.KADRO.find(k => k.id === id));
const LT = { sistem: 'Alan', pres: 1, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli', siddet: .5, ...(CFG.tac || {}) };
const m = M.createMatch(seed * 7919 + rk, { len: 5400, tac: [LT, R.tac], diz: [{ sav: '2-2-2', huc: '2-1-3' }, R.diz] });
for (const p of m.ps) { if (p.team === 0) { const o = liman[p.i]; if (o) { p.name = o.ad; p.a = { ...p.a, ...(o.a || o) }; } } else { const o = R.oyuncular[p.i]; if (o) { p.name = o[0]; p.a = { ...p.a, ...o[1] }; } } }
M.assignRoles(m, [null, R.roller]); for (const t of [0, 1]) M.dizUygula(m, t, t ? R.diz : { sav: '2-2-2', huc: '2-1-3' });
for (const [i, v] of Object.entries(CFG.tl || {})) { const p = m.ps.find(q => q.team === 0 && q.i === +i); if (!p) continue; p.tl = p.tl || {}; for (const [ev, o] of Object.entries(v)) p.tl[ev] = { ...(p.tl[ev] || {}), ...o }; }
const P = {}, ev = { 0: {}, 1: {} }, evPres = {}, J = { doldur: 0, tuzak: 0 }; let kosul = 0, nT = 0; const pasY = { sol: 0, sag: 0 }; let fl0 = null; const prev = {};
const yakin = (q, D) => { let b = 1e9; for (const o of D) b = Math.min(b, Math.hypot(o.x - q.x, o.y - q.y)); return b; };
while (m.tick < LEN && !m.over) { M.step(m); nT++; const h = m.holder, E = h || m.ball; for (const t of [0, 1]) { const k = M.evreBul(m, t); ev[t][k] = (ev[t][k] || 0) + 1; }
  if (m.kosulAkt && m.kosulAkt[0] && m.kosulAkt[0].length) kosul++;
  for (const p of m.ps) { if (p.team !== 0 || p.role === 'Bekçi') continue; const S = P[p.i] = P[p.i] || { n1: 0, ileri: 0, mes: 0, yol: 0, n2: 0, pres: 0, mk: 0, mkN: 0 }; const pr = prev[p.i]; if (pr) S.yol += Math.hypot(p.x - pr[0], p.y - pr[1]); prev[p.i] = [p.x, p.y];
    const j = typeof p.job === 'string' ? p.job : ''; if (j.includes('doldurma')) J.doldur++; if (j.includes('tuzağı')) J.tuzak++;
    if (h && h.team === 0 && p !== h) { S.n1++; S.ileri += p.x - h.x; S.mes += Math.hypot(p.x - h.x, p.y - h.y); }
    if (h && h.team === 1) { S.n2++; if (j.startsWith('pres')) S.pres++; const kimi = p.tl && p.tl.sav && p.tl.sav.kimi; if (kimi != null) { const o = m.ps.find(q => q.team === 1 && q.i === kimi); if (o) { S.mk += Math.hypot(o.x - p.x, o.y - p.y); S.mkN++; } } } }
  if (h && h.team === 1) { const k = M.evreBul(m, 0); const pj = m.ps.filter(p => p.team === 0 && typeof p.job === 'string' && p.job.startsWith('pres')).length; const e = evPres[k] = evPres[k] || [0, 0]; e[0] += pj; e[1]++; }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; const o = m.fl.o, b = m.ball; if (o && b && b.from && b.from.team === 0 && o.kind !== 'gönder' && o.to) { if (o.to.y < 25) pasY.sol++; else pasY.sag++; } } }
const r2 = x => +x.toFixed(2), out = { rk, seed, cfg: CFG, skor: m.score, oyuncu: {}, evre: {}, evPres: {}, J, kosul: r2(kosul / nT), pasY, pas: m.st.pass[0], duel: m.st.duel[0], kapma: m.st.steal[0] };
for (const [i, S] of Object.entries(P)) out.oyuncu[i] = { ileri: r2(S.ileri / Math.max(1, S.n1)), mes: r2(S.mes / Math.max(1, S.n1)), yol: r2(S.yol), pres: r2(S.pres / Math.max(1, S.n2)), mk: S.mkN ? r2(S.mk / S.mkN) : null };
for (const [k, v] of Object.entries(ev[0])) out.evre[k] = r2(v / nT); for (const [k, v] of Object.entries(evPres)) out.evPres[k] = r2(v[0] / v[1]);
console.log(JSON.stringify(out));
