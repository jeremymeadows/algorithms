#![feature(once_cell)]

use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::{One, Pow, Zero};
use std::lazy::SyncLazy;
use std::ops::{Add, AddAssign, Mul};

use sha::{sha512::Sha512, Digest, Sha};

// base field: 2**255 - 19
static P: SyncLazy<BigInt> = SyncLazy::new(|| BigInt::from(2).pow(255u8) - 19);

// curve constant
static D: SyncLazy<BigInt> =
    SyncLazy::new(|| BigInt::from(121666).modpow(&(P.clone() - 2), &P) - 1);

// group order
static Q: SyncLazy<BigInt> =
    SyncLazy::new(|| BigInt::from(2).pow(252u8) + 0x14def9dea2f79cd65812631a5cf5d3edu128);

// base point
static G: SyncLazy<Point> = SyncLazy::new(|| {
    let y = (BigInt::from(4) * modp_inv(&BigInt::from(5))).mod_floor(&P);
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
    x.modpow(&(&*P - 2), &P)
}

#[inline]
fn sha512_modq(bytes: &[u8]) -> BigInt {
    BigInt::from_bytes_le(Sign::Plus, &Sha512::hash(bytes).as_bytes()).mod_floor(&Q)
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
                let a = ((&self.y - &self.x) * (&other.y - &other.x)).mod_floor(&P);
                let b = ((self.y + self.x) * (&other.y + &other.x)).mod_floor(&P);
                let c = (BigInt::from(2) * self.t * &other.t * &*D).mod_floor(&P);
                let d = (BigInt::from(2) * self.z * &other.z).mod_floor(&P);

                let e = &b - &a;
                let f = &d - &c;
                let g = d + c;
                let h = b + a;

                Self {
                    x: &e * &f,
                    y: &g * &h,
                    z: f * g,
                    t: e * h,
                }
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

macro_rules! impl_point_mul {
    ($t:ty) => {
        impl Mul<$t> for Point {
            type Output = Point;

            fn mul(mut self, s: $t) -> Self::Output {
                let mut other = Point::new();
                let mut s = s.clone();

                while s > BigInt::zero() {
                    if &s & BigInt::one() != BigInt::zero() {
                        other += self.clone();
                    }
                    self += self.clone();
                    s = s >> 1;
                }

                other
            }
        }
    };
}

impl_point_mul!(BigInt);
impl_point_mul!(&BigInt);

fn recover_x(y: &BigInt, sign: Sign) -> Option<BigInt> {
    if y >= &*P {
        return None;
    }

    let y2 = y * y;
    let x2: BigInt = (&y2 - 1) * modp_inv(&(&*D * y2 + 1));

    if x2 == BigInt::zero() {
        if sign == Sign::Minus {
            return None;
        } else {
            return Some(BigInt::zero());
        }
    }

    let mut x = x2.modpow(&BigInt::from((P.clone() + 3) / 8), &P);
    if ((&x * &x) - &x2).mod_floor(&*P) != BigInt::zero() {
        x = (&x * BigInt::from(2).modpow(&((P.clone() - 1) / 4), &P)).mod_floor(&P);
    }
    if (&x * &x - &x2).mod_floor(&P) != BigInt::zero() {
        return None;
    }

    if (x.iter_u32_digits().next().unwrap() & 1) as u8 != if let Sign::Minus = sign { 1 } else { 0 }
    {
        x = P.clone() - x;
    }
    Some(x)
}

fn point_compress(p: &Point) -> Vec<u8> {
    let z_inv = modp_inv(&p.z);
    let x = (&p.x * &z_inv).mod_floor(&P);
    let y = (&p.y * &z_inv).mod_floor(&P);

    (y | ((x.clone() & BigInt::one()) << 255u8))
        .iter_u32_digits()
        .flat_map(|e| e.to_le_bytes())
        .collect()
}

fn point_decompress(sign: Sign, bytes: &[u8]) -> Result<Point, &str> {
    if bytes.len() != 32 {
        return Err("Invalid point length for decompression");
    }
    let mut y = BigInt::from_bytes_le(sign, bytes);
    let sign = &y >> 255;
    y &= (BigInt::one() << 255) - 1;

    let x = recover_x(
        &y,
        if sign > BigInt::zero() {
            Sign::Minus
        } else {
            Sign::Plus
        },
    );
    if x.is_none() {
        Err("Invalid point")
    } else {
        Ok(Point {
            x: x.clone().unwrap(),
            y: y.clone(),
            z: BigInt::one(),
            t: (x.unwrap() * y).mod_floor(&P),
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

pub fn secret_to_public(secret: &[u8; 32]) -> Vec<u8> {
    let (a, _) = secret_expand(secret);
    point_compress(&(G.clone() * a))
}

pub fn sign(secret: &[u8; 32], message: &[u8]) -> Vec<u8> {
    let (secret, prefix) = secret_expand(&secret);
    let a = point_compress(&(G.clone() * &secret));

    let r = sha512_modq(&[&prefix, message].concat());
    let mut sig = point_compress(&(G.clone() * &r));
    let hash = sha512_modq(&[&sig, &a, message].concat());

    let s = (r + (hash * secret)).mod_floor(&Q);
    sig.append(&mut s.to_bytes_le().1[..32].to_owned());
    sig
}

pub fn verify(public: &[u8], msg: &[u8], signature: &[u8]) -> bool {
    if public.len() != 32 {
        panic!("bad public key length");
    }
    if signature.len() != 64 {
        panic!("bad signature length");
    }

    let a = match point_decompress(Sign::Plus, public) {
        Ok(x) => x,
        Err(_) => return false,
    };

    let sig = &signature[..32];
    let r = match point_decompress(Sign::Plus, &sig) {
        Ok(x) => x,
        Err(_) => return false,
    };

    let s = BigInt::from_bytes_le(Sign::Plus, &signature[32..]);
    if s >= *Q {
        return false;
    }

    let h = a * &sha512_modq(&[&sig, public, msg].concat());
    let p = G.clone() * &s;
    let q = h + r;

    ((&p.x * &q.z) - (&q.x * &p.z).mod_floor(&P)) | ((&p.y * &q.z) - (&q.y * &p.z).mod_floor(&P))
        != BigInt::zero()
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
        0x34, 0xd2, 0x65, 0x79, 0xdb, 0xb4, 0x56, 0x69, 0x3e, 0x54, 0x06, 0x72, 0xcf, 0x92, 0x2f,
        0x52, 0xdd, 0xe0, 0xd6, 0x53, 0x2e, 0x35, 0xbf, 0x06, 0xbe, 0x01, 0x3a, 0x7c, 0x53, 0x2f,
        0x20, 0xe0,
    ];

    static MESSAGE: &[u8] = b"Hello, world!";

    static SIGNATURE: &[u8] = &[
        0x98, 0x3c, 0x71, 0x7a, 0x1a, 0x92, 0xc7, 0x80, 0x04, 0x71, 0x7b, 0x80, 0x3a, 0xe4, 0xa0,
        0xde, 0xe7, 0x1a, 0xe2, 0x60, 0x7a, 0xfe, 0xc4, 0xa8, 0xbd, 0x76, 0xee, 0x7a, 0x8f, 0xa8,
        0x3d, 0x54, 0xf6, 0xac, 0xc1, 0x48, 0x84, 0xa4, 0xb2, 0xba, 0xea, 0x60, 0xf8, 0x61, 0x00,
        0x15, 0xef, 0x71, 0x17, 0xe2, 0xdf, 0x17, 0x53, 0xb5, 0xf4, 0xe6, 0x03, 0xb5, 0x57, 0xef,
        0x8b, 0xc2, 0xd8, 0x08,
    ];

    #[test]
    fn point_add() {
        assert_eq!(
            Point {
                x: BigInt::from_str("-296018569523652896372234514327709550206908554287152058624460862838859047041084022801327206127550044265769833862414983365876661076483822655155324979117350").unwrap(),
                y: BigInt::from_str("1490495321877056891969153563203901731061912970822103855338626923326930450369546055809989262026333609603664456667855626969875010168057827705323189208562226").unwrap(),
                z: BigInt::from_str("-472268759938110625622824785352915578001432258900265237190564369304728646195033505047516790369922900318679953813930015786830629522119352625245844870504700").unwrap(),
                t: BigInt::from_str("934244079836156744814377949647390897099953823983631381324122244125069081065138972390096358496487455289554769813310888065787613665947803319614067579747813").unwrap(),
            } + Point { x: BigInt::zero(), y: BigInt::one(), z: BigInt::one(), t: BigInt::zero() },
            Point {
                x: BigInt::from_str("-96720069842460291797938870517886859962485740142320070423613995671740497718858311798775495828584144407663446509877413375795620722700305351282730487751411").unwrap(),
                y: BigInt::from_str("2445976220003203967096891697700621578629252374160621852127530226297368368906708146503108862027047256192963442974854511196621032246716669431448327848201179").unwrap(),
                z: BigInt::from_str("548709829761699606984552933499151766797648929301152071422878665786736660760034763091932596573004173478455124607409578148995930233169982001819914108226761").unwrap(),
                t: BigInt::from_str("-431147717791841962355427150511426056796771870726164855195374766048111118495319433478986200978311134275491162180364095474437485097374411865782309907037529").unwrap(),
            }
        );
    }

    #[test]
    fn point_mul() {
        assert_eq!(
            G.clone() * &BigInt::from(100),
            Point {
                x: BigInt::from_str("33852737548248494013141095506843493074257898291746417836258079774863462698885584432776666791975147136439327306698681712345266581993070911157421796574392").unwrap(),
                y: BigInt::from_str("4933664688387354798079233450641357380582637490575459763024405674820024408175145867466438050215676652569619965866954994164396318484909508300117122109977768").unwrap(),
                z: BigInt::from_str("298007774811682781711699325678270570070529777868294235640799607929585520633355270542731125044403282799964471860363725081162554308270158992985887435988288").unwrap(),
                t: BigInt::from_str("560448652564787756707595398430077284252610085855547619377841883246490124725226610873473814998175430223647426293659331058614256611811547084416685858372087").unwrap(),
            }
        );
    }

    #[test]
    fn test_secret_expand() {
        assert_eq!(
            secret_expand(&SECRET),
            (
                BigInt::from_str(
                    "48060031659963821445853372622794355987034352078359660699171973346756570217744"
                )
                .unwrap(),
                [
                    0xf0, 0x4f, 0x2a, 0x0d, 0xd6, 0x92, 0x15, 0xba, 0xd4, 0x9b, 0x46, 0x53, 0x7d,
                    0x9b, 0xfa, 0x8e, 0x30, 0xe0, 0x54, 0x0b, 0x03, 0xbc, 0x1e, 0xef, 0x61, 0xbd,
                    0x13, 0x7d, 0x80, 0x8b, 0x7d, 0xbb,
                ]
                .to_vec(),
            )
        )
    }

    #[test]
    fn test_point_compress() {
        assert_eq!(
            point_compress(&G),
            [
                0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
                0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
                0x66, 0x66, 0x66, 0x66,
            ],
        );
    }

    // #[test]
    // fn test_point_decompress() {
    //     todo!()
    //     // assert_eq!(
    //     //     point_compress(&G).1,
    //     //     [
    //     //         0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
    //     //         0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
    //     //         0x66, 0x66, 0x66, 0x66,
    //     //     ],
    //     // );
    // }

    #[test]
    fn test_secret_to_public() {
        assert_eq!(secret_to_public(&SECRET), PUBLIC);
    }

    #[test]
    fn test_sign() {
        assert_eq!(sign(&SECRET, MESSAGE), SIGNATURE);
    }

    #[test]
    fn test_verify() {
        assert!(verify(&PUBLIC, &MESSAGE, &sign(&SECRET, MESSAGE)));
    }
}
