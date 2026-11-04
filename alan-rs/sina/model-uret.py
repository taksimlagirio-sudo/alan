# Çalışma anı biçim envanterinden (ic-sekil2.txt) taslak Rust modeli üretir: model.rs
import re, sys, unicodedata
from collections import OrderedDict
yollar = OrderedDict()
for l in open(sys.argv[1]):
    l = l.rstrip('\n')
    m = re.match(r'^(\S+) : ([^{]+?)(?: \{(.*)\})?$', l)
    if not m: continue
    yol, t, k = m.group(1), set(m.group(2).split('|')), (m.group(3).split(',') if m.group(3) else [])
    yollar[yol] = (t, k)
KW = {'type','ref','as','in','fn','mod','move','loop','match','where','use','self','Self','struct','enum','impl','let','mut','pub','return','static','trait','true','false','while','for','if','else','break','continue','const','crate','dyn','extern','super','unsafe','async','await','box','yield','try','abstract','final','override','priv','typeof','unsized','virtual','do','macro'}
TR = str.maketrans('çğıöşüÇĞİÖŞÜâÂ', 'cgiosuCGIOSUaA')
def ident(k):
    s = k.translate(TR)
    s = unicodedata.normalize('NFKD', s).encode('ascii', 'ignore').decode()
    s = re.sub(r'[^A-Za-z0-9_]', '_', s)
    if not s or s[0].isdigit(): s = 'k' + s
    return ('r#' + s) if s in KW else s
def tipad(yol):
    s = re.sub(r'\[\]', 'E', yol); s = re.sub(r'\{k\}', 'K', s); s = re.sub(r'\{v\}', 'V', s); s = re.sub(r'\{\}', 'E', s)
    parts = [p for p in s.split('.') if p]
    out = ''.join((ident(p).replace('r#', '')[:1].upper() + ident(p).replace('r#', '')[1:]) for p in parts)
    return 'J' + out
structs = OrderedDict()
def tip(yol):
    if yol not in yollar: return 'JsDeger', 'görülmedi'
    t, k = yollar[yol]
    opt = bool(t & {'null', 'undef'})
    tt = t - {'null', 'undef'}
    not_ = '|'.join(sorted(t))
    if not tt: return 'Option<JsDeger>', not_
    if 'OYUNCU' in tt and tt <= {'OYUNCU', 'object'}: r = 'Pid'
    elif 'TOP' in tt or 'TOPKOPYA' in tt: r = 'TopRef'
    elif tt == {'number'}: r = 'f64'
    elif tt == {'string'}: r = 'String'
    elif tt == {'boolean'}: r = 'bool'
    elif tt == {'function'}: r = 'JsFn'
    elif tt == {'tipli'}: r = 'Vec<f64>'
    elif tt == {'object'}:
        if k and all(re.fullmatch(r'\d+', x) for x in k):
            el = '|'.join(sorted(set.union(*[yollar.get(yol + '.' + x, (set(), []))[0] for x in k])))
            ku = OrderedDict()
            for x in k:
                for kk in yollar.get(yol + '.' + x, (set(), []))[1]: ku[kk] = 1
            # sayı anahtarlı nesne: sıralı harita
            ad = tipad(yol + '[]')
            yollar[yol + '[]'] = (set(el.split('|')), list(ku))
            for x in k:
                for (yy, vv) in list(yollar.items()):
                    if yy.startswith(yol + '.' + x + '.'):
                        yollar.setdefault(yol + '[]' + yy[len(yol + '.' + x):], vv)
            e, _ = tip(yol + '[]')
            r = f'std::collections::BTreeMap<usize, {e}>'
        else:
            r = tipad(yol); structs[r] = (yol, k)
    elif tt == {'dizi'}:
        e, _ = tip(yol + '[]'); r = f'Vec<{e}>'
    elif tt == {'Map'}:
        kk, _ = tip(yol + '{k}'); vv, _ = tip(yol + '{v}'); r = f'Vec<({kk}, {vv})>'
    elif tt == {'Set'}:
        e, _ = tip(yol + '{}'); r = f'Vec<{e}>'
    else:
        r = 'JsDeger'
    return (f'Option<{r}>' if opt else r), not_
