#[doc = "Register `CKGR_MCFR` reader"]
pub struct R(crate::R<CKGR_MCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_MCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_MCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_MCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_MCFR` writer"]
pub struct W(crate::W<CKGR_MCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_MCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CKGR_MCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_MCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAINF` reader - Main Clock Frequency"]
pub type MAINF_R = crate::FieldReader<u16>;
#[doc = "Field `MAINF` writer - Main Clock Frequency"]
pub type MAINF_W<'a, const O: u8> = crate::FieldWriter<'a, CKGR_MCFR_SPEC, 16, O, u16>;
#[doc = "Field `MAINFRDY` reader - Main Clock Frequency Measure Ready"]
pub type MAINFRDY_R = crate::BitReader;
#[doc = "Field `MAINFRDY` writer - Main Clock Frequency Measure Ready"]
pub type MAINFRDY_W<'a, const O: u8> = crate::BitWriter<'a, CKGR_MCFR_SPEC, O>;
#[doc = "Field `RCMEAS` reader - RC Oscillator Frequency Measure (write-only)"]
pub type RCMEAS_R = crate::BitReader;
#[doc = "Field `RCMEAS` writer - RC Oscillator Frequency Measure (write-only)"]
pub type RCMEAS_W<'a, const O: u8> = crate::BitWriter<'a, CKGR_MCFR_SPEC, O>;
#[doc = "Field `CCSS` reader - Counter Clock Source Selection"]
pub type CCSS_R = crate::BitReader;
#[doc = "Field `CCSS` writer - Counter Clock Source Selection"]
pub type CCSS_W<'a, const O: u8> = crate::BitWriter<'a, CKGR_MCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    pub fn mainf(&self) -> MAINF_R {
        MAINF_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Main Clock Frequency Measure Ready"]
    #[inline(always)]
    pub fn mainfrdy(&self) -> MAINFRDY_R {
        MAINFRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - RC Oscillator Frequency Measure (write-only)"]
    #[inline(always)]
    pub fn rcmeas(&self) -> RCMEAS_R {
        RCMEAS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Counter Clock Source Selection"]
    #[inline(always)]
    pub fn ccss(&self) -> CCSS_R {
        CCSS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn mainf(&mut self) -> MAINF_W<0> {
        MAINF_W::new(self)
    }
    #[doc = "Bit 16 - Main Clock Frequency Measure Ready"]
    #[inline(always)]
    #[must_use]
    pub fn mainfrdy(&mut self) -> MAINFRDY_W<16> {
        MAINFRDY_W::new(self)
    }
    #[doc = "Bit 20 - RC Oscillator Frequency Measure (write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn rcmeas(&mut self) -> RCMEAS_W<20> {
        RCMEAS_W::new(self)
    }
    #[doc = "Bit 24 - Counter Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccss(&mut self) -> CCSS_W<24> {
        CCSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mcfr](index.html) module"]
pub struct CKGR_MCFR_SPEC;
impl crate::RegisterSpec for CKGR_MCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_mcfr::R](R) reader structure"]
impl crate::Readable for CKGR_MCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_mcfr::W](W) writer structure"]
impl crate::Writable for CKGR_MCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKGR_MCFR to value 0"]
impl crate::Resettable for CKGR_MCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
