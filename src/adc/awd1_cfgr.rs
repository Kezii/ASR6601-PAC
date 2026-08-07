#[doc = "Register `AWD1_CFGR` reader"]
pub type R = crate::R<Awd1CfgrSpec>;
#[doc = "Register `AWD1_CFGR` writer"]
pub type W = crate::W<Awd1CfgrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AWD1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd1_cfgr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd1CfgrSpec;
impl crate::RegisterSpec for Awd1CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd1_cfgr::R`](R) reader structure"]
impl crate::Readable for Awd1CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`awd1_cfgr::W`](W) writer structure"]
impl crate::Writable for Awd1CfgrSpec {
    type Safety = crate::Unsafe;
}
