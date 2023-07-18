#[doc = "Register `CIS` reader"]
pub struct R(crate::R<CIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIS` reader - End of Block Interrupt Status Bit"]
pub type BIS_R = crate::BitReader;
#[doc = "Field `LIS` reader - End of Linked List Interrupt Status Bit"]
pub type LIS_R = crate::BitReader;
#[doc = "Field `DIS` reader - End of Disable Interrupt Status Bit"]
pub type DIS_R = crate::BitReader;
#[doc = "Field `FIS` reader - End of Flush Interrupt Status Bit"]
pub type FIS_R = crate::BitReader;
#[doc = "Field `RBEIS` reader - Read Bus Error Interrupt Status Bit"]
pub type RBEIS_R = crate::BitReader;
#[doc = "Field `WBEIS` reader - Write Bus Error Interrupt Status Bit"]
pub type WBEIS_R = crate::BitReader;
#[doc = "Field `ROIS` reader - Request Overflow Error Interrupt Status Bit"]
pub type ROIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Block Interrupt Status Bit"]
    #[inline(always)]
    pub fn bis(&self) -> BIS_R {
        BIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Status Bit"]
    #[inline(always)]
    pub fn lis(&self) -> LIS_R {
        LIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Status Bit"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Status Bit"]
    #[inline(always)]
    pub fn fis(&self) -> FIS_R {
        FIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rbeis(&self) -> RBEIS_R {
        RBEIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn wbeis(&self) -> WBEIS_R {
        WBEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rois(&self) -> ROIS_R {
        ROIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Channel Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis](index.html) module"]
pub struct CIS_SPEC;
impl crate::RegisterSpec for CIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cis::R](R) reader structure"]
impl crate::Readable for CIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIS to value 0"]
impl crate::Resettable for CIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
