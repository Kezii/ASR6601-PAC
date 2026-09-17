#[doc = "Register `CLEAR_DST_TRAN` writer"]
pub type W = crate::W<ClearDstTranSpec>;
#[doc = "Field `CHAN0_CLEAR` writer - Clear DMA channel 0 destination transfer completion status"]
pub type Chan0ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN1_CLEAR` writer - Clear DMA channel 1 destination transfer completion status"]
pub type Chan1ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN2_CLEAR` writer - Clear DMA channel 2 destination transfer completion status"]
pub type Chan2ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN3_CLEAR` writer - Clear DMA channel 3 destination transfer completion status"]
pub type Chan3ClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear DMA channel 0 destination transfer completion status"]
    #[inline(always)]
    pub fn chan0_clear(&mut self) -> Chan0ClearW<'_, ClearDstTranSpec> {
        Chan0ClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear DMA channel 1 destination transfer completion status"]
    #[inline(always)]
    pub fn chan1_clear(&mut self) -> Chan1ClearW<'_, ClearDstTranSpec> {
        Chan1ClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear DMA channel 2 destination transfer completion status"]
    #[inline(always)]
    pub fn chan2_clear(&mut self) -> Chan2ClearW<'_, ClearDstTranSpec> {
        Chan2ClearW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear DMA channel 3 destination transfer completion status"]
    #[inline(always)]
    pub fn chan3_clear(&mut self) -> Chan3ClearW<'_, ClearDstTranSpec> {
        Chan3ClearW::new(self, 3)
    }
}
#[doc = "DMA destination transfer completion status clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_dst_tran::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearDstTranSpec;
impl crate::RegisterSpec for ClearDstTranSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`clear_dst_tran::W`](W) writer structure"]
impl crate::Writable for ClearDstTranSpec {
    type Safety = crate::Unsafe;
}
