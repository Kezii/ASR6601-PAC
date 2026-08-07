#[doc = "Register `AWD2_CFGR` reader"]
pub type R = crate::R<Awd2CfgrSpec>;
#[doc = "Register `AWD2_CFGR` writer"]
pub type W = crate::W<Awd2CfgrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AWD2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd2_cfgr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd2CfgrSpec;
impl crate::RegisterSpec for Awd2CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd2_cfgr::R`](R) reader structure"]
impl crate::Readable for Awd2CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`awd2_cfgr::W`](W) writer structure"]
impl crate::Writable for Awd2CfgrSpec {
    type Safety = crate::Unsafe;
}
