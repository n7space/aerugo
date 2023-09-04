#[doc = "Register `MCTL` reader"]
pub type R = crate::R<MCTL_SPEC>;
#[doc = "Register `MCTL` writer"]
pub type W = crate::W<MCTL_SPEC>;
#[doc = "Field `XCMP` reader - Transfer Complete (Write 0 to Clear)"]
pub type XCMP_R = crate::BitReader;
#[doc = "Field `XCMP` writer - Transfer Complete (Write 0 to Clear)"]
pub type XCMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transfer Complete (Write 0 to Clear)"]
    #[inline(always)]
    pub fn xcmp(&self) -> XCMP_R {
        XCMP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete (Write 0 to Clear)"]
    #[inline(always)]
    #[must_use]
    pub fn xcmp(&mut self) -> XCMP_W<MCTL_SPEC, 0> {
        XCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MIF Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCTL_SPEC;
impl crate::RegisterSpec for MCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctl::R`](R) reader structure"]
impl crate::Readable for MCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mctl::W`](W) writer structure"]
impl crate::Writable for MCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTL to value 0"]
impl crate::Resettable for MCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
