#[doc = "Register `RBQBAPQ[%s]` reader"]
pub struct R(crate::R<RBQBAPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBQBAPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBQBAPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBQBAPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBQBAPQ[%s]` writer"]
pub struct W(crate::W<RBQBAPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBQBAPQ_SPEC>;
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
impl From<crate::W<RBQBAPQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBQBAPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXBQBA` reader - Receive Buffer Queue Base Address"]
pub type RXBQBA_R = crate::FieldReader<u32>;
#[doc = "Field `RXBQBA` writer - Receive Buffer Queue Base Address"]
pub type RXBQBA_W<'a, const O: u8> = crate::FieldWriter<'a, RBQBAPQ_SPEC, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&self) -> RXBQBA_R {
        RXBQBA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn rxbqba(&mut self) -> RXBQBA_W<2> {
        RXBQBA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbqbapq](index.html) module"]
pub struct RBQBAPQ_SPEC;
impl crate::RegisterSpec for RBQBAPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbqbapq::R](R) reader structure"]
impl crate::Readable for RBQBAPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbqbapq::W](W) writer structure"]
impl crate::Writable for RBQBAPQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBQBAPQ[%s]
to value 0"]
impl crate::Resettable for RBQBAPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