def kok(ad, yol0, anahtarlar, aciklama):
    structs[ad] = (yol0, anahtarlar)
KALICI = {'p': {'id','team','i','role','slot','name','a','R','D','x','y','vx','vy','tx','ty','cd'}, 'b': {'x','y','vx','vy','e','ch','team','alive','dead','t','dist','walls','done','log'}, 'm': {'r','seed','tick','len','devre','devre2','score','ps','ball','holder','ch','decT','over','events','fl','tac','st'}, 'o': {'kind'}}
ana = OrderedDict([('Oyuncu', 'p'), ('TopEk', 'b'), ('Mac', 'm'), ('KafaD', 'D'), ('Secenek', 'o')])
for ad, y in ana.items():
    ks = [x[len(y) + 1:] for x in yollar if x.startswith(y + '.') and '.' not in x[len(y) + 1:] and '[' not in x[len(y) + 1:] and '{' not in x[len(y) + 1:]]
    structs[ad] = (y, ks)
yaz = []
yaz.append('//! Ortak veri modeli: TASLAK, çalışma anı envanterinden üretildi (sina/model-uret.py, sina/ic-sekil2.txt).')
yaz.append('//! Alan adları JS adlarıyla birebir (izlenebilirlik). Tipler gözlemden: Option = null/undefined görüldü;')
yaz.append('//! Pid = m.ps içindeki oyuncunun sırası; TopRef = topun kimliği (JS nesne kimliği / kopya).')
yaz.append('//! JsDeger = karışık tip görüldü: sahibi olan modül uygun tipe çevirir. Biçimi değiştirmek: model-sahiplik.md')
yaz.append('#![allow(non_snake_case, non_camel_case_types, dead_code)]')
yaz.append('pub type Pid = usize;')
yaz.append('pub type TopRef = u64;')
yaz.append('#[derive(Clone, Debug, Default)] pub struct JsFn;')
yaz.append('/// Karışık tipli JS değeri (taslakta; sahibi düzeltir).')
yaz.append('#[derive(Clone, Debug, Default)] pub enum JsDeger { #[default] Yok, Sayi(f64), Metin(String), Mantik(bool), Dizi(Vec<JsDeger>), Nesne(Vec<(String, JsDeger)>) }')
yapilan = set()
while True:
    bek = [a for a in structs if a not in yapilan]
    if not bek: break
    for ad in bek:
        yapilan.add(ad)
        yol, ks = structs[ad]
        alanlar = []
        for k in ks:
            t, n = tip(yol + '.' + k)
            if yol in KALICI or yol == 'D':
                kal = KALICI.get(yol, set())
                if yol == 'D': kal = {x for x in ks if not x.startswith('_')}
                if k not in kal and not t.startswith('Option<') and t != 'bool':
                    t = f'Option<{t}>'; n += ' (sonradan eklenir)'
            alanlar.append((k, t, n))
        yaz.append('')
        yaz.append(f'/// JS `{yol}`')
        yaz.append('#[derive(Clone, Debug, Default)]')
        yaz.append(f'pub struct {ad} {{')
        for k, t, n in alanlar:
            yaz.append(f'    /// `{k}`: {n}')
            yaz.append(f'    pub {ident(k)}: {t},')
        if ad == 'Oyuncu':
            yaz.append('    /// (Rust) yetenek satırı: `a`dan maç başında kurulur (ALAN_Y)')
            yaz.append('    pub yet: crate::yetenek::Yetenek,')
        yaz.append('}')
open(sys.argv[2], 'w').write('\n'.join(yaz) + '\n')
print('yapı', len(structs))
