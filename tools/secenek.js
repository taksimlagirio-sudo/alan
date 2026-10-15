const {oku}=require(process.argv[2]+"/tools/kayit-oku.js");
for(const f of process.argv.slice(3)){const k=oku(f);const N=k.N;const lim=k.oyuncular.map((o,i)=>i).filter(i=>k.oyuncular[i].takim===0&&k.oyuncular[i].rol!=="Bekçi");
 const X=(t,i)=>k.P.x[t*N+i]/100,Y=(t,i)=>k.P.y[t*N+i]/100;const S={};let anlar=0;const acik=[];
 for(let t=0;t<k.n;t+=6){const h=k.T.hold[t];if(h===255||k.oyuncular[h].takim!==0||k.oyuncular[h].rol==="Bekçi")continue;anlar++;let n=0;
  for(const i of lim){if(i===h)continue;const l=k.lane[(t/6)*N+i];const d=Math.hypot(X(t,i)-X(t,h),Y(t,i)-Y(t,h));const ok=l!==255&&l>=50&&d<30;if(ok)n++;
   const ad=k.oyuncular[i].ad;const s=S[ad]=S[ad]||{n:0,ac:0,d:[],on:0,geri:0,yan:0};s.n++;if(ok)s.ac++;s.d.push(d);const dx=X(t,i)-X(t,h);if(dx>4)s.on++;else if(dx<-4)s.geri++;else s.yan++;}
  acik.push(n);}
 acik.sort((a,b)=>a-b);const say=c=>acik.filter(x=>x===c).length;console.log(f.split("/").slice(-2).join("/"),"· taşıyıcı anı",anlar,"· açık seçenek sayısı: 0 →",(100*say(0)/anlar).toFixed(0)+"%, 1 →",(100*say(1)/anlar).toFixed(0)+"%, 2 →",(100*say(2)/anlar).toFixed(0)+"%, 3+ →",(100*acik.filter(x=>x>=3).length/anlar).toFixed(0)+"%");
 for(const [ad,s] of Object.entries(S)){s.d.sort((a,b)=>a-b);console.log("  ",ad.padEnd(6),"açık seçenek %"+(100*s.ac/s.n).toFixed(0),"· taşıyıcıya uzaklık medyan",s.d[s.d.length>>1].toFixed(0),"· topun önünde %"+(100*s.on/s.n).toFixed(0),"yanında %"+(100*s.yan/s.n).toFixed(0),"gerisinde %"+(100*s.geri/s.n).toFixed(0))}}
