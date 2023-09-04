#[doc = "Register `US_TTGR_LON_MODE` reader"]
pub type R = crate::R<US_TTGR_LON_MODE_SPEC>;
#[doc = "Register `US_TTGR_LON_MODE` writer"]
pub type W = crate::W<US_TTGR_LON_MODE_SPEC>;
#[doc = "Field `PCYCLE` reader - LON PCYCLE Length"]
pub type PCYCLE_R = crate::FieldReader<u32>;
#[doc = "Field `PCYCLE` writer - LON PCYCLE Length"]
pub type PCYCLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - LON PCYCLE Length"]
    #[inline(always)]
    pub fn pcycle(&self) -> PCYCLE_R {
        PCYCLE_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON PCYCLE Length"]
    #[inline(always)]
    #[must_use]
    pub fn pcycle(&mut self) -> PCYCLE_W<US_TTGR_LON_MODE_SPEC, 0> {
        PCYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmitter Timeguard Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_ttgr_lon_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_ttgr_lon_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_TTGR_LON_MODE_SPEC;
impl crate::RegisterSpec for US_TTGR_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_ttgr_lon_mode::R`](R) reader structure"]
impl crate::Readable for US_TTGR_LON_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_ttgr_lon_mode::W`](W) writer structure"]
impl crate::Writable for US_TTGR_LON_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_TTGR_LON_MODE to value 0"]
impl crate::Resettable for US_TTGR_LON_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
