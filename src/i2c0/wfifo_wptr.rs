#[doc = "Register `WFIFO_WPTR` reader"]
pub type R = crate::R<WfifoWptrSpec>;
#[doc = "Register `WFIFO_WPTR` writer"]
pub type W = crate::W<WfifoWptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "write fifo write pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo_wptr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo_wptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfifoWptrSpec;
impl crate::RegisterSpec for WfifoWptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wfifo_wptr::R`](R) reader structure"]
impl crate::Readable for WfifoWptrSpec {}
#[doc = "`write(|w| ..)` method takes [`wfifo_wptr::W`](W) writer structure"]
impl crate::Writable for WfifoWptrSpec {
    type Safety = crate::Unsafe;
}
