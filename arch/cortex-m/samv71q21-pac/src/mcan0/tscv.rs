#[doc = "Register `TSCV` reader"]
pub type R = crate::R<TSCV_SPEC>;
#[doc = "Register `TSCV` writer"]
pub type W = crate::W<TSCV_SPEC>;
#[doc = "Field `TSC` reader - Timestamp Counter (cleared on write)"]
pub type TSC_R = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - Timestamp Counter (cleared on write)"]
pub type TSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter (cleared on write)"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Counter (cleared on write)"]
    #[inline(always)]
    #[must_use]
    pub fn tsc(&mut self) -> TSC_W<TSCV_SPEC, 0> {
        TSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timestamp Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscv::R`](R) reader structure"]
impl crate::Readable for TSCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscv::W`](W) writer structure"]
impl crate::Writable for TSCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TSCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
