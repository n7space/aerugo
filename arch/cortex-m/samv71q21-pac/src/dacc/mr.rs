#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXS0` reader - Max Speed Mode for Channel 0"]
pub type MAXS0_R = crate::BitReader<MAXS0SELECT_A>;
#[doc = "Max Speed Mode for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAXS0SELECT_A {
    #[doc = "0: External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT = 0,
    #[doc = "1: Max speed mode enabled."]
    MAXIMUM = 1,
}
impl From<MAXS0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MAXS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXS0SELECT_A {
        match self.bits {
            false => MAXS0SELECT_A::TRIG_EVENT,
            true => MAXS0SELECT_A::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        *self == MAXS0SELECT_A::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS0SELECT_A::MAXIMUM
    }
}
#[doc = "Field `MAXS0` writer - Max Speed Mode for Channel 0"]
pub type MAXS0_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O, MAXS0SELECT_A>;
impl<'a, const O: u8> MAXS0_W<'a, O> {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS0SELECT_A::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS0SELECT_A::MAXIMUM)
    }
}
#[doc = "Field `MAXS1` reader - Max Speed Mode for Channel 1"]
pub type MAXS1_R = crate::BitReader<MAXS1SELECT_A>;
#[doc = "Max Speed Mode for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAXS1SELECT_A {
    #[doc = "0: External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT = 0,
    #[doc = "1: Max speed mode enabled."]
    MAXIMUM = 1,
}
impl From<MAXS1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MAXS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXS1SELECT_A {
        match self.bits {
            false => MAXS1SELECT_A::TRIG_EVENT,
            true => MAXS1SELECT_A::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        *self == MAXS1SELECT_A::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS1SELECT_A::MAXIMUM
    }
}
#[doc = "Field `MAXS1` writer - Max Speed Mode for Channel 1"]
pub type MAXS1_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O, MAXS1SELECT_A>;
impl<'a, const O: u8> MAXS1_W<'a, O> {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS1SELECT_A::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS1SELECT_A::MAXIMUM)
    }
}
#[doc = "Field `WORD` reader - Word Transfer Mode"]
pub type WORD_R = crate::BitReader<WORDSELECT_A>;
#[doc = "Word Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WORDSELECT_A {
    #[doc = "0: One data to convert is written to the FIFO per access to DACC."]
    DISABLED = 0,
    #[doc = "1: Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    ENABLED = 1,
}
impl From<WORDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WORDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORDSELECT_A {
        match self.bits {
            false => WORDSELECT_A::DISABLED,
            true => WORDSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WORDSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WORDSELECT_A::ENABLED
    }
}
#[doc = "Field `WORD` writer - Word Transfer Mode"]
pub type WORD_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O, WORDSELECT_A>;
impl<'a, const O: u8> WORD_W<'a, O> {
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WORDSELECT_A::DISABLED)
    }
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WORDSELECT_A::ENABLED)
    }
}
#[doc = "Field `ZERO` reader - Must always be written to 0."]
pub type ZERO_R = crate::BitReader;
#[doc = "Field `ZERO` writer - Must always be written to 0."]
pub type ZERO_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DIFF_R = crate::BitReader<DIFFSELECT_A>;
#[doc = "Differential Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFFSELECT_A {
    #[doc = "0: DAC0 and DAC1 are single-ended outputs."]
    DISABLED = 0,
    #[doc = "1: DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    ENABLED = 1,
}
impl From<DIFFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DIFFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DIFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFFSELECT_A {
        match self.bits {
            false => DIFFSELECT_A::DISABLED,
            true => DIFFSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIFFSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIFFSELECT_A::ENABLED
    }
}
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DIFF_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O, DIFFSELECT_A>;
impl<'a, const O: u8> DIFF_W<'a, O> {
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIFFSELECT_A::DISABLED)
    }
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIFFSELECT_A::ENABLED)
    }
}
#[doc = "Field `PRESCALER` reader - Peripheral Clock to DAC Clock Ratio"]
pub type PRESCALER_R = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - Peripheral Clock to DAC Clock Ratio"]
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, MR_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&self) -> MAXS0_R {
        MAXS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&self) -> MAXS1_R {
        MAXS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn maxs0(&mut self) -> MAXS0_W<0> {
        MAXS0_W::new(self)
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn maxs1(&mut self) -> MAXS1_W<1> {
        MAXS1_W::new(self)
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn word(&mut self) -> WORD_W<4> {
        WORD_W::new(self)
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZERO_W<5> {
        ZERO_W::new(self)
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<23> {
        DIFF_W::new(self)
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<24> {
        PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
