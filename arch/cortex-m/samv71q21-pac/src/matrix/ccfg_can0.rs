#[doc = "Register `CCFG_CAN0` reader"]
pub type R = crate::R<CCFG_CAN0_SPEC>;
#[doc = "Register `CCFG_CAN0` writer"]
pub type W = crate::W<CCFG_CAN0_SPEC>;
#[doc = "Field `CAN0DMABA` reader - CAN0 DMA Base Address"]
pub type CAN0DMABA_R = crate::FieldReader<u16>;
#[doc = "Field `CAN0DMABA` writer - CAN0 DMA Base Address"]
pub type CAN0DMABA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    pub fn can0dmaba(&self) -> CAN0DMABA_R {
        CAN0DMABA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn can0dmaba(&mut self) -> CAN0DMABA_W<CCFG_CAN0_SPEC, 16> {
        CAN0DMABA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CAN0 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_can0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_can0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFG_CAN0_SPEC;
impl crate::RegisterSpec for CCFG_CAN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_can0::R`](R) reader structure"]
impl crate::Readable for CCFG_CAN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccfg_can0::W`](W) writer structure"]
impl crate::Writable for CCFG_CAN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_CAN0 to value 0"]
impl crate::Resettable for CCFG_CAN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
