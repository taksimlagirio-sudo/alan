# Taşıma ajanları için ortak kurallar

ALAN maç motoru JS'ten (alan-v5) Rust'a taşınıyor. Hedef: telefon ve bilgisayar uygulamasında yerel motor. Oyun JS ile **tıpatıp aynı olmak zorunda değil, ama aynı kalitede olmalı** ("salaklaşmasın"). ~~Yine de hedef işlev işlev aynı sonuç~~ — **5 Kasım: Rust tek doğru kaynak** (`MIMARI.md` Aşama 6): JS ile bit bit aynılık kısıtı kalktı; doğrulama `sina/ayni.sh` (davranış aynı) ve maç istatistikleri/izleme (davranış değişiyorsa). Aşağıdaki taşıma kuralları tarihsel.

## Kaynak

- **Taşınacak kaynak:** `alan-rs/kaynak/<dosya>.js` — sade JS: eski deney dalları (`_ESKI` bayrakları varsayılan değerleriyle) silinmiş, yorumsuz, biçimlenmiş. Orijinaliyle bit bit aynı maçı oynar. Yalnız bu davranışı taşı.
- **Anlam için:** `alan-v5/<dosya>.js` (orijinal, uzun Türkçe yorumlarla). Bir kuralın neden öyle olduğunu oradan oku; davranışı sade dosyadan al.
- Satır numaraları sade dosyaya göredir.

## Hazır olan (dokunma, kullan)

`alan-rs/motor/src/`:
- `mat.rs` — belirlenimli matematik. JS `Math.exp/log/pow/sin/cos/atan/atan2/acos/tanh/hypot` yerine **bunları** kullan (telefon/PC aynı maç). `Math.max/min` → `jmax/jmin` (NaN yayar), `Math.round` → `jround`, `Math.sign` → `jsign`, `Math.max(a, Math.min(b, x))` → `kis(x, a, b)`. `Math.sqrt/abs/floor/ceil` Rust'takiyle aynı. JS doğruluğu (0 ve NaN yanlış) → `dogru`.
- `saha.rs` (W, H, CY, MOUTH, TPS…, top fiziği `P`), `core_sabit.rs` (Q, IN, TQ).
- `alan.rs` (alan değeri K, `Etki`), `top.rs` (stepBall=`adim`, predict=`tahmin`, solveLaunch=`cikis_coz`, rollTo=`roll_to`, `hyp`), `temas.rs` (touchCap, contact, ctrlP, menzil, bekR, kuyuGirer, govde, `Kural` trait: maçın sprint katı), `donus.rs` (Turn), `oyuncu.rs` (Oyuncu yardımcıları, `tepki` = ALAN_TEPKI), `yetenek.rs` (ALAN_Y = `p.yet.y(mek::X)`, ALAN_OZ = `p.yet.ozk("ad")`, zar, kademe, AH, AG, durum payı), `rng.rs` (maçın zar dizisi `m.r`).
- `model.rs` — **ortak veri modeli (taslak, üretilmiş)**: `Oyuncu` (JS `p`, 132 alan + `yet`), `TopEk` (topun ek alanları), `Mac` (JS `m`), `KafaD` (JS `AlanDecide.D`), `Secenek` (kafanın seçeneği `o`). Alan adları JS adlarıyla birebir. `Pid` = `m.ps` içindeki sıra, `TopRef` = topun kimliği. `JsDeger` = karışık tip görüldü (taslak).
  - İç içe yapıların adları yoldan: `JP_sf` = `p._sf`, `JMCtlRBest` = `m.ctl.r.best` …
  - Biçim kaynağı: `sina/ic-sekil2.txt` (çalışma anında görülen her yol ve tip).
