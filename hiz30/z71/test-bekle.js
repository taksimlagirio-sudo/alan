// Deney: kayıtlı maçı eski bekleme kuralıyla (taklit) T tikine kadar oynatır, sonra aynı anda taşıyıcının seçeneklerini iki kuralla da tartar.
self.window = self; importScripts('core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js');
importScripts('mac-isci.js');
onmessage = e => { const d = e.data; self.ALAN_BEKLE_TAKLIT = !d.yeniTaklit; self.ALAN_BASKI_ESKI = true; const M = self.AlanMatch; const mm = M.createMatch(d.yeni.tohum, { tac: d.yeni.tac, len: d.yeni.len, diz: d.yeni.diz, devre: d.yeni.devre }); kurUygula(mm, d.yeni);
  const out = []; while (mm.tick < d.T) M.step(mm); const h = mm.holder; if (!h) { postMessage({ yok: true, tick: mm.tick }); return; }
  for (const eski of [true, false]) { self.ALAN_BEKLE_TAKLIT = eski && !d.yeniTaklit; self.ALAN_BASKI_ESKI = eski; const c = M.kopya(mm), hc = c.holder, AD = self.AlanDecide; AD.D._tick = c.tick; const r = AD.decide(hc, c.ps, c.ch, c.r, M.tacEff ? M.tacEff(c, hc.team) : c.tac[hc.team]); out.push({ kural: eski ? 'taklit' : 'maçın kuralı', ad: hc.name, x: hc.x, y: hc.y, aday: r.opts.slice(0, 5).map(o => o.kind + ' ' + (o.v != null ? o.v.toFixed(3) : '–')) }); }
  postMessage({ tick: mm.tick, out }); };
