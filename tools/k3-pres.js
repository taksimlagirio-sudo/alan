// Ölçüm 3 · Pres kararının sonucu (kopyalı). Maç oynanır; bir savunmacı ilk presçi olarak basmaya başladığı anda maç kopyalanır.
// Kollar: A kural (değişiklik yok) · B bu presçi basmaz (yerine görevini sürdürür; başka biri basabilir) · C o takımda kimse basmaz (savunma yerinde, oyalar).
// Her kol R zarla UFUK tik oynatılır. Ölçülen: hücum eden takımın göndermesi ve sayısı, savunmanın topu kazanması, topun ilerlemesi.
// Kullanım: node tools/k3-pres.js <motor> <senaryo> <çıktı.jsonl> [ARALIK=300] [R=2] [UFUK=180]
const fs = require('fs'); const { kur } = require('./kurulum'); const [dir, sS, out, aS, rS, uS] = process.argv.slice(2), ARALIK = +aS || 300, R = +rS || 2, UFUK = +uS || 180;
const { M, m, adim, g } = kur(dir, +sS, 0, { 'shape.js': [
  ["cand = defs.filter(p => !up.has(p)", "cand = defs.filter(p => !(self.__pyasak && self.__pyasak.has(p.id)) && !up.has(p)"],
  ["const q = defs.find(d => d.id === id); if (q) press.push(q);", "const q = defs.find(d => d.id === id); if (q && !(self.__pyasak && self.__pyasak.has(q.id))) press.push(q);"]] });
const W = (g.ALAN_SAHA && g.ALAN_SAHA.W) || 120, ileri = (t, x) => t === 0 ? x : W - x, presJob = j => /^pres|^fake: |^kenara itiyor/.test(j || '');
function kol(snap, yasak, r, at) { const c = M.kopya(snap); c.r.s = (r * 2654435761 + 7777) >>> 0; g.__pyasak = yasak; const s0 = c.st.shot.slice(), g0 = c.score.slice();
  let kazandi = 0, maxI = c.holder ? ileri(at, c.holder.x) : 0, ilkKaz = null;
  for (let t = 0; t < UFUK && !c.over; t++) { M.step(c); const h = c.holder; if (h && h.team !== at && ilkKaz == null) { ilkKaz = t; kazandi = 1; } const bx = h ? h.x : c.ball ? c.ball.x : null; if (bx != null && (!h || h.team === at)) maxI = Math.max(maxI, ileri(at, bx)); }
  g.__pyasak = null; return { sut: c.st.shot[at] - s0[at], sayi: c.score[at] - g0[at], yenen: c.score[1 - at] - g0[1 - at], kazandi, ilkKaz, ilerle: maxI }; }
const onceki = new Map(); let son = -1e9; fs.writeFileSync(out, '');
while (!m.over && m.tick < (+process.env.LEN || m.len || 7200)) { adim(); for (const p of m.ps) { const j = presJob(p.job), o = onceki.get(p.id); onceki.set(p.id, j); if (!j || o || p.role === 'Bekçi') continue;
    const h = m.holder; if (!h || h.team === p.team || m.tick - son < ARALIK) continue; const pres = m.ps.filter(q => q.team === p.team && presJob(q.job)); if (pres.length !== 1) continue; son = m.tick;
    const at = h.team, snap = M.kopya(m); let hat = 1e9; for (const q of m.ps) if (q.team === p.team && q !== p && q.role !== 'Bekçi') hat = Math.min(hat, ileri(p.team, q.x));
    const rec = { sc: +sS, t: m.tick, d0: +Math.hypot(p.x - h.x, p.y - h.y).toFixed(1), onde: +(ileri(p.team, p.x) - hat).toFixed(1), bolge: +(ileri(at, h.x) / W).toFixed(2), kol: {} };
    const tum = new Set(m.ps.filter(q => q.team === p.team).map(q => q.id));
    rec.kol.A = Array.from({ length: R }, (_, r) => kol(snap, null, r, at)); rec.kol.B = Array.from({ length: R }, (_, r) => kol(snap, new Set([p.id]), r, at)); rec.kol.C = Array.from({ length: R }, (_, r) => kol(snap, tum, r, at));
    fs.appendFileSync(out, JSON.stringify(rec) + '\n'); } }
console.log(JSON.stringify({ sc: +sS, skor: m.score }));
