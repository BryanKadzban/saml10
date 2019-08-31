#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u8, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u8, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
#[doc = "Reader of field `DRP`"]
pub type DRP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRP`"]
pub struct DRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DRP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TrustRAM Readout Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Remanence Prevention Ended Interrupt Enable"]
    #[inline(always)]
    pub fn drp(&self) -> DRP_R {
        DRP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TrustRAM Readout Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 1 - Data Remanence Prevention Ended Interrupt Enable"]
    #[inline(always)]
    pub fn drp(&mut self) -> DRP_W {
        DRP_W { w: self }
    }
}
