// Motoru (mac-isci.js ile birlikte) node'a yükler. Dışarıya: M (AlanMatch), kurulumla maç kurma (işçinin kendi kurUygula'sı), işçinin m'si.
const fs = require('fs'), path = require('path');
module.exports = function yukle(dir) {
  globalThis.self = globalThis; globalThis.window = globalThis; globalThis.location = { search: '' }; globalThis.postMessage = () => {};
  globalThis.importScripts = (...F) => { for (const f of F) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f); };
  const isci = fs.readFileSync(path.join(dir, 'mac-isci.js'), 'utf8');
  (0, eval)(isci + '\nself.__isci = { kur: (d) => { kur = d; const mm = M.createMatch(d.tohum, { tac: d.tac, len: d.len, diz: d.diz, devre: d.devre ?? false }); if (d.oz === "rastgele") rastgele(mm); kurUygula(mm, d); return mm; } };');
  return { M: self.AlanMatch, kurMac: self.__isci.kur };
};
