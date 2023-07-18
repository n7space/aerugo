#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKEN` writer - Acknowledge Update Interrupt Enable"]
pub type ACKEN_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `ALREN` writer - Alarm Interrupt Enable"]
pub type ALREN_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `SECEN` writer - Second Event Interrupt Enable"]
pub type SECEN_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `TIMEN` writer - Time Event Interrupt Enable"]
pub type TIMEN_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `CALEN` writer - Calendar Event Interrupt Enable"]
pub type CALEN_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `TDERREN` writer - Time and/or Date Error Interrupt Enable"]
pub type TDERREN_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Acknowledge Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acken(&mut self) -> ACKEN_W<0> {
        ACKEN_W::new(self)
    }
    #[doc = "Bit 1 - Alarm Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alren(&mut self) -> ALREN_W<1> {
        ALREN_W::new(self)
    }
    #[doc = "Bit 2 - Second Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn secen(&mut self) -> SECEN_W<2> {
        SECEN_W::new(self)
    }
    #[doc = "Bit 3 - Time Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timen(&mut self) -> TIMEN_W<3> {
        TIMEN_W::new(self)
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<4> {
        CALEN_W::new(self)
    }
    #[doc = "Bit 5 - Time and/or Date Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tderren(&mut self) -> TDERREN_W<5> {
        TDERREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
