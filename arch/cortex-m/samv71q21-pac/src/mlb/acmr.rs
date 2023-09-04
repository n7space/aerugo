#[doc = "Register `ACMR[%s]` reader"]
pub type R = crate::R<ACMR_SPEC>;
#[doc = "Register `ACMR[%s]` writer"]
pub type W = crate::W<ACMR_SPEC>;
#[doc = "Field `CHM` reader - Bitwise Channel Mask Bits 31 to 0"]
pub type CHM_R = crate::FieldReader<u32>;
#[doc = "Field `CHM` writer - Bitwise Channel Mask Bits 31 to 0"]
pub type CHM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bits 31 to 0"]
    #[inline(always)]
    pub fn chm(&self) -> CHM_R {
        CHM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bits 31 to 0"]
    #[inline(always)]
    #[must_use]
    pub fn chm(&mut self) -> CHM_W<ACMR_SPEC, 0> {
        CHM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB Channel Mask 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACMR_SPEC;
impl crate::RegisterSpec for ACMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acmr::R`](R) reader structure"]
impl crate::Readable for ACMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acmr::W`](W) writer structure"]
impl crate::Writable for ACMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACMR[%s]
to value 0"]
impl crate::Resettable for ACMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
