#[doc = "Register `US_IDTTX` reader"]
pub type R = crate::R<US_IDTTX_SPEC>;
#[doc = "Register `US_IDTTX` writer"]
pub type W = crate::W<US_IDTTX_SPEC>;
#[doc = "Field `IDTTX` reader - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
pub type IDTTX_R = crate::FieldReader<u32>;
#[doc = "Field `IDTTX` writer - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
pub type IDTTX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&self) -> IDTTX_R {
        IDTTX_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn idttx(&mut self) -> IDTTX_W<US_IDTTX_SPEC, 0> {
        IDTTX_W::new(self)
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
#[doc = "LON IDT Tx Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_idttx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_idttx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_IDTTX_SPEC;
impl crate::RegisterSpec for US_IDTTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_idttx::R`](R) reader structure"]
impl crate::Readable for US_IDTTX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_idttx::W`](W) writer structure"]
impl crate::Writable for US_IDTTX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_IDTTX to value 0"]
impl crate::Resettable for US_IDTTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
