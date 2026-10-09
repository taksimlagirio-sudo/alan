// zeka.js çıktısını özetler: node tools/zeka-ozet.js <zeka.jsonl>
const R=require('fs').readFileSync(process.argv[2],'utf8').trim().split('\n').map(JSON.parse);
const all=k=>R.flatMap(r=>r.O[k]),avg=a=>a.length?a.reduce((s,x)=>s+x,0)/a.length:NaN,pc=v=>'%'+Math.round(v*100),med=a=>{a=a.slice().sort((x,y)=>x-y);return a[a.length>>1]},q=(a,p)=>{a=a.slice().sort((x,y)=>x-y);return a[Math.floor(a.length*p)]};
const D=all('dec').filter(d=>!d.err),E=all('dec').filter(d=>d.err);
console.log('1 · KARAR ('+D.length+' karar, '+E.length+' hata)');
console.log('  oyuncu = kâhin:',pc(D.filter(d=>d.ok).length/D.length),' · kâhin = kâhin (gürültü tabanı):',pc(D.filter(d=>d.ok2).length/D.length));
const rg=D.filter(d=>d.reg!=null).map(d=>d.reg),rg2=D.filter(d=>d.reg2!=null).map(d=>d.reg2);
console.log('  kayıp (kâhin ölçüsüyle) oyuncu: medyan',med(rg).toFixed(3),'· %90',q(rg,.9).toFixed(3),'· ort',avg(rg).toFixed(3),' | kâhin2: medyan',med(rg2).toFixed(3),'· %90',q(rg2,.9).toFixed(3),'· ort',avg(rg2).toFixed(3));
console.log('  kararın tipik değeri (kâhinin en iyisi) medyan',med(D.map(d=>d.v)).toFixed(3));
const big=D.filter(d=>d.reg!=null&&d.reg>.1);console.log('  büyük kayıp (>0,10):',pc(big.length/D.length),'· kâhin2 için',pc(D.filter(d=>d.reg2!=null&&d.reg2>.1).length/D.length));
const conf={};for(const d of big){const k=d.k+' yerine '+d.ob;conf[k]=(conf[k]||0)+1;}console.log('  büyük kayıpta en sık: ',Object.entries(conf).sort((a,b)=>b[1]-a[1]).slice(0,6).map(([a,b])=>a+' '+b).join(', '));
for(const[lo,hi,nm]of[[0,33,'kendi'],[33,67,'orta'],[67,101,'rakip']]){const d=D.filter(x=>x.adv>=lo&&x.adv<hi);console.log('  '+nm+' üçte bir ('+d.length+'): oyuncu=kâhin',pc(d.filter(x=>x.ok).length/d.length),'· kâhin=kâhin',pc(d.filter(x=>x.ok2).length/d.length),'· ort kayıp',avg(d.filter(x=>x.reg!=null).map(x=>x.reg)).toFixed(3),'vs',avg(d.filter(x=>x.reg2!=null).map(x=>x.reg2)).toFixed(3));}
const P=all('pos');console.log('\n2 · TOPSUZ YERLEŞİM ('+P.length+' ölçüm)');const eff=P.filter(p=>p.best>1e-4).map(p=>p.cur/p.best);
console.log('  yerinin değeri / 1 sn içindeki en iyi yerin değeri: medyan',pc(med(eff)),'· %25',pc(q(eff,.25)),'· pas alamayacak yerde (değer ≈0)',pc(P.filter(p=>p.cur<.005).length/P.length));
for(const[lo,hi,nm]of[[0,40,'geride'],[40,70,'ortada'],[70,101,'önde']]){const e=P.filter(p=>p.adv>=lo&&p.adv<hi&&p.best>1e-4).map(p=>p.cur/p.best);console.log('   '+nm+' ('+e.length+'): medyan',pc(med(e)),'· %25',pc(q(e,.25)));}
const J={};for(const p of P.filter(p=>p.best>1e-4)){(J[p.job]=J[p.job]||[]).push(p.cur/p.best);}console.log('   işe göre:',Object.entries(J).filter(([,v])=>v.length>30).map(([k,v])=>(k||'(yok)')+' '+pc(med(v))).join(', '));
const Ru=all('runs');console.log('\n3 · KOŞULAR ('+Ru.length+' koşu, maç başı '+(Ru.length/R.length).toFixed(1)+')');console.log('  savunmanın arkasına geçen',pc(Ru.filter(r=>r.behind).length/Ru.length),'· koşuya pas gelen',pc(Ru.filter(r=>r.passed).length/Ru.length),'· arkaya geçen koşuya pas gelen',pc(Ru.filter(r=>r.behind&&r.passed).length/Math.max(1,Ru.filter(r=>r.behind).length)),'· koşucunun yanında (4 br) savunmacı olan zaman',pc(avg(Ru.map(r=>r.track))),'· arkaya geçen koşularda',pc(avg(Ru.filter(r=>r.behind).map(r=>r.track))));
const Rc=all('rec');console.log('\n4 · SAVUNMA');console.log('  kayıptan sonra 4+ savunmacı topun gerisinde: ',pc(Rc.filter(r=>r.ok).length/Rc.length),'durumda; süre medyan',med(Rc.filter(r=>r.ok).map(r=>r.t)).toFixed(2),'sn · %90',q(Rc.filter(r=>r.ok).map(r=>r.t),.9).toFixed(2),'sn');
const rv=all('recv');console.log('  rakip yarının ileri 40\'ında pası alan hücumcu: en yakın savunmacı medyan',med(rv),'br · 5 br içinde savunmacı OLMAYAN',pc(rv.filter(x=>x>5).length/rv.length),'('+rv.length+')');
const sh=all('shoot');console.log('  gönderen oyuncuya en yakın savunmacı medyan',med(sh),'br · 3 br içinde savunmacı olan',pc(sh.filter(x=>x<3).length/sh.length),'('+sh.length+')');
console.log('  tehlikeli bölgedeki hücumcuların Kuyu tarafında 6 br içinde savunmacısı olan',pc(avg(all('cover'))));
