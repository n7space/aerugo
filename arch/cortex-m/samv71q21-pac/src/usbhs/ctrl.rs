#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDERRE` reader - Remote Device Connection Error Interrupt Enable"]
pub type RDERRE_R = crate::BitReader;
#[doc = "Field `RDERRE` writer - Remote Device Connection Error Interrupt Enable"]
pub type RDERRE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `VBUSHWC` reader - VBUS Hardware Control"]
pub type VBUSHWC_R = crate::BitReader;
#[doc = "Field `VBUSHWC` writer - VBUS Hardware Control"]
pub type VBUSHWC_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FRZCLK` reader - Freeze USB Clock"]
pub type FRZCLK_R = crate::BitReader;
#[doc = "Field `FRZCLK` writer - Freeze USB Clock"]
pub type FRZCLK_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `USBE` reader - USBHS Enable"]
pub type USBE_R = crate::BitReader;
#[doc = "Field `USBE` writer - USBHS Enable"]
pub type USBE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `UID` reader - UID Pin Enable"]
pub type UID_R = crate::BitReader;
#[doc = "Field `UID` writer - UID Pin Enable"]
pub type UID_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `UIMOD` reader - USBHS Mode"]
pub type UIMOD_R = crate::BitReader<UIMODSELECT_A>;
#[doc = "USBHS Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIMODSELECT_A {
    #[doc = "0: The module is in USB Host mode."]
    HOST = 0,
    #[doc = "1: The module is in USB Device mode."]
    DEVICE = 1,
}
impl From<UIMODSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: UIMODSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl UIMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIMODSELECT_A {
        match self.bits {
            false => UIMODSELECT_A::HOST,
            true => UIMODSELECT_A::DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == UIMODSELECT_A::HOST
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == UIMODSELECT_A::DEVICE
    }
}
#[doc = "Field `UIMOD` writer - USBHS Mode"]
pub type UIMOD_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O, UIMODSELECT_A>;
impl<'a, const O: u8> UIMOD_W<'a, O> {
    #[doc = "The module is in USB Host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(UIMODSELECT_A::HOST)
    }
    #[doc = "The module is in USB Device mode."]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(UIMODSELECT_A::DEVICE)
    }
}
impl R {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn rderre(&self) -> RDERRE_R {
        RDERRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&self) -> VBUSHWC_R {
        VBUSHWC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - UID Pin Enable"]
    #[inline(always)]
    pub fn uid(&self) -> UID_R {
        UID_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rderre(&mut self) -> RDERRE_W<4> {
        RDERRE_W::new(self)
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline(always)]
    #[must_use]
    pub fn vbushwc(&mut self) -> VBUSHWC_W<8> {
        VBUSHWC_W::new(self)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    #[must_use]
    pub fn frzclk(&mut self) -> FRZCLK_W<14> {
        FRZCLK_W::new(self)
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbe(&mut self) -> USBE_W<15> {
        USBE_W::new(self)
    }
    #[doc = "Bit 24 - UID Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uid(&mut self) -> UID_W<24> {
        UID_W::new(self)
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uimod(&mut self) -> UIMOD_W<25> {
        UIMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
