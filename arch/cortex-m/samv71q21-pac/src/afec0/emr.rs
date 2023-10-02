#[doc = "Register `EMR` reader"]
pub type R = crate::R<EMR_SPEC>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EMR_SPEC>;
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CMPMODE_R = crate::FieldReader<CMPMODESELECT_A>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMODESELECT_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    LOW = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    IN = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    OUT = 3,
}
impl From<CMPMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPMODESELECT_A {
    type Ux = u8;
}
impl CMPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODESELECT_A {
        match self.bits {
            0 => CMPMODESELECT_A::LOW,
            1 => CMPMODESELECT_A::HIGH,
            2 => CMPMODESELECT_A::IN,
            3 => CMPMODESELECT_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMPMODESELECT_A::LOW
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMPMODESELECT_A::HIGH
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == CMPMODESELECT_A::IN
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == CMPMODESELECT_A::OUT
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CMPMODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CMPMODESELECT_A>;
impl<'a, REG, const O: u8> CMPMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODESELECT_A::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODESELECT_A::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODESELECT_A::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODESELECT_A::OUT)
    }
}
#[doc = "Field `CMPSEL` reader - Comparison Selected Channel"]
pub type CMPSEL_R = crate::FieldReader;
#[doc = "Field `CMPSEL` writer - Comparison Selected Channel"]
pub type CMPSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CMPALL` reader - Compare All Channels"]
pub type CMPALL_R = crate::BitReader;
#[doc = "Field `CMPALL` writer - Compare All Channels"]
pub type CMPALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPFILTER` reader - Compare Event Filtering"]
pub type CMPFILTER_R = crate::FieldReader;
#[doc = "Field `CMPFILTER` writer - Compare Event Filtering"]
pub type CMPFILTER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RES` reader - Resolution"]
pub type RES_R = crate::FieldReader<RESSELECT_A>;
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESSELECT_A {
    #[doc = "0: 12-bit resolution, AFE sample rate is maximum (no averaging)."]
    NO_AVERAGE = 0,
    #[doc = "2: 13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    OSR4 = 2,
    #[doc = "3: 14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    OSR16 = 3,
    #[doc = "4: 15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    OSR64 = 4,
    #[doc = "5: 16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    OSR256 = 5,
}
impl From<RESSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RESSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESSELECT_A {
    type Ux = u8;
}
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESSELECT_A> {
        match self.bits {
            0 => Some(RESSELECT_A::NO_AVERAGE),
            2 => Some(RESSELECT_A::OSR4),
            3 => Some(RESSELECT_A::OSR16),
            4 => Some(RESSELECT_A::OSR64),
            5 => Some(RESSELECT_A::OSR256),
            _ => None,
        }
    }
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        *self == RESSELECT_A::NO_AVERAGE
    }
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    #[inline(always)]
    pub fn is_osr4(&self) -> bool {
        *self == RESSELECT_A::OSR4
    }
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    #[inline(always)]
    pub fn is_osr16(&self) -> bool {
        *self == RESSELECT_A::OSR16
    }
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        *self == RESSELECT_A::OSR64
    }
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        *self == RESSELECT_A::OSR256
    }
}
#[doc = "Field `RES` writer - Resolution"]
pub type RES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, RESSELECT_A>;
impl<'a, REG, const O: u8> RES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELECT_A::NO_AVERAGE)
    }
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    #[inline(always)]
    pub fn osr4(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELECT_A::OSR4)
    }
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    #[inline(always)]
    pub fn osr16(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELECT_A::OSR16)
    }
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELECT_A::OSR64)
    }
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELECT_A::OSR256)
    }
}
#[doc = "Field `TAG` reader - TAG of the AFEC_LDCR"]
pub type TAG_R = crate::BitReader;
#[doc = "Field `TAG` writer - TAG of the AFEC_LDCR"]
pub type TAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STM` reader - Single Trigger Mode"]
pub type STM_R = crate::BitReader;
#[doc = "Field `STM` writer - Single Trigger Mode"]
pub type STM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SIGNMODE` reader - Sign Mode"]
pub type SIGNMODE_R = crate::FieldReader<SIGNMODESELECT_A>;
#[doc = "Sign Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIGNMODESELECT_A {
    #[doc = "0: Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    SE_UNSG_DF_SIGN = 0,
    #[doc = "1: Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    SE_SIGN_DF_UNSG = 1,
    #[doc = "2: All channels: Unsigned conversions."]
    ALL_UNSIGNED = 2,
    #[doc = "3: All channels: Signed conversions."]
    ALL_SIGNED = 3,
}
impl From<SIGNMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SIGNMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SIGNMODESELECT_A {
    type Ux = u8;
}
impl SIGNMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGNMODESELECT_A {
        match self.bits {
            0 => SIGNMODESELECT_A::SE_UNSG_DF_SIGN,
            1 => SIGNMODESELECT_A::SE_SIGN_DF_UNSG,
            2 => SIGNMODESELECT_A::ALL_UNSIGNED,
            3 => SIGNMODESELECT_A::ALL_SIGNED,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    #[inline(always)]
    pub fn is_se_unsg_df_sign(&self) -> bool {
        *self == SIGNMODESELECT_A::SE_UNSG_DF_SIGN
    }
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    #[inline(always)]
    pub fn is_se_sign_df_unsg(&self) -> bool {
        *self == SIGNMODESELECT_A::SE_SIGN_DF_UNSG
    }
    #[doc = "All channels: Unsigned conversions."]
    #[inline(always)]
    pub fn is_all_unsigned(&self) -> bool {
        *self == SIGNMODESELECT_A::ALL_UNSIGNED
    }
    #[doc = "All channels: Signed conversions."]
    #[inline(always)]
    pub fn is_all_signed(&self) -> bool {
        *self == SIGNMODESELECT_A::ALL_SIGNED
    }
}
#[doc = "Field `SIGNMODE` writer - Sign Mode"]
pub type SIGNMODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SIGNMODESELECT_A>;
impl<'a, REG, const O: u8> SIGNMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    #[inline(always)]
    pub fn se_unsg_df_sign(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNMODESELECT_A::SE_UNSG_DF_SIGN)
    }
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    #[inline(always)]
    pub fn se_sign_df_unsg(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNMODESELECT_A::SE_SIGN_DF_UNSG)
    }
    #[doc = "All channels: Unsigned conversions."]
    #[inline(always)]
    pub fn all_unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNMODESELECT_A::ALL_UNSIGNED)
    }
    #[doc = "All channels: Signed conversions."]
    #[inline(always)]
    pub fn all_signed(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNMODESELECT_A::ALL_SIGNED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CMPALL_R {
        CMPALL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CMPFILTER_R {
        CMPFILTER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&self) -> SIGNMODE_R {
        SIGNMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmode(&mut self) -> CMPMODE_W<EMR_SPEC, 0> {
        CMPMODE_W::new(self)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel(&mut self) -> CMPSEL_W<EMR_SPEC, 3> {
        CMPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    #[must_use]
    pub fn cmpall(&mut self) -> CMPALL_W<EMR_SPEC, 9> {
        CMPALL_W::new(self)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn cmpfilter(&mut self) -> CMPFILTER_W<EMR_SPEC, 12> {
        CMPFILTER_W::new(self)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<EMR_SPEC, 16> {
        RES_W::new(self)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TAG_W<EMR_SPEC, 24> {
        TAG_W::new(self)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> STM_W<EMR_SPEC, 25> {
        STM_W::new(self)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    #[must_use]
    pub fn signmode(&mut self) -> SIGNMODE_W<EMR_SPEC, 28> {
        SIGNMODE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AFEC Extended Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
