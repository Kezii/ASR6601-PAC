#[doc = "Register `DSTAT_L` reader"]
pub type R = crate::R<DstatLSpec>;
#[doc = "Register `DSTAT_L` writer"]
pub type W = crate::W<DstatLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dstat_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstat_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstatLSpec;
impl crate::RegisterSpec for DstatLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstat_l::R`](R) reader structure"]
impl crate::Readable for DstatLSpec {}
#[doc = "`write(|w| ..)` method takes [`dstat_l::W`](W) writer structure"]
impl crate::Writable for DstatLSpec {
    type Safety = crate::Unsafe;
}
