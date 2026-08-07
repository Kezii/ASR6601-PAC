#[doc = "Register `DSR_L` reader"]
pub type R = crate::R<DsrLSpec>;
#[doc = "Register `DSR_L` writer"]
pub type W = crate::W<DsrLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsrLSpec;
impl crate::RegisterSpec for DsrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr_l::R`](R) reader structure"]
impl crate::Readable for DsrLSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr_l::W`](W) writer structure"]
impl crate::Writable for DsrLSpec {
    type Safety = crate::Unsafe;
}
