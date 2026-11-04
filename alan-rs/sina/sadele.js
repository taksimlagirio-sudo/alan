// Yalnız bugünkü davranış: her self.ALAN_*_ESKI okuması varsayılan değeriyle değiştirilir, ölü dallar silinir.
const { minify } = require(process.env.TERSER || 'terser'); const fs = require('fs');
const [, , GIR, CIK, BAY, ...DOS] = process.argv; /* node sina/sadele.js ../alan-v5 kaynak sina/bayrak-varsayilan.json core match … */
const d = JSON.parse(fs.readFileSync(BAY));
const gd = {}; for (const [k, v] of Object.entries(d)) { if (typeof v !== 'boolean') continue; gd['self.' + k] = v; gd['window.' + k] = v; }
(async () => {
  fs.mkdirSync(CIK, { recursive: true }); for (const f of DOS) {
    let s = fs.readFileSync(`${GIR}/${f}.js`, 'utf8');
    // atamaların sol yanı global_defs'e takılmasın: `self.X_ESKI =` → `self.__ATA_X_ESKI =`
    s = s.replace(/(?:self|window)\.(ALAN_[A-Z0-9_]+_ESKI)\b(?!\s*(?:=(?!=)|\|\|=|&&=|\?\?=))/g, (m, k) => (k in d && typeof d[k] === 'boolean') ? '(' + d[k] + ')' : (k in d ? m : '(false)'));
    try {
      const r = await minify(s, { compress: { global_defs: gd, passes: 3, dead_code: true, conditionals: true, booleans: true, unused: false, evaluate: true, keep_fargs: true, keep_fnames: true, sequences: false, join_vars: false, collapse_vars: false, reduce_vars: false, inline: false }, mangle: false, format: { beautify: true, comments: false, max_line_len: 160 } });
      fs.writeFileSync(`${CIK}/${f}.js`, r.code); console.log(f, 'tamam', r.code.split('\n').length, 'satır');
    } catch (e) { console.log(f, 'HATA', e.message, e.line, e.col); }
  }
})();
