#[doc = "Register `CKGR_UCKR` reader"]
pub type R = crate::R<CKGR_UCKR_SPEC>;
#[doc = "Register `CKGR_UCKR` writer"]
pub type W = crate::W<CKGR_UCKR_SPEC>;
#[doc = "Field `UPLLEN` reader - UTMI PLL Enable"]
pub type UPLLEN_R = crate::BitReader;
#[doc = "Field `UPLLEN` writer - UTMI PLL Enable"]
pub type UPLLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPLLCOUNT` reader - UTMI PLL Start-up Time"]
pub type UPLLCOUNT_R = crate::FieldReader;
#[doc = "Field `UPLLCOUNT` writer - UTMI PLL Start-up Time"]
pub type UPLLCOUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 16 - UTMI PLL Enable"]
    #[inline(always)]
    pub fn upllen(&self) -> UPLLEN_R {
        UPLLEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - UTMI PLL Start-up Time"]
    #[inline(always)]
    pub fn upllcount(&self) -> UPLLCOUNT_R {
        UPLLCOUNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - UTMI PLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upllen(&mut self) -> UPLLEN_W<CKGR_UCKR_SPEC, 16> {
        UPLLEN_W::new(self)
    }
    #[doc = "Bits 20:23 - UTMI PLL Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn upllcount(&mut self) -> UPLLCOUNT_W<CKGR_UCKR_SPEC, 20> {
        UPLLCOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UTMI Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_uckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_uckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKGR_UCKR_SPEC;
impl crate::RegisterSpec for CKGR_UCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_uckr::R`](R) reader structure"]
impl crate::Readable for CKGR_UCKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckgr_uckr::W`](W) writer structure"]
impl crate::Writable for CKGR_UCKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKGR_UCKR to value 0"]
impl crate::Resettable for CKGR_UCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
