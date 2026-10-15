// Deney: kayıtlı maçı T1'e kadar oynatır, T1–T2 arasında her N tikte savunan takımın görevlerini (job), yerini ve hedefini döndürür.
self.window = self; importScripts('core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js');
importScripts('mac-isci.js');
onmessage = e => { const d = e.data, M = self.AlanMatch, y = d.yeni; const mm = M.createMatch(y.tohum, { tac: y.tac, len: y.len, diz: y.diz, devre: y.devre }); kurUygula(mm, y);
  const out = []; self.ALAN_IZ = d.iz || null; self.ALAN_SAV_ESKI = !!d.eski; self.ALAN_GECIS_ESKI = !!d.gecisEski; self.ALAN_PLAN_ESKI = !!d.planEski; while (mm.tick < d.T1) M.step(mm); self.ALAN_SAV_ESKI = !!d.eskiSonra; self.ALAN_GECIS_ESKI = false; self.ALAN_PLAN_ESKI = false; /* deney: T1'e kadar eski savunma kuralı (zip54), sonra yeni */
  while (mm.tick < d.T2) { M.step(mm); if (mm.tick % (d.N || 10)) continue; const h = mm.holder, b = mm.ball;
    out.push({ t: mm.tick, top: h ? h.name + ' ' + h.x.toFixed(0) + ',' + h.y.toFixed(0) : (b ? 'havada ' + b.x.toFixed(0) + ',' + b.y.toFixed(0) + (mm.fl && mm.fl.q ? ' → ' + mm.fl.q.name : '') : '-'),
      g: mm.ps.filter(p => p.team === d.takim && p.role !== 'Bekçi').map(p => `${p.name} (${p.x.toFixed(0)},${p.y.toFixed(0)})→(${(p.tx ?? 0).toFixed(0)},${(p.ty ?? 0).toFixed(0)}) ${p.job || ''}${p.press ? ' [P]' : ''}`) }); }
  postMessage({ out, iz: self.__iz || null }); };
