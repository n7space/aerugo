#[doc = "Register `US_LINIR` reader"]
pub type R = crate::R<US_LINIR_SPEC>;
#[doc = "Register `US_LINIR` writer"]
pub type W = crate::W<US_LINIR_SPEC>;
#[doc = "Field `IDCHR` reader - Identifier Character"]
pub type IDCHR_R = crate::FieldReader;
#[doc = "Field `IDCHR` writer - Identifier Character"]
pub type IDCHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&self) -> IDCHR_R {
        IDCHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    #[must_use]
    pub fn idchr(&mut self) -> IDCHR_W<US_LINIR_SPEC, 0> {
        IDCHR_W::new(self)
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
#[doc = "LIN Identifier Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_linir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_linir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LINIR_SPEC;
impl crate::RegisterSpec for US_LINIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_linir::R`](R) reader structure"]
impl crate::Readable for US_LINIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_linir::W`](W) writer structure"]
impl crate::Writable for US_LINIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LINIR to value 0"]
impl crate::Resettable for US_LINIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
