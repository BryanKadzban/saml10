#[doc = "Writer for register DSCC"]
pub type W = crate::W<u32, super::DSCC>;
#[doc = "Register DSCC `reset()`'s with value 0"]
impl crate::ResetValue for super::DSCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DSCKEY`"]
pub struct DSCKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> DSCKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:29 - Data Scramble Key"]
    #[inline(always)]
    pub fn dsckey(&mut self) -> DSCKEY_W {
        DSCKEY_W { w: self }
    }
}
