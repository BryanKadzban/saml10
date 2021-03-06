#[doc = "Reader of register CRCCHKSUM"]
pub type R = crate::R<u32, super::CRCCHKSUM>;
#[doc = "Writer for register CRCCHKSUM"]
pub type W = crate::W<u32, super::CRCCHKSUM>;
#[doc = "Register CRCCHKSUM `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCCHKSUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRCCHKSUM`"]
pub type CRCCHKSUM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRCCHKSUM`"]
pub struct CRCCHKSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCCHKSUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Checksum"]
    #[inline(always)]
    pub fn crcchksum(&self) -> CRCCHKSUM_R {
        CRCCHKSUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Checksum"]
    #[inline(always)]
    pub fn crcchksum(&mut self) -> CRCCHKSUM_W {
        CRCCHKSUM_W { w: self }
    }
}
