#[doc = "Register `TOCC` reader"]
pub struct R(crate::R<TOCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCC` writer"]
pub struct W(crate::W<TOCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCC_SPEC>;
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
impl From<crate::W<TOCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETOC` reader - Enable Timeout Counter"]
pub type ETOC_R = crate::BitReader<ETOCSELECT_A>;
#[doc = "Enable Timeout Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETOCSELECT_A {
    #[doc = "0: Timeout Counter disabled."]
    NO_TIMEOUT = 0,
    #[doc = "1: Timeout Counter enabled."]
    TOS_CONTROLLED = 1,
}
impl From<ETOCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ETOCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ETOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETOCSELECT_A {
        match self.bits {
            false => ETOCSELECT_A::NO_TIMEOUT,
            true => ETOCSELECT_A::TOS_CONTROLLED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == ETOCSELECT_A::NO_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `TOS_CONTROLLED`"]
    #[inline(always)]
    pub fn is_tos_controlled(&self) -> bool {
        *self == ETOCSELECT_A::TOS_CONTROLLED
    }
}
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub type ETOC_W<'a, const O: u8> = crate::BitWriter<'a, TOCC_SPEC, O, ETOCSELECT_A>;
impl<'a, const O: u8> ETOC_W<'a, O> {
    #[doc = "Timeout Counter disabled."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(ETOCSELECT_A::NO_TIMEOUT)
    }
    #[doc = "Timeout Counter enabled."]
    #[inline(always)]
    pub fn tos_controlled(self) -> &'a mut W {
        self.variant(ETOCSELECT_A::TOS_CONTROLLED)
    }
}
#[doc = "Field `TOS` reader - Timeout Select"]
pub type TOS_R = crate::FieldReader<TOSSELECT_A>;
#[doc = "Timeout Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOSSELECT_A {
    #[doc = "0: Continuous operation"]
    CONTINUOUS = 0,
    #[doc = "1: Timeout controlled by Tx Event FIFO"]
    TX_EV_TIMEOUT = 1,
    #[doc = "2: Timeout controlled by Receive FIFO 0"]
    RX0_EV_TIMEOUT = 2,
    #[doc = "3: Timeout controlled by Receive FIFO 1"]
    RX1_EV_TIMEOUT = 3,
}
impl From<TOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOSSELECT_A {
    type Ux = u8;
}
impl TOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOSSELECT_A {
        match self.bits {
            0 => TOSSELECT_A::CONTINUOUS,
            1 => TOSSELECT_A::TX_EV_TIMEOUT,
            2 => TOSSELECT_A::RX0_EV_TIMEOUT,
            3 => TOSSELECT_A::RX1_EV_TIMEOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TOSSELECT_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TX_EV_TIMEOUT`"]
    #[inline(always)]
    pub fn is_tx_ev_timeout(&self) -> bool {
        *self == TOSSELECT_A::TX_EV_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `RX0_EV_TIMEOUT`"]
    #[inline(always)]
    pub fn is_rx0_ev_timeout(&self) -> bool {
        *self == TOSSELECT_A::RX0_EV_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `RX1_EV_TIMEOUT`"]
    #[inline(always)]
    pub fn is_rx1_ev_timeout(&self) -> bool {
        *self == TOSSELECT_A::RX1_EV_TIMEOUT
    }
}
#[doc = "Field `TOS` writer - Timeout Select"]
pub type TOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, TOCC_SPEC, 2, O, TOSSELECT_A>;
impl<'a, const O: u8> TOS_W<'a, O> {
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TOSSELECT_A::CONTINUOUS)
    }
    #[doc = "Timeout controlled by Tx Event FIFO"]
    #[inline(always)]
    pub fn tx_ev_timeout(self) -> &'a mut W {
        self.variant(TOSSELECT_A::TX_EV_TIMEOUT)
    }
    #[doc = "Timeout controlled by Receive FIFO 0"]
    #[inline(always)]
    pub fn rx0_ev_timeout(self) -> &'a mut W {
        self.variant(TOSSELECT_A::RX0_EV_TIMEOUT)
    }
    #[doc = "Timeout controlled by Receive FIFO 1"]
    #[inline(always)]
    pub fn rx1_ev_timeout(self) -> &'a mut W {
        self.variant(TOSSELECT_A::RX1_EV_TIMEOUT)
    }
}
#[doc = "Field `TOP` reader - Timeout Period"]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout Period"]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, TOCC_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> ETOC_W<0> {
        ETOC_W::new(self)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<1> {
        TOS_W::new(self)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<16> {
        TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocc](index.html) module"]
pub struct TOCC_SPEC;
impl crate::RegisterSpec for TOCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tocc::R](R) reader structure"]
impl crate::Readable for TOCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocc::W](W) writer structure"]
impl crate::Writable for TOCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOCC to value 0"]
impl crate::Resettable for TOCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
