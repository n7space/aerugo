#[doc = "Register `TDR` writer"]
pub type W = crate::W<TDR_SPEC>;
#[doc = "Field `TD` writer - Transmit Data"]
pub type TD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Peripheral Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCSSELECT_AW {
    #[doc = "14: NPCS0 as Chip Select"]
    NPCS0 = 14,
    #[doc = "13: NPCS1 as Chip Select"]
    NPCS1 = 13,
    #[doc = "11: NPCS2 as Chip Select"]
    NPCS2 = 11,
    #[doc = "7: NPCS3 as Chip Select"]
    NPCS3 = 7,
}
impl From<PCSSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: PCSSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCSSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PCSSELECT_AW>;
impl<'a, REG, const O: u8> PCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPCS0 as Chip Select"]
    #[inline(always)]
    pub fn npcs0(self) -> &'a mut crate::W<REG> {
        self.variant(PCSSELECT_AW::NPCS0)
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline(always)]
    pub fn npcs1(self) -> &'a mut crate::W<REG> {
        self.variant(PCSSELECT_AW::NPCS1)
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline(always)]
    pub fn npcs2(self) -> &'a mut crate::W<REG> {
        self.variant(PCSSELECT_AW::NPCS2)
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline(always)]
    pub fn npcs3(self) -> &'a mut crate::W<REG> {
        self.variant(PCSSELECT_AW::NPCS3)
    }
}
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LASTXFER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TD_W<TDR_SPEC, 0> {
        TD_W::new(self)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PCS_W<TDR_SPEC, 16> {
        PCS_W::new(self)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn lastxfer(&mut self) -> LASTXFER_W<TDR_SPEC, 24> {
        LASTXFER_W::new(self)
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
#[doc = "Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
