#[doc = "Register `IADR` reader"]
pub type R = crate::R<IADR_SPEC>;
#[doc = "Register `IADR` writer"]
pub type W = crate::W<IADR_SPEC>;
#[doc = "Field `IADR` reader - Internal Address"]
pub type IADR_R = crate::FieldReader<u32>;
#[doc = "Field `IADR` writer - Internal Address"]
pub type IADR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    pub fn iadr(&self) -> IADR_R {
        IADR_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    #[must_use]
    pub fn iadr(&mut self) -> IADR_W<IADR_SPEC, 0> {
        IADR_W::new(self)
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
#[doc = "Internal Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iadr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iadr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IADR_SPEC;
impl crate::RegisterSpec for IADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iadr::R`](R) reader structure"]
impl crate::Readable for IADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iadr::W`](W) writer structure"]
impl crate::Writable for IADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IADR to value 0"]
impl crate::Resettable for IADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
