#[doc = "Register `DMA` reader"]
pub struct R(crate::R<DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA` writer"]
pub struct W(crate::W<DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SPEC>;
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
impl From<crate::W<DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHKSIZE` reader - DMA Channel Read and Write Chunk Size"]
pub type CHKSIZE_R = crate::FieldReader<CHKSIZESELECT_A>;
#[doc = "DMA Channel Read and Write Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHKSIZESELECT_A {
    #[doc = "0: 1 data available"]
    _1 = 0,
    #[doc = "1: 2 data available"]
    _2 = 1,
    #[doc = "2: 4 data available"]
    _4 = 2,
    #[doc = "3: 8 data available"]
    _8 = 3,
    #[doc = "4: 16 data available"]
    _16 = 4,
}
impl From<CHKSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHKSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHKSIZESELECT_A {
    type Ux = u8;
}
impl CHKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHKSIZESELECT_A> {
        match self.bits {
            0 => Some(CHKSIZESELECT_A::_1),
            1 => Some(CHKSIZESELECT_A::_2),
            2 => Some(CHKSIZESELECT_A::_4),
            3 => Some(CHKSIZESELECT_A::_8),
            4 => Some(CHKSIZESELECT_A::_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHKSIZESELECT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == CHKSIZESELECT_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == CHKSIZESELECT_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CHKSIZESELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CHKSIZESELECT_A::_16
    }
}
#[doc = "Field `CHKSIZE` writer - DMA Channel Read and Write Chunk Size"]
pub type CHKSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, DMA_SPEC, 3, O, CHKSIZESELECT_A>;
impl<'a, const O: u8> CHKSIZE_W<'a, O> {
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHKSIZESELECT_A::_1)
    }
    #[doc = "2 data available"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CHKSIZESELECT_A::_2)
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CHKSIZESELECT_A::_4)
    }
    #[doc = "8 data available"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHKSIZESELECT_A::_8)
    }
    #[doc = "16 data available"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CHKSIZESELECT_A::_16)
    }
}
#[doc = "Field `DMAEN` reader - DMA Hardware Handshaking Enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Hardware Handshaking Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_SPEC, O>;
impl R {
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&self) -> CHKSIZE_R {
        CHKSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    #[must_use]
    pub fn chksize(&mut self) -> CHKSIZE_W<4> {
        CHKSIZE_W::new(self)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<8> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](index.html) module"]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma::R](R) reader structure"]
impl crate::Readable for DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma::W](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
