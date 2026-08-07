#[doc = "Register `DMACFGREG_L` reader"]
pub type R = crate::R<DmacfgregLSpec>;
#[doc = "Register `DMACFGREG_L` writer"]
pub type W = crate::W<DmacfgregLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfgreg_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfgreg_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgregLSpec;
impl crate::RegisterSpec for DmacfgregLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfgreg_l::R`](R) reader structure"]
impl crate::Readable for DmacfgregLSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfgreg_l::W`](W) writer structure"]
impl crate::Writable for DmacfgregLSpec {
    type Safety = crate::Unsafe;
}
