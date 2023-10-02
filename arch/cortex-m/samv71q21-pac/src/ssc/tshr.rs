#[doc = "Register `TSHR` reader"]
pub type R = crate::R<TSHR_SPEC>;
#[doc = "Register `TSHR` writer"]
pub type W = crate::W<TSHR_SPEC>;
#[doc = "Field `TSDAT` reader - Transmit Synchronization Data"]
pub type TSDAT_R = crate::FieldReader<u16>;
#[doc = "Field `TSDAT` writer - Transmit Synchronization Data"]
pub type TSDAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Synchronization Data"]
    #[inline(always)]
    pub fn tsdat(&self) -> TSDAT_R {
        TSDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Synchronization Data"]
    #[inline(always)]
    #[must_use]
    pub fn tsdat(&mut self) -> TSDAT_W<TSHR_SPEC, 0> {
        TSDAT_W::new(self)
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
#[doc = "Transmit Sync. Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tshr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tshr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSHR_SPEC;
impl crate::RegisterSpec for TSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tshr::R`](R) reader structure"]
impl crate::Readable for TSHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tshr::W`](W) writer structure"]
impl crate::Writable for TSHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSHR to value 0"]
impl crate::Resettable for TSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
