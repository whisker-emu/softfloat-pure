use crate::softfloat::softfloat_propagateNaNF16;

use super::{
    expF16UI, float16_t, fracF16UI, packToF16, packToF16UI, signF16UI, softfloat_flag_inexact,
    softfloat_flag_overflow, softfloat_propagateNaNF16UI, softfloat_roundPackToF16,
    softfloat_round_max, softfloat_round_min, softfloat_round_near_even, softfloat_round_odd,
};

#[must_use]
pub const fn softfloat_addMagsF16(
    uiA: u16,
    uiB: u16,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    let expA = expF16UI(uiA);
    let sigA = fracF16UI(uiA);
    let expB = expF16UI(uiB);
    let sigB = fracF16UI(uiB);
    let expDiff = expA.wrapping_sub(expB);
    if expDiff == 0 {
        if expA == 0 {
            let uiZ = uiA.wrapping_add(sigB);
            return (float16_t { v: uiZ }, 0);
        }
        if expA == 0x1F {
            if (sigA | sigB) != 0 {
                return softfloat_propagateNaNF16(uiA, uiB);
            }
            return (float16_t { v: uiA }, 0);
        }
        let signZ = signF16UI(uiA);
        let expZ = expA;
        let mut sigZ = 0x0800_u16.wrapping_add(sigA).wrapping_add(sigB);
        if (sigZ & 1) == 0 && (expZ < 0x1E) {
            sigZ >>= 1;
            return (packToF16(signZ, expZ, sigZ), 0);
        }
        sigZ <<= 3;
        return softfloat_roundPackToF16(signZ, expZ as i16, sigZ, roundingMode, detectTininess);
    }
    let signZ = signF16UI(uiA);
    let expZ: i8;
    let sigX: u16;
    let sigY: u16;
    let shiftDist: i8;
    if expDiff < 0 {
        if expB == 0x1F {
            if sigB != 0 {
                return softfloat_propagateNaNF16(uiA, uiB);
            }
            let uiZ = packToF16UI(signZ, 0x1F, 0);
            return (float16_t { v: uiZ }, 0);
        }
        if expDiff <= -13 {
            let uiZ = packToF16UI(signZ, expB, sigB);
            if (expA | (sigA as i8)) != 0 {
                let mut flags = softfloat_flag_inexact;
                let mut result_uiZ = uiZ;
                if roundingMode != softfloat_round_near_even {
                    if roundingMode
                        == if signF16UI(result_uiZ) {
                            softfloat_round_min
                        } else {
                            softfloat_round_max
                        }
                    {
                        result_uiZ = result_uiZ.wrapping_add(1);
                        if ((result_uiZ << 1) as u16) == 0xF800 {
                            flags |= softfloat_flag_overflow;
                        }
                    } else if roundingMode == softfloat_round_odd {
                        result_uiZ |= 1;
                    }
                }
                return (float16_t { v: result_uiZ }, flags);
            }
            return (float16_t { v: uiZ }, 0);
        }
        expZ = expB;
        sigX = sigB | 0x0400;
        sigY = sigA.wrapping_add(if expA != 0 { 0x0400 } else { sigA });
        shiftDist = 19 + expDiff;
    } else {
        let uiZ = uiA;
        if expA == 0x1F {
            if sigA != 0 {
                return softfloat_propagateNaNF16(uiA, uiB);
            }
            return (float16_t { v: uiZ }, 0);
        }
        if 13 <= expDiff {
            if (expB | (sigB as i8)) != 0 {
                let mut flags = softfloat_flag_inexact;
                let mut result_uiZ = uiZ;
                if roundingMode != softfloat_round_near_even {
                    if roundingMode
                        == if signF16UI(result_uiZ) {
                            softfloat_round_min
                        } else {
                            softfloat_round_max
                        }
                    {
                        result_uiZ = result_uiZ.wrapping_add(1);
                        if ((result_uiZ << 1) as u16) == 0xF800 {
                            flags |= softfloat_flag_overflow;
                        }
                    } else if roundingMode == softfloat_round_odd {
                        result_uiZ |= 1;
                    }
                }
                return (float16_t { v: result_uiZ }, flags);
            }
            return (float16_t { v: uiZ }, 0);
        }
        expZ = expA;
        sigX = sigA | 0x0400;
        sigY = sigB.wrapping_add(if expB != 0 { 0x0400 } else { sigB });
        shiftDist = 19 - expDiff;
    }
    let mut sig32Z = ((sigX as u32) << 19).wrapping_add((sigY as u32) << shiftDist);
    if sig32Z < 0x4000_0000 {
        let expZ = expZ.wrapping_sub(1);
        sig32Z <<= 1;
        let sigZ = (sig32Z >> 16) as u16;
        if (sig32Z & 0xFFFF) != 0 {
            let sigZ = sigZ | 1;
            return softfloat_roundPackToF16(
                signZ,
                expZ as i16,
                sigZ,
                roundingMode,
                detectTininess,
            );
        }
        if (sigZ & 0xF) == 0 && (expZ < 0x1E) {
            let sigZ = sigZ >> 4;
            return (packToF16(signZ, expZ, sigZ), 0);
        }
        return softfloat_roundPackToF16(signZ, expZ as i16, sigZ, roundingMode, detectTininess);
    }
    let sigZ = (sig32Z >> 16) as u16;
    if (sig32Z & 0xFFFF) != 0 {
        let sigZ = sigZ | 1;
        return softfloat_roundPackToF16(signZ, expZ as i16, sigZ, roundingMode, detectTininess);
    }
    if (sigZ & 0xF) == 0 && (expZ < 0x1E) {
        let sigZ = sigZ >> 4;
        return (packToF16(signZ, expZ, sigZ), 0);
    }
    return softfloat_roundPackToF16(signZ, expZ as i16, sigZ, roundingMode, detectTininess);
}
