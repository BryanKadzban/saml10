#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u32, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u32, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "2: SPI slave mode"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI master mode"]
    SPI_MASTER = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(MODE_A::SPI_SLAVE),
            3 => Val(MODE_A::SPI_MASTER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODE_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODE_A::SPI_MASTER
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI slave mode"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODE_A::SPI_SLAVE)
    }
    #[doc = "SPI master mode"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODE_A::SPI_MASTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IBON`"]
pub type IBON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBON`"]
pub struct IBON_W<'a> {
    w: &'a mut W,
}
impl<'a> IBON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Data Out Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DOPO_A {
    #[doc = "0: DO (MOSI master mode, MISO slave mode) is PAD\\[0\\], SCK is PAD\\[1\\], SS in slave mode is PAD\\[2\\]"]
    CFG0 = 0,
    #[doc = "1: DO (MOSI master mode, MISO slave mode) is PAD\\[2\\], SCK is PAD\\[3\\], SS in slave mode is PAD\\[1\\]"]
    CFG1 = 1,
    #[doc = "2: DO (MOSI master mode, MISO slave mode) is PAD\\[3\\], SCK is PAD\\[1\\], SS in slave mode is PAD\\[2\\]"]
    CFG2 = 2,
    #[doc = "3: DO (MOSI master mode, MISO slave mode) is PAD\\[0\\], SCK is PAD\\[3\\], SS in slave mode is PAD\\[1\\]"]
    CFG3 = 3,
}
impl From<DOPO_A> for u8 {
    #[inline(always)]
    fn from(variant: DOPO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DOPO`"]
