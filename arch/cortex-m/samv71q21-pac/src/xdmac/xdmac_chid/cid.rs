#[doc = "Register `CID` writer"]
pub type W = crate::W<CID_SPEC>;
#[doc = "Field `BID` writer - End of Block Interrupt Disable Bit"]
pub type BID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LID` writer - End of Linked List Interrupt Disable Bit"]
pub type LID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DID` writer - End of Disable Interrupt Disable Bit"]
pub type DID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FID` writer - End of Flush Interrupt Disable Bit"]
pub type FID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBEID` writer - Read Bus Error Interrupt Disable Bit"]
pub type RBEID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WBEID` writer - Write Bus Error Interrupt Disable Bit"]
pub type WBEID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROID` writer - Request Overflow Error Interrupt Disable Bit"]
pub type ROID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn bid(&mut self) -> BID_W<CID_SPEC, 0> {
        BID_W::new(self)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lid(&mut self) -> LID_W<CID_SPEC, 1> {
        LID_W::new(self)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn did(&mut self) -> DID_W<CID_SPEC, 2> {
        DID_W::new(self)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn fid(&mut self) -> FID_W<CID_SPEC, 3> {
        FID_W::new(self)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rbeid(&mut self) -> RBEID_W<CID_SPEC, 4> {
        RBEID_W::new(self)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn wbeid(&mut self) -> WBEID_W<CID_SPEC, 5> {
        WBEID_W::new(self)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn roid(&mut self) -> ROID_W<CID_SPEC, 6> {
        ROID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID_SPEC;
impl crate::RegisterSpec for CID_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cid::W`](W) writer structure"]
impl crate::Writable for CID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CID to value 0"]
impl crate::Resettable for CID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
