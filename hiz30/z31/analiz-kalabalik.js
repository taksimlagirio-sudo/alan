// Kalabalık anı incelemesi: kayıtlı maçı oynatır; Liman taşıyıcısı rakip Kuyu'ya yakınken dibinde 3+ savunmacı olan anlarda savunmacıların görevini, rolünü ve yerini döker
importScripts('mac-isci.js');
self.onmessage = e => { const d = e.data; try { kur = d.yeni; len = d.yeni.len; gecmis = (d.degisiklik || []).map(c => ({ tick: c.snapT, t: c.t, tac: c.tac, diz: c.diz, roller: c.roller || null })).sort((a, b) => a.tick - b.tick);
  m = M.createMatch(kur.tohum, { tac: kur.tac, len, diz: kur.diz }); if (kur.oz === 'rastgele') rastgele(m); kurUygula(m, kur); let ci = 0; const out = [], gx = 100;
  while (!m.over && m.tick < len) { while (ci < gecmis.length && gecmis[ci].tick <= m.tick) uygula(m, gecmis[ci++]); M.step(m); const h = m.holder; if (!h || h.team !== 0 || m.tick % 6) continue; if (Math.hypot(gx - h.x, 25 - h.y) > 26) continue;
    const D = m.ps.filter(p => p.team === 1 && p.role !== 'Bekçi'), near = D.filter(p => Math.hypot(p.x - h.x, p.y - h.y) < 8); if (near.length < 3) continue;
    const A = m.ps.filter(p => p.team === 0 && p.role !== 'Bekçi' && p !== h).map(a => ({ ad: a.name, x: +a.x.toFixed(1), y: +a.y.toFixed(1), bos: +Math.min(...D.map(p => Math.hypot(p.x - a.x, p.y - a.y))).toFixed(1) }));
    out.push({ t: m.tick, skor: m.score.join('-'), h: [h.name, +h.x.toFixed(1), +h.y.toFixed(1)], D: D.map(p => [p.name, p.rs, (p.job || '-').slice(0, 28), +p.x.toFixed(1), +p.y.toFixed(1), +Math.hypot(p.x - h.x, p.y - h.y).toFixed(1)]), A, pres: m.tac[1].pres }); if (out.length > 40) break; }
  postMessage({ ok: true, out }); } catch (x) { postMessage({ ok: false, err: x.message + ' ' + String(x.stack).slice(0, 300) }); } };
