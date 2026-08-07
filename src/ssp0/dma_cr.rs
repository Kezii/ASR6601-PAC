#[doc = "Register `DMA_CR` reader"]
pub type R = crate::R<DmaCrSpec>;
#[doc = "Register `DMA_CR` writer"]
pub type W = crate::W<DmaCrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCrSpec;
impl crate::RegisterSpec for DmaCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cr::R`](R) reader structure"]
impl crate::Readable for DmaCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_cr::W`](W) writer structure"]
impl crate::Writable for DmaCrSpec {
    type Safety = crate::Unsafe;
}
