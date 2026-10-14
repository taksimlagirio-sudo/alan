# ALAN · Design için plan: oyuncu özellikleri (9 → 15 + Bekçi + taraf)

Kullanıcı bu listeyi onayladı. Bu belge yönü ve motordaki karşılığını anlatır; değerler (katsayılar) Claude Code'un ölçümüyle oturur.

## 0 · Neden

- Bugün 9 özellik var ve çok dengesiz. **Okuma** motorda 40–70 yerde (tepki, algı hatası, pas okuma, boşluk görme, fake'i fark etme, yanılgıdan dönme, tek dokunuş, Bekçi yerleşimi, pres tepkisi…): neredeyse bütün zihinsel işler tek sayıda. **Cesaret** tek yerde (kararın risk ağırlığı) ve yükseldikçe iyi olmuyor: özellik değil kişilik.
- **Aktarım** hem pas hem gönderme; **Tutuş** hem top kontrolü hem şarj; **Yoğunluk** hem dokunma menzili hem omuz ikilisi (adı ne yaptığını anlatmıyor).
- Kullanıcı: özellik sayısı az; oyuncular birbirinden yeterince ayrışmıyor.

## 1 · Kurallar (her özellik için)

1. **Tek mekanik ailesi.** Bir özellik bir işi yapar; bir iş tek özellikten beslenir (gerekirse iki özelliğin açık bir birleşimi, yazılı olarak).
2. **Her yerde aynı anlam.** Oyuncu nerede oynarsa oynasın.
3. **Yükseldikçe hiç kötüleşmez** (tek istisna: Taraf, puan değil).
4. **Sahada görünür.** Hızlı oyuncu hızlı görünmeli, iyi pasör iyi pas atmalı; gizli bilgi ve scout sistemi buna dayanacak.
5. **Akıl da görür.** Kafadaki maç aynı fonksiyonları kullandığı için kendiliğinden; AKIL-FARKINDALIK'a her özellik için bir satır.

## 2 · Liste

### Fiziksel
| # | Özellik | Ne yapar | Bugün nereden |
|---|---|---|---|
| 1 | **Hız** | Koşu hızı | Hız (aynı) |
| 2 | **Çeviklik** | İvmelenme, yön değiştirme, hamleden toparlanma; koşu penceresindeki dönüş kaybı (`ivme`, `gec` tablosu) | Çeviklik (aynı) |
| 3 | **Güç** | Omuz ikilisi, gövdeyle koruma, çarpışmada ayakta kalma | Yoğunluk'un omuz kısmı (`match.js` ikili: `(q.a.yogunluk - h.a.yogunluk) * .03`) |
| 4 | **Uzanma** | Dokunma menzili | Yoğunluk'un menzil kısmı (`oppReach`, `Q.reach + (yogunluk - 10) * .04`) |
| 5 | **Alan gücü** | Oyuncunun alanının gücü ve genişliği (`p.D`, `p.R`; bugün herkes R 8, D 1). Savunmada Çekirdeği yavaşlatır ve söndürür, hücumda takımdaşın pas yolunu kolaylaştırır (`ownFlow`). Oyunun adı buradan. | yeni |

### Topla
| # | Özellik | Ne yapar | Bugün nereden |
|---|---|---|---|
| 6 | **Kontrol** | Gelen topu karşılama, ilk dokunuş (`ctrlP`, `touchCap`) | Tutuş'un kontrol kısmı |
| 7 | **Sürme** | Topla dönüş, çalım, topu koruma, gövde fake'i | Sürme (aynı) |
| 8 | **Pas** | Pasın isabeti ve hızı (`applyError` pasta, pas `v0`) | Aktarım'ın pas kısmı |
| 9 | **Gönderme** | Göndermenin isabeti ve gücü (`gonEk`, gönderme `v0`, plase) | Aktarım'ın gönderme kısmı |
| 10 | **Yükleme** | Çekirdeğe şarj aktarma. Üç parçası var (aşağıda) | Tutuş'un şarj kısmı |

**Yükleme (kullanıcı isteği, üç parça):**
- *Dolum hızı:* `chargeRate`'teki taban hız (bugün Tutuş'tan).
- *Hareket halinde dolum:* Bugün koşan oyuncu enerjisinin yalnız `chgMin` kadarını aktarıyor; durmak en hızlı dolum. Yükleme yüksek oyuncuda koşarken kaybedilen pay küçülür. Bu, şarjın oyuncuyu durmaya itmesini de azaltır (durma sorunuyla ilgili).
- *Pasla gelen topta kayıp:* Bugün pas şarjı düşürüyor (`recvCh`: en az `ch × 0,7`, boşta karşılayan daha çok korur). Yükleme yüksek alıcıda kırılım küçülür. Böylece hızlı pas oyunu da şarjlı oynanabilir: şarj ile pas arasındaki gerilim oyuncu seçimine bağlanır.

