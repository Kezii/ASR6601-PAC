#[doc = "Register `WFIFO_RPTR` reader"]
pub type R = crate::R<WfifoRptrSpec>;
#[doc = "Register `WFIFO_RPTR` writer"]
pub type W = crate::W<WfifoRptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "write fifo read pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo_rptr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo_rptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfifoRptrSpec;
impl crate::RegisterSpec for WfifoRptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wfifo_rptr::R`](R) reader structure"]
impl crate::Readable for WfifoRptrSpec {}
#[doc = "`write(|w| ..)` method takes [`wfifo_rptr::W`](W) writer structure"]
impl crate::Writable for WfifoRptrSpec {
    type Safety = crate::Unsafe;
}
