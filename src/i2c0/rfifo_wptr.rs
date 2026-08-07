#[doc = "Register `RFIFO_WPTR` reader"]
pub type R = crate::R<RfifoWptrSpec>;
#[doc = "Register `RFIFO_WPTR` writer"]
pub type W = crate::W<RfifoWptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "read fifo write pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo_wptr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfifo_wptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifoWptrSpec;
impl crate::RegisterSpec for RfifoWptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo_wptr::R`](R) reader structure"]
impl crate::Readable for RfifoWptrSpec {}
#[doc = "`write(|w| ..)` method takes [`rfifo_wptr::W`](W) writer structure"]
impl crate::Writable for RfifoWptrSpec {
    type Safety = crate::Unsafe;
}
