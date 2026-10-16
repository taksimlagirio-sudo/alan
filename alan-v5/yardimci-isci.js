// v5 · yardımcı işçi: maç işçisinin kafadaki gönderme denemelerini oynar (paralel.js). Motorun aynısını yükler; denemeyi maç işçisinin çektiği tohumla oynar.
self.window = self;
{ const sb = /[?&]saha=(\d+)x(\d+)/.exec(self.location && self.location.search || ''); if (sb) self.ALAN_SAHA_BOY = [+sb[1], +sb[2]]; }
importScripts('sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'paralel.js');
{ const mesaj = self.AlanParalel.yardimciKur(self.AlanMatch.rng); self.onmessage = e => mesaj(e.data); postMessage('hazir'); }
