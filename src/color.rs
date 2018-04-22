#[derive(Debug, PartialEq, Clone)]
pub struct HSV(pub f64, pub f64, pub f64);

/// data structure representing a RGB color value
#[derive(Debug, PartialEq, Clone)]
pub struct RGB(pub f64, pub f64, pub f64);

/// convert a hsv color representations into a rgb representation
fn hsv2rgb(hsv: &HSV) -> RGB {
    // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB
    let &HSV(h, s, v) = hsv;
    let hi = (h * 6.).floor() as u32;
    let f = h * 6. - hi as f64;
    let p = v*(1.-s);
    let q = v*(1.-s*f);
    let t = v*(1.-s*(1.-f));

    match hi {
        0 | 6 => RGB(v, t, p),
        1 => RGB(q, v, p),
        2 => RGB(p, v, t),
        3 => RGB(p, q, v),
        4 => RGB(t, p, v),
        5 => RGB(v, p, q),
        _ => RGB(0., 0., 0.)
    }
}

impl HSV {
    /// convert `HSV` into `RGB`
    pub fn to_rgb(&self) -> RGB {
        hsv2rgb(self)
    }
}
