#[doc = "Register `CDA` reader"]
pub type R = crate::R<CDA_SPEC>;
#[doc = "Register `CDA` writer"]
pub type W = crate::W<CDA_SPEC>;
#[doc = "Field `DA` reader - Channel x Destination Address"]
pub type DA_R = crate::FieldReader<u32>;
#[doc = "Field `DA` writer - Channel x Destination Address"]
pub type DA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<CDA_SPEC, 0> {
        DA_W::new(self)
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
#[doc = "Channel Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDA_SPEC;
impl crate::RegisterSpec for CDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cda::R`](R) reader structure"]
impl crate::Readable for CDA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cda::W`](W) writer structure"]
impl crate::Writable for CDA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDA to value 0"]
impl crate::Resettable for CDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
