#[doc = "Register `US_THR` writer"]
pub type W = crate::W<US_THR_SPEC>;
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TXCHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `TXSYNH` writer - Sync Field to be Transmitted"]
pub type TXSYNH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txchr(&mut self) -> TXCHR_W<US_THR_SPEC, 0> {
        TXCHR_W::new(self)
    }
    #[doc = "Bit 15 - Sync Field to be Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txsynh(&mut self) -> TXSYNH_W<US_THR_SPEC, 15> {
        TXSYNH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Holding Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_THR_SPEC;
impl crate::RegisterSpec for US_THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_thr::W`](W) writer structure"]
impl crate::Writable for US_THR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_THR to value 0"]
impl crate::Resettable for US_THR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
