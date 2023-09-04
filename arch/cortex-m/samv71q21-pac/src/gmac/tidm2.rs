#[doc = "Register `TIDM2` reader"]
pub type R = crate::R<TIDM2_SPEC>;
#[doc = "Register `TIDM2` writer"]
pub type W = crate::W<TIDM2_SPEC>;
#[doc = "Field `TID` reader - Type ID Match 2"]
pub type TID_R = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 2"]
pub type TID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ENID2` reader - Enable Copying of TID Matched Frames"]
pub type ENID2_R = crate::BitReader;
#[doc = "Field `ENID2` writer - Enable Copying of TID Matched Frames"]
pub type ENID2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 2"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid2(&self) -> ENID2_R {
        ENID2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 2"]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TID_W<TIDM2_SPEC, 0> {
        TID_W::new(self)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    #[must_use]
    pub fn enid2(&mut self) -> ENID2_W<TIDM2_SPEC, 31> {
        ENID2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Type ID Match 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tidm2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tidm2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIDM2_SPEC;
impl crate::RegisterSpec for TIDM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm2::R`](R) reader structure"]
impl crate::Readable for TIDM2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tidm2::W`](W) writer structure"]
impl crate::Writable for TIDM2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIDM2 to value 0"]
impl crate::Resettable for TIDM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
