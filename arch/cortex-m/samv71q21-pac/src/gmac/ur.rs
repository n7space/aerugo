#[doc = "Register `UR` reader"]
pub type R = crate::R<UR_SPEC>;
#[doc = "Register `UR` writer"]
pub type W = crate::W<UR_SPEC>;
#[doc = "Field `RMII` reader - Reduced MII Mode"]
pub type RMII_R = crate::BitReader;
#[doc = "Field `RMII` writer - Reduced MII Mode"]
pub type RMII_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Reduced MII Mode"]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reduced MII Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rmii(&mut self) -> RMII_W<UR_SPEC, 0> {
        RMII_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "User Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR_SPEC;
impl crate::RegisterSpec for UR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur::R`](R) reader structure"]
impl crate::Readable for UR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ur::W`](W) writer structure"]
impl crate::Writable for UR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR to value 0"]
impl crate::Resettable for UR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
