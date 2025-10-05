use super::{
    defaultNaNF16UI, expF16UI, float16_t, fracF16UI, packToF16, packToF16UI, signF16UI,
    softfloat_countLeadingZeros32, softfloat_flag_invalid, softfloat_normSubnormalF16Sig,
    softfloat_propagateNaNF16, softfloat_propagateNaNF16UI, softfloat_roundPackToF16,
    softfloat_round_min, softfloat_shiftRightJam32,
};

pub const softfloat_mulAdd_subProd: u8 = 2;
pub const softfloat_mulAdd_subC: u8 = 1;

#[inline]
const fn propagateNaN_ZC(uiZ: u16, uiC: u16) -> (float16_t, u8) {
    softfloat_propagateNaNF16(uiZ, uiC)
}

#[inline]
const fn propagateNaN_ABC(uiA: u16, uiB: u16, uiC: u16) -> (float16_t, u8) {
    let (uiZ, flags) = softfloat_propagateNaNF16UI(uiA, uiB);
    let (res, new_flags) = propagateNaN_ZC(uiZ, uiC);
    return (res, flags | new_flags);
}

#[inline]
const fn infProdArg(
    magBits: u16,
    signProd: bool,
    expC: i8,
    sigC: u16,
    signC: bool,
    uiC: u16,
) -> (float16_t, u8) {
    if magBits != 0 {
        let uiZ = packToF16UI(signProd, 0x1F, 0);
        if expC != 0x1F {
            return (float16_t { v: uiZ }, 0);
        }
        if sigC != 0 {
            return propagateNaN_ZC(uiZ, uiC);
        }
        if signProd == signC {
            return (float16_t { v: uiZ }, 0);
        }
    }
    let (res, flags) = propagateNaN_ZC(defaultNaNF16UI, uiC);
    return (res, flags | softfloat_flag_invalid);
}

#[inline]
const fn completeCancellation(roundingMode: u8) -> float16_t {
    packToF16((roundingMode == softfloat_round_min), 0, 0)
}

#[inline]
const fn zeroProd(
    uiC: u16,
    expC: i8,
    sigC: u16,
    signProd: bool,
    signC: bool,
    roundingMode: u8,
) -> float16_t {
    if ((expC as u16) | sigC) == 0 && (signProd != signC) {
        return completeCancellation(roundingMode);
    }
    return float16_t { v: uiC };
}

#[must_use]
pub const fn softfloat_mulAddF16(
    uiA: u16,
    uiB: u16,
    uiC: u16,
    op: u8,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    let signA = signF16UI(uiA);
    let mut expA = expF16UI(uiA);
    let mut sigA = fracF16UI(uiA);
    let signB = signF16UI(uiB);
    let mut expB = expF16UI(uiB);
    let mut sigB = fracF16UI(uiB);
    let signC = signF16UI(uiC) ^ (op == softfloat_mulAdd_subC);
    let mut expC = expF16UI(uiC);
    let mut sigC = fracF16UI(uiC);
    let signProd = signA ^ signB ^ (op == softfloat_mulAdd_subProd);

    if expA == 0x1F {
        if sigA != 0 || ((expB == 0x1F) && (sigB != 0)) {
            return propagateNaN_ABC(uiA, uiB, uiC);
        }
        return infProdArg((expB as u16) | sigB, signProd, expC, sigC, signC, uiC);
    }
    if expB == 0x1F {
        if sigB != 0 {
            return propagateNaN_ABC(uiA, uiB, uiC);
        }
        return infProdArg((expA as u16) | sigA, signProd, expC, sigC, signC, uiC);
    }
    if expC == 0x1F {
        if sigC != 0 {
            return propagateNaN_ZC(0, uiC);
        }
        return (float16_t { v: uiC }, 0);
    }

    if expA == 0 {
        if sigA == 0 {
            return (zeroProd(uiC, expC, sigC, signProd, signC, roundingMode), 0);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    if expB == 0 {
        if sigB == 0 {
            return (zeroProd(uiC, expC, sigC, signProd, signC, roundingMode), 0);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }

    let mut expProd = expA.wrapping_add(expB).wrapping_sub(0xE);
    sigA = (sigA | 0x0400) << 4;
    sigB = (sigB | 0x0400) << 4;
    let mut sigProd = (sigA as u32).wrapping_mul(sigB as u32);
    if sigProd < 0x2000_0000 {
        expProd = expProd.wrapping_sub(1);
        sigProd <<= 1;
    }

    let mut signZ = signProd;
    if expC == 0 {
        if sigC == 0 {
            let expZ = expProd.wrapping_sub(1);
            let sigZ = (sigProd >> 15) as u16 | ((sigProd & 0x7FFF) != 0) as u16;
            return softfloat_roundPackToF16(
                signZ,
                expZ as i16,
                sigZ,
                roundingMode,
                detectTininess,
            );
        }
        let normExpSig = softfloat_normSubnormalF16Sig(sigC);
        expC = normExpSig.exp;
        sigC = normExpSig.sig;
    }
    sigC = (sigC | 0x0400) << 3;

    let expDiff = expProd - expC;
    let mut sig32Z: u32;
    let mut sigZ: u16;
    let mut expZ: i8;
    if signProd == signC {
        if expDiff <= 0 {
            expZ = expC;
            sigZ = (sigC as u32).wrapping_add(softfloat_shiftRightJam32(
                sigProd,
                (16_i8).wrapping_sub(expDiff) as u16,
            )) as u16;
        } else {
            expZ = expProd;
            sig32Z = sigProd.wrapping_add(softfloat_shiftRightJam32(
                (sigC as u32) << 16,
                expDiff as u16,
            ));
            sigZ = (sig32Z >> 16) as u16 | ((sig32Z & 0xFFFF) != 0) as u16;
        }
        if sigZ < 0x4000 {
            expZ = expZ.wrapping_sub(1);
            sigZ <<= 1;
        }
    } else {
        let sig32C = (sigC as u32) << 16;
        if expDiff < 0 {
            signZ = signC;
            expZ = expC;
            sig32Z = sig32C.wrapping_sub(softfloat_shiftRightJam32(
                sigProd,
                expDiff.wrapping_neg() as u16,
            ));
        } else if expDiff == 0 {
            expZ = expProd;
            sig32Z = sigProd.wrapping_sub(sig32C);
            if sig32Z == 0 {
                return (packToF16((roundingMode == softfloat_round_min), 0, 0), 0);
            }
            if (sig32Z & 0x8000_0000) != 0 {
                signZ = !signZ;
                sig32Z = sig32Z.wrapping_neg();
            }
        } else {
            expZ = expProd;
            sig32Z = sigProd.wrapping_sub(softfloat_shiftRightJam32(sig32C, expDiff as u16));
        }
        let mut shiftDist: i8 = softfloat_countLeadingZeros32(sig32Z).wrapping_sub(1) as i8;
        expZ = expZ.wrapping_sub(shiftDist);
        shiftDist = shiftDist.wrapping_sub(16);
        if shiftDist < 0 {
            sigZ = (sig32Z >> shiftDist.wrapping_neg()) as u16
                | ((sig32Z << (shiftDist & 31)) != 0) as u16;
        } else {
            sigZ = (sig32Z << shiftDist) as u16;
        }
    }

    return softfloat_roundPackToF16(signZ, expZ as i16, sigZ, roundingMode, detectTininess);
}
