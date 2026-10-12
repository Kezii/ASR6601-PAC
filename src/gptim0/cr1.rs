#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIS` reader - Update events disable"]
pub type UdisR = crate::BitReader;
#[doc = "Field `UDIS` writer - Update events disable"]
pub type UdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - Update event source selection"]
pub type UrsR = crate::BitReader;
#[doc = "Field `URS` writer - Update event source selection"]
pub type UrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM` reader - Single pulse mode enable"]
pub type OpmR = crate::BitReader;
#[doc = "Field `OPM` writer - Single pulse mode enable"]
pub type OpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Counting direction selection"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Counting direction selection"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Center alignment mode selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cms {
    #[doc = "0: Edge-aligned, DIR controls direction"]
    EdgeAligned = 0,
    #[doc = "1: Center alignment mode 1"]
    Center1 = 1,
    #[doc = "2: Center alignment mode 2"]
    Center2 = 2,
    #[doc = "3: Center alignment mode 3"]
    Center3 = 3,
}
impl From<Cms> for u8 {
    #[inline(always)]
    fn from(variant: Cms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cms {
    type Ux = u8;
}
impl crate::IsEnum for Cms {}
#[doc = "Field `CMS` reader - Center alignment mode selection"]
pub type CmsR = crate::FieldReader<Cms>;
impl CmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cms {
        match self.bits {
            0 => Cms::EdgeAligned,
            1 => Cms::Center1,
            2 => Cms::Center2,
            3 => Cms::Center3,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge-aligned, DIR controls direction"]
    #[inline(always)]
    pub fn is_edge_aligned(&self) -> bool {
        *self == Cms::EdgeAligned
    }
    #[doc = "Center alignment mode 1"]
    #[inline(always)]
    pub fn is_center1(&self) -> bool {
        *self == Cms::Center1
    }
    #[doc = "Center alignment mode 2"]
    #[inline(always)]
    pub fn is_center2(&self) -> bool {
        *self == Cms::Center2
    }
    #[doc = "Center alignment mode 3"]
    #[inline(always)]
    pub fn is_center3(&self) -> bool {
        *self == Cms::Center3
    }
}
#[doc = "Field `CMS` writer - Center alignment mode selection"]
pub type CmsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cms, crate::Safe>;
impl<'a, REG> CmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge-aligned, DIR controls direction"]
    #[inline(always)]
    pub fn edge_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(Cms::EdgeAligned)
    }
    #[doc = "Center alignment mode 1"]
    #[inline(always)]
    pub fn center1(self) -> &'a mut crate::W<REG> {
        self.variant(Cms::Center1)
    }
    #[doc = "Center alignment mode 2"]
    #[inline(always)]
    pub fn center2(self) -> &'a mut crate::W<REG> {
        self.variant(Cms::Center2)
    }
    #[doc = "Center alignment mode 3"]
    #[inline(always)]
    pub fn center3(self) -> &'a mut crate::W<REG> {
        self.variant(Cms::Center3)
    }
}
#[doc = "Field `ARPE` reader - Reload shadow register enable"]
pub type ArpeR = crate::BitReader;
#[doc = "Field `ARPE` writer - Reload shadow register enable"]
pub type ArpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sample clock divider"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckd {
    #[doc = "0: fDTS equals fpclk"]
    Div1 = 0,
    #[doc = "1: fDTS equals fpclk divided by 2"]
    Div2 = 1,
    #[doc = "2: fDTS equals fpclk divided by 4"]
    Div4 = 2,
}
impl From<Ckd> for u8 {
    #[inline(always)]
    fn from(variant: Ckd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckd {
    type Ux = u8;
}
impl crate::IsEnum for Ckd {}
#[doc = "Field `CKD` reader - Sample clock divider"]
pub type CkdR = crate::FieldReader<Ckd>;
impl CkdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckd> {
        match self.bits {
            0 => Some(Ckd::Div1),
            1 => Some(Ckd::Div2),
            2 => Some(Ckd::Div4),
            _ => None,
        }
    }
    #[doc = "fDTS equals fpclk"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ckd::Div1
    }
    #[doc = "fDTS equals fpclk divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ckd::Div2
    }
    #[doc = "fDTS equals fpclk divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ckd::Div4
    }
}
#[doc = "Field `CKD` writer - Sample clock divider"]
pub type CkdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckd>;
impl<'a, REG> CkdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fDTS equals fpclk"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckd::Div1)
    }
    #[doc = "fDTS equals fpclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckd::Div2)
    }
    #[doc = "fDTS equals fpclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ckd::Div4)
    }
}
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update events disable"]
    #[inline(always)]
    pub fn udis(&self) -> UdisR {
        UdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update event source selection"]
    #[inline(always)]
    pub fn urs(&self) -> UrsR {
        UrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single pulse mode enable"]
    #[inline(always)]
    pub fn opm(&self) -> OpmR {
        OpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counting direction selection"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Center alignment mode selection"]
    #[inline(always)]
    pub fn cms(&self) -> CmsR {
        CmsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reload shadow register enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ArpeR {
        ArpeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Sample clock divider"]
    #[inline(always)]
    pub fn ckd(&self) -> CkdR {
        CkdR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, Cr1Spec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - Update events disable"]
    #[inline(always)]
    pub fn udis(&mut self) -> UdisW<'_, Cr1Spec> {
        UdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Update event source selection"]
    #[inline(always)]
    pub fn urs(&mut self) -> UrsW<'_, Cr1Spec> {
        UrsW::new(self, 2)
    }
    #[doc = "Bit 3 - Single pulse mode enable"]
    #[inline(always)]
    pub fn opm(&mut self) -> OpmW<'_, Cr1Spec> {
        OpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Counting direction selection"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, Cr1Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Center alignment mode selection"]
    #[inline(always)]
    pub fn cms(&mut self) -> CmsW<'_, Cr1Spec> {
        CmsW::new(self, 5)
    }
    #[doc = "Bit 7 - Reload shadow register enable"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ArpeW<'_, Cr1Spec> {
        ArpeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Sample clock divider"]
    #[inline(always)]
    pub fn ckd(&mut self) -> CkdW<'_, Cr1Spec> {
        CkdW::new(self, 8)
    }
}
#[doc = "GPTIM control register 1, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
