#[doc = "Register `US_LONL2HDR` reader"]
pub type R = crate::R<US_LONL2HDR_SPEC>;
#[doc = "Register `US_LONL2HDR` writer"]
pub type W = crate::W<US_LONL2HDR_SPEC>;
#[doc = "Field `BLI` reader - LON Backlog Increment"]
pub type BLI_R = crate::FieldReader;
#[doc = "Field `BLI` writer - LON Backlog Increment"]
pub type BLI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `ALTP` reader - LON Alternate Path Bit"]
pub type ALTP_R = crate::BitReader;
#[doc = "Field `ALTP` writer - LON Alternate Path Bit"]
pub type ALTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PB` reader - LON Priority Bit"]
pub type PB_R = crate::BitReader;
#[doc = "Field `PB` writer - LON Priority Bit"]
pub type PB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - LON Backlog Increment"]
    #[inline(always)]
    pub fn bli(&self) -> BLI_R {
        BLI_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - LON Alternate Path Bit"]
    #[inline(always)]
    pub fn altp(&self) -> ALTP_R {
        ALTP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LON Priority Bit"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - LON Backlog Increment"]
    #[inline(always)]
    #[must_use]
    pub fn bli(&mut self) -> BLI_W<US_LONL2HDR_SPEC, 0> {
        BLI_W::new(self)
    }
    #[doc = "Bit 6 - LON Alternate Path Bit"]
    #[inline(always)]
    #[must_use]
    pub fn altp(&mut self) -> ALTP_W<US_LONL2HDR_SPEC, 6> {
        ALTP_W::new(self)
    }
    #[doc = "Bit 7 - LON Priority Bit"]
    #[inline(always)]
    #[must_use]
    pub fn pb(&mut self) -> PB_W<US_LONL2HDR_SPEC, 7> {
        PB_W::new(self)
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
#[doc = "LON L2HDR Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_lonl2hdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_lonl2hdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LONL2HDR_SPEC;
impl crate::RegisterSpec for US_LONL2HDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonl2hdr::R`](R) reader structure"]
impl crate::Readable for US_LONL2HDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_lonl2hdr::W`](W) writer structure"]
impl crate::Writable for US_LONL2HDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONL2HDR to value 0"]
impl crate::Resettable for US_LONL2HDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
