const L=require("fs").readFileSync(process.argv[2],"utf8").split("\n").filter(Boolean);const A={0:[],1:[]};
for(const l of L){const m=/WN (\d) (\S+)/.exec(l); if(!m)continue; for(const x of m[2].split(",")){const [w,n,t,h]=x.split(":");A[+m[1]].push({w:+w,n:+n,t,h});}}
console.log("dalga",A[0].length,A[1].length); const H=new Map(A[1].map(x=>[x.w,x])); let k=0;
for(const a of A[0]){const b=H.get(a.w); if(b&&(b.n!==a.n||b.t!==a.t||b.h!==a.h)){console.log("ilk fark ana",JSON.stringify(a),"yardımcı",JSON.stringify(b)); if(++k>6)break;}}
