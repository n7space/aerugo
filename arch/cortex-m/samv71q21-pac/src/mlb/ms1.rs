#[doc = "Register `MS1` reader"]
pub type R = crate::R<MS1_SPEC>;
#[doc = "Register `MS1` writer"]
pub type W = crate::W<MS1_SPEC>;
#[doc = "Field `MCS` reader - MediaLB Channel Status \\[63:32\\]
(cleared by writing a 0)"]
pub type MCS_R = crate::FieldReader<u32>;
#[doc = "Field `MCS` writer - MediaLB Channel Status \\[63:32\\]
(cleared by writing a 0)"]
pub type MCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[63:32\\]
(cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&self) -> MCS_R {
        MCS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[63:32\\]
(cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn mcs(&mut self) -> MCS_W<MS1_SPEC, 0> {
        MCS_W::new(self)
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
#[doc = "MediaLB Channel Status1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MS1_SPEC;
impl crate::RegisterSpec for MS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms1::R`](R) reader structure"]
impl crate::Readable for MS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ms1::W`](W) writer structure"]
impl crate::Writable for MS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS1 to value 0"]
impl crate::Resettable for MS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
