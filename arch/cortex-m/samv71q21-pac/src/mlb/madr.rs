#[doc = "Register `MADR` reader"]
pub struct R(crate::R<MADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MADR` writer"]
pub struct W(crate::W<MADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MADR_SPEC>;
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
impl From<crate::W<MADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - CTR or DBR Address"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - CTR or DBR Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, MADR_SPEC, 14, O, u16>;
#[doc = "Field `TB` reader - Target Location Bit"]
pub type TB_R = crate::BitReader<TBSELECT_A>;
#[doc = "Target Location Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBSELECT_A {
    #[doc = "0: Selects CTR"]
    CTR = 0,
    #[doc = "1: Selects DBR"]
    DBR = 1,
}
impl From<TBSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TBSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBSELECT_A {
        match self.bits {
            false => TBSELECT_A::CTR,
            true => TBSELECT_A::DBR,
        }
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == TBSELECT_A::CTR
    }
    #[doc = "Checks if the value of the field is `DBR`"]
    #[inline(always)]
    pub fn is_dbr(&self) -> bool {
        *self == TBSELECT_A::DBR
    }
}
#[doc = "Field `TB` writer - Target Location Bit"]
pub type TB_W<'a, const O: u8> = crate::BitWriter<'a, MADR_SPEC, O, TBSELECT_A>;
impl<'a, const O: u8> TB_W<'a, O> {
    #[doc = "Selects CTR"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(TBSELECT_A::CTR)
    }
    #[doc = "Selects DBR"]
    #[inline(always)]
    pub fn dbr(self) -> &'a mut W {
        self.variant(TBSELECT_A::DBR)
    }
}
#[doc = "Field `WNR` reader - Write-Not-Read Selection"]
pub type WNR_R = crate::BitReader;
#[doc = "Field `WNR` writer - Write-Not-Read Selection"]
pub type WNR_W<'a, const O: u8> = crate::BitWriter<'a, MADR_SPEC, O>;
impl R {
    #[doc = "Bits 0:13 - CTR or DBR Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Target Location Bit"]
    #[inline(always)]
    pub fn tb(&self) -> TB_R {
        TB_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write-Not-Read Selection"]
    #[inline(always)]
    pub fn wnr(&self) -> WNR_R {
        WNR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - CTR or DBR Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 30 - Target Location Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tb(&mut self) -> TB_W<30> {
        TB_W::new(self)
    }
    #[doc = "Bit 31 - Write-Not-Read Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wnr(&mut self) -> WNR_W<31> {
        WNR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIF Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [madr](index.html) module"]
pub struct MADR_SPEC;
impl crate::RegisterSpec for MADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [madr::R](R) reader structure"]
impl crate::Readable for MADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [madr::W](W) writer structure"]
impl crate::Writable for MADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MADR to value 0"]
impl crate::Resettable for MADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
