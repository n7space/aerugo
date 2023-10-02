#[doc = "Register `BLKR` reader"]
pub type R = crate::R<BLKR_SPEC>;
#[doc = "Register `BLKR` writer"]
pub type W = crate::W<BLKR_SPEC>;
#[doc = "Field `BCNT` reader - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `BCNT` writer - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `BLKLEN` reader - Data Block Length"]
pub type BLKLEN_R = crate::FieldReader<u16>;
#[doc = "Field `BLKLEN` writer - Data Block Length"]
pub type BLKLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    pub fn bcnt(&mut self) -> BCNT_W<BLKR_SPEC, 0> {
        BCNT_W::new(self)
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    #[must_use]
    pub fn blklen(&mut self) -> BLKLEN_W<BLKR_SPEC, 16> {
        BLKLEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Block Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLKR_SPEC;
impl crate::RegisterSpec for BLKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blkr::R`](R) reader structure"]
impl crate::Readable for BLKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blkr::W`](W) writer structure"]
impl crate::Writable for BLKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLKR to value 0"]
impl crate::Resettable for BLKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
