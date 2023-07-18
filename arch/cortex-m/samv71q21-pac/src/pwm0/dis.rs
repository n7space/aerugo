#[doc = "Register `DIS` writer"]
pub struct W(crate::W<DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHID0` writer - Channel ID"]
pub type CHID0_W<'a, const O: u8> = crate::BitWriter<'a, DIS_SPEC, O>;
#[doc = "Field `CHID1` writer - Channel ID"]
pub type CHID1_W<'a, const O: u8> = crate::BitWriter<'a, DIS_SPEC, O>;
#[doc = "Field `CHID2` writer - Channel ID"]
pub type CHID2_W<'a, const O: u8> = crate::BitWriter<'a, DIS_SPEC, O>;
#[doc = "Field `CHID3` writer - Channel ID"]
pub type CHID3_W<'a, const O: u8> = crate::BitWriter<'a, DIS_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid0(&mut self) -> CHID0_W<0> {
        CHID0_W::new(self)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid1(&mut self) -> CHID1_W<1> {
        CHID1_W::new(self)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid2(&mut self) -> CHID2_W<2> {
        CHID2_W::new(self)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid3(&mut self) -> CHID3_W<3> {
        CHID3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis](index.html) module"]
pub struct DIS_SPEC;
impl crate::RegisterSpec for DIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dis::W](W) writer structure"]
impl crate::Writable for DIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIS to value 0"]
impl crate::Resettable for DIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
