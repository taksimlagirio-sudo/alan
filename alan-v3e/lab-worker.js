// v3e laboratuvar iş parçacığı: tek bir maçı tam beyinle oynatır, özetini döndürür.
self.window = self;
const V = '?v=' + (self.location.search.match(/v=(\w+)/) || [, '1'])[1]; importScripts('core.js' + V, 'engine.js' + V);
function rng(seed) { let a = seed >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
self.onmessage = e => { const q = e.data; try { const t0 = Date.now(), r = self.AlanV3e.play(q.cfg, rng(q.seed)); self.postMessage({ id: q.id, cell: q.cell, k: q.k, swap: q.swap, ok: true, score: r.score, S: r.S, ms: Date.now() - t0 }); } catch (err) { self.postMessage({ id: q.id, cell: q.cell, k: q.k, ok: false, msg: String(err && err.message || err) }); } };
