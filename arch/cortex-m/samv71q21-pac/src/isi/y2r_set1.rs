#[doc = "Register `Y2R_SET1` reader"]
pub type R = crate::R<Y2R_SET1_SPEC>;
#[doc = "Register `Y2R_SET1` writer"]
pub type W = crate::W<Y2R_SET1_SPEC>;
#[doc = "Field `C4` reader - Color Space Conversion Matrix Coefficient C4"]
pub type C4_R = crate::FieldReader<u16>;
#[doc = "Field `C4` writer - Color Space Conversion Matrix Coefficient C4"]
pub type C4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `Yoff` reader - Color Space Conversion Luminance Default Offset"]
pub type YOFF_R = crate::BitReader;
#[doc = "Field `Yoff` writer - Color Space Conversion Luminance Default Offset"]
pub type YOFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `Croff` reader - Color Space Conversion Red Chrominance Default Offset"]
pub type CROFF_R = crate::BitReader;
#[doc = "Field `Croff` writer - Color Space Conversion Red Chrominance Default Offset"]
pub type CROFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `Cboff` reader - Color Space Conversion Blue Chrominance Default Offset"]
pub type CBOFF_R = crate::BitReader;
#[doc = "Field `Cboff` writer - Color Space Conversion Blue Chrominance Default Offset"]
pub type CBOFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:8 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - Color Space Conversion Luminance Default Offset"]
    #[inline(always)]
    pub fn yoff(&self) -> YOFF_R {
        YOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Color Space Conversion Red Chrominance Default Offset"]
    #[inline(always)]
    pub fn croff(&self) -> CROFF_R {
        CROFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Color Space Conversion Blue Chrominance Default Offset"]
    #[inline(always)]
    pub fn cboff(&self) -> CBOFF_R {
        CBOFF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    #[must_use]
    pub fn c4(&mut self) -> C4_W<Y2R_SET1_SPEC, 0> {
        C4_W::new(self)
    }
    #[doc = "Bit 12 - Color Space Conversion Luminance Default Offset"]
    #[inline(always)]
    #[must_use]
    pub fn yoff(&mut self) -> YOFF_W<Y2R_SET1_SPEC, 12> {
        YOFF_W::new(self)
    }
    #[doc = "Bit 13 - Color Space Conversion Red Chrominance Default Offset"]
    #[inline(always)]
    #[must_use]
    pub fn croff(&mut self) -> CROFF_W<Y2R_SET1_SPEC, 13> {
        CROFF_W::new(self)
    }
    #[doc = "Bit 14 - Color Space Conversion Blue Chrominance Default Offset"]
    #[inline(always)]
    #[must_use]
    pub fn cboff(&mut self) -> CBOFF_W<Y2R_SET1_SPEC, 14> {
        CBOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`y2r_set1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`y2r_set1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Y2R_SET1_SPEC;
impl crate::RegisterSpec for Y2R_SET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`y2r_set1::R`](R) reader structure"]
impl crate::Readable for Y2R_SET1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`y2r_set1::W`](W) writer structure"]
impl crate::Writable for Y2R_SET1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Y2R_SET1 to value 0"]
impl crate::Resettable for Y2R_SET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
