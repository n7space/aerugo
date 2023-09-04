#[doc = "Register `HCTL` reader"]
pub type R = crate::R<HCTL_SPEC>;
#[doc = "Register `HCTL` writer"]
pub type W = crate::W<HCTL_SPEC>;
#[doc = "Field `RST0` reader - Address Generation Unit 0 Software Reset"]
pub type RST0_R = crate::BitReader;
#[doc = "Field `RST0` writer - Address Generation Unit 0 Software Reset"]
pub type RST0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RST1` reader - Address Generation Unit 1 Software Reset"]
pub type RST1_R = crate::BitReader;
#[doc = "Field `RST1` writer - Address Generation Unit 1 Software Reset"]
pub type RST1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN` reader - HBI Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - HBI Enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> RST0_W<HCTL_SPEC, 0> {
        RST0_W::new(self)
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> RST1_W<HCTL_SPEC, 1> {
        RST1_W::new(self)
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<HCTL_SPEC, 15> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HBI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTL_SPEC;
impl crate::RegisterSpec for HCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctl::R`](R) reader structure"]
impl crate::Readable for HCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctl::W`](W) writer structure"]
impl crate::Writable for HCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTL to value 0"]
impl crate::Resettable for HCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
