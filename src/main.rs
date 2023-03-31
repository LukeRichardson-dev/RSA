use std::{str::FromStr, ops::Sub, io::stdin};
use maths::mod_pow;
use rug::Integer;

mod maths;

fn main() {
    
    let p = Integer::from_str("518331276497664627061425292753").unwrap();
    let q = Integer::from_str("462850225141647002812927382281").unwrap();
    
    println!("{:?}", gcd_extended(p.clone(), q.clone()));

    let key = KeyPair::generate(p, q);

    println!("{} {} {}", key.d, key.e, key.n);

    let plain: Integer = Integer::from_str("1234").unwrap();
    let enc = key.encrypt(plain);
    println!("{}", enc);
    let dec = key.decrypt(enc);
    println!("{}", dec);

    let c = key.encrypt_bytes("TcCTCCtcCJVHV".as_bytes());
    let e = key.decrypt_bytes(&c);
    
    println!("{:?}", c);
    println!("{:?}", String::from_utf8(e).unwrap());

    let kb = stdin();

    let key = KeyPair {
        e: 65537.into(),
        d: 0.into(),
        n: Integer::from_str("14684193580969157562812512177154929103599082221947469405671231948964666490885593698291546935441512932729314113186427410993167873112317705959760785621666915749099039251138172978079782018244223325561973").unwrap(),
    };

    loop {
        let mut buf = "".to_owned();
        kb.read_line(&mut buf).unwrap();

        let cipher = key.encrypt_bytes(buf.as_bytes());
        println!("{:?}", cipher);
    }
}

fn gcd_extended(mut a: Integer, mut n: Integer) -> (Integer, Integer, Integer) {
    let mut q;
    let (mut x0, mut x1, mut y0, mut y1) = (
        Integer::from(1),
        Integer::from(0),
        Integer::from(0),
        Integer::from(1),
    );
    while n != 0 {
        (q, a, n) = (Integer::from(&a / &n), n.clone(), (a % n).into());
        (x1, x0) = (x0 - &q * &x1, x1);
        (y1, y0) = (y0 - &q * &y1, y1);
    }

    (a, x0, y0)
}

struct KeyPair {
    d: Integer,
    e: Integer,
    n: Integer,
} 

impl KeyPair {
    pub fn generate(p: Integer, q: Integer) -> Self {
        let n: Integer = (&p * &q).into();
        println!("{:?}", n);

        let phi_n: Integer = Integer::product([
            p.clone().sub(Integer::from(1)), 
            q.sub(Integer::from(1)),
        ].iter()).into();

        let d: Integer = Integer::from_str("65537").unwrap();
        // TODO verify d is coprime to n
        let (_, e1, e2) = gcd_extended(d.clone(), phi_n);

        Self { d, e: e1.max(e2), n }
    }

    pub fn encrypt(&self, x: Integer) -> Integer {
        mod_pow(&x, &self.e, &self.n)
    }

    pub fn decrypt(&self, x: Integer) -> Integer {
        mod_pow(&x, &self.d, &self.n)
    }

    pub fn encrypt_bytes(&self, data: &[u8]) -> Vec<Integer> {
        data.iter() 
            .map(|&x| self.encrypt(x.into()))
            .collect()
    }

    pub fn decrypt_bytes(&self, data: &[Integer]) -> Vec<u8> {
        data.iter()
            .map(|x| self.decrypt(x.into()).to_u8_wrapping())
            .collect()
    }

}

#[cfg(test)]
mod test {
    use rug::Integer;
    use crate::maths::mod_pow;

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(&7.into(), &11.into(), &12.into()), 7);
        assert_eq!(
            mod_pow(&403038.into(), &82365.into(), &2727.into()),
            Integer::from(403038).pow_mod(&82365.into(), &2727.into()).unwrap(),
        );
        assert_eq!(
            mod_pow(&12741.into(), &3948753.into(), &34.into()),
            Integer::from(12741).pow_mod(&3948753.into(), &34.into()).unwrap(),
        );
        assert_eq!(
            mod_pow(&1209.into(), &1907.into(), &2.into()),
            Integer::from(1209).pow_mod(&1907.into(), &2.into()).unwrap(),
        );
    }
}