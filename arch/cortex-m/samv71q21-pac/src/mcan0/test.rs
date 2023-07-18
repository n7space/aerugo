#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBCK` reader - Loop Back Mode (read/write)"]
pub type LBCK_R = crate::BitReader<LBCKSELECT_A>;
#[doc = "Loop Back Mode (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCKSELECT_A {
    #[doc = "0: Reset value. Loop Back mode is disabled."]
    DISABLED = 0,
    #[doc = "1: Loop Back mode is enabled (see Section 6.1.9)."]
    ENABLED = 1,
}
impl From<LBCKSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LBCKSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LBCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBCKSELECT_A {
        match self.bits {
            false => LBCKSELECT_A::DISABLED,
            true => LBCKSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBCKSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBCKSELECT_A::ENABLED
    }
}
#[doc = "Field `LBCK` writer - Loop Back Mode (read/write)"]
pub type LBCK_W<'a, const O: u8> = crate::BitWriter<'a, TEST_SPEC, O, LBCKSELECT_A>;
impl<'a, const O: u8> LBCK_W<'a, O> {
    #[doc = "Reset value. Loop Back mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBCKSELECT_A::DISABLED)
    }
    #[doc = "Loop Back mode is enabled (see Section 6.1.9)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBCKSELECT_A::ENABLED)
    }
}
#[doc = "Field `TX` reader - Control of Transmit Pin (read/write)"]
pub type TX_R = crate::FieldReader<TXSELECT_A>;
#[doc = "Control of Transmit Pin (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXSELECT_A {
    #[doc = "0: Reset value, CANTX controlled by the CAN Core, updated at the end of the CAN bit time."]
    RESET = 0,
    #[doc = "1: Sample Point can be monitored at pin CANTX."]
    SAMPLE_POINT_MONITORING = 1,
    #[doc = "2: Dominant ('0') level at pin CANTX."]
    DOMINANT = 2,
    #[doc = "3: Recessive ('1') at pin CANTX."]
    RECESSIVE = 3,
}
impl From<TXSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TXSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXSELECT_A {
    type Ux = u8;
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSELECT_A {
        match self.bits {
            0 => TXSELECT_A::RESET,
            1 => TXSELECT_A::SAMPLE_POINT_MONITORING,
            2 => TXSELECT_A::DOMINANT,
            3 => TXSELECT_A::RECESSIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TXSELECT_A::RESET
    }
    #[doc = "Checks if the value of the field is `SAMPLE_POINT_MONITORING`"]
    #[inline(always)]
    pub fn is_sample_point_monitoring(&self) -> bool {
        *self == TXSELECT_A::SAMPLE_POINT_MONITORING
    }
    #[doc = "Checks if the value of the field is `DOMINANT`"]
    #[inline(always)]
    pub fn is_dominant(&self) -> bool {
        *self == TXSELECT_A::DOMINANT
    }
    #[doc = "Checks if the value of the field is `RECESSIVE`"]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == TXSELECT_A::RECESSIVE
    }
}
#[doc = "Field `TX` writer - Control of Transmit Pin (read/write)"]
pub type TX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, TEST_SPEC, 2, O, TXSELECT_A>;
impl<'a, const O: u8> TX_W<'a, O> {
    #[doc = "Reset value, CANTX controlled by the CAN Core, updated at the end of the CAN bit time."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TXSELECT_A::RESET)
    }
    #[doc = "Sample Point can be monitored at pin CANTX."]
    #[inline(always)]
    pub fn sample_point_monitoring(self) -> &'a mut W {
        self.variant(TXSELECT_A::SAMPLE_POINT_MONITORING)
    }
    #[doc = "Dominant ('0') level at pin CANTX."]
    #[inline(always)]
    pub fn dominant(self) -> &'a mut W {
        self.variant(TXSELECT_A::DOMINANT)
    }
    #[doc = "Recessive ('1') at pin CANTX."]
    #[inline(always)]
    pub fn recessive(self) -> &'a mut W {
        self.variant(TXSELECT_A::RECESSIVE)
    }
}
#[doc = "Field `RX` reader - Receive Pin (read-only)"]
pub type RX_R = crate::BitReader;
#[doc = "Field `RX` writer - Receive Pin (read-only)"]
pub type RX_W<'a, const O: u8> = crate::BitWriter<'a, TEST_SPEC, O>;
impl R {
    #[doc = "Bit 4 - Loop Back Mode (read/write)"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin (read/write)"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin (read-only)"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<4> {
        LBCK_W::new(self)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<5> {
        TX_W::new(self)
    }
    #[doc = "Bit 7 - Receive Pin (read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<7> {
        RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
