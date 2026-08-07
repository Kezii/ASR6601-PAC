#[doc = "Register `RER` reader"]
pub type R = crate::R<RerSpec>;
#[doc = "Register `RER` writer"]
pub type W = crate::W<RerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "receiver enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rer::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RerSpec;
impl crate::RegisterSpec for RerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rer::R`](R) reader structure"]
impl crate::Readable for RerSpec {}
#[doc = "`write(|w| ..)` method takes [`rer::W`](W) writer structure"]
impl crate::Writable for RerSpec {
    type Safety = crate::Unsafe;
}
