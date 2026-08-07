#[doc = "Register `DMA_COMP_PARAMS_3_L` reader"]
pub type R = crate::R<DmaCompParams3LSpec>;
#[doc = "Register `DMA_COMP_PARAMS_3_L` writer"]
pub type W = crate::W<DmaCompParams3LSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_3_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_comp_params_3_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams3LSpec;
impl crate::RegisterSpec for DmaCompParams3LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_3_l::R`](R) reader structure"]
impl crate::Readable for DmaCompParams3LSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_comp_params_3_l::W`](W) writer structure"]
impl crate::Writable for DmaCompParams3LSpec {
    type Safety = crate::Unsafe;
}
