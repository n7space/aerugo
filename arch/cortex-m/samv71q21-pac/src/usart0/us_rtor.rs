#[doc = "Register `US_RTOR` reader"]
pub type R = crate::R<US_RTOR_SPEC>;
#[doc = "Register `US_RTOR` writer"]
pub type W = crate::W<US_RTOR_SPEC>;
#[doc = "Field `TO` reader - Timeout Value"]
pub type TO_R = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - Timeout Value"]
pub type TO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 17, O, u32>;
impl R {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<US_RTOR_SPEC, 0> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receiver Timeout Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_rtor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_rtor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_RTOR_SPEC;
impl crate::RegisterSpec for US_RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_rtor::R`](R) reader structure"]
impl crate::Readable for US_RTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_rtor::W`](W) writer structure"]
impl crate::Writable for US_RTOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_RTOR to value 0"]
impl crate::Resettable for US_RTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
