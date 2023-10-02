#[doc = "Register `ACTL` reader"]
pub type R = crate::R<ACTL_SPEC>;
#[doc = "Register `ACTL` writer"]
pub type W = crate::W<ACTL_SPEC>;
#[doc = "Field `SCE` reader - Software Clear Enable"]
pub type SCE_R = crate::BitReader;
#[doc = "Field `SCE` writer - Software Clear Enable"]
pub type SCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMX` reader - AHB Interrupt Mux Enable"]
pub type SMX_R = crate::BitReader;
#[doc = "Field `SMX` writer - AHB Interrupt Mux Enable"]
pub type SMX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_MODE` reader - DMA Mode"]
pub type DMA_MODE_R = crate::BitReader;
#[doc = "Field `DMA_MODE` writer - DMA Mode"]
pub type DMA_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPB` reader - DMA Packet Buffering Mode"]
pub type MPB_R = crate::BitReader<MPBSELECT_A>;
#[doc = "DMA Packet Buffering Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPBSELECT_A {
    #[doc = "0: Single-packet mode"]
    SINGLE_PACKET = 0,
    #[doc = "1: Multiple-packet mode"]
    MULTIPLE_PACKET = 1,
}
impl From<MPBSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MPBSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPBSELECT_A {
        match self.bits {
            false => MPBSELECT_A::SINGLE_PACKET,
            true => MPBSELECT_A::MULTIPLE_PACKET,
        }
    }
    #[doc = "Single-packet mode"]
    #[inline(always)]
    pub fn is_single_packet(&self) -> bool {
        *self == MPBSELECT_A::SINGLE_PACKET
    }
    #[doc = "Multiple-packet mode"]
    #[inline(always)]
    pub fn is_multiple_packet(&self) -> bool {
        *self == MPBSELECT_A::MULTIPLE_PACKET
    }
}
#[doc = "Field `MPB` writer - DMA Packet Buffering Mode"]
pub type MPB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MPBSELECT_A>;
impl<'a, REG, const O: u8> MPB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-packet mode"]
    #[inline(always)]
    pub fn single_packet(self) -> &'a mut crate::W<REG> {
        self.variant(MPBSELECT_A::SINGLE_PACKET)
    }
    #[doc = "Multiple-packet mode"]
    #[inline(always)]
    pub fn multiple_packet(self) -> &'a mut crate::W<REG> {
        self.variant(MPBSELECT_A::MULTIPLE_PACKET)
    }
}
impl R {
    #[doc = "Bit 0 - Software Clear Enable"]
    #[inline(always)]
    pub fn sce(&self) -> SCE_R {
        SCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB Interrupt Mux Enable"]
    #[inline(always)]
    pub fn smx(&self) -> SMX_R {
        SMX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Mode"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DMA_MODE_R {
        DMA_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Packet Buffering Mode"]
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sce(&mut self) -> SCE_W<ACTL_SPEC, 0> {
        SCE_W::new(self)
    }
    #[doc = "Bit 1 - AHB Interrupt Mux Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smx(&mut self) -> SMX_W<ACTL_SPEC, 1> {
        SMX_W::new(self)
    }
    #[doc = "Bit 2 - DMA Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma_mode(&mut self) -> DMA_MODE_W<ACTL_SPEC, 2> {
        DMA_MODE_W::new(self)
    }
    #[doc = "Bit 4 - DMA Packet Buffering Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpb(&mut self) -> MPB_W<ACTL_SPEC, 4> {
        MPB_W::new(self)
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
#[doc = "AHB Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTL_SPEC;
impl crate::RegisterSpec for ACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actl::R`](R) reader structure"]
impl crate::Readable for ACTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actl::W`](W) writer structure"]
impl crate::Writable for ACTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTL to value 0"]
impl crate::Resettable for ACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
