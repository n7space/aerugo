#[doc = "Register `HSTCTRL` reader"]
pub type R = crate::R<HSTCTRL_SPEC>;
#[doc = "Register `HSTCTRL` writer"]
pub type W = crate::W<HSTCTRL_SPEC>;
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub type SOFE_R = crate::BitReader;
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub type SOFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESET` reader - Send USB Reset"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Send USB Reset"]
pub type RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub type RESUME_R = crate::BitReader;
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub type RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPDCONF` reader - Mode Configuration"]
pub type SPDCONF_R = crate::FieldReader<SPDCONFSELECT_A>;
#[doc = "Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDCONFSELECT_A {
    #[doc = "0: The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    NORMAL = 0,
    #[doc = "1: For a better consumption, if high speed is not needed."]
    LOW_POWER = 1,
    #[doc = "2: Forced high speed."]
    HIGH_SPEED = 2,
    #[doc = "3: The host remains in Full-speed mode whatever the peripheral speed capability."]
    FORCED_FS = 3,
}
impl From<SPDCONFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONFSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPDCONFSELECT_A {
    type Ux = u8;
}
impl SPDCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDCONFSELECT_A {
        match self.bits {
            0 => SPDCONFSELECT_A::NORMAL,
            1 => SPDCONFSELECT_A::LOW_POWER,
            2 => SPDCONFSELECT_A::HIGH_SPEED,
            3 => SPDCONFSELECT_A::FORCED_FS,
            _ => unreachable!(),
        }
    }
    #[doc = "The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SPDCONFSELECT_A::NORMAL
    }
    #[doc = "For a better consumption, if high speed is not needed."]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == SPDCONFSELECT_A::LOW_POWER
    }
    #[doc = "Forced high speed."]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == SPDCONFSELECT_A::HIGH_SPEED
    }
    #[doc = "The host remains in Full-speed mode whatever the peripheral speed capability."]
    #[inline(always)]
    pub fn is_forced_fs(&self) -> bool {
        *self == SPDCONFSELECT_A::FORCED_FS
    }
}
#[doc = "Field `SPDCONF` writer - Mode Configuration"]
pub type SPDCONF_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SPDCONFSELECT_A>;
impl<'a, REG, const O: u8> SPDCONF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::NORMAL)
    }
    #[doc = "For a better consumption, if high speed is not needed."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::LOW_POWER)
    }
    #[doc = "Forced high speed."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::HIGH_SPEED)
    }
    #[doc = "The host remains in Full-speed mode whatever the peripheral speed capability."]
    #[inline(always)]
    pub fn forced_fs(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::FORCED_FS)
    }
}
impl R {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SOFE_W<HSTCTRL_SPEC, 8> {
        SOFE_W::new(self)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<HSTCTRL_SPEC, 9> {
        RESET_W::new(self)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<HSTCTRL_SPEC, 10> {
        RESUME_W::new(self)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SPDCONF_W<HSTCTRL_SPEC, 12> {
        SPDCONF_W::new(self)
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
#[doc = "Host General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTCTRL_SPEC;
impl crate::RegisterSpec for HSTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstctrl::R`](R) reader structure"]
impl crate::Readable for HSTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstctrl::W`](W) writer structure"]
impl crate::Writable for HSTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTCTRL to value 0"]
impl crate::Resettable for HSTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
