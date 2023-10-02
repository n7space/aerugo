#[doc = "Register `IERPQ[%s]` writer"]
pub type W = crate::W<IERPQ_SPEC>;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RXUBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded or Late Collision"]
pub type RLEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFC` writer - Transmit Frame Corruption Due to AHB Error"]
pub type TFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HRESP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rcomp(&mut self) -> RCOMP_W<IERPQ_SPEC, 1> {
        RCOMP_W::new(self)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn rxubr(&mut self) -> RXUBR_W<IERPQ_SPEC, 2> {
        RXUBR_W::new(self)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    #[must_use]
    pub fn rlex(&mut self) -> RLEX_W<IERPQ_SPEC, 5> {
        RLEX_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn tfc(&mut self) -> TFC_W<IERPQ_SPEC, 6> {
        TFC_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TCOMP_W<IERPQ_SPEC, 7> {
        TCOMP_W::new(self)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> ROVR_W<IERPQ_SPEC, 10> {
        ROVR_W::new(self)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HRESP_W<IERPQ_SPEC, 11> {
        HRESP_W::new(self)
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
#[doc = "Interrupt Enable Register Priority Queue (1..5)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ierpq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERPQ_SPEC;
impl crate::RegisterSpec for IERPQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ierpq::W`](W) writer structure"]
impl crate::Writable for IERPQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IERPQ[%s]
to value 0"]
impl crate::Resettable for IERPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
