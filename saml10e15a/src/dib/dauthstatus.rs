#[doc = "Reader of register DAUTHSTATUS"]
pub type R = crate::R<u32, super::DAUTHSTATUS>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SID_A {
    #[doc = "0: Security Extension not implemented"]
    NOSEC = 0,
    #[doc = "2: Secure invasive debug prohibited"]
    NO = 2,
    #[doc = "3: Secure invasive debug allowed"]
    YES = 3,
}
impl From<SID_A> for u8 {
    #[inline(always)]
    fn from(variant: SID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SID`"]
pub type SID_R = crate::R<u8, SID_A>;
impl SID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SID_A::NOSEC),
            2 => Val(SID_A::NO),
            3 => Val(SID_A::YES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSEC`"]
    #[inline(always)]
    pub fn is_nosec(&self) -> bool {
        *self == SID_A::NOSEC
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SID_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SID_A::YES
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SNID_A {
    #[doc = "0: Security Extension not implemented"]
    NOSEC = 0,
    #[doc = "2: Secure non-invasive debug prohibited"]
    NO = 2,
    #[doc = "3: Secure non-invasive debug allowed"]
    YES = 3,
}
impl From<SNID_A> for u8 {
    #[inline(always)]
    fn from(variant: SNID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SNID`"]
pub type SNID_R = crate::R<u8, SNID_A>;
impl SNID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SNID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SNID_A::NOSEC),
            2 => Val(SNID_A::NO),
            3 => Val(SNID_A::YES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSEC`"]
    #[inline(always)]
    pub fn is_nosec(&self) -> bool {
        *self == SNID_A::NOSEC
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SNID_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SNID_A::YES
    }
}
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn snid(&self) -> SNID_R {
        SNID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
