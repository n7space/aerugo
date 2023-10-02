#[doc = "Register `CBSISQB` reader"]
pub type R = crate::R<CBSISQB_SPEC>;
#[doc = "Register `CBSISQB` writer"]
pub type W = crate::W<CBSISQB_SPEC>;
#[doc = "Field `IS` reader - IdleSlope"]
pub type IS_R = crate::FieldReader<u32>;
#[doc = "Field `IS` writer - IdleSlope"]
pub type IS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    #[must_use]
    pub fn is(&mut self) -> IS_W<CBSISQB_SPEC, 0> {
        IS_W::new(self)
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
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbsisqb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbsisqb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBSISQB_SPEC;
impl crate::RegisterSpec for CBSISQB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbsisqb::R`](R) reader structure"]
impl crate::Readable for CBSISQB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cbsisqb::W`](W) writer structure"]
impl crate::Writable for CBSISQB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBSISQB to value 0"]
impl crate::Resettable for CBSISQB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
