#[doc = "Register `RFIFO` reader"]
pub type R = crate::R<RfifoSpec>;
#[doc = "Register `RFIFO` writer"]
pub type W = crate::W<RfifoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "read fifo register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifoSpec;
impl crate::RegisterSpec for RfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo::R`](R) reader structure"]
impl crate::Readable for RfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`rfifo::W`](W) writer structure"]
impl crate::Writable for RfifoSpec {
    type Safety = crate::Unsafe;
}
