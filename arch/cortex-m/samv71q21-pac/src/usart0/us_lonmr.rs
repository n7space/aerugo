#[doc = "Register `US_LONMR` reader"]
pub type R = crate::R<US_LONMR_SPEC>;
#[doc = "Register `US_LONMR` writer"]
pub type W = crate::W<US_LONMR_SPEC>;
#[doc = "Field `COMMT` reader - LON comm_type Parameter Value"]
pub type COMMT_R = crate::BitReader;
#[doc = "Field `COMMT` writer - LON comm_type Parameter Value"]
pub type COMMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COLDET` reader - LON Collision Detection Feature"]
pub type COLDET_R = crate::BitReader;
#[doc = "Field `COLDET` writer - LON Collision Detection Feature"]
pub type COLDET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCOL` reader - Terminate Frame upon Collision Notification"]
pub type TCOL_R = crate::BitReader;
#[doc = "Field `TCOL` writer - Terminate Frame upon Collision Notification"]
pub type TCOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDTAIL` reader - LON Collision Detection on Frame Tail"]
pub type CDTAIL_R = crate::BitReader;
#[doc = "Field `CDTAIL` writer - LON Collision Detection on Frame Tail"]
pub type CDTAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAM` reader - LON DMA Mode"]
pub type DMAM_R = crate::BitReader;
#[doc = "Field `DMAM` writer - LON DMA Mode"]
pub type DMAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCDS` reader - LON Collision Detection Source"]
pub type LCDS_R = crate::BitReader;
#[doc = "Field `LCDS` writer - LON Collision Detection Source"]
pub type LCDS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOFS` reader - End of Frame Condition Size"]
pub type EOFS_R = crate::FieldReader;
#[doc = "Field `EOFS` writer - End of Frame Condition Size"]
pub type EOFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - LON comm_type Parameter Value"]
    #[inline(always)]
    pub fn commt(&self) -> COMMT_R {
        COMMT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LON Collision Detection Feature"]
    #[inline(always)]
    pub fn coldet(&self) -> COLDET_R {
        COLDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Terminate Frame upon Collision Notification"]
    #[inline(always)]
    pub fn tcol(&self) -> TCOL_R {
        TCOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LON Collision Detection on Frame Tail"]
    #[inline(always)]
    pub fn cdtail(&self) -> CDTAIL_R {
        CDTAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LON DMA Mode"]
    #[inline(always)]
    pub fn dmam(&self) -> DMAM_R {
        DMAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LON Collision Detection Source"]
    #[inline(always)]
    pub fn lcds(&self) -> LCDS_R {
        LCDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:23 - End of Frame Condition Size"]
    #[inline(always)]
    pub fn eofs(&self) -> EOFS_R {
        EOFS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LON comm_type Parameter Value"]
    #[inline(always)]
    #[must_use]
    pub fn commt(&mut self) -> COMMT_W<US_LONMR_SPEC, 0> {
        COMMT_W::new(self)
    }
    #[doc = "Bit 1 - LON Collision Detection Feature"]
    #[inline(always)]
    #[must_use]
    pub fn coldet(&mut self) -> COLDET_W<US_LONMR_SPEC, 1> {
        COLDET_W::new(self)
    }
    #[doc = "Bit 2 - Terminate Frame upon Collision Notification"]
    #[inline(always)]
    #[must_use]
    pub fn tcol(&mut self) -> TCOL_W<US_LONMR_SPEC, 2> {
        TCOL_W::new(self)
    }
    #[doc = "Bit 3 - LON Collision Detection on Frame Tail"]
    #[inline(always)]
    #[must_use]
    pub fn cdtail(&mut self) -> CDTAIL_W<US_LONMR_SPEC, 3> {
        CDTAIL_W::new(self)
    }
    #[doc = "Bit 4 - LON DMA Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmam(&mut self) -> DMAM_W<US_LONMR_SPEC, 4> {
        DMAM_W::new(self)
    }
    #[doc = "Bit 5 - LON Collision Detection Source"]
    #[inline(always)]
    #[must_use]
    pub fn lcds(&mut self) -> LCDS_W<US_LONMR_SPEC, 5> {
        LCDS_W::new(self)
    }
    #[doc = "Bits 16:23 - End of Frame Condition Size"]
    #[inline(always)]
    #[must_use]
    pub fn eofs(&mut self) -> EOFS_W<US_LONMR_SPEC, 16> {
        EOFS_W::new(self)
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
#[doc = "LON Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_lonmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_lonmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LONMR_SPEC;
impl crate::RegisterSpec for US_LONMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonmr::R`](R) reader structure"]
impl crate::Readable for US_LONMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_lonmr::W`](W) writer structure"]
impl crate::Writable for US_LONMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONMR to value 0"]
impl crate::Resettable for US_LONMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
