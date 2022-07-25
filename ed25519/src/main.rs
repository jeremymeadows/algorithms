#![allow(soft_unstable)]
#![feature(once_cell)]

use num_bigint::{BigInt, Sign};
use num_traits::{One, Pow, Zero};
use std::lazy::SyncLazy;
use std::ops::{Add, AddAssign, Mul};

use sha::{sha256::Sha256, sha512::Sha512, Digest, Sha};

static P: SyncLazy<BigInt> = SyncLazy::new(|| BigInt::from(2).pow(255u8) - 19);
static D: SyncLazy<BigInt> =
    SyncLazy::new(|| BigInt::from(121666).modpow(&(P.clone() - 2), &P) - 1);
// static D: SyncLazy<BigInt> = SyncLazy::new(|| BigInt::from(-121665) * modp_inv(&BigInt::from(121666)) % &*P);
static Q: SyncLazy<BigInt> =
    SyncLazy::new(|| BigInt::from(2).pow(252u8) + 0x14def9dea2f79cd65812631a5cf5d3edu128);

static MODP_SQRT_M1: SyncLazy<BigInt> =
    SyncLazy::new(|| BigInt::from(2).modpow(&(P.clone() - 1), &P));

static G: SyncLazy<Point> = SyncLazy::new(|| {
    let y = BigInt::from(4) * modp_inv(&BigInt::from(5)) % &*P;
    let x = recover_x(&y, Sign::Plus).unwrap();

    Point {
        x: x.clone(),
        y: y.clone(),
        z: BigInt::one(),
        t: x * y % &*P,
    }
});

#[inline]
fn modp_inv(x: &BigInt) -> BigInt {
    x.modpow(&(&*P - 2), &*P)
}

#[inline]
fn sha512_modq(bytes: &[u8]) -> BigInt {
    BigInt::from_bytes_be(Sign::Plus, &Sha512::hash(bytes).as_bytes()) % &*Q
}

#[derive(Clone, Debug)]
struct Point {
    x: BigInt,
    y: BigInt,
    z: BigInt,
    t: BigInt,
}

impl Point {
    fn new() -> Self {
        Point {
            x: BigInt::zero(),
            y: BigInt::one(),
            z: BigInt::one(),
            t: BigInt::zero(),
        }
    }
}

macro_rules! impl_point_add {
    ($t:ty) => {
        impl Add<$t> for Point {
            type Output = Self;

            fn add(self, other: $t) -> Self::Output {
                let a = (&self.y - &self.x) * (&other.y - &other.x) % &*P;
                let b = (self.y + &self.x) * (&other.y + &other.x) % &*P;
    // a = (p[1] - p[0]) * (q[1] - q[0]) % P
    // b = (p[1] + p[0]) * (q[1] + q[0]) % P
                let c = 2 * self.t * &other.t * &*D % &*P;
                let d = 2 * self.z * &other.z % &*P;
    // c = 2 * p[3] * q[3] * D % P
    // d = 2 * p[2] * q[2] % P

                let e = &b - &a;
                let f = &d - &c;
                let g = d + c;
                let h = b + a;
    // e = b - a
    // f = d - c
    // g = d + c
    // h = b + a

                Self {
                    x: &e * &f,
                    y: &g * &h,
                    z: f * g,
                    t: e * h,
                }
    // return (e * f, g * h, f * g, e * h)
            }
        }

        impl AddAssign<$t> for Point {
            fn add_assign(&mut self, other: $t) {
                *self = self.clone() + other;
            }
        }
    };
}

impl_point_add!(Point);
impl_point_add!(&Point);

impl Mul<&BigInt> for Point {
    type Output = Point;

    fn mul(mut self, s: &BigInt) -> Self::Output {
        let mut other = Point::new();
        let mut s = s.clone();
        let mut i = 0;
        let mut j = 0;

        while s > BigInt::zero() {
            if &s & BigInt::one() != BigInt::zero() {
                j += 1;
                other = self.clone() + &other;
                // other += self.clone();
            }
            self += self.clone();
            s = s >> 1;

            if true {
                println!("{i}, {j}");
                println!("s: {:#?}", s);
                println!("p: {:#?}", self);
                println!("q: {:#?}", other);
                // return other;
            }
            i += 1
        }

        other
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        !(self.x.clone() * &other.z - &other.x * &self.z % &*P != BigInt::zero()
            || self.y.clone() * &other.z - &other.y * &self.z % &*P != BigInt::zero())
    }
}

impl Eq for Point {}

