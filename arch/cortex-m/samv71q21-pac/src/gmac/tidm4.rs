#[doc = "Register `TIDM4` reader"]
pub type R = crate::R<TIDM4_SPEC>;
#[doc = "Register `TIDM4` writer"]
pub type W = crate::W<TIDM4_SPEC>;
#[doc = "Field `TID` reader - Type ID Match 4"]
pub type TID_R = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 4"]
pub type TID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ENID4` reader - Enable Copying of TID Matched Frames"]
pub type ENID4_R = crate::BitReader;
#[doc = "Field `ENID4` writer - Enable Copying of TID Matched Frames"]
pub type ENID4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 4"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid4(&self) -> ENID4_R {
        ENID4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 4"]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TID_W<TIDM4_SPEC, 0> {
        TID_W::new(self)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    #[must_use]
    pub fn enid4(&mut self) -> ENID4_W<TIDM4_SPEC, 31> {
        ENID4_W::new(self)
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
#[doc = "Type ID Match 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tidm4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tidm4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIDM4_SPEC;
impl crate::RegisterSpec for TIDM4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm4::R`](R) reader structure"]
impl crate::Readable for TIDM4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tidm4::W`](W) writer structure"]
impl crate::Writable for TIDM4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIDM4 to value 0"]
impl crate::Resettable for TIDM4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
