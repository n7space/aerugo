#[doc = "Register `US_FIDI_USART_MODE` reader"]
pub type R = crate::R<US_FIDI_USART_MODE_SPEC>;
#[doc = "Register `US_FIDI_USART_MODE` writer"]
pub type W = crate::W<US_FIDI_USART_MODE_SPEC>;
#[doc = "Field `FI_DI_RATIO` reader - FI Over DI Ratio Value"]
pub type FI_DI_RATIO_R = crate::FieldReader<u16>;
#[doc = "Field `FI_DI_RATIO` writer - FI Over DI Ratio Value"]
pub type FI_DI_RATIO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&self) -> FI_DI_RATIO_R {
        FI_DI_RATIO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FI Over DI Ratio Value"]
    #[inline(always)]
    #[must_use]
    pub fn fi_di_ratio(&mut self) -> FI_DI_RATIO_W<US_FIDI_USART_MODE_SPEC, 0> {
        FI_DI_RATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FI DI Ratio Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_fidi_usart_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_fidi_usart_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_FIDI_USART_MODE_SPEC;
impl crate::RegisterSpec for US_FIDI_USART_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_fidi_usart_mode::R`](R) reader structure"]
impl crate::Readable for US_FIDI_USART_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_fidi_usart_mode::W`](W) writer structure"]
impl crate::Writable for US_FIDI_USART_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_FIDI_USART_MODE to value 0"]
impl crate::Resettable for US_FIDI_USART_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
