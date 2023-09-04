#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `WDV` reader - Watchdog Counter Value"]
pub type WDV_R = crate::FieldReader<u16>;
#[doc = "Field `WDV` writer - Watchdog Counter Value"]
pub type WDV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `WDFIEN` reader - Watchdog Fault Interrupt Enable"]
pub type WDFIEN_R = crate::BitReader;
#[doc = "Field `WDFIEN` writer - Watchdog Fault Interrupt Enable"]
pub type WDFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDRSTEN` reader - Watchdog Reset Enable"]
pub type WDRSTEN_R = crate::BitReader;
#[doc = "Field `WDRSTEN` writer - Watchdog Reset Enable"]
pub type WDRSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDDIS` reader - Watchdog Disable"]
pub type WDDIS_R = crate::BitReader;
#[doc = "Field `WDDIS` writer - Watchdog Disable"]
pub type WDDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDD` reader - Watchdog Delta Value"]
pub type WDD_R = crate::FieldReader<u16>;
#[doc = "Field `WDD` writer - Watchdog Delta Value"]
pub type WDD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `WDDBGHLT` reader - Watchdog Debug Halt"]
pub type WDDBGHLT_R = crate::BitReader;
#[doc = "Field `WDDBGHLT` writer - Watchdog Debug Halt"]
pub type WDDBGHLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDIDLEHLT` reader - Watchdog Idle Halt"]
pub type WDIDLEHLT_R = crate::BitReader;
#[doc = "Field `WDIDLEHLT` writer - Watchdog Idle Halt"]
pub type WDIDLEHLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Watchdog Counter Value"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Watchdog Fault Interrupt Enable"]
    #[inline(always)]
    pub fn wdfien(&self) -> WDFIEN_R {
        WDFIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdrsten(&self) -> WDRSTEN_R {
        WDRSTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Watchdog Disable"]
    #[inline(always)]
    pub fn wddis(&self) -> WDDIS_R {
        WDDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Watchdog Delta Value"]
    #[inline(always)]
    pub fn wdd(&self) -> WDD_R {
        WDD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Watchdog Debug Halt"]
    #[inline(always)]
    pub fn wddbghlt(&self) -> WDDBGHLT_R {
        WDDBGHLT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Watchdog Idle Halt"]
    #[inline(always)]
    pub fn wdidlehlt(&self) -> WDIDLEHLT_R {
        WDIDLEHLT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdv(&mut self) -> WDV_W<MR_SPEC, 0> {
        WDV_W::new(self)
    }
    #[doc = "Bit 12 - Watchdog Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdfien(&mut self) -> WDFIEN_W<MR_SPEC, 12> {
        WDFIEN_W::new(self)
    }
    #[doc = "Bit 13 - Watchdog Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdrsten(&mut self) -> WDRSTEN_W<MR_SPEC, 13> {
        WDRSTEN_W::new(self)
    }
    #[doc = "Bit 15 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wddis(&mut self) -> WDDIS_W<MR_SPEC, 15> {
        WDDIS_W::new(self)
    }
    #[doc = "Bits 16:27 - Watchdog Delta Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdd(&mut self) -> WDD_W<MR_SPEC, 16> {
        WDD_W::new(self)
    }
    #[doc = "Bit 28 - Watchdog Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn wddbghlt(&mut self) -> WDDBGHLT_W<MR_SPEC, 28> {
        WDDBGHLT_W::new(self)
    }
    #[doc = "Bit 29 - Watchdog Idle Halt"]
    #[inline(always)]
    #[must_use]
    pub fn wdidlehlt(&mut self) -> WDIDLEHLT_W<MR_SPEC, 29> {
        WDIDLEHLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
