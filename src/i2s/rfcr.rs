#[doc = "Register `RFCR` reader"]
pub type R = crate::R<RfcrSpec>;
#[doc = "Register `RFCR` writer"]
pub type W = crate::W<RfcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "receiver FIFO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcrSpec;
impl crate::RegisterSpec for RfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcr::R`](R) reader structure"]
impl crate::Readable for RfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcr::W`](W) writer structure"]
impl crate::Writable for RfcrSpec {
    type Safety = crate::Unsafe;
}
