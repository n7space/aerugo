#[doc = "Register `US_LONB1RX` reader"]
pub type R = crate::R<US_LONB1RX_SPEC>;
#[doc = "Register `US_LONB1RX` writer"]
pub type W = crate::W<US_LONB1RX_SPEC>;
#[doc = "Field `BETA1RX` reader - LON Beta1 Length after Reception"]
pub type BETA1RX_R = crate::FieldReader<u32>;
#[doc = "Field `BETA1RX` writer - LON Beta1 Length after Reception"]
pub type BETA1RX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Beta1 Length after Reception"]
    #[inline(always)]
    pub fn beta1rx(&self) -> BETA1RX_R {
        BETA1RX_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Beta1 Length after Reception"]
    #[inline(always)]
    #[must_use]
    pub fn beta1rx(&mut self) -> BETA1RX_W<US_LONB1RX_SPEC, 0> {
        BETA1RX_W::new(self)
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
#[doc = "LON Beta1 Rx Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_lonb1rx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_lonb1rx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LONB1RX_SPEC;
impl crate::RegisterSpec for US_LONB1RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonb1rx::R`](R) reader structure"]
impl crate::Readable for US_LONB1RX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_lonb1rx::W`](W) writer structure"]
impl crate::Writable for US_LONB1RX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONB1RX to value 0"]
impl crate::Resettable for US_LONB1RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
