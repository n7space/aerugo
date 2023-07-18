#[doc = "Register `CIE` writer"]
pub struct W(crate::W<CIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIE_SPEC>;
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
impl From<crate::W<CIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIE` writer - End of Block Interrupt Enable Bit"]
pub type BIE_W<'a, const O: u8> = crate::BitWriter<'a, CIE_SPEC, O>;
#[doc = "Field `LIE` writer - End of Linked List Interrupt Enable Bit"]
pub type LIE_W<'a, const O: u8> = crate::BitWriter<'a, CIE_SPEC, O>;
#[doc = "Field `DIE` writer - End of Disable Interrupt Enable Bit"]
pub type DIE_W<'a, const O: u8> = crate::BitWriter<'a, CIE_SPEC, O>;
#[doc = "Field `FIE` writer - End of Flush Interrupt Enable Bit"]
pub type FIE_W<'a, const O: u8> = crate::BitWriter<'a, CIE_SPEC, O>;
#[doc = "Field `RBIE` writer - Read Bus Error Interrupt Enable Bit"]
pub type RBIE_W<'a, const O: u8> = crate::BitWriter<'a, CIE_SPEC, O>;
#[doc = "Field `WBIE` writer - Write Bus Error Interrupt Enable Bit"]
pub type WBIE_W<'a, const O: u8> = crate::BitWriter<'a, CIE_SPEC, O>;
#[doc = "Field `ROIE` writer - Request Overflow Error Interrupt Enable Bit"]
pub type ROIE_W<'a, const O: u8> = crate::BitWriter<'a, CIE_SPEC, O>;
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<0> {
        BIE_W::new(self)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lie(&mut self) -> LIE_W<1> {
        LIE_W::new(self)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn die(&mut self) -> DIE_W<2> {
        DIE_W::new(self)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn fie(&mut self) -> FIE_W<3> {
        FIE_W::new(self)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rbie(&mut self) -> RBIE_W<4> {
        RBIE_W::new(self)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn wbie(&mut self) -> WBIE_W<5> {
        WBIE_W::new(self)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> ROIE_W<6> {
        ROIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie](index.html) module"]
pub struct CIE_SPEC;
impl crate::RegisterSpec for CIE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cie::W](W) writer structure"]
impl crate::Writable for CIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIE to value 0"]
impl crate::Resettable for CIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
