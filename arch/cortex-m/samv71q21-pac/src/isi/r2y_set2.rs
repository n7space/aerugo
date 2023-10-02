#[doc = "Register `R2Y_SET2` reader"]
pub type R = crate::R<R2Y_SET2_SPEC>;
#[doc = "Register `R2Y_SET2` writer"]
pub type W = crate::W<R2Y_SET2_SPEC>;
#[doc = "Field `C6` reader - Color Space Conversion Matrix Coefficient C6"]
pub type C6_R = crate::FieldReader;
#[doc = "Field `C6` writer - Color Space Conversion Matrix Coefficient C6"]
pub type C6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `C7` reader - Color Space Conversion Matrix Coefficient C7"]
pub type C7_R = crate::FieldReader;
#[doc = "Field `C7` writer - Color Space Conversion Matrix Coefficient C7"]
pub type C7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `C8` reader - Color Space Conversion Matrix Coefficient C8"]
pub type C8_R = crate::FieldReader;
#[doc = "Field `C8` writer - Color Space Conversion Matrix Coefficient C8"]
pub type C8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `Boff` reader - Color Space Conversion Blue Component Offset"]
pub type BOFF_R = crate::BitReader;
#[doc = "Field `Boff` writer - Color Space Conversion Blue Component Offset"]
pub type BOFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    pub fn c6(&self) -> C6_R {
        C6_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    pub fn c7(&self) -> C7_R {
        C7_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    pub fn c8(&self) -> C8_R {
        C8_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    #[must_use]
    pub fn c6(&mut self) -> C6_W<R2Y_SET2_SPEC, 0> {
        C6_W::new(self)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    #[must_use]
    pub fn c7(&mut self) -> C7_W<R2Y_SET2_SPEC, 8> {
        C7_W::new(self)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    #[must_use]
    pub fn c8(&mut self) -> C8_W<R2Y_SET2_SPEC, 16> {
        C8_W::new(self)
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    #[must_use]
    pub fn boff(&mut self) -> BOFF_W<R2Y_SET2_SPEC, 24> {
        BOFF_W::new(self)
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
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2y_set2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2y_set2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2Y_SET2_SPEC;
impl crate::RegisterSpec for R2Y_SET2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r2y_set2::R`](R) reader structure"]
impl crate::Readable for R2Y_SET2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r2y_set2::W`](W) writer structure"]
impl crate::Writable for R2Y_SET2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R2Y_SET2 to value 0"]
impl crate::Resettable for R2Y_SET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
