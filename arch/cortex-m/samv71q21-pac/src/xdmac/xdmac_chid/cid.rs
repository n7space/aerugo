#[doc = "Register `CID` writer"]
pub struct W(crate::W<CID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CID_SPEC>;
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
impl From<crate::W<CID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BID` writer - End of Block Interrupt Disable Bit"]
pub type BID_W<'a, const O: u8> = crate::BitWriter<'a, CID_SPEC, O>;
#[doc = "Field `LID` writer - End of Linked List Interrupt Disable Bit"]
pub type LID_W<'a, const O: u8> = crate::BitWriter<'a, CID_SPEC, O>;
#[doc = "Field `DID` writer - End of Disable Interrupt Disable Bit"]
pub type DID_W<'a, const O: u8> = crate::BitWriter<'a, CID_SPEC, O>;
#[doc = "Field `FID` writer - End of Flush Interrupt Disable Bit"]
pub type FID_W<'a, const O: u8> = crate::BitWriter<'a, CID_SPEC, O>;
#[doc = "Field `RBEID` writer - Read Bus Error Interrupt Disable Bit"]
pub type RBEID_W<'a, const O: u8> = crate::BitWriter<'a, CID_SPEC, O>;
#[doc = "Field `WBEID` writer - Write Bus Error Interrupt Disable Bit"]
pub type WBEID_W<'a, const O: u8> = crate::BitWriter<'a, CID_SPEC, O>;
#[doc = "Field `ROID` writer - Request Overflow Error Interrupt Disable Bit"]
pub type ROID_W<'a, const O: u8> = crate::BitWriter<'a, CID_SPEC, O>;
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn bid(&mut self) -> BID_W<0> {
        BID_W::new(self)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lid(&mut self) -> LID_W<1> {
        LID_W::new(self)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn did(&mut self) -> DID_W<2> {
        DID_W::new(self)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn fid(&mut self) -> FID_W<3> {
        FID_W::new(self)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rbeid(&mut self) -> RBEID_W<4> {
        RBEID_W::new(self)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn wbeid(&mut self) -> WBEID_W<5> {
        WBEID_W::new(self)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn roid(&mut self) -> ROID_W<6> {
        ROID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid](index.html) module"]
pub struct CID_SPEC;
impl crate::RegisterSpec for CID_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cid::W](W) writer structure"]
impl crate::Writable for CID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CID to value 0"]
impl crate::Resettable for CID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
