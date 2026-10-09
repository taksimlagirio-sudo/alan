#!/bin/sh
# "Kafa" kopyası: alan-v3f'in decide.js'ine sadece KAYIT ekler (D._dbgOn açıkken). Kararlar değişmez (iz testiyle doğrulanır).
# Her gerçek kararda: ön elemede elenen seçenekler (kaba değerleriyle), tartılan seçeneklerin bileşenleri (tutma ihtimali, varış değeri, kayıp bedeli), ileriye bakış öncesi/sonrası değer, seçilen.
set -e; cd "$(dirname "$0")/.."; D=deney/kafa; rm -rf $D; mkdir -p deney; cp -r alan-v3f $D
python3 - $D/decide.js <<'PY'
import sys; f=sys.argv[1]; s=open(f).read()
def rep(a,b):
    global s; assert s.count(a)==1,a[:60]; s=s.replace(a,b)
rep("  function decide(h, src, ch, rnd, tac) {\n","  function decide(h, src, ch, rnd, tac) {\n    const DBG = D._dbgOn && !D._inLook && !D._ot ? { drop: [] } : null;\n")
rep("const pl = opts.filter(o => o.q).map(o => [o, rough(o)])","const pl = opts.filter(o => o.q).map(o => { const r = rough(o); o._rough = r; return [o, r]; })")
rep("const drop = new Set([...pl.filter(x => !keepSet.has(x[0])), ...sh.slice(nS)].map(x => x[0]));","const drop = new Set([...pl.filter(x => !keepSet.has(x[0])), ...sh.slice(nS)].map(x => x[0])); if (DBG) for (const o of drop) DBG.drop.push({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), q: o.q ? o.q.name : '', to: o.to ? { x: +o.to.x.toFixed(1), y: +o.to.y.toFixed(1) } : null, rough: o._rough ?? null });")
rep("o.v = im.Pk + im.P * recvVal(w, o.q, at, recvCh(w, team, at.x, at.y, ch)) - im.Pl * lossAt(im.end.x, im.end.y);","{ const _rv = recvVal(w, o.q, at, recvCh(w, team, at.x, at.y, ch)), _ls = lossAt(im.end.x, im.end.y); o.v = im.Pk + im.P * _rv - im.Pl * _ls; if (DBG) o._c = { P: im.P, Pk: im.Pk, Pl: im.Pl, varis: _rv, kayip: _ls }; }")
import re
m=re.search(r"o\.v = im\.P \* recvVal\(w, q, c\.T, (.*?)\) - im\.Pl \* lossAt\(im\.end\.x, im\.end\.y\);", s); assert m
old=m.group(0); arg=m.group(1)
rep(old, "{ const _rv = recvVal(w, q, c.T, "+arg+"), _ls = lossAt(im.end.x, im.end.y); o.v = im.P * _rv - im.Pl * _ls; if (DBG) o._c = { P: im.P, Pk: 0, Pl: im.Pl, varis: _rv, kayip: _ls }; }")
rep("if (o.kind === 'tut') { const P = keepP(h, src, null, T), c2 = Math.min(1, ch + chargeRate(h, src, 1, ch) * T), w = respond(src, team, h, T, 'drive'); o.v = P * V(w, team, h.x, h.y, c2) - (1 - P) * lossAt(h.x, h.y); o.det = { P, ch: c2 }",
    "if (o.kind === 'tut') { const P = keepP(h, src, null, T), c2 = Math.min(1, ch + chargeRate(h, src, 1, ch) * T), w = respond(src, team, h, T, 'drive'); o.v = P * V(w, team, h.x, h.y, c2) - (1 - P) * lossAt(h.x, h.y); o.det = { P, ch: c2 }; if (DBG) o._c = { P, varis: V(w, team, h.x, h.y, c2), kayip: lossAt(h.x, h.y) }")
rep("o.v = P * Math.max(V(w, team, o.to.x, o.to.y, c2), D.useSpace ? runOn(w, team, h, o.to) : 0) - (1 - P) * lossAt(o.to.x, o.to.y); o.det = { P, ch: c2 }",
    "o.v = P * Math.max(V(w, team, o.to.x, o.to.y, c2), D.useSpace ? runOn(w, team, h, o.to) : 0) - (1 - P) * lossAt(o.to.x, o.to.y); o.det = { P, ch: c2 }; if (DBG) o._c = { P, varis: Math.max(V(w, team, o.to.x, o.to.y, c2), D.useSpace ? runOn(w, team, h, o.to) : 0), kayip: lossAt(o.to.x, o.to.y) }")
rep("if (!reuseLook(h, src, opts)) { lookAhead(h, src, ch, tac, opts, rnd); saveLook(h, src, opts); }","if (DBG) for (const o of opts) o._pre = o.v; if (!reuseLook(h, src, opts)) { lookAhead(h, src, ch, tac, opts, rnd); saveLook(h, src, opts); }")
rep("const ok = opts.filter(o => o.v != null).sort((a, b) => b.v - a.v);","const ok = opts.filter(o => o.v != null).sort((a, b) => b.v - a.v); if (DBG) (D._dbgLog || (D._dbgLog = [])).push({ name: h.name, team, x: h.x, y: h.y, tick: D._tick, drop: DBG.drop, opts: ok.map(o => ({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), q: o.q ? o.q.name : '', to: o.to ? { x: +o.to.x.toFixed(1), y: +o.to.y.toFixed(1) } : null, rough: o._rough ?? null, c: o._c || null, pre: o._pre ?? null, look: o.look ?? null, v: o.v })) });")
open(f,'w').write(s)
PY
echo kuruldu
