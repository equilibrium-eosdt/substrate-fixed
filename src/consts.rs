// Copyright © 2018–2019 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

/*!
This module contains constants.

The constants have the maximum precision possible for a fixed-point
number, and are rounded down at that precision.

# Examples

```rust
let pi: fixed::types::I8F8 = fixed::consts::PI.to_fixed();
println!("π with eight binary places is {:b}", pi);
assert_eq!(format!("{:b}", pi), "11.00100100");
```
*/

use crate::{
    frac::{U126, U127, U128},
    FixedU128,
};
use core::marker::PhantomData;

/// π = 3.141…
// PI = 3.141592653589793238462643383279502884197
pub const PI: FixedU128<U126> = FixedU128 {
    bits: 0xC90F_DAA2_2168_C234_C4C6_628B_80DC_1CD1,
    phantom: PhantomData,
};

/// π/2 = 1.570…
// FRAC_PI_2 = 1.570796326794896619231321691639751442099
pub const FRAC_PI_2: FixedU128<U127> = FixedU128 {
    bits: 0xC90F_DAA2_2168_C234_C4C6_628B_80DC_1CD1,
    phantom: PhantomData,
};

/// π/3 = 1.047…
// FRAC_PI_3 = 1.047197551196597746154214461093167628066
pub const FRAC_PI_3: FixedU128<U127> = FixedU128 {
    bits: 0x860A_91C1_6B9B_2C23_2DD9_9707_AB3D_688B,
    phantom: PhantomData,
};

/// π/4 = 0.7853…
// FRAC_PI_4 = 0.7853981633974483096156608458198757210493
pub const FRAC_PI_4: FixedU128<U128> = FixedU128 {
    bits: 0xC90F_DAA2_2168_C234_C4C6_628B_80DC_1CD1,
    phantom: PhantomData,
};

/// π/6 = 0.5235…
// FRAC_PI_6 = 0.5235987755982988730771072305465838140329
pub const FRAC_PI_6: FixedU128<U128> = FixedU128 {
    bits: 0x860A_91C1_6B9B_2C23_2DD9_9707_AB3D_688B,
    phantom: PhantomData,
};

/// π/8 = 0.3926…
// FRAC_PI_8 = 0.3926990816987241548078304229099378605246
pub const FRAC_PI_8: FixedU128<U128> = FixedU128 {
    bits: 0x6487_ED51_10B4_611A_6263_3145_C06E_0E68,
    phantom: PhantomData,
};

/// 1/π = 0.3183…
// FRAC_1_PI = 0.3183098861837906715377675267450287240689
pub const FRAC_1_PI: FixedU128<U128> = FixedU128 {
    bits: 0x517C_C1B7_2722_0A94_FE13_ABE8_FA9A_6EE0,
    phantom: PhantomData,
};

/// 2/π = 0.6366…
// FRAC_2_PI = 0.6366197723675813430755350534900574481378
pub const FRAC_2_PI: FixedU128<U128> = FixedU128 {
    bits: 0xA2F9_836E_4E44_1529_FC27_57D1_F534_DDC0,
    phantom: PhantomData,
};

/// 2/√π = 1.128…
// FRAC_2_SQRT_PI = 1.128379167095512573896158903121545171688
pub const FRAC_2_SQRT_PI: FixedU128<U127> = FixedU128 {
    bits: 0x906E_BA82_14DB_688D_71D4_8A7F_6BFE_C344,
    phantom: PhantomData,
};

/// √2 = 1.414…
// SQRT_2 = 1.414213562373095048801688724209698078570
pub const SQRT_2: FixedU128<U127> = FixedU128 {
    bits: 0xB504_F333_F9DE_6484_597D_89B3_754A_BE9F,
    phantom: PhantomData,
};

/// 1/√2 = 0.7071…
// FRAC_1_SQRT_2 = 0.7071067811865475244008443621048490392848
pub const FRAC_1_SQRT_2: FixedU128<U128> = FixedU128 {
    bits: 0xB504_F333_F9DE_6484_597D_89B3_754A_BE9F,
    phantom: PhantomData,
};

