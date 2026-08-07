#[doc = "Register `DMACFGREG_H` reader"]
pub type R = crate::R<DmacfgregHSpec>;
#[doc = "Register `DMACFGREG_H` writer"]
pub type W = crate::W<DmacfgregHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfgreg_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfgreg_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgregHSpec;
impl crate::RegisterSpec for DmacfgregHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfgreg_h::R`](R) reader structure"]
impl crate::Readable for DmacfgregHSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfgreg_h::W`](W) writer structure"]
impl crate::Writable for DmacfgregHSpec {
    type Safety = crate::Unsafe;
}
