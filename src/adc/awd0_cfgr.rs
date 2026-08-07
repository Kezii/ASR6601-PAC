#[doc = "Register `AWD0_CFGR` reader"]
pub type R = crate::R<Awd0CfgrSpec>;
#[doc = "Register `AWD0_CFGR` writer"]
pub type W = crate::W<Awd0CfgrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AWD0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd0_cfgr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd0_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd0CfgrSpec;
impl crate::RegisterSpec for Awd0CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd0_cfgr::R`](R) reader structure"]
impl crate::Readable for Awd0CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`awd0_cfgr::W`](W) writer structure"]
impl crate::Writable for Awd0CfgrSpec {
    type Safety = crate::Unsafe;
}
