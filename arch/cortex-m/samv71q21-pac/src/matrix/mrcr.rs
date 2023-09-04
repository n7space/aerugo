#[doc = "Register `MRCR` reader"]
pub type R = crate::R<MRCR_SPEC>;
#[doc = "Register `MRCR` writer"]
pub type W = crate::W<MRCR_SPEC>;
#[doc = "Field `RCB0` reader - Remap Command Bit for Master 0"]
pub type RCB0_R = crate::BitReader;
#[doc = "Field `RCB0` writer - Remap Command Bit for Master 0"]
pub type RCB0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB1` reader - Remap Command Bit for Master 1"]
pub type RCB1_R = crate::BitReader;
#[doc = "Field `RCB1` writer - Remap Command Bit for Master 1"]
pub type RCB1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB2` reader - Remap Command Bit for Master 2"]
pub type RCB2_R = crate::BitReader;
#[doc = "Field `RCB2` writer - Remap Command Bit for Master 2"]
pub type RCB2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB3` reader - Remap Command Bit for Master 3"]
pub type RCB3_R = crate::BitReader;
#[doc = "Field `RCB3` writer - Remap Command Bit for Master 3"]
pub type RCB3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB4` reader - Remap Command Bit for Master 4"]
pub type RCB4_R = crate::BitReader;
#[doc = "Field `RCB4` writer - Remap Command Bit for Master 4"]
pub type RCB4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB5` reader - Remap Command Bit for Master 5"]
pub type RCB5_R = crate::BitReader;
#[doc = "Field `RCB5` writer - Remap Command Bit for Master 5"]
pub type RCB5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB6` reader - Remap Command Bit for Master 6"]
pub type RCB6_R = crate::BitReader;
#[doc = "Field `RCB6` writer - Remap Command Bit for Master 6"]
pub type RCB6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB7` reader - Remap Command Bit for Master 7"]
pub type RCB7_R = crate::BitReader;
#[doc = "Field `RCB7` writer - Remap Command Bit for Master 7"]
pub type RCB7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB8` reader - Remap Command Bit for Master 8"]
pub type RCB8_R = crate::BitReader;
#[doc = "Field `RCB8` writer - Remap Command Bit for Master 8"]
pub type RCB8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB9` reader - Remap Command Bit for Master 9"]
pub type RCB9_R = crate::BitReader;
#[doc = "Field `RCB9` writer - Remap Command Bit for Master 9"]
pub type RCB9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB10` reader - Remap Command Bit for Master 10"]
pub type RCB10_R = crate::BitReader;
#[doc = "Field `RCB10` writer - Remap Command Bit for Master 10"]
pub type RCB10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB11` reader - Remap Command Bit for Master 11"]
pub type RCB11_R = crate::BitReader;
#[doc = "Field `RCB11` writer - Remap Command Bit for Master 11"]
pub type RCB11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCB12` reader - Remap Command Bit for Master 12"]
pub type RCB12_R = crate::BitReader;
#[doc = "Field `RCB12` writer - Remap Command Bit for Master 12"]
pub type RCB12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> RCB0_R {
        RCB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> RCB1_R {
        RCB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> RCB2_R {
        RCB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> RCB3_R {
        RCB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> RCB4_R {
        RCB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&self) -> RCB5_R {
        RCB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&self) -> RCB6_R {
        RCB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Remap Command Bit for Master 7"]
    #[inline(always)]
    pub fn rcb7(&self) -> RCB7_R {
        RCB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&self) -> RCB8_R {
        RCB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&self) -> RCB9_R {
        RCB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&self) -> RCB10_R {
        RCB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&self) -> RCB11_R {
        RCB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&self) -> RCB12_R {
        RCB12_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    #[must_use]
    pub fn rcb0(&mut self) -> RCB0_W<MRCR_SPEC, 0> {
        RCB0_W::new(self)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    #[must_use]
    pub fn rcb1(&mut self) -> RCB1_W<MRCR_SPEC, 1> {
        RCB1_W::new(self)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    #[must_use]
    pub fn rcb2(&mut self) -> RCB2_W<MRCR_SPEC, 2> {
        RCB2_W::new(self)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    #[must_use]
    pub fn rcb3(&mut self) -> RCB3_W<MRCR_SPEC, 3> {
        RCB3_W::new(self)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    #[must_use]
    pub fn rcb4(&mut self) -> RCB4_W<MRCR_SPEC, 4> {
        RCB4_W::new(self)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    #[must_use]
    pub fn rcb5(&mut self) -> RCB5_W<MRCR_SPEC, 5> {
        RCB5_W::new(self)
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    #[must_use]
    pub fn rcb6(&mut self) -> RCB6_W<MRCR_SPEC, 6> {
        RCB6_W::new(self)
    }
    #[doc = "Bit 7 - Remap Command Bit for Master 7"]
    #[inline(always)]
    #[must_use]
    pub fn rcb7(&mut self) -> RCB7_W<MRCR_SPEC, 7> {
        RCB7_W::new(self)
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    #[must_use]
    pub fn rcb8(&mut self) -> RCB8_W<MRCR_SPEC, 8> {
        RCB8_W::new(self)
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    #[must_use]
    pub fn rcb9(&mut self) -> RCB9_W<MRCR_SPEC, 9> {
        RCB9_W::new(self)
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    #[must_use]
    pub fn rcb10(&mut self) -> RCB10_W<MRCR_SPEC, 10> {
        RCB10_W::new(self)
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    #[must_use]
    pub fn rcb11(&mut self) -> RCB11_W<MRCR_SPEC, 11> {
        RCB11_W::new(self)
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    #[must_use]
    pub fn rcb12(&mut self) -> RCB12_W<MRCR_SPEC, 12> {
        RCB12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Master Remap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MRCR_SPEC;
impl crate::RegisterSpec for MRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcr::R`](R) reader structure"]
impl crate::Readable for MRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mrcr::W`](W) writer structure"]
impl crate::Writable for MRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRCR to value 0"]
impl crate::Resettable for MRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
