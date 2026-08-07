#[doc = "Register `RNGRESEED` reader"]
pub type R = crate::R<RngreseedSpec>;
#[doc = "Register `RNGRESEED` writer"]
pub type W = crate::W<RngreseedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reseed register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngreseed::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngreseed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngreseedSpec;
impl crate::RegisterSpec for RngreseedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngreseed::R`](R) reader structure"]
impl crate::Readable for RngreseedSpec {}
#[doc = "`write(|w| ..)` method takes [`rngreseed::W`](W) writer structure"]
impl crate::Writable for RngreseedSpec {
    type Safety = crate::Unsafe;
}
