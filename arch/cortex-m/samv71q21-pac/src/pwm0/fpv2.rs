#[doc = "Register `FPV2` reader"]
pub type R = crate::R<FPV2_SPEC>;
#[doc = "Register `FPV2` writer"]
pub type W = crate::W<FPV2_SPEC>;
#[doc = "Field `FPZH0` reader - Fault Protection to Hi-Z for PWMH output on channel 0"]
pub type FPZH0_R = crate::BitReader;
#[doc = "Field `FPZH0` writer - Fault Protection to Hi-Z for PWMH output on channel 0"]
pub type FPZH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPZH1` reader - Fault Protection to Hi-Z for PWMH output on channel 1"]
pub type FPZH1_R = crate::BitReader;
#[doc = "Field `FPZH1` writer - Fault Protection to Hi-Z for PWMH output on channel 1"]
pub type FPZH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPZH2` reader - Fault Protection to Hi-Z for PWMH output on channel 2"]
pub type FPZH2_R = crate::BitReader;
#[doc = "Field `FPZH2` writer - Fault Protection to Hi-Z for PWMH output on channel 2"]
pub type FPZH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPZH3` reader - Fault Protection to Hi-Z for PWMH output on channel 3"]
pub type FPZH3_R = crate::BitReader;
#[doc = "Field `FPZH3` writer - Fault Protection to Hi-Z for PWMH output on channel 3"]
pub type FPZH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPZL0` reader - Fault Protection to Hi-Z for PWML output on channel 0"]
pub type FPZL0_R = crate::BitReader;
#[doc = "Field `FPZL0` writer - Fault Protection to Hi-Z for PWML output on channel 0"]
pub type FPZL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPZL1` reader - Fault Protection to Hi-Z for PWML output on channel 1"]
pub type FPZL1_R = crate::BitReader;
#[doc = "Field `FPZL1` writer - Fault Protection to Hi-Z for PWML output on channel 1"]
pub type FPZL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPZL2` reader - Fault Protection to Hi-Z for PWML output on channel 2"]
pub type FPZL2_R = crate::BitReader;
#[doc = "Field `FPZL2` writer - Fault Protection to Hi-Z for PWML output on channel 2"]
pub type FPZL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPZL3` reader - Fault Protection to Hi-Z for PWML output on channel 3"]
pub type FPZL3_R = crate::BitReader;
#[doc = "Field `FPZL3` writer - Fault Protection to Hi-Z for PWML output on channel 3"]
pub type FPZL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&self) -> FPZH0_R {
        FPZH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&self) -> FPZH1_R {
        FPZH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&self) -> FPZH2_R {
        FPZH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&self) -> FPZH3_R {
        FPZH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&self) -> FPZL0_R {
        FPZL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&self) -> FPZL1_R {
        FPZL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&self) -> FPZL2_R {
        FPZL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&self) -> FPZL3_R {
        FPZL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpzh0(&mut self) -> FPZH0_W<FPV2_SPEC, 0> {
        FPZH0_W::new(self)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpzh1(&mut self) -> FPZH1_W<FPV2_SPEC, 1> {
        FPZH1_W::new(self)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpzh2(&mut self) -> FPZH2_W<FPV2_SPEC, 2> {
        FPZH2_W::new(self)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpzh3(&mut self) -> FPZH3_W<FPV2_SPEC, 3> {
        FPZH3_W::new(self)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpzl0(&mut self) -> FPZL0_W<FPV2_SPEC, 16> {
        FPZL0_W::new(self)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpzl1(&mut self) -> FPZL1_W<FPV2_SPEC, 17> {
        FPZL1_W::new(self)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpzl2(&mut self) -> FPZL2_W<FPV2_SPEC, 18> {
        FPZL2_W::new(self)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpzl3(&mut self) -> FPZL3_W<FPV2_SPEC, 19> {
        FPZL3_W::new(self)
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
#[doc = "PWM Fault Protection Value 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpv2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpv2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPV2_SPEC;
impl crate::RegisterSpec for FPV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpv2::R`](R) reader structure"]
impl crate::Readable for FPV2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpv2::W`](W) writer structure"]
impl crate::Writable for FPV2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPV2 to value 0"]
impl crate::Resettable for FPV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
