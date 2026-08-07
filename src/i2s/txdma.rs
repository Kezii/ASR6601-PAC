#[doc = "Register `TXDMA` reader"]
pub type R = crate::R<TxdmaSpec>;
#[doc = "Register `TXDMA` writer"]
pub type W = crate::W<TxdmaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "transmitter block dma register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdma::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdmaSpec;
impl crate::RegisterSpec for TxdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdma::R`](R) reader structure"]
impl crate::Readable for TxdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`txdma::W`](W) writer structure"]
impl crate::Writable for TxdmaSpec {
    type Safety = crate::Unsafe;
}
