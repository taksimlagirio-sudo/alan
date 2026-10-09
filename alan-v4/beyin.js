(function () {
  // Beyin (v4). Oyun tek bir beynin dünyasında oynanır: oyuncular kafasızdır, her komutu beyin verir.
  // Dünyanın yasaları (hareket, Çekirdek, alanlar, şarj, temas, ikili mücadelenin zarı, sayı) core.js ve match.js'tedir; beyin onları bilir ve
  // karar verirken kendi dünyasını ileri oynatabilir, ama değiştiremez. Beynin serbest olduğu tek yer komutlardır.
  // Komut noktaları (match.js çağırır): konum (topsuz herkes: yerleşim, pres, kovalama) · ikili (savunmacı girsin mi) · karar (top sahibi) · tekDokunus (alıcı).
  // Refleks katmanı: aşağıdaki varsayılan kurallar (v3g'nin oyuncu kuralları). İleri bakış katmanı bu refleksleri temel alır ve üstüne kurulur.
  const hyp = window.AlanCore.hyp, C = () => window.AlanCore, Dd = () => window.AlanDecide, Mm = () => window.AlanMatch;
  const PASSK = ['pas', 'önüne', 'aşırt', 'kenardan'];
  const R = {
    // topsuz oyuncular: yerleşim, pres, kovalama (Katman 4 kuralları)
    konum(m) { if (window.AlanShape) window.AlanShape.position(m); else Mm().position(m); },
    // ikili mücadele: savunmacı, kendi kestirimine (Okuma) göre girer
    ikili(m, q, h, P, est) { return !(est < Dd().commitThr(q, h, m.ps)); },
    // top sahibinin kararı
    karar(m, h) { const D = Dd(), tE = Mm().tacEff(m, h.team), r = D.decide(h, m.ps, m.ch, m.r, tE); let o = r.best;
      // Karşılamadan önce görülen pas: alıcı Çekirdek gelmeden bakmıştı (plan). Kontrol süresi geçince, aynı hedef hâlâ makulse (tutma ihtimali en fazla 0,15 düşmüşse) onu oynar.
      const pl = h._plan; h._plan = null; if (pl && m.tick - pl.t < 30) { const same = r.opts.find(x => x.kind === pl.kind && x.q === pl.q && x.to && hyp(x.to.x - pl.to.x, x.to.y - pl.to.y) < 4); if (same && same.det && same.det.P >= pl.P - .15 && same.v > 0) o = same; }
      m.lastDec = { name: h.name, team: h.team, x: h.x, y: h.y, opts: r.opts.slice(0, 4).map(o => ({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), to: o.to, q: o.q ? o.q.name : '', P: o.det ? (o.kind === 'gönder' ? o.det.Pk : o.det.P) : null, v: o.v })) };
      return o; },
    // alıcı: tek dokunuşla mı oynasın, kontrol mü etsin
    tekDokunus(m, p, chFull, chCtrl, ot) { const D = Dd(); let one = null;
      if (!m._lite && self.ALAN_OKC(p, 'karar') >= D.D.otOk) { const tE = Mm().tacEff(m, p.team), r = D.decideOT(p, m.ps, chFull, m.r, tE, ot), rc = (() => { /* Kontrol etmenin bedeli zamandır: Çekirdeği durdurup gitmek istediği tarafa döndürene kadar savunma kapanır. */
          let tTurn = 0; const K2 = C(), aim = r && (r.T || r.to); if (aim && p.ca != null) { const ta = Math.atan2(aim.y - p.y, aim.x - p.x); tTurn = K2.Turn.arc(p.ca, ta, K2.Turn.shortDir(p.ca, ta)) / K2.Turn.omega(p); }
          const w = D.D.diagCtrlReal ? m.ps : D.predictWorld(m.ps, p.team, p, D.D.ctrlT + tTurn + 9), rc1 = D.decide(p, w, chCtrl, m.r, tE); for (const o of rc1.opts) if (o.q) o.q = m.ps.find(z => z.id === o.q.id) || o.q; return rc1; })(); const ctrlBest = rc.opts.length ? rc.opts[0].v : 0; const b0 = rc.best; p._plan = b0 && b0.q && b0.to && b0.det && PASSK.includes(b0.kind) ? { kind: b0.kind, q: b0.q, to: { x: b0.to.x, y: b0.to.y }, P: b0.det.P ?? 0, t: m.tick } : null; if (r && r.v > ctrlBest + D.D.otMargin) one = r; }
      return one; }
  };
  window.AlanBeyin = { refleks: R, konum: m => R.konum(m), ikili: (m, q, h, P, est) => R.ikili(m, q, h, P, est), karar: (m, h) => R.karar(m, h), tekDokunus: (m, p, a, b, c) => R.tekDokunus(m, p, a, b, c) };
})();
