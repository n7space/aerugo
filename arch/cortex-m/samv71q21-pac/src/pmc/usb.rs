#[doc = "Register `USB` reader"]
pub struct R(crate::R<USB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB` writer"]
pub struct W(crate::W<USB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_SPEC>;
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
impl From<crate::W<USB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBS` reader - USB Input Clock Selection"]
pub type USBS_R = crate::BitReader;
#[doc = "Field `USBS` writer - USB Input Clock Selection"]
pub type USBS_W<'a, const O: u8> = crate::BitWriter<'a, USB_SPEC, O>;
#[doc = "Field `USBDIV` reader - Divider for USB_48M"]
pub type USBDIV_R = crate::FieldReader;
#[doc = "Field `USBDIV` writer - Divider for USB_48M"]
pub type USBDIV_W<'a, const O: u8> = crate::FieldWriter<'a, USB_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - USB Input Clock Selection"]
    #[inline(always)]
    pub fn usbs(&self) -> USBS_R {
        USBS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Divider for USB_48M"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB Input Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbs(&mut self) -> USBS_W<0> {
        USBS_W::new(self)
    }
    #[doc = "Bits 8:11 - Divider for USB_48M"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv(&mut self) -> USBDIV_W<8> {
        USBDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb](index.html) module"]
pub struct USB_SPEC;
impl crate::RegisterSpec for USB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb::R](R) reader structure"]
impl crate::Readable for USB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb::W](W) writer structure"]
impl crate::Writable for USB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB to value 0"]
impl crate::Resettable for USB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
