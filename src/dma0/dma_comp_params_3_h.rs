#[doc = "Register `DMA_COMP_PARAMS_3_H` reader"]
pub type R = crate::R<DmaCompParams3HSpec>;
#[doc = "Register `DMA_COMP_PARAMS_3_H` writer"]
pub type W = crate::W<DmaCompParams3HSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_3_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_comp_params_3_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams3HSpec;
impl crate::RegisterSpec for DmaCompParams3HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_3_h::R`](R) reader structure"]
impl crate::Readable for DmaCompParams3HSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_comp_params_3_h::W`](W) writer structure"]
impl crate::Writable for DmaCompParams3HSpec {
    type Safety = crate::Unsafe;
}
