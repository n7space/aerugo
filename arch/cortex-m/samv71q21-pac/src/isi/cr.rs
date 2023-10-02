#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `ISI_EN` writer - ISI Module Enable Request"]
pub type ISI_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISI_DIS` writer - ISI Module Disable Request"]
pub type ISI_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISI_SRST` writer - ISI Software Reset Request"]
pub type ISI_SRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISI_CDC` writer - ISI Codec Request"]
pub type ISI_CDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - ISI Module Enable Request"]
    #[inline(always)]
    #[must_use]
    pub fn isi_en(&mut self) -> ISI_EN_W<CR_SPEC, 0> {
        ISI_EN_W::new(self)
    }
    #[doc = "Bit 1 - ISI Module Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn isi_dis(&mut self) -> ISI_DIS_W<CR_SPEC, 1> {
        ISI_DIS_W::new(self)
    }
    #[doc = "Bit 2 - ISI Software Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn isi_srst(&mut self) -> ISI_SRST_W<CR_SPEC, 2> {
        ISI_SRST_W::new(self)
    }
    #[doc = "Bit 8 - ISI Codec Request"]
    #[inline(always)]
    #[must_use]
    pub fn isi_cdc(&mut self) -> ISI_CDC_W<CR_SPEC, 8> {
        ISI_CDC_W::new(self)
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
#[doc = "ISI Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
