// Beyin işçisi: ileri bakış denemelerini paralel çalıştırır. Tarayıcıda Web Worker, Node'da worker_threads (tools/v4-paralel.js) olarak çalışır.
// Mesaj: { id, durum (AlanBeyin.paketle), isler, ayar } → { id, sonuc: [her iş için değer] }
self.window = self;
importScripts('core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'beyin.js', 'vs-table.js', 'pas-table.js');
onmessage = e => { const { id, durum, isler, ayar } = e.data; try { const B = self.AlanBeyin; if (ayar) Object.assign(B.A, ayar);
  const m = B.ac(durum); postMessage({ id, sonuc: isler.map(j => B.rollout(m, j)) }); } catch (err) { postMessage({ id, hata: String(err && err.stack || err) }); } };
