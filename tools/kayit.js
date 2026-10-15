// Birleşik ölçüm, adım 1: maçı bir kez oynatır, her şeyi kaydeder. Bütün ölçümler sonra kayıttan okunur (kayit-oku.js), maç yeniden oynatılmaz.
// Kayıt (gzip'li JSON, diziler base64):
//   her tik: taşıyıcı (oyuncu sırası, yoksa 255), Çekirdek konumu ve takımı, şarj, iki takımın evresi
//   her tik × her oyuncu: konum, hız, hedef (tx,ty; yoksa -32768); konumlar ×100, görev (sözlük sırası)
//   her 6 tikte: taşıyıcıdan her takımdaşa laneOk (×100)
//   geçiş (zip39+): her tik m.gc takımı, git, koşucu sayısı, karşı presçi sayısı · LTAC='{...}' Liman'ın taktiğine eklenir (deney)
//   olaylar: paslar ve göndermeler (aklın tahminiyle: P, Pk, Pl), sahiplik değişimleri (nasıl), motorun olay metinleri, kararlar (ilk 4 seçenek)
//   sonda: skor, m.st, hesap süresi
// Kullanım: node tools/kayit.js <motor> <senaryo 0-15> <çıktı.json.gz> [tohum kayması]
const fs = require('fs'), zlib = require('zlib'); const { kur } = require('./kurulum');
const [dir, sS, out, kS] = process.argv.slice(2), sc = +sS, KAY = +kS || 0;
const HOOK = "if (self.__tem && self.__m && b === self.__m.ball && best.role === 'Bekçi' && best.team !== b.team) self.__tem.push({ t: self.__m.tick, bek: best.team, gon: !!b.gon, vr: +T.vr.toFixed(2), cap: +T.cap.toFixed(2), haz: T.haz == null ? null : +T.haz.toFixed(2) }); ";
const coreSrc = fs.readFileSync(require('path').join(dir, 'core.js'), 'utf8'), YA = [["if (T.vr <= capE) { b.done = 'tutuldu';", HOOK + "if (T.vr <= capE) { b.done = 'tutuldu';"], ["return { p: best, took: false, P };", "if (self.__tem && self.__m && b === self.__m.ball && best.role === 'Bekçi' && best.team !== b.team) { const e = self.__tem[self.__tem.length - 1]; e.thru = thru; e.ovx = +b.vx.toFixed(3); e.ovy = +b.vy.toFixed(3); e.ox = +b.x.toFixed(2); e.oy = +b.y.toFixed(2); } return { p: best, took: false, P };"], ["return { cap: Math.max(.08, cap * (p.role", "return { haz, cap: Math.max(.08, cap * (p.role"]].filter(([a]) => coreSrc.includes(a)); /* Bekçi teması (yalnız okur); eski motorlarda bulunmayan yer atlanır */
const { M, m, ad, adim, g } = kur(dir, sc, KAY, { 'match.js': ['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'], 'core.js': YA.length ? YA : undefined }); globalThis.__tem = []; globalThis.__m = m; const AD = g.AlanDecide;
if (process.env.LTABAN) { const v = +process.env.LTABAN; for (const p of m.ps) for (const k of Object.keys(p.a)) if (typeof p.a[k] === 'number') p.a[k] = v; } /* özellik ölçümü: iki takımda bütün sayısal özellikler aynı taban */
if (process.env.LOZ) { for (const kv of process.env.LOZ.split(',')) { const [k, v] = kv.split(':'), bek = ['refleks', 'yerlesim', 'hakimiyet'].includes(k); for (const p of m.ps) if (p.team === 0 && (p.role === 'Bekçi') === bek) p.a = { ...p.a, [k]: +v }; } } /* Liman'ın saha oyuncularında (Bekçi özelliğiyse Bekçi'sinde) tek özellik */
if (process.env.LTAC) { const o = JSON.parse(process.env.LTAC); m.tac[0] = { ...m.tac[0], ...o }; if (m.tac0 && m.tac0[0]) m.tac0[0] = { ...m.tac0[0], ...o }; } /* deney: Liman'ın (takım 0) taktiğine ek */
const LEN = +process.env.LEN || m.len || 5400, N = m.ps.length, ps = m.ps, idx = new Map(ps.map((p, i) => [p, i]));
const T = { hold: new Uint8Array(LEN), bx: new Int16Array(LEN), by: new Int16Array(LEN), bt: new Int8Array(LEN).fill(-1), ch: new Uint8Array(LEN), ev0: new Int8Array(LEN).fill(-1), ev1: new Int8Array(LEN).fill(-1), gcT: new Int8Array(LEN).fill(-1), gcGit: new Uint8Array(LEN), gcOn: new Uint8Array(LEN), kp: new Uint8Array(LEN), kpT: new Int8Array(LEN).fill(-1) };
const P = { x: new Int16Array(LEN * N), y: new Int16Array(LEN * N), vx: new Int16Array(LEN * N), vy: new Int16Array(LEN * N), tx: new Int16Array(LEN * N), ty: new Int16Array(LEN * N), job: new Uint8Array(LEN * N) };
const LN = new Uint8Array(Math.ceil(LEN / 6) * N).fill(255); const jobs = [], jobI = new Map(); const ji = j => { const s = String(j ?? ''); let i = jobI.get(s); if (i == null) { i = jobs.length; jobs.push(s); jobI.set(s, i); } return i; };
const r1 = v => Math.round(v * 100), r3 = v => Math.round((v || 0) * 1000), pid = p => p ? idx.get(m.ps.find(q => q.id === p.id)) : null;
const pas = [], sahip = [], olay = [], karar = []; let fl = null, h0 = null, e0 = m.events[0], ld = null, hesapMs = 0, n = 0;
while (!m.over && m.tick < LEN) { const t1 = Date.now(); adim(); hesapMs += Date.now() - t1; const t = n++; if (t >= LEN) break; const h = m.holder;
  T.hold[t] = h ? idx.get(h) : 255; const b = h || m.ball; if (b) { T.bx[t] = r1(b.x); T.by[t] = r1(b.y); } T.bt[t] = h ? h.team : m.ball ? m.ball.team ?? -1 : -1; T.ch[t] = Math.round((m.ch || 0) * 200);
  T.ev0[t] = M.EVKOD.indexOf(M.evreBul(m, 0)); T.ev1[t] = M.EVKOD.indexOf(M.evreBul(m, 1)); if (m.gc) { T.gcT[t] = m.gc.team; T.gcGit[t] = m.gc.git ? 1 : 0; T.gcOn[t] = (m.gc.on || []).length; T.kp[t] = m.gc.kp && m.tick < m.gc.kp.bitis ? m.gc.kp.ids.length : 0; } if (m.kp && m.tick < m.kp.bitis) { T.kpT[t] = m.kp.team; T.kp[t] = (m.kp.ids || []).length; }
  for (let i = 0; i < N; i++) { const p = ps[i], k = t * N + i; P.x[k] = r1(p.x); P.y[k] = r1(p.y); P.vx[k] = r3(p.vx); P.vy[k] = r3(p.vy); P.tx[k] = p.tx == null ? -32768 : r1(p.tx); P.ty[k] = p.ty == null ? -32768 : r1(p.ty); P.job[k] = ji(p.job); }
  if (h && t % 6 === 0) for (let i = 0; i < N; i++) { const q = ps[i]; if (q.team === h.team && q !== h && q.role !== 'Bekçi') LN[(t / 6) * N + i] = Math.round(AD.laneOk(m.ps, h.team, h, q) * 100); }
  if (h !== h0) { sahip.push({ t, kim: h ? idx.get(h) : null, takim: h ? h.team : null, onceki: h0 ? idx.get(h0) : null }); h0 = h; }
  if (m.fl && m.fl !== fl) { fl = m.fl; const o = fl.o; if (o) { const from = m.ball && m.ball.from; pas.push({ t, tur: o.kendine ? 'kendine' : o.kind, kim: from ? idx.get(from) : null, alici: o.q ? pid(o.q) : null, to: o.to ? [r1(o.to.x), r1(o.to.y)] : null, P: o.det ? o.det.P : null, Pk: o.det ? o.det.Pk : null, Pl: o.det ? o.det.Pl : null, ot: !!o.ot, sut: o.shot || null, fake: m.fake ? { hid: m.fake.hid, t0: m.fake.t0 } : null }); } }
  if (m.events[0] !== e0) { const yeni = []; for (const e of m.events) { if (e === e0) break; yeni.push(e); } for (const e of yeni.reverse()) olay.push({ t, tur: e.type, takim: e.team, metin: e.text }); e0 = m.events[0]; }
  if (m.lastDec && m.lastDec !== ld) { ld = m.lastDec; karar.push({ t, kim: h ? idx.get(h) : null, s: ld.opts.map(x => [x.kind, x.q, x.P == null ? null : +x.P.toFixed(3), x.v == null ? null : +x.v.toFixed(4), x.hw ? [x.hw.kayip ? 1 : 0, x.hw.tik, x.hw.T, x.hw.Pk ?? null, x.hw.sonraP ?? null, x.hw.sonra ?? x.sonra ?? null] : null]) }); } }
const b64 = a => Buffer.from(a.buffer, a.byteOffset, a.byteLength).toString('base64');
const kayit = { surum: 3, saha: g.ALAN_SAHA || null, ltac: process.env.LTAC || null, ltaban: process.env.LTABAN || null, loz: process.env.LOZ || null, motor: dir, sc, ad, kay: KAY, n, N, LEN, oyuncular: ps.map(p => ({ id: p.id, takim: p.team, rol: p.role, ad: p.name, a: p.a })), tac: m.tac0 || m.tac, gorevler: jobs,
  T: Object.fromEntries(Object.entries(T).map(([k, v]) => [k, b64(v)])), P: Object.fromEntries(Object.entries(P).map(([k, v]) => [k, b64(v)])), lane: b64(LN),
  pas, sahip, olay, karar, tem: globalThis.__tem, skor: m.score, st: m.st, hesapSn: +(hesapMs / 1000).toFixed(1) };
fs.writeFileSync(out, zlib.gzipSync(JSON.stringify(kayit)));
console.log(JSON.stringify({ sc, ad, skor: m.score, hesapSn: kayit.hesapSn, boyutKB: Math.round(fs.statSync(out).size / 1024) }));
