#[doc = "Register `RC1R` reader"]
pub type R = crate::R<RC1R_SPEC>;
#[doc = "Register `RC1R` writer"]
pub type W = crate::W<RC1R_SPEC>;
#[doc = "Field `CP1` reader - Receive Compare Data 1"]
pub type CP1_R = crate::FieldReader<u16>;
#[doc = "Field `CP1` writer - Receive Compare Data 1"]
pub type CP1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> CP1_W<RC1R_SPEC, 0> {
        CP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC1R_SPEC;
impl crate::RegisterSpec for RC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc1r::R`](R) reader structure"]
impl crate::Readable for RC1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc1r::W`](W) writer structure"]
impl crate::Writable for RC1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC1R to value 0"]
impl crate::Resettable for RC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
