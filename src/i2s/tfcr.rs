#[doc = "Register `TFCR` reader"]
pub type R = crate::R<TfcrSpec>;
#[doc = "Register `TFCR` writer"]
pub type W = crate::W<TfcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "transmitter FIFO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfcrSpec;
impl crate::RegisterSpec for TfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfcr::R`](R) reader structure"]
impl crate::Readable for TfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tfcr::W`](W) writer structure"]
impl crate::Writable for TfcrSpec {
    type Safety = crate::Unsafe;
}
