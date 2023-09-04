#[doc = "Register `RCMR` reader"]
pub type R = crate::R<RCMR_SPEC>;
#[doc = "Register `RCMR` writer"]
pub type W = crate::W<RCMR_SPEC>;
#[doc = "Field `CKS` reader - Receive Clock Selection"]
pub type CKS_R = crate::FieldReader<CKSSELECT_A>;
#[doc = "Receive Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKSSELECT_A {
    #[doc = "0: Divided Clock"]
    MCK = 0,
    #[doc = "1: TK Clock signal"]
    TK = 1,
    #[doc = "2: RK pin"]
    RK = 2,
}
impl From<CKSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKSSELECT_A {
    type Ux = u8;
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKSSELECT_A> {
        match self.bits {
            0 => Some(CKSSELECT_A::MCK),
            1 => Some(CKSSELECT_A::TK),
            2 => Some(CKSSELECT_A::RK),
            _ => None,
        }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CKSSELECT_A::MCK
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == CKSSELECT_A::TK
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == CKSSELECT_A::RK
    }
}
#[doc = "Field `CKS` writer - Receive Clock Selection"]
pub type CKS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CKSSELECT_A>;
impl<'a, REG, const O: u8> CKS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(CKSSELECT_A::MCK)
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut crate::W<REG> {
        self.variant(CKSSELECT_A::TK)
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut crate::W<REG> {
        self.variant(CKSSELECT_A::RK)
    }
}
#[doc = "Field `CKO` reader - Receive Clock Output Mode Selection"]
pub type CKO_R = crate::FieldReader<CKOSELECT_A>;
#[doc = "Receive Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKOSELECT_A {
    #[doc = "0: None, RK pin is an input"]
    NONE = 0,
    #[doc = "1: Continuous Receive Clock, RK pin is an output"]
    CONTINUOUS = 1,
    #[doc = "2: Receive Clock only during data transfers, RK pin is an output"]
    TRANSFER = 2,
}
impl From<CKOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKOSELECT_A {
    type Ux = u8;
}
impl CKO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKOSELECT_A> {
        match self.bits {
            0 => Some(CKOSELECT_A::NONE),
            1 => Some(CKOSELECT_A::CONTINUOUS),
            2 => Some(CKOSELECT_A::TRANSFER),
            _ => None,
        }
    }
    #[doc = "None, RK pin is an input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKOSELECT_A::NONE
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKOSELECT_A::CONTINUOUS
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == CKOSELECT_A::TRANSFER
    }
}
#[doc = "Field `CKO` writer - Receive Clock Output Mode Selection"]
pub type CKO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CKOSELECT_A>;
impl<'a, REG, const O: u8> CKO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None, RK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSELECT_A::NONE)
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSELECT_A::CONTINUOUS)
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSELECT_A::TRANSFER)
    }
}
#[doc = "Field `CKI` reader - Receive Clock Inversion"]
pub type CKI_R = crate::BitReader;
#[doc = "Field `CKI` writer - Receive Clock Inversion"]
pub type CKI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKG` reader - Receive Clock Gating Selection"]
pub type CKG_R = crate::FieldReader<CKGSELECT_A>;
#[doc = "Receive Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKGSELECT_A {
    #[doc = "0: None"]
    CONTINUOUS = 0,
    #[doc = "1: Receive Clock enabled only if RF Low"]
    EN_RF_LOW = 1,
    #[doc = "2: Receive Clock enabled only if RF High"]
    EN_RF_HIGH = 2,
}
impl From<CKGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKGSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKGSELECT_A {
    type Ux = u8;
}
impl CKG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKGSELECT_A> {
        match self.bits {
            0 => Some(CKGSELECT_A::CONTINUOUS),
            1 => Some(CKGSELECT_A::EN_RF_LOW),
            2 => Some(CKGSELECT_A::EN_RF_HIGH),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKGSELECT_A::CONTINUOUS
    }
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline(always)]
    pub fn is_en_rf_low(&self) -> bool {
        *self == CKGSELECT_A::EN_RF_LOW
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline(always)]
    pub fn is_en_rf_high(&self) -> bool {
        *self == CKGSELECT_A::EN_RF_HIGH
    }
}
#[doc = "Field `CKG` writer - Receive Clock Gating Selection"]
pub type CKG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CKGSELECT_A>;
impl<'a, REG, const O: u8> CKG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CKGSELECT_A::CONTINUOUS)
    }
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline(always)]
    pub fn en_rf_low(self) -> &'a mut crate::W<REG> {
        self.variant(CKGSELECT_A::EN_RF_LOW)
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline(always)]
    pub fn en_rf_high(self) -> &'a mut crate::W<REG> {
        self.variant(CKGSELECT_A::EN_RF_HIGH)
    }
}
#[doc = "Field `START` reader - Receive Start Selection"]
pub type START_R = crate::FieldReader<STARTSELECT_A>;
#[doc = "Receive Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTSELECT_A {
    #[doc = "0: Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    CONTINUOUS = 0,
    #[doc = "1: Transmit start"]
    TRANSMIT = 1,
    #[doc = "2: Detection of a low level on RF signal"]
    RF_LOW = 2,
    #[doc = "3: Detection of a high level on RF signal"]
    RF_HIGH = 3,
    #[doc = "4: Detection of a falling edge on RF signal"]
    RF_FALLING = 4,
    #[doc = "5: Detection of a rising edge on RF signal"]
    RF_RISING = 5,
    #[doc = "6: Detection of any level change on RF signal"]
    RF_LEVEL = 6,
    #[doc = "7: Detection of any edge on RF signal"]
    RF_EDGE = 7,
    #[doc = "8: Compare 0"]
    CMP_0 = 8,
}
impl From<STARTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STARTSELECT_A {
    type Ux = u8;
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTSELECT_A> {
        match self.bits {
            0 => Some(STARTSELECT_A::CONTINUOUS),
            1 => Some(STARTSELECT_A::TRANSMIT),
            2 => Some(STARTSELECT_A::RF_LOW),
            3 => Some(STARTSELECT_A::RF_HIGH),
            4 => Some(STARTSELECT_A::RF_FALLING),
            5 => Some(STARTSELECT_A::RF_RISING),
            6 => Some(STARTSELECT_A::RF_LEVEL),
            7 => Some(STARTSELECT_A::RF_EDGE),
            8 => Some(STARTSELECT_A::CMP_0),
            _ => None,
        }
    }
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == STARTSELECT_A::CONTINUOUS
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == STARTSELECT_A::TRANSMIT
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn is_rf_low(&self) -> bool {
        *self == STARTSELECT_A::RF_LOW
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn is_rf_high(&self) -> bool {
        *self == STARTSELECT_A::RF_HIGH
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_falling(&self) -> bool {
        *self == STARTSELECT_A::RF_FALLING
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_rising(&self) -> bool {
        *self == STARTSELECT_A::RF_RISING
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn is_rf_level(&self) -> bool {
        *self == STARTSELECT_A::RF_LEVEL
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_edge(&self) -> bool {
        *self == STARTSELECT_A::RF_EDGE
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn is_cmp_0(&self) -> bool {
        *self == STARTSELECT_A::CMP_0
    }
}
#[doc = "Field `START` writer - Receive Start Selection"]
pub type START_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, STARTSELECT_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::CONTINUOUS)
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::TRANSMIT)
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn rf_low(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::RF_LOW)
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn rf_high(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::RF_HIGH)
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn rf_falling(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::RF_FALLING)
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn rf_rising(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::RF_RISING)
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn rf_level(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::RF_LEVEL)
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn rf_edge(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::RF_EDGE)
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn cmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(STARTSELECT_A::CMP_0)
    }
}
#[doc = "Field `STOP` reader - Receive Stop Selection"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Receive Stop Selection"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STTDLY` reader - Receive Start Delay"]
pub type STTDLY_R = crate::FieldReader;
#[doc = "Field `STTDLY` writer - Receive Start Delay"]
pub type STTDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PERIOD` reader - Receive Period Divider Selection"]
pub type PERIOD_R = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Receive Period Divider Selection"]
pub type PERIOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CKO_R {
        CKO_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CKI_R {
        CKI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> STTDLY_R {
        STTDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<RCMR_SPEC, 0> {
        CKS_W::new(self)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cko(&mut self) -> CKO_W<RCMR_SPEC, 2> {
        CKO_W::new(self)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn cki(&mut self) -> CKI_W<RCMR_SPEC, 5> {
        CKI_W::new(self)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckg(&mut self) -> CKG_W<RCMR_SPEC, 6> {
        CKG_W::new(self)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<RCMR_SPEC, 8> {
        START_W::new(self)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<RCMR_SPEC, 12> {
        STOP_W::new(self)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sttdly(&mut self) -> STTDLY_W<RCMR_SPEC, 16> {
        STTDLY_W::new(self)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<RCMR_SPEC, 24> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCMR_SPEC;
impl crate::RegisterSpec for RCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcmr::R`](R) reader structure"]
impl crate::Readable for RCMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcmr::W`](W) writer structure"]
impl crate::Writable for RCMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCMR to value 0"]
impl crate::Resettable for RCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
