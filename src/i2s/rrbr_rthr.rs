#[doc = "Register `RRBR_RTHR` reader"]
pub type R = crate::R<RrbrRthrSpec>;
#[doc = "Register `RRBR_RTHR` writer"]
pub type W = crate::W<RrbrRthrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "right transmit holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrbr_rthr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrbr_rthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrbrRthrSpec;
impl crate::RegisterSpec for RrbrRthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrbr_rthr::R`](R) reader structure"]
impl crate::Readable for RrbrRthrSpec {}
#[doc = "`write(|w| ..)` method takes [`rrbr_rthr::W`](W) writer structure"]
impl crate::Writable for RrbrRthrSpec {
    type Safety = crate::Unsafe;
}
