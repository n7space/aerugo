#[doc = "Register `TIDM3` reader"]
pub type R = crate::R<TIDM3_SPEC>;
#[doc = "Register `TIDM3` writer"]
pub type W = crate::W<TIDM3_SPEC>;
#[doc = "Field `TID` reader - Type ID Match 3"]
pub type TID_R = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 3"]
pub type TID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ENID3` reader - Enable Copying of TID Matched Frames"]
pub type ENID3_R = crate::BitReader;
#[doc = "Field `ENID3` writer - Enable Copying of TID Matched Frames"]
pub type ENID3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 3"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid3(&self) -> ENID3_R {
        ENID3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 3"]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TID_W<TIDM3_SPEC, 0> {
        TID_W::new(self)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    #[must_use]
    pub fn enid3(&mut self) -> ENID3_W<TIDM3_SPEC, 31> {
        ENID3_W::new(self)
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
#[doc = "Type ID Match 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tidm3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tidm3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIDM3_SPEC;
impl crate::RegisterSpec for TIDM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm3::R`](R) reader structure"]
impl crate::Readable for TIDM3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tidm3::W`](W) writer structure"]
impl crate::Writable for TIDM3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIDM3 to value 0"]
impl crate::Resettable for TIDM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
