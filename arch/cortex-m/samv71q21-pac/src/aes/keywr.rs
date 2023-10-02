#[doc = "Register `KEYWR[%s]` writer"]
pub type W = crate::W<KEYWR_SPEC>;
#[doc = "Field `KEYW` writer - Key Word"]
pub type KEYW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Key Word"]
    #[inline(always)]
    #[must_use]
    pub fn keyw(&mut self) -> KEYW_W<KEYWR_SPEC, 0> {
        KEYW_W::new(self)
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
#[doc = "Key Word Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keywr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYWR_SPEC;
impl crate::RegisterSpec for KEYWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keywr::W`](W) writer structure"]
impl crate::Writable for KEYWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYWR[%s]
to value 0"]
impl crate::Resettable for KEYWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
