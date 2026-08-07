#[doc = "Register `DSR` reader"]
pub type R = crate::R<DsrSpec>;
#[doc = "Register `DSR` writer"]
pub type W = crate::W<DsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "dirve set register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsrSpec;
impl crate::RegisterSpec for DsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr::R`](R) reader structure"]
impl crate::Readable for DsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr::W`](W) writer structure"]
impl crate::Writable for DsrSpec {
    type Safety = crate::Unsafe;
}
