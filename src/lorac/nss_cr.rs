#[doc = "Register `NSS_CR` reader"]
pub type R = crate::R<NssCrSpec>;
#[doc = "Register `NSS_CR` writer"]
pub type W = crate::W<NssCrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "nss control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nss_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nss_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NssCrSpec;
impl crate::RegisterSpec for NssCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nss_cr::R`](R) reader structure"]
impl crate::Readable for NssCrSpec {}
#[doc = "`write(|w| ..)` method takes [`nss_cr::W`](W) writer structure"]
impl crate::Writable for NssCrSpec {
    type Safety = crate::Unsafe;
}
