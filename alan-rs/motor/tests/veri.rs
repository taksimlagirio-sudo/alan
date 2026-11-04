use alan_motor::veri::*;
#[test]
fn tablolar_yuklenir() {
    let s = sut_tablosu(); assert_eq!(s.cells.len(), 1120); assert!(s.surum.starts_with("v5-22"));
    let i = imkan_sut(); assert_eq!(i.parmak, "104b9sy"); assert!(!i.v.is_empty());
    let d = deger_tablosu(); assert_eq!(d.cells.len(), d.nb * d.ng);
}
