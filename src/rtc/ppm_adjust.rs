#[doc = "Register `PPM_ADJUST` reader"]
pub type R = crate::R<PpmAdjustSpec>;
#[doc = "Register `PPM_ADJUST` writer"]
pub type W = crate::W<PpmAdjustSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ppm adjust value\n\nYou can [`read`](crate::Reg::read) this register and get [`ppm_adjust::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppm_adjust::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpmAdjustSpec;
impl crate::RegisterSpec for PpmAdjustSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppm_adjust::R`](R) reader structure"]
impl crate::Readable for PpmAdjustSpec {}
#[doc = "`write(|w| ..)` method takes [`ppm_adjust::W`](W) writer structure"]
impl crate::Writable for PpmAdjustSpec {
    type Safety = crate::Unsafe;
}
