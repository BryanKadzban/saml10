#[doc = "Reader of register CRCCTRL"]
pub type R = crate::R<u16, super::CRCCTRL>;
#[doc = "Writer for register CRCCTRL"]
pub type W = crate::W<u16, super::CRCCTRL>;
#[doc = "Register CRCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CRC Beat Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCBEATSIZE_A {
    #[doc = "0: 8-bit bus transfer"]
    BYTE = 0,
    #[doc = "1: 16-bit bus transfer"]
    HWORD = 1,
    #[doc = "2: 32-bit bus transfer"]
    WORD = 2,
}
impl From<CRCBEATSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCBEATSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CRCBEATSIZE`"]
pub type CRCBEATSIZE_R = crate::R<u8, CRCBEATSIZE_A>;
impl CRCBEATSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CRCBEATSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CRCBEATSIZE_A::BYTE),
            1 => Val(CRCBEATSIZE_A::HWORD),
            2 => Val(CRCBEATSIZE_A::WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == CRCBEATSIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HWORD`"]
    #[inline(always)]
    pub fn is_hword(&self) -> bool {
        *self == CRCBEATSIZE_A::HWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == CRCBEATSIZE_A::WORD
    }
}
#[doc = "Write proxy for field `CRCBEATSIZE`"]
pub struct CRCBEATSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCBEATSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCBEATSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit bus transfer"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::BYTE)
    }
    #[doc = "16-bit bus transfer"]
    #[inline(always)]
    pub fn hword(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::HWORD)
    }
    #[doc = "32-bit bus transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "CRC Polynomial Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCPOLY_A {
    #[doc = "0: CRC-16 (CRC-CCITT)"]
    CRC16 = 0,
    #[doc = "1: CRC32 (IEEE 802.3)"]
    CRC32 = 1,
}
impl From<CRCPOLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCPOLY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CRCPOLY`"]
pub type CRCPOLY_R = crate::R<u8, CRCPOLY_A>;
impl CRCPOLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CRCPOLY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CRCPOLY_A::CRC16),
            1 => Val(CRCPOLY_A::CRC32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC16`"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == CRCPOLY_A::CRC16
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == CRCPOLY_A::CRC32
    }
}
#[doc = "Write proxy for field `CRCPOLY`"]
pub struct CRCPOLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCPOLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCPOLY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CRC-16 (CRC-CCITT)"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut W {
        self.variant(CRCPOLY_A::CRC16)
    }
    #[doc = "CRC32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut W {
        self.variant(CRCPOLY_A::CRC32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "CRC Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCSRC_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: I/O interface"]
    IO = 1,
    #[doc = "32: DMA channel 0"]
    CHN0 = 32,
    #[doc = "33: DMA channel 1"]
    CHN1 = 33,
    #[doc = "34: DMA channel 2"]
    CHN2 = 34,
    #[doc = "35: DMA channel 3"]
    CHN3 = 35,
    #[doc = "36: DMA channel 4"]
    CHN4 = 36,
    #[doc = "37: DMA channel 5"]
    CHN5 = 37,
    #[doc = "38: DMA channel 6"]
    CHN6 = 38,
    #[doc = "39: DMA channel 7"]
    CHN7 = 39,
}
impl From<CRCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CRCSRC`"]
pub type CRCSRC_R = crate::R<u8, CRCSRC_A>;
impl CRCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CRCSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CRCSRC_A::NOACT),
            1 => Val(CRCSRC_A::IO),
            32 => Val(CRCSRC_A::CHN0),
            33 => Val(CRCSRC_A::CHN1),
            34 => Val(CRCSRC_A::CHN2),
            35 => Val(CRCSRC_A::CHN3),
            36 => Val(CRCSRC_A::CHN4),
            37 => Val(CRCSRC_A::CHN5),
            38 => Val(CRCSRC_A::CHN6),
            39 => Val(CRCSRC_A::CHN7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == CRCSRC_A::NOACT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == CRCSRC_A::IO
    }
    #[doc = "Checks if the value of the field is `CHN0`"]
    #[inline(always)]
    pub fn is_chn0(&self) -> bool {
        *self == CRCSRC_A::CHN0
    }
    #[doc = "Checks if the value of the field is `CHN1`"]
    #[inline(always)]
    pub fn is_chn1(&self) -> bool {
        *self == CRCSRC_A::CHN1
    }
    #[doc = "Checks if the value of the field is `CHN2`"]
    #[inline(always)]
    pub fn is_chn2(&self) -> bool {
        *self == CRCSRC_A::CHN2
    }
    #[doc = "Checks if the value of the field is `CHN3`"]
    #[inline(always)]
    pub fn is_chn3(&self) -> bool {
        *self == CRCSRC_A::CHN3
    }
    #[doc = "Checks if the value of the field is `CHN4`"]
    #[inline(always)]
    pub fn is_chn4(&self) -> bool {
        *self == CRCSRC_A::CHN4
    }
    #[doc = "Checks if the value of the field is `CHN5`"]
    #[inline(always)]
    pub fn is_chn5(&self) -> bool {
        *self == CRCSRC_A::CHN5
    }
    #[doc = "Checks if the value of the field is `CHN6`"]
    #[inline(always)]
    pub fn is_chn6(&self) -> bool {
        *self == CRCSRC_A::CHN6
    }
    #[doc = "Checks if the value of the field is `CHN7`"]
    #[inline(always)]
    pub fn is_chn7(&self) -> bool {
        *self == CRCSRC_A::CHN7
    }
}
#[doc = "Write proxy for field `CRCSRC`"]
pub struct CRCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(CRCSRC_A::NOACT)
    }
    #[doc = "I/O interface"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(CRCSRC_A::IO)
    }
    #[doc = "DMA channel 0"]
    #[inline(always)]
    pub fn chn0(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN0)
    }
    #[doc = "DMA channel 1"]
    #[inline(always)]
    pub fn chn1(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN1)
    }
    #[doc = "DMA channel 2"]
    #[inline(always)]
    pub fn chn2(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN2)
    }
    #[doc = "DMA channel 3"]
    #[inline(always)]
    pub fn chn3(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN3)
    }
    #[doc = "DMA channel 4"]
    #[inline(always)]
    pub fn chn4(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN4)
    }
    #[doc = "DMA channel 5"]
    #[inline(always)]
    pub fn chn5(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN5)
    }
    #[doc = "DMA channel 6"]
    #[inline(always)]
    pub fn chn6(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN6)
    }
    #[doc = "DMA channel 7"]
    #[inline(always)]
    pub fn chn7(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    pub fn crcbeatsize(&self) -> CRCBEATSIZE_R {
        CRCBEATSIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    pub fn crcsrc(&self) -> CRCSRC_R {
        CRCSRC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    pub fn crcbeatsize(&mut self) -> CRCBEATSIZE_W {
        CRCBEATSIZE_W { w: self }
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W {
        CRCPOLY_W { w: self }
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    pub fn crcsrc(&mut self) -> CRCSRC_W {
        CRCSRC_W { w: self }
    }
}
