#[doc = "Register `RBQBAPQ[%s]` reader"]
pub type R = crate::R<RBQBAPQ_SPEC>;
#[doc = "Register `RBQBAPQ[%s]` writer"]
pub type W = crate::W<RBQBAPQ_SPEC>;
#[doc = "Field `RXBQBA` reader - Receive Buffer Queue Base Address"]
pub type RXBQBA_R = crate::FieldReader<u32>;
#[doc = "Field `RXBQBA` writer - Receive Buffer Queue Base Address"]
pub type RXBQBA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
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
    pub fn rxbqba(&mut self) -> RXBQBA_W<RBQBAPQ_SPEC, 2> {
        RXBQBA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (1..5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbqbapq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbqbapq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBQBAPQ_SPEC;
impl crate::RegisterSpec for RBQBAPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbqbapq::R`](R) reader structure"]
impl crate::Readable for RBQBAPQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rbqbapq::W`](W) writer structure"]
impl crate::Writable for RBQBAPQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBQBAPQ[%s]
to value 0"]
impl crate::Resettable for RBQBAPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
