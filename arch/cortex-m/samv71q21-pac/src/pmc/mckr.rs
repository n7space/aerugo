#[doc = "Register `MCKR` reader"]
pub struct R(crate::R<MCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCKR` writer"]
pub struct W(crate::W<MCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCKR_SPEC>;
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
impl From<crate::W<MCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub type CSS_R = crate::FieldReader<CSSSELECT_A>;
#[doc = "Master Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSSSELECT_A {
    #[doc = "0: Slow Clock is selected"]
    SLOW_CLK = 0,
    #[doc = "1: Main Clock is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLA Clock is selected"]
    PLLA_CLK = 2,
    #[doc = "3: Divided UPLL Clock is selected"]
    UPLL_CLK = 3,
}
impl From<CSSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSSSELECT_A {
    type Ux = u8;
}
impl CSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSSELECT_A {
        match self.bits {
            0 => CSSSELECT_A::SLOW_CLK,
            1 => CSSSELECT_A::MAIN_CLK,
            2 => CSSSELECT_A::PLLA_CLK,
            3 => CSSSELECT_A::UPLL_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSSSELECT_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSSSELECT_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSSSELECT_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSSSELECT_A::UPLL_CLK
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub type CSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MCKR_SPEC, 2, O, CSSSELECT_A>;
impl<'a, const O: u8> CSS_W<'a, O> {
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSSSELECT_A::SLOW_CLK)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSSSELECT_A::MAIN_CLK)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSSSELECT_A::PLLA_CLK)
    }
    #[doc = "Divided UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSSSELECT_A::UPLL_CLK)
    }
}
#[doc = "Field `PRES` reader - Processor Clock Prescaler"]
pub type PRES_R = crate::FieldReader<PRESSELECT_A>;
#[doc = "Processor Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESSELECT_A {
    #[doc = "0: Selected clock"]
    CLK_1 = 0,
    #[doc = "1: Selected clock divided by 2"]
    CLK_2 = 1,
    #[doc = "2: Selected clock divided by 4"]
    CLK_4 = 2,
    #[doc = "3: Selected clock divided by 8"]
    CLK_8 = 3,
    #[doc = "4: Selected clock divided by 16"]
    CLK_16 = 4,
    #[doc = "5: Selected clock divided by 32"]
    CLK_32 = 5,
    #[doc = "6: Selected clock divided by 64"]
    CLK_64 = 6,
    #[doc = "7: Selected clock divided by 3"]
    CLK_3 = 7,
}
impl From<PRESSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESSELECT_A {
    type Ux = u8;
}
impl PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESSELECT_A {
        match self.bits {
            0 => PRESSELECT_A::CLK_1,
            1 => PRESSELECT_A::CLK_2,
            2 => PRESSELECT_A::CLK_4,
            3 => PRESSELECT_A::CLK_8,
            4 => PRESSELECT_A::CLK_16,
            5 => PRESSELECT_A::CLK_32,
            6 => PRESSELECT_A::CLK_64,
            7 => PRESSELECT_A::CLK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_1`"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == PRESSELECT_A::CLK_1
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == PRESSELECT_A::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_4`"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        *self == PRESSELECT_A::CLK_4
    }
    #[doc = "Checks if the value of the field is `CLK_8`"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        *self == PRESSELECT_A::CLK_8
    }
    #[doc = "Checks if the value of the field is `CLK_16`"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        *self == PRESSELECT_A::CLK_16
    }
    #[doc = "Checks if the value of the field is `CLK_32`"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        *self == PRESSELECT_A::CLK_32
    }
    #[doc = "Checks if the value of the field is `CLK_64`"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        *self == PRESSELECT_A::CLK_64
    }
    #[doc = "Checks if the value of the field is `CLK_3`"]
    #[inline(always)]
    pub fn is_clk_3(&self) -> bool {
        *self == PRESSELECT_A::CLK_3
    }
}
#[doc = "Field `PRES` writer - Processor Clock Prescaler"]
pub type PRES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MCKR_SPEC, 3, O, PRESSELECT_A>;
impl<'a, const O: u8> PRES_W<'a, O> {
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(PRESSELECT_A::CLK_1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(PRESSELECT_A::CLK_2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(PRESSELECT_A::CLK_4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(PRESSELECT_A::CLK_8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(PRESSELECT_A::CLK_16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(PRESSELECT_A::CLK_32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(PRESSELECT_A::CLK_64)
    }
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn clk_3(self) -> &'a mut W {
        self.variant(PRESSELECT_A::CLK_3)
    }
}
#[doc = "Field `MDIV` reader - Master Clock Division"]
pub type MDIV_R = crate::FieldReader<MDIVSELECT_A>;
#[doc = "Master Clock Division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDIVSELECT_A {
    #[doc = "0: Master Clock is Prescaler Output Clock divided by 1."]
    EQ_PCK = 0,
    #[doc = "1: Master Clock is Prescaler Output Clock divided by 2."]
    PCK_DIV2 = 1,
    #[doc = "2: Master Clock is Prescaler Output Clock divided by 4."]
    PCK_DIV4 = 2,
    #[doc = "3: Master Clock is Prescaler Output Clock divided by 3."]
    PCK_DIV3 = 3,
}
impl From<MDIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIVSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDIVSELECT_A {
    type Ux = u8;
}
impl MDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIVSELECT_A {
        match self.bits {
            0 => MDIVSELECT_A::EQ_PCK,
            1 => MDIVSELECT_A::PCK_DIV2,
            2 => MDIVSELECT_A::PCK_DIV4,
            3 => MDIVSELECT_A::PCK_DIV3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EQ_PCK`"]
    #[inline(always)]
    pub fn is_eq_pck(&self) -> bool {
        *self == MDIVSELECT_A::EQ_PCK
    }
    #[doc = "Checks if the value of the field is `PCK_DIV2`"]
    #[inline(always)]
    pub fn is_pck_div2(&self) -> bool {
        *self == MDIVSELECT_A::PCK_DIV2
    }
    #[doc = "Checks if the value of the field is `PCK_DIV4`"]
    #[inline(always)]
    pub fn is_pck_div4(&self) -> bool {
        *self == MDIVSELECT_A::PCK_DIV4
    }
    #[doc = "Checks if the value of the field is `PCK_DIV3`"]
    #[inline(always)]
    pub fn is_pck_div3(&self) -> bool {
        *self == MDIVSELECT_A::PCK_DIV3
    }
}
#[doc = "Field `MDIV` writer - Master Clock Division"]
pub type MDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MCKR_SPEC, 2, O, MDIVSELECT_A>;
impl<'a, const O: u8> MDIV_W<'a, O> {
    #[doc = "Master Clock is Prescaler Output Clock divided by 1."]
    #[inline(always)]
    pub fn eq_pck(self) -> &'a mut W {
        self.variant(MDIVSELECT_A::EQ_PCK)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 2."]
    #[inline(always)]
    pub fn pck_div2(self) -> &'a mut W {
        self.variant(MDIVSELECT_A::PCK_DIV2)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 4."]
    #[inline(always)]
    pub fn pck_div4(self) -> &'a mut W {
        self.variant(MDIVSELECT_A::PCK_DIV4)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 3."]
    #[inline(always)]
    pub fn pck_div3(self) -> &'a mut W {
        self.variant(MDIVSELECT_A::PCK_DIV3)
    }
}
#[doc = "Field `UPLLDIV2` reader - UPLL Divider by 2"]
pub type UPLLDIV2_R = crate::BitReader;
#[doc = "Field `UPLLDIV2` writer - UPLL Divider by 2"]
pub type UPLLDIV2_W<'a, const O: u8> = crate::BitWriter<'a, MCKR_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 13 - UPLL Divider by 2"]
    #[inline(always)]
    pub fn uplldiv2(&self) -> UPLLDIV2_R {
        UPLLDIV2_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<0> {
        CSS_W::new(self)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<4> {
        PRES_W::new(self)
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv(&mut self) -> MDIV_W<8> {
        MDIV_W::new(self)
    }
    #[doc = "Bit 13 - UPLL Divider by 2"]
    #[inline(always)]
    #[must_use]
    pub fn uplldiv2(&mut self) -> UPLLDIV2_W<13> {
        UPLLDIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mckr](index.html) module"]
pub struct MCKR_SPEC;
impl crate::RegisterSpec for MCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mckr::R](R) reader structure"]
impl crate::Readable for MCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mckr::W](W) writer structure"]
impl crate::Writable for MCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCKR to value 0"]
impl crate::Resettable for MCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
