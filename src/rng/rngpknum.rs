#[doc = "Register `RNGPKNUM` reader"]
pub type R = crate::R<RngpknumSpec>;
#[doc = "Register `RNGPKNUM` writer"]
pub type W = crate::W<RngpknumSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Poker test sample count\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpknum::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngpknum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngpknumSpec;
impl crate::RegisterSpec for RngpknumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngpknum::R`](R) reader structure"]
impl crate::Readable for RngpknumSpec {}
#[doc = "`write(|w| ..)` method takes [`rngpknum::W`](W) writer structure"]
impl crate::Writable for RngpknumSpec {
    type Safety = crate::Unsafe;
}
