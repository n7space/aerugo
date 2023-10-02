#[doc = "Register `US_LONB1TX` reader"]
pub type R = crate::R<US_LONB1TX_SPEC>;
#[doc = "Register `US_LONB1TX` writer"]
pub type W = crate::W<US_LONB1TX_SPEC>;
#[doc = "Field `BETA1TX` reader - LON Beta1 Length after Transmission"]
pub type BETA1TX_R = crate::FieldReader<u32>;
#[doc = "Field `BETA1TX` writer - LON Beta1 Length after Transmission"]
pub type BETA1TX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Beta1 Length after Transmission"]
    #[inline(always)]
    pub fn beta1tx(&self) -> BETA1TX_R {
        BETA1TX_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Beta1 Length after Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn beta1tx(&mut self) -> BETA1TX_W<US_LONB1TX_SPEC, 0> {
        BETA1TX_W::new(self)
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
#[doc = "LON Beta1 Tx Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_lonb1tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_lonb1tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LONB1TX_SPEC;
impl crate::RegisterSpec for US_LONB1TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonb1tx::R`](R) reader structure"]
impl crate::Readable for US_LONB1TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_lonb1tx::W`](W) writer structure"]
impl crate::Writable for US_LONB1TX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONB1TX to value 0"]
impl crate::Resettable for US_LONB1TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
