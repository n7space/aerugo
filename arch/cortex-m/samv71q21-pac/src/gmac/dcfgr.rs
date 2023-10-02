#[doc = "Register `DCFGR` reader"]
pub type R = crate::R<DCFGR_SPEC>;
#[doc = "Register `DCFGR` writer"]
pub type W = crate::W<DCFGR_SPEC>;
#[doc = "Field `FBLDO` reader - Fixed Burst Length for DMA Data Operations:"]
pub type FBLDO_R = crate::FieldReader<FBLDOSELECT_A>;
#[doc = "Fixed Burst Length for DMA Data Operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FBLDOSELECT_A {
    #[doc = "1: 00001: Always use SINGLE AHB bursts"]
    SINGLE = 1,
    #[doc = "4: 001xx: Attempt to use INCR4 AHB bursts (Default)"]
    INCR4 = 4,
    #[doc = "8: 01xxx: Attempt to use INCR8 AHB bursts"]
    INCR8 = 8,
    #[doc = "16: 1xxxx: Attempt to use INCR16 AHB bursts"]
    INCR16 = 16,
}
impl From<FBLDOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FBLDOSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FBLDOSELECT_A {
    type Ux = u8;
}
impl FBLDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FBLDOSELECT_A> {
        match self.bits {
            1 => Some(FBLDOSELECT_A::SINGLE),
            4 => Some(FBLDOSELECT_A::INCR4),
            8 => Some(FBLDOSELECT_A::INCR8),
            16 => Some(FBLDOSELECT_A::INCR16),
            _ => None,
        }
    }
    #[doc = "00001: Always use SINGLE AHB bursts"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == FBLDOSELECT_A::SINGLE
    }
    #[doc = "001xx: Attempt to use INCR4 AHB bursts (Default)"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == FBLDOSELECT_A::INCR4
    }
    #[doc = "01xxx: Attempt to use INCR8 AHB bursts"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == FBLDOSELECT_A::INCR8
    }
    #[doc = "1xxxx: Attempt to use INCR16 AHB bursts"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == FBLDOSELECT_A::INCR16
    }
}
#[doc = "Field `FBLDO` writer - Fixed Burst Length for DMA Data Operations:"]
pub type FBLDO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, FBLDOSELECT_A>;
impl<'a, REG, const O: u8> FBLDO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "00001: Always use SINGLE AHB bursts"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(FBLDOSELECT_A::SINGLE)
    }
    #[doc = "001xx: Attempt to use INCR4 AHB bursts (Default)"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(FBLDOSELECT_A::INCR4)
    }
    #[doc = "01xxx: Attempt to use INCR8 AHB bursts"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(FBLDOSELECT_A::INCR8)
    }
    #[doc = "1xxxx: Attempt to use INCR16 AHB bursts"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(FBLDOSELECT_A::INCR16)
    }
}
#[doc = "Field `ESMA` reader - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub type ESMA_R = crate::BitReader;
#[doc = "Field `ESMA` writer - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub type ESMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESPA` reader - Endian Swap Mode Enable for Packet Data Accesses"]
pub type ESPA_R = crate::BitReader;
#[doc = "Field `ESPA` writer - Endian Swap Mode Enable for Packet Data Accesses"]
pub type ESPA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBMS` reader - Receiver Packet Buffer Memory Size Select"]
pub type RXBMS_R = crate::FieldReader<RXBMSSELECT_A>;
#[doc = "Receiver Packet Buffer Memory Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXBMSSELECT_A {
    #[doc = "0: 4/8 Kbyte Memory Size"]
    EIGHTH = 0,
    #[doc = "1: 4/4 Kbytes Memory Size"]
    QUARTER = 1,
    #[doc = "2: 4/2 Kbytes Memory Size"]
    HALF = 2,
    #[doc = "3: 4 Kbytes Memory Size"]
    FULL = 3,
}
impl From<RXBMSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXBMSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXBMSSELECT_A {
    type Ux = u8;
}
impl RXBMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBMSSELECT_A {
        match self.bits {
            0 => RXBMSSELECT_A::EIGHTH,
            1 => RXBMSSELECT_A::QUARTER,
            2 => RXBMSSELECT_A::HALF,
            3 => RXBMSSELECT_A::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "4/8 Kbyte Memory Size"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == RXBMSSELECT_A::EIGHTH
    }
    #[doc = "4/4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == RXBMSSELECT_A::QUARTER
    }
    #[doc = "4/2 Kbytes Memory Size"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == RXBMSSELECT_A::HALF
    }
    #[doc = "4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXBMSSELECT_A::FULL
    }
}
#[doc = "Field `RXBMS` writer - Receiver Packet Buffer Memory Size Select"]
pub type RXBMS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RXBMSSELECT_A>;
impl<'a, REG, const O: u8> RXBMS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4/8 Kbyte Memory Size"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut crate::W<REG> {
        self.variant(RXBMSSELECT_A::EIGHTH)
    }
    #[doc = "4/4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(RXBMSSELECT_A::QUARTER)
    }
    #[doc = "4/2 Kbytes Memory Size"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(RXBMSSELECT_A::HALF)
    }
    #[doc = "4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(RXBMSSELECT_A::FULL)
    }
}
#[doc = "Field `TXPBMS` reader - Transmitter Packet Buffer Memory Size Select"]
pub type TXPBMS_R = crate::BitReader;
#[doc = "Field `TXPBMS` writer - Transmitter Packet Buffer Memory Size Select"]
pub type TXPBMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXCOEN` reader - Transmitter Checksum Generation Offload Enable"]
pub type TXCOEN_R = crate::BitReader;
#[doc = "Field `TXCOEN` writer - Transmitter Checksum Generation Offload Enable"]
pub type TXCOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRBS` reader - DMA Receive Buffer Size"]
pub type DRBS_R = crate::FieldReader;
#[doc = "Field `DRBS` writer - DMA Receive Buffer Size"]
pub type DRBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DDRP` reader - DMA Discard Receive Packets"]
pub type DDRP_R = crate::BitReader;
#[doc = "Field `DDRP` writer - DMA Discard Receive Packets"]
pub type DDRP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&self) -> FBLDO_R {
        FBLDO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&self) -> ESMA_R {
        ESMA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&self) -> ESPA_R {
        ESPA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn rxbms(&self) -> RXBMS_R {
        RXBMS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn txpbms(&self) -> TXPBMS_R {
        TXPBMS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    pub fn txcoen(&self) -> TXCOEN_R {
        TXCOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&self) -> DRBS_R {
        DRBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    pub fn ddrp(&self) -> DDRP_R {
        DDRP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    #[must_use]
    pub fn fbldo(&mut self) -> FBLDO_W<DCFGR_SPEC, 0> {
        FBLDO_W::new(self)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn esma(&mut self) -> ESMA_W<DCFGR_SPEC, 6> {
        ESMA_W::new(self)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn espa(&mut self) -> ESPA_W<DCFGR_SPEC, 7> {
        ESPA_W::new(self)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxbms(&mut self) -> RXBMS_W<DCFGR_SPEC, 8> {
        RXBMS_W::new(self)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn txpbms(&mut self) -> TXPBMS_W<DCFGR_SPEC, 10> {
        TXPBMS_W::new(self)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txcoen(&mut self) -> TXCOEN_W<DCFGR_SPEC, 11> {
        TXCOEN_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn drbs(&mut self) -> DRBS_W<DCFGR_SPEC, 16> {
        DRBS_W::new(self)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    #[must_use]
    pub fn ddrp(&mut self) -> DDRP_W<DCFGR_SPEC, 24> {
        DDRP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFGR_SPEC;
impl crate::RegisterSpec for DCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfgr::R`](R) reader structure"]
impl crate::Readable for DCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcfgr::W`](W) writer structure"]
impl crate::Writable for DCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFGR to value 0"]
impl crate::Resettable for DCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
