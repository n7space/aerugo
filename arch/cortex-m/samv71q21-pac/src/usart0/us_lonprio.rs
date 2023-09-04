#[doc = "Register `US_LONPRIO` reader"]
pub type R = crate::R<US_LONPRIO_SPEC>;
#[doc = "Register `US_LONPRIO` writer"]
pub type W = crate::W<US_LONPRIO_SPEC>;
#[doc = "Field `PSNB` reader - LON Priority Slot Number"]
pub type PSNB_R = crate::FieldReader;
#[doc = "Field `PSNB` writer - LON Priority Slot Number"]
pub type PSNB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `NPS` reader - LON Node Priority Slot"]
pub type NPS_R = crate::FieldReader;
#[doc = "Field `NPS` writer - LON Node Priority Slot"]
pub type NPS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    pub fn psnb(&self) -> PSNB_R {
        PSNB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    pub fn nps(&self) -> NPS_R {
        NPS_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    #[must_use]
    pub fn psnb(&mut self) -> PSNB_W<US_LONPRIO_SPEC, 0> {
        PSNB_W::new(self)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    #[must_use]
    pub fn nps(&mut self) -> NPS_W<US_LONPRIO_SPEC, 8> {
        NPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LON Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_lonprio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_lonprio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LONPRIO_SPEC;
impl crate::RegisterSpec for US_LONPRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonprio::R`](R) reader structure"]
impl crate::Readable for US_LONPRIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_lonprio::W`](W) writer structure"]
impl crate::Writable for US_LONPRIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONPRIO to value 0"]
impl crate::Resettable for US_LONPRIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
