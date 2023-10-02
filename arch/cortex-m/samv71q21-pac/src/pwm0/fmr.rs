#[doc = "Register `FMR` reader"]
pub type R = crate::R<FMR_SPEC>;
#[doc = "Register `FMR` writer"]
pub type W = crate::W<FMR_SPEC>;
#[doc = "Field `FPOL` reader - Fault Polarity"]
pub type FPOL_R = crate::FieldReader;
#[doc = "Field `FPOL` writer - Fault Polarity"]
pub type FPOL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FMOD` reader - Fault Activation Mode"]
pub type FMOD_R = crate::FieldReader;
#[doc = "Field `FMOD` writer - Fault Activation Mode"]
pub type FMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FFIL` reader - Fault Filtering"]
pub type FFIL_R = crate::FieldReader;
#[doc = "Field `FFIL` writer - Fault Filtering"]
pub type FFIL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&self) -> FPOL_R {
        FPOL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&self) -> FMOD_R {
        FMOD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&self) -> FFIL_R {
        FFIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn fpol(&mut self) -> FPOL_W<FMR_SPEC, 0> {
        FPOL_W::new(self)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod(&mut self) -> FMOD_W<FMR_SPEC, 8> {
        FMOD_W::new(self)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn ffil(&mut self) -> FFIL_W<FMR_SPEC, 16> {
        FFIL_W::new(self)
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
#[doc = "PWM Fault Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMR_SPEC;
impl crate::RegisterSpec for FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmr::R`](R) reader structure"]
impl crate::Readable for FMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
