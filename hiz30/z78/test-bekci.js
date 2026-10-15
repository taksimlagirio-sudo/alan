// Deney: kayıtlı maçı T1'e kadar oynatır, sonra her tik Çekirdeği ve savunan Bekçi'yi kaydeder (konum, hedef, görev, menzil), sayı ya da T2'ye kadar.
self.window = self; importScripts('core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js');
importScripts('mac-isci.js');
onmessage = e => { const d = e.data, M = self.AlanMatch, C = self.AlanCore, y = d.yeni; for (const k of Object.keys(d.bayrak || {})) self[k] = d.bayrak[k];
  const mm = M.createMatch(y.tohum, { tac: y.tac, len: y.len, diz: y.diz, devre: y.devre }); kurUygula(mm, y);
  while (mm.tick < d.T1) M.step(mm); for (const k of Object.keys(d.bayrakSonra || {})) self[k] = d.bayrakSonra[k]; const out = [], s0 = mm.score.join('-');
  while (mm.tick < d.T2) { M.step(mm); const b = mm.ball, bk = mm.ps.find(p => p.role === 'Bekçi' && p.team === d.takim);
    out.push({ t: mm.tick, b: b ? [+b.x.toFixed(2), +b.y.toFixed(2), +b.vx.toFixed(2), +b.vy.toFixed(2), b.done || '', b.defl || 0, b.tp ?? b.t] : null, h: mm.holder ? mm.holder.name : '', bk: [+bk.x.toFixed(2), +bk.y.toFixed(2), +(bk.tx ?? 0).toFixed(2), +(bk.ty ?? 0).toFixed(2), bk.job || '', b ? +C.bekR(b, bk).toFixed(2) : null, bk.yanil ? 'aldandı' : ''], ev: mm.events[0] && mm.events[0].text });
    if (mm.score.join('-') !== s0) break; }
  postMessage({ out }); };
