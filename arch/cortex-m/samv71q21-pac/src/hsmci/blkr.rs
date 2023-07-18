#[doc = "Register `BLKR` reader"]
pub struct R(crate::R<BLKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLKR` writer"]
pub struct W(crate::W<BLKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLKR_SPEC>;
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
impl From<crate::W<BLKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNT` reader - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `BCNT` writer - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BCNT_W<'a, const O: u8> = crate::FieldWriter<'a, BLKR_SPEC, 16, O, u16>;
#[doc = "Field `BLKLEN` reader - Data Block Length"]
pub type BLKLEN_R = crate::FieldReader<u16>;
#[doc = "Field `BLKLEN` writer - Data Block Length"]
pub type BLKLEN_W<'a, const O: u8> = crate::FieldWriter<'a, BLKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    pub fn blklen(&self) -> BLKLEN_R {
        BLKLEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    #[must_use]
    pub fn bcnt(&mut self) -> BCNT_W<0> {
        BCNT_W::new(self)
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    #[must_use]
    pub fn blklen(&mut self) -> BLKLEN_W<16> {
        BLKLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blkr](index.html) module"]
pub struct BLKR_SPEC;
impl crate::RegisterSpec for BLKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blkr::R](R) reader structure"]
impl crate::Readable for BLKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blkr::W](W) writer structure"]
impl crate::Writable for BLKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLKR to value 0"]
impl crate::Resettable for BLKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
