#[doc = "Register `RSC_ECR` reader"]
pub type R = crate::R<RscEcrSpec>;
#[doc = "Register `RSC_ECR` writer"]
pub type W = crate::W<RscEcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "receive status register / error clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsc_ecr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsc_ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RscEcrSpec;
impl crate::RegisterSpec for RscEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsc_ecr::R`](R) reader structure"]
impl crate::Readable for RscEcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rsc_ecr::W`](W) writer structure"]
impl crate::Writable for RscEcrSpec {
    type Safety = crate::Unsafe;
}
