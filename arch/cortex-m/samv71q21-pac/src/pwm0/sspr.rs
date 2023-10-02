#[doc = "Register `SSPR` reader"]
pub type R = crate::R<SSPR_SPEC>;
#[doc = "Register `SSPR` writer"]
pub type W = crate::W<SSPR_SPEC>;
#[doc = "Field `SPRD` reader - Spread Spectrum Limit Value"]
pub type SPRD_R = crate::FieldReader<u32>;
#[doc = "Field `SPRD` writer - Spread Spectrum Limit Value"]
pub type SPRD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `SPRDM` reader - Spread Spectrum Counter Mode"]
pub type SPRDM_R = crate::BitReader;
#[doc = "Field `SPRDM` writer - Spread Spectrum Counter Mode"]
pub type SPRDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    pub fn sprd(&self) -> SPRD_R {
        SPRD_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    pub fn sprdm(&self) -> SPRDM_R {
        SPRDM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    #[must_use]
    pub fn sprd(&mut self) -> SPRD_W<SSPR_SPEC, 0> {
        SPRD_W::new(self)
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sprdm(&mut self) -> SPRDM_W<SSPR_SPEC, 24> {
        SPRDM_W::new(self)
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
#[doc = "PWM Spread Spectrum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sspr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPR_SPEC;
impl crate::RegisterSpec for SSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspr::R`](R) reader structure"]
impl crate::Readable for SSPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspr::W`](W) writer structure"]
impl crate::Writable for SSPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPR to value 0"]
impl crate::Resettable for SSPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
