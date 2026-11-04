// sabit-mat.js'den sınama vektörleri: her işlev için rastgele girdiler ve çıktıların bitleri (16'lık)
global.self = global; require(process.argv[2] + '/sabit-mat.js');
const F = new Float64Array(1), U = new BigUint64Array(F.buffer), b = v => { F[0] = v; return U[0].toString(16); };
let s = 12345; const r = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const out = [];
const gir = [];
for (let i = 0; i < 20000; i++) { const k = r(); gir.push(k < .3 ? (r() - .5) * 20 : k < .6 ? (r() - .5) * 2000 : k < .8 ? (r() - .5) * 1e-6 : (r() - .5) * 1e8); }
gir.push(0, -0, 1, -1, .5, -.5, 709.78, -745.2, 1e-300, 5e-324, Infinity, -Infinity, NaN);
for (const x of gir) { const y = gir[(out.length * 7) % gir.length]; out.push([b(x), b(y), b(Math.exp(x)), b(Math.log(Math.abs(x))), b(Math.pow(Math.abs(x), y % 7)), b(Math.pow(x, Math.round(y) % 9)), b(Math.sin(x)), b(Math.cos(x)), b(Math.atan(x)), b(Math.atan2(x, y)), b(Math.acos(Math.max(-1, Math.min(1, x / 10)))), b(Math.tanh(x)), b(Math.hypot(x, y)), b(Math.round(x)), b(Math.max(x, y)), b(Math.min(x, y))].join(' ')); }
require('fs').writeFileSync(process.argv[3], out.join('\n') + '\n');
