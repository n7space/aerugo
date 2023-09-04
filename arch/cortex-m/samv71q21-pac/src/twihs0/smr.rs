#[doc = "Register `SMR` reader"]
pub type R = crate::R<SMR_SPEC>;
#[doc = "Register `SMR` writer"]
pub type W = crate::W<SMR_SPEC>;
#[doc = "Field `NACKEN` reader - Slave Receiver Data Phase NACK enable"]
pub type NACKEN_R = crate::BitReader;
#[doc = "Field `NACKEN` writer - Slave Receiver Data Phase NACK enable"]
pub type NACKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMDA` reader - SMBus Default Address"]
pub type SMDA_R = crate::BitReader;
#[doc = "Field `SMDA` writer - SMBus Default Address"]
pub type SMDA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMHH` reader - SMBus Host Header"]
pub type SMHH_R = crate::BitReader;
#[doc = "Field `SMHH` writer - SMBus Host Header"]
pub type SMHH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCLWSDIS` reader - Clock Wait State Disable"]
pub type SCLWSDIS_R = crate::BitReader;
#[doc = "Field `SCLWSDIS` writer - Clock Wait State Disable"]
pub type SCLWSDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK` reader - Slave Address Mask"]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - Slave Address Mask"]
pub type MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SADR` reader - Slave Address"]
pub type SADR_R = crate::FieldReader;
#[doc = "Field `SADR` writer - Slave Address"]
pub type SADR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SADR1EN` reader - Slave Address 1 Enable"]
pub type SADR1EN_R = crate::BitReader;
#[doc = "Field `SADR1EN` writer - Slave Address 1 Enable"]
pub type SADR1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SADR2EN` reader - Slave Address 2 Enable"]
pub type SADR2EN_R = crate::BitReader;
#[doc = "Field `SADR2EN` writer - Slave Address 2 Enable"]
pub type SADR2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SADR3EN` reader - Slave Address 3 Enable"]
pub type SADR3EN_R = crate::BitReader;
#[doc = "Field `SADR3EN` writer - Slave Address 3 Enable"]
pub type SADR3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATAMEN` reader - Data Matching Enable"]
pub type DATAMEN_R = crate::BitReader;
#[doc = "Field `DATAMEN` writer - Data Matching Enable"]
pub type DATAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NACKEN_R {
        NACKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SMDA_R {
        SMDA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SMHH_R {
        SMHH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&self) -> SCLWSDIS_R {
        SCLWSDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    pub fn sadr1en(&self) -> SADR1EN_R {
        SADR1EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    pub fn sadr2en(&self) -> SADR2EN_R {
        SADR2EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    pub fn sadr3en(&self) -> SADR3EN_R {
        SADR3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    pub fn datamen(&self) -> DATAMEN_R {
        DATAMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK enable"]
    #[inline(always)]
    #[must_use]
    pub fn nacken(&mut self) -> NACKEN_W<SMR_SPEC, 0> {
        NACKEN_W::new(self)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    #[must_use]
    pub fn smda(&mut self) -> SMDA_W<SMR_SPEC, 2> {
        SMDA_W::new(self)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    #[must_use]
    pub fn smhh(&mut self) -> SMHH_W<SMR_SPEC, 3> {
        SMHH_W::new(self)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sclwsdis(&mut self) -> SCLWSDIS_W<SMR_SPEC, 6> {
        SCLWSDIS_W::new(self)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<SMR_SPEC, 8> {
        MASK_W::new(self)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn sadr(&mut self) -> SADR_W<SMR_SPEC, 16> {
        SADR_W::new(self)
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sadr1en(&mut self) -> SADR1EN_W<SMR_SPEC, 28> {
        SADR1EN_W::new(self)
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sadr2en(&mut self) -> SADR2EN_W<SMR_SPEC, 29> {
        SADR2EN_W::new(self)
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sadr3en(&mut self) -> SADR3EN_W<SMR_SPEC, 30> {
        SADR3EN_W::new(self)
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datamen(&mut self) -> DATAMEN_W<SMR_SPEC, 31> {
        DATAMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Slave Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMR_SPEC;
impl crate::RegisterSpec for SMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smr::R`](R) reader structure"]
impl crate::Readable for SMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smr::W`](W) writer structure"]
impl crate::Writable for SMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMR to value 0"]
impl crate::Resettable for SMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
