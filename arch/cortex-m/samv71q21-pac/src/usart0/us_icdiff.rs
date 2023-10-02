#[doc = "Register `US_ICDIFF` reader"]
pub type R = crate::R<US_ICDIFF_SPEC>;
#[doc = "Register `US_ICDIFF` writer"]
pub type W = crate::W<US_ICDIFF_SPEC>;
#[doc = "Field `ICDIFF` reader - IC Differentiator Number"]
pub type ICDIFF_R = crate::FieldReader;
#[doc = "Field `ICDIFF` writer - IC Differentiator Number"]
pub type ICDIFF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    pub fn icdiff(&self) -> ICDIFF_R {
        ICDIFF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    #[must_use]
    pub fn icdiff(&mut self) -> ICDIFF_W<US_ICDIFF_SPEC, 0> {
        ICDIFF_W::new(self)
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
#[doc = "IC DIFF Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_icdiff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_icdiff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_ICDIFF_SPEC;
impl crate::RegisterSpec for US_ICDIFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_icdiff::R`](R) reader structure"]
impl crate::Readable for US_ICDIFF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_icdiff::W`](W) writer structure"]
impl crate::Writable for US_ICDIFF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_ICDIFF to value 0"]
impl crate::Resettable for US_ICDIFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
