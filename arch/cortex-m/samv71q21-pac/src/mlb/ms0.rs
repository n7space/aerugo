#[doc = "Register `MS0` reader"]
pub type R = crate::R<MS0_SPEC>;
#[doc = "Register `MS0` writer"]
pub type W = crate::W<MS0_SPEC>;
#[doc = "Field `MCS` reader - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
pub type MCS_R = crate::FieldReader<u32>;
#[doc = "Field `MCS` writer - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
pub type MCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&self) -> MCS_R {
        MCS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn mcs(&mut self) -> MCS_W<MS0_SPEC, 0> {
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
#[doc = "MediaLB Channel Status 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MS0_SPEC;
impl crate::RegisterSpec for MS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms0::R`](R) reader structure"]
impl crate::Readable for MS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ms0::W`](W) writer structure"]
impl crate::Writable for MS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS0 to value 0"]
impl crate::Resettable for MS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
