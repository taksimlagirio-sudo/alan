/* Rust motoru (WebAssembly): sitenin ana maç işçisi (mac-isci.js'in yerine). JS motoruna dönmek için sayfa adresine ?motor=js ya da localStorage 'alan-motor' = 'js' (isciAdres).
   Motor alan-rs/ (Rust) altından derlenir: alan-v5/alan.wasm. İletiler mac-isci.js ile aynı: 'yeni' ile maç kurulur, 'basla' ve 60 tikte bir 'kare' gider (kare: 174 sayı, kaydet'in biçimi).
   Maç içi taktik değişikliği ('degistir'): motor her 60 tikte maçın kopyasını alır, değişiklik gelince en yakın kopyadan yeniden oynar (mac-isci.js gibi); 'izle' ile oynatılanın gerisindeki ara kopyalar atılır.
   Karar kartı (karar: taşıyıcının son kararı, seçenekler, değer dökümü) ve kafa ↔ gerçek kayıtları (kafa: sürpriz) motordan gelir.
   Henüz yok: anlar ('an'; sayfa bunun için JS işçisi açar); karar kartında JS kafasına özgü birkaç ayrıntı (hw, cek, neden, bak, rc, drk, am). */
const ALAN_SURUM_Q = (v => v ? '?v=' + v[1] : '')(/[?&]v=(\w+)/.exec(self.location && self.location.search || ''));
/* seyrek yerleşim (uzak hücumcunun işi 3 tikte bir; motorun varsayılanı): adreste seyrek=0 ise kapalı, eski davranış */
const SEYREK = !/[?&]seyrek=0(&|$)/.test(self.location && self.location.search || '');
/* kafanın uçuşunda yoldaki gövde ve kovalamayan savunmacının yola koşabilmesi (varsayılan açık): adreste govdeyol=0 ise kapalı */
const GOVDE_YOL = !/[?&]govdeyol=0(&|$)/.test(self.location && self.location.search || '');
/* sürmenin tutma olasılığı değerin tartıldığı ara noktaya dek (varsayılan açık): adreste surE=0 ise kapalı */
const SUR_E = !/[?&]surE=0(&|$)/.test(self.location && self.location.search || '');
/* kafanın Bekçi dalış modeli (varsayılan açık): adreste dalis=0 ise kapalı */
const BEK_DALIS = !/[?&]dalis=0(&|$)/.test(self.location && self.location.search || '');
/* sürme yeniden karar anına dek (ufuk), yumuşak bölge değeri (yumusak), sürekli seçim gürültüsü (surekli): varsayılan açık; adreste =0 ise kapalı */
const AC = ad => !new RegExp('[?&]' + ad + '=0(&|$)').test(self.location && self.location.search || '');
let W = null, mem = null, gen = 0, len = 5400, dur = 5400, kosuyor = false, istek = null, buf = null, hazir = null;
/* wasm'da panik geri sarılmaz: örnek bozuk kalır (Rust'taki kilit açık kalır, sonraki her çağrı düşer). Hatadan sonra örnek atılır, sonraki 'yeni' taze örnek kurar. */
function birak() { hazir = null; W = null; mem = null; }
/* wasm göstericileri işaretli i32 gelir: bellek 2 GB'ı aşınca negatif olur ("byteOffset cannot be negative"); >>> 0 ile işaretsiz */
const NF = 174, dek = new TextDecoder(), kod = new TextEncoder();
function yukle() { if (!hazir) hazir = fetch('alan.wasm' + ALAN_SURUM_Q).then(r => { if (!r.ok) throw new Error('alan.wasm yüklenemedi (' + r.status + ')'); return r.arrayBuffer(); }).then(b => WebAssembly.instantiate(b, {})).then(r => { W = r.instance.exports; mem = W.memory; if (W.seyrek) W.seyrek(SEYREK ? 1 : 0); if (W.govde_yol) W.govde_yol(GOVDE_YOL ? 1 : 0); if (W.sur_e) W.sur_e(SUR_E ? 1 : 0); if (W.bek_dalis) W.bek_dalis(BEK_DALIS ? 1 : 0); if (W.sur_ufuk) W.sur_ufuk(AC('ufuk') ? 1 : 0); if (W.sah_yumusak) W.sah_yumusak(AC('yumusak') ? 1 : 0); if (W.gurultu_surekli) W.gurultu_surekli(AC('surekli') ? 1 : 0); if (W.bolge) W.bolge(/[?&]bolge=1(&|$)/.test(self.location && self.location.search || '') ? 1 : 0); if (W.sah_surekli) W.sah_surekli(AC('sahsur') ? 1 : 0); if (W.kayip_sahip) W.kayip_sahip(AC('kayipsahip') ? 1 : 0); if (W.kovsuz) W.kovsuz(AC('kovsuz') ? 1 : 0); if (W.plan_sur) W.plan_sur(AC('plansur') ? 1 : 0); if (W.yerlesik_adam) W.yerlesik_adam(/[?&]yerlesik=1(&|$)/.test(self.location && self.location.search || '') ? 1 : 0); /* deney: yerleşik hücumda adam adama (varsayılan kapalı): adreste yerlesik=1 ile açık */ if (W.plan) W.plan(/[?&]plan=1(&|$)/.test(self.location && self.location.search || '') ? 1 : 0); /* hücum/savunma oturumlarının deneyleri: varsayılan kapalı; adreste ad=1 açar, yeni=1 hepsini (onsira ve savtaban hariç) açar; s7=1 a, d–m parçalarını açar (b etkisiz, c'nin yerini d aldı) */ { const q = self.location && self.location.search || '', A1 = (ad, hepsi) => new RegExp('[?&]' + ad + '=1(&|$)').test(q) || (hepsi && /[?&]yeni=1(&|$)/.test(q) && !new RegExp('[?&]' + ad + '=0(&|$)').test(q)); if (W.sah_yakinsa) W.sah_yakinsa(A1('sahyak', 0) ? 1 : 0); if (W.on_sira) W.on_sira(A1('onsira', 0) ? 1 : 0); if (W.savur_tepki) W.savur_tepki(A1('savtepki', 1) ? 1 : 0); if (W.tut_elde) W.tut_elde(A1('tutelde', 1) ? 1 : 0); if (W.pres_kazanc) W.pres_kazanc(A1('preskazanc', 1) ? 1 : 0); if (W.ua_hist) W.ua_hist(A1('uahist', 1) ? 1 : 0); if (W.sav_taban) W.sav_taban(A1('savtaban', 0) ? 1 : 0); if (W.hucum_boy) W.hucum_boy(A1('hucumboy', 1) ? 1 : 0); if (W.s7) W.s7(A1('s7', 1) ? 8185 : 0); if (W.onde_ac) W.onde_ac(A1('ondeac', 1) ? 1 : 0); if (W.kuyu_pay) W.kuyu_pay(A1('kuyupay', 1) ? 1 : 0); if (W.sah_bekci) W.sah_bekci(A1('sahbekci', 1) ? 1 : 0); if (W.kayip_harita) W.kayip_harita(A1('kayipharita', 1) ? 1 : 0); if (W.yuru) W.yuru(A1('yuru', 1) ? 1 : 0); if (W.hiz2) W.hiz2(A1('hiz2', 0) ? 1 : 0); if (W.ktab) W.ktab(A1('ktab', 0) ? 1 : 0); if (W.sah_zaman) W.sah_zaman(A1('sahzaman', 0) ? 1 : 0); if (W.bek_direk) W.bek_direk(A1('bekdirek', 1) ? 1 : 0); if (W.yerlesik_adam) W.yerlesik_adam(A1('yerlesik', 1) ? 1 : 0); } }); return hazir; }
const metin = n => dek.decode(new Uint8Array(mem.buffer, (W.metin_ptr() >>> 0), n));
function yeniBuf() { return { kare: [], olay: [], karar: [], kafa: [], is: [] }; }
function kaydet() { const p = (W.kare() >>> 0); const f = new Float32Array(NF); f.set(new Float32Array(mem.buffer, p, NF)); buf.kare.push([W.tik(), f]); const y = JSON.parse(metin(W.yenilikler())); for (const o of y.olay) buf.olay.push(o); for (const j of y.is) buf.is.push(j); if (y.karar) for (const k of y.karar) buf.karar.push(k); if (y.kafa) for (const k of y.kafa) buf.kafa.push(k); }
let hsT0 = 0;
function gonder(g, son, yari) { const n = performance.now(), ms = hsT0 ? n - hsT0 : 0; hsT0 = n; postMessage({ ms, pd: null, tip: 'kare', gen: g, kare: buf.kare, olay: buf.olay, karar: buf.karar, kafa: buf.kafa, is: buf.is, tick: W.tik(), son: !!son, yari: !!yari }); buf = yeniBuf(); }
/* Döngü beklerken yeni maç gelirse istek not edilir; eski döngü çıkınca yeni maçı o başlatır (eskiden hiç başlamıyordu). */
async function calistir(g) { if (kosuyor) { istek = g; return; } if (!W) return; kosuyor = true; hsT0 = performance.now(); const bas = W.tik();
  try { while (!W.bitti() && W.tik() < Math.min(len, dur)) { W.adim(); if (g !== gen) return; kaydet(); const t = W.tik(); if (t % 20 === 0) { if (t % 60 === 0 || t - bas < 120) gonder(g); await new Promise(r => setTimeout(r, 0)); hsT0 = performance.now(); if (g !== gen) return; } } }
  catch (e) { let t = '?'; try { t = W.tik(); } catch (e2) {} birak(); postMessage({ tip: 'hata', msg: 'Rust motoru durdu (tik ' + t + '): ' + String(e && e.message || e) }); return; }
  finally { kosuyor = false; const r = istek; istek = null; if (r !== null && r !== g && r === gen && W) calistir(r); }
  if (g === gen) { if (W.bitti() || W.tik() >= len) gonder(g, true); else gonder(g, false, true); } }
