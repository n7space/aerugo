#[doc = "Register `US_FIDI_LON_MODE` reader"]
pub type R = crate::R<US_FIDI_LON_MODE_SPEC>;
#[doc = "Register `US_FIDI_LON_MODE` writer"]
pub type W = crate::W<US_FIDI_LON_MODE_SPEC>;
#[doc = "Field `BETA2` reader - LON BETA2 Length"]
pub type BETA2_R = crate::FieldReader<u32>;
#[doc = "Field `BETA2` writer - LON BETA2 Length"]
pub type BETA2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - LON BETA2 Length"]
    #[inline(always)]
    pub fn beta2(&self) -> BETA2_R {
        BETA2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON BETA2 Length"]
    #[inline(always)]
    #[must_use]
    pub fn beta2(&mut self) -> BETA2_W<US_FIDI_LON_MODE_SPEC, 0> {
        BETA2_W::new(self)
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
#[doc = "FI DI Ratio Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_fidi_lon_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_fidi_lon_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_FIDI_LON_MODE_SPEC;
impl crate::RegisterSpec for US_FIDI_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_fidi_lon_mode::R`](R) reader structure"]
impl crate::Readable for US_FIDI_LON_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_fidi_lon_mode::W`](W) writer structure"]
impl crate::Writable for US_FIDI_LON_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_FIDI_LON_MODE to value 0"]
impl crate::Resettable for US_FIDI_LON_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