/// e = 2.718…
// E = 2.718281828459045235360287471352662497757
pub const E: FixedU128<U126> = FixedU128 {
    bits: 0xADF8_5458_A2BB_4A9A_AFDC_5620_273D_3CF1,
    phantom: PhantomData,
};

/// log<sub>2</sub> 10 = 3.321…
// LOG2_10 = 3.321928094887362347870319429489390175865
pub const LOG2_10: FixedU128<U126> = FixedU128 {
    bits: 0xD49A_784B_CD1B_8AFE_492B_F6FF_4DAF_DB4C,
    phantom: PhantomData,
};

/// log<sub>2</sub> e = 1.442…
// LOG2_E = 1.442695040888963407359924681001892137427
pub const LOG2_E: FixedU128<U127> = FixedU128 {
    bits: 0xB8AA_3B29_5C17_F0BB_BE87_FED0_691D_3E88,
    phantom: PhantomData,
};

/// log<sub>10</sub> 2 = 0.3010…
// LOG10_2 = 0.3010299956639811952137388947244930267682
pub const LOG10_2: FixedU128<U128> = FixedU128 {
    bits: 0x4D10_4D42_7DE7_FBCC_47C4_ACD6_05BE_48BC,
    phantom: PhantomData,
};

/// log<sub>10</sub> e = 0.4342…
// LOG10_E = 0.4342944819032518276511289189166050822944
pub const LOG10_E: FixedU128<U128> = FixedU128 {
    bits: 0x6F2D_EC54_9B94_38CA_9AAD_D557_D699_EE19,
    phantom: PhantomData,
};

/// ln 2 = 0.6931…
// LN_2 = 0.6931471805599453094172321214581765680755
pub const LN_2: FixedU128<U128> = FixedU128 {
    bits: 0xB172_17F7_D1CF_79AB_C9E3_B398_03F2_F6AF,
    phantom: PhantomData,
};

/// ln 10 = 2.302…
// LN_10 = 2.302585092994045684017991454684364207601
pub const LN_10: FixedU128<U126> = FixedU128 {
    bits: 0x935D_8DDD_AAA8_AC16_EA56_D62B_82D3_0A28,
    phantom: PhantomData,
};

#[cfg(test)]
mod tests {
    use crate::{consts::*, traits::FromFixed};
    use core::f64::consts as F;

    #[test]
    #[allow(clippy::float_cmp)]
    fn check() {
        assert_eq!(f64::from_fixed(PI), F::PI);
        assert_eq!(f64::from_fixed(FRAC_PI_2), F::FRAC_PI_2);
        assert_eq!(f64::from_fixed(FRAC_PI_3), F::FRAC_PI_3);
        assert_eq!(f64::from_fixed(FRAC_PI_4), F::FRAC_PI_4);
        assert_eq!(f64::from_fixed(FRAC_PI_6), F::FRAC_PI_6);
        assert_eq!(f64::from_fixed(FRAC_PI_8), F::FRAC_PI_8);
        assert_eq!(f64::from_fixed(FRAC_1_PI), F::FRAC_1_PI);
        assert_eq!(f64::from_fixed(FRAC_2_PI), F::FRAC_2_PI);
        assert_eq!(f64::from_fixed(FRAC_2_SQRT_PI), F::FRAC_2_SQRT_PI);
        assert_eq!(f64::from_fixed(SQRT_2), F::SQRT_2);
        assert_eq!(f64::from_fixed(FRAC_1_SQRT_2), F::FRAC_1_SQRT_2);
        assert_eq!(f64::from_fixed(E), F::E);
        // assert_eq!(f64::from_fixed(LOG2_10), F::LOG2_10);
        assert_eq!(f64::from_fixed(LOG2_E), F::LOG2_E);
        // assert_eq!(f64::from_fixed(LOG10_2), F::LOG10_2);
        assert_eq!(f64::from_fixed(LOG10_E), F::LOG10_E);
        assert_eq!(f64::from_fixed(LN_2), F::LN_2);
        assert_eq!(f64::from_fixed(LN_10), F::LN_10);
    }
}
