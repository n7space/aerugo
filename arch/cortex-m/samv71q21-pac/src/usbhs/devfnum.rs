#[doc = "Register `DEVFNUM` reader"]
pub type R = crate::R<DEVFNUM_SPEC>;
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub type MFNUM_R = crate::FieldReader;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FNCERR` reader - Frame Number CRC Error"]
pub type FNCERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Frame Number CRC Error"]
    #[inline(always)]
    pub fn fncerr(&self) -> FNCERR_R {
        FNCERR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Device Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devfnum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVFNUM_SPEC;
impl crate::RegisterSpec for DEVFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devfnum::R`](R) reader structure"]
impl crate::Readable for DEVFNUM_SPEC {}
#[doc = "`reset()` method sets DEVFNUM to value 0"]
impl crate::Resettable for DEVFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
