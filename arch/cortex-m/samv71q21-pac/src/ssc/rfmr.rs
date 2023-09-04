#[doc = "Register `RFMR` reader"]
pub type R = crate::R<RFMR_SPEC>;
#[doc = "Register `RFMR` writer"]
pub type W = crate::W<RFMR_SPEC>;
#[doc = "Field `DATLEN` reader - Data Length"]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data Length"]
pub type DATLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `LOOP` reader - Loop Mode"]
pub type LOOP_R = crate::BitReader;
#[doc = "Field `LOOP` writer - Loop Mode"]
pub type LOOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATNB` reader - Data Number per Frame"]
pub type DATNB_R = crate::FieldReader;
#[doc = "Field `DATNB` writer - Data Number per Frame"]
pub type DATNB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FSLEN` reader - Receive Frame Sync Length"]
pub type FSLEN_R = crate::FieldReader;
#[doc = "Field `FSLEN` writer - Receive Frame Sync Length"]
pub type FSLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FSOS` reader - Receive Frame Sync Output Selection"]
pub type FSOS_R = crate::FieldReader<FSOSSELECT_A>;
#[doc = "Receive Frame Sync Output Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSOSSELECT_A {
    #[doc = "0: None, RF pin is an input"]
    NONE = 0,
    #[doc = "1: Negative Pulse, RF pin is an output"]
    NEGATIVE = 1,
    #[doc = "2: Positive Pulse, RF pin is an output"]
    POSITIVE = 2,
    #[doc = "3: Driven Low during data transfer, RF pin is an output"]
    LOW = 3,
    #[doc = "4: Driven High during data transfer, RF pin is an output"]
    HIGH = 4,
    #[doc = "5: Toggling at each start of data transfer, RF pin is an output"]
    TOGGLING = 5,
}
impl From<FSOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FSOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSOSSELECT_A {
    type Ux = u8;
}
impl FSOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSOSSELECT_A> {
        match self.bits {
            0 => Some(FSOSSELECT_A::NONE),
            1 => Some(FSOSSELECT_A::NEGATIVE),
            2 => Some(FSOSSELECT_A::POSITIVE),
            3 => Some(FSOSSELECT_A::LOW),
            4 => Some(FSOSSELECT_A::HIGH),
            5 => Some(FSOSSELECT_A::TOGGLING),
            _ => None,
        }
    }
    #[doc = "None, RF pin is an input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FSOSSELECT_A::NONE
    }
    #[doc = "Negative Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSOSSELECT_A::NEGATIVE
    }
    #[doc = "Positive Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSOSSELECT_A::POSITIVE
    }
    #[doc = "Driven Low during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FSOSSELECT_A::LOW
    }
    #[doc = "Driven High during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FSOSSELECT_A::HIGH
    }
    #[doc = "Toggling at each start of data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        *self == FSOSSELECT_A::TOGGLING
    }
}
#[doc = "Field `FSOS` writer - Receive Frame Sync Output Selection"]
pub type FSOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, FSOSSELECT_A>;
impl<'a, REG, const O: u8> FSOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None, RF pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(FSOSSELECT_A::NONE)
    }
    #[doc = "Negative Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(FSOSSELECT_A::NEGATIVE)
    }
    #[doc = "Positive Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(FSOSSELECT_A::POSITIVE)
    }
    #[doc = "Driven Low during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(FSOSSELECT_A::LOW)
    }
    #[doc = "Driven High during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(FSOSSELECT_A::HIGH)
    }
    #[doc = "Toggling at each start of data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut crate::W<REG> {
        self.variant(FSOSSELECT_A::TOGGLING)
    }
}
#[doc = "Field `FSEDGE` reader - Frame Sync Edge Detection"]
pub type FSEDGE_R = crate::BitReader<FSEDGESELECT_A>;
#[doc = "Frame Sync Edge Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSEDGESELECT_A {
    #[doc = "0: Positive Edge Detection"]
    POSITIVE = 0,
    #[doc = "1: Negative Edge Detection"]
    NEGATIVE = 1,
}
impl From<FSEDGESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FSEDGESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FSEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEDGESELECT_A {
        match self.bits {
            false => FSEDGESELECT_A::POSITIVE,
            true => FSEDGESELECT_A::NEGATIVE,
        }
    }
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSEDGESELECT_A::POSITIVE
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSEDGESELECT_A::NEGATIVE
    }
}
#[doc = "Field `FSEDGE` writer - Frame Sync Edge Detection"]
pub type FSEDGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FSEDGESELECT_A>;
impl<'a, REG, const O: u8> FSEDGE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(FSEDGESELECT_A::POSITIVE)
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(FSEDGESELECT_A::NEGATIVE)
    }
}
#[doc = "Field `FSLEN_EXT` reader - FSLEN Field Extension"]
pub type FSLEN_EXT_R = crate::FieldReader;
#[doc = "Field `FSLEN_EXT` writer - FSLEN Field Extension"]
pub type FSLEN_EXT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    pub fn datnb(&self) -> DATNB_R {
        DATNB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&self) -> FSLEN_R {
        FSLEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&self) -> FSOS_R {
        FSOS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    pub fn fsedge(&self) -> FSEDGE_R {
        FSEDGE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    pub fn fslen_ext(&self) -> FSLEN_EXT_R {
        FSLEN_EXT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<RFMR_SPEC, 0> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LOOP_W<RFMR_SPEC, 5> {
        LOOP_W::new(self)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<RFMR_SPEC, 7> {
        MSBF_W::new(self)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    #[must_use]
    pub fn datnb(&mut self) -> DATNB_W<RFMR_SPEC, 8> {
        DATNB_W::new(self)
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    #[must_use]
    pub fn fslen(&mut self) -> FSLEN_W<RFMR_SPEC, 16> {
        FSLEN_W::new(self)
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fsos(&mut self) -> FSOS_W<RFMR_SPEC, 20> {
        FSOS_W::new(self)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    #[must_use]
    pub fn fsedge(&mut self) -> FSEDGE_W<RFMR_SPEC, 24> {
        FSEDGE_W::new(self)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    #[must_use]
    pub fn fslen_ext(&mut self) -> FSLEN_EXT_W<RFMR_SPEC, 28> {
        FSLEN_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFMR_SPEC;
impl crate::RegisterSpec for RFMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfmr::R`](R) reader structure"]
impl crate::Readable for RFMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfmr::W`](W) writer structure"]
impl crate::Writable for RFMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFMR to value 0"]
impl crate::Resettable for RFMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
