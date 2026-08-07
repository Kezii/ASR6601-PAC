#[doc = "Register `CTL_H` reader"]
pub type R = crate::R<CtlHSpec>;
#[doc = "Register `CTL_H` writer"]
pub type W = crate::W<CtlHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlHSpec;
impl crate::RegisterSpec for CtlHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_h::R`](R) reader structure"]
impl crate::Readable for CtlHSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl_h::W`](W) writer structure"]
impl crate::Writable for CtlHSpec {
    type Safety = crate::Unsafe;
}