- Sınama örnekleri: `motor/tests/*.rs` ve vektör üreticileri `sina/*-vektor.js` (JS motorunu node'da `sina/yukle.js` ile yükler).

## Kurallar

1. **Kapsam:** yalnız sana verilen dosyayı/satır aralığını taşı, yalnız kendi klasörüne (`motor/src/<modül>/`) ve kendi sınamalarına (`motor/tests/<modül>_*.rs`, `sina/<modül>-*.js`) yaz. `lib.rs`'e tek satır `pub mod <modül>;` ekle.
2. **Model:** `model.rs`'te yalnız **senin modülünün yazdığı** alanların tipini düzeltebilirsin (sahiplik: `sina/sahiplik.json` — `yazan` listesinde yalnız senin dosyan varsa senindir). Ortak yazılan alanlara (x, y, vx, vy, tx, ty, en, den, sprint, job…) dokunma; ihtiyacın varsa notla. Yeni alan gerekiyorsa modele değil kendi modülünde bir yapıya koy ve notla.
3. **Başka modülün işlevi:** henüz yok. Kendi klasöründe `gecici_dis.rs` aç, çağıracağın her yabancı işlevi orada **aynı JS adıyla** `todo!()` gövdeli imza olarak yaz (ör. `pub fn threat(team: f64, x: f64, y: f64) -> f64 { todo!() }`), çağrıları oradan yap. Birleştirmede bunlar gerçeklerine bağlanacak. `gecici_dis.rs`'in başına her işlevin JS kaynağını (dosya:satır) yaz.
4. **Adlar:** işlev adları JS adlarıyla birebir (`#![allow(non_snake_case)]`). Kapanışla yakalanan durum (JS'te dış değişken) → açık parametre ya da bağlam yapısı. Gizli küresel durum yok.
5. **JS anlamı:** `a || b` (0/NaN/null/undefined yanlış), `a ?? b` (yalnız null/undefined), `x | 0` (ToInt32), `Object.create(h)` / `{...p}` kopyaları (Rust: klon), `Array.sort` kararlı, Map ekleme sırasıyla gezilir. Regex ile iş adı eşleme (`/^(pres|kapatma)/.test(job)`) → yardımcı işlev.
6. **Doğrulama:** saf işlevler için JS ile **bit bit aynılık** sınaması yaz (örnek: `motor/tests/temas.rs` + `sina/temas-vektor.js`: JS maçından anlık durumlar alınır, JS işlevi çağrılır, girdi ve çıktı 16'lık bitlerle yazılır). Vektör JSON'ları git'e girmez (`sina/*.json`). Her önemli işlev için en az bir sınama; hepsi olamıyorsa en sıcak/en temel olanlar.
6b. **Birebir aynılık:** "davranış aynı" diyen her değişiklik `sina/ayni.sh`'tan geçer (3 maç × 900 tik, her tikte maçın durumu bit bit aynı). Davranışı bilerek değiştiren iş ayrı commit'te, başvurular `sina/ayni.sh --yaz` ile yenilenerek gelir. Hedef yapı ve aşamalar: `MIMARI.md`.
7. **Derleme:** `cargo build --release` ve `cargo test --release` her zaman geçmeli (yabancı işlevler `todo!()`; sınamaların onlara uğramamalı).
8. **Git:** kendi çalışma ağacında, kendi dalında küçük adımlarla commit et (mesajlar Türkçe). Push etme.
9. **Bitince** kendi klasörüne `NOT.md` yaz: neler taşındı, neler eksik/kuşkulu, `gecici_dis.rs`'teki her işlevin beklenen anlamı, modelde değiştirdiğin tipler, bit bit aynı çıkan / çıkmayan işlevler.

## Modüller ve sahipler

| Modül | Klasör | Kaynak | Ajan |
|---|---|---|---|
| Yerleşim | `motor/src/sekil/` | `kaynak/shape.js` (1–3642) | sekil |
| Kafa 1: imkân, uçuş, şut, tutma/ikili (keepP, kesisK, duelP, girEsik…) | `motor/src/kafa/` (`kafa1_*.rs`) | `kaynak/decide.js` 1–4716 | kafa1 |
| Kafa 2: dünyalar, seçenekler, tartı, karar (holdWorld … decide, realize) | `motor/src/kafa/` (`kafa2_*.rs`) | `kaynak/decide.js` 4717–8890 | kafa2 |
| Maç döngüsü | `motor/src/mac/` | `kaynak/match.js` (1–4194) | mac |

Kafa'nın iki ajanı aynı klasörde ama ayrı dosyalarda çalışır: kafa1 `kafa/mod.rs`'i kurar (`pub mod kafa1_...; pub mod kafa2_...;` satırlarını ikisi de kendi dosyaları için ekler; çakışırsa birleştirmede çözülür). Ortak durum `model::KafaD` (JS `D`).