fn recover_x(y: &BigInt, sign: Sign) -> Option<BigInt> {
    if y >= &*P {
        return None;
    }

    let y2 = y * y;
    let x2: BigInt = (&y2 - 1) * modp_inv(&(&*D * &y2 + 1));

    if x2 == BigInt::zero() {
        if sign != Sign::Minus {
            return None;
        } else {
            return Some(BigInt::zero());
        }
    }

    let mut x = x2.modpow(&BigInt::from((P.clone() + 3) / 8), &*P);
    if (&x * &x - &x2) % &*P != BigInt::zero() {
        x = &x * &*MODP_SQRT_M1 % &*P;
    }
    if (&x * &x - &x2) % &*P != BigInt::zero() {
        return None;
    }

    if (x.iter_u32_digits().next().unwrap() & 1) as u8 != 0 {
        x = P.clone() - x;
    }
    Some(x)
}

fn point_compress(p: &Point) -> (Sign, Vec<u8>) {
    let z_inv = modp_inv(&p.z);
    let x = &p.x * &z_inv % &*P;
    let y = &p.y * &z_inv % &*P;

    (
        y.sign(),
        (y | ((x.clone() & BigInt::one()) << 255u8))
            .iter_u32_digits()
            .map(|e| e.to_le_bytes())
            .flatten()
            .collect(),
    )
}

fn point_decompress(sign: Sign, bytes: &[u8]) -> Result<Point, &str> {
    if bytes.len() != 32 {
        return Err("Invalid point length for decompression");
    }
    let y = BigInt::from_bytes_le(sign, bytes);
    // y &= BigInt::from((1 << 255) - 1);

    let x = recover_x(&y, sign);
    if x.is_none() {
        Err("Invalid point")
    } else {
        Ok(Point {
            x: x.clone().unwrap(),
            y: y.clone(),
            z: BigInt::one(),
            t: x.unwrap() * y % &*P,
        })
    }
}

fn secret_expand(secret: &[u8; 32]) -> (BigInt, Vec<u8>) {
    let h = Sha512::hash(secret);
    let mut a = BigInt::from_bytes_le(Sign::Plus, &h[0..32]);
    a &= (BigInt::from(1) << 254) - 8;
    a |= BigInt::from(1) << 254;

    (a, h[32..].to_vec())
}

fn secret_to_public(secret: &[u8; 32]) -> (Sign, Vec<u8>) {
    let (a, _) = secret_expand(secret);
    let p = G.clone() * &a;
    println!("{:#?}", p);
    point_compress(&p)
}

fn sign(secret: &[u8; 32], message: &[u8]) -> Vec<u8> {
    let (a, mut prefix) = secret_expand(&secret);
    prefix.append(&mut message.to_owned());
    let A = point_compress(&(G.clone() * &a)).1;
    let r = sha512_modq(&prefix);
    let R = G.clone() * &r;
    let mut Rs = point_compress(&R).1;
    Rs.append(&mut A.to_owned());
    Rs.append(&mut message.to_owned());
    let h = sha512_modq(&Rs);
    let s = (r + h + a) % &*Q;

    Rs.append(&mut s.to_bytes_le().1[..32].to_owned());
    Rs
}

// fn verify(public: &[u8], msg: &[u8], signature: &[u8]) -> bool {
//     if public.len() != 32 {
//         panic!("bad public key length");
//     }
//     if signature.len() != 64 {
//         panic!("bad signature length");
//     }

//     todo!()
// }

