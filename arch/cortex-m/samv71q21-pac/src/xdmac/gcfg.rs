#[doc = "Register `GCFG` reader"]
pub type R = crate::R<GCFG_SPEC>;
#[doc = "Register `GCFG` writer"]
pub type W = crate::W<GCFG_SPEC>;
#[doc = "Field `CGDISREG` reader - Configuration Registers Clock Gating Disable"]
pub type CGDISREG_R = crate::BitReader;
#[doc = "Field `CGDISREG` writer - Configuration Registers Clock Gating Disable"]
pub type CGDISREG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGDISPIPE` reader - Pipeline Clock Gating Disable"]
pub type CGDISPIPE_R = crate::BitReader;
#[doc = "Field `CGDISPIPE` writer - Pipeline Clock Gating Disable"]
pub type CGDISPIPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGDISFIFO` reader - FIFO Clock Gating Disable"]
pub type CGDISFIFO_R = crate::BitReader;
#[doc = "Field `CGDISFIFO` writer - FIFO Clock Gating Disable"]
pub type CGDISFIFO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGDISIF` reader - Bus Interface Clock Gating Disable"]
pub type CGDISIF_R = crate::BitReader;
#[doc = "Field `CGDISIF` writer - Bus Interface Clock Gating Disable"]
pub type CGDISIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BXKBEN` reader - Boundary X Kilobyte Enable"]
pub type BXKBEN_R = crate::BitReader;
#[doc = "Field `BXKBEN` writer - Boundary X Kilobyte Enable"]
pub type BXKBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisreg(&self) -> CGDISREG_R {
        CGDISREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdispipe(&self) -> CGDISPIPE_R {
        CGDISPIPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisfifo(&self) -> CGDISFIFO_R {
        CGDISFIFO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisif(&self) -> CGDISIF_R {
        CGDISIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    pub fn bxkben(&self) -> BXKBEN_R {
        BXKBEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cgdisreg(&mut self) -> CGDISREG_W<GCFG_SPEC, 0> {
        CGDISREG_W::new(self)
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cgdispipe(&mut self) -> CGDISPIPE_W<GCFG_SPEC, 1> {
        CGDISPIPE_W::new(self)
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cgdisfifo(&mut self) -> CGDISFIFO_W<GCFG_SPEC, 2> {
        CGDISFIFO_W::new(self)
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cgdisif(&mut self) -> CGDISIF_W<GCFG_SPEC, 3> {
        CGDISIF_W::new(self)
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bxkben(&mut self) -> BXKBEN_W<GCFG_SPEC, 8> {
        BXKBEN_W::new(self)
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
#[doc = "Global Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCFG_SPEC;
impl crate::RegisterSpec for GCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcfg::R`](R) reader structure"]
impl crate::Readable for GCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcfg::W`](W) writer structure"]
impl crate::Writable for GCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCFG to value 0"]
impl crate::Resettable for GCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
