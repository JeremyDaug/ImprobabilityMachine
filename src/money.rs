pub(crate) fn to_lsd(pence: f64) -> String {
    let l = (pence / 240.0).floor();
    let s = (pence % 240.0 / 12.0).floor();
    let d = (pence % 12.0).floor();
    let f = (pence.fract() * 4.0).floor();
    format!("£{} {}s {}d {}/4 f", l, s, d, f)
}