fn main() {
    // println!("{}", &*P);
    // println!("{}", &*D);
    // println!("{}", &*Q);

    println!("\nenter password: ");
    let mut sec = String::new();
    std::io::stdin().read_line(&mut sec).unwrap();
    let sec = Sha256::hash(sec.trim().as_bytes());

    println!("secret: {:02x?}", sec);
    // println!("{:x}", secret_expand(&sec).0);
    // pub = secret_to_public(sec)
    // print(to_hex(pub))
    let public = secret_to_public(&sec);
    println!("{:x?}", public.1);
    println!("\n{:x?}", sign(&sec, b"hello"));
    // msg = input('enter message: ').encode('utf8')
    // sig = sign(sec, msg)
    // print(to_hex(sig))
    // print(verify(pub, msg, sig))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    static SECRET: [u8; 32] = [
        0x2c, 0x26, 0xb4, 0x6b, 0x68, 0xff, 0xc6, 0x8f, 0xf9, 0x9b, 0x45, 0x3c, 0x1d, 0x30, 0x41,
        0x34, 0x13, 0x42, 0x2d, 0x70, 0x64, 0x83, 0xbf, 0xa0, 0xf9, 0x8a, 0x5e, 0x88, 0x62, 0x66,
        0xe7, 0xae,
    ];
    static PUBLIC: [u8; 32] = [
        0xeb, 0x27, 0x67, 0xc1, 0x37, 0xab, 0x7a, 0xd8, 0x27, 0x9c, 0x07, 0x8e, 0xff, 0x11, 0x6a,
        0xb0, 0x78, 0x6e, 0xad, 0x3a, 0x2e, 0x0f, 0x98, 0x9f, 0x72, 0xc3, 0x7f, 0x82, 0xf2, 0x96,
        0x96, 0x70,
    ];

    #[test]
    fn test_point_add() {
        fn point_add(p: (BigInt, BigInt, BigInt, BigInt), q: (BigInt, BigInt, BigInt, BigInt)) -> (BigInt, BigInt, BigInt, BigInt) {
            let a = (p.1.clone() - p.0.clone()) * (q.1.clone() - q.0.clone()) % &*P;
            let b = (p.1 + p.0) * (q.1 + q.0) % &*P;
            let c: BigInt = 2 * p.3 * q.3 * &*D % &*P;
            let d: BigInt = 2 * p.2 * q.2 % &*P;

            let e = b.clone() - a.clone();
            let f = d.clone() - c.clone();
            let g = d + c;
            let h = b + a;
            (e.clone() * f.clone(), g.clone() * h.clone(), f * g, e * h)
        }

        println!("{}\n{}", *P, *D);
        // println!("{:#?}", 
        //     Point { x: BigInt::from_str("-296018569523652896372234514327709550206908554287152058624460862838859047041084022801327206127550044265769833862414983365876661076483822655155324979117350").unwrap(), y: BigInt::from_str("1490495321877056891969153563203901731061912970822103855338626923326930450369546055809989262026333609603664456667855626969875010168057827705323189208562226").unwrap(), z: BigInt::from_str("-472268759938110625622824785352915578001432258900265237190564369304728646195033505047516790369922900318679953813930015786830629522119352625245844870504700").unwrap(), t: BigInt::from_str("934244079836156744814377949647390897099953823983631381324122244125069081065138972390096358496487455289554769813310888065787613665947803319614067579747813").unwrap() }
        //     + Point { x: BigInt::zero(), y: BigInt::one(), z: BigInt::one(), t: BigInt::zero() }
        // );
        println!("{:#?}", point_add(
            (
                BigInt::from_str("-296018569523652896372234514327709550206908554287152058624460862838859047041084022801327206127550044265769833862414983365876661076483822655155324979117350").unwrap(),
                BigInt::from_str("1490495321877056891969153563203901731061912970822103855338626923326930450369546055809989262026333609603664456667855626969875010168057827705323189208562226").unwrap(),
                BigInt::from_str("-472268759938110625622824785352915578001432258900265237190564369304728646195033505047516790369922900318679953813930015786830629522119352625245844870504700").unwrap(),
                BigInt::from_str("934244079836156744814377949647390897099953823983631381324122244125069081065138972390096358496487455289554769813310888065787613665947803319614067579747813").unwrap(),
            ),
            (BigInt::zero(), BigInt::one(), BigInt::one(), BigInt::zero())
        ));
        assert!(false);
    }

    #[test]
    fn test_point_mul() {
        println!("{:#?}", G.clone() * &BigInt::from(2));
        assert!(false);
    }

    #[test]
    fn test_secret_expand() {
        let expanded = secret_expand(&SECRET);

        assert_eq!(
            expanded.0,
            BigInt::from_str(
                "48060031659963821445853372622794355987034352078359660699171973346756570217744"
            )
            .unwrap(),
        );
        assert_eq!(
            expanded.1,
            [
                0xf0, 0x4f, 0x2a, 0x0d, 0xd6, 0x92, 0x15, 0xba, 0xd4, 0x9b, 0x46, 0x53, 0x7d, 0x9b,
                0xfa, 0x8e, 0x30, 0xe0, 0x54, 0x0b, 0x03, 0xbc, 0x1e, 0xef, 0x61, 0xbd, 0x13, 0x7d,
                0x80, 0x8b, 0x7d, 0xbb,
            ],
        )
    }

    #[test]
    fn test_point_compress() {
        assert_eq!(
            point_compress(&G).1,
            [
                0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
                0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
                0x66, 0x66, 0x66, 0x66,
            ],
        );
    }

    #[test]
    fn test_secret_to_public() {
        let public = secret_to_public(&SECRET);

        assert_eq!(public.1, PUBLIC);
    }

    #[test]
    fn test_sign() {
        let s = sign(&SECRET, b"hello");
        println!("{:02x?}", s);
        assert_eq!(
            s,
            [
                0x51, 0x1c, 0xa4, 0x97, 0xc4, 0xd4, 0x27, 0x0b, 0x09, 0x8b, 0x1a, 0xfd, 0x5a, 0xe4,
                0xe3, 0xb9, 0x51, 0xa5, 0xda, 0x2c, 0x9d, 0xa6, 0xe9, 0xc0, 0x52, 0x8f, 0x57, 0x61,
                0x88, 0x36, 0x76, 0xe7, 0xdf, 0x6e, 0x4c, 0x0f, 0x0e, 0x1b, 0x5a, 0x0a, 0x44, 0x44,
                0xf4, 0x29, 0x8b, 0x18, 0x82, 0xdd, 0x82, 0x2f, 0xb1, 0x13, 0x3c, 0xbd, 0x49, 0xab,
                0xfb, 0x99, 0x6c, 0x87, 0xcd, 0x5b, 0x85, 0x06,
            ],
        )
    }
}
