#[doc = "Register `OCR` reader"]
pub struct R(crate::R<OCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCR` writer"]
pub struct W(crate::W<OCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCR_SPEC>;
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
impl From<crate::W<OCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL4` reader - Main RC Oscillator Calibration Bits for 4 MHz"]
pub type CAL4_R = crate::FieldReader;
#[doc = "Field `CAL4` writer - Main RC Oscillator Calibration Bits for 4 MHz"]
pub type CAL4_W<'a, const O: u8> = crate::FieldWriter<'a, OCR_SPEC, 7, O>;
#[doc = "Field `SEL4` reader - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
pub type SEL4_R = crate::BitReader;
#[doc = "Field `SEL4` writer - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
pub type SEL4_W<'a, const O: u8> = crate::BitWriter<'a, OCR_SPEC, O>;
#[doc = "Field `CAL8` reader - Main RC Oscillator Calibration Bits for 8 MHz"]
pub type CAL8_R = crate::FieldReader;
#[doc = "Field `CAL8` writer - Main RC Oscillator Calibration Bits for 8 MHz"]
pub type CAL8_W<'a, const O: u8> = crate::FieldWriter<'a, OCR_SPEC, 7, O>;
#[doc = "Field `SEL8` reader - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
pub type SEL8_R = crate::BitReader;
#[doc = "Field `SEL8` writer - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
pub type SEL8_W<'a, const O: u8> = crate::BitWriter<'a, OCR_SPEC, O>;
#[doc = "Field `CAL12` reader - Main RC Oscillator Calibration Bits for 12 MHz"]
pub type CAL12_R = crate::FieldReader;
#[doc = "Field `CAL12` writer - Main RC Oscillator Calibration Bits for 12 MHz"]
pub type CAL12_W<'a, const O: u8> = crate::FieldWriter<'a, OCR_SPEC, 7, O>;
#[doc = "Field `SEL12` reader - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
pub type SEL12_R = crate::BitReader;
#[doc = "Field `SEL12` writer - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
pub type SEL12_W<'a, const O: u8> = crate::BitWriter<'a, OCR_SPEC, O>;
impl R {
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&self) -> CAL4_R {
        CAL4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> CAL8_R {
        CAL8_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&self) -> CAL12_R {
        CAL12_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal4(&mut self) -> CAL4_W<0> {
        CAL4_W::new(self)
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL4_W<7> {
        SEL4_W::new(self)
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal8(&mut self) -> CAL8_W<8> {
        CAL8_W::new(self)
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> SEL8_W<15> {
        SEL8_W::new(self)
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal12(&mut self) -> CAL12_W<16> {
        CAL12_W::new(self)
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL12_W<23> {
        SEL12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr](index.html) module"]
pub struct OCR_SPEC;
impl crate::RegisterSpec for OCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocr::R](R) reader structure"]
impl crate::Readable for OCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocr::W](W) writer structure"]
impl crate::Writable for OCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR to value 0"]
impl crate::Resettable for OCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
