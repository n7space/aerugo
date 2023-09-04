#[doc = "Register `BRGR` reader"]
pub type R = crate::R<BRGR_SPEC>;
#[doc = "Register `BRGR` writer"]
pub type W = crate::W<BRGR_SPEC>;
#[doc = "Field `CD` reader - Clock Divisor"]
pub type CD_R = crate::FieldReader<u16>;
#[doc = "Field `CD` writer - Clock Divisor"]
pub type CD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<BRGR_SPEC, 0> {
        CD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRGR_SPEC;
impl crate::RegisterSpec for BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brgr::R`](R) reader structure"]
impl crate::Readable for BRGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brgr::W`](W) writer structure"]
impl crate::Writable for BRGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRGR to value 0"]
impl crate::Resettable for BRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
