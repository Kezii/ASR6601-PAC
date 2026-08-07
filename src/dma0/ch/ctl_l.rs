#[doc = "Register `CTL_L` reader"]
pub type R = crate::R<CtlLSpec>;
#[doc = "Register `CTL_L` writer"]
pub type W = crate::W<CtlLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlLSpec;
impl crate::RegisterSpec for CtlLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_l::R`](R) reader structure"]
impl crate::Readable for CtlLSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl_l::W`](W) writer structure"]
impl crate::Writable for CtlLSpec {
    type Safety = crate::Unsafe;
}
