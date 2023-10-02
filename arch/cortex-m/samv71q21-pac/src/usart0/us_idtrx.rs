#[doc = "Register `US_IDTRX` reader"]
pub type R = crate::R<US_IDTRX_SPEC>;
#[doc = "Register `US_IDTRX` writer"]
pub type W = crate::W<US_IDTRX_SPEC>;
#[doc = "Field `IDTRX` reader - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
pub type IDTRX_R = crate::FieldReader<u32>;
#[doc = "Field `IDTRX` writer - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
pub type IDTRX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idtrx(&self) -> IDTRX_R {
        IDTRX_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn idtrx(&mut self) -> IDTRX_W<US_IDTRX_SPEC, 0> {
        IDTRX_W::new(self)
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
#[doc = "LON IDT Rx Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_idtrx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_idtrx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_IDTRX_SPEC;
impl crate::RegisterSpec for US_IDTRX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_idtrx::R`](R) reader structure"]
impl crate::Readable for US_IDTRX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_idtrx::W`](W) writer structure"]
impl crate::Writable for US_IDTRX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_IDTRX to value 0"]
impl crate::Resettable for US_IDTRX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
