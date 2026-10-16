// Maç durumunu (döngüsel bağlı nesne ağı) JSON'a çevirir ve geri kurar. Oyuncuya işaret eden her yer aynı nesneye bağlanır; Map/Set/typed array/rng korunur.
function yaz(kok) { const N = [], id = new Map(), uyari = new Set();
  const enc = v => { if (v === undefined) return { u: 1 }; if (v === null || typeof v === 'boolean' || typeof v === 'string') return v; if (typeof v === 'number') return Number.isFinite(v) ? (Object.is(v, -0) ? { n: '-0' } : v) : { n: String(v) }; if (typeof v === 'function') { if (v.s !== undefined) return { f: v.s }; uyari.add('fonksiyon'); return { u: 1 }; } if (typeof v !== 'object') return { u: 1 };
    if (id.has(v)) return { '@': id.get(v) }; const k = N.length; id.set(v, k); N.push(null);
    let d; if (ArrayBuffer.isView(v)) d = { t: v.constructor.name, d: Array.from(v, enc) }; else if (v instanceof Map) d = { m: [...v].map(([a, b]) => [enc(a), enc(b)]) }; else if (v instanceof Set) d = { s: [...v].map(enc) }; else if (Array.isArray(v)) { d = { a: v.map(enc) }; const ek = Object.keys(v).filter(k => !(String(k >>> 0) === k && (k >>> 0) < v.length)); if (ek.length) { d.p = {}; for (const x of ek) d.p[x] = enc(v[x]); } } else { const pr = Object.getPrototypeOf(v); if (pr !== Object.prototype && pr !== null) uyari.add('sınıf: ' + (pr.constructor && pr.constructor.name)); const o = {}; for (const x of Object.keys(v)) o[x] = enc(v[x]); d = { o }; }
    N[k] = d; return { '@': k }; };
  const r = enc(kok); return { r, N, uyari: [...uyari] }; }
function oku(G, rngYap) { const C = new Array(G.N.length);
  for (let k = 0; k < G.N.length; k++) { const d = G.N[k]; C[k] = d.t ? new globalThis[d.t](d.d.length) : d.m ? new Map() : d.s ? new Set() : d.a ? [] : {}; }
  const dec = v => { if (v === null || typeof v !== 'object') return v; if ('@' in v) return C[v['@']]; if (v.u) return undefined; if ('n' in v) return v.n === '-0' ? -0 : Number(v.n); if ('f' in v) { const f = rngYap(0); f.s = v.f; return f; } return v; };
  for (let k = 0; k < G.N.length; k++) { const d = G.N[k], c = C[k]; if (d.t) d.d.forEach((x, i) => c[i] = dec(x)); else if (d.m) for (const [a, b] of d.m) c.set(dec(a), dec(b)); else if (d.s) for (const a of d.s) c.add(dec(a)); else if (d.a) { d.a.forEach((x, i) => c[i] = dec(x)); if (d.p) for (const x of Object.keys(d.p)) c[x] = dec(d.p[x]); } else for (const x of Object.keys(d.o)) c[x] = dec(d.o[x]); }
  return dec(G.r); }
module.exports = { yaz, oku };
