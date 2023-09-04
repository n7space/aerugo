#[doc = "Register `CIE` writer"]
pub type W = crate::W<CIE_SPEC>;
#[doc = "Field `BIE` writer - End of Block Interrupt Enable Bit"]
pub type BIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LIE` writer - End of Linked List Interrupt Enable Bit"]
pub type LIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIE` writer - End of Disable Interrupt Enable Bit"]
pub type DIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIE` writer - End of Flush Interrupt Enable Bit"]
pub type FIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBIE` writer - Read Bus Error Interrupt Enable Bit"]
pub type RBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WBIE` writer - Write Bus Error Interrupt Enable Bit"]
pub type WBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROIE` writer - Request Overflow Error Interrupt Enable Bit"]
pub type ROIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<CIE_SPEC, 0> {
        BIE_W::new(self)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lie(&mut self) -> LIE_W<CIE_SPEC, 1> {
        LIE_W::new(self)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn die(&mut self) -> DIE_W<CIE_SPEC, 2> {
        DIE_W::new(self)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn fie(&mut self) -> FIE_W<CIE_SPEC, 3> {
        FIE_W::new(self)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rbie(&mut self) -> RBIE_W<CIE_SPEC, 4> {
        RBIE_W::new(self)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn wbie(&mut self) -> WBIE_W<CIE_SPEC, 5> {
        WBIE_W::new(self)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> ROIE_W<CIE_SPEC, 6> {
        ROIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cie::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIE_SPEC;
impl crate::RegisterSpec for CIE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cie::W`](W) writer structure"]
impl crate::Writable for CIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIE to value 0"]
impl crate::Resettable for CIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
