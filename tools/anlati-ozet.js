// anlati.js çıktısını özetler: node tools/anlati-ozet.js <anlati.jsonl> <goller.json>
const R=require('fs').readFileSync(process.argv[2],'utf8').trim().split('\n').map(JSON.parse);
const an={},ex={};let goals=[];
for(const r of R){for(const[k,v]of Object.entries(r.an))an[k]=(an[k]||0)+v;for(const[k,v]of Object.entries(r.ex))(ex[k]=ex[k]||[]).push(...v.map(x=>r.cell+r.i+' '+x));
 const d=[0,...r.danger,r.ticks].sort((a,b)=>a-b);let mx=0;for(let i=1;i<d.length;i++)mx=Math.max(mx,d[i]-d[i-1]);
 console.log(r.cell,r.i,'skor',r.score.join('-'),'· tehlikeli an',r.danger.length,'· en uzun ölü süre',(mx/60).toFixed(0)+' sn','· önde değişimi',r.leadCh,'· eşitlenme',r.eq,'· sahipsiz duran Çekirdek',(r.loose/r.ticks*100).toFixed(1)+'%');goals.push(...r.goals.map(g=>({...g,cell:r.cell+r.i})));}
console.log('\nDEDEKTÖRLER (9 maç):');for(const[k,v]of Object.entries(an))console.log(' ',k,v,'· örnek:',(ex[k]||[]).slice(0,3).join(' | '));
const typ={};for(const g of goals){const st=g.steps,last=st[st.length-1]||{},passes=st.filter(s=>!s.kind.startsWith('gönder')).length;let k=g.own?'kendi kalesine':g.start&&/kesme|mücadele/.test(g.start.how)&&g.dur<6?'kontra (kazan→6 sn içinde)':g.dur>=8||passes>=5?'kurulu hücum':'kısa hücum';typ[k]=(typ[k]||0)+1;g.k=k;g.last=last;g.passes=passes;}
console.log('\nGOLLER',goals.length,typ);
const sh=goals.filter(g=>g.last.gd!=null);console.log('bitiriş uzaklığı',sh.map(g=>g.last.gd).sort((a,b)=>a-b).join(','));console.log('tek dokunuş bitiriş',sh.filter(g=>g.last.ot).length,'/',sh.length,'· Bekçi kale merkezinden uzaklık',sh.map(g=>g.last.bk).sort((a,b)=>a-b).join(','),'· bitiriş tahmini',sh.filter(g=>g.last.P!=null).map(g=>g.last.P).sort().join(','));
console.log('golün başladığı yer (kendi kalesinden ilerleme 0-100):',goals.filter(g=>g.start).map(g=>g.start.at+':'+g.start.how).join(', '));
console.log('gol öncesi pas sayısı',goals.map(g=>g.passes).join(','),'· süre',goals.map(g=>g.dur).join(','));
require('fs').writeFileSync(process.argv[3],JSON.stringify(goals));