onmessage = async e => { const d = e.data;
  if (d.tip === 'izle') { if (d.gen === gen && W && W.seyrelt) W.seyrelt(d.tick); return; }
  if (d.tip === 'kur' || d.tip === 'paralel') return;
  if (d.tip === 'degistir') { if (!W) return; gen = d.gen;
    try { const b = kod.encode(JSON.stringify(d)), p = (W.ayir(b.length) >>> 0); new Uint8Array(mem.buffer, p, b.length).set(b);
      const r = W.degistir(p, b.length); W.birak(p, b.length);
      if (r !== 0) { postMessage({ tip: 'hata', msg: 'Rust motoru taktik değişikliğini uygulayamadı: ' + metin(W.metin_len()) }); return; } }
    catch (er) { birak(); postMessage({ tip: 'hata', msg: 'Rust motoru taktik değişikliğinde durdu: ' + String(er && er.message || er) }); return; }
    buf = yeniBuf(); calistir(gen); return; }
  if (d.tip === 'devam') { if (d.gen !== gen) return; dur = len; if (!kosuyor) calistir(gen); return; }
  if (d.tip === 'yeni') { gen = d.gen; len = d.len || 5400; dur = d.dur || len;
    try { await yukle(); } catch (er) { postMessage({ tip: 'hata', msg: String(er && er.message || er) }); return; }
    let ps;
    try { const b = kod.encode(JSON.stringify(d)), p = (W.ayir(b.length) >>> 0); new Uint8Array(mem.buffer, p, b.length).set(b);
      const r = W.kur(p, b.length); W.birak(p, b.length);
      if (r !== 0) { postMessage({ tip: 'hata', msg: 'Rust motoru maçı kuramadı: ' + metin(W.metin_len()) }); return; }
      buf = yeniBuf(); kaydet(); ps = JSON.parse(metin(W.statik())); }
    catch (er) { birak(); postMessage({ tip: 'hata', msg: 'Rust motoru maçı kurarken durdu: ' + String(er && er.message || er) }); return; }
    postMessage({ tip: 'basla', gen, ps }); calistir(gen); return; }
  if (d.tip === 'an') { postMessage({ tip: 'hata', msg: 'Rust motorunda bu henüz yok: anlar. JS motoruna dönmek için adrese ?motor=js eklenir.' }); return; } };
