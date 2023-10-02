#[doc = "Register `CUBC` reader"]
pub type R = crate::R<CUBC_SPEC>;
#[doc = "Register `CUBC` writer"]
pub type W = crate::W<CUBC_SPEC>;
#[doc = "Field `UBLEN` reader - Channel x Microblock Length"]
pub type UBLEN_R = crate::FieldReader<u32>;
#[doc = "Field `UBLEN` writer - Channel x Microblock Length"]
pub type UBLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&self) -> UBLEN_R {
        UBLEN_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    #[must_use]
    pub fn ublen(&mut self) -> UBLEN_W<CUBC_SPEC, 0> {
        UBLEN_W::new(self)
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
#[doc = "Channel Microblock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cubc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cubc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CUBC_SPEC;
impl crate::RegisterSpec for CUBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cubc::R`](R) reader structure"]
impl crate::Readable for CUBC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cubc::W`](W) writer structure"]
impl crate::Writable for CUBC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CUBC to value 0"]
impl crate::Resettable for CUBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
