#[doc = "Register `SSP_DMA_CR` reader"]
pub type R = crate::R<SspDmaCrSpec>;
#[doc = "Register `SSP_DMA_CR` writer"]
pub type W = crate::W<SspDmaCrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ssp DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_dma_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_dma_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspDmaCrSpec;
impl crate::RegisterSpec for SspDmaCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_dma_cr::R`](R) reader structure"]
impl crate::Readable for SspDmaCrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_dma_cr::W`](W) writer structure"]
impl crate::Writable for SspDmaCrSpec {
    type Safety = crate::Unsafe;
}
