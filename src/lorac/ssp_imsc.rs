#[doc = "Register `SSP_IMSC` reader"]
pub type R = crate::R<SspImscSpec>;
#[doc = "Register `SSP_IMSC` writer"]
pub type W = crate::W<SspImscSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ssp interrupt mask set or clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_imsc::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_imsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspImscSpec;
impl crate::RegisterSpec for SspImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_imsc::R`](R) reader structure"]
impl crate::Readable for SspImscSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_imsc::W`](W) writer structure"]
impl crate::Writable for SspImscSpec {
    type Safety = crate::Unsafe;
}
