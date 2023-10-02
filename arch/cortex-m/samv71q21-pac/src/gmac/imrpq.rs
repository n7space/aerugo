#[doc = "Register `IMRPQ[%s]` reader"]
pub type R = crate::R<IMRPQ_SPEC>;
#[doc = "Register `IMRPQ[%s]` writer"]
pub type W = crate::W<IMRPQ_SPEC>;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RCOMP_R = crate::BitReader;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub type RXUBR_R = crate::BitReader;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RXUBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded or Late Collision"]
pub type RLEX_R = crate::BitReader;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded or Late Collision"]
pub type RLEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHB` reader - AHB Error"]
pub type AHB_R = crate::BitReader;
#[doc = "Field `AHB` writer - AHB Error"]
pub type AHB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TCOMP_R = crate::BitReader;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type ROVR_R = crate::BitReader;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HRESP_R = crate::BitReader;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HRESP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&self) -> RLEX_R {
        RLEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AHB Error"]
    #[inline(always)]
    pub fn ahb(&self) -> AHB_R {
        AHB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rcomp(&mut self) -> RCOMP_W<IMRPQ_SPEC, 1> {
        RCOMP_W::new(self)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn rxubr(&mut self) -> RXUBR_W<IMRPQ_SPEC, 2> {
        RXUBR_W::new(self)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    #[must_use]
    pub fn rlex(&mut self) -> RLEX_W<IMRPQ_SPEC, 5> {
        RLEX_W::new(self)
    }
    #[doc = "Bit 6 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahb(&mut self) -> AHB_W<IMRPQ_SPEC, 6> {
        AHB_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TCOMP_W<IMRPQ_SPEC, 7> {
        TCOMP_W::new(self)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> ROVR_W<IMRPQ_SPEC, 10> {
        ROVR_W::new(self)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HRESP_W<IMRPQ_SPEC, 11> {
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
#[doc = "Interrupt Mask Register Priority Queue (1..5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imrpq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imrpq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMRPQ_SPEC;
impl crate::RegisterSpec for IMRPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imrpq::R`](R) reader structure"]
impl crate::Readable for IMRPQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imrpq::W`](W) writer structure"]
impl crate::Writable for IMRPQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMRPQ[%s]
to value 0"]
impl crate::Resettable for IMRPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
