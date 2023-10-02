#[doc = "Register `PDECF` reader"]
pub type R = crate::R<PDECF_SPEC>;
#[doc = "Register `PDECF` writer"]
pub type W = crate::W<PDECF_SPEC>;
#[doc = "Field `DEC_FACTOR` reader - Decimation Factor"]
pub type DEC_FACTOR_R = crate::FieldReader;
#[doc = "Field `DEC_FACTOR` writer - Decimation Factor"]
pub type DEC_FACTOR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    pub fn dec_factor(&self) -> DEC_FACTOR_R {
        DEC_FACTOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn dec_factor(&mut self) -> DEC_FACTOR_W<PDECF_SPEC, 0> {
        DEC_FACTOR_W::new(self)
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
#[doc = "ISI Preview Decimation Factor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdecf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdecf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDECF_SPEC;
impl crate::RegisterSpec for PDECF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdecf::R`](R) reader structure"]
impl crate::Readable for PDECF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdecf::W`](W) writer structure"]
impl crate::Writable for PDECF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDECF to value 0"]
impl crate::Resettable for PDECF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
