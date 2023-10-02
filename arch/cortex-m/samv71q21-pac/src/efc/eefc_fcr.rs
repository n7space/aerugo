#[doc = "Register `EEFC_FCR` writer"]
pub type W = crate::W<EEFC_FCR_SPEC>;
#[doc = "Flash Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCMDSELECT_AW {
    #[doc = "0: Get Flash descriptor"]
    GETD = 0,
    #[doc = "1: Write page"]
    WP = 1,
    #[doc = "2: Write page and lock"]
    WPL = 2,
    #[doc = "3: Erase page and write page"]
    EWP = 3,
    #[doc = "4: Erase page and write page then lock"]
    EWPL = 4,
    #[doc = "5: Erase all"]
    EA = 5,
    #[doc = "7: Erase pages"]
    EPA = 7,
    #[doc = "8: Set lock bit"]
    SLB = 8,
    #[doc = "9: Clear lock bit"]
    CLB = 9,
    #[doc = "10: Get lock bit"]
    GLB = 10,
    #[doc = "11: Set GPNVM bit"]
    SGPB = 11,
    #[doc = "12: Clear GPNVM bit"]
    CGPB = 12,
    #[doc = "13: Get GPNVM bit"]
    GGPB = 13,
    #[doc = "14: Start read unique identifier"]
    STUI = 14,
    #[doc = "15: Stop read unique identifier"]
    SPUI = 15,
    #[doc = "16: Get CALIB bit"]
    GCALB = 16,
    #[doc = "17: Erase sector"]
    ES = 17,
    #[doc = "18: Write user signature"]
    WUS = 18,
    #[doc = "19: Erase user signature"]
    EUS = 19,
    #[doc = "20: Start read user signature"]
    STUS = 20,
    #[doc = "21: Stop read user signature"]
    SPUS = 21,
}
impl From<FCMDSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: FCMDSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FCMDSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `FCMD` writer - Flash Command"]
pub type FCMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, FCMDSELECT_AW>;
impl<'a, REG, const O: u8> FCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Get Flash descriptor"]
    #[inline(always)]
    pub fn getd(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::GETD)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::WP)
    }
    #[doc = "Write page and lock"]
    #[inline(always)]
    pub fn wpl(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::WPL)
    }
    #[doc = "Erase page and write page"]
    #[inline(always)]
    pub fn ewp(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::EWP)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline(always)]
    pub fn ewpl(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::EWPL)
    }
    #[doc = "Erase all"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::EA)
    }
    #[doc = "Erase pages"]
    #[inline(always)]
    pub fn epa(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::EPA)
    }
    #[doc = "Set lock bit"]
    #[inline(always)]
    pub fn slb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::SLB)
    }
    #[doc = "Clear lock bit"]
    #[inline(always)]
    pub fn clb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::CLB)
    }
    #[doc = "Get lock bit"]
    #[inline(always)]
    pub fn glb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::GLB)
    }
    #[doc = "Set GPNVM bit"]
    #[inline(always)]
    pub fn sgpb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::SGPB)
    }
    #[doc = "Clear GPNVM bit"]
    #[inline(always)]
    pub fn cgpb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::CGPB)
    }
    #[doc = "Get GPNVM bit"]
    #[inline(always)]
    pub fn ggpb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::GGPB)
    }
    #[doc = "Start read unique identifier"]
    #[inline(always)]
    pub fn stui(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::STUI)
    }
    #[doc = "Stop read unique identifier"]
    #[inline(always)]
    pub fn spui(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::SPUI)
    }
    #[doc = "Get CALIB bit"]
    #[inline(always)]
    pub fn gcalb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::GCALB)
    }
    #[doc = "Erase sector"]
    #[inline(always)]
    pub fn es(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::ES)
    }
    #[doc = "Write user signature"]
    #[inline(always)]
    pub fn wus(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::WUS)
    }
    #[doc = "Erase user signature"]
    #[inline(always)]
    pub fn eus(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::EUS)
    }
    #[doc = "Start read user signature"]
    #[inline(always)]
    pub fn stus(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::STUS)
    }
    #[doc = "Stop read user signature"]
    #[inline(always)]
    pub fn spus(self) -> &'a mut crate::W<REG> {
        self.variant(FCMDSELECT_AW::SPUS)
    }
}
#[doc = "Field `FARG` writer - Flash Command Argument"]
pub type FARG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Flash Writing Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FKEYSELECT_AW {
    #[doc = "90: The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    PASSWD = 90,
}
impl From<FKEYSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: FKEYSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FKEYSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `FKEY` writer - Flash Writing Protection Key"]
pub type FKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, FKEYSELECT_AW>;
impl<'a, REG, const O: u8> FKEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(FKEYSELECT_AW::PASSWD)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline(always)]
    #[must_use]
    pub fn fcmd(&mut self) -> FCMD_W<EEFC_FCR_SPEC, 0> {
        FCMD_W::new(self)
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline(always)]
    #[must_use]
    pub fn farg(&mut self) -> FARG_W<EEFC_FCR_SPEC, 8> {
        FARG_W::new(self)
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline(always)]
    #[must_use]
    pub fn fkey(&mut self) -> FKEY_W<EEFC_FCR_SPEC, 24> {
        FKEY_W::new(self)
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
#[doc = "EEFC Flash Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefc_fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEFC_FCR_SPEC;
impl crate::RegisterSpec for EEFC_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eefc_fcr::W`](W) writer structure"]
impl crate::Writable for EEFC_FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEFC_FCR to value 0"]
impl crate::Resettable for EEFC_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
