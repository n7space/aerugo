#[doc = "Register `RC0R` reader"]
pub type R = crate::R<RC0R_SPEC>;
#[doc = "Register `RC0R` writer"]
pub type W = crate::W<RC0R_SPEC>;
#[doc = "Field `CP0` reader - Receive Compare Data 0"]
pub type CP0_R = crate::FieldReader<u16>;
#[doc = "Field `CP0` writer - Receive Compare Data 0"]
pub type CP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> CP0_W<RC0R_SPEC, 0> {
        CP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Compare 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC0R_SPEC;
impl crate::RegisterSpec for RC0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc0r::R`](R) reader structure"]
impl crate::Readable for RC0R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc0r::W`](W) writer structure"]
impl crate::Writable for RC0R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC0R to value 0"]
impl crate::Resettable for RC0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
