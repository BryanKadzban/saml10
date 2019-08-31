#[doc = "Reader of register SEQCTRL[%s]"]
pub type R = crate::R<u8, super::SEQCTRL>;
#[doc = "Writer for register SEQCTRL[%s]"]
pub type W = crate::W<u8, super::SEQCTRL>;
#[doc = "Register SEQCTRL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `SEQSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSEL_A {
    #[doc = "Sequential logic is disabled"]
    DISABLE,
    #[doc = "D flip flop"]
    DFF,
    #[doc = "JK flip flop"]
    JK,
    #[doc = "D latch"]
    LATCH,
    #[doc = "RS latch"]
    RS,
}
impl crate::ToBits<u8> for SEQSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SEQSEL_A::DISABLE => 0,
            SEQSEL_A::DFF => 1,
            SEQSEL_A::JK => 2,
            SEQSEL_A::LATCH => 3,
            SEQSEL_A::RS => 4,
        }
    }
}
#[doc = "Reader of field `SEQSEL`"]
pub type SEQSEL_R = crate::R<u8, SEQSEL_A>;
impl SEQSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEQSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEQSEL_A::DISABLE),
            1 => Val(SEQSEL_A::DFF),
            2 => Val(SEQSEL_A::JK),
            3 => Val(SEQSEL_A::LATCH),
            4 => Val(SEQSEL_A::RS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEQSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DFF`"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == SEQSEL_A::DFF
    }
    #[doc = "Checks if the value of the field is `JK`"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == SEQSEL_A::JK
    }
    #[doc = "Checks if the value of the field is `LATCH`"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == SEQSEL_A::LATCH
    }
    #[doc = "Checks if the value of the field is `RS`"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == SEQSEL_A::RS
    }
}
#[doc = "Write proxy for field `SEQSEL`"]
pub struct SEQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEQSEL_A::DISABLE)
    }
    #[doc = "D flip flop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut W {
        self.variant(SEQSEL_A::DFF)
    }
    #[doc = "JK flip flop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut W {
        self.variant(SEQSEL_A::JK)
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut W {
        self.variant(SEQSEL_A::LATCH)
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut W {
        self.variant(SEQSEL_A::RS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline(always)]
    pub fn seqsel(&self) -> SEQSEL_R {
        SEQSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline(always)]
    pub fn seqsel(&mut self) -> SEQSEL_W {
        SEQSEL_W { w: self }
    }
}
