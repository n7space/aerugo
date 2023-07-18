#[doc = "Register `OHCIICR` reader"]
pub struct R(crate::R<OHCIICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OHCIICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OHCIICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OHCIICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OHCIICR` writer"]
pub struct W(crate::W<OHCIICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OHCIICR_SPEC>;
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
impl From<crate::W<OHCIICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OHCIICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RES0` reader - USB PORTx Reset"]
pub type RES0_R = crate::BitReader;
#[doc = "Field `RES0` writer - USB PORTx Reset"]
pub type RES0_W<'a, const O: u8> = crate::BitWriter<'a, OHCIICR_SPEC, O>;
#[doc = "Field `ARIE` reader - OHCI Asynchronous Resume Interrupt Enable"]
pub type ARIE_R = crate::BitReader;
#[doc = "Field `ARIE` writer - OHCI Asynchronous Resume Interrupt Enable"]
pub type ARIE_W<'a, const O: u8> = crate::BitWriter<'a, OHCIICR_SPEC, O>;
#[doc = "Field `APPSTART` reader - "]
pub type APPSTART_R = crate::BitReader;
#[doc = "Field `APPSTART` writer - "]
pub type APPSTART_W<'a, const O: u8> = crate::BitWriter<'a, OHCIICR_SPEC, O>;
#[doc = "Field `UDPPUDIS` reader - USB Device Pull-up Disable"]
pub type UDPPUDIS_R = crate::BitReader;
#[doc = "Field `UDPPUDIS` writer - USB Device Pull-up Disable"]
pub type UDPPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, OHCIICR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - USB PORTx Reset"]
    #[inline(always)]
    pub fn res0(&self) -> RES0_R {
        RES0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - OHCI Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn arie(&self) -> ARIE_R {
        ARIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn appstart(&self) -> APPSTART_R {
        APPSTART_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 23 - USB Device Pull-up Disable"]
    #[inline(always)]
    pub fn udppudis(&self) -> UDPPUDIS_R {
        UDPPUDIS_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PORTx Reset"]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> RES0_W<0> {
        RES0_W::new(self)
    }
    #[doc = "Bit 4 - OHCI Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arie(&mut self) -> ARIE_W<4> {
        ARIE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn appstart(&mut self) -> APPSTART_W<5> {
        APPSTART_W::new(self)
    }
    #[doc = "Bit 23 - USB Device Pull-up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn udppudis(&mut self) -> UDPPUDIS_W<23> {
        UDPPUDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Interrupt Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ohciicr](index.html) module"]
pub struct OHCIICR_SPEC;
impl crate::RegisterSpec for OHCIICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ohciicr::R](R) reader structure"]
impl crate::Readable for OHCIICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ohciicr::W](W) writer structure"]
impl crate::Writable for OHCIICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OHCIICR to value 0"]
impl crate::Resettable for OHCIICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
