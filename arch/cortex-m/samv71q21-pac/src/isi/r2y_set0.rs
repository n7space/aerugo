#[doc = "Register `R2Y_SET0` reader"]
pub type R = crate::R<R2Y_SET0_SPEC>;
#[doc = "Register `R2Y_SET0` writer"]
pub type W = crate::W<R2Y_SET0_SPEC>;
#[doc = "Field `C0` reader - Color Space Conversion Matrix Coefficient C0"]
pub type C0_R = crate::FieldReader;
#[doc = "Field `C0` writer - Color Space Conversion Matrix Coefficient C0"]
pub type C0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `C1` reader - Color Space Conversion Matrix Coefficient C1"]
pub type C1_R = crate::FieldReader;
#[doc = "Field `C1` writer - Color Space Conversion Matrix Coefficient C1"]
pub type C1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `C2` reader - Color Space Conversion Matrix Coefficient C2"]
pub type C2_R = crate::FieldReader;
#[doc = "Field `C2` writer - Color Space Conversion Matrix Coefficient C2"]
pub type C2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `Roff` reader - Color Space Conversion Red Component Offset"]
pub type ROFF_R = crate::BitReader;
#[doc = "Field `Roff` writer - Color Space Conversion Red Component Offset"]
pub type ROFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Red Component Offset"]
    #[inline(always)]
    pub fn roff(&self) -> ROFF_R {
        ROFF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    #[must_use]
    pub fn c0(&mut self) -> C0_W<R2Y_SET0_SPEC, 0> {
        C0_W::new(self)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1_W<R2Y_SET0_SPEC, 8> {
        C1_W::new(self)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2_W<R2Y_SET0_SPEC, 16> {
        C2_W::new(self)
    }
    #[doc = "Bit 24 - Color Space Conversion Red Component Offset"]
    #[inline(always)]
    #[must_use]
    pub fn roff(&mut self) -> ROFF_W<R2Y_SET0_SPEC, 24> {
        ROFF_W::new(self)
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
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2y_set0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2y_set0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2Y_SET0_SPEC;
impl crate::RegisterSpec for R2Y_SET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r2y_set0::R`](R) reader structure"]
impl crate::Readable for R2Y_SET0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r2y_set0::W`](W) writer structure"]
impl crate::Writable for R2Y_SET0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R2Y_SET0 to value 0"]
impl crate::Resettable for R2Y_SET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
