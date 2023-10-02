#[doc = "Register `CBC` reader"]
pub type R = crate::R<CBC_SPEC>;
#[doc = "Register `CBC` writer"]
pub type W = crate::W<CBC_SPEC>;
#[doc = "Field `BLEN` reader - Channel x Block Length"]
pub type BLEN_R = crate::FieldReader<u16>;
#[doc = "Field `BLEN` writer - Channel x Block Length"]
pub type BLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    #[must_use]
    pub fn blen(&mut self) -> BLEN_W<CBC_SPEC, 0> {
        BLEN_W::new(self)
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
#[doc = "Channel Block Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBC_SPEC;
impl crate::RegisterSpec for CBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbc::R`](R) reader structure"]
impl crate::Readable for CBC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cbc::W`](W) writer structure"]
impl crate::Writable for CBC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBC to value 0"]
impl crate::Resettable for CBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
