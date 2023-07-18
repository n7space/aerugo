#[doc = "Register `WPCR` writer"]
pub struct W(crate::W<WPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR_SPEC>;
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
impl From<crate::W<WPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write Protection Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WPCMDSELECT_AW {
    #[doc = "0: Disables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    DISABLE_SW_PROT = 0,
    #[doc = "1: Enables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    ENABLE_SW_PROT = 1,
    #[doc = "2: Enables the hardware write protection of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protection. Moreover, to meet security requirements, the PIO lines associated with the PWM can not be configured through the PIO interface."]
    ENABLE_HW_PROT = 2,
}
impl From<WPCMDSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: WPCMDSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WPCMDSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `WPCMD` writer - Write Protection Command"]
pub type WPCMD_W<'a, const O: u8> = crate::FieldWriter<'a, WPCR_SPEC, 2, O, WPCMDSELECT_AW>;
impl<'a, const O: u8> WPCMD_W<'a, O> {
    #[doc = "Disables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn disable_sw_prot(self) -> &'a mut W {
        self.variant(WPCMDSELECT_AW::DISABLE_SW_PROT)
    }
    #[doc = "Enables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn enable_sw_prot(self) -> &'a mut W {
        self.variant(WPCMDSELECT_AW::ENABLE_SW_PROT)
    }
    #[doc = "Enables the hardware write protection of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protection. Moreover, to meet security requirements, the PIO lines associated with the PWM can not be configured through the PIO interface."]
    #[inline(always)]
    pub fn enable_hw_prot(self) -> &'a mut W {
        self.variant(WPCMDSELECT_AW::ENABLE_HW_PROT)
    }
}
#[doc = "Field `WPRG0` writer - Write Protection Register Group 0"]
pub type WPRG0_W<'a, const O: u8> = crate::BitWriter<'a, WPCR_SPEC, O>;
#[doc = "Field `WPRG1` writer - Write Protection Register Group 1"]
pub type WPRG1_W<'a, const O: u8> = crate::BitWriter<'a, WPCR_SPEC, O>;
#[doc = "Field `WPRG2` writer - Write Protection Register Group 2"]
pub type WPRG2_W<'a, const O: u8> = crate::BitWriter<'a, WPCR_SPEC, O>;
#[doc = "Field `WPRG3` writer - Write Protection Register Group 3"]
pub type WPRG3_W<'a, const O: u8> = crate::BitWriter<'a, WPCR_SPEC, O>;
#[doc = "Field `WPRG4` writer - Write Protection Register Group 4"]
pub type WPRG4_W<'a, const O: u8> = crate::BitWriter<'a, WPCR_SPEC, O>;
#[doc = "Field `WPRG5` writer - Write Protection Register Group 5"]
pub type WPRG5_W<'a, const O: u8> = crate::BitWriter<'a, WPCR_SPEC, O>;
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WPKEYSELECT_AW {
    #[doc = "5265229: Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    PASSWD = 5265229,
}
impl From<WPKEYSELECT_AW> for u32 {
    #[inline(always)]
    fn from(variant: WPKEYSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WPKEYSELECT_AW {
    type Ux = u32;
}
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WPKEY_W<'a, const O: u8> = crate::FieldWriter<'a, WPCR_SPEC, 24, O, WPKEYSELECT_AW>;
impl<'a, const O: u8> WPKEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WPKEYSELECT_AW::PASSWD)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write Protection Command"]
    #[inline(always)]
    #[must_use]
    pub fn wpcmd(&mut self) -> WPCMD_W<0> {
        WPCMD_W::new(self)
    }
    #[doc = "Bit 2 - Write Protection Register Group 0"]
    #[inline(always)]
    #[must_use]
    pub fn wprg0(&mut self) -> WPRG0_W<2> {
        WPRG0_W::new(self)
    }
    #[doc = "Bit 3 - Write Protection Register Group 1"]
    #[inline(always)]
    #[must_use]
    pub fn wprg1(&mut self) -> WPRG1_W<3> {
        WPRG1_W::new(self)
    }
    #[doc = "Bit 4 - Write Protection Register Group 2"]
    #[inline(always)]
    #[must_use]
    pub fn wprg2(&mut self) -> WPRG2_W<4> {
        WPRG2_W::new(self)
    }
    #[doc = "Bit 5 - Write Protection Register Group 3"]
    #[inline(always)]
    #[must_use]
    pub fn wprg3(&mut self) -> WPRG3_W<5> {
        WPRG3_W::new(self)
    }
    #[doc = "Bit 6 - Write Protection Register Group 4"]
    #[inline(always)]
    #[must_use]
    pub fn wprg4(&mut self) -> WPRG4_W<6> {
        WPRG4_W::new(self)
    }
    #[doc = "Bit 7 - Write Protection Register Group 5"]
    #[inline(always)]
    #[must_use]
    pub fn wprg5(&mut self) -> WPRG5_W<7> {
        WPRG5_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<8> {
        WPKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Write Protection Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr](index.html) module"]
pub struct WPCR_SPEC;
impl crate::RegisterSpec for WPCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wpcr::W](W) writer structure"]
impl crate::Writable for WPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPCR to value 0"]
impl crate::Resettable for WPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
