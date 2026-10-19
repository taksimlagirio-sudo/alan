// Taktik ölçümlerinin ayar tablosu (taktik sayfasının tac() alanları). taktik-olc.js ve taktik-iz.js kullanır.
// taktik sayfasının varsayılanı (VARS → tac())
const V = { sistem: 'Alan', tuzak: null, pres: 1, siddet: .5, blok: 'Orta', genislik: 'Normal', tempo: .55, risk: .5, gecisH: 'ustun', kpYer: 'rakip', kpSure: 2, pasSurme: .5, sabir: .5, sut: .5, tetik: ['Geri pas', 'Kanatta'] }, VD = { sav: '2-2-2', huc: '2-1-3' };
const AYAR = {
  varsayilan: {},
  pres0: { pres: 0 }, pres3: { pres: 3 }, siddet0: { siddet: 0 }, siddet1: { siddet: 1 },
  blokDusuk: { blok: 'Düşük' }, blokYuksek: { blok: 'Yüksek' }, genDar: { genislik: 'Dar' }, genGenis: { genislik: 'Geniş' },
  tempo01: { tempo: .1 }, tempo09: { tempo: .9 }, risk01: { risk: .1 }, risk09: { risk: .9 },
  pasSurme01: { pasSurme: .1 }, pasSurme09: { pasSurme: .9 }, sabir01: { sabir: .1 }, sabir09: { sabir: .9 }, sut01: { sut: .1 }, sut09: { sut: .9 },
  gecisHic: { gecisH: 'hic' }, gecisHer: { gecisH: 'her' }, kpHic: { kpYer: 'hic' }, kpHer: { kpYer: 'her' }, kpSure1: { kpSure: 1 }, kpSure3: { kpSure: 3 },
  kenar: { sistem: 'Kenara sıkıştır', tuzak: 'kenar' }, tetikYok: { tetik: [] }, tetikHepsi: { tetik: ['Geri pas', 'Kanatta', 'Zayıf kontrol', 'Arkası dönük', 'Uzun pas havada'] },
  diz321: { _diz: { sav: '3-2-1', huc: '2-2-2' } }, diz123: { _diz: { sav: '1-2-3', huc: '1-2-3' } },
  // hazır tarzlar (bileşim)
  T_kale: { blok: 'Düşük', pres: 0, siddet: .3, tempo: .3, risk: .2, gecisH: 'ustun', kpYer: 'hic', sabir: .7, _diz: { sav: '3-2-1', huc: '2-2-2' } },
  T_kontra: { blok: 'Düşük', pres: 1, tempo: .9, risk: .8, gecisH: 'her', kpYer: 'hic', sabir: .2, pasSurme: .3, _diz: { sav: '2-3-1', huc: '1-2-3' } },
  T_sahip: { blok: 'Orta', pres: 1, tempo: .2, risk: .2, sabir: .9, pasSurme: .2, gecisH: 'hic', kpYer: 'rakip', genislik: 'Geniş', _diz: { sav: '2-3-1', huc: '2-2-2' } },
  T_presci: { blok: 'Yüksek', pres: 3, siddet: .9, tempo: .7, risk: .6, gecisH: 'esit', kpYer: 'her', kpSure: 3, tetik: ['Geri pas', 'Kanatta', 'Zayıf kontrol', 'Arkası dönük', 'Uzun pas havada'], _diz: { sav: '2-2-2', huc: '2-1-3' } },
  T_dikine: { blok: 'Orta', pres: 1, tempo: .8, risk: .8, sabir: .2, pasSurme: .2, sut: .8, gecisH: 'ustun', _diz: { sav: '2-2-2', huc: '1-2-3' } },
  T_kanat: { sistem: 'Kenara sıkıştır', tuzak: 'kenar', blok: 'Orta', pres: 2, genislik: 'Geniş', tempo: .5, risk: .5, pasSurme: .7, _diz: { sav: '2-2-2', huc: '2-1-3' } }
};
module.exports = { V, VD, AYAR };
