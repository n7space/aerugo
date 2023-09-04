#[doc = "Register `SDCR` reader"]
pub type R = crate::R<SDCR_SPEC>;
#[doc = "Register `SDCR` writer"]
pub type W = crate::W<SDCR_SPEC>;
#[doc = "Field `SDCSEL` reader - SDCard/SDIO Slot"]
pub type SDCSEL_R = crate::FieldReader<SDCSELSELECT_A>;
#[doc = "SDCard/SDIO Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDCSELSELECT_A {
    #[doc = "0: Slot A is selected."]
    SLOTA = 0,
}
impl From<SDCSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDCSELSELECT_A {
    type Ux = u8;
}
impl SDCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDCSELSELECT_A> {
        match self.bits {
            0 => Some(SDCSELSELECT_A::SLOTA),
            _ => None,
        }
    }
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn is_slota(&self) -> bool {
        *self == SDCSELSELECT_A::SLOTA
    }
}
#[doc = "Field `SDCSEL` writer - SDCard/SDIO Slot"]
pub type SDCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SDCSELSELECT_A>;
impl<'a, REG, const O: u8> SDCSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn slota(self) -> &'a mut crate::W<REG> {
        self.variant(SDCSELSELECT_A::SLOTA)
    }
}
#[doc = "Field `SDCBUS` reader - SDCard/SDIO Bus Width"]
pub type SDCBUS_R = crate::FieldReader<SDCBUSSELECT_A>;
#[doc = "SDCard/SDIO Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDCBUSSELECT_A {
    #[doc = "0: 1 bit"]
    _1 = 0,
    #[doc = "2: 4 bits"]
    _4 = 2,
    #[doc = "3: 8 bits"]
    _8 = 3,
}
impl From<SDCBUSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCBUSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDCBUSSELECT_A {
    type Ux = u8;
}
impl SDCBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDCBUSSELECT_A> {
        match self.bits {
            0 => Some(SDCBUSSELECT_A::_1),
            2 => Some(SDCBUSSELECT_A::_4),
            3 => Some(SDCBUSSELECT_A::_8),
            _ => None,
        }
    }
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCBUSSELECT_A::_1
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SDCBUSSELECT_A::_4
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SDCBUSSELECT_A::_8
    }
}
#[doc = "Field `SDCBUS` writer - SDCard/SDIO Bus Width"]
pub type SDCBUS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SDCBUSSELECT_A>;
impl<'a, REG, const O: u8> SDCBUS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCBUSSELECT_A::_1)
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(SDCBUSSELECT_A::_4)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(SDCBUSSELECT_A::_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&self) -> SDCSEL_R {
        SDCSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&self) -> SDCBUS_R {
        SDCBUS_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    #[must_use]
    pub fn sdcsel(&mut self) -> SDCSEL_W<SDCR_SPEC, 0> {
        SDCSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn sdcbus(&mut self) -> SDCBUS_W<SDCR_SPEC, 6> {
        SDCBUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SD/SDIO Card Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDCR_SPEC;
impl crate::RegisterSpec for SDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcr::R`](R) reader structure"]
impl crate::Readable for SDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdcr::W`](W) writer structure"]
impl crate::Writable for SDCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDCR to value 0"]
impl crate::Resettable for SDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
