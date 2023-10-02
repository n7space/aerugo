#[doc = "Register `CCFG_PCCR` reader"]
pub type R = crate::R<CCFG_PCCR_SPEC>;
#[doc = "Register `CCFG_PCCR` writer"]
pub type W = crate::W<CCFG_PCCR_SPEC>;
#[doc = "Field `TC0CC` reader - TC0 Clock Configuration"]
pub type TC0CC_R = crate::BitReader;
#[doc = "Field `TC0CC` writer - TC0 Clock Configuration"]
pub type TC0CC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2SC0CC` reader - I2SC0 Clock Configuration"]
pub type I2SC0CC_R = crate::BitReader;
#[doc = "Field `I2SC0CC` writer - I2SC0 Clock Configuration"]
pub type I2SC0CC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2SC1CC` reader - I2SC1 Clock Configuration"]
pub type I2SC1CC_R = crate::BitReader;
#[doc = "Field `I2SC1CC` writer - I2SC1 Clock Configuration"]
pub type I2SC1CC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    pub fn tc0cc(&self) -> TC0CC_R {
        TC0CC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc0cc(&self) -> I2SC0CC_R {
        I2SC0CC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc1cc(&self) -> I2SC1CC_R {
        I2SC1CC_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn tc0cc(&mut self) -> TC0CC_W<CCFG_PCCR_SPEC, 20> {
        TC0CC_W::new(self)
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn i2sc0cc(&mut self) -> I2SC0CC_W<CCFG_PCCR_SPEC, 21> {
        I2SC0CC_W::new(self)
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn i2sc1cc(&mut self) -> I2SC1CC_W<CCFG_PCCR_SPEC, 22> {
        I2SC1CC_W::new(self)
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
#[doc = "Peripheral Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_pccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_pccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFG_PCCR_SPEC;
impl crate::RegisterSpec for CCFG_PCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_pccr::R`](R) reader structure"]
impl crate::Readable for CCFG_PCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccfg_pccr::W`](W) writer structure"]
impl crate::Writable for CCFG_PCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_PCCR to value 0"]
impl crate::Resettable for CCFG_PCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
