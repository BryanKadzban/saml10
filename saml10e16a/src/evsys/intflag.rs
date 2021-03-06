#[doc = "Reader of register INTFLAG"]
pub type R = crate::R<u8, super::INTFLAG>;
#[doc = "Writer for register INTFLAG"]
pub type W = crate::W<u8, super::INTFLAG>;
#[doc = "Register INTFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NSCHK`"]
pub type NSCHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSCHK`"]
pub struct NSCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> NSCHK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non-Secure Check"]
    #[inline(always)]
    pub fn nschk(&self) -> NSCHK_R {
        NSCHK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Secure Check"]
    #[inline(always)]
    pub fn nschk(&mut self) -> NSCHK_W {
        NSCHK_W { w: self }
    }
}
