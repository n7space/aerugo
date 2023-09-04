#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `INST` reader - Instruction Code"]
pub type INST_R = crate::FieldReader;
#[doc = "Field `INST` writer - Instruction Code"]
pub type INST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `OPT` reader - Option Code"]
pub type OPT_R = crate::FieldReader;
#[doc = "Field `OPT` writer - Option Code"]
pub type OPT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn inst(&self) -> INST_R {
        INST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn opt(&self) -> OPT_R {
        OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    #[must_use]
    pub fn inst(&mut self) -> INST_W<ICR_SPEC, 0> {
        INST_W::new(self)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    #[must_use]
    pub fn opt(&mut self) -> OPT_W<ICR_SPEC, 16> {
        OPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Instruction Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
