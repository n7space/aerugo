#[doc = "Register `ST1RPQ[%s]` reader"]
pub type R = crate::R<ST1RPQ_SPEC>;
#[doc = "Register `ST1RPQ[%s]` writer"]
pub type W = crate::W<ST1RPQ_SPEC>;
#[doc = "Field `QNB` reader - Queue Number (0-5)"]
pub type QNB_R = crate::FieldReader;
#[doc = "Field `QNB` writer - Queue Number (0-5)"]
pub type QNB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DSTCM` reader - Differentiated Services or Traffic Class Match"]
pub type DSTCM_R = crate::FieldReader;
#[doc = "Field `DSTCM` writer - Differentiated Services or Traffic Class Match"]
pub type DSTCM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `UDPM` reader - UDP Port Match"]
pub type UDPM_R = crate::FieldReader<u16>;
#[doc = "Field `UDPM` writer - UDP Port Match"]
pub type UDPM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DSTCE` reader - Differentiated Services or Traffic Class Match Enable"]
pub type DSTCE_R = crate::BitReader;
#[doc = "Field `DSTCE` writer - Differentiated Services or Traffic Class Match Enable"]
pub type DSTCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UDPE` reader - UDP Port Match Enable"]
pub type UDPE_R = crate::BitReader;
#[doc = "Field `UDPE` writer - UDP Port Match Enable"]
pub type UDPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&self) -> QNB_R {
        QNB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:11 - Differentiated Services or Traffic Class Match"]
    #[inline(always)]
    pub fn dstcm(&self) -> DSTCM_R {
        DSTCM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    pub fn udpm(&self) -> UDPM_R {
        UDPM_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - Differentiated Services or Traffic Class Match Enable"]
    #[inline(always)]
    pub fn dstce(&self) -> DSTCE_R {
        DSTCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - UDP Port Match Enable"]
    #[inline(always)]
    pub fn udpe(&self) -> UDPE_R {
        UDPE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    #[must_use]
    pub fn qnb(&mut self) -> QNB_W<ST1RPQ_SPEC, 0> {
        QNB_W::new(self)
    }
    #[doc = "Bits 4:11 - Differentiated Services or Traffic Class Match"]
    #[inline(always)]
    #[must_use]
    pub fn dstcm(&mut self) -> DSTCM_W<ST1RPQ_SPEC, 4> {
        DSTCM_W::new(self)
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    #[must_use]
    pub fn udpm(&mut self) -> UDPM_W<ST1RPQ_SPEC, 12> {
        UDPM_W::new(self)
    }
    #[doc = "Bit 28 - Differentiated Services or Traffic Class Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dstce(&mut self) -> DSTCE_W<ST1RPQ_SPEC, 28> {
        DSTCE_W::new(self)
    }
    #[doc = "Bit 29 - UDP Port Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udpe(&mut self) -> UDPE_W<ST1RPQ_SPEC, 29> {
        UDPE_W::new(self)
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
#[doc = "Screening Type 1 Register Priority Queue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1rpq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1rpq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1RPQ_SPEC;
impl crate::RegisterSpec for ST1RPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1rpq::R`](R) reader structure"]
impl crate::Readable for ST1RPQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st1rpq::W`](W) writer structure"]
impl crate::Writable for ST1RPQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1RPQ[%s]
to value 0"]
impl crate::Resettable for ST1RPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
