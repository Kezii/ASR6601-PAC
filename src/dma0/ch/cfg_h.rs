#[doc = "Register `CFG_H` reader"]
pub type R = crate::R<CfgHSpec>;
#[doc = "Register `CFG_H` writer"]
pub type W = crate::W<CfgHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgHSpec;
impl crate::RegisterSpec for CfgHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_h::R`](R) reader structure"]
impl crate::Readable for CfgHSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_h::W`](W) writer structure"]
impl crate::Writable for CfgHSpec {
    type Safety = crate::Unsafe;
}
