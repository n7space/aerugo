#[doc = "Register `US_IF` reader"]
pub type R = crate::R<US_IF_SPEC>;
#[doc = "Register `US_IF` writer"]
pub type W = crate::W<US_IF_SPEC>;
#[doc = "Field `IRDA_FILTER` reader - IrDA Filter"]
pub type IRDA_FILTER_R = crate::FieldReader;
#[doc = "Field `IRDA_FILTER` writer - IrDA Filter"]
pub type IRDA_FILTER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    pub fn irda_filter(&self) -> IRDA_FILTER_R {
        IRDA_FILTER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    #[must_use]
    pub fn irda_filter(&mut self) -> IRDA_FILTER_W<US_IF_SPEC, 0> {
        IRDA_FILTER_W::new(self)
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
#[doc = "IrDA Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_if::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_if::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_IF_SPEC;
impl crate::RegisterSpec for US_IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_if::R`](R) reader structure"]
impl crate::Readable for US_IF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_if::W`](W) writer structure"]
impl crate::Writable for US_IF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_IF to value 0"]
impl crate::Resettable for US_IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
