#[doc = "Register `IDR1` writer"]
pub struct W(crate::W<IDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR1_SPEC>;
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
impl From<crate::W<IDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHID0` writer - Counter Event on Channel 0 Interrupt Disable"]
pub type CHID0_W<'a, const O: u8> = crate::BitWriter<'a, IDR1_SPEC, O>;
#[doc = "Field `CHID1` writer - Counter Event on Channel 1 Interrupt Disable"]
pub type CHID1_W<'a, const O: u8> = crate::BitWriter<'a, IDR1_SPEC, O>;
#[doc = "Field `CHID2` writer - Counter Event on Channel 2 Interrupt Disable"]
pub type CHID2_W<'a, const O: u8> = crate::BitWriter<'a, IDR1_SPEC, O>;
#[doc = "Field `CHID3` writer - Counter Event on Channel 3 Interrupt Disable"]
pub type CHID3_W<'a, const O: u8> = crate::BitWriter<'a, IDR1_SPEC, O>;
#[doc = "Field `FCHID0` writer - Fault Protection Trigger on Channel 0 Interrupt Disable"]
pub type FCHID0_W<'a, const O: u8> = crate::BitWriter<'a, IDR1_SPEC, O>;
#[doc = "Field `FCHID1` writer - Fault Protection Trigger on Channel 1 Interrupt Disable"]
pub type FCHID1_W<'a, const O: u8> = crate::BitWriter<'a, IDR1_SPEC, O>;
#[doc = "Field `FCHID2` writer - Fault Protection Trigger on Channel 2 Interrupt Disable"]
pub type FCHID2_W<'a, const O: u8> = crate::BitWriter<'a, IDR1_SPEC, O>;
#[doc = "Field `FCHID3` writer - Fault Protection Trigger on Channel 3 Interrupt Disable"]
pub type FCHID3_W<'a, const O: u8> = crate::BitWriter<'a, IDR1_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chid0(&mut self) -> CHID0_W<0> {
        CHID0_W::new(self)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chid1(&mut self) -> CHID1_W<1> {
        CHID1_W::new(self)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chid2(&mut self) -> CHID2_W<2> {
        CHID2_W::new(self)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chid3(&mut self) -> CHID3_W<3> {
        CHID3_W::new(self)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid0(&mut self) -> FCHID0_W<16> {
        FCHID0_W::new(self)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid1(&mut self) -> FCHID1_W<17> {
        FCHID1_W::new(self)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid2(&mut self) -> FCHID2_W<18> {
        FCHID2_W::new(self)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid3(&mut self) -> FCHID3_W<19> {
        FCHID3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Disable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr1](index.html) module"]
pub struct IDR1_SPEC;
impl crate::RegisterSpec for IDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr1::W](W) writer structure"]
impl crate::Writable for IDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR1 to value 0"]
impl crate::Resettable for IDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
