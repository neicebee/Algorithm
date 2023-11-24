//! # Art
//! 
//! 미술품을 모델링하기 위한 라이브러리

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// RYB 색상 모델에 따른 주 색상
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB 색상 모델에 따른 보조 색상
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 두 개의 주 색상을 조합해서
    /// 보조 색상을 생성한다.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // -- 생략 --
    }
}