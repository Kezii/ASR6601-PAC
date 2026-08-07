#[doc = "Register `SSTAT_H` reader"]
pub type R = crate::R<SstatHSpec>;
#[doc = "Register `SSTAT_H` writer"]
pub type W = crate::W<SstatHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sstat_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstat_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatHSpec;
impl crate::RegisterSpec for SstatHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat_h::R`](R) reader structure"]
impl crate::Readable for SstatHSpec {}
#[doc = "`write(|w| ..)` method takes [`sstat_h::W`](W) writer structure"]
impl crate::Writable for SstatHSpec {
    type Safety = crate::Unsafe;
}
