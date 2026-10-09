// v3g · Önde koşan maç iş parçacığı. Maç izlemenin önünde, olabildiğince hızlı oynanır; her tik bir kare olarak sayfaya gönderilir, sayfa kareleri istediği hızda oynatır.
// Her 60 tikte maçın tam kopyası (AlanMatch.cloneMatch) saklanır. Taktik T anında değişince: T'den önceki en yakın kopya geri yüklenir, aynı taktikle T'ye kadar yeniden oynanır (belirlenimci: aynı sonuç), yeni taktik uygulanır, T'den sonrası atılıp yeniden hesaplanır.
self.window = self;
const V = '?v=' + (self.location.search.match(/v=(\w+)/) || [, '1'])[1]; importScripts('core.js' + V, 'shot-table.js' + V, 'decide.js' + V, 'match.js' + V, 'shape.js' + V, 'vs-table.js' + V, 'vs-fiz.js' + V);
const M = self.AlanMatch, CP = 60;
let m = null, gen = 0, cps = new Map(), batch = [], lastEv = null, lastDec = null, busy = false;
function frame() { const h = m.holder, b = m.ball, ev = m.events.length ? m.events[0] : null, f = { t: m.tick, hi: h ? m.ps.indexOf(h) : -1, ch: m.ch, sc: [m.score[0], m.score[1]], over: m.over,
    ps: m.ps.map(p => [p.x, p.y, p.vx, p.vy, p.tx, p.ty, p.ca, p.sprint ? 1 : 0, p.rh || '', p.rs || '', p.job || '']), b: b ? [b.x, b.y, b.vx, b.vy, b.alive ? 1 : 0, b.team] : null };
  if (ev !== lastEv) { lastEv = ev; f.ev = m.events.slice(0, 40); }
  if (m.lastDec !== lastDec) { lastDec = m.lastDec; f.dec = m.lastDec || null; }
  if (m.tick % 15 === 0 || m.over) f.st = JSON.parse(JSON.stringify(m.st));
  return f; }
function statics() { return m.ps.map(p => ({ name: p.name, team: p.team, role: p.role, i: p.i, id: p.id, R: p.R, D: p.D, a: p.a, slot: p.slot })); }
function flush() { if (batch.length) { self.postMessage({ type: 'frames', gen, frames: batch }); batch = []; } }
function run(my) { if (my !== gen || !m) return; const t0 = Date.now();
  try { while (!m.over && Date.now() - t0 < 40) { M.step(m); if (m.tick % CP === 0) cps.set(m.tick, M.cloneMatch(m)); batch.push(frame()); if (batch.length >= 30) flush(); } }
  catch (e) { flush(); self.postMessage({ type: 'error', gen, msg: (e && e.message || String(e)) + ' · ' + String(e && e.stack || '').split('\n').slice(0, 3).join(' | '), tick: m.tick }); return; }
  flush(); if (m.over) { self.postMessage({ type: 'done', gen, tick: m.tick }); return; } setTimeout(() => run(my), 0); }
self.onmessage = async e => { await self.ALAN_VS_CHECK; /* tablo damgası kontrol edilmeden maç başlamaz */ const q = e.data;
  if (q.type === 'new') { gen++; m = M.createMatch(q.seed, { tac: q.tac, len: q.len }); cps = new Map([[0, M.cloneMatch(m)]]); lastEv = lastDec = null; batch = [];
    self.postMessage({ type: 'start', gen, statics: statics(), seed: q.seed, len: m.len, tac: m.tac }); batch.push(frame()); run(gen); }
  else if (q.type === 'tac' && m) { gen++; const T = q.tick; let k = 0; for (const t of cps.keys()) if (t <= T && t > k) k = t;
    m = M.cloneMatch(cps.get(k)); while (m.tick < T && !m.over) M.step(m);
    for (const t of [...cps.keys()]) if (t > T) cps.delete(t);
    for (const t of [0, 1]) m.tac[t] = { ...m.tac[t], ...q.tac[t] }; cps.set(m.tick, M.cloneMatch(m)); /* değişiklik anı da kopya noktası: sonraki geri yüklemeler bu taktikle başlar */ lastEv = lastDec = null; batch = [];
    self.postMessage({ type: 'rewind', gen, tick: m.tick, tac: m.tac }); run(gen); } };
