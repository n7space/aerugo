#[doc = "Register `RC` reader"]
pub type R = crate::R<RC_SPEC>;
#[doc = "Register `RC` writer"]
pub type W = crate::W<RC_SPEC>;
#[doc = "Field `RC` reader - Register C"]
pub type RC_R = crate::FieldReader<u32>;
#[doc = "Field `RC` writer - Register C"]
pub type RC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RC_W<RC_SPEC, 0> {
        RC_W::new(self)
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
#[doc = "Register C (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC_SPEC;
impl crate::RegisterSpec for RC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc::R`](R) reader structure"]
impl crate::Readable for RC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc::W`](W) writer structure"]
impl crate::Writable for RC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC to value 0"]
impl crate::Resettable for RC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
