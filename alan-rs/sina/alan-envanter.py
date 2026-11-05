# sina/alan-envanter.sh için: derleyici hatalarından alan kullanım yerlerinin sınıflaması.
import json, re, sys, collections
adlar_d, hata, kok = sys.argv[1], sys.argv[2], sys.argv[3]
yerli = len(sys.argv) > 4 and sys.argv[4] == '--yer'
adlar=[l.strip() for l in open(adlar_d) if l.strip()]
src={}
def satir(f,n):
    if f not in src: src[f]=open(kok+'/'+f).read().split('\n')
    return src[f][n-1]
def katman(f):
    f=f.replace('motor/src/','')
    if f.startswith('kural/'): return 'kural'
    if f.startswith('dunya/'): return 'dunya'
    if f.startswith('mac/'): return 'mac'
    if f.startswith('sekil/konum'): return 'yerlesim'
    if f.startswith('sekil/'): return 'sekil'
    if re.match(r'kafa/kafa2_(karar|tarti)',f): return 'karar'
    if f.startswith('kafa/'): return 'kafa'
    return 'temel:'+f
yaz=re.compile(r'^\s*(=(?!=)|\+=|-=|\*=|/=|\|=|&=)')
mutm=re.compile(r'^\s*\.(take\(\)|insert\(|push\(|get_or_insert|as_mut\(\)|clear\(\)|retain\(|extend\(|truncate\(|remove\(|pop\(\)|sort|iter_mut|entry\(|replace\(|make_mut|get_mut)')
R=collections.defaultdict(lambda: {'oku':collections.Counter(),'yaz':collections.Counter(),'kur':collections.Counter(),'yer':collections.defaultdict(list)})
bilinmeyen=0
for l in open(hata):
    try: d=json.loads(l)
    except: continue
    if d.get('reason')!='compiler-message': continue
    m=d['message']
    if m['level']!='error': continue
    msg=m['message']
    a=re.search(r'field (?:named )?`([A-Za-z_0-9]+)`',msg)
    if not a or a.group(1) not in adlar: bilinmeyen+=1; continue
    ad=a.group(1)
    sp=[s for s in m['spans'] if s['is_primary']][0]
    f=sp['file_name']; n=sp['line_start']; c0=sp['column_start']-1; c1=sp['column_end']-1
    t=satir(f,n); k=katman(f)
    once=t[:c0]; sonra=t[c1:]
    # struct ifadesi / desen: kuruluş
    if 'has no field named' in msg or m.get('code') and m['code']['code'] in ('E0560','E0026'):
        tur='kur'
    elif yaz.match(sonra) or mutm.match(sonra) or re.search(r'&mut\s+[\w\.\[\]\(\)\*]*$',once):
        tur='yaz'
    else: tur='oku'
    R[ad][tur][k]+=1
    if tur=='yaz': R[ad]['yer'][k].append(f.replace('motor/src/','')+':'+str(n))
print(f"# Oyuncu alan envanteri: {len(adlar)} alan; tanınmayan hata {bilinmeyen}")
print("# alan oku yaz(katman:sayı)")
for a in adlar:
    r=R[a]
    print(f"{a:14} oku={sum(r['oku'].values()):4} yaz=" + ','.join(f"{k}:{v}" for k,v in sorted(r['yaz'].items())))
    if yerli:
        for k,v in sorted(r['yer'].items()):
            print('    ', k, ' '.join(v))
