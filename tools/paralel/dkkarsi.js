const L=require("fs").readFileSync(process.argv[2],"utf8").split("\n").filter(Boolean); const H={0:new Map(),1:new Map()};
for(const l of L){const m=/DKH (\d) (\S+)/.exec(l); if(!m)continue; for(const x of m[2].split(",")){const [w,i,k,h]=x.split(".");H[+m[1]].set(w+"."+i,{k,h});}}
const ks=[...H[0].keys()].filter(k=>H[1].has(k)).sort((a,b)=>{const [w1,i1]=a.split(".").map(Number),[w2,i2]=b.split(".").map(Number);return w1-w2||i1-i2;});
let n=0;for(const k of ks){const a=H[0].get(k),b=H[1].get(k); if(a.h!==b.h){console.log("fark",k,"ana",a.k,"yard",b.k); if(++n>12)break;}} console.log("ortak",ks.length);
