#[doc = "Reader of register DPIDR2"]
pub type R = crate::R<u32, super::DPIDR2>;
#[doc = "Reader of field `DES_1`"]
pub type DES_1_R = crate::R<u8, u8>;
#[doc = "Reader of field `JEDEC`"]
pub type JEDEC_R = crate::R<bool, bool>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - JEP106 identification code bits\\[6:4\\]"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - JEDEC assignee value is used"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Component revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
