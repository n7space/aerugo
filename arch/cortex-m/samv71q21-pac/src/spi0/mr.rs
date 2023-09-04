#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub type MSTR_R = crate::BitReader<MSTRSELECT_A>;
#[doc = "Master/Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTRSELECT_A {
    #[doc = "1: Master"]
    MASTER = 1,
    #[doc = "0: Slave"]
    SLAVE = 0,
}
impl From<MSTRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MSTRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTRSELECT_A {
        match self.bits {
            true => MSTRSELECT_A::MASTER,
            false => MSTRSELECT_A::SLAVE,
        }
    }
    #[doc = "Master"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTRSELECT_A::MASTER
    }
    #[doc = "Slave"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTRSELECT_A::SLAVE
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub type MSTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTRSELECT_A>;
impl<'a, REG, const O: u8> MSTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MSTRSELECT_A::MASTER)
    }
    #[doc = "Slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MSTRSELECT_A::SLAVE)
    }
}
#[doc = "Field `PS` reader - Peripheral Select"]
pub type PS_R = crate::BitReader;
#[doc = "Field `PS` writer - Peripheral Select"]
pub type PS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub type PCSDEC_R = crate::BitReader;
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub type PCSDEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub type MODFDIS_R = crate::BitReader;
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub type MODFDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WDRBT_R = crate::BitReader;
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WDRBT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LLB_R = crate::BitReader;
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LLB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PCS_R = crate::FieldReader<PCSSELECT_A>;
#[doc = "Peripheral Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCSSELECT_A {
    #[doc = "14: NPCS0 as Chip Select"]
    NPCS0 = 14,
    #[doc = "13: NPCS1 as Chip Select"]
    NPCS1 = 13,
    #[doc = "11: NPCS2 as Chip Select"]
    NPCS2 = 11,
    #[doc = "7: NPCS3 as Chip Select"]
    NPCS3 = 7,
}
impl From<PCSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PCSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCSSELECT_A {
    type Ux = u8;
}
impl PCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PCSSELECT_A> {
        match self.bits {
            14 => Some(PCSSELECT_A::NPCS0),
            13 => Some(PCSSELECT_A::NPCS1),
            11 => Some(PCSSELECT_A::NPCS2),
            7 => Some(PCSSELECT_A::NPCS3),
            _ => None,
        }
    }
    #[doc = "NPCS0 as Chip Select"]
    #[inline(always)]
    pub fn is_npcs0(&self) -> bool {
        *self == PCSSELECT_A::NPCS0
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline(always)]
    pub fn is_npcs1(&self) -> bool {
        *self == PCSSELECT_A::NPCS1
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline(always)]
    pub fn is_npcs2(&self) -> bool {
        *self == PCSSELECT_A::NPCS2
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline(always)]
    pub fn is_npcs3(&self) -> bool {
        *self == PCSSELECT_A::NPCS3
    }
}
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PCSSELECT_A>;
impl<'a, REG, const O: u8> PCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPCS0 as Chip Select"]
    #[inline(always)]
    pub fn npcs0(self) -> &'a mut crate::W<REG> {
        self.variant(PCSSELECT_A::NPCS0)
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline(always)]
    pub fn npcs1(self) -> &'a mut crate::W<REG> {
        self.variant(PCSSELECT_A::NPCS1)
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline(always)]
    pub fn npcs2(self) -> &'a mut crate::W<REG> {
        self.variant(PCSSELECT_A::NPCS2)
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline(always)]
    pub fn npcs3(self) -> &'a mut crate::W<REG> {
        self.variant(PCSSELECT_A::NPCS3)
    }
}
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub type DLYBCS_R = crate::FieldReader;
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub type DLYBCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PCSDEC_R {
        PCSDEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> MODFDIS_R {
        MODFDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&self) -> DLYBCS_R {
        DLYBCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<MR_SPEC, 0> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<MR_SPEC, 1> {
        PS_W::new(self)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    #[must_use]
    pub fn pcsdec(&mut self) -> PCSDEC_W<MR_SPEC, 2> {
        PCSDEC_W::new(self)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    #[must_use]
    pub fn modfdis(&mut self) -> MODFDIS_W<MR_SPEC, 4> {
        MODFDIS_W::new(self)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wdrbt(&mut self) -> WDRBT_W<MR_SPEC, 5> {
        WDRBT_W::new(self)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn llb(&mut self) -> LLB_W<MR_SPEC, 7> {
        LLB_W::new(self)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PCS_W<MR_SPEC, 16> {
        PCS_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    #[must_use]
    pub fn dlybcs(&mut self) -> DLYBCS_W<MR_SPEC, 24> {
        DLYBCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
