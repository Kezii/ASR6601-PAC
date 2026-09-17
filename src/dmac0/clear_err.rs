#[doc = "Register `CLEAR_ERR` writer"]
pub type W = crate::W<ClearErrSpec>;
#[doc = "Field `CHAN0_CLEAR` writer - Clear DMA channel 0 transfer error status"]
pub type Chan0ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN1_CLEAR` writer - Clear DMA channel 1 transfer error status"]
pub type Chan1ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN2_CLEAR` writer - Clear DMA channel 2 transfer error status"]
pub type Chan2ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN3_CLEAR` writer - Clear DMA channel 3 transfer error status"]
pub type Chan3ClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear DMA channel 0 transfer error status"]
    #[inline(always)]
    pub fn chan0_clear(&mut self) -> Chan0ClearW<'_, ClearErrSpec> {
        Chan0ClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear DMA channel 1 transfer error status"]
    #[inline(always)]
    pub fn chan1_clear(&mut self) -> Chan1ClearW<'_, ClearErrSpec> {
        Chan1ClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear DMA channel 2 transfer error status"]
    #[inline(always)]
    pub fn chan2_clear(&mut self) -> Chan2ClearW<'_, ClearErrSpec> {
        Chan2ClearW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear DMA channel 3 transfer error status"]
    #[inline(always)]
    pub fn chan3_clear(&mut self) -> Chan3ClearW<'_, ClearErrSpec> {
        Chan3ClearW::new(self, 3)
    }
}
#[doc = "DMA transfer error status clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_err::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearErrSpec;
impl crate::RegisterSpec for ClearErrSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`clear_err::W`](W) writer structure"]
impl crate::Writable for ClearErrSpec {
    type Safety = crate::Unsafe;
}
