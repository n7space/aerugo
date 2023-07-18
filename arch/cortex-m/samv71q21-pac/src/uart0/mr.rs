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
#[doc = "Field `FILTER` reader - Receiver Digital Filter"]
pub type FILTER_R = crate::BitReader<FILTERSELECT_A>;
#[doc = "Receiver Digital Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTERSELECT_A {
    #[doc = "0: UART does not filter the receive line."]
    DISABLED = 0,
    #[doc = "1: UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    ENABLED = 1,
}
impl From<FILTERSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FILTERSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTERSELECT_A {
        match self.bits {
            false => FILTERSELECT_A::DISABLED,
            true => FILTERSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FILTERSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FILTERSELECT_A::ENABLED
    }
}
#[doc = "Field `FILTER` writer - Receiver Digital Filter"]
pub type FILTER_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O, FILTERSELECT_A>;
impl<'a, const O: u8> FILTER_W<'a, O> {
    #[doc = "UART does not filter the receive line."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTERSELECT_A::DISABLED)
    }
    #[doc = "UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTERSELECT_A::ENABLED)
    }
}
#[doc = "Field `PAR` reader - Parity Type"]
pub type PAR_R = crate::FieldReader<PARSELECT_A>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARSELECT_A {
    #[doc = "0: Even Parity"]
    EVEN = 0,
    #[doc = "1: Odd Parity"]
    ODD = 1,
    #[doc = "2: Space: parity forced to 0"]
    SPACE = 2,
    #[doc = "3: Mark: parity forced to 1"]
    MARK = 3,
    #[doc = "4: No parity"]
    NO = 4,
}
impl From<PARSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PARSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PARSELECT_A {
    type Ux = u8;
}
impl PAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARSELECT_A> {
        match self.bits {
            0 => Some(PARSELECT_A::EVEN),
            1 => Some(PARSELECT_A::ODD),
            2 => Some(PARSELECT_A::SPACE),
            3 => Some(PARSELECT_A::MARK),
            4 => Some(PARSELECT_A::NO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARSELECT_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARSELECT_A::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == PARSELECT_A::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == PARSELECT_A::MARK
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PARSELECT_A::NO
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type PAR_W<'a, const O: u8> = crate::FieldWriter<'a, MR_SPEC, 3, O, PARSELECT_A>;
impl<'a, const O: u8> PAR_W<'a, O> {
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARSELECT_A::EVEN)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARSELECT_A::ODD)
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn space(self) -> &'a mut W {
        self.variant(PARSELECT_A::SPACE)
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut W {
        self.variant(PARSELECT_A::MARK)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PARSELECT_A::NO)
    }
}
#[doc = "Field `BRSRCCK` reader - Baud Rate Source Clock"]
pub type BRSRCCK_R = crate::BitReader<BRSRCCKSELECT_A>;
#[doc = "Baud Rate Source Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSRCCKSELECT_A {
    #[doc = "0: The baud rate is driven by the peripheral clock"]
    PERIPH_CLK = 0,
    #[doc = "1: The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    PMC_PCK = 1,
}
impl From<BRSRCCKSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BRSRCCKSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BRSRCCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSRCCKSELECT_A {
        match self.bits {
            false => BRSRCCKSELECT_A::PERIPH_CLK,
            true => BRSRCCKSELECT_A::PMC_PCK,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK`"]
    #[inline(always)]
    pub fn is_periph_clk(&self) -> bool {
        *self == BRSRCCKSELECT_A::PERIPH_CLK
    }
    #[doc = "Checks if the value of the field is `PMC_PCK`"]
    #[inline(always)]
    pub fn is_pmc_pck(&self) -> bool {
        *self == BRSRCCKSELECT_A::PMC_PCK
    }
}
#[doc = "Field `BRSRCCK` writer - Baud Rate Source Clock"]
pub type BRSRCCK_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O, BRSRCCKSELECT_A>;
impl<'a, const O: u8> BRSRCCK_W<'a, O> {
    #[doc = "The baud rate is driven by the peripheral clock"]
    #[inline(always)]
    pub fn periph_clk(self) -> &'a mut W {
        self.variant(BRSRCCKSELECT_A::PERIPH_CLK)
    }
    #[doc = "The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    #[inline(always)]
    pub fn pmc_pck(self) -> &'a mut W {
        self.variant(BRSRCCKSELECT_A::PMC_PCK)
    }
}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type CHMODE_R = crate::FieldReader<CHMODESELECT_A>;
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMODESELECT_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Automatic echo"]
    AUTOMATIC = 1,
    #[doc = "2: Local loopback"]
    LOCAL_LOOPBACK = 2,
    #[doc = "3: Remote loopback"]
    REMOTE_LOOPBACK = 3,
}
impl From<CHMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHMODESELECT_A {
    type Ux = u8;
}
impl CHMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMODESELECT_A {
        match self.bits {
            0 => CHMODESELECT_A::NORMAL,
            1 => CHMODESELECT_A::AUTOMATIC,
            2 => CHMODESELECT_A::LOCAL_LOOPBACK,
            3 => CHMODESELECT_A::REMOTE_LOOPBACK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHMODESELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == CHMODESELECT_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOPBACK`"]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == CHMODESELECT_A::LOCAL_LOOPBACK
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOPBACK`"]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == CHMODESELECT_A::REMOTE_LOOPBACK
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type CHMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MR_SPEC, 2, O, CHMODESELECT_A>;
impl<'a, const O: u8> CHMODE_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODESELECT_A::NORMAL)
    }
    #[doc = "Automatic echo"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(CHMODESELECT_A::AUTOMATIC)
    }
    #[doc = "Local loopback"]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut W {
        self.variant(CHMODESELECT_A::LOCAL_LOOPBACK)
    }
    #[doc = "Remote loopback"]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut W {
        self.variant(CHMODESELECT_A::REMOTE_LOOPBACK)
    }
}
impl R {
    #[doc = "Bit 4 - Receiver Digital Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Baud Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcck(&self) -> BRSRCCK_R {
        BRSRCCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> CHMODE_R {
        CHMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Receiver Digital Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<4> {
        FILTER_W::new(self)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn par(&mut self) -> PAR_W<9> {
        PAR_W::new(self)
    }
    #[doc = "Bit 12 - Baud Rate Source Clock"]
    #[inline(always)]
    #[must_use]
    pub fn brsrcck(&mut self) -> BRSRCCK_W<12> {
        BRSRCCK_W::new(self)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chmode(&mut self) -> CHMODE_W<14> {
        CHMODE_W::new(self)
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
