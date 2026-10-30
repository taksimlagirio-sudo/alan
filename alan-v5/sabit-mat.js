// v5 · Her tarayıcıda aynı sayı: motorun kullandığı Math fonksiyonları (exp, log, pow, sin, cos, atan, atan2, acos, tanh, hypot) yalnız toplama, çarpma, bölme ve karekökle hesaplanır.
// Neden: iPad'deki Safari (JavaScriptCore) ile Chrome/node (V8) bu fonksiyonlarda son basamakta farklı sonuç verir (çağrıların %3–33'ünde). Motor bu farkı birkaç saniyede büyütür: aynı tohum iki cihazda iki ayrı maç olur.
// + − × ÷ ve √ IEEE 754'te her yerde bit bit aynıdır; buradaki formüller fdlibm'in (Sun) algoritmaları. Yalnız işçilerde yüklenir (maç sayfasının çizimi etkilenmez). Deney: self.ALAN_MAT_ESKI = true → tarayıcının kendi Math'ı.
(function () {
  if (self.ALAN_MAT_ESKI) return;
  const F = new Float64Array(1), U = new Uint32Array(F.buffer); /* little-endian: U[1] üst kelime (üs) */
  const iki = k => { F[0] = 0; U[0] = 0; U[1] = (k + 1023) << 20; return F[0]; }; /* 2^k, -1022 ≤ k ≤ 1023, tam */
  const sk = (y, k) => k > 1023 ? y * iki(1023) * iki(k - 1023) : k < -1022 ? y * iki(-1022) * iki(k + 1022) : y * iki(k);
  const yuv = x => x >= 0 ? Math.floor(x + .5) : Math.ceil(x - .5); /* en yakın tamsayı, yarımda sıfırdan uzağa (Math.round tam işlem) */
  const LN2H = 6.93147180369123816490e-01, LN2L = 1.90821492927058770002e-10, ILN2 = 1.44269504088896338700e+00;
  const P1 = 1.66666666666666019037e-01, P2 = -2.77777777770155933842e-03, P3 = 6.61375632143793436117e-05, P4 = -1.65339022054652515390e-06, P5 = 4.13813679705723846039e-08;
  function exp(x) { if (x !== x) return x; if (x > 709.782712893384) return Infinity; if (x < -745.1332191019411) return 0; if (Math.abs(x) < 3.725290298461914e-9) return 1 + x;
    const k = Math.abs(x) < .34657359027997264 ? 0 : yuv(x * ILN2), hi = x - k * LN2H, lo = k * LN2L, r = hi - lo, t = r * r, c = r - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5)))), y = 1 - ((lo - (r * c) / (2 - c)) - hi); return k === 0 ? y : sk(y, k); }
  const Lg1 = 6.666666666666735130e-01, Lg2 = 3.999999999940941908e-01, Lg3 = 2.857142874366239149e-01, Lg4 = 2.222219843214978396e-01, Lg5 = 1.818357216161805012e-01, Lg6 = 1.531383769920937332e-01, Lg7 = 1.479819860511658591e-01;
  function log(x) { if (x !== x || x < 0) return NaN; if (x === 0) return -Infinity; if (x === Infinity) return x; let k = 0; F[0] = x; let hx = U[1];
    if (hx < 0x00100000) { F[0] = x * 18014398509481984; hx = U[1]; k -= 54; } k += (hx >>> 20) - 1023; hx &= 0x000fffff; const i = (hx + 0x95f64) & 0x100000; U[1] = hx | (i ^ 0x3ff00000); k += i >>> 20; const f = F[0] - 1;
    const hf = .5 * f * f, s = f / (2 + f), z = s * s, w = z * z, R = z * (Lg1 + w * (Lg3 + w * (Lg5 + w * Lg7))) + w * (Lg2 + w * (Lg4 + w * Lg6)); return k * LN2H - ((hf - (s * (hf + R) + k * LN2L)) - f); }
  function pow(x, y) { if (y === 0) return 1; if (x !== x || y !== y) return NaN; if (Number.isInteger(y) && Math.abs(y) <= 64) { let n = Math.abs(y), b = x, r = 1; while (n) { if (n & 1) r *= b; b *= b; n >>= 1; } return y < 0 ? 1 / r : r; }
    if (x === 0) return y > 0 ? 0 : Infinity; if (x < 0) return NaN; if (x === Infinity) return y > 0 ? Infinity : 0; return exp(y * log(x)); }
  const S1 = -1.66666666666666324348e-01, S2 = 8.33333333332248946124e-03, S3 = -1.98412698298579493134e-04, S4 = 2.75573137070700676789e-06, S5 = -2.50507602534068634195e-08, S6 = 1.58969099521155010221e-10;
  const C1 = 4.16666666666666019037e-02, C2 = -1.38888888888741095749e-03, C3 = 2.48015872894767294178e-05, C4 = -2.75573143513906633035e-07, C5 = 2.08757232129817482790e-09, C6 = -1.13596475577881948265e-11;
  const ks = (x, y) => { const z = x * x, v = z * x, r = S2 + z * (S3 + z * (S4 + z * (S5 + z * S6))); return x - ((z * (.5 * y - v * r) - y) - v * S1); };
  const kc = (x, y) => { const z = x * x, r = z * (C1 + z * (C2 + z * (C3 + z * (C4 + z * (C5 + z * C6))))), hz = .5 * z, w = 1 - hz; return w + (((1 - w) - hz) + (z * r - x * y)); };
  const IPIO2 = 6.36619772367581382433e-01, PIO2_1 = 1.57079632673412561417e+00, PIO2_1T = 6.07710050650619224932e-11, PIO2_2 = 6.07710050630396597660e-11, PIO2_2T = 2.02226624879595063154e-21;
  function ind(x) { const n = yuv(x * IPIO2); let r = x - n * PIO2_1, w = n * PIO2_1T; let y0 = r - w; if (Math.abs(n) > 32) { const t = r; w = n * PIO2_2; r = t - w; w = n * PIO2_2T - ((t - r) - w); y0 = r - w; } return [n, y0, (r - y0) - w]; } /* x = n·π/2 + (y0 + y1) */
  function sin(x) { if (x !== x || x === Infinity || x === -Infinity) return NaN; if (Math.abs(x) < .7853981633974483) return Math.abs(x) < 7.450580596923828e-9 ? x : ks(x, 0); const [n, a, b] = ind(x); switch (n & 3) { case 0: return ks(a, b); case 1: return kc(a, b); case 2: return -ks(a, b); default: return -kc(a, b); } }
  function cos(x) { if (x !== x || x === Infinity || x === -Infinity) return NaN; if (Math.abs(x) < .7853981633974483) return kc(x, 0); const [n, a, b] = ind(x); switch (n & 3) { case 0: return kc(a, b); case 1: return -ks(a, b); case 2: return -kc(a, b); default: return ks(a, b); } }
  const AH = [4.63647609000806093515e-01, 7.85398163397448278999e-01, 9.82793723247329054082e-01, 1.57079632679489655800e+00], AL = [2.26987774529616870924e-17, 3.06161699786838301793e-17, 1.39033110312309984516e-17, 6.12323399573676603587e-17];
  const T = [3.33333333333329318027e-01, -1.99999999998764832476e-01, 1.42857142725034663711e-01, -1.11111104054623557880e-01, 9.09088713343650656196e-02, -7.69187620504482999495e-02, 6.66107313738753120669e-02, -5.83357013379057348645e-02, 4.97687799461593236017e-02, -3.65315727442169155270e-02, 1.62858201153657823623e-02];
  function atan(x) { if (x !== x) return x; const ng = x < 0; let a = Math.abs(x), id; if (a >= 7.378697629483821e19) return ng ? -AH[3] - AL[3] : AH[3] + AL[3];
    if (a < .4375) { if (a < 1.862645149230957e-9) return x; id = -1; } else if (a < 1.1875) { if (a < .6875) { id = 0; a = (2 * a - 1) / (2 + a); } else { id = 1; a = (a - 1) / (a + 1); } } else if (a < 2.4375) { id = 2; a = (a - 1.5) / (1 + 1.5 * a); } else { id = 3; a = -1 / a; }
    const z = a * a, w = z * z, s1 = z * (T[0] + w * (T[2] + w * (T[4] + w * (T[6] + w * (T[8] + w * T[10]))))), s2 = w * (T[1] + w * (T[3] + w * (T[5] + w * (T[7] + w * T[9])))); if (id < 0) return x - x * (s1 + s2);
    const r = AH[id] - ((a * (s1 + s2) - AL[id]) - a); return ng ? -r : r; }
  const PI = 3.1415926535897931160E+00, PI_LO = 1.2246467991473531772E-16;
  function atan2(y, x) { if (x !== x || y !== y) return NaN; if (x === 1) return atan(y); if (y === 0) { if (x > 0 || (x === 0 && 1 / x > 0)) return y; return 1 / y > 0 ? PI : -PI; } if (x === 0) return y > 0 ? PI / 2 : -PI / 2;
    if (x === Infinity || x === -Infinity || y === Infinity || y === -Infinity) { const ix = x === Infinity ? 1 : x === -Infinity ? -1 : 0, iy = y === Infinity ? 1 : y === -Infinity ? -1 : 0; if (ix && iy) return iy * (ix > 0 ? PI / 4 : 3 * PI / 4); if (ix) return ix > 0 ? (y > 0 ? 0 : -0) : (y > 0 ? PI : -PI); return iy * PI / 2; }
    const z = atan(Math.abs(y / x)); if (x > 0) return y > 0 ? z : -z; return y > 0 ? PI - (z - PI_LO) : -(PI - (z - PI_LO)); }
  const acos = x => x !== x || x > 1 || x < -1 ? NaN : atan2(Math.sqrt((1 - x) * (1 + x)), x);
  function tanh(x) { if (x !== x) return x; if (x > 22) return 1; if (x < -22) return -1; if (Math.abs(x) < 1e-8) return x; const e = exp(2 * x); return (e - 1) / (e + 1); }
  function hypotN() { let s = 0; for (let i = 0; i < arguments.length; i++) { const v = +arguments[i]; if (v === Infinity || v === -Infinity) return Infinity; s += v * v; } return Math.sqrt(s); }
  const H2 = !self.ALAN_HYPOT2_ESKI; function hypot(a, b) { if (!H2 || arguments.length !== 2) return hypotN.apply(null, arguments); a = +a; b = +b; if (a === Infinity || a === -Infinity || b === Infinity || b === -Infinity) return Infinity; return Math.sqrt(a * a + b * b); } /* v5 · hız: iki argümanlı çağrı (motorda hemen hepsi) döngüsüz, aynı işlemler aynı sırayla (0 + a² = a² tam): sonuç bit bit aynı. arguments döngüsü tarayıcıda iyileştirilemiyordu (Chromium'da sürenin %3,7'si). Deney: self.ALAN_HYPOT2_ESKI */
  Object.assign(Math, { exp, log, pow, sin, cos, atan, atan2, acos, tanh, hypot }); self.ALAN_MAT = true;
})();
