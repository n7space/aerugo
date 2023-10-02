#[doc = "Register `ACTLR` reader"]
pub type R = crate::R<ACTLR_SPEC>;
#[doc = "Register `ACTLR` writer"]
pub type W = crate::W<ACTLR_SPEC>;
#[doc = "Field `DISFOLD` reader - Disables folding of IT instructions"]
pub type DISFOLD_R = crate::BitReader;
#[doc = "Field `DISFOLD` writer - Disables folding of IT instructions"]
pub type DISFOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPEXCODIS` reader - Disables FPU exception outputs"]
pub type FPEXCODIS_R = crate::BitReader;
#[doc = "Field `FPEXCODIS` writer - Disables FPU exception outputs"]
pub type FPEXCODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISRAMODE` reader - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
pub type DISRAMODE_R = crate::BitReader;
#[doc = "Field `DISRAMODE` writer - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
pub type DISRAMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISITMATBFLUSH` reader - Disables ITM and DWT ATB flush"]
pub type DISITMATBFLUSH_R = crate::BitReader;
#[doc = "Field `DISITMATBFLUSH` writer - Disables ITM and DWT ATB flush"]
pub type DISITMATBFLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISBTACREAD` reader - "]
pub type DISBTACREAD_R = crate::BitReader;
#[doc = "Field `DISBTACREAD` writer - "]
pub type DISBTACREAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISBTACALLOC` reader - "]
pub type DISBTACALLOC_R = crate::BitReader;
#[doc = "Field `DISBTACALLOC` writer - "]
pub type DISBTACALLOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISCRITAXIRUR` reader - "]
pub type DISCRITAXIRUR_R = crate::BitReader;
#[doc = "Field `DISCRITAXIRUR` writer - "]
pub type DISCRITAXIRUR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISDI` reader - "]
pub type DISDI_R = crate::FieldReader;
#[doc = "Field `DISDI` writer - "]
pub type DISDI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DISISSCH1` reader - "]
pub type DISISSCH1_R = crate::FieldReader;
#[doc = "Field `DISISSCH1` writer - "]
pub type DISISSCH1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DISDYNADD` reader - Disables dynamic allocation of ADD and SUB instructions"]
pub type DISDYNADD_R = crate::BitReader;
#[doc = "Field `DISDYNADD` writer - Disables dynamic allocation of ADD and SUB instructions"]
pub type DISDYNADD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISCRITAXIRUW` reader - Disable critical AXI read-under-write"]
pub type DISCRITAXIRUW_R = crate::BitReader;
#[doc = "Field `DISCRITAXIRUW` writer - Disable critical AXI read-under-write"]
pub type DISCRITAXIRUW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISFPUISSOPT` reader - Disables dynamic allocation of ADD and SUB instructions"]
pub type DISFPUISSOPT_R = crate::BitReader;
#[doc = "Field `DISFPUISSOPT` writer - Disables dynamic allocation of ADD and SUB instructions"]
pub type DISFPUISSOPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Disables folding of IT instructions"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Disables FPU exception outputs"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn disbtacread(&self) -> DISBTACREAD_R {
        DISBTACREAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn disbtacalloc(&self) -> DISBTACALLOC_R {
        DISBTACALLOC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn discritaxirur(&self) -> DISCRITAXIRUR_R {
        DISCRITAXIRUR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn disdi(&self) -> DISDI_R {
        DISDI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25"]
    #[inline(always)]
    pub fn disissch1(&self) -> DISISSCH1_R {
        DISISSCH1_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disdynadd(&self) -> DISDYNADD_R {
        DISDYNADD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Disable critical AXI read-under-write"]
    #[inline(always)]
    pub fn discritaxiruw(&self) -> DISCRITAXIRUW_R {
        DISCRITAXIRUW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disfpuissopt(&self) -> DISFPUISSOPT_R {
        DISFPUISSOPT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Disables folding of IT instructions"]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<ACTLR_SPEC, 2> {
        DISFOLD_W::new(self)
    }
    #[doc = "Bit 10 - Disables FPU exception outputs"]
    #[inline(always)]
    #[must_use]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W<ACTLR_SPEC, 10> {
        FPEXCODIS_W::new(self)
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
    #[inline(always)]
    #[must_use]
    pub fn disramode(&mut self) -> DISRAMODE_W<ACTLR_SPEC, 11> {
        DISRAMODE_W::new(self)
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush"]
    #[inline(always)]
    #[must_use]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W<ACTLR_SPEC, 12> {
        DISITMATBFLUSH_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn disbtacread(&mut self) -> DISBTACREAD_W<ACTLR_SPEC, 13> {
        DISBTACREAD_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn disbtacalloc(&mut self) -> DISBTACALLOC_W<ACTLR_SPEC, 14> {
        DISBTACALLOC_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn discritaxirur(&mut self) -> DISCRITAXIRUR_W<ACTLR_SPEC, 15> {
        DISCRITAXIRUR_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn disdi(&mut self) -> DISDI_W<ACTLR_SPEC, 16> {
        DISDI_W::new(self)
    }
    #[doc = "Bits 21:25"]
    #[inline(always)]
    #[must_use]
    pub fn disissch1(&mut self) -> DISISSCH1_W<ACTLR_SPEC, 21> {
        DISISSCH1_W::new(self)
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    #[must_use]
    pub fn disdynadd(&mut self) -> DISDYNADD_W<ACTLR_SPEC, 26> {
        DISDYNADD_W::new(self)
    }
    #[doc = "Bit 27 - Disable critical AXI read-under-write"]
    #[inline(always)]
    #[must_use]
    pub fn discritaxiruw(&mut self) -> DISCRITAXIRUW_W<ACTLR_SPEC, 27> {
        DISCRITAXIRUW_W::new(self)
    }
    #[doc = "Bit 28 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    #[must_use]
    pub fn disfpuissopt(&mut self) -> DISFPUISSOPT_W<ACTLR_SPEC, 28> {
        DISFPUISSOPT_W::new(self)
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
#[doc = "Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actlr::R`](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actlr::W`](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