### Zihinsel (Okuma dörde ayrılır)
| # | Özellik | Ne yapar | Bugün nereden |
|---|---|---|---|
| 11 | **Tepki** | Pası, yön değişimini, fake'i ne kadar çabuk görür; pres tepkisi; yanılgıdan dönüş | Okuma (`ALAN_OKC(p,'tepki')` kanalı zaten var) |
| 12 | **Görüş** | Seçenekleri ne kadar doğru tartar: algı hatası (`errOk`), kafadaki maçın isabeti, tek dokunuş eşiği, kenardan pas eşiği | Okuma |
| 13 | **Yer alma** | Topsuz oyunda boşluğu bulma, açılan alanı fark etme (`vacOk0`, aday tartımının yenilenme sıklığı) | Okuma |
| 14 | **Soğukkanlılık** | Baskı altında pas ve göndermenin ne kadar az dağıldığı (`pasEk`, `gonEk`'in baskı payı) | yeni; Cesaret'in yerine |

### Savunma
| # | Özellik | Ne yapar | Bugün nereden |
|---|---|---|---|
| 15 | **Kesme** | İkilide topu alma, pas arası | Kesme; pas arasındaki Okuma payı Tepki'ye geçer |

### Bekçi'ye özel (Bekçi gerçek bir Bekçi kalsın)
| | Özellik | Ne yapar |
|---|---|---|
| B1 | **Refleks** | Dalış, uzanma (`bekR`) |
| B2 | **Yerleşim** | Açıortayda doğru durma, çıkış zamanı (`bekci`) |
| B3 | **Alan hakimiyeti** | Kuyu alanına gelen topu toplama |

Bekçi ayrıca Pas, Kontrol ve Tepki'yi kullanır; saha özelliklerinin geri kalanı Bekçi'de gösterilmez.

### Taraf (puan değil, özellik)
Çekirdek gövdenin bir yanında taşınıyor (`coreSide`, `corePos`). **Taraf**, oyuncunun topu taşımayı tercih ettiği yan (sol / sağ / iki yan): o yanda dönüş ve fiske daha hızlı, öbür yanda biraz yavaş; iki yanlı oyuncu ikisinde de ortalama. Kanatta oyuncu seçimine gerçek bir fark katar. Bu bir ödünleşim olduğu için kural 3'ün tek istisnası.

### Kalkan
**Cesaret.** Kişilik özelliği; rolsüz sistemde davranışı menajerin talimatı belirler, ikisi çakışır.

## 3 · Geçiş (eski kayıtlar ve kadro verisi)

İlk değerler eskilerden: Okuma → Tepki, Görüş, Yer alma; Aktarım → Pas, Gönderme; Tutuş → Kontrol, Yükleme; Yoğunluk → Güç, Uzanma; Soğukkanlılık ve Alan gücü 10; Taraf kadroda elle (yoksa sağ). Bekçi'nin üç özelliği eski Okuma ve Tutuş'tan. `HEPSI_20` kullanıcı kararıyla şimdilik açık kalır.

## 4 · Bağlama tablosu (Design yazar)

`alan-v5/OZELLIKLER.md`: her özellik için motorda hangi fonksiyonu, hangi formülle beslediği (dosya, aranacak kod, katsayı). Ölçüm ve ileride değişiklik bu tabloya göre yapılır. Bir mekanik iki özellikten besleniyorsa tabloda açıkça yazılır.

## 5 · Ölçüm (Claude Code, Design'ın zip'inden sonra)

Ölçümde `HEPSI_20` yerine ölçüm aracı kendi değerlerini verir (oyundaki ayar değişmez).
1. **Etki ve tek yönlülük:** Her özellik tek tek, takımın bütün saha oyuncularında 10'dan 6 ve 14'e (kopyalı ve birleşik ölçümle). Etkisi var mı; yükseldikçe hiç kötüleşiyor mu?
2. **Sahada görünürlük:** Özelliğin değeri kayıttan ayırt edilebiliyor mu (hızlı oyuncu ölçülen hızda, iyi pasör pas isabetinde öne çıkıyor mu)?
3. **Taktiğe göre değer (çeşitlilik şartı):** Aynı özellik farklı taktiklerde ne kadar değer üretiyor (Hız kontrada, Görüş ve Yükleme sabırlı kurulumda, Alan gücü düşük blokta daha mı değerli)? Kadro derinliğinin anlamı buna bağlı.
4. **Ağırlık dengesi:** Hiçbir özellik tek başına sonucu belirlemesin; hiçbiri etkisiz kalmasın.
5. **Hız:** Kontrol paketi.

## 6 · Ekran

Oyuncu kartında dört grup (Fiziksel, Topla, Zihinsel, Savunma) ve Bekçi'de kendi grubu; Taraf, kartta simge olarak. Rol cümlesi (Oyuncular tablosu) ileride talimatla birlikte özelliklerden de beslenebilir ("hızlı, arkaya koşan").
