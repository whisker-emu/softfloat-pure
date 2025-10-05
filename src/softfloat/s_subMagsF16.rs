use crate::softfloat::softfloat_propagateNaNF16;

use super::{
    defaultNaNF16UI, expF16UI, float16_t, fracF16UI, packToF16, packToF16UI, signF16UI,
    softfloat_countLeadingZeros16, softfloat_countLeadingZeros32, softfloat_flag_inexact,
    softfloat_flag_invalid, softfloat_propagateNaNF16UI, softfloat_roundPackToF16,
    softfloat_round_max, softfloat_round_min, softfloat_round_minMag, softfloat_round_near_even,
    softfloat_round_odd,
};

#[must_use]
pub const fn softfloat_subMagsF16(
    uiA: u16,
    uiB: u16,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    let mut expA = expF16UI(uiA);
    let mut sigA = fracF16UI(uiA);
    let mut expB = expF16UI(uiB);
    let mut sigB = fracF16UI(uiB);
    // ------------------------------------------------------------------------
    let mut expDiff = expA.wrapping_sub(expB);
    if expDiff == 0 {
        // --------------------------------------------------------------------
        if expA == 0x1F {
            if (sigA | sigB) != 0 {
                return softfloat_propagateNaNF16(uiA, uiB);
            }
            return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
        }
        let mut sigDiff = (sigA as i16).wrapping_sub(sigB as i16);
        if sigDiff == 0 {
            return (packToF16(roundingMode == softfloat_round_min, 0, 0), 0);
        }
        if expA != 0 {
            expA = expA.wrapping_sub(1);
        }
        let mut signZ = signF16UI(uiA);
        if sigDiff < 0 {
            signZ = !signZ;
            sigDiff = sigDiff.wrapping_neg();
        }
        let mut shiftDist = (softfloat_countLeadingZeros16(sigDiff as u16) as i8).wrapping_sub(5);
        let mut expZ = (expA as i8).wrapping_sub(shiftDist);
        if expZ < 0 {
            shiftDist = expA as i8;
            expZ = 0;
        }
        return (packToF16(signZ, expZ, (sigDiff << shiftDist) as u16), 0);
    }
    // --------------------------------------------------------------------
    let mut signZ = signF16UI(uiA);
    let mut expZ: i8;
    let mut sigX: u16;
    let mut sigY: u16;
    if expDiff < 0 {
        // ----------------------------------------------------------------
        signZ = !signZ;
        if expB == 0x1F {
            if sigB != 0 {
                return softfloat_propagateNaNF16(uiA, uiB);
            }
            return (packToF16(signZ, 0x1F, 0), 0);
        }
        if expDiff <= -13 {
            let uiZ = packToF16UI(signZ, expB, sigB);
            if (expA | (sigA as i8)) != 0 {
                // --------------------------------------------------------
                let mut flags = softfloat_flag_inexact;
                let mut result_uiZ = uiZ;
                if roundingMode != softfloat_round_near_even {
                    if roundingMode == softfloat_round_minMag
                        || (roundingMode
                            == if signF16UI(result_uiZ) {
                                softfloat_round_max
                            } else {
                                softfloat_round_min
                            })
                    {
                        result_uiZ = result_uiZ.wrapping_sub(1);
                    } else if roundingMode == softfloat_round_odd {
                        result_uiZ = (result_uiZ.wrapping_sub(1)) | 1;
                    }
                }
                return (float16_t { v: result_uiZ }, flags);
            }
            return (float16_t { v: uiZ }, 0);
        }
        expZ = (expA as i8) + 19;
        sigX = sigB | 0x0400;
        sigY = sigA.wrapping_add(if expA != 0 { 0x0400 } else { sigA });
        expDiff = expDiff.wrapping_neg();
    }
    // --------------------------------------------------------------------
    let uiZ = uiA;
    if expA == 0x1F {
        if sigA != 0 {
            return softfloat_propagateNaNF16(uiA, uiB);
        }
        return (float16_t { v: uiZ }, 0);
    }
    if 13 <= expDiff {
        if (expB | (sigB as i8)) != 0 {
            // ------------------------------------------------------------
            let mut flags = softfloat_flag_inexact;
            let mut result_uiZ = uiZ;
            if roundingMode != softfloat_round_near_even {
                if roundingMode == softfloat_round_minMag
                    || (roundingMode
                        == if signF16UI(result_uiZ) {
                            softfloat_round_max
                        } else {
                            softfloat_round_min
                        })
                {
                    result_uiZ = result_uiZ.wrapping_sub(1);
                } else if roundingMode == softfloat_round_odd {
                    result_uiZ = (result_uiZ.wrapping_sub(1)) | 1;
                }
            }
            return (float16_t { v: result_uiZ }, flags);
        }
        return (float16_t { v: uiZ }, 0);
    }
    expZ = (expB as i8) + 19;
    sigX = sigA | 0x0400;
    sigY = sigB.wrapping_add(if expB != 0 { 0x0400 } else { sigB });
    let mut sig32Z = ((sigX as u32) << expDiff).wrapping_sub(sigY as u32);
    let shiftDist = (softfloat_countLeadingZeros32(sig32Z) as i8).wrapping_sub(1);
    sig32Z <<= shiftDist;
    let expZ = expZ.wrapping_sub(shiftDist);
    let mut sigZ = (sig32Z >> 16) as u16;
    if (sig32Z & 0xFFFF) != 0 {
        sigZ |= 1;
        return softfloat_roundPackToF16(signZ, expZ as i16, sigZ, roundingMode, detectTininess);
    }
    if (sigZ & 0xF) == 0 && ((expZ as u32) < 0x1E) {
        sigZ >>= 4;
        return (packToF16(signZ, expZ, sigZ), 0);
    }
    return softfloat_roundPackToF16(signZ, expZ as i16, sigZ, roundingMode, detectTininess);
}
