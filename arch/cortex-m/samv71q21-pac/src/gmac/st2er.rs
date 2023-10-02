#[doc = "Register `ST2ER[%s]` reader"]
pub type R = crate::R<ST2ER_SPEC>;
#[doc = "Register `ST2ER[%s]` writer"]
pub type W = crate::W<ST2ER_SPEC>;
#[doc = "Field `COMPVAL` reader - Ethertype Compare Value"]
pub type COMPVAL_R = crate::FieldReader<u16>;
#[doc = "Field `COMPVAL` writer - Ethertype Compare Value"]
pub type COMPVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn compval(&mut self) -> COMPVAL_W<ST2ER_SPEC, 0> {
        COMPVAL_W::new(self)
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
#[doc = "Screening Type 2 Ethertype Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2er::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2er::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST2ER_SPEC;
impl crate::RegisterSpec for ST2ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2er::R`](R) reader structure"]
impl crate::Readable for ST2ER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st2er::W`](W) writer structure"]
impl crate::Writable for ST2ER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST2ER[%s]
to value 0"]
impl crate::Resettable for ST2ER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
