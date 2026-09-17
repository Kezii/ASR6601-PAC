#[doc = "Register `CLEAR_TFR` writer"]
pub type W = crate::W<ClearTfrSpec>;
#[doc = "Field `CHAN0_CLEAR` writer - Clear DMA channel 0 transfer completion status"]
pub type Chan0ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN1_CLEAR` writer - Clear DMA channel 1 transfer completion status"]
pub type Chan1ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN2_CLEAR` writer - Clear DMA channel 2 transfer completion status"]
pub type Chan2ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN3_CLEAR` writer - Clear DMA channel 3 transfer completion status"]
pub type Chan3ClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear DMA channel 0 transfer completion status"]
    #[inline(always)]
    pub fn chan0_clear(&mut self) -> Chan0ClearW<'_, ClearTfrSpec> {
        Chan0ClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear DMA channel 1 transfer completion status"]
    #[inline(always)]
    pub fn chan1_clear(&mut self) -> Chan1ClearW<'_, ClearTfrSpec> {
        Chan1ClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear DMA channel 2 transfer completion status"]
    #[inline(always)]
    pub fn chan2_clear(&mut self) -> Chan2ClearW<'_, ClearTfrSpec> {
        Chan2ClearW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear DMA channel 3 transfer completion status"]
    #[inline(always)]
    pub fn chan3_clear(&mut self) -> Chan3ClearW<'_, ClearTfrSpec> {
        Chan3ClearW::new(self, 3)
    }
}
#[doc = "DMA transfer completion status clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_tfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearTfrSpec;
impl crate::RegisterSpec for ClearTfrSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`clear_tfr::W`](W) writer structure"]
impl crate::Writable for ClearTfrSpec {
    type Safety = crate::Unsafe;
}
