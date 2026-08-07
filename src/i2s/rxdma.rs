#[doc = "Register `RXDMA` reader"]
pub type R = crate::R<RxdmaSpec>;
#[doc = "Register `RXDMA` writer"]
pub type W = crate::W<RxdmaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "receiver block dma register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdma::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdmaSpec;
impl crate::RegisterSpec for RxdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma::R`](R) reader structure"]
impl crate::Readable for RxdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdma::W`](W) writer structure"]
impl crate::Writable for RxdmaSpec {
    type Safety = crate::Unsafe;
}