pub type DOPO_R = crate::R<u8, DOPO_A>;
impl DOPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOPO_A {
        match self.bits {
            0 => DOPO_A::CFG0,
            1 => DOPO_A::CFG1,
            2 => DOPO_A::CFG2,
            3 => DOPO_A::CFG3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CFG0`"]
    #[inline(always)]
    pub fn is_cfg0(&self) -> bool {
        *self == DOPO_A::CFG0
    }
    #[doc = "Checks if the value of the field is `CFG1`"]
    #[inline(always)]
    pub fn is_cfg1(&self) -> bool {
        *self == DOPO_A::CFG1
    }
    #[doc = "Checks if the value of the field is `CFG2`"]
    #[inline(always)]
    pub fn is_cfg2(&self) -> bool {
        *self == DOPO_A::CFG2
    }
    #[doc = "Checks if the value of the field is `CFG3`"]
    #[inline(always)]
    pub fn is_cfg3(&self) -> bool {
        *self == DOPO_A::CFG3
    }
}
#[doc = "Write proxy for field `DOPO`"]
pub struct DOPO_W<'a> {
    w: &'a mut W,
}
impl<'a> DOPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOPO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DO (MOSI master mode, MISO slave mode) is PAD\\[0\\], SCK is PAD\\[1\\], SS in slave mode is PAD\\[2\\]"]
    #[inline(always)]
    pub fn cfg0(self) -> &'a mut W {
        self.variant(DOPO_A::CFG0)
    }
    #[doc = "DO (MOSI master mode, MISO slave mode) is PAD\\[2\\], SCK is PAD\\[3\\], SS in slave mode is PAD\\[1\\]"]
    #[inline(always)]
    pub fn cfg1(self) -> &'a mut W {
        self.variant(DOPO_A::CFG1)
    }
    #[doc = "DO (MOSI master mode, MISO slave mode) is PAD\\[3\\], SCK is PAD\\[1\\], SS in slave mode is PAD\\[2\\]"]
    #[inline(always)]
    pub fn cfg2(self) -> &'a mut W {
        self.variant(DOPO_A::CFG2)
    }
    #[doc = "DO (MOSI master mode, MISO slave mode) is PAD\\[0\\], SCK is PAD\\[3\\], SS in slave mode is PAD\\[1\\]"]
    #[inline(always)]
    pub fn cfg3(self) -> &'a mut W {
        self.variant(DOPO_A::CFG3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Data In Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIPO_A {
    #[doc = "0: DI (MISO master mode, MOSI slave mode) is PAD\\[0\\]"]
    PAD0 = 0,
    #[doc = "1: DI (MISO master mode, MOSI slave mode) is PAD\\[1\\]"]
    PAD1 = 1,
    #[doc = "2: DI (MISO master mode, MOSI slave mode) is PAD\\[2\\]"]
    PAD2 = 2,
    #[doc = "3: DI (MISO master mode, MOSI slave mode) is PAD\\[3\\]"]
    PAD3 = 3,
}
impl From<DIPO_A> for u8 {
    #[inline(always)]
    fn from(variant: DIPO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIPO`"]
pub type DIPO_R = crate::R<u8, DIPO_A>;
impl DIPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIPO_A {
        match self.bits {
            0 => DIPO_A::PAD0,
            1 => DIPO_A::PAD1,
            2 => DIPO_A::PAD2,
            3 => DIPO_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == DIPO_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == DIPO_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == DIPO_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == DIPO_A::PAD3
    }
}
#[doc = "Write proxy for field `DIPO`"]
pub struct DIPO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIPO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DI (MISO master mode, MOSI slave mode) is PAD\\[0\\]"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DIPO_A::PAD0)
    }
    #[doc = "DI (MISO master mode, MOSI slave mode) is PAD\\[1\\]"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(DIPO_A::PAD1)
    }
    #[doc = "DI (MISO master mode, MOSI slave mode) is PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DIPO_A::PAD2)
    }
    #[doc = "DI (MISO master mode, MOSI slave mode) is PAD\\[3\\]"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(DIPO_A::PAD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORM_A {
    #[doc = "0: SPI frame"]
    SPI = 0,
    #[doc = "2: SPI frame with address as first byte"]
    SPI_ADDR = 2,
}
impl From<FORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FORM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORM`"]
pub type FORM_R = crate::R<u8, FORM_A>;
impl FORM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FORM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FORM_A::SPI),
            2 => Val(FORM_A::SPI_ADDR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == FORM_A::SPI
    }
    #[doc = "Checks if the value of the field is `SPI_ADDR`"]
    #[inline(always)]
    pub fn is_spi_addr(&self) -> bool {
        *self == FORM_A::SPI_ADDR
    }
}
#[doc = "Write proxy for field `FORM`"]
pub struct FORM_W<'a> {
    w: &'a mut W,
}
impl<'a> FORM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI frame"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(FORM_A::SPI)
    }
    #[doc = "SPI frame with address as first byte"]
    #[inline(always)]
    pub fn spi_addr(self) -> &'a mut W {
        self.variant(FORM_A::SPI_ADDR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    SAMPLE_LEADING = 0,
    #[doc = "1: Data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    SAMPLE_TRAILING = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::SAMPLE_LEADING,
            true => CPHA_A::SAMPLE_TRAILING,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_LEADING`"]
    #[inline(always)]
    pub fn is_sample_leading(&self) -> bool {
        *self == CPHA_A::SAMPLE_LEADING
    }
    #[doc = "Checks if the value of the field is `SAMPLE_TRAILING`"]
    #[inline(always)]
    pub fn is_sample_trailing(&self) -> bool {
        *self == CPHA_A::SAMPLE_TRAILING
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline(always)]
    pub fn sample_leading(self) -> &'a mut W {
        self.variant(CPHA_A::SAMPLE_LEADING)
    }
    #[doc = "Data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline(always)]
    pub fn sample_trailing(self) -> &'a mut W {
        self.variant(CPHA_A::SAMPLE_TRAILING)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: SCK is idle when low.  The leading edge is rising."]
    IDLE_LOW = 0,
    #[doc = "1: SCK is idle when high.  The leading edge is falling."]
    IDLE_HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLE_LOW,
            true => CPOL_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL_A::IDLE_HIGH
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCK is idle when low. The leading edge is rising."]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_LOW)
    }
    #[doc = "SCK is idle when high. The leading edge is falling."]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_HIGH)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DORD_A {
    #[doc = "0: MSB is transferred first"]
    MSBFIRST = 0,
    #[doc = "1: LSB is transferred first"]
    LSBFIRST = 1,
}
impl From<DORD_A> for bool {
    #[inline(always)]
    fn from(variant: DORD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DORD`"]
pub type DORD_R = crate::R<bool, DORD_A>;
impl DORD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DORD_A {
        match self.bits {
            false => DORD_A::MSBFIRST,
            true => DORD_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == DORD_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == DORD_A::LSBFIRST
    }
}
#[doc = "Write proxy for field `DORD`"]
pub struct DORD_W<'a> {
    w: &'a mut W,
}
impl<'a> DORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DORD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MSB is transferred first"]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(DORD_A::MSBFIRST)
    }
    #[doc = "LSB is transferred first"]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(DORD_A::LSBFIRST)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IBON_R {
        IBON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&self) -> DOPO_R {
        DOPO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&self) -> DIPO_R {
        DIPO_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&mut self) -> IBON_W {
        IBON_W { w: self }
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&mut self) -> DOPO_W {
        DOPO_W { w: self }
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&mut self) -> DIPO_W {
        DIPO_W { w: self }
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&mut self) -> FORM_W {
        FORM_W { w: self }
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&mut self) -> DORD_W {
        DORD_W { w: self }
    }
}
