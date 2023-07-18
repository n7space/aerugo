#[doc = "Register `GTYPE` reader"]
pub struct R(crate::R<GTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NB_CH` reader - Number of Channels Minus One"]
pub type NB_CH_R = crate::FieldReader;
#[doc = "Field `FIFO_SZ` reader - Number of Bytes"]
pub type FIFO_SZ_R = crate::FieldReader<u16>;
#[doc = "Field `NB_REQ` reader - Number of Peripheral Requests Minus One"]
pub type NB_REQ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline(always)]
    pub fn nb_ch(&self) -> NB_CH_R {
        NB_CH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline(always)]
    pub fn fifo_sz(&self) -> FIFO_SZ_R {
        FIFO_SZ_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline(always)]
    pub fn nb_req(&self) -> NB_REQ_R {
        NB_REQ_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Global Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtype](index.html) module"]
pub struct GTYPE_SPEC;
impl crate::RegisterSpec for GTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtype::R](R) reader structure"]
impl crate::Readable for GTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GTYPE to value 0"]
impl crate::Resettable for GTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
