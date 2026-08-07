#[doc = "Register `SCK_CR` reader"]
pub type R = crate::R<SckCrSpec>;
#[doc = "Register `SCK_CR` writer"]
pub type W = crate::W<SckCrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "sck control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sck_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sck_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SckCrSpec;
impl crate::RegisterSpec for SckCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sck_cr::R`](R) reader structure"]
impl crate::Readable for SckCrSpec {}
#[doc = "`write(|w| ..)` method takes [`sck_cr::W`](W) writer structure"]
impl crate::Writable for SckCrSpec {
    type Safety = crate::Unsafe;
}
