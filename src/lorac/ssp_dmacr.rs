#[doc = "Register `SSP_DMACR` reader"]
pub type R = crate::R<SspDmacrSpec>;
#[doc = "Register `SSP_DMACR` writer"]
pub type W = crate::W<SspDmacrSpec>;
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
    pub fn rxdmae(&mut self) -> RxdmaeW<'_, SspDmacrSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - dma tx enable"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TxdmaeW<'_, SspDmacrSpec> {
        TxdmaeW::new(self, 1)
    }
}
#[doc = "ssp DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_dmacr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspDmacrSpec;
impl crate::RegisterSpec for SspDmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_dmacr::R`](R) reader structure"]
impl crate::Readable for SspDmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_dmacr::W`](W) writer structure"]
impl crate::Writable for SspDmacrSpec {
    type Safety = crate::Unsafe;
}
