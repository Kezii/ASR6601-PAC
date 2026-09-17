#[doc = "Register `CHENREG` reader"]
pub type R = crate::R<ChenregSpec>;
#[doc = "Register `CHENREG` writer"]
pub type W = crate::W<ChenregSpec>;
#[doc = "Field `CH_EN_0` reader - DMA channel 0 enable control"]
pub type ChEn0R = crate::BitReader;
#[doc = "Field `CH_EN_0` writer - DMA channel 0 enable control"]
pub type ChEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_EN_1` reader - DMA channel 1 enable control"]
pub type ChEn1R = crate::BitReader;
#[doc = "Field `CH_EN_1` writer - DMA channel 1 enable control"]
pub type ChEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_EN_2` reader - DMA channel 2 enable control"]
pub type ChEn2R = crate::BitReader;
#[doc = "Field `CH_EN_2` writer - DMA channel 2 enable control"]
pub type ChEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_EN_3` reader - DMA channel 3 enable control"]
pub type ChEn3R = crate::BitReader;
#[doc = "Field `CH_EN_3` writer - DMA channel 3 enable control"]
pub type ChEn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_EN_WE_0` reader - DMA channel 0 enable control information write enable"]
pub type ChEnWe0R = crate::BitReader;
#[doc = "Field `CH_EN_WE_0` writer - DMA channel 0 enable control information write enable"]
pub type ChEnWe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_EN_WE_1` reader - DMA channel 1 enable control information write enable"]
pub type ChEnWe1R = crate::BitReader;
#[doc = "Field `CH_EN_WE_1` writer - DMA channel 1 enable control information write enable"]
pub type ChEnWe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_EN_WE_2` reader - DMA channel 2 enable control information write enable"]
pub type ChEnWe2R = crate::BitReader;
#[doc = "Field `CH_EN_WE_2` writer - DMA channel 2 enable control information write enable"]
pub type ChEnWe2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_EN_WE_3` reader - DMA channel 3 enable control information write enable"]
pub type ChEnWe3R = crate::BitReader;
#[doc = "Field `CH_EN_WE_3` writer - DMA channel 3 enable control information write enable"]
pub type ChEnWe3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA channel 0 enable control"]
    #[inline(always)]
    pub fn ch_en_0(&self) -> ChEn0R {
        ChEn0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA channel 1 enable control"]
    #[inline(always)]
    pub fn ch_en_1(&self) -> ChEn1R {
        ChEn1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA channel 2 enable control"]
    #[inline(always)]
    pub fn ch_en_2(&self) -> ChEn2R {
        ChEn2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA channel 3 enable control"]
    #[inline(always)]
    pub fn ch_en_3(&self) -> ChEn3R {
        ChEn3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA channel 0 enable control information write enable"]
    #[inline(always)]
    pub fn ch_en_we_0(&self) -> ChEnWe0R {
        ChEnWe0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA channel 1 enable control information write enable"]
    #[inline(always)]
    pub fn ch_en_we_1(&self) -> ChEnWe1R {
        ChEnWe1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA channel 2 enable control information write enable"]
    #[inline(always)]
    pub fn ch_en_we_2(&self) -> ChEnWe2R {
        ChEnWe2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA channel 3 enable control information write enable"]
    #[inline(always)]
    pub fn ch_en_we_3(&self) -> ChEnWe3R {
        ChEnWe3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA channel 0 enable control"]
    #[inline(always)]
    pub fn ch_en_0(&mut self) -> ChEn0W<'_, ChenregSpec> {
        ChEn0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA channel 1 enable control"]
    #[inline(always)]
    pub fn ch_en_1(&mut self) -> ChEn1W<'_, ChenregSpec> {
        ChEn1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA channel 2 enable control"]
    #[inline(always)]
    pub fn ch_en_2(&mut self) -> ChEn2W<'_, ChenregSpec> {
        ChEn2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA channel 3 enable control"]
    #[inline(always)]
    pub fn ch_en_3(&mut self) -> ChEn3W<'_, ChenregSpec> {
        ChEn3W::new(self, 3)
    }
    #[doc = "Bit 8 - DMA channel 0 enable control information write enable"]
    #[inline(always)]
    pub fn ch_en_we_0(&mut self) -> ChEnWe0W<'_, ChenregSpec> {
        ChEnWe0W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA channel 1 enable control information write enable"]
    #[inline(always)]
    pub fn ch_en_we_1(&mut self) -> ChEnWe1W<'_, ChenregSpec> {
        ChEnWe1W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA channel 2 enable control information write enable"]
    #[inline(always)]
    pub fn ch_en_we_2(&mut self) -> ChEnWe2W<'_, ChenregSpec> {
        ChEnWe2W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA channel 3 enable control information write enable"]
    #[inline(always)]
    pub fn ch_en_we_3(&mut self) -> ChEnWe3W<'_, ChenregSpec> {
        ChEnWe3W::new(self, 11)
    }
}
#[doc = "DMA channel enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenregSpec;
impl crate::RegisterSpec for ChenregSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`chenreg::R`](R) reader structure"]
impl crate::Readable for ChenregSpec {}
#[doc = "`write(|w| ..)` method takes [`chenreg::W`](W) writer structure"]
impl crate::Writable for ChenregSpec {
    type Safety = crate::Unsafe;
}
