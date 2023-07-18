#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKDIS` writer - Acknowledge Update Interrupt Disable"]
pub type ACKDIS_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `ALRDIS` writer - Alarm Interrupt Disable"]
pub type ALRDIS_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `SECDIS` writer - Second Event Interrupt Disable"]
pub type SECDIS_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TIMDIS` writer - Time Event Interrupt Disable"]
pub type TIMDIS_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `CALDIS` writer - Calendar Event Interrupt Disable"]
pub type CALDIS_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TDERRDIS` writer - Time and/or Date Error Interrupt Disable"]
pub type TDERRDIS_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Acknowledge Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ackdis(&mut self) -> ACKDIS_W<0> {
        ACKDIS_W::new(self)
    }
    #[doc = "Bit 1 - Alarm Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn alrdis(&mut self) -> ALRDIS_W<1> {
        ALRDIS_W::new(self)
    }
    #[doc = "Bit 2 - Second Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn secdis(&mut self) -> SECDIS_W<2> {
        SECDIS_W::new(self)
    }
    #[doc = "Bit 3 - Time Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn timdis(&mut self) -> TIMDIS_W<3> {
        TIMDIS_W::new(self)
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn caldis(&mut self) -> CALDIS_W<4> {
        CALDIS_W::new(self)
    }
    #[doc = "Bit 5 - Time and/or Date Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tderrdis(&mut self) -> TDERRDIS_W<5> {
        TDERRDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
