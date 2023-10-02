#[doc = "Register `CBSCR` reader"]
pub type R = crate::R<CBSCR_SPEC>;
#[doc = "Register `CBSCR` writer"]
pub type W = crate::W<CBSCR_SPEC>;
#[doc = "Field `QBE` reader - Queue B CBS Enable"]
pub type QBE_R = crate::BitReader;
#[doc = "Field `QBE` writer - Queue B CBS Enable"]
pub type QBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QAE` reader - Queue A CBS Enable"]
pub type QAE_R = crate::BitReader;
#[doc = "Field `QAE` writer - Queue A CBS Enable"]
pub type QAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    pub fn qbe(&self) -> QBE_R {
        QBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    pub fn qae(&self) -> QAE_R {
        QAE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qbe(&mut self) -> QBE_W<CBSCR_SPEC, 0> {
        QBE_W::new(self)
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qae(&mut self) -> QAE_W<CBSCR_SPEC, 1> {
        QAE_W::new(self)
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
#[doc = "Credit-Based Shaping Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBSCR_SPEC;
impl crate::RegisterSpec for CBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbscr::R`](R) reader structure"]
impl crate::Readable for CBSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cbscr::W`](W) writer structure"]
impl crate::Writable for CBSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBSCR to value 0"]
impl crate::Resettable for CBSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
