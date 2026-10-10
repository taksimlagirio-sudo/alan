// Node için beyin işçi havuzu: alan-v4/beyin-isci.js'i worker_threads içinde, tarayıcıdaki gibi (importScripts, onmessage, postMessage) çalıştırır.
// havuz(n, dir) → { isler(m, jobs): Promise<sonuçlar>, kapat() } · AlanBeyin.oynaAsync'e { isler } olarak verilir.
const { Worker, isMainThread, parentPort, workerData } = require('worker_threads'), fs = require('fs'), path = require('path');
if (!isMainThread) { const dir = workerData.dir; globalThis.window = globalThis; globalThis.self = globalThis;
  let a = 12345; Math.random = () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; };
  globalThis.importScripts = (...fs_) => { for (const f of fs_) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f); };
  globalThis.postMessage = d => parentPort.postMessage(d); parentPort.on('message', d => globalThis.onmessage({ data: d }));
  (0, eval)(fs.readFileSync(path.join(dir, 'beyin-isci.js'), 'utf8')); }
function havuz(n, dir) { const ws = [], wait = new Map(); let id = 0;
  for (let k = 0; k < n; k++) { const w = new Worker(__filename, { workerData: { dir: path.resolve(dir) } }); w.on('message', d => { const f = wait.get(d.id); wait.delete(d.id); f(d.sonuc); }); ws.push(w); }
  return { n, async isler(m, jobs, ayar) { const durum = globalThis.AlanBeyin.paketle(m), parts = ws.map(() => []); // ilk iş ana iş parçacığında (beklerken boş durmasın), diğerleri işçilere
      jobs.forEach((j, k) => { if (k) parts[(k - 1) % n].push(k); });
      const res = new Array(jobs.length), ps = parts.map((ix, w) => ix.length ? new Promise(ok => { const i = ++id; wait.set(i, r => { r.forEach((v, q) => res[ix[q]] = v); ok(); }); ws[w].postMessage({ id: i, durum, isler: ix.map(q => jobs[q]), ayar: ayar || globalThis.AlanBeyin.A }); }) : null);
      if (jobs.length) res[0] = globalThis.AlanBeyin.rollout(m, jobs[0]); await Promise.all(ps); return res; },
    kapat() { ws.forEach(w => w.terminate()); } }; }
module.exports = { havuz };
