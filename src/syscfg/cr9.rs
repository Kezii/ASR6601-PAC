#[doc = "Register `CR9` reader"]
pub type R = crate::R<Cr9Spec>;
#[doc = "Register `CR9` writer"]
pub type W = crate::W<Cr9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "control register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`cr9::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr9Spec;
impl crate::RegisterSpec for Cr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr9::R`](R) reader structure"]
impl crate::Readable for Cr9Spec {}
#[doc = "`write(|w| ..)` method takes [`cr9::W`](W) writer structure"]
impl crate::Writable for Cr9Spec {
    type Safety = crate::Unsafe;
}
