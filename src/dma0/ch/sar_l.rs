#[doc = "Register `SAR_L` reader"]
pub type R = crate::R<SarLSpec>;
#[doc = "Register `SAR_L` writer"]
pub type W = crate::W<SarLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarLSpec;
impl crate::RegisterSpec for SarLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_l::R`](R) reader structure"]
impl crate::Readable for SarLSpec {}
#[doc = "`write(|w| ..)` method takes [`sar_l::W`](W) writer structure"]
impl crate::Writable for SarLSpec {
    type Safety = crate::Unsafe;
}
