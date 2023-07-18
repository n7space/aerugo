#[doc = "Register `US_TTGR_USART_MODE` reader"]
pub struct R(crate::R<US_TTGR_USART_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_TTGR_USART_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_TTGR_USART_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_TTGR_USART_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_TTGR_USART_MODE` writer"]
pub struct W(crate::W<US_TTGR_USART_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_TTGR_USART_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<US_TTGR_USART_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_TTGR_USART_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TG` reader - Timeguard Value"]
pub type TG_R = crate::FieldReader;
#[doc = "Field `TG` writer - Timeguard Value"]
pub type TG_W<'a, const O: u8> = crate::FieldWriter<'a, US_TTGR_USART_MODE_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<0> {
        TG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Timeguard Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ttgr_usart_mode](index.html) module"]
pub struct US_TTGR_USART_MODE_SPEC;
impl crate::RegisterSpec for US_TTGR_USART_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_ttgr_usart_mode::R](R) reader structure"]
impl crate::Readable for US_TTGR_USART_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_ttgr_usart_mode::W](W) writer structure"]
impl crate::Writable for US_TTGR_USART_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_TTGR_USART_MODE to value 0"]
impl crate::Resettable for US_TTGR_USART_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
