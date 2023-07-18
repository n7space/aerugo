#[doc = "Register `GSWF` writer"]
pub struct W(crate::W<GSWF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GSWF_SPEC>;
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
impl From<crate::W<GSWF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GSWF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWF0` writer - XDMAC Channel 0 Software Flush Request Bit"]
pub type SWF0_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF1` writer - XDMAC Channel 1 Software Flush Request Bit"]
pub type SWF1_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF2` writer - XDMAC Channel 2 Software Flush Request Bit"]
pub type SWF2_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF3` writer - XDMAC Channel 3 Software Flush Request Bit"]
pub type SWF3_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF4` writer - XDMAC Channel 4 Software Flush Request Bit"]
pub type SWF4_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF5` writer - XDMAC Channel 5 Software Flush Request Bit"]
pub type SWF5_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF6` writer - XDMAC Channel 6 Software Flush Request Bit"]
pub type SWF6_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF7` writer - XDMAC Channel 7 Software Flush Request Bit"]
pub type SWF7_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF8` writer - XDMAC Channel 8 Software Flush Request Bit"]
pub type SWF8_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF9` writer - XDMAC Channel 9 Software Flush Request Bit"]
pub type SWF9_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF10` writer - XDMAC Channel 10 Software Flush Request Bit"]
pub type SWF10_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF11` writer - XDMAC Channel 11 Software Flush Request Bit"]
pub type SWF11_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF12` writer - XDMAC Channel 12 Software Flush Request Bit"]
pub type SWF12_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF13` writer - XDMAC Channel 13 Software Flush Request Bit"]
pub type SWF13_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF14` writer - XDMAC Channel 14 Software Flush Request Bit"]
pub type SWF14_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF15` writer - XDMAC Channel 15 Software Flush Request Bit"]
pub type SWF15_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF16` writer - XDMAC Channel 16 Software Flush Request Bit"]
pub type SWF16_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF17` writer - XDMAC Channel 17 Software Flush Request Bit"]
pub type SWF17_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF18` writer - XDMAC Channel 18 Software Flush Request Bit"]
pub type SWF18_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF19` writer - XDMAC Channel 19 Software Flush Request Bit"]
pub type SWF19_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF20` writer - XDMAC Channel 20 Software Flush Request Bit"]
pub type SWF20_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF21` writer - XDMAC Channel 21 Software Flush Request Bit"]
pub type SWF21_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF22` writer - XDMAC Channel 22 Software Flush Request Bit"]
pub type SWF22_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
#[doc = "Field `SWF23` writer - XDMAC Channel 23 Software Flush Request Bit"]
pub type SWF23_W<'a, const O: u8> = crate::BitWriter<'a, GSWF_SPEC, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf0(&mut self) -> SWF0_W<0> {
        SWF0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf1(&mut self) -> SWF1_W<1> {
        SWF1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf2(&mut self) -> SWF2_W<2> {
        SWF2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf3(&mut self) -> SWF3_W<3> {
        SWF3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf4(&mut self) -> SWF4_W<4> {
        SWF4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf5(&mut self) -> SWF5_W<5> {
        SWF5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf6(&mut self) -> SWF6_W<6> {
        SWF6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf7(&mut self) -> SWF7_W<7> {
        SWF7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf8(&mut self) -> SWF8_W<8> {
        SWF8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf9(&mut self) -> SWF9_W<9> {
        SWF9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf10(&mut self) -> SWF10_W<10> {
        SWF10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf11(&mut self) -> SWF11_W<11> {
        SWF11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf12(&mut self) -> SWF12_W<12> {
        SWF12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf13(&mut self) -> SWF13_W<13> {
        SWF13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf14(&mut self) -> SWF14_W<14> {
        SWF14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf15(&mut self) -> SWF15_W<15> {
        SWF15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf16(&mut self) -> SWF16_W<16> {
        SWF16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf17(&mut self) -> SWF17_W<17> {
        SWF17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf18(&mut self) -> SWF18_W<18> {
        SWF18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf19(&mut self) -> SWF19_W<19> {
        SWF19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf20(&mut self) -> SWF20_W<20> {
        SWF20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf21(&mut self) -> SWF21_W<21> {
        SWF21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf22(&mut self) -> SWF22_W<22> {
        SWF22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Flush Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swf23(&mut self) -> SWF23_W<23> {
        SWF23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Software Flush Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gswf](index.html) module"]
pub struct GSWF_SPEC;
impl crate::RegisterSpec for GSWF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gswf::W](W) writer structure"]
impl crate::Writable for GSWF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GSWF to value 0"]
impl crate::Resettable for GSWF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
