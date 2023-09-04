#[doc = "Register `CCFG_SYSIO` reader"]
pub type R = crate::R<CCFG_SYSIO_SPEC>;
#[doc = "Register `CCFG_SYSIO` writer"]
pub type W = crate::W<CCFG_SYSIO_SPEC>;
#[doc = "Field `SYSIO4` reader - PB4 or TDI Assignment"]
pub type SYSIO4_R = crate::BitReader;
#[doc = "Field `SYSIO4` writer - PB4 or TDI Assignment"]
pub type SYSIO4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSIO5` reader - PB5 or TDO/TRACESWO Assignment"]
pub type SYSIO5_R = crate::BitReader;
#[doc = "Field `SYSIO5` writer - PB5 or TDO/TRACESWO Assignment"]
pub type SYSIO5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSIO6` reader - PB6 or TMS/SWDIO Assignment"]
pub type SYSIO6_R = crate::BitReader;
#[doc = "Field `SYSIO6` writer - PB6 or TMS/SWDIO Assignment"]
pub type SYSIO6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSIO7` reader - PB7 or TCK/SWCLK Assignment"]
pub type SYSIO7_R = crate::BitReader;
#[doc = "Field `SYSIO7` writer - PB7 or TCK/SWCLK Assignment"]
pub type SYSIO7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSIO12` reader - PB12 or ERASE Assignment"]
pub type SYSIO12_R = crate::BitReader;
#[doc = "Field `SYSIO12` writer - PB12 or ERASE Assignment"]
pub type SYSIO12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1DMABA` reader - CAN1 DMA Base Address"]
pub type CAN1DMABA_R = crate::FieldReader<u16>;
#[doc = "Field `CAN1DMABA` writer - CAN1 DMA Base Address"]
pub type CAN1DMABA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&self) -> SYSIO4_R {
        SYSIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&self) -> SYSIO5_R {
        SYSIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&self) -> SYSIO6_R {
        SYSIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&self) -> SYSIO7_R {
        SYSIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> SYSIO12_R {
        SYSIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - CAN1 DMA Base Address"]
    #[inline(always)]
    pub fn can1dmaba(&self) -> CAN1DMABA_R {
        CAN1DMABA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio4(&mut self) -> SYSIO4_W<CCFG_SYSIO_SPEC, 4> {
        SYSIO4_W::new(self)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio5(&mut self) -> SYSIO5_W<CCFG_SYSIO_SPEC, 5> {
        SYSIO5_W::new(self)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio6(&mut self) -> SYSIO6_W<CCFG_SYSIO_SPEC, 6> {
        SYSIO6_W::new(self)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio7(&mut self) -> SYSIO7_W<CCFG_SYSIO_SPEC, 7> {
        SYSIO7_W::new(self)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio12(&mut self) -> SYSIO12_W<CCFG_SYSIO_SPEC, 12> {
        SYSIO12_W::new(self)
    }
    #[doc = "Bits 16:31 - CAN1 DMA Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn can1dmaba(&mut self) -> CAN1DMABA_W<CCFG_SYSIO_SPEC, 16> {
        CAN1DMABA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System I/O and CAN1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_sysio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_sysio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFG_SYSIO_SPEC;
impl crate::RegisterSpec for CCFG_SYSIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_sysio::R`](R) reader structure"]
impl crate::Readable for CCFG_SYSIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccfg_sysio::W`](W) writer structure"]
impl crate::Writable for CCFG_SYSIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_SYSIO to value 0"]
impl crate::Resettable for CCFG_SYSIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
