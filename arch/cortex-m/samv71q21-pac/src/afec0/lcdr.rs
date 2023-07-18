#[doc = "Register `LCDR` reader"]
pub struct R(crate::R<LCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LDATA` reader - Last Data Converted"]
pub type LDATA_R = crate::FieldReader<u16>;
#[doc = "Field `CHNB` reader - Channel Number"]
pub type CHNB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Last Data Converted"]
    #[inline(always)]
    pub fn ldata(&self) -> LDATA_R {
        LDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - Channel Number"]
    #[inline(always)]
    pub fn chnb(&self) -> CHNB_R {
        CHNB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "AFEC Last Converted Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdr](index.html) module"]
pub struct LCDR_SPEC;
impl crate::RegisterSpec for LCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdr::R](R) reader structure"]
impl crate::Readable for LCDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCDR to value 0"]
impl crate::Resettable for LCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
