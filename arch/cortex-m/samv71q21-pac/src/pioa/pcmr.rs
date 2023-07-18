#[doc = "Register `PCMR` reader"]
pub struct R(crate::R<PCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMR` writer"]
pub struct W(crate::W<PCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMR_SPEC>;
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
impl From<crate::W<PCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCEN` reader - Parallel Capture Mode Enable"]
pub type PCEN_R = crate::BitReader;
#[doc = "Field `PCEN` writer - Parallel Capture Mode Enable"]
pub type PCEN_W<'a, const O: u8> = crate::BitWriter<'a, PCMR_SPEC, O>;
#[doc = "Field `DSIZE` reader - Parallel Capture Mode Data Size"]
pub type DSIZE_R = crate::FieldReader<DSIZESELECT_A>;
#[doc = "Parallel Capture Mode Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSIZESELECT_A {
    #[doc = "0: The reception data in the PIO_PCRHR is a byte (8-bit)"]
    BYTE = 0,
    #[doc = "1: The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    HALFWORD = 1,
    #[doc = "2: The reception data in the PIO_PCRHR is a word (32-bit)"]
    WORD = 2,
}
impl From<DSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSIZESELECT_A {
    type Ux = u8;
}
impl DSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSIZESELECT_A> {
        match self.bits {
            0 => Some(DSIZESELECT_A::BYTE),
            1 => Some(DSIZESELECT_A::HALFWORD),
            2 => Some(DSIZESELECT_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DSIZESELECT_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == DSIZESELECT_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DSIZESELECT_A::WORD
    }
}
#[doc = "Field `DSIZE` writer - Parallel Capture Mode Data Size"]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, PCMR_SPEC, 2, O, DSIZESELECT_A>;
impl<'a, const O: u8> DSIZE_W<'a, O> {
    #[doc = "The reception data in the PIO_PCRHR is a byte (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DSIZESELECT_A::BYTE)
    }
    #[doc = "The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DSIZESELECT_A::HALFWORD)
    }
    #[doc = "The reception data in the PIO_PCRHR is a word (32-bit)"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DSIZESELECT_A::WORD)
    }
}
#[doc = "Field `ALWYS` reader - Parallel Capture Mode Always Sampling"]
pub type ALWYS_R = crate::BitReader;
#[doc = "Field `ALWYS` writer - Parallel Capture Mode Always Sampling"]
pub type ALWYS_W<'a, const O: u8> = crate::BitWriter<'a, PCMR_SPEC, O>;
#[doc = "Field `HALFS` reader - Parallel Capture Mode Half Sampling"]
pub type HALFS_R = crate::BitReader;
#[doc = "Field `HALFS` writer - Parallel Capture Mode Half Sampling"]
pub type HALFS_W<'a, const O: u8> = crate::BitWriter<'a, PCMR_SPEC, O>;
#[doc = "Field `FRSTS` reader - Parallel Capture Mode First Sample"]
pub type FRSTS_R = crate::BitReader;
#[doc = "Field `FRSTS` writer - Parallel Capture Mode First Sample"]
pub type FRSTS_W<'a, const O: u8> = crate::BitWriter<'a, PCMR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    pub fn alwys(&self) -> ALWYS_R {
        ALWYS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    pub fn halfs(&self) -> HALFS_R {
        HALFS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    pub fn frsts(&self) -> FRSTS_R {
        FRSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<0> {
        PCEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<4> {
        DSIZE_W::new(self)
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn alwys(&mut self) -> ALWYS_W<9> {
        ALWYS_W::new(self)
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn halfs(&mut self) -> HALFS_W<10> {
        HALFS_W::new(self)
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    #[must_use]
    pub fn frsts(&mut self) -> FRSTS_W<11> {
        FRSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel Capture Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmr](index.html) module"]
pub struct PCMR_SPEC;
impl crate::RegisterSpec for PCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcmr::R](R) reader structure"]
impl crate::Readable for PCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmr::W](W) writer structure"]
impl crate::Writable for PCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMR to value 0"]
impl crate::Resettable for PCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
