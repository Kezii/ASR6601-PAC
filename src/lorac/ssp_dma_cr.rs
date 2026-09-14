#[doc = "Register `SSP_DMA_CR` reader"]
pub type R = crate::R<SspDmaCrSpec>;
#[doc = "Register `SSP_DMA_CR` writer"]
pub type W = crate::W<SspDmaCrSpec>;
#[doc = "Field `RXDMAE` reader - dma rx enable"]
pub type RxdmaeR = crate::BitReader;
#[doc = "Field `RXDMAE` writer - dma rx enable"]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - dma tx enable"]
pub type TxdmaeR = crate::BitReader;
#[doc = "Field `TXDMAE` writer - dma tx enable"]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - dma rx enable"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - dma tx enable"]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - dma rx enable"]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RxdmaeW<'_, SspDmaCrSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - dma tx enable"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TxdmaeW<'_, SspDmaCrSpec> {
        TxdmaeW::new(self, 1)
    }
}
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
