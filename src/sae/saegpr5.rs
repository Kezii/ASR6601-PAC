#[doc = "Register `SAEGPR5` reader"]
pub type R = crate::R<Saegpr5Spec>;
#[doc = "Register `SAEGPR5` writer"]
pub type W = crate::W<Saegpr5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "General-purpose register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`saegpr5::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saegpr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saegpr5Spec;
impl crate::RegisterSpec for Saegpr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saegpr5::R`](R) reader structure"]
impl crate::Readable for Saegpr5Spec {}
#[doc = "`write(|w| ..)` method takes [`saegpr5::W`](W) writer structure"]
impl crate::Writable for Saegpr5Spec {
    type Safety = crate::Unsafe;
}
