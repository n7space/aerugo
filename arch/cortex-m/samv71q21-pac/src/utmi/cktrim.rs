#[doc = "Register `CKTRIM` reader"]
pub type R = crate::R<CKTRIM_SPEC>;
#[doc = "Register `CKTRIM` writer"]
pub type W = crate::W<CKTRIM_SPEC>;
#[doc = "Field `FREQ` reader - UTMI Reference Clock Frequency"]
pub type FREQ_R = crate::FieldReader<FREQSELECT_A>;
#[doc = "UTMI Reference Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FREQSELECT_A {
    #[doc = "0: 12 MHz reference clock"]
    XTAL12 = 0,
    #[doc = "1: 16 MHz reference clock"]
    XTAL16 = 1,
}
impl From<FREQSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FREQSELECT_A {
    type Ux = u8;
}
impl FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQSELECT_A> {
        match self.bits {
            0 => Some(FREQSELECT_A::XTAL12),
            1 => Some(FREQSELECT_A::XTAL16),
            _ => None,
        }
    }
    #[doc = "12 MHz reference clock"]
    #[inline(always)]
    pub fn is_xtal12(&self) -> bool {
        *self == FREQSELECT_A::XTAL12
    }
    #[doc = "16 MHz reference clock"]
    #[inline(always)]
    pub fn is_xtal16(&self) -> bool {
        *self == FREQSELECT_A::XTAL16
    }
}
#[doc = "Field `FREQ` writer - UTMI Reference Clock Frequency"]
pub type FREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, FREQSELECT_A>;
impl<'a, REG, const O: u8> FREQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12 MHz reference clock"]
    #[inline(always)]
    pub fn xtal12(self) -> &'a mut crate::W<REG> {
        self.variant(FREQSELECT_A::XTAL12)
    }
    #[doc = "16 MHz reference clock"]
    #[inline(always)]
    pub fn xtal16(self) -> &'a mut crate::W<REG> {
        self.variant(FREQSELECT_A::XTAL16)
    }
}
impl R {
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<CKTRIM_SPEC, 0> {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UTMI Clock Trimming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cktrim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cktrim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKTRIM_SPEC;
impl crate::RegisterSpec for CKTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cktrim::R`](R) reader structure"]
impl crate::Readable for CKTRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cktrim::W`](W) writer structure"]
impl crate::Writable for CKTRIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKTRIM to value 0"]
impl crate::Resettable for CKTRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
