#[doc = "Register `MSD` reader"]
pub type R = crate::R<MSD_SPEC>;
#[doc = "Field `SD0` reader - System Data (Byte 0)"]
pub type SD0_R = crate::FieldReader;
#[doc = "Field `SD1` reader - System Data (Byte 1)"]
pub type SD1_R = crate::FieldReader;
#[doc = "Field `SD2` reader - System Data (Byte 2)"]
pub type SD2_R = crate::FieldReader;
#[doc = "Field `SD3` reader - System Data (Byte 3)"]
pub type SD3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - System Data (Byte 0)"]
    #[inline(always)]
    pub fn sd0(&self) -> SD0_R {
        SD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - System Data (Byte 1)"]
    #[inline(always)]
    pub fn sd1(&self) -> SD1_R {
        SD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - System Data (Byte 2)"]
    #[inline(always)]
    pub fn sd2(&self) -> SD2_R {
        SD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - System Data (Byte 3)"]
    #[inline(always)]
    pub fn sd3(&self) -> SD3_R {
        SD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "MediaLB System Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSD_SPEC;
impl crate::RegisterSpec for MSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msd::R`](R) reader structure"]
impl crate::Readable for MSD_SPEC {}
#[doc = "`reset()` method sets MSD to value 0"]
impl crate::Resettable for MSD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
