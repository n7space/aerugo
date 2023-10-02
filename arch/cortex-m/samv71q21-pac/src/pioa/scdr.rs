#[doc = "Register `SCDR` reader"]
pub type R = crate::R<SCDR_SPEC>;
#[doc = "Register `SCDR` writer"]
pub type W = crate::W<SCDR_SPEC>;
#[doc = "Field `DIV` reader - Slow Clock Divider Selection for Debouncing"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Slow Clock Divider Selection for Debouncing"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Slow Clock Divider Selection for Debouncing"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Slow Clock Divider Selection for Debouncing"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<SCDR_SPEC, 0> {
        DIV_W::new(self)
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
#[doc = "Slow Clock Divider Debouncing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCDR_SPEC;
impl crate::RegisterSpec for SCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scdr::R`](R) reader structure"]
impl crate::Readable for SCDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scdr::W`](W) writer structure"]
impl crate::Writable for SCDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCDR to value 0"]
impl crate::Resettable for SCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
