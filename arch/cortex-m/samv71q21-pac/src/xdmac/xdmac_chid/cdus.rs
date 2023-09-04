#[doc = "Register `CDUS` reader"]
pub type R = crate::R<CDUS_SPEC>;
#[doc = "Register `CDUS` writer"]
pub type W = crate::W<CDUS_SPEC>;
#[doc = "Field `DUBS` reader - Channel x Destination Microblock Stride"]
pub type DUBS_R = crate::FieldReader<u32>;
#[doc = "Field `DUBS` writer - Channel x Destination Microblock Stride"]
pub type DUBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    pub fn dubs(&self) -> DUBS_R {
        DUBS_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    #[must_use]
    pub fn dubs(&mut self) -> DUBS_W<CDUS_SPEC, 0> {
        DUBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Destination Microblock Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDUS_SPEC;
impl crate::RegisterSpec for CDUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdus::R`](R) reader structure"]
impl crate::Readable for CDUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdus::W`](W) writer structure"]
impl crate::Writable for CDUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDUS to value 0"]
impl crate::Resettable for CDUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
