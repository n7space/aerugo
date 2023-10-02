#[doc = "Register `OS` reader"]
pub type R = crate::R<OS_SPEC>;
#[doc = "Register `OS` writer"]
pub type W = crate::W<OS_SPEC>;
#[doc = "Field `OSH0` reader - Output Selection for PWMH output of the channel 0"]
pub type OSH0_R = crate::BitReader;
#[doc = "Field `OSH0` writer - Output Selection for PWMH output of the channel 0"]
pub type OSH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH1` reader - Output Selection for PWMH output of the channel 1"]
pub type OSH1_R = crate::BitReader;
#[doc = "Field `OSH1` writer - Output Selection for PWMH output of the channel 1"]
pub type OSH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH2` reader - Output Selection for PWMH output of the channel 2"]
pub type OSH2_R = crate::BitReader;
#[doc = "Field `OSH2` writer - Output Selection for PWMH output of the channel 2"]
pub type OSH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH3` reader - Output Selection for PWMH output of the channel 3"]
pub type OSH3_R = crate::BitReader;
#[doc = "Field `OSH3` writer - Output Selection for PWMH output of the channel 3"]
pub type OSH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL0` reader - Output Selection for PWML output of the channel 0"]
pub type OSL0_R = crate::BitReader;
#[doc = "Field `OSL0` writer - Output Selection for PWML output of the channel 0"]
pub type OSL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL1` reader - Output Selection for PWML output of the channel 1"]
pub type OSL1_R = crate::BitReader;
#[doc = "Field `OSL1` writer - Output Selection for PWML output of the channel 1"]
pub type OSL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL2` reader - Output Selection for PWML output of the channel 2"]
pub type OSL2_R = crate::BitReader;
#[doc = "Field `OSL2` writer - Output Selection for PWML output of the channel 2"]
pub type OSL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL3` reader - Output Selection for PWML output of the channel 3"]
pub type OSL3_R = crate::BitReader;
#[doc = "Field `OSL3` writer - Output Selection for PWML output of the channel 3"]
pub type OSL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn osh0(&self) -> OSH0_R {
        OSH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn osh1(&self) -> OSH1_R {
        OSH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn osh2(&self) -> OSH2_R {
        OSH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn osh3(&self) -> OSH3_R {
        OSH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    pub fn osl0(&self) -> OSL0_R {
        OSL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    pub fn osl1(&self) -> OSL1_R {
        OSL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    pub fn osl2(&self) -> OSL2_R {
        OSL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    pub fn osl3(&self) -> OSL3_R {
        OSL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osh0(&mut self) -> OSH0_W<OS_SPEC, 0> {
        OSH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osh1(&mut self) -> OSH1_W<OS_SPEC, 1> {
        OSH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn osh2(&mut self) -> OSH2_W<OS_SPEC, 2> {
        OSH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn osh3(&mut self) -> OSH3_W<OS_SPEC, 3> {
        OSH3_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osl0(&mut self) -> OSL0_W<OS_SPEC, 16> {
        OSL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osl1(&mut self) -> OSL1_W<OS_SPEC, 17> {
        OSL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn osl2(&mut self) -> OSL2_W<OS_SPEC, 18> {
        OSL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn osl3(&mut self) -> OSL3_W<OS_SPEC, 19> {
        OSL3_W::new(self)
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
#[doc = "PWM Output Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OS_SPEC;
impl crate::RegisterSpec for OS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`os::R`](R) reader structure"]
impl crate::Readable for OS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`os::W`](W) writer structure"]
impl crate::Writable for OS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OS to value 0"]
impl crate::Resettable for OS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
