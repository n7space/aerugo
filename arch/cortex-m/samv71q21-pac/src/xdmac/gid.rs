#[doc = "Register `GID` writer"]
pub type W = crate::W<GID_SPEC>;
#[doc = "Field `ID0` writer - XDMAC Channel 0 Interrupt Disable Bit"]
pub type ID0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID1` writer - XDMAC Channel 1 Interrupt Disable Bit"]
pub type ID1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID2` writer - XDMAC Channel 2 Interrupt Disable Bit"]
pub type ID2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID3` writer - XDMAC Channel 3 Interrupt Disable Bit"]
pub type ID3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID4` writer - XDMAC Channel 4 Interrupt Disable Bit"]
pub type ID4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID5` writer - XDMAC Channel 5 Interrupt Disable Bit"]
pub type ID5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID6` writer - XDMAC Channel 6 Interrupt Disable Bit"]
pub type ID6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID7` writer - XDMAC Channel 7 Interrupt Disable Bit"]
pub type ID7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID8` writer - XDMAC Channel 8 Interrupt Disable Bit"]
pub type ID8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID9` writer - XDMAC Channel 9 Interrupt Disable Bit"]
pub type ID9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID10` writer - XDMAC Channel 10 Interrupt Disable Bit"]
pub type ID10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID11` writer - XDMAC Channel 11 Interrupt Disable Bit"]
pub type ID11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID12` writer - XDMAC Channel 12 Interrupt Disable Bit"]
pub type ID12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID13` writer - XDMAC Channel 13 Interrupt Disable Bit"]
pub type ID13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID14` writer - XDMAC Channel 14 Interrupt Disable Bit"]
pub type ID14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID15` writer - XDMAC Channel 15 Interrupt Disable Bit"]
pub type ID15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID16` writer - XDMAC Channel 16 Interrupt Disable Bit"]
pub type ID16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID17` writer - XDMAC Channel 17 Interrupt Disable Bit"]
pub type ID17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID18` writer - XDMAC Channel 18 Interrupt Disable Bit"]
pub type ID18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID19` writer - XDMAC Channel 19 Interrupt Disable Bit"]
pub type ID19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID20` writer - XDMAC Channel 20 Interrupt Disable Bit"]
pub type ID20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID21` writer - XDMAC Channel 21 Interrupt Disable Bit"]
pub type ID21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID22` writer - XDMAC Channel 22 Interrupt Disable Bit"]
pub type ID22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ID23` writer - XDMAC Channel 23 Interrupt Disable Bit"]
pub type ID23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id0(&mut self) -> ID0_W<GID_SPEC, 0> {
        ID0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id1(&mut self) -> ID1_W<GID_SPEC, 1> {
        ID1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id2(&mut self) -> ID2_W<GID_SPEC, 2> {
        ID2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id3(&mut self) -> ID3_W<GID_SPEC, 3> {
        ID3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id4(&mut self) -> ID4_W<GID_SPEC, 4> {
        ID4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id5(&mut self) -> ID5_W<GID_SPEC, 5> {
        ID5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id6(&mut self) -> ID6_W<GID_SPEC, 6> {
        ID6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id7(&mut self) -> ID7_W<GID_SPEC, 7> {
        ID7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id8(&mut self) -> ID8_W<GID_SPEC, 8> {
        ID8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id9(&mut self) -> ID9_W<GID_SPEC, 9> {
        ID9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id10(&mut self) -> ID10_W<GID_SPEC, 10> {
        ID10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id11(&mut self) -> ID11_W<GID_SPEC, 11> {
        ID11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id12(&mut self) -> ID12_W<GID_SPEC, 12> {
        ID12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id13(&mut self) -> ID13_W<GID_SPEC, 13> {
        ID13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id14(&mut self) -> ID14_W<GID_SPEC, 14> {
        ID14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id15(&mut self) -> ID15_W<GID_SPEC, 15> {
        ID15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id16(&mut self) -> ID16_W<GID_SPEC, 16> {
        ID16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id17(&mut self) -> ID17_W<GID_SPEC, 17> {
        ID17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id18(&mut self) -> ID18_W<GID_SPEC, 18> {
        ID18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id19(&mut self) -> ID19_W<GID_SPEC, 19> {
        ID19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id20(&mut self) -> ID20_W<GID_SPEC, 20> {
        ID20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id21(&mut self) -> ID21_W<GID_SPEC, 21> {
        ID21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id22(&mut self) -> ID22_W<GID_SPEC, 22> {
        ID22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn id23(&mut self) -> ID23_W<GID_SPEC, 23> {
        ID23_W::new(self)
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
#[doc = "Global Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gid::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GID_SPEC;
impl crate::RegisterSpec for GID_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gid::W`](W) writer structure"]
impl crate::Writable for GID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GID to value 0"]
impl crate::Resettable for GID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
