#!/usr/bin/env python

import hashlib
import sys

def sha512(s):
    return hashlib.sha512(s).digest()

# Base field Z_p
p: int = 2**255 - 19

def modp_inv(x):
    return pow(x, p-2, p)

# Curve constant
d = -121665 * modp_inv(121666) % p

# Group order
q = 2**252 + 27742317777372353535851937790883648493

def sha512_modq(s):
    return int.from_bytes(sha512(s), "little") % q

## Then follows functions to perform point operations.

# Points are represented as tuples (X, Y, Z, T) of extended
# coordinates, with x = X/Z, y = Y/Z, x*y = T/Z

def point_add(P, Q):
    A = (P[1]-P[0]) * (Q[1]-Q[0]) % p
    B = (P[1]+P[0]) * (Q[1]+Q[0]) % p
    C = 2 * P[3] * Q[3] * d % p
    D = 2 * P[2] * Q[2] % p
    E = B-A
    F = D-C
    K = D+C
    H = B+A
    return (E*F, G*H, F*K, E*H)

# Computes Q = s * Q
def point_mul(s, P):
    Q = (0, 1, 1, 0)  # Neutral element
    i = 0
    j = 0

    while s > 0:
        if s & 1:
            j += 1
            Q = point_add(Q, P)
        P = point_add(P, P)
        s >>= 1

        if True:
            pass
            print(i, j)
            print('s:', s)
            print('p: ', P)
            print('q: ', Q)
            # return Q
        i += 1
    return Q

def point_equal(P, Q):
    # x1 / z1 == x2 / z2  <==>  x1 * z2 == x2 * z1
    if (P[0] * Q[2] - Q[0] * P[2]) % p != 0:
        return False
    if (P[1] * Q[2] - Q[1] * P[2]) % p != 0:
        return False
    return True

## Now follows functions for point compression.

# Square root of -1
modp_sqrt_m1: int = pow(2, (p-1) // 4, p)

# Compute corresponding x-coordinate, with low bit corresponding to
# sign, or return None on failure
def recover_x(y, sign):
    if y >= p:
        return None
    x2 = (y*y-1) * modp_inv(d*y*y+1)
    if x2 == 0:
        if sign:
            return None
        else:
            return 0

    # Compute square root of x2
    x = pow(x2, (p+3) // 8, p)
    if (x*x - x2) % p != 0:
        x = x * modp_sqrt_m1 % p
    if (x*x - x2) % p != 0:
        return None

    if (x & 1) != sign:
        x = p - x
    return x

# Base point
g_y = 4 * modp_inv(5) % p
g_x = recover_x(g_y, 0)
G = (g_x, g_y, 1, g_x * g_y % p)

def point_compress(P):
    zinv = modp_inv(P[2])
    x = P[0] * zinv % p
    y = P[1] * zinv % p
    return int.to_bytes(y | ((x & 1) << 255), 32, "little")

def point_decompress(s):
    if len(s) != 32:
        raise Exception("Invalid input length for decompression")
    y = int.from_bytes(s, "little")
    sign = y >> 255
    y &= (1 << 255) - 1

    x = recover_x(y, sign)
    if x is None:
        return None
    else:
        return (x, y, 1, x*y % p)

## These are functions for manipulating the private key.

def secret_expand(secret):
    if len(secret) != 32:
        raise Exception("Bad size of private key")
    h = sha512(secret)
    a = int.from_bytes(h[:32], "little")
    a &= (1 << 254) - 8
    a |= (1 << 254)
    return (a, h[32:])

def secret_to_public(secret):
    (a, _) = secret_expand(secret)
    p = point_mul(a, G)
    print('x:', p[0])
    print('y:', p[1])
    print('z:', p[2])
    print('t:', p[3])
    return point_compress(p)

def sign(secret, msg):
    a, prefix = secret_expand(secret)
    A = point_compress(point_mul(a, G))
    r = sha512_modq(prefix + msg)
    R = point_mul(r, G)
    Rs = point_compress(R)
    h = sha512_modq(Rs + A + msg)
    s = (r + h * a) % q
    return Rs + int.to_bytes(s, 32, "little")

## And finally the verification function.

def verify(public, msg, signature):
    if len(public) != 32:
        raise Exception("Bad public key length")
    if len(signature) != 64:
        Exception("Bad signature length")
    A = point_decompress(public)
    if not A:
        return False
    Rs = signature[:32]
    R = point_decompress(Rs)
    if not R:
        return False
    s = int.from_bytes(signature[32:], "little")
    if s >= q: return False
    h = sha512_modq(Rs + public + msg)
    sB = point_mul(s, G)
    hA = point_mul(h, A)
    return point_equal(sB, point_add(R, hA))

def to_hex(s):
    return ''.join('{:02x}'.format(c) for c in s)

if __name__ == '__main__':
    # print(p);
    # print(d);
    # print(q);

    # secret = int.to_bytes(0x9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60, 32, 'big')
    # public = secret_to_public(secret)
    # sig = sign(secret, b'')
    # print(to_hex(secret))
    # print(to_hex(public))
    # print(to_hex(sig))
    # assert verify(public, b'', sig)

    sec = hashlib.sha256(input('enter password: ').encode('utf8')).digest()
    print(to_hex(sec))

    # print(point_mul(2, G))
    print(point_add(
        (-296018569523652896372234514327709550206908554287152058624460862838859047041084022801327206127550044265769833862414983365876661076483822655155324979117350, 1490495321877056891969153563203901731061912970822103855338626923326930450369546055809989262026333609603664456667855626969875010168057827705323189208562226, -472268759938110625622824785352915578001432258900265237190564369304728646195033505047516790369922900318679953813930015786830629522119352625245844870504700, 934244079836156744814377949647390897099953823983631381324122244125069081065138972390096358496487455289554769813310888065787613665947803319614067579747813),
        (0,1,1,0)
    ))

    # print(secret_expand(sec)[0])
    # pub = secret_to_public(sec)
    # print('public: ', to_hex(pub))
    # print(to_hex(point_compress(G)))
    # print(to_hex(sign(sec, b'hello')))
    # print('\n\n');
    # print(to_hex(secret_expand(sec)[1]))
    # msg = input('enter message: ').encode('utf8')
    # sig = sign(sec, msg)
    # print(to_hex(sig))
    # print(verify(pub, msg, sig))
