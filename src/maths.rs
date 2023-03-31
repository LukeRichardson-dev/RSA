use rug::Integer;

pub fn mod_pow(x: &Integer, e: &Integer, m: &Integer) -> Integer {
    let mut r: Integer = 1.into();
    let mut e = e.clone();
    let mut y = x.clone();
    let m = m.clone();

    while e > 0 {
        if e.clone() % 2 == 1 {
            r = (r * &y) % m.clone();   
        }

        y = Integer::from(&y * &y) % m.clone();
        e = e.div_exact_u(2);
    }

    r
}