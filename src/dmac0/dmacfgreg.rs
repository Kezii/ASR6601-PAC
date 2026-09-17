#[doc = "Register `DMACFGREG` reader"]
pub type R = crate::R<DmacfgregSpec>;
#[doc = "Register `DMACFGREG` writer"]
pub type W = crate::W<DmacfgregSpec>;
#[doc = "Field `DMA_EN` reader - DMA enable control"]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `DMA_EN` writer - DMA enable control"]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA enable control"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable control"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, DmacfgregSpec> {
        DmaEnW::new(self, 0)
    }
}
#[doc = "DMA enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfgreg::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfgreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgregSpec;
impl crate::RegisterSpec for DmacfgregSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`dmacfgreg::R`](R) reader structure"]
impl crate::Readable for DmacfgregSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfgreg::W`](W) writer structure"]
impl crate::Writable for DmacfgregSpec {
    type Safety = crate::Unsafe;
}
