#[doc = "Register `CSA` reader"]
pub type R = crate::R<CSA_SPEC>;
#[doc = "Register `CSA` writer"]
pub type W = crate::W<CSA_SPEC>;
#[doc = "Field `SA` reader - Channel x Source Address"]
pub type SA_R = crate::FieldReader<u32>;
#[doc = "Field `SA` writer - Channel x Source Address"]
pub type SA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<CSA_SPEC, 0> {
        SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSA_SPEC;
impl crate::RegisterSpec for CSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csa::R`](R) reader structure"]
impl crate::Readable for CSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csa::W`](W) writer structure"]
impl crate::Writable for CSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSA to value 0"]
impl crate::Resettable for CSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
