#[doc = "Register `RBSRPQ[%s]` reader"]
pub type R = crate::R<RBSRPQ_SPEC>;
#[doc = "Register `RBSRPQ[%s]` writer"]
pub type W = crate::W<RBSRPQ_SPEC>;
#[doc = "Field `RBS` reader - Receive Buffer Size"]
pub type RBS_R = crate::FieldReader<u16>;
#[doc = "Field `RBS` writer - Receive Buffer Size"]
pub type RBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<RBSRPQ_SPEC, 0> {
        RBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Buffer Size Register Priority Queue (1..5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbsrpq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbsrpq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBSRPQ_SPEC;
impl crate::RegisterSpec for RBSRPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbsrpq::R`](R) reader structure"]
impl crate::Readable for RBSRPQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rbsrpq::W`](W) writer structure"]
impl crate::Writable for RBSRPQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBSRPQ[%s]
to value 0"]
impl crate::Resettable for RBSRPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
