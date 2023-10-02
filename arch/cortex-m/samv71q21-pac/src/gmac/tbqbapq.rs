#[doc = "Register `TBQBAPQ[%s]` reader"]
pub type R = crate::R<TBQBAPQ_SPEC>;
#[doc = "Register `TBQBAPQ[%s]` writer"]
pub type W = crate::W<TBQBAPQ_SPEC>;
#[doc = "Field `TXBQBA` reader - Transmit Buffer Queue Base Address"]
pub type TXBQBA_R = crate::FieldReader<u32>;
#[doc = "Field `TXBQBA` writer - Transmit Buffer Queue Base Address"]
pub type TXBQBA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn txbqba(&self) -> TXBQBA_R {
        TXBQBA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn txbqba(&mut self) -> TXBQBA_W<TBQBAPQ_SPEC, 2> {
        TXBQBA_W::new(self)
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
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (1..5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbqbapq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbqbapq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBQBAPQ_SPEC;
impl crate::RegisterSpec for TBQBAPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbqbapq::R`](R) reader structure"]
impl crate::Readable for TBQBAPQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbqbapq::W`](W) writer structure"]
impl crate::Writable for TBQBAPQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBQBAPQ[%s]
to value 0"]
impl crate::Resettable for TBQBAPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
