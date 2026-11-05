/* DENEY · Rust motoru (WebAssembly): mac-isci.js'in yerine geçen maç işçisi. Sayfa adresine ?motor=rust ya da localStorage 'alan-motor' = 'rust' ile açılır (isciAdres).
   Motor alan-rs/ (Rust) altından derlenir: alan-v5/alan.wasm. İletiler mac-isci.js ile aynı: 'yeni' ile maç kurulur, 'basla' ve 60 tikte bir 'kare' gider (kare: 174 sayı, kaydet'in biçimi).
   Henüz yok: maç içi taktik değişikliği ('degistir'), anlar ('an'), kafa kartları (karar/kafa listeleri boş gider). */
const ALAN_SURUM_Q = (v => v ? '?v=' + v[1] : '')(/[?&]v=(\w+)/.exec(self.location && self.location.search || ''));
/* seyrek yerleşim (uzak hücumcunun işi 3 tikte bir; motorun varsayılanı): adreste seyrek=0 ise kapalı, eski davranış */
const SEYREK = !/[?&]seyrek=0(&|$)/.test(self.location && self.location.search || '');
let W = null, mem = null, gen = 0, len = 5400, dur = 5400, kosuyor = false, istek = null, buf = null, hazir = null;
/* wasm'da panik geri sarılmaz: örnek bozuk kalır (Rust'taki kilit açık kalır, sonraki her çağrı düşer). Hatadan sonra örnek atılır, sonraki 'yeni' taze örnek kurar. */
function birak() { hazir = null; W = null; mem = null; }
const NF = 174, dek = new TextDecoder(), kod = new TextEncoder();
function yukle() { if (!hazir) hazir = fetch('alan.wasm' + ALAN_SURUM_Q).then(r => { if (!r.ok) throw new Error('alan.wasm yüklenemedi (' + r.status + ')'); return r.arrayBuffer(); }).then(b => WebAssembly.instantiate(b, {})).then(r => { W = r.instance.exports; mem = W.memory; if (W.seyrek) W.seyrek(SEYREK ? 1 : 0); }); return hazir; }
const metin = n => dek.decode(new Uint8Array(mem.buffer, W.metin_ptr(), n));
function yeniBuf() { return { kare: [], olay: [], karar: [], kafa: [], is: [] }; }
function kaydet() { const p = W.kare(); const f = new Float32Array(NF); f.set(new Float32Array(mem.buffer, p, NF)); buf.kare.push([W.tik(), f]); const y = JSON.parse(metin(W.yenilikler())); for (const o of y.olay) buf.olay.push(o); for (const j of y.is) buf.is.push(j); }
let hsT0 = 0;
function gonder(g, son, yari) { const n = performance.now(), ms = hsT0 ? n - hsT0 : 0; hsT0 = n; postMessage({ ms, pd: null, tip: 'kare', gen: g, kare: buf.kare, olay: buf.olay, karar: buf.karar, kafa: buf.kafa, is: buf.is, tick: W.tik(), son: !!son, yari: !!yari }); buf = yeniBuf(); }
/* Döngü beklerken yeni maç gelirse istek not edilir; eski döngü çıkınca yeni maçı o başlatır (eskiden hiç başlamıyordu). */
async function calistir(g) { if (kosuyor) { istek = g; return; } if (!W) return; kosuyor = true; hsT0 = performance.now(); const bas = W.tik();
  try { while (!W.bitti() && W.tik() < Math.min(len, dur)) { W.adim(); if (g !== gen) return; kaydet(); const t = W.tik(); if (t % 20 === 0) { if (t % 60 === 0 || t - bas < 120) gonder(g); await new Promise(r => setTimeout(r, 0)); hsT0 = performance.now(); if (g !== gen) return; } } }
  catch (e) { let t = '?'; try { t = W.tik(); } catch (e2) {} birak(); postMessage({ tip: 'hata', msg: 'Rust motoru durdu (tik ' + t + '): ' + String(e && e.message || e) }); return; }
  finally { kosuyor = false; const r = istek; istek = null; if (r !== null && r !== g && r === gen && W) calistir(r); }
  if (g === gen) { if (W.bitti() || W.tik() >= len) gonder(g, true); else gonder(g, false, true); } }
onmessage = async e => { const d = e.data;
  if (d.tip === 'izle' || d.tip === 'kur' || d.tip === 'paralel') return;
  if (d.tip === 'devam') { if (d.gen !== gen) return; dur = len; if (!kosuyor) calistir(gen); return; }
  if (d.tip === 'yeni') { gen = d.gen; len = d.len || 5400; dur = d.dur || len;
    try { await yukle(); } catch (er) { postMessage({ tip: 'hata', msg: String(er && er.message || er) }); return; }
    let ps;
    try { const b = kod.encode(JSON.stringify(d)), p = W.ayir(b.length); new Uint8Array(mem.buffer, p, b.length).set(b);
      const r = W.kur(p, b.length); W.birak(p, b.length);
      if (r !== 0) { postMessage({ tip: 'hata', msg: 'Rust motoru maçı kuramadı: ' + metin(W.metin_len()) }); return; }
      buf = yeniBuf(); kaydet(); ps = JSON.parse(metin(W.statik())); }
    catch (er) { birak(); postMessage({ tip: 'hata', msg: 'Rust motoru maçı kurarken durdu: ' + String(er && er.message || er) }); return; }
    postMessage({ tip: 'basla', gen, ps }); calistir(gen); return; }
  if (d.tip === 'degistir' || d.tip === 'an') { postMessage({ tip: 'hata', msg: 'Rust motorunda bu henüz yok: ' + (d.tip === 'an' ? 'anlar' : 'maç içinde taktik değişikliği') + '. JS motoruna dönmek için adresten ?motor=rust kaldırılır.' }); return; } };
