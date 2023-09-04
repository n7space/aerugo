#[doc = "Register `MADR` reader"]
pub type R = crate::R<MADR_SPEC>;
#[doc = "Register `MADR` writer"]
pub type W = crate::W<MADR_SPEC>;
#[doc = "Field `ADDR` reader - CTR or DBR Address"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - CTR or DBR Address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
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
    #[doc = "Selects CTR"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == TBSELECT_A::CTR
    }
    #[doc = "Selects DBR"]
    #[inline(always)]
    pub fn is_dbr(&self) -> bool {
        *self == TBSELECT_A::DBR
    }
}
#[doc = "Field `TB` writer - Target Location Bit"]
pub type TB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TBSELECT_A>;
impl<'a, REG, const O: u8> TB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects CTR"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(TBSELECT_A::CTR)
    }
    #[doc = "Selects DBR"]
    #[inline(always)]
    pub fn dbr(self) -> &'a mut crate::W<REG> {
        self.variant(TBSELECT_A::DBR)
    }
}
#[doc = "Field `WNR` reader - Write-Not-Read Selection"]
pub type WNR_R = crate::BitReader;
#[doc = "Field `WNR` writer - Write-Not-Read Selection"]
pub type WNR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn addr(&mut self) -> ADDR_W<MADR_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 30 - Target Location Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tb(&mut self) -> TB_W<MADR_SPEC, 30> {
        TB_W::new(self)
    }
    #[doc = "Bit 31 - Write-Not-Read Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wnr(&mut self) -> WNR_W<MADR_SPEC, 31> {
        WNR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MIF Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`madr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`madr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MADR_SPEC;
impl crate::RegisterSpec for MADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`madr::R`](R) reader structure"]
impl crate::Readable for MADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`madr::W`](W) writer structure"]
impl crate::Writable for MADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MADR to value 0"]
impl crate::Resettable for MADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
