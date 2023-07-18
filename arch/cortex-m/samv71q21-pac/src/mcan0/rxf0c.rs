#[doc = "Register `RXF0C` reader"]
pub struct R(crate::R<RXF0C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXF0C` writer"]
pub struct W(crate::W<RXF0C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF0C_SPEC>;
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
impl From<crate::W<RXF0C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF0C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0SA` reader - Receive FIFO 0 Start Address"]
pub type F0SA_R = crate::FieldReader<u16>;
#[doc = "Field `F0SA` writer - Receive FIFO 0 Start Address"]
pub type F0SA_W<'a, const O: u8> = crate::FieldWriter<'a, RXF0C_SPEC, 14, O, u16>;
#[doc = "Field `F0S` reader - Receive FIFO 0 Start Address"]
pub type F0S_R = crate::FieldReader;
#[doc = "Field `F0S` writer - Receive FIFO 0 Start Address"]
pub type F0S_W<'a, const O: u8> = crate::FieldWriter<'a, RXF0C_SPEC, 7, O>;
#[doc = "Field `F0WM` reader - Receive FIFO 0 Watermark"]
pub type F0WM_R = crate::FieldReader;
#[doc = "Field `F0WM` writer - Receive FIFO 0 Watermark"]
pub type F0WM_W<'a, const O: u8> = crate::FieldWriter<'a, RXF0C_SPEC, 7, O>;
#[doc = "Field `F0OM` reader - FIFO 0 Operation Mode"]
pub type F0OM_R = crate::BitReader;
#[doc = "Field `F0OM` writer - FIFO 0 Operation Mode"]
pub type F0OM_W<'a, const O: u8> = crate::BitWriter<'a, RXF0C_SPEC, O>;
impl R {
    #[doc = "Bits 2:15 - Receive FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Receive FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Receive FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Receive FIFO 0 Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn f0sa(&mut self) -> F0SA_W<2> {
        F0SA_W::new(self)
    }
    #[doc = "Bits 16:22 - Receive FIFO 0 Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn f0s(&mut self) -> F0S_W<16> {
        F0S_W::new(self)
    }
    #[doc = "Bits 24:30 - Receive FIFO 0 Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn f0wm(&mut self) -> F0WM_W<24> {
        F0WM_W::new(self)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<31> {
        F0OM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO 0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0c](index.html) module"]
pub struct RXF0C_SPEC;
impl crate::RegisterSpec for RXF0C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf0c::R](R) reader structure"]
impl crate::Readable for RXF0C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxf0c::W](W) writer structure"]
impl crate::Writable for RXF0C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXF0C to value 0"]
impl crate::Resettable for RXF0C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
