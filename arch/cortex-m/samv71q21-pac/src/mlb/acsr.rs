#[doc = "Register `ACSR[%s]` reader"]
pub type R = crate::R<ACSR_SPEC>;
#[doc = "Register `ACSR[%s]` writer"]
pub type W = crate::W<ACSR_SPEC>;
#[doc = "Field `CHS` reader - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
pub type CHS_R = crate::FieldReader<u32>;
#[doc = "Field `CHS` writer - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
pub type CHS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
    #[inline(always)]
    #[must_use]
    pub fn chs(&mut self) -> CHS_W<ACSR_SPEC, 0> {
        CHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB Channel Status 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSR_SPEC;
impl crate::RegisterSpec for ACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acsr::R`](R) reader structure"]
impl crate::Readable for ACSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acsr::W`](W) writer structure"]
impl crate::Writable for ACSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR[%s]
to value 0"]
impl crate::Resettable for ACSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
