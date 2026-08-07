#[doc = "Register `SSTAT_L` reader"]
pub type R = crate::R<SstatLSpec>;
#[doc = "Register `SSTAT_L` writer"]
pub type W = crate::W<SstatLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sstat_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstat_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatLSpec;
impl crate::RegisterSpec for SstatLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat_l::R`](R) reader structure"]
impl crate::Readable for SstatLSpec {}
#[doc = "`write(|w| ..)` method takes [`sstat_l::W`](W) writer structure"]
impl crate::Writable for SstatLSpec {
    type Safety = crate::Unsafe;
}
