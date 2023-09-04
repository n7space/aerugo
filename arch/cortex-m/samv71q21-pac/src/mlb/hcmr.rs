#[doc = "Register `HCMR[%s]` reader"]
pub type R = crate::R<HCMR_SPEC>;
#[doc = "Register `HCMR[%s]` writer"]
pub type W = crate::W<HCMR_SPEC>;
#[doc = "Field `CHM` reader - Bitwise Channel Mask Bit \\[31:0\\]"]
pub type CHM_R = crate::FieldReader<u32>;
#[doc = "Field `CHM` writer - Bitwise Channel Mask Bit \\[31:0\\]"]
pub type CHM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chm(&self) -> CHM_R {
        CHM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bit \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn chm(&mut self) -> CHM_W<HCMR_SPEC, 0> {
        CHM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HBI Channel Mask 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCMR_SPEC;
impl crate::RegisterSpec for HCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcmr::R`](R) reader structure"]
impl crate::Readable for HCMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcmr::W`](W) writer structure"]
impl crate::Writable for HCMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCMR[%s]
to value 0"]
impl crate::Resettable for HCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